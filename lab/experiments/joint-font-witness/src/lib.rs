//! Joint native discovery and evidence of its observed inputs, in quarantine.
//! Policy completeness additionally requires the pipeline's original config reparse.
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_descriptors: usize,
    pub max_path_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self { max_descriptors: 16_384, max_path_bytes: 1_048_576 }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stamp {
    pub path: PathBuf,
    pub link: Option<[u64; 8]>,
    pub target: Option<[u64; 8]>,
}
#[derive(Clone, Debug)]
pub struct ExpectedConfig {
    pub roots: Vec<PathBuf>,
    pub aliases: Vec<String>,
    pub files: Vec<(PathBuf, Vec<u8>)>,
    pub env: Vec<(OsString, OsString)>,
    pub cwd: PathBuf,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    /// Real std::fs attempts during capture only; not kernel syscall counts.
    pub capture_lstat: u64,
    pub capture_stat: u64,
    pub directory_attempts: u64,
    pub entry_callbacks: u64,
    pub file_attempts: u64,
    pub visits: u64,
    pub path_bytes: u64,
    pub declined: bool,
}
#[derive(Clone, Debug)]
pub struct Evidence {
    stamps: Vec<Stamp>,
    roots: Vec<PathBuf>,
    env: Vec<(OsString, OsString)>,
    cwd: PathBuf,
    files: Vec<(PathBuf, Vec<u8>)>,
}
impl Evidence {
    pub fn stamps(&self) -> &[Stamp] {
        &self.stamps
    }
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }
}
pub struct Attempt {
    pub db: fontdb::Database,
    pub evidence: Option<Evidence>,
    pub stats: Stats,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    RootCaptured(PathBuf),
    EntryCaptured(PathBuf),
    DirectoryCaptured(PathBuf),
    FileBefore(PathBuf),
    FileAfter(PathBuf),
    ConfigObserved,
    Finished,
    Validated,
}

fn environment() -> Vec<(OsString, OsString)> {
    let mut env: Vec<_> = std::env::vars_os().collect();
    env.sort_by(|a, b| a.0.cmp(&b.0));
    env
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
        if IGNORE_CTIME { 0 } else { m.ctime() as u64 },
        if IGNORE_CTIME { 0 } else { m.ctime_nsec() as u64 },
    ]
}
/// Capture-only policy. Validation always rechecks lstat and target separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureMode { Legacy, SingleRegular }

const IGNORE_CTIME: bool = false;
const OMIT_WITNESS: bool = false;
const OMIT_SELFCHECK: bool = false;
const OMIT_G2: bool = false;
const OMIT_G3: bool = false;
const OMIT_POLICY: bool = false;

#[cfg(unix)]
fn capture_stamp(path: &Path, missing_root: bool, mode: CaptureMode, stats: &mut Stats) -> Option<Stamp> {
    stats.capture_lstat = stats.capture_lstat.saturating_add(1);
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => Some(metadata),
        Err(error) if missing_root && error.kind() == io::ErrorKind::NotFound => None,
        Err(_) => return None,
    };
    let link = metadata.as_ref().map(metadata_identity);
    let target = match metadata {
        None => None,
        Some(metadata) if mode == CaptureMode::SingleRegular && !metadata.file_type().is_symlink() => {
            // Same-instant evidence for non-links only. validate() below still
            // performs both independent filesystem observations afterward.
            link
        }
        Some(_) => {
            stats.capture_stat = stats.capture_stat.saturating_add(1);
            match std::fs::metadata(path) {
                Ok(metadata) => Some(metadata_identity(&metadata)),
                Err(error) if error.kind() == io::ErrorKind::NotFound => None,
                Err(_) => return None,
            }
        }
    };
    Some(Stamp { path: try_path(path)?, link, target })
}
#[cfg(not(unix))]
fn capture_stamp(_path: &Path, _missing_root: bool, _mode: CaptureMode, _stats: &mut Stats) -> Option<Stamp> { None }

