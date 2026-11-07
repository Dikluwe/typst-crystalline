//! Independent contract tests. Run with --test-threads=1; all temporary I/O is RAM.
#![cfg(target_os = "linux")]
use joint_font_witness::{
    capture_config_file, capture_dirs, validate, Attempt, Event, ExpectedConfig, Limits,
    Stamp,
};
use std::os::unix::{
    ffi::OsStringExt,
    fs::{symlink, MetadataExt, PermissionsExt},
};
use std::{
    ffi::OsString,
    fs::{self, File, FileTimes},
    io,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
    time::{Duration, UNIX_EPOCH},
};

static SERIAL: Mutex<()> = Mutex::new(());
const NOTO: &[u8] = include_bytes!("../fixtures/NotoSans-Regular.ttf");
const DEJAVU: &[u8] = include_bytes!("../fixtures/DejaVuSans.ttf");
struct Fixture {
    dir: PathBuf,
    env: Vec<(&'static str, Option<OsString>)>,
    cwd: PathBuf,
    _guard: MutexGuard<'static, ()>,
}
impl Fixture {
    fn new() -> Self {
        let guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let base = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/dev/shm/tekt-joint-discovery-20261105"));
        assert!(
            base.is_absolute() && base.starts_with("/dev/shm"),
            "test TMPDIR must be absolute RAM"
        );
        fs::create_dir_all(&base).unwrap();
        let mut n = 0u64;
        let dir = loop {
            let p = base.join(format!("joint-b-{}-{n}", std::process::id()));
            match fs::create_dir(&p) {
                Ok(()) => break p,
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => n += 1,
                Err(e) => panic!("RAM fixture: {e}"),
            }
        };
        let env = ["TEKT_JOINT_B_CONTEXT", "FONTCONFIG_FILE", "HOME", "XDG_CONFIG_HOME"]
            .into_iter()
            .map(|k| (k, std::env::var_os(k)))
            .collect();
        Self {
            dir,
            env,
            cwd: std::env::current_dir().unwrap(),
            _guard: guard,
        }
    }
    fn sub(&self, name: &str) -> PathBuf {
        let p = self.dir.join(name);
        fs::create_dir_all(&p).unwrap();
        p
    }
    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let p = self.dir.join(name);
        if let Some(p) = p.parent() {
            fs::create_dir_all(p).unwrap();
        }
        fs::write(&p, bytes).unwrap();
        p
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.cwd).unwrap();
        for (k, v) in &self.env {
            match v {
                Some(v) => std::env::set_var(k, v),
                None => std::env::remove_var(k),
            }
        }
        let _ = fs::remove_dir_all(&self.dir);
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Face {
    path: PathBuf,
    index: u32,
    families: String,
    style: String,
    weight: String,
    stretch: String,
    monospace: bool,
}
fn catalog(db: &fontdb::Database) -> Vec<Face> {
    db.faces()
        .map(|f| Face {
            path: match &f.source {
                fontdb::Source::File(p) => p.clone(),
                fontdb::Source::SharedFile(p, _) => p.clone(),
                _ => panic!("file discovery must preserve file sources"),
            },
            index: f.index,
            families: format!("{:?}", f.families),
            style: format!("{:?}", f.style),
            weight: format!("{:?}", f.weight),
            stretch: format!("{:?}", f.stretch),
            monospace: f.monospaced,
        })
        .collect()
}
fn reference(roots: &[PathBuf]) -> fontdb::Database {
    let mut db = fontdb::Database::new();
    db.load_fonts_dirs_reference(roots);
    db
}
fn capture(roots: &[PathBuf]) -> Attempt {
    capture_dirs(roots, Limits::default(), &mut |_| {})
}
fn accepted(a: &Attempt) {
    assert!(
        a.evidence.is_some(),
        "complete stable fixture must have evidence: {:?}",
        a.stats
    );
    assert!(!a.stats.declined);
    assert!(validate(a.evidence.as_ref().unwrap()));
}
fn declined(a: &Attempt) {
    assert!(a.evidence.is_none());
    assert!(a.stats.declined);
}
fn identity(m: &fs::Metadata) -> [u64; 8] {
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
fn stamp(path: &Path) -> Stamp {
    let m = fs::symlink_metadata(path).unwrap();
    Stamp {
        path: path.to_owned(),
        link: Some(identity(&m)),
        target: fs::metadata(path).ok().map(|m| identity(&m)),
    }
}
fn expected(path: &Path) -> ExpectedConfig {
    let mut c = fontconfig_parser::FontConfig::default();
    c.merge_config(path).unwrap();
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let roots = c
        .dirs
        .iter()
        .map(|d| {
            if let Ok(rest) = d.path.strip_prefix("~") {
                home.as_ref().unwrap().join(rest)
            } else {
                d.path.clone()
            }
        })
        .collect();
    let aliases = c.aliases.iter().map(|a| format!("{a:?}")).collect();
    let mut paths: Vec<_> = c.config_files.into_iter().collect();
    paths.sort();
    let files = paths
        .into_iter()
        .map(|p| {
            let b = fs::read(&p).unwrap();
            (p, b)
        })
        .collect();
    let mut env: Vec<_> = std::env::vars_os().collect();
    env.sort_by(|a, b| a.0.cmp(&b.0));
    ExpectedConfig {
        roots,
        aliases,
        files,
        env,
        cwd: std::env::current_dir().unwrap(),
    }
}
fn xml(body: &str) -> Vec<u8> {
    format!("<?xml version=\"1.0\"?><!DOCTYPE fontconfig SYSTEM \"fonts.dtd\"><fontconfig>{body}</fontconfig>").into_bytes()
}
fn dir_xml(path: &Path) -> String {
    assert!(!path.as_os_str().as_encoded_bytes().contains(&b'&'));
    format!("<dir>{}</dir>", path.display())
}

#[test]
fn default_limits_match_the_generation_contract() {
    let l = Limits::default();
    assert_eq!(l.max_descriptors, 16384);
    assert_eq!(l.max_path_bytes, 1048576);
}
#[test]
fn native_catalog_order_metadata_and_shared_seen_across_duplicate_roots() {
    let f = Fixture::new();
    let a = f.sub("a");
    let b = f.sub("b");
    f.file("a/n.ttf", NOTO);
    f.file("b/d.ttf", DEJAVU);
    let roots = vec![b.clone(), a.clone(), b.clone(), a];
    let out = capture(&roots);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&roots)));
    assert_eq!(out.evidence.as_ref().unwrap().roots(), roots.as_slice());
}
#[test]
fn all_eight_stamp_fields_and_lexical_non_utf8_paths_are_preserved() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = root.join(OsString::from_vec(b"font-\xff.ttf".to_vec()));
    fs::write(&p, NOTO).unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(
        out.stats.visits, 2,
        "one root and one lexical entry; no new canonical auxiliary path"
    );
    assert_eq!(
        out.stats.path_bytes,
        (root.as_os_str().as_encoded_bytes().len()
            + p.as_os_str().as_encoded_bytes().len()) as u64,
        "budgets count raw Unix bytes, not lossy UTF-8 lengths"
    );
    assert_eq!(catalog(&out.db), catalog(&reference(&[root.clone()])));
    for path in [&root, &p] {
        assert!(
            out.evidence.as_ref().unwrap().stamps().contains(&stamp(path)),
            "missing exact stamp {path:?}"
        );
    }
}
#[test]
fn empty_unknown_and_invalid_fonts_remain_observed_then_become_valid() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = f.file("root/invalid.ttf", b"not font");
    f.file("root/ordinary.txt", b"x");
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(out.db.faces().count(), 0);
    assert!(out.evidence.as_ref().unwrap().stamps().iter().any(|s| s.path == p));
    fs::write(&p, NOTO).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
    let next = capture(&[root.clone()]);
    accepted(&next);
    assert_eq!(catalog(&next.db), catalog(&reference(&[root])));
    assert_eq!(next.db.faces().count(), 1);
}
#[test]
fn absent_and_non_directory_roots_preserve_native_empty_result_and_negative_proof() {
    let f = Fixture::new();
    let missing = f.dir.join("missing");
    let regular = f.file("regular.ttf", NOTO);
    let roots = vec![missing.clone(), regular.clone()];
    let out = capture(&roots);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&roots)));
    assert!(catalog(&out.db).is_empty());
    let stamps = out.evidence.as_ref().unwrap().stamps();
    assert!(stamps.contains(&Stamp { path: missing.clone(), link: None, target: None }));
    assert!(stamps.contains(&stamp(&regular)));
    fs::create_dir(&missing).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
}
#[test]
fn no_extension_link_reaches_external_target_and_retains_both_observations() {
    let f = Fixture::new();
    let root = f.sub("root");
    let target = f.file("outside/font.ttf", NOTO);
    let link = root.join("no-extension");
    symlink(&target, &link).unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    assert_eq!(out.db.faces().count(), 1);
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&link)));
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&target)));
    fs::write(&target, b"invalid now").unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
}
#[test]
fn symlink_directory_traversal_outside_root_has_complete_proof() {
    let f = Fixture::new();
    let root = f.sub("root");
    let external = f.sub("outside/nested");
    let target = f.file("outside/nested/f.ttf", DEJAVU);
    let link = root.join("directory-alias");
    symlink(external.parent().unwrap(), &link).unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&link)));
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&target)));
    f.file("outside/nested/new.ttf", NOTO);
    assert!(!validate(out.evidence.as_ref().unwrap()));
}
#[test]
fn dangling_target_creation_and_retargeting_are_not_hidden_by_seen() {
    let f = Fixture::new();
    let root = f.sub("root");
    let target = f.dir.join("target.ttf");
    let link = root.join("dangling");
    symlink(&target, &link).unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(out.db.faces().count(), 0);
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&link)));
    fs::write(&target, NOTO).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
    let next = capture(&[root.clone()]);
    accepted(&next);
    let other = f.file("other.ttf", DEJAVU);
    fs::remove_file(&link).unwrap();
    symlink(&other, &link).unwrap();
    assert!(!validate(next.evidence.as_ref().unwrap()));
    assert_eq!(catalog(&capture(&[root.clone()]).db), catalog(&reference(&[root])));
}
#[test]
fn true_symlink_resolution_cycle_refuses_but_repair_is_not_negatively_cached() {
    let f = Fixture::new();
    let root = f.sub("root");
    let cycle = root.join("cycle");
    symlink("cycle", &cycle).unwrap();
    let out = capture(&[root.clone()]);
    declined(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root.clone()])));
    fs::remove_file(&cycle).unwrap();
    fs::write(root.join("valid.ttf"), NOTO).unwrap();
    let repaired = capture(&[root]);
    accepted(&repaired);
    assert_eq!(repaired.db.faces().count(), 1);
}
#[test]
fn directory_alias_cycle_uses_native_seen_without_truncating_the_catalog() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/font.ttf", NOTO);
    symlink(&root, root.join("back")).unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    assert!(out.db.faces().count() >= 1);
}
#[test]
fn parent_rename_and_atomic_inode_replacement_expire_evidence() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = f.file("root/font.ttf", NOTO);
    let out = capture(&[root.clone()]);
    accepted(&out);
    let replacement = f.file("replacement.ttf", NOTO);
    let old = fs::metadata(&p).unwrap();
    File::options()
        .write(true)
        .open(&replacement)
        .unwrap()
        .set_times(FileTimes::new().set_modified(old.modified().unwrap()))
        .unwrap();
    fs::rename(replacement, &p).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
    let next = capture(&[root.clone()]);
    accepted(&next);
    fs::rename(&root, f.dir.join("renamed")).unwrap();
    assert!(!validate(next.evidence.as_ref().unwrap()));
}
#[test]
fn restored_mtime_equal_size_edit_is_detected_by_ctime_without_a_race_sleep() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = f.file("root/font.ttf", NOTO);
    // Deliberately old mtime; ctime is advanced by controlled mutations below.
    let old_time = UNIX_EPOCH + Duration::from_secs(42);
    let h = File::options().write(true).open(&p).unwrap();
    h.set_times(FileTimes::new().set_modified(old_time)).unwrap();
    let out = capture(&[root]);
    accepted(&out);
    let old = identity(&fs::metadata(&p).unwrap());
    let mut bytes = NOTO.to_vec();
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    fs::write(&p, bytes).unwrap();
    h.set_times(FileTimes::new().set_modified(old_time)).unwrap();
    let new = identity(&fs::metadata(&p).unwrap());
    assert_eq!(old[2], new[2]);
    assert_eq!(&old[4..6], &new[4..6]);
    assert_ne!(&old[6..8], &new[6..8], "tmpfs must expose the scheduled edit in ctime");
    assert!(!validate(out.evidence.as_ref().unwrap()));
}
#[test]
fn complete_mode_and_signed_subsecond_times_participate_in_evidence() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = f.file("root/font.ttf", NOTO);
    let h = File::options().write(true).open(&p).unwrap();
    h.set_times(
        FileTimes::new().set_modified(UNIX_EPOCH - Duration::new(2, 123_456_789)),
    )
    .unwrap();
    let out = capture(&[root.clone()]);
    accepted(&out);
    assert!(out.evidence.as_ref().unwrap().stamps().contains(&stamp(&p)));
    fs::set_permissions(&p, fs::Permissions::from_mode(0o400)).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
    let next = capture(&[root]);
    accepted(&next);
    h.set_times(
        FileTimes::new().set_modified(UNIX_EPOCH - Duration::new(2, 123_456_788)),
    )
    .unwrap();
    assert!(!validate(next.evidence.as_ref().unwrap()));
}
#[test]
fn budgets_are_global_over_visits_and_bytes_without_catalog_truncation() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/a.ttf", NOTO);
    f.file("root/b.ttf", DEJAVU);
    for i in 0..12 {
        f.file(&format!("root/no-font-{i}"), b"x");
    }
    let roots = vec![root.clone(), root];
    let normal = capture(&roots);
    accepted(&normal);
    let expected = catalog(&reference(&roots));
    assert!(normal.stats.visits > normal.db.faces().count() as u64);
    assert!(normal.stats.path_bytes > 0);
    let visits = usize::try_from(normal.stats.visits).unwrap();
    let bytes = usize::try_from(normal.stats.path_bytes).unwrap();
    let exact = capture_dirs(
        &roots,
        Limits { max_descriptors: visits, max_path_bytes: bytes },
        &mut |_| {},
    );
    accepted(&exact);
    assert_eq!(catalog(&exact.db), expected);
    for limits in [
        Limits {
            max_descriptors: visits - 1,
            max_path_bytes: usize::MAX,
        },
        Limits {
            max_descriptors: usize::MAX,
            max_path_bytes: bytes - 1,
        },
        Limits { max_descriptors: 0, max_path_bytes: usize::MAX },
        Limits { max_descriptors: usize::MAX, max_path_bytes: 0 },
    ] {
        let mut file_events = 0;
        let out = capture_dirs(&roots, limits, &mut |e| {
            if matches!(e, Event::FileBefore(_)) {
                file_events += 1
            }
        });
        declined(&out);
        assert_eq!(catalog(&out.db), expected);
        assert!(file_events >= 2, "proof budget must not stop native callbacks");
        assert!(out.stats.visits >= normal.stats.visits);
        assert!(out.stats.path_bytes >= normal.stats.path_bytes);
    }
}
#[test]
fn root_captured_mutation_precedes_native_enumeration_and_refuses_stale_proof() {
    let f = Fixture::new();
    let root = f.sub("root");
    let new = root.join("late.ttf");
    let mut fired = false;
    let out = capture_dirs(&[root.clone()], Limits::default(), &mut |e| {
        if matches!(&e,Event::RootCaptured(p) if p==&root) && !fired {
            fired = true;
            fs::write(&new, NOTO).unwrap();
        }
    });
    assert!(fired);
    declined(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    assert_eq!(out.db.faces().count(), 1);
}
#[test]
fn file_before_mutation_keeps_native_load_result_and_refuses_old_stamp() {
    let f = Fixture::new();
    let root = f.sub("root");
    let p = f.file("root/font.ttf", NOTO);
    let mut fired = false;
    let out = capture_dirs(&[root.clone()], Limits::default(), &mut |e| {
        if matches!(&e,Event::FileBefore(path) if path==&p) && !fired {
            fired = true;
            fs::write(&p, b"now invalid").unwrap();
        }
    });
    assert!(fired);
    declined(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    assert_eq!(out.db.faces().count(), 0);
}
#[test]
fn finished_mutation_is_caught_by_selfcheck_but_validated_mutation_is_later() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/font.ttf", NOTO);
    for mutate_after_validated in [false, true] {
        let mut fired = false;
        let p =
            root.join(if mutate_after_validated { "after.ttf" } else { "finished.ttf" });
        let out = capture_dirs(&[root.clone()], Limits::default(), &mut |e| {
            if ((!mutate_after_validated && e == Event::Finished)
                || (mutate_after_validated && e == Event::Validated))
                && !fired
            {
                fired = true;
                fs::write(&p, DEJAVU).unwrap();
            }
        });
        assert!(fired);
        if mutate_after_validated {
            assert!(out.evidence.is_some());
            assert!(!out.stats.declined);
            assert!(!validate(out.evidence.as_ref().unwrap()));
        } else {
            declined(&out);
        }
    }
}
#[test]
fn environment_and_cwd_are_rechecked_at_capture_and_later_validation() {
    let f = Fixture::new();
    let root = f.sub("root");
    std::env::set_var("TEKT_JOINT_B_CONTEXT", "before");
    let out = capture(&[root.clone()]);
    accepted(&out);
    std::env::set_var("TEKT_JOINT_B_CONTEXT", "after");
    assert!(!validate(out.evidence.as_ref().unwrap()));
    std::env::set_var("TEKT_JOINT_B_CONTEXT", "before");
    assert!(validate(out.evidence.as_ref().unwrap()));
    std::env::set_current_dir(&f.dir).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
    std::env::set_current_dir(&f.cwd).unwrap();
    let mut fired = false;
    let changed = capture_dirs(&[root], Limits::default(), &mut |e| {
        if e == Event::Finished && !fired {
            fired = true;
            std::env::set_var("TEKT_JOINT_B_CONTEXT", "during");
        }
    });
    assert!(fired);
    declined(&changed);
}
#[test]
fn explicit_config_keeps_root_order_duplicates_includes_aliases_and_environment() {
    let f = Fixture::new();
    let a = f.sub("a");
    let b = f.sub("b");
    f.file("a/font.ttf", NOTO);
    f.file("b/font.ttf", DEJAVU);
    let child=f.file("included.conf",&xml(&format!("{}<alias><family>sans-serif</family><prefer><family>IndependentSans</family></prefer></alias>",dir_xml(&b))));
    let config = f.file(
        "fonts.conf",
        &xml(&format!(
            "{}<include>{}</include>{}",
            dir_xml(&a),
            child.display(),
            dir_xml(&a)
        )),
    );
    let expected = expected(&config);
    let env = std::env::var_os("FONTCONFIG_FILE");
    assert_eq!(expected.roots, vec![a.clone(), b, a]);
    let out = capture_config_file(&config, &expected, Limits::default(), &mut |_| {});
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&expected.roots)));
    assert_eq!(out.evidence.as_ref().unwrap().roots(), expected.roots.as_slice());
    assert_eq!(out.db.family_name(&fontdb::Family::SansSerif), "IndependentSans");
    assert_eq!(std::env::var_os("FONTCONFIG_FILE"), env);
    fs::write(&child, xml("<description>changed include</description>")).unwrap();
    assert!(!validate(out.evidence.as_ref().unwrap()));
}
#[test]
fn mismatched_expected_roots_aliases_bytes_env_or_cwd_decline_without_changing_db() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/font.ttf", NOTO);
    let config=f.file("fonts.conf",&xml(&format!("{}<alias><family>sans-serif</family><prefer><family>Noto Sans</family></prefer></alias>",dir_xml(&root))));
    let wanted = expected(&config);
    let native = catalog(&reference(&wanted.roots));
    for field in 0..5 {
        let mut wrong = wanted.clone();
        match field {
            0 => wrong.roots.push(root.clone()),
            1 => wrong.aliases.push("not actual alias".into()),
            2 => wrong.files[0].1.push(b' '),
            3 => wrong.env.push(("EXTRA_EXPECTED".into(), "1".into())),
            _ => wrong.cwd = f.dir.join("different"),
        };
        let out = capture_config_file(&config, &wrong, Limits::default(), &mut |_| {});
        declined(&out);
        assert_eq!(catalog(&out.db), native);
    }
}
#[test]
fn config_change_after_policy_load_declines_but_native_roots_still_load() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/font.ttf", NOTO);
    let config = f.file("fonts.conf", &xml(&dir_xml(&root)));
    let wanted = expected(&config);
    let mut fired = false;
    let out = capture_config_file(&config, &wanted, Limits::default(), &mut |e| {
        if e == Event::ConfigObserved && !fired {
            fired = true;
            let mut bytes = fs::read(&config).unwrap();
            bytes.push(b' ');
            fs::write(&config, bytes).unwrap();
        }
    });
    assert!(fired);
    declined(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&wanted.roots)));
}
#[test]
fn invalid_empty_or_missing_config_never_yields_evidence_and_repair_is_fresh() {
    let f = Fixture::new();
    let root = f.sub("root");
    f.file("root/font.ttf", NOTO);
    let config = f.file("fonts.conf", &xml(&dir_xml(&root)));
    let wanted = expected(&config);
    for bytes in [b"not XML".to_vec(), xml("")] {
        fs::write(&config, bytes).unwrap();
        let out = capture_config_file(&config, &wanted, Limits::default(), &mut |_| {});
        declined(&out);
    }
    fs::remove_file(&config).unwrap();
    declined(&capture_config_file(&config, &wanted, Limits::default(), &mut |_| {}));
    fs::write(&config, xml(&dir_xml(&root))).unwrap();
    let restored = expected(&config);
    let out = capture_config_file(&config, &restored, Limits::default(), &mut |_| {});
    accepted(&out);
    assert_eq!(out.db.faces().count(), 1);
}
fn ttc(fonts: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"ttcf");
    out.extend_from_slice(&0x00010000u32.to_be_bytes());
    out.extend_from_slice(&(fonts.len() as u32).to_be_bytes());
    out.resize(12 + 4 * fonts.len(), 0);
    for (index, font) in fonts.iter().enumerate() {
        while out.len() % 4 != 0 {
            out.push(0);
        }
        let base = out.len();
        out[12 + 4 * index..16 + 4 * index].copy_from_slice(&(base as u32).to_be_bytes());
        let mut bytes = font.to_vec();
        let count = u16::from_be_bytes(bytes[4..6].try_into().unwrap()) as usize;
        assert!(12 + 16 * count <= bytes.len());
        for i in 0..count {
            let at = 12 + 16 * i + 8;
            let old = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap());
            let new = old.checked_add(base as u32).unwrap();
            bytes[at..at + 4].copy_from_slice(&new.to_be_bytes());
        }
        out.extend_from_slice(&bytes);
    }
    out
}
#[test]
fn independently_rebased_collection_preserves_original_face_indices_and_catalog_order() {
    let f = Fixture::new();
    let root = f.sub("root");
    let collection = f.file("root/both.ttc", &ttc(&[NOTO, DEJAVU]));
    let roots = vec![root];
    let native = reference(&roots);
    let native_faces = catalog(&native);
    assert_eq!(
        native_faces.len(),
        2,
        "independent TTC fixture must expose both real faces"
    );
    assert_eq!(native_faces.iter().map(|f| f.index).collect::<Vec<_>>(), vec![0, 1]);
    let out = capture(&roots);
    accepted(&out);
    assert_eq!(catalog(&out.db), native_faces);
    assert!(out
        .evidence
        .as_ref()
        .unwrap()
        .stamps()
        .iter()
        .any(|s| s.path == collection));
}

