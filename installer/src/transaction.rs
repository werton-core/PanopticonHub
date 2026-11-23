use super::*;

const WORK_PREFIX: &str = ".panopticon-hub-work-";

#[derive(Serialize, Deserialize)]
pub(super) struct Pending {
    #[serde(flatten)]
    pub(super) receipt: Receipt,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) recovery: Option<Recovery>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct Recovery {
    work: String,
    backups: BTreeMap<String, String>,
    #[serde(rename = "created_shortcuts")]
    shortcuts: Vec<PathBuf>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    shortcut_backups: BTreeMap<PathBuf, String>,
    legacy_pending: Option<Receipt>,
}

impl Pending {
    pub(super) fn validate(&self, directory: &Path) -> Result<(), String> {
        validate_receipt(&self.receipt, directory)?;
        if let Some(recovery) = &self.recovery {
            let valid_work = recovery
                .work
                .strip_prefix(WORK_PREFIX)
                .is_some_and(|suffix| {
                    suffix.split_once('-').is_some_and(|(pid, time)| {
                        !pid.is_empty()
                            && !time.is_empty()
                            && pid.bytes().chain(time.bytes()).all(|b| b.is_ascii_digit())
                    })
                });
            if !valid_work
                || recovery
                    .backups
                    .iter()
                    .any(|(name, hash)| {
                        let known = self.receipt.binary_hash(name).is_some() || name == CORE;
                        !known || !valid_digest(hash)
                    })
                || recovery.shortcuts.len() > 2
                || recovery
                    .shortcuts
                    .iter()
                    .any(|path| !self.receipt.shortcut_hashes.contains_key(path))
                || recovery.shortcut_backups.iter().any(|(path, hash)| {
                    !recovery.shortcuts.contains(path) || !valid_digest(hash)
                })
            {
                return Err("Повреждён журнал установки.".into());
            }
            if let Some(legacy) = &recovery.legacy_pending {
                validate_receipt(legacy, directory)?;
            }
        }
        Ok(())
    }

    pub(super) fn removed_core_hash(&self) -> Option<&str> {
        if self.receipt.binary_hash(CORE).is_some() {
            return None;
        }
        self.recovery.as_ref()?.backups.get(CORE).map(String::as_str)
    }

    pub(super) fn shortcut_before(&self, path: &Path) -> Option<&str> {
        self.recovery
            .as_ref()?
            .shortcut_backups
            .get(path)
            .map(String::as_str)
    }

    pub(super) fn remove_legacy_core(&self, directory: &Path) -> Result<(), String> {
        if let Some(hash) = self.removed_core_hash() {
            let path = directory.join(CORE);
            if !platform::delete_matching(&path, &[hash])? {
                return Err(format!("Файл {} изменился. Повторите установку.", path.display()));
            }
        }
        Ok(())
    }
}

pub(super) struct Work {
    directory: PathBuf,
    owned: Vec<(PathBuf, String)>,
    keep: bool,
}

impl Work {
    pub(super) fn create(directory: &Path) -> Result<Self, String> {
        let directory = directory.join(format!("{WORK_PREFIX}{}-{}", std::process::id(), nonce()?));
        // Даже случайно занятую папку не используем для подготовки установки.
        fs::create_dir(&directory).map_err(|e| e.to_string())?;
        Ok(Self {
            directory,
            owned: Vec::new(),
            keep: false,
        })
    }

    fn file(&self, name: &str, suffix: &str) -> PathBuf {
        self.directory.join(format!("{name}.{suffix}"))
    }