#[cfg(unix)]
fn stamp(path: &Path, missing_root: bool) -> Option<Stamp> {
    let link = match std::fs::symlink_metadata(path) {
        Ok(m) => Some(metadata_identity(&m)),
        Err(error) if missing_root && error.kind() == io::ErrorKind::NotFound => None,
        Err(_) => return None,
    };
    // These are separate observations, even when the first one is regular.
    let target = match link {
        None => None,
        Some(_) => match std::fs::metadata(path) {
            Ok(m) => Some(metadata_identity(&m)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(_) => return None,
        },
    };
    Some(Stamp { path: try_path(path)?, link, target })
}
#[cfg(not(unix))]
fn stamp(_path: &Path, _missing_root: bool) -> Option<Stamp> {
    None
}
#[cfg(unix)]
fn try_path(path: &Path) -> Option<PathBuf> {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let bytes = path.as_os_str().as_bytes();
    let mut owned = Vec::new();
    owned.try_reserve_exact(bytes.len()).ok()?;
    owned.extend_from_slice(bytes);
    Some(PathBuf::from(OsString::from_vec(owned)))
}
#[cfg(not(unix))]
fn try_path(_path: &Path) -> Option<PathBuf> {
    None
}
fn kind(identity: &[u64; 8]) -> u64 {
    identity[3] & 0o170000
}
fn files_match(files: &[(PathBuf, Vec<u8>)]) -> bool {
    files.iter().all(|(path, bytes)| {
        std::fs::read(path).ok().as_deref() == Some(bytes.as_slice())
    })
}
/// Recheck the observed paths/bytes/context without enumerating directories.
/// This is not a reparse of policy or an atomic filesystem snapshot.
pub fn validate(evidence: &Evidence) -> bool {
    if environment() != evidence.env
        || std::env::current_dir().ok().as_ref() != Some(&evidence.cwd)
    {
        return false;
    }
    if !evidence
        .stamps
        .iter()
        .all(|old| stamp(&old.path, old.link.is_none()).as_ref() == Some(old))
    {
        return false;
    }
    files_match(&evidence.files)
        && environment() == evidence.env
        && std::env::current_dir().ok().as_ref() == Some(&evidence.cwd)
}

struct Capture<'a> {
    mode: CaptureMode,
    limits: Limits,
    stats: Stats,
    stamps: Vec<Stamp>,
    indices: HashMap<PathBuf, usize>,
    roots: Vec<PathBuf>,
    env: Vec<(OsString, OsString)>,
    cwd: Option<PathBuf>,
    expected: Option<&'a ExpectedConfig>,
    configs: u64,
    hook: &'a mut dyn FnMut(Event),
}
impl<'a> Capture<'a> {
    fn new(
        limits: Limits,
        expected: Option<&'a ExpectedConfig>,
        hook: &'a mut dyn FnMut(Event),
        mode: CaptureMode,
    ) -> Self {
        let env = environment();
        let cwd = std::env::current_dir().ok();
        let mut stats = Stats::default();
        if cwd.is_none()
            || expected.is_some_and(|e| {
                e.env != env || Some(&e.cwd) != cwd.as_ref() || !files_match(&e.files)
            })
        {
            stats.declined = true;
        }
        Self {
            mode,
            limits,
            stats,
            stamps: Vec::new(),
            indices: HashMap::new(),
            roots: Vec::new(),
            env,
            cwd,
            expected,
            configs: 0,
            hook,
        }
    }
    fn increment(value: &mut u64) -> bool {
        if let Some(next) = value.checked_add(1) {
            *value = next;
            true
        } else {
            *value = u64::MAX;
            false
        }
    }
    fn visit(&mut self, path: &Path) {
        if !Self::increment(&mut self.stats.visits) {
            self.stats.declined = true;
        }
        let bytes = u64::try_from(path.as_os_str().as_encoded_bytes().len()).ok();
        if let Some(next) = bytes.and_then(|n| self.stats.path_bytes.checked_add(n)) {
            self.stats.path_bytes = next;
        } else {
            self.stats.path_bytes = u64::MAX;
            self.stats.declined = true;
        }
        if u128::from(self.stats.visits) > self.limits.max_descriptors as u128
            || u128::from(self.stats.path_bytes) > self.limits.max_path_bytes as u128
        {
            self.stats.declined = true;
        }
    }
    fn remember(&mut self, path: &Path, is_root: bool) -> bool {
        if self.indices.contains_key(path) {
            return true;
        }
        if self.stats.declined {
            return false;
        }
        let Some(observed) = capture_stamp(path, is_root, self.mode, &mut self.stats) else {
            self.stats.declined = true;
            return false;
        };
        if observed
            .target
            .as_ref()
            .is_some_and(|m| !matches!(kind(m), 0o100000 | 0o040000))
        {
            self.stats.declined = true;
            return false;
        }
        // Match the original probe's retained classes. Every entry still
        // consumed a global visit and got its initial observation above.
        let candidate = matches!(
            path.extension().and_then(|x| x.to_str()),
            Some("ttf" | "ttc" | "TTF" | "TTC" | "otf" | "otc" | "OTF" | "OTC")
        );
        let directory = observed.target.as_ref().is_some_and(|m| kind(m) == 0o040000);
        if !is_root && !directory && !candidate && observed.link == observed.target {
            return true;
        }
        if OMIT_WITNESS && candidate { return true; }
        if self.stamps.try_reserve(1).is_err() || self.indices.try_reserve(1).is_err() {
            self.stats.declined = true;
            return false;
        }
        let Some(key) = try_path(path) else {
            self.stats.declined = true;
            return false;
        };
        self.indices.insert(key, self.stamps.len());
        self.stamps.push(observed);
        true
    }
    fn auxiliary(&mut self, path: &Path) -> bool {
        if !self.indices.contains_key(path) {
            self.visit(path);
        }
        self.remember(path, false)
    }
    fn finish(mut self, db: fontdb::Database) -> Attempt {
        (self.hook)(Event::Finished);
        if let Some(expected) = self.expected {
            if self.configs != 1 || self.roots != expected.roots {
                self.stats.declined = true;
            }
        }
        let evidence = if !self.stats.declined {
            if let Some(cwd) = self.cwd.take() {
                let evidence = Evidence {
                    stamps: self.stamps,
                    roots: self.roots,
                    env: self.env,
                    cwd,
                    files: self.expected.map(|e| e.files.clone()).unwrap_or_default(),
                };
                if OMIT_SELFCHECK || validate(&evidence) {
                    (self.hook)(Event::Validated);
                    Some(evidence)
                } else {
                    self.stats.declined = true;
                    None
                }
            } else {
                self.stats.declined = true;
                None
            }
        } else {
            None
        };
        Attempt { db, evidence, stats: self.stats }
    }
}
impl fontdb::DiscoveryObserver for Capture<'_> {
    fn root(&mut self, path: &Path) {
        self.visit(path);
        if !self.stats.declined {
            if self.roots.try_reserve(1).is_err() {
                self.stats.declined = true;
            } else if let Some(root) = try_path(path) {
                self.roots.push(root);
            } else {
                self.stats.declined = true;
            }
        }
        if self.remember(path, true) {
            (self.hook)(Event::RootCaptured(path.to_path_buf()));
        }
    }
    fn entry(&mut self, path: &Path) {
        if !Self::increment(&mut self.stats.entry_callbacks) {
            self.stats.declined = true;
        }
        self.visit(path);
        if self.remember(path, false) {
            (self.hook)(Event::EntryCaptured(path.to_path_buf()));
        }
    }
    fn directory(&mut self, path: &Path, is_root: bool) {
        if !Self::increment(&mut self.stats.directory_attempts) {
            self.stats.declined = true;
        }
        let remembered =
            if is_root { self.remember(path, true) } else { self.auxiliary(path) };
        if remembered {
            (self.hook)(Event::DirectoryCaptured(path.to_path_buf()));
        }
    }
    fn resolved(&mut self, lexical: &Path, canonical: &Path) {
        if canonical != lexical && self.auxiliary(canonical) {
            let is_dir = self
                .indices
                .get(canonical)
                .and_then(|i| self.stamps.get(*i))
                .and_then(|s| s.target.as_ref())
                .is_some_and(|m| kind(m) == 0o040000);
            (self.hook)(if is_dir {
                Event::DirectoryCaptured(canonical.to_path_buf())
            } else {
                Event::EntryCaptured(canonical.to_path_buf())
            });
        }
    }
    fn file_before(&mut self, path: &Path) {
        if !Self::increment(&mut self.stats.file_attempts) {
            self.stats.declined = true;
        }
        self.auxiliary(path);
        (self.hook)(Event::FileBefore(path.to_path_buf()));
    }
    fn file_after(&mut self, path: &Path) {
        (self.hook)(Event::FileAfter(path.to_path_buf()));
    }
    fn error(
        &mut self,
        path: &Path,
        operation: &'static str,
        error: &io::Error,
        is_root: bool,
    ) {
        let old = self.indices.get(path).and_then(|i| self.stamps.get(*i));
        let explained_root = is_root
            && operation == "read_dir"
            && old.is_some_and(|s| match error.kind() {
                io::ErrorKind::NotFound => s.target.is_none(),
                io::ErrorKind::NotADirectory => {
                    s.target.as_ref().is_some_and(|m| kind(m) != 0o040000)
                }
                _ => false,
            });
        let explained_dangling = matches!(operation, "follow" | "canonicalize")
            && error.kind() == io::ErrorKind::NotFound
            && old.is_some_and(|s| {
                s.target.is_none() && s.link.as_ref().is_some_and(|m| kind(m) == 0o120000)
            });
        if !explained_root && !explained_dangling {
            self.stats.declined = true;
        }
    }
    fn config(&mut self, aliases: &[String], files: &[PathBuf], valid: bool) {
        if !Self::increment(&mut self.configs) {
            self.stats.declined = true;
        }
        if !valid
            || self.expected.is_none_or(|e| {
                aliases != e.aliases.as_slice()
                    || files
                        .iter()
                        .map(PathBuf::as_path)
                        .ne(e.files.iter().map(|(p, _)| p.as_path()))
            })
        {
            self.stats.declined = true;
        }
        (self.hook)(Event::ConfigObserved);
    }
}

