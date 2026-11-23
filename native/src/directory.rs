use serde_json::{json, Value};
use std::{
    collections::HashSet,
    net::SocketAddrV4,
    sync::LazyLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const CATEGORIES: &[&str] = &["lobby", "game", "story", "media", "other", "test"];
const MAX_RESPONSE: usize = 1024 * 1024;
const MAX_DIRECTORY_AGE: Duration = Duration::from_secs(10 * 60);
const LOBBY: &[&str] = &["45.11.16.75:27713"];
const GAMES: &[&str] = &[
    "169.58.64.34:27025",
    "169.58.64.34:27035",
    "169.58.64.34:27045",
    "169.58.64.34:27055",
];

// Если публичный каталог недоступен, движок запрашивает адреса с его страницы.
// Строка появляется только после ответа сервера.
static SEEDS: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../assets/shinri-servers.json"))
        .expect("invalid embedded Shinri directory")
});

pub fn plan(project: &str, region: &str, category: &str) -> Option<Value> {
    match (project, region, category) {
        ("panopticon", "global", "lobby" | "game") => Some(json!({"status": "noGlobal"})),
        ("panopticon", "cis", "lobby") => Some(json!({"addresses": LOBBY})),
        ("panopticon", "cis", "game") => Some(json!({"addresses": GAMES})),
        ("shinri", "cis" | "global", category) if CATEGORIES.contains(&category) => {
            let domain = if region == "cis" {
                "shinri-trial.ru"
            } else {
                "shinri-trial.com"
            };
            Some(json!({
                "url": format!("https://{domain}/api/servers"),
                "addresses": SEEDS[region][category].as_array().cloned().unwrap_or_default(),
            }))
        }
        _ => None,
    }
}

pub fn parse(body: &[u8]) -> Option<Value> {
    parse_at(body, SystemTime::now())
}

fn parse_at(body: &[u8], now: SystemTime) -> Option<Value> {
    if body.len() > MAX_RESPONSE {
        return None;
    }
    let response: Value = serde_json::from_slice(body).ok()?;
    let updated = UNIX_EPOCH.checked_add(Duration::from_millis(response["updatedAt"].as_u64()?))?;
    if now.duration_since(updated).unwrap_or_default() > MAX_DIRECTORY_AGE
        || updated.duration_since(now).unwrap_or_default() > MAX_DIRECTORY_AGE
    {
        return None;
    }
    let mut groups = json!({
        "lobby": [], "game": [], "story": [], "media": [], "other": [], "test": []
    });
    let mut seen = HashSet::new();
    let servers = response.get("servers")?.as_array()?;
    let mut valid_rows = 0;
    for server in servers {
        let Some(address) = server["address"]
            .as_str()
            .filter(|value| public_address(value))
        else {
            continue;
        };
        if server["online"] == false {
            valid_rows += 1;
            continue;
        }
        let (Some(name), Some(players)) = (text(&server["name"], 512), count(&server["online"]))
        else {
            continue;
        };
        if !seen.insert(address) {
            continue;
        }
        valid_rows += 1;
        let category = match (server["category"].as_str(), name) {
            (Some("test" | "t"), _) | (_, "Тест" | "Test") => "test",
            (_, name)
                if ["Тест ", "Тест#", "Тестовый ", "Test ", "Test#"]
                    .iter()
                    .any(|prefix| name.starts_with(prefix)) =>
            {
                "test"
            }
            (Some("lobby" | "l"), _) => "lobby",
            (Some("fast_game" | "f"), _) => "game",
            (Some("game" | "g" | "story"), _) => "story",
            (Some("media" | "m"), _) => "media",
            _ => "other",
        };
        groups[category].as_array_mut()?.push(json!({
            "address": address,
            "name": name,
            "map": text(&server["map"], 128),
            "players": players,
            "maxplayers": count(&server["maxPlayers"]),
            "password": server["password"].as_bool(),
        }));
    }
    (servers.is_empty() || valid_rows > 0).then_some(groups)
}