    pub(super) fn stage_binary(
        &mut self,
        directory: &Path,
        name: &str,
        bytes: &[u8],
        receipts: &[&Receipt],
        backups: &mut BTreeMap<String, String>,
    ) -> Result<(), String> {
        let target = directory.join(name);
        platform::reject_links(&target)?;
        if target.try_exists().map_err(|e| e.to_string())? {
            let hash = file_digest(&target)?;
            if !receipts.iter().any(|r| r.binary_hash(name) == Some(hash.as_str())) {
                return Err(format!(
                    "{} изменён пользователем или принадлежит другой программе. Файл сохранён.",
                    target.display()
                ));
            }
            backups.insert(name.into(), hash.clone());
            if hash == digest(bytes) {
                return Ok(());
            }
            self.stage_backup(&target, name, &hash)?;
        }
        let staged = self.file(name, "new");
        atomic_file(&staged, bytes, false)?;
        self.owned.push((staged, digest(bytes)));
        Ok(())
    }

    pub(super) fn stage_legacy_core(
        &mut self,
        directory: &Path,
        receipts: &[&Receipt],
        backups: &mut BTreeMap<String, String>,
    ) -> Result<(), String> {
        let mut hashes = receipts.iter().filter_map(|r| r.binary_hash(CORE)).peekable();
        if hashes.peek().is_none() {
            return Ok(());
        }
        let path = directory.join(CORE);
        platform::reject_links(&path)?;
        if path.is_file() {
            let hash = file_digest(&path)?;
            if hashes.any(|expected| expected == hash) {
                self.stage_backup(&path, CORE, &hash)?;
                backups.insert(CORE.into(), hash);
            }
        }
        Ok(())
    }

    fn stage_backup(&mut self, target: &Path, name: &str, hash: &str) -> Result<(), String> {
        let bytes = fs::read(target).map_err(|e| e.to_string())?;
        if digest(&bytes) != hash {
            return Err(format!("Файл {} изменился во время подготовки.", target.display()));
        }
        let backup = self.file(name, "bak");
        atomic_file(&backup, &bytes, false)?;
        self.owned.push((backup, hash.to_owned()));
        Ok(())
    }

    fn shortcut_file(&self, index: usize) -> PathBuf {
        self.file(&format!("shortcut-{index}"), "lnk")
    }

    pub(super) fn stage_shortcut(
        &mut self,
        index: usize,
        directory: &Path,
        existing: Option<(&Path, &str)>,
    ) -> Result<String, String> {
        if let Some((target, hash)) = existing {
            self.stage_backup(target, &format!("shortcut-{index}"), hash)?;
        }
        let path = self.shortcut_file(index);
        platform::create_shortcut(&path, &directory.join(HOST), directory)?;
        let hash = file_digest(&path)?;
        self.owned.push((path, hash.clone()));
        Ok(hash)
    }

    pub(super) fn publish_shortcut(
        &self,
        index: usize,
        target: &Path,
        hash: &str,
        before: Option<&str>,
    ) -> Result<(), String> {
        let staged = self.shortcut_file(index);
        platform::reject_links(&staged)?;
        let bytes = fs::read(&staged).map_err(|e| e.to_string())?;
        if digest(&bytes) != hash {
            return Err("Подготовленный ярлык изменился.".into());
        }
        if let Some(before) = before {
            if !platform::delete_matching(target, &[before])? {
                return Err(format!("Ярлык {} изменился. Файл сохранён.", target.display()));
            }
        }
        atomic_file(target, &bytes, false)
    }

    pub(super) fn journal(
        &mut self,
        receipt: Receipt,
        backups: BTreeMap<String, String>,
        shortcuts: Vec<PathBuf>,
        shortcut_backups: BTreeMap<PathBuf, String>,
        legacy_pending: Option<Receipt>,
    ) -> Pending {
        self.keep = true;
        Pending {
            receipt,
            recovery: Some(Recovery {
                work: self
                    .directory
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                backups,
                shortcuts,
                shortcut_backups,
                legacy_pending,
            }),
        }
    }

    pub(super) fn clean_on_drop(&mut self) {
        self.keep = false;
    }