pub fn capture_dirs_with_mode(
    roots: &[PathBuf],
    limits: Limits,
    hook: &mut dyn FnMut(Event),
    mode: CaptureMode,
) -> Attempt {
    let mut capture = Capture::new(limits, None, hook, mode);
    let mut db = fontdb::Database::new();
    db.load_fonts_dirs_observed(roots, &mut capture);
    capture.finish(db)
}
pub fn capture_system_with_mode(
    expected: &ExpectedConfig,
    limits: Limits,
    hook: &mut dyn FnMut(Event),
    mode: CaptureMode,
) -> Attempt {
    let mut capture = Capture::new(limits, Some(expected), hook, mode);
    let mut db = fontdb::Database::new();
    #[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
    db.load_system_fonts_observed(&mut capture);
    #[cfg(not(all(unix, not(any(target_os = "macos", target_os = "android")))))]
    {
        capture.stats.declined = true;
        db.load_system_fonts();
    }
    capture.finish(db)
}
pub fn capture_config_file_with_mode(
    path: &Path,
    expected: &ExpectedConfig,
    limits: Limits,
    hook: &mut dyn FnMut(Event),
    mode: CaptureMode,
) -> Attempt {
    let mut capture = Capture::new(limits, Some(expected), hook, mode);
    let mut db = fontdb::Database::new();
    #[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
    db.load_config_file_observed(path, &mut capture);
    #[cfg(not(all(unix, not(any(target_os = "macos", target_os = "android")))))]
    {
        let _ = path;
        capture.stats.declined = true;
        db.load_system_fonts();
    }
    capture.finish(db)
}