fn text(value: &Value, limit: usize) -> Option<&str> {
    let value = value.as_str()?.trim();
    (!value.is_empty() && value.len() <= limit && !value.chars().any(char::is_control))
        .then_some(value)
}

fn count(value: &Value) -> Option<u16> {
    value.as_u64()?.try_into().ok()
}

fn public_address(value: &str) -> bool {
    let Ok(address) = value.parse::<SocketAddrV4>() else {
        return false;
    };
    let ip = address.ip();
    let [a, b, ..] = ip.octets();
    address.port() > 0
        && (1..224).contains(&a)
        && !ip.is_loopback()
        && !ip.is_private()
        && !ip.is_link_local()
        && !(a == 100 && (64..128).contains(&b))
        && address.to_string() == value
}

#[cfg(test)]
mod tests {
    use super::*;

    const UPDATED: u64 = 1_795_028_205_833;

    fn current_time() -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(UPDATED)
    }

    #[test]
    fn groups_live_rows_and_rejects_invalid_addresses() {
        let body = json!({"updatedAt": UPDATED, "servers": [
            {"address": "185.71.66.87:28015", "name": "Lobby", "online": 0, "category": "lobby"},
            {"address": "185.71.66.87:28016", "name": "Fast", "online": 1, "category": "fast_game"},
            {"address": "185.71.66.87:28017", "name": "Story", "online": 2, "category": "game"},
            {"address": "185.71.66.87:28018", "name": "Media", "online": 3, "category": "media"},
            {"address": "185.71.66.87:28019", "name": "Test #1", "online": 4, "category": "game"},
            {"address": "185.71.66.87:28020", "name": "Other", "online": 5},
            {"address": "185.71.66.87:28015", "name": "Duplicate", "online": 0, "category": "other"},
            {"address": "127.0.0.1:28016", "name": "Private", "online": 0, "category": "lobby"},
            {"address": "185.71.66.87:0", "name": "Invalid", "online": 0, "category": "lobby"},
            {"address": "185.71.66.87:28021", "name": "Unknown status", "category": "lobby"},
            {"address": "185.71.66.87:28022", "name": "Offline", "online": false, "category": "lobby"},
        ]});
        let rows = parse_at(body.to_string().as_bytes(), current_time()).unwrap();
        for category in CATEGORIES {
            assert_eq!(rows[category].as_array().unwrap().len(), 1, "{category}");
        }
        assert_eq!(rows["lobby"][0]["players"], 0);
        assert!(parse(b"<html>Unavailable</html>").is_none());
    }

    #[test]
    fn distinguishes_empty_current_data_from_stale_or_invalid_data() {
        let empty = json!({"updatedAt": UPDATED, "servers": []}).to_string();
        assert!(parse_at(empty.as_bytes(), current_time()).is_some());
        assert!(parse_at(empty.as_bytes(), current_time() + MAX_DIRECTORY_AGE * 2).is_none());
        let invalid = json!({"updatedAt": UPDATED, "servers": [
            {"address": "185.71.66.87:28015", "name": "No live status"}
        ]})
        .to_string();
        assert!(parse_at(invalid.as_bytes(), current_time()).is_none());
    }

    #[test]
    fn plans_queries_only_for_available_regions_and_categories() {
        assert_eq!(
            plan("panopticon", "global", "lobby").unwrap()["status"],
            "noGlobal"
        );
        assert!(plan("panopticon", "cis", "test").is_none());
        assert_eq!(
            plan("panopticon", "cis", "game").unwrap()["addresses"],
            json!(GAMES)
        );
        for region in ["cis", "global"] {
            assert!(
                plan("shinri", region, "lobby").unwrap()["addresses"]
                    .as_array()
                    .unwrap()
                    .len()
                    > 1
            );
            for category in CATEGORIES {
                assert!(SEEDS[region][category]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .all(|address| public_address(address.as_str().unwrap())));
            }
        }
    }
}
