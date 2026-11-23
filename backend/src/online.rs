use std::{
    net::{Ipv4Addr, UdpSocket},
    thread,
    time::Duration,
};

const SERVERS: &[(Ipv4Addr, u16)] = &[
    (Ipv4Addr::new(45, 11, 16, 75), 27713),
    (Ipv4Addr::new(169, 58, 64, 34), 27025),
    (Ipv4Addr::new(169, 58, 64, 34), 27035),
    (Ipv4Addr::new(169, 58, 64, 34), 27045),
    (Ipv4Addr::new(169, 58, 64, 34), 27055),
];
const INFO: &[u8] = b"\xff\xff\xff\xffTSource Engine Query\0";
const RESPONSE_PREFIX: &[u8] = b"\xff\xff\xff\xff";
pub const EXPECTED: usize = SERVERS.len();

#[derive(Clone, Copy, Default)]
pub struct Count {
    pub total: Option<u16>,
    pub available: usize,
}

fn info_players(bytes: &[u8]) -> Option<u8> {
    let mut data = bytes.strip_prefix(RESPONSE_PREFIX)?.strip_prefix(b"I")?.get(1..)?;
    for _ in 0..4 {
        let end = data.iter().position(|byte| *byte == 0)?;
        data = &data[end + 1..];
    }
    let app_id = u16::from_le_bytes(data.get(..2)?.try_into().ok()?);
    let players = *data.get(2)?;
    let capacity = *data.get(3)?;
    (app_id == 4000 && players <= capacity).then_some(players)
}

fn query(address: (Ipv4Addr, u16)) -> Option<u8> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    socket.connect(address).ok()?;
    socket.set_read_timeout(Some(Duration::from_millis(900))).ok()?;
    socket.send(INFO).ok()?;
    let mut buffer = [0; 4096];
    let mut length = socket.recv(&mut buffer).ok()?;
    if let Some(challenge) = buffer[..length].strip_prefix(b"\xff\xff\xff\xffA") {
        let bytes: [u8; 4] = challenge.try_into().ok()?;
        let mut request = INFO.to_vec();
        request.extend(bytes);
        socket.send(&request).ok()?;
        length = socket.recv(&mut buffer).ok()?;
    }
    info_players(&buffer[..length])
}

pub fn panopticon() -> Count {
    thread::scope(|scope| {
        let jobs = SERVERS.iter().map(|&address| scope.spawn(move || query(address))).collect::<Vec<_>>();
        let mut total = 0;
        let mut available = 0;
        for result in jobs.into_iter().filter_map(|job| job.join().ok().flatten()) {
            total += u16::from(result);
            available += 1;
        }
        Count { total: (available > 0).then_some(total), available }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_source_gmod_info_and_rejects_truncated_packets() {
        let mut response = b"\xff\xff\xff\xffI\x11Lobby\0map\0garrysmod\0Game\0\xa0\x0f\0\x40".to_vec();
        assert_eq!(info_players(&response), Some(0));
        let len = response.len();
        response[len - 2] = 37;
        assert_eq!(info_players(&response), Some(37));
        response[len - 2] = 70;
        assert_eq!(info_players(&response), None);
        assert_eq!(info_players(&response[..len - 6]), None);
    }
}