    pub(super) fn publish_binary(
        &self,
        directory: &Path,
        name: &str,
        pending: &Pending,
    ) -> Result<(), String> {
        let target = directory.join(name);
        let after = pending.receipt.binary_hash(name).unwrap();
        let recovery = pending.recovery.as_ref().unwrap();
        if recovery.backups.get(name).map(String::as_str) == Some(after) {
            return Ok(());
        }
        let staged = self.file(name, "new");
        require_digest(&staged, after)?;
        if let Some(before) = recovery.backups.get(name) {
            if !platform::delete_matching(&target, &[before])? {
                return Err(format!(
                    "Файл {} изменился. Повторите установку.",
                    target.display()
                ));
            }
        }
        platform::move_file(&staged, &target, false)
    }
}

impl Drop for Work {
    fn drop(&mut self) {
        if !self.keep {
            for (path, hash) in &self.owned {
                let _ = platform::delete_matching(path, &[hash]);
            }
            let _ = fs::remove_dir(&self.directory);
        }
    }
}

pub(super) fn recover(
    directory: &Path,
    pending: &Pending,
    pending_hash: &str,
    previous: Option<(&Receipt, &str)>,
    shell: bool,
    committed: bool,
) -> Result<(), String> {
    let recovery = pending.recovery.as_ref().unwrap();
    let work = directory.join(&recovery.work);
    platform::reject_links(&work)?;
    let mut error = None;
    if !committed {
        if pending.removed_core_hash().is_some_and(|hash| {
            previous.and_then(|(r, _)| r.binary_hash(CORE)) != Some(hash)
                && recovery.legacy_pending.as_ref().and_then(|r| r.binary_hash(CORE)) != Some(hash)
        }) {
            return Err("SHA-256 прежнего ядра отсутствует в проверенном манифесте.".into());
        }
        for (index, path) in recovery.shortcuts.iter().enumerate() {
            if platform::allowed_shortcut(path, pending.receipt.scope)? {
                let before = pending.shortcut_before(path);
                if before.is_some_and(|hash| {
                    !previous
                        .map(|(receipt, _)| receipt)
                        .into_iter()
                        .chain(recovery.legacy_pending.as_ref())
                        .any(|receipt| {
                            receipt.shortcut_hashes.get(path).map(String::as_str) == Some(hash)
                        })
                }) {
                    return Err("SHA-256 прежнего ярлыка отсутствует в проверенном манифесте.".into());
                }
                let backup = work.join(format!("shortcut-{index}.bak"));
                if let Err(failure) = restore_shortcut(
                    path,
                    &pending.receipt.shortcut_hashes[path],
                    before.map(|hash| (backup.as_path(), hash)),
                ) {
                    error.get_or_insert(failure);
                }
            }
        }
        for name in [UNINSTALL, HOST, CORE] {
            if pending.receipt.binary_hash(name).is_none() && !recovery.backups.contains_key(name) {
                continue;
            }
            if let Err(failure) = restore_binary(directory, &work, pending, name) {
                error.get_or_insert(failure);
            }
        }
        if shell {
            let result = if let Some((previous, _)) = previous {
                platform::register(previous)
            } else {
                platform::unregister(&pending.receipt)
            };
            if let Err(failure) = result {
                error.get_or_insert(failure);
            }
        }
        if let Some(error) = error {
            return Err(error);
        }
    }
    require_digest(&directory.join(PENDING), pending_hash)?;
    if let Some(legacy) = recovery.legacy_pending.as_ref().filter(|_| !committed) {
        write_record(directory, PENDING, legacy, true)?;
    } else if !platform::delete_matching(&directory.join(PENDING), &[pending_hash])? {
        return Err("Журнал установки изменился.".into());
    }

    // После восстановления/commit удаляем только знакомые временные файлы и пустую папку.
    for (name, after) in pending.receipt.binaries() {
        let _ = platform::delete_matching(&work.join(format!("{name}.new")), &[after]);
    }
    for (name, before) in &recovery.backups {
        let _ = platform::delete_matching(&work.join(format!("{name}.bak")), &[before]);
    }
    for (index, path) in recovery.shortcuts.iter().enumerate() {
        let _ = platform::delete_matching(
            &work.join(format!("shortcut-{index}.lnk")),
            &[&pending.receipt.shortcut_hashes[path]],
        );
        if let Some(before) = pending.shortcut_before(path) {
            let _ = platform::delete_matching(&work.join(format!("shortcut-{index}.bak")), &[before]);
        }
    }
    let _ = fs::remove_dir(work);
    Ok(())
}

