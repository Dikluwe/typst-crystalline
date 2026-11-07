//! Independent mode/counter tests; real I/O in RAM, --test-threads=1.
#![cfg(target_os = "linux")]
use joint_font_witness::{
    capture_dirs, capture_dirs_with_mode, capture_system_with_mode,
    capture_config_file_with_mode, establish, validate, Attempt, CaptureMode,
    Event, ExpectedConfig, Limits,
};
use std::ffi::OsString;
use std::fs::{self, File, FileTimes};
use std::os::unix::{ffi::OsStringExt, fs::{symlink, MetadataExt, PermissionsExt}};
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

const NOTO: &[u8] = include_bytes!("../fixtures/NotoSans-Regular.ttf");
const DEJAVU: &[u8] = include_bytes!("../fixtures/DejaVuSans.ttf");
fn modes() -> [CaptureMode; 2] { [CaptureMode::Legacy, CaptureMode::SingleRegular] }
struct Fixture { dir: PathBuf, env: Vec<(&'static str, Option<OsString>)>, cwd: PathBuf }
impl Fixture {
    fn new() -> Self {
        let base = std::env::var_os("TMPDIR").map(PathBuf::from)
            .unwrap_or_else(|| "/dev/shm/tekt-joint-witness-20261106".into());
        assert!(base.is_absolute() && base.starts_with("/dev/shm/tekt-joint-witness-20261106"));
        fs::create_dir_all(&base).unwrap();
        assert!(fs::canonicalize(&base).unwrap().starts_with("/dev/shm/tekt-joint-witness-20261106"));
        let mut n = 0;
        let dir = loop {
            let p = base.join(format!("capture-b-{}-{n}", std::process::id()));
            match fs::create_dir(&p) {
                Ok(()) => break p,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => n += 1,
                Err(e) => panic!("RAM fixture: {e}"),
            }
        };
        let env = ["FONTCONFIG_FILE", "HOME", "XDG_CONFIG_HOME", "TEKT_WITNESS_B_CONTEXT"]
            .into_iter().map(|key| (key, std::env::var_os(key))).collect();
        Self { dir, env, cwd: std::env::current_dir().unwrap() }
    }
    fn sub(&self, name: &str) -> PathBuf { let p = self.dir.join(name); fs::create_dir_all(&p).unwrap(); p }
    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let p = self.dir.join(name); fs::create_dir_all(p.parent().unwrap()).unwrap(); fs::write(&p, bytes).unwrap(); p
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.cwd).unwrap();
        for (key, value) in &self.env {
            match value { Some(value) => std::env::set_var(key, value), None => std::env::remove_var(key) }
        }
        let _ = fs::remove_dir_all(&self.dir);
    }
}
fn capture(roots: &[PathBuf], mode: CaptureMode) -> Attempt {
    capture_dirs_with_mode(roots, Limits::default(), &mut |_| {}, mode)
}
fn accepted(out: &Attempt) {
    assert!(!out.stats.declined);
    assert!(out.evidence.is_some());
    assert!(validate(out.evidence.as_ref().unwrap()));
}
fn declined(out: &Attempt) { assert!(out.stats.declined); assert!(out.evidence.is_none()); }
fn catalog(db: &fontdb::Database) -> Vec<String> {
    db.faces().map(|face| {
        let path = match &face.source {
            fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => path,
            _ => panic!("native filesystem discovery must retain file sources"),
        };
        format!("{:?}|{}|{:?}|{:?}|{:?}|{:?}|{}", path,
            face.index, face.families, face.style, face.weight, face.stretch, face.monospaced)
    }).collect()
}
fn oracle(roots: &[PathBuf]) -> fontdb::Database { let mut db = fontdb::Database::new(); db.load_fonts_dirs_reference(roots); db }
fn xml(body: &str) -> Vec<u8> { format!("<?xml version=\"1.0\"?><fontconfig>{body}</fontconfig>").into_bytes() }
fn dir_xml(root: &Path) -> String { format!("<dir>{}</dir>", root.display()) }
fn expected(path: &Path) -> ExpectedConfig {
    let mut config = fontconfig_parser::FontConfig::default();
    config.merge_config(path).unwrap();
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let roots = config.dirs.iter().map(|d| d.path.strip_prefix("~").ok().map(|p| home.as_ref().unwrap().join(p)).unwrap_or_else(|| d.path.clone())).collect();
    let aliases = config.aliases.iter().map(|alias| format!("{alias:?}")).collect();
    let mut paths: Vec<_> = config.config_files.into_iter().collect(); paths.sort();
    let files = paths.into_iter().map(|p| { let bytes = fs::read(&p).unwrap(); (p, bytes) }).collect();
    let mut env: Vec<_> = std::env::vars_os().collect(); env.sort_by(|a, b| a.0.cmp(&b.0));
    ExpectedConfig { roots, aliases, files, env, cwd: std::env::current_dir().unwrap() }
}
fn same_policy(a: &ExpectedConfig, b: &ExpectedConfig) -> bool {
    a.roots == b.roots && a.aliases == b.aliases && a.files == b.files && a.env == b.env && a.cwd == b.cwd
}

