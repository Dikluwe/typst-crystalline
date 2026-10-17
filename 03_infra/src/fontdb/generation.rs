//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb/generation.md
//! @prompt-hash fc42405f
//! @layer L3
//! @updated 2026-10-09

use crate::fonts::FontSlot;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GenerationKey {
    paths: Vec<PathBuf>,
    stamps: Vec<Stamp>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp {
    path: PathBuf,
    index: u32,
    project: bool,
    identity: [u64; 8],
}
pub(crate) struct PreparedDiscovery {
    pub(crate) db: fontdb::Database,
    pub(crate) project_slots: Vec<FontSlot>,
    pub(crate) key: GenerationKey,
}
const MAX_DESCRIPTORS: usize = 16384;
const MAX_PATH_BYTES: usize = 1048576;

#[cfg(unix)]
#[derive(Debug, PartialEq, Eq)]
struct ScanStamp {
    path: PathBuf,
    link: Option<[u64; 8]>,
    target: Option<[u64; 8]>,
}

#[cfg(unix)]
#[derive(Debug, PartialEq, Eq)]
struct ProbeKey(Vec<ScanStamp>);

#[cfg(unix)]
impl ProbeKey {
    fn unchanged(&self) -> bool {
        self.0.iter().all(|previous| {
            scan_stamp(&previous.path, previous.link.is_none())
                .is_some_and(|(current, _)| current == *previous)
        })
    }
}

#[cfg(unix)]
fn metadata_identity(m: &std::fs::Metadata) -> [u64; 8] {
    use std::os::unix::fs::MetadataExt;
    [
        m.dev(),
        m.ino(),
        m.len(),
        u64::from(m.mode()),
        m.mtime() as u64,
        m.mtime_nsec() as u64,
        m.ctime() as u64,
        m.ctime_nsec() as u64,
    ]
}

#[cfg(unix)]
fn scan_stamp(
    path: &Path,
    missing_root: bool,
) -> Option<(ScanStamp, Option<std::fs::Metadata>)> {
    let link = match std::fs::symlink_metadata(path) {
        Ok(m) => Some(metadata_identity(&m)),
        Err(e) => {
            if missing_root && e.kind() == std::io::ErrorKind::NotFound {
                None
            } else {
                return None;
            }
        }
    };
    let target = match link {
        None => None,
        Some(_) => match std::fs::metadata(path) {
            Ok(m) => Some(m),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(_) => return None,
        },
    };
    let stamp = ScanStamp {
        path: path.to_path_buf(),
        link,
        target: target.as_ref().map(metadata_identity),
    };
    Some((stamp, target))
}

#[cfg(unix)]
fn probe_roots(roots: &[PathBuf]) -> Option<ProbeKey> {
    use std::collections::HashSet;
    fn visit(
        path: &Path,
        is_root: bool,
        stamps: &mut Vec<ScanStamp>,
        dirs: &mut HashSet<(u64, u64)>,
        bytes: &mut usize,
        entries: &mut usize,
    ) -> Option<()> {
        *bytes = bytes.checked_add(path.as_os_str().as_encoded_bytes().len())?;
        *entries = entries.checked_add(1)?;
        if *bytes > MAX_PATH_BYTES || *entries > MAX_DESCRIPTORS {
            return None;
        }
        let (stamp, target) = scan_stamp(path, is_root)?;
        let is_dir = target.as_ref().is_some_and(|m| m.is_dir());
        let candidate = matches!(
            path.extension().and_then(|x| x.to_str()),
            Some("ttf" | "ttc" | "TTF" | "TTC" | "otf" | "otc" | "OTF" | "OTC")
        );
        if is_root || is_dir || candidate || stamp.link != stamp.target {
            stamps.push(stamp);
        }
        if !is_dir {
            return Some(());
        }
        let meta = target.unwrap();
        use std::os::unix::fs::MetadataExt;
        if !dirs.insert((meta.dev(), meta.ino())) {
            return Some(());
        }
        for entry in std::fs::read_dir(path).ok()? {
            let entry = entry.ok()?;
            visit(&entry.path(), false, stamps, dirs, bytes, entries)?;
        }
        Some(())
    }
    let mut stamps = Vec::new();
    let mut dirs = HashSet::new();
    let mut bytes = 0;
    let mut entries = 0;
    for root in roots {
        visit(root, true, &mut stamps, &mut dirs, &mut bytes, &mut entries)?;
    }
    // Directory ctime covers entry creation/removal; candidate and symlink
    // identities cover in-place changes, including invalid-to-valid fonts.
    let key = ProbeKey(stamps);
    key.unchanged().then_some(key)
}

#[derive(Debug, PartialEq)]
pub(crate) struct FastKey {
    #[cfg(target_os = "linux")]
    config: ConfigKey,
    #[cfg(target_os = "linux")]
    roots: ProbeKey,
}

#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
struct ConfigKey {
    dirs: Vec<PathBuf>,
    aliases: Vec<String>,
    files: Vec<(PathBuf, Vec<u8>)>,
    env: Vec<(std::ffi::OsString, std::ffi::OsString)>,
    cwd: PathBuf,
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn fast_key(_paths: &[PathBuf]) -> Option<FastKey> {
    None
}

#[cfg(target_os = "linux")]
pub(crate) fn fast_key(paths: &[PathBuf]) -> Option<FastKey> {
    let config = config_key(paths)?;
    let roots = probe_roots(&config.dirs)?;
    // Config files may have changed while roots were traversed.
    if config.files.iter().any(|(path, content)| {
        std::fs::read(path).ok().as_deref() != Some(content.as_slice())
    }) {
        return None;
    }
    Some(FastKey { config, roots })
}

#[cfg(target_os = "linux")]
pub(crate) fn fast_key_unchanged(old: &FastKey, paths: &[PathBuf]) -> bool {
    config_key(paths).is_some_and(|current| current == old.config)
        && old.roots.unchanged()
}

#[cfg(target_os = "linux")]
fn config_key(paths: &[PathBuf]) -> Option<ConfigKey> {
    // Until project discovery has a matching lightweight witness, keep its
    // existing fresh route. Likewise decline nonstandard fontconfig roots.
    if !paths.is_empty() || std::env::var_os("FONTCONFIG_FILE").is_some() {
        return None;
    }
    if !matches!(std::fs::symlink_metadata("/etc/fonts/local.conf"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound)
    {
        return None;
    }
    let home = std::env::var("HOME").ok()?;
    let user_config = match std::env::var("XDG_CONFIG_HOME") {
        Ok(xdg) => PathBuf::from(xdg),
        Err(std::env::VarError::NotPresent) => Path::new(&home).join(".config"),
        Err(_) => return None,
    }
    .join("fontconfig/fonts.conf");
    if !matches!(std::fs::symlink_metadata(user_config),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound)
    {
        return None;
    }

    let mut config = fontconfig_parser::FontConfig::default();
    config.merge_config("/etc/fonts/fonts.conf").ok()?;
    if config.dirs.is_empty() {
        return None;
    }
    let dirs: Vec<_> = config
        .dirs
        .iter()
        .filter_map(|dir| {
            if dir.path.starts_with("~") {
                Some(Path::new(&home).join(dir.path.strip_prefix("~").ok()?))
            } else {
                Some(dir.path.clone())
            }
        })
        .collect();
    if dirs.len() != config.dirs.len() {
        return None;
    }
    let aliases = config.aliases.iter().map(|alias| format!("{alias:?}")).collect();
    let mut config_paths: Vec<_> = config.config_files.into_iter().collect();
    config_paths.sort();
    let mut files = Vec::with_capacity(config_paths.len());
    let mut bytes = 0usize;
    for path in config_paths {
        let content = std::fs::read(&path).ok()?;
        bytes = bytes
            .checked_add(path.as_os_str().as_encoded_bytes().len())?
            .checked_add(content.len())?;
        if bytes > MAX_PATH_BYTES {
            return None;
        }
        files.push((path, content));
    }
    let mut env: Vec<_> = std::env::vars_os().collect();
    env.sort_by(|a, b| a.0.cmp(&b.0));
    let cwd = std::env::current_dir().ok()?;
    Some(ConfigKey { dirs, aliases, files, env, cwd })
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn fast_key_unchanged(_old: &FastKey, _paths: &[PathBuf]) -> bool {
    false
}

#[cfg(all(test, target_os = "linux"))]
pub(crate) fn fixture_fast_key(roots: &[PathBuf]) -> Option<FastKey> {
    Some(FastKey {
        config: ConfigKey {
            dirs: roots.to_vec(),
            aliases: Vec::new(),
            files: Vec::new(),
            env: Vec::new(),
            cwd: PathBuf::new(),
        },
        roots: probe_roots(roots)?,
    })
}

#[cfg(all(test, target_os = "linux"))]
pub(crate) fn fixture_fast_key_unchanged(old: &FastKey) -> bool {
    old.roots.unchanged()
}

#[cfg(all(test, unix))]
mod quick_proof_tests {
    use super::*;
    use std::fs;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "tekt-font-proof-{}-{:016x}",
                std::process::id(),
                fastrand::u64(..)
            ));
            fs::create_dir_all(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn same_roots_keep_proof_and_nested_addition_invalidates_it() {
        let f = Fixture::new();
        fs::write(f.0.join("a.ttf"), b"alpha").unwrap();
        let roots = [f.0.clone()];
        let first = probe_roots(&roots).expect("eligible stable root");
        assert!(first.unchanged());
        assert_eq!(first, probe_roots(&roots).unwrap());
        let old_mtime = fs::metadata(&f.0).unwrap().modified().unwrap();
        fs::create_dir(f.0.join("nested")).unwrap();
        fs::write(f.0.join("nested/b.ttf"), b"beta").unwrap();
        fs::File::open(&f.0)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(old_mtime))
            .unwrap();
        assert!(!first.unchanged());
        assert_ne!(first, probe_roots(&roots).unwrap());
    }

    #[test]
    fn same_size_with_restored_mtime_and_symlink_retarget_invalidate() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        let a = f.0.join("a.ttf");
        let b = f.0.join("b.ttf");
        fs::write(&a, b"alpha").unwrap();
        fs::write(&b, b"bravo").unwrap();
        let link = f.0.join("link.ttf");
        symlink(&a, &link).unwrap();
        let roots = [f.0.clone()];
        let first = probe_roots(&roots).expect("eligible symlink root");
        let old = fs::metadata(&a).unwrap().modified().unwrap();
        fs::write(&a, b"ALPHA").unwrap();
        fs::File::options()
            .write(true)
            .open(&a)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(old))
            .unwrap();
        assert!(!first.unchanged());
        let changed = probe_roots(&roots).unwrap();
        assert_ne!(first, changed);
        fs::remove_file(&link).unwrap();
        symlink(&b, &link).unwrap();
        assert!(!changed.unchanged());
        assert_ne!(changed, probe_roots(&roots).unwrap());
    }

    #[test]
    fn missing_root_creation_and_budget_refuse_reuse() {
        let f = Fixture::new();
        let missing = f.0.join("later");
        let first = probe_roots(std::slice::from_ref(&missing)).unwrap();
        assert!(first.unchanged());
        fs::create_dir(&missing).unwrap();
        assert!(!first.unchanged());
        assert!(probe_roots(&[PathBuf::from("x".repeat(MAX_PATH_BYTES + 1))]).is_none());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn custom_fontconfig_declines_in_child_process() {
        const MARKER: &str = "TEKT_TEST_CUSTOM_FONTCONFIG";
        if std::env::var_os(MARKER).is_some() {
            assert!(fast_key(&[]).is_none());
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "fontdb::generation::quick_proof_tests::custom_fontconfig_declines_in_child_process"])
            .env(MARKER, "1")
            .env("FONTCONFIG_FILE", "/tmp/tekt-custom-fontconfig.xml")
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub(crate) fn prepare(paths: &[PathBuf]) -> Option<PreparedDiscovery> {
    if !cfg!(unix) {
        return None;
    }
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    prepare_from(db, paths, MAX_DESCRIPTORS, MAX_PATH_BYTES)
}

fn prepare_from(
    db: fontdb::Database,
    paths: &[PathBuf],
    count: usize,
    bytes: usize,
) -> Option<PreparedDiscovery> {
    if !cfg!(unix) {
        return None;
    }
    let mut used = 0usize;
    for path in paths {
        used = used.checked_add(path.as_os_str().as_encoded_bytes().len())?;
    }
    if used > bytes || paths.len() > count {
        return None;
    }
    let project_slots = crate::fonts::discover_fonts(paths);
    let mut stamps = Vec::new();
    let mut push = |path: &Path, index, project| -> Option<()> {
        if stamps.len() >= count {
            return None;
        }
        used = used.checked_add(path.as_os_str().as_encoded_bytes().len())?;
        if used > bytes {
            return None;
        }
        let identity = identity(path)?;
        stamps.try_reserve(1).ok()?;
        stamps.push(Stamp { path: path.to_path_buf(), index, project, identity });
        Some(())
    };
    for face in db.faces() {
        match &face.source {
            fontdb::Source::File(path) => push(path, face.index, false)?,
            fontdb::Source::SharedFile(..) => return None,
            fontdb::Source::Binary(_) => {}
        }
    }
    for slot in &project_slots {
        push(&slot.path, slot.index, true)?;
    }
    let key = GenerationKey { paths: paths.to_vec(), stamps };
    if !key.unchanged() {
        return None;
    }
    Some(PreparedDiscovery { db, project_slots, key })
}

#[cfg(unix)]
fn identity(path: &Path) -> Option<[u64; 8]> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::metadata(path).ok()?;
    if !m.is_file() {
        return None;
    }
    Some([
        m.dev(),
        m.ino(),
        m.len(),
        u64::from(m.mode()),
        m.mtime() as u64,
        m.mtime_nsec() as u64,
        m.ctime() as u64,
        m.ctime_nsec() as u64,
    ])
}
#[cfg(not(unix))]
fn identity(_path: &Path) -> Option<[u64; 8]> {
    None
}
impl PreparedDiscovery {
    pub(crate) fn unchanged(&self) -> bool {
        self.key.unchanged()
    }
}
impl GenerationKey {
    pub(crate) fn unchanged(&self) -> bool {
        self.stamps.iter().all(|s| identity(&s.path) == Some(s.identity))
    }
}

#[cfg(test)]
pub(crate) fn fixture_prepare(paths: &[PathBuf]) -> Option<PreparedDiscovery> {
    prepare_from(fontdb::Database::new(), paths, 16384, 1048576)
}

#[cfg(all(test, unix))]
pub(crate) fn fixture_system_prepare(dir: &Path) -> Option<PreparedDiscovery> {
    let mut db = fontdb::Database::new();
    db.load_fonts_dir(dir);
    prepare_from(db, &[], MAX_DESCRIPTORS, MAX_PATH_BYTES)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    const FONT: &[u8] = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "watch-generation-{}-{:016x}",
                std::process::id(),
                fastrand::u64(..)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn font(&self, name: &str) -> PathBuf {
            let p = self.0.join(name);
            fs::write(&p, FONT).unwrap();
            p
        }
        fn snapshot(&self) -> PreparedDiscovery {
            let mut db = fontdb::Database::new();
            db.load_fonts_dir(&self.0);
            prepare_from(db, &[], 16384, 1048576)
                .expect("stable regular sources eligible")
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn watch_generation_stable_then_added_removed() {
        let f = Fixture::new();
        f.font("a.ttf");
        let a = f.snapshot();
        assert!(a.unchanged());
        assert_eq!(a.key, f.snapshot().key);
        let p = f.font("b.ttf");
        assert_ne!(a.key, f.snapshot().key);
        fs::remove_file(p).unwrap();
        assert_eq!(a.key, f.snapshot().key);
    }
    #[test]
    fn watch_generation_restored_mtime_still_changes() {
        let f = Fixture::new();
        let p = f.font("a.ttf");
        let old = f.snapshot();
        let modified = fs::metadata(&p).unwrap().modified().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let mut data = FONT.to_vec();
        *data.last_mut().unwrap() ^= 1;
        fs::write(&p, data).unwrap();
        fs::File::options()
            .write(true)
            .open(&p)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(modified))
            .unwrap();
        assert!(!old.unchanged());
        assert_ne!(old.key, f.snapshot().key);
    }
    #[test]
    fn watch_generation_atomic_replace_and_missing() {
        let f = Fixture::new();
        let p = f.font("a.ttf");
        let old = f.snapshot();
        let temp = f.font("replace.bin");
        fs::rename(temp, &p).unwrap();
        assert!(!old.unchanged());
        let next = f.snapshot();
        fs::remove_file(p).unwrap();
        assert!(!next.unchanged());
    }
    #[test]
    fn watch_generation_negative_becomes_valid() {
        let f = Fixture::new();
        f.font("a.ttf");
        let old = f.snapshot();
        fs::write(f.0.join("b.ttf"), b"invalid").unwrap();
        assert_eq!(old.key, f.snapshot().key);
        f.font("b.ttf");
        assert_ne!(old.key, f.snapshot().key);
    }
    #[test]
    fn watch_generation_project_nested_and_configuration() {
        let f = Fixture::new();
        let a = fixture_prepare(std::slice::from_ref(&f.0)).unwrap();
        fs::create_dir(f.0.join("nested")).unwrap();
        f.font("nested/a.ttf");
        let b = fixture_prepare(std::slice::from_ref(&f.0)).unwrap();
        assert_ne!(a.key, b.key);
        assert_ne!(b.key, fixture_prepare(&[]).unwrap().key);
        assert_eq!(b.project_slots.len(), 1);
    }
    #[test]
    fn watch_generation_shared_file_declines() {
        let f = Fixture::new();
        let p = f.font("a.ttf");
        let mut db = fontdb::Database::new();
        db.load_font_source(fontdb::Source::SharedFile(
            p,
            std::sync::Arc::new(FONT.to_vec()),
        ));
        assert!(prepare_from(db, &[], 16384, 1048576).is_none());
    }
    #[test]
    fn watch_generation_limits_decline_instead_of_truncate() {
        let f = Fixture::new();
        f.font("a.ttf");
        let db = || {
            let mut d = fontdb::Database::new();
            d.load_fonts_dir(&f.0);
            d
        };
        assert!(prepare_from(db(), &[], 0, 1048576).is_none());
        assert!(prepare_from(db(), &[], 16384, 0).is_none());
        assert!(prepare_from(db(), &[], 1, 1048576).is_some());
    }
    #[test]
    fn watch_generation_symlink_retarget() {
        let f = Fixture::new();
        let targets = Fixture::new();
        let a = targets.font("a.ttf");
        let b = targets.font("b.ttf");
        let link = f.0.join("link.ttf");
        std::os::unix::fs::symlink(a, &link).unwrap();
        let old = f.snapshot();
        fs::remove_file(&link).unwrap();
        std::os::unix::fs::symlink(b, &link).unwrap();
        assert_ne!(old.key, f.snapshot().key);
    }
    #[test]
    fn watch_generation_fontconfig_changes_in_child() {
        let a = Fixture::new();
        a.font("a.ttf");
        let b = Fixture::new();
        b.font("b.ttf");
        for f in [&a, &b] {
            let config = f.0.join("fonts.conf");
            fs::write(
                &config,
                format!("<fontconfig><dir>{}</dir></fontconfig>", f.0.display()),
            )
            .unwrap();
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "fontdb::generation::tests::watch_generation_config_child",
                    "--ignored",
                    "--nocapture",
                ])
                .env("FONTCONFIG_FILE", &config)
                .env("TEKT_GENERATION_TEST_DIR", &f.0)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    #[test]
    #[ignore = "isolated fontconfig child, invoked by parent test"]
    fn watch_generation_config_child() {
        let dir = PathBuf::from(std::env::var_os("TEKT_GENERATION_TEST_DIR").unwrap());
        let p = prepare(&[]).unwrap();
        assert_eq!(p.key.stamps.len(), 1);
        assert!(p.key.stamps[0].path.starts_with(&dir));
    }
}