fn restore_shortcut(
    target: &Path,
    after: &str,
    backup: Option<(&Path, &str)>,
) -> Result<(), String> {
    let Some((backup, before)) = backup else {
        return platform::delete_matching(target, &[after]).map(|_| ());
    };
    platform::reject_links(target)?;
    let exists = target.try_exists().map_err(|e| e.to_string())?;
    if exists && file_digest(target)? != after {
        // Сохраняем исходный ярлык или более позднее пользовательское изменение.
        return Ok(());
    }
    platform::reject_links(backup)?;
    let bytes = fs::read(backup).map_err(|e| e.to_string())?;
    if digest(&bytes) != before {
        return Err(format!("Повреждена резервная копия {}.", backup.display()));
    }
    if exists && !platform::delete_matching(target, &[after])? {
        return Ok(());
    }
    atomic_file(target, &bytes, false)?;
    platform::notify_shortcut(target);
    Ok(())
}

fn restore_binary(
    directory: &Path,
    work: &Path,
    pending: &Pending,
    name: &str,
) -> Result<(), String> {
    let target = directory.join(name);
    platform::reject_links(&target)?;
    let after = pending.receipt.binary_hash(name);
    let recovery = pending.recovery.as_ref().unwrap();
    let before = recovery.backups.get(name);
    let current = if target.try_exists().map_err(|e| e.to_string())? {
        Some(file_digest(&target)?)
    } else {
        None
    };
    if current.as_ref() == before {
        return Ok(());
    }
    if current.as_deref().is_some_and(|hash| Some(hash) != after) {
        if before.is_none() || after.is_none() {
            return Ok(());
        }
        return Err(format!(
            "Файл {} изменился. Файл сохранён.",
            target.display()
        ));
    }
    let backup = before
        .map(|hash| {
            let path = work.join(format!("{name}.bak"));
            platform::reject_links(&path)?;
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            if digest(&bytes) != *hash {
                return Err(format!("Повреждена резервная копия {}.", path.display()));
            }
            Ok(bytes)
        })
        .transpose()?;
    if let Some(hash) = current.as_deref() {
        if !platform::delete_matching(&target, &[hash])? {
            return Err(format!(
                "Файл {} изменился. Файл сохранён.",
                target.display()
            ));
        }
    }
    if let Some(bytes) = backup {
        atomic_file(&target, &bytes, false)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcut_upgrade_is_reversible_and_preserves_user_changes() {
        let root = std::env::temp_dir().join(format!("panopticon-shortcut-upgrade-{}", nonce().unwrap()));
        fs::create_dir(&root).unwrap();
        let target = root.join("Panopticon Hub.lnk");
        fs::write(&target, b"original shortcut").unwrap();
        let before = file_digest(&target).unwrap();
        let mut work = Work::create(&root).unwrap();
        let after = work.stage_shortcut(0, &root, Some((&target, &before))).unwrap();
        let backup = work.file("shortcut-0", "bak");
        assert_eq!(file_digest(&backup).unwrap(), before);
        work.publish_shortcut(0, &target, &after, Some(&before)).unwrap();
        assert!(platform::shortcut_is_current(&target, &root.join(HOST)).unwrap());
        restore_shortcut(&target, &after, Some((&backup, &before))).unwrap();
        assert_eq!(file_digest(&target).unwrap(), before);

        fs::remove_file(&target).unwrap();
        restore_shortcut(&target, &after, Some((&backup, &before))).unwrap();
        assert_eq!(file_digest(&target).unwrap(), before);

        fs::write(&target, b"user shortcut").unwrap();
        assert!(work.publish_shortcut(0, &target, &after, Some(&before)).is_err());
        restore_shortcut(&target, &after, Some((&backup, &before))).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"user shortcut");
        drop(work);
        fs::remove_file(target).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