#[test]
fn default_is_single_regular_and_counters_exclude_independent_selfcheck() {
    let f = Fixture::new(); let root = f.sub("root"); f.file("root/a.ttf", NOTO);
    let roots = [root];
    let legacy = capture(&roots, CaptureMode::Legacy);
    let single = capture(&roots, CaptureMode::SingleRegular);
    let default = capture_dirs(&roots, Limits::default(), &mut |_| {});
    for out in [&legacy, &single, &default] { accepted(out); assert_eq!(catalog(&out.db), catalog(&oracle(&roots))); }
    assert_eq!(legacy.evidence.as_ref().unwrap().stamps(), single.evidence.as_ref().unwrap().stamps());
    assert_eq!(single.evidence.as_ref().unwrap().stamps(), default.evidence.as_ref().unwrap().stamps());
    assert_eq!(legacy.stats.capture_lstat, 2);
    assert_eq!(legacy.stats.capture_stat, 2);
    assert_eq!(single.stats.capture_lstat, 2);
    assert_eq!(single.stats.capture_stat, 0);
    assert_eq!(default.stats.capture_lstat, single.stats.capture_lstat);
    assert_eq!(default.stats.capture_stat, single.stats.capture_stat);
}

#[test]
fn both_modes_keep_native_order_duplicate_roots_lexical_and_non_utf8_witnesses() {
    let f = Fixture::new(); let root = f.sub("root");
    f.file("root/a.ttf", NOTO); f.file("root/b.ttf", DEJAVU);
    let strange = root.join(OsString::from_vec(b"unparsed-\xff.ttf".to_vec())); fs::write(&strange, b"not font").unwrap();
    let lexical = root.join("..").join("root");
    let roots = [lexical.clone(), root.clone(), lexical];
    let wanted = catalog(&oracle(&roots));
    for mode in modes() {
        let out = capture(&roots, mode); accepted(&out);
        assert_eq!(catalog(&out.db), wanted);
        assert_eq!(out.evidence.as_ref().unwrap().roots(), &roots);
        assert!(out.evidence.as_ref().unwrap().stamps().iter().any(|s| s.path.as_os_str().as_encoded_bytes().ends_with(b"unparsed-\xff.ttf")));
        assert!(out.evidence.as_ref().unwrap().stamps().iter().any(|s| s.path == root));
    }
}

#[test]
fn symlinks_still_observe_targets_and_capture_counters_count_real_attempts() {
    let f = Fixture::new(); let root = f.sub("root");
    let target = f.file("outside/a.ttf", NOTO);
    let link = root.join("no-extension"); symlink(&target, &link).unwrap();
    for mode in modes() {
        let out = capture(std::slice::from_ref(&root), mode); accepted(&out);
        let evidence = out.evidence.as_ref().unwrap();
        assert!(evidence.stamps().iter().any(|s| s.path == link && s.link != s.target));
        assert!(evidence.stamps().iter().any(|s| s.path == target));
        let link_count = evidence.stamps().iter().filter(|s| fs::symlink_metadata(&s.path).unwrap().file_type().is_symlink()).count() as u64;
        assert_eq!(out.stats.capture_lstat, evidence.stamps().len() as u64);
        match mode {
            CaptureMode::Legacy => assert_eq!(out.stats.capture_stat, out.stats.capture_lstat),
            CaptureMode::SingleRegular => assert_eq!(out.stats.capture_stat, link_count),
        }
        assert_eq!(catalog(&out.db), catalog(&oracle(std::slice::from_ref(&root))));
    }
    let out = capture(std::slice::from_ref(&root), CaptureMode::SingleRegular); accepted(&out);
    fs::write(&target, DEJAVU).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
}

#[test]
fn missing_and_dangling_paths_keep_negative_witnesses_and_attempt_counts() {
    for mode in modes() {
        let f = Fixture::new(); let missing = f.dir.join("missing");
        let out = capture(std::slice::from_ref(&missing), mode); accepted(&out);
        assert_eq!(out.stats.capture_lstat, 1); assert_eq!(out.stats.capture_stat, 0);
        assert_eq!(out.evidence.as_ref().unwrap().stamps()[0].link, None);
        fs::create_dir(&missing).unwrap();
        assert!(!validate(out.evidence.as_ref().unwrap()));
        let target = f.dir.join("absent.ttf"); let link = missing.join("link.ttf"); symlink(&target, &link).unwrap();
        let dangling = capture(std::slice::from_ref(&missing), mode); accepted(&dangling);
        assert!(dangling.evidence.as_ref().unwrap().stamps().iter().any(|s| s.path == link && s.target.is_none()));
        fs::write(&target, NOTO).unwrap();
        assert!(!validate(dangling.evidence.as_ref().unwrap()));
    }
}