#[test]
fn empty_requested_roots_are_a_complete_empty_discovery() {
    let _f = Fixture::new();
    let out = capture(&[]);
    accepted(&out);
    assert_eq!(catalog(&out.db), catalog(&reference(&[])));
    assert!(out.evidence.as_ref().unwrap().roots().is_empty());
}
#[test]
fn entry_directory_and_file_after_hooks_observe_the_declared_boundaries() {
    let f = Fixture::new();
    for stage in 0..3 {
        let root = f.sub(&format!("stage-{stage}"));
        let nested = root.join("nested");
        fs::create_dir(&nested).unwrap();
        let p = nested.join("font.ttf");
        fs::write(&p, NOTO).unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
        let mut fired = false;
        let out = capture_dirs(&[root.clone()], Limits::default(), &mut |event| {
            let hit = match (&event, stage) {
                (Event::EntryCaptured(path), 0) => path == &p,
                (Event::DirectoryCaptured(path), 1) => path == &nested,
                (Event::FileAfter(path), 2) => path == &p,
                _ => false,
            };
            if hit && !fired {
                fired = true;
                match stage {
                    0 => fs::write(&p, b"invalid before canonicalize/load").unwrap(),
                    1 => fs::write(nested.join("added.ttf"), DEJAVU).unwrap(),
                    _ => fs::set_permissions(&p, fs::Permissions::from_mode(0o400))
                        .unwrap(),
                }
            }
        });
        assert!(fired, "hook stage {stage} must be emitted");
        declined(&out);
        assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    }
}
#[test]
fn invalid_kernel_paths_decline_without_fabricating_absence_or_catalog_faces() {
    let f = Fixture::new();
    for root in [
        f.dir.join(OsString::from_vec(b"embedded\0nul".to_vec())),
        f.dir.join("x".repeat(300)),
    ] {
        let roots = [root];
        let out = capture(&roots);
        declined(&out);
        assert_eq!(catalog(&out.db), catalog(&reference(&roots)));
    }
}
#[test]
fn unreadable_directory_refuses_when_host_enforces_search_permissions() {
    let f = Fixture::new();
    let root = f.sub("private");
    f.file("private/font.ttf", NOTO);
    struct Restore(PathBuf);
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o700));
        }
    }
    let _restore = Restore(root.clone());
    fs::set_permissions(&root, fs::Permissions::from_mode(0)).unwrap();
    if fs::read_dir(&root)
        .err()
        .is_some_and(|e| e.kind() == io::ErrorKind::PermissionDenied)
    {
        let out = capture(&[root.clone()]);
        declined(&out);
        assert_eq!(catalog(&out.db), catalog(&reference(&[root])));
    }
}