/// Default capture uses the minimum non-symlink observation policy.
pub fn capture_dirs(roots: &[PathBuf], limits: Limits, hook: &mut dyn FnMut(Event)) -> Attempt {
    capture_dirs_with_mode(roots, limits, hook, CaptureMode::SingleRegular)
}

/// Default capture uses the minimum non-symlink observation policy.
pub fn capture_system(expected: &ExpectedConfig, limits: Limits, hook: &mut dyn FnMut(Event)) -> Attempt {
    capture_system_with_mode(expected, limits, hook, CaptureMode::SingleRegular)
}

/// Default capture uses the minimum non-symlink observation policy.
pub fn capture_config_file(path: &Path, expected: &ExpectedConfig, limits: Limits, hook: &mut dyn FnMut(Event)) -> Attempt {
    capture_config_file_with_mode(path, expected, limits, hook, CaptureMode::SingleRegular)
}

/// Real cold-session barrier controller. Nothing is published on refusal.
/// Discovery includes G1; the subsequent order is G2, key, build, G3, policy.
/// Config/evidence callbacks alone are not a complete policy proof.
pub fn establish<F, P, K, C>(
    discover: impl FnOnce() -> Option<(F, P)>,
    check_prepared: impl FnOnce(&P) -> bool,
    key_of: impl FnOnce(&P) -> K,
    build: impl FnOnce(P) -> C,
    check_key: impl FnOnce(&K) -> bool,
    check_policy: impl FnOnce(&F) -> bool,
) -> Option<(F, K, C)> {
    let (fast, prepared) = discover()?;
    if !OMIT_G2 && !check_prepared(&prepared) { return None; }
    let key = key_of(&prepared);
    let catalog = build(prepared);
    if !OMIT_G3 && !check_key(&key) { return None; }
    if !OMIT_POLICY && !check_policy(&fast) { return None; }
    Some((fast, key, catalog))
}