#[test]
fn invalid_font_becomes_valid_and_restored_mtime_ctime_still_invalidates_both_modes() {
    for mode in modes() {
        let f = Fixture::new(); let root = f.sub("root"); let font = f.file("root/a.ttf", b"invalid");
        let before = capture(std::slice::from_ref(&root), mode); accepted(&before);
        fs::write(&font, NOTO).unwrap(); assert!(!validate(before.evidence.as_ref().unwrap()));
        let file = File::options().write(true).open(&font).unwrap();
        let modified = UNIX_EPOCH + Duration::from_secs(42);
        file.set_times(FileTimes::new().set_modified(modified)).unwrap();
        let valid = capture(std::slice::from_ref(&root), mode); accepted(&valid);
        let old = fs::metadata(&font).unwrap();
        let mut changed = NOTO.to_vec(); let last = changed.len() - 1; changed[last] ^= 1;
        fs::write(&font, changed).unwrap(); file.set_times(FileTimes::new().set_modified(modified)).unwrap();
        let new = fs::metadata(&font).unwrap();
        assert_eq!(old.len(), new.len()); assert_eq!(old.modified().unwrap(), new.modified().unwrap());
        assert_ne!((old.ctime(), old.ctime_nsec()), (new.ctime(), new.ctime_nsec()));
        assert!(!validate(valid.evidence.as_ref().unwrap()));
    }
}

#[test]
fn exact_global_limits_and_declines_do_not_truncate_either_native_catalog() {
    let f = Fixture::new(); let root = f.sub("root"); f.file("root/a.ttf", NOTO); f.file("root/b.ttf", DEJAVU);
    for i in 0..8 { f.file(&format!("root/invalid-{i}"), b"invalid"); }
    let roots = [root.clone(), root]; let wanted = catalog(&oracle(&roots));
    for mode in modes() {
        let complete = capture(&roots, mode); accepted(&complete);
        let visits = complete.stats.visits as usize; let bytes = complete.stats.path_bytes as usize;
        let exact = capture_dirs_with_mode(&roots, Limits { max_descriptors: visits, max_path_bytes: bytes }, &mut |_| {}, mode);
        accepted(&exact); assert_eq!(catalog(&exact.db), wanted);
        for limits in [Limits { max_descriptors: visits - 1, max_path_bytes: usize::MAX }, Limits { max_descriptors: usize::MAX, max_path_bytes: bytes - 1 }] {
            let out = capture_dirs_with_mode(&roots, limits, &mut |_| {}, mode); declined(&out); assert_eq!(catalog(&out.db), wanted);
        }
    }
}

#[test]
fn both_modes_keep_selfcheck_and_hooks_at_file_and_discovery_boundaries() {
    for mode in modes() {
        for at_file in [false, true] {
            let f = Fixture::new(); let root = f.sub("root"); let path = f.file("root/a.ttf", NOTO);
            let before_catalog = catalog(&oracle(std::slice::from_ref(&root)));
            let mut fired = false;
            let out = capture_dirs_with_mode(std::slice::from_ref(&root), Limits::default(), &mut |event| {
                let selected = if at_file { matches!(event, Event::FileBefore(_)) } else { event == Event::Finished };
                if selected && !fired { fired = true; fs::write(&path, DEJAVU).unwrap(); }
            }, mode);
            assert!(fired); declined(&out);
            let expected_catalog = if at_file { catalog(&oracle(std::slice::from_ref(&root))) } else { before_catalog };
            assert_eq!(catalog(&out.db), expected_catalog);
        }
    }
}

#[test]
fn both_modes_keep_complete_mode_subsecond_context_and_permission_guards() {
    for mode in modes() {
        let f = Fixture::new(); let root = f.sub("root"); let p = f.file("root/a.ttf", NOTO);
        // Keep a writable descriptor acquired before testing the read-only mode.
        let file = File::options().write(true).open(&p).unwrap();
        let before = capture(std::slice::from_ref(&root), mode); accepted(&before);
        fs::set_permissions(&p, fs::Permissions::from_mode(0o400)).unwrap();
        assert!(!validate(before.evidence.as_ref().unwrap()));
        let next = capture(std::slice::from_ref(&root), mode); accepted(&next);
        file.set_times(FileTimes::new().set_modified(UNIX_EPOCH - Duration::new(2, 123_456_789))).unwrap();
        assert!(!validate(next.evidence.as_ref().unwrap()));
        let context = capture(std::slice::from_ref(&root), mode); accepted(&context);
        std::env::set_var("TEKT_WITNESS_B_CONTEXT", "changed");
        assert!(!validate(context.evidence.as_ref().unwrap()));
        std::env::remove_var("TEKT_WITNESS_B_CONTEXT");
        let context = capture(std::slice::from_ref(&root), mode); accepted(&context);
        std::env::set_current_dir(&root).unwrap();
        assert!(!validate(context.evidence.as_ref().unwrap()));
    }
}