// Composition controls: these do not invoke the product pipeline. The evidence
// boundary covers known inputs; the preserved post-build config_key guard must
// still fresh-parse the policy and compare its whole value before reuse.
#[test]
fn newly_created_optional_include_needs_the_separate_fresh_policy_guard() {
    let f = Fixture::new();
    let root = f.sub("fonts-before");
    let added_root = f.sub("fonts-after");
    f.file("fonts-before/a.ttf", NOTO);
    f.file("fonts-after/b.ttf", DEJAVU);
    let policy_dir = f.sub("policy");
    let missing_include = policy_dir.join("previously-absent.conf");
    let config = f.file(
        "policy/fonts.conf",
        &xml(&format!(
            "{}<include ignore_missing=\"yes\">{}</include>",
            dir_xml(&root),
            missing_include.display()
        )),
    );
    let before = expected(&config);
    assert_eq!(before.roots, vec![root]);
    assert!(!before.files.iter().any(|(p, _)| p == &missing_include));
    let new_policy = xml(&format!(
        "{}<alias><family>sans-serif</family><prefer><family>AddedByOptionalInclude</family></prefer></alias>",
        dir_xml(&added_root)
    ));
    let mut fired = false;
    let out = capture_config_file(&config, &before, Limits::default(), &mut |event| {
        if event == Event::Validated && !fired {
            fired = true;
            fs::write(&missing_include, &new_policy).unwrap();
        }
    });
    assert!(fired);
    accepted(&out); // validate intentionally does not rediscover missing includes.
    assert_eq!(catalog(&out.db), catalog(&reference(&before.roots)));
    for (path, bytes) in &before.files {
        assert_eq!(
            fs::read(path).unwrap(),
            *bytes,
            "all known policy bytes remain equal"
        );
    }
    let fresh = expected(&config); // Original fontconfig-parser, not candidate code.
    assert_ne!(fresh.roots, before.roots);
    assert_ne!(fresh.aliases, before.aliases);
    assert_ne!(fresh.files, before.files);
    assert!(fresh.files.iter().any(|(p, _)| p == &missing_include));
    assert_eq!(fresh.env, before.env);
    assert_eq!(fresh.cwd, before.cwd);
    // An equality check of the original whole ConfigKey would reject this state.
}