#[test]
fn explicit_config_and_system_apis_preserve_native_catalog_in_both_modes() {
    let f = Fixture::new(); let root = f.sub("root"); f.file("root/a.ttf", NOTO);
    let config = f.file("fonts.conf", &xml(&dir_xml(&root)));
    std::env::set_var("FONTCONFIG_FILE", &config);
    let wanted = expected(&config);
    let mut native = fontdb::Database::new(); native.load_system_fonts();
    for mode in modes() {
        let explicit = capture_config_file_with_mode(&config, &wanted, Limits::default(), &mut |_| {}, mode); accepted(&explicit);
        assert_eq!(catalog(&explicit.db), catalog(&oracle(&wanted.roots)));
        let system = capture_system_with_mode(&wanted, Limits::default(), &mut |_| {}, mode); accepted(&system);
        assert_eq!(catalog(&system.db), catalog(&native));
    }
}

#[test]
fn incomplete_policy_witness_requires_actual_reparse_before_establish_can_publish() {
    for mode in modes() {
        for include_directory in [false, true] {
            let f = Fixture::new(); let root = f.sub("before"); let after = f.sub("after");
            f.file("before/a.ttf", NOTO); f.file("after/b.ttf", DEJAVU);
            let policy_dir = f.sub("policy/conf.d");
            let added = if include_directory { policy_dir.join("50-new.conf") } else { f.dir.join("policy/missing.conf") };
            let include = if include_directory { &policy_dir } else { &added };
            let config = f.file("policy/fonts.conf", &xml(&format!("{}<include ignore_missing=\"yes\">{}</include>", dir_xml(&root), include.display())));
            let old = expected(&config);
            let mut fired = false;
            let out = capture_config_file_with_mode(&config, &old, Limits::default(), &mut |event| {
                if event == Event::Validated && !fired {
                    fired = true;
                    fs::write(&added, xml(&format!("{}<alias><family>serif</family><prefer><family>NewFamily</family></prefer></alias>", dir_xml(&after)))).unwrap();
                }
            }, mode);
            assert!(fired); accepted(&out);
            let fresh = expected(&config);
            assert!(!same_policy(&old, &fresh));
            // All captured bytes and stamps still validate. They deliberately
            // do not claim completeness for previously absent policy inputs.
            let evidence = out.evidence.unwrap(); assert!(validate(&evidence));
            let result = establish(
                || Some((old, evidence)),
                validate,
                |_| (),
                |_| "catalog-not-yet-published",
                |_| true,
                |loaded| same_policy(loaded, &expected(&config)),
            );
            assert!(result.is_none(), "policy must be reparsed, not replaced by witness validation");
        }
    }
}

#[test]
fn invalid_kernel_paths_and_unreadable_roots_never_fabricate_evidence() {
    for mode in modes() {
        let invalid = PathBuf::from(OsString::from_vec(b"bad\0kernel-path".to_vec()));
        let out = capture(&[invalid], mode); declined(&out);
        let f = Fixture::new(); let root = f.sub("locked"); f.file("locked/a.ttf", NOTO);
        fs::set_permissions(&root, fs::Permissions::from_mode(0)).unwrap();
        if fs::read_dir(&root).is_err() {
            declined(&capture(std::slice::from_ref(&root), mode));
        }
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

#[test]
fn real_evidence_refuses_changes_at_g2_and_g3_before_any_publication() {
    for mode in modes() {
        for gate in ["g2", "g3"] {
            let f = Fixture::new(); let root = f.sub("root"); let path = f.file("root/a.ttf", NOTO);
            let out = capture(std::slice::from_ref(&root), mode); accepted(&out);
            let evidence = out.evidence.unwrap();
            let builds = std::cell::Cell::new(0);
            let result = establish(
                || {
                    if gate == "g2" { fs::write(&path, DEJAVU).unwrap(); }
                    Some(((), evidence))
                },
                validate,
                |evidence| evidence.clone(),
                |_| {
                    builds.set(builds.get() + 1);
                    if gate == "g3" { fs::write(&path, DEJAVU).unwrap(); }
                    "unpublished-catalog"
                },
                validate,
                |_| panic!("policy must not run after a failed freshness gate"),
            );
            assert!(result.is_none(), "changed real evidence must fail {gate}");
            assert_eq!(builds.get(), usize::from(gate == "g3"));
        }
    }
}