#[test]
fn new_include_directory_member_needs_the_separate_fresh_policy_guard() {
    let f = Fixture::new();
    let root = f.sub("fonts-before");
    let added_root = f.sub("fonts-after");
    f.file("fonts-before/a.ttf", NOTO);
    f.file("fonts-after/b.ttf", DEJAVU);
    let include_dir = f.sub("policy/conf.d");
    let added_config = include_dir.join("50-new-policy.conf");
    let config = f.file(
        "policy/fonts.conf",
        &xml(&format!("{}<include>{}</include>", dir_xml(&root), include_dir.display())),
    );
    let before = expected(&config);
    assert_eq!(before.roots, vec![root]);
    assert!(!before.files.iter().any(|(p, _)| p == &added_config));
    let new_policy = xml(&format!(
        "{}<alias><family>serif</family><prefer><family>AddedByDirectoryMember</family></prefer></alias>",
        dir_xml(&added_root)
    ));
    let mut fired = false;
    let out = capture_config_file(&config, &before, Limits::default(), &mut |event| {
        if event == Event::Validated && !fired {
            fired = true;
            fs::write(&added_config, &new_policy).unwrap();
        }
    });
    assert!(fired);
    accepted(&out); // No font-root or previously known config bytes changed.
    assert_eq!(catalog(&out.db), catalog(&reference(&before.roots)));
    for (path, bytes) in &before.files {
        assert_eq!(
            fs::read(path).unwrap(),
            *bytes,
            "all known policy bytes remain equal"
        );
    }
    let fresh = expected(&config);
    assert_ne!(fresh.roots, before.roots);
    assert_ne!(fresh.aliases, before.aliases);
    assert_ne!(fresh.files, before.files);
    assert!(fresh.files.iter().any(|(p, _)| p == &added_config));
    assert_eq!(fresh.env, before.env);
    assert_eq!(fresh.cwd, before.cwd);
    // The unchanged post-build whole-ConfigKey comparison is a separate guard.
}
