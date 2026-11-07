//! Public-API tests of the font session's observable freshness.
//! Standalone rustc --test, --extern typst_core=... --extern typst_infra=...
//! Each scenario executes in a child; all mutable fixtures reside in RAM.
#![cfg(target_os = "linux")]
use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::sync::Arc;
use typst_core::contracts::world::World;
use typst_core::entities::font_book::FontInfo;
use typst_core::entities::world_types::Font;
use typst_infra::world::{FontCatalogSession, SystemWorld};

const NOTO: &[u8] = include_bytes!("../../../../03_infra/fixtures/fonts/NotoSans-Regular.ttf");
const NIMBUS: &[u8] = include_bytes!("../../../../03_infra/fixtures/fonts/NimbusSans-Regular.otf");

struct Fixture { dir: PathBuf, fonts: PathBuf, system_roots: bool }
impl Fixture {
    fn new() -> Self {
        let mode = std::env::var("TEKT_WATCH_FONT_TEST_MODE").unwrap_or_else(|_| "explicit".into());
        assert!(matches!(mode.as_str(), "explicit" | "system-roots"), "unknown font test mode: {mode}");
        let system_roots = mode == "system-roots";
        let base = std::env::var_os("TMPDIR").map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/dev/shm/tekt-joint-witness-20261106"));
        assert!(base.is_absolute() && base.starts_with("/dev/shm"), "TMPDIR must be RAM");
        fs::create_dir_all(&base).unwrap();
        let canonical = fs::canonicalize(&base).unwrap();
        assert!(canonical.starts_with("/dev/shm"), "TMPDIR must not escape RAM via a symlink");
        let mut n = 0u64;
        let dir = loop {
            let candidate = canonical.join(format!("font-contract-b-{}-{n}", std::process::id()));
            match fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => n += 1,
                Err(e) => panic!("RAM fixture: {e}"),
            }
        };
        fs::write(dir.join("main.typ"), "Before").unwrap();
        let home = dir.join("home");
        fs::create_dir(&home).unwrap();
        let fonts = if system_roots { home.join(".fonts") } else { dir.join("fonts") };
        fs::create_dir(&fonts).unwrap();
        std::env::set_var("HOME", &home);
        for (name, suffix) in [("XDG_CONFIG_HOME", ".config"), ("XDG_DATA_HOME", ".local/share"), ("XDG_CACHE_HOME", ".cache")] {
            let path = home.join(suffix);
            fs::create_dir_all(&path).unwrap();
            std::env::set_var(name, path);
        }
        if system_roots {
            // Exercise the host's default /etc/fonts configuration with no
            // custom config or project roots. Whether it qualifies for fast
            // proof is observable by C's probes, not assumed by this test.
            for key in ["FONTCONFIG_FILE", "FONTCONFIG_PATH", "FONTCONFIG_SYSROOT"] {
                std::env::remove_var(key);
            }
        } else {
            let config = dir.join("fonts.conf");
            fs::write(&config, format!("<?xml version=\"1.0\"?><fontconfig><dir>{}</dir></fontconfig>", fonts.display())).unwrap();
            std::env::set_var("FONTCONFIG_FILE", config);
        }
        Self { dir, fonts, system_roots }
    }
    fn paths(&self) -> Vec<PathBuf> {
        if self.system_roots { Vec::new() } else { vec![self.fonts.clone()] }
    }
    fn world(&self, session: &Arc<FontCatalogSession>) -> SystemWorld {
        SystemWorld::new(&self.dir, "main.typ").unwrap()
            .with_font_catalog_session(session.clone(), &self.paths())
    }
    fn fresh(&self) -> SystemWorld {
        SystemWorld::new(&self.dir, "main.typ").unwrap()
            .with_fonts_and_system(&self.paths())
    }
    fn put(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.fonts.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.dir); } }
fn infos(world: &SystemWorld) -> Vec<FontInfo> { world.book().infos().to_vec() }
fn observed_indices(f: &Fixture, world: &SystemWorld) -> Vec<usize> {
    if !f.system_roots { return (0..world.book().len()).collect(); }
    // Only our two fixture families can be mutated by these scenarios. Derive
    // their names from real font metadata rather than hard-code family names.
    // Catalog metadata/order remains checked for EVERY host face below; avoid
    // eagerly extracting/copying every unrelated TTC face just for byte checks.
    let families = [NOTO, NIMBUS].map(|bytes|
        typst_infra::fonts::font_info_from_bytes(bytes, 0).expect("valid fixture").family);
    world.book().infos().iter().enumerate()
        .filter_map(|(index, info)| families.contains(&info.family).then_some(index)).collect()
}
fn font_bytes(f: &Fixture, world: &SystemWorld) -> Vec<Vec<u8>> {
    observed_indices(f, world).into_iter().map(|i| world.font(i).expect("aligned font must load").as_slice().to_vec()).collect()
}
fn fresh_equivalent(f: &Fixture, world: &SystemWorld) {
    let reference = f.fresh();
    assert_eq!(infos(world), infos(&reference));
    assert_eq!(font_bytes(f, world), font_bytes(f, &reference));
}
fn initial(f: &Fixture, session: &Arc<FontCatalogSession>) -> (SystemWorld, Vec<FontInfo>) {
    let world = f.world(session);
    let first = infos(&world);
    assert!(!first.is_empty());
    assert_eq!(world.source(world.main()).unwrap().text(), "Before");
    fresh_equivalent(f, &world);
    (world, first)
}
fn addition_removal(f: &Fixture) {
    f.put("a.ttf", NOTO);
    let session = Arc::new(FontCatalogSession::new());
    let (old, old_infos) = initial(f, &session);
    let warm = f.world(&session);
    assert_eq!(infos(&warm), old_infos);
    f.put("b.otf", NIMBUS);
    fs::write(f.dir.join("main.typ"), "After").unwrap();
    let added = f.world(&session);
    fresh_equivalent(f, &added);
    assert!(added.book().len() > old_infos.len());
    let added_infos = infos(&added);
    assert_eq!(infos(&old), old_infos);
    assert_eq!(old.source(old.main()).unwrap().text(), "Before");
    assert_eq!(added.source(added.main()).unwrap().text(), "After");
    fs::remove_file(f.fonts.join("a.ttf")).unwrap();
    let removed = f.world(&session);
    fresh_equivalent(f, &removed);
    assert!(removed.book().len() < added_infos.len());
    assert_eq!(infos(&added), added_infos);
    assert_eq!(infos(&old), old_infos);
}
fn invalid_valid(f: &Fixture) {
    let path = f.put("a.ttf", b"invalid");
    let session = Arc::new(FontCatalogSession::new());
    let old = f.world(&session);
    let before = infos(&old);
    fresh_equivalent(f, &old);
    fs::write(path, NOTO).unwrap();
    let valid = f.world(&session);
    fresh_equivalent(f, &valid);
    assert!(valid.book().len() > before.len());
    assert_eq!(infos(&old), before);
}
fn padded(bytes: &[u8], len: usize) -> Vec<u8> { let mut out = bytes.to_vec(); out.resize(len, 0); out }
fn valid_equal_length_restored_mtime(f: &Fixture) {
    let len = NOTO.len().max(NIMBUS.len());
    let a = padded(NOTO, len);
    let b = padded(NIMBUS, len);
    assert_ne!(a, b);
    let path = f.put("a.ttf", &a);
    let session = Arc::new(FontCatalogSession::new());
    let (old, before) = initial(f, &session);
    let metadata = fs::metadata(&path).unwrap();
    let modified = metadata.modified().unwrap();
    // Same inode, same length, restored mtime. No sleeps, no reliance on mtime
    // advancing: the session must inspect the actual freshness evidence.
    let mut file = OpenOptions::new().write(true).open(&path).unwrap();
    file.write_all(&b).unwrap();
    file.set_times(FileTimes::new().set_modified(modified)).unwrap();
    drop(file);
    assert_eq!(fs::metadata(&path).unwrap().len(), metadata.len());
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
    assert_eq!(fs::read(&path).unwrap(), b);
    let next = f.world(&session);
    fresh_equivalent(f, &next);
    assert_ne!(infos(&next), before, "two real font families must be distinguished");
    assert_eq!(infos(&old), before);
}
fn symlink_retarget(f: &Fixture) {
    let a = f.dir.join("outside-a.ttf");
    let b = f.dir.join("outside-b.otf");
    fs::write(&a, NOTO).unwrap();
    fs::write(&b, NIMBUS).unwrap();
    let link = f.fonts.join("linked.ttf");
    symlink(&a, &link).unwrap();
    let session = Arc::new(FontCatalogSession::new());
    let (old, before) = initial(f, &session);
    fs::remove_file(&link).unwrap();
    symlink(&b, &link).unwrap();
    let changed = f.world(&session);
    fresh_equivalent(f, &changed);
    assert_ne!(infos(&changed), before);
    assert_eq!(infos(&old), before);
    fs::remove_file(&b).unwrap();
    let dangling = f.world(&session);
    fresh_equivalent(f, &dangling);
    assert!(dangling.book().len() < changed.book().len());
    fs::write(&b, NIMBUS).unwrap();
    let repaired = f.world(&session);
    fresh_equivalent(f, &repaired);
    assert_eq!(infos(&repaired), infos(&changed));
}
fn live_world_atomic_replacement(f: &Fixture) {
    let path = f.put("a.ttf", NOTO);
    let session = Arc::new(FontCatalogSession::new());
    let (old, before) = initial(f, &session);
    let held: Vec<Font> = observed_indices(f, &old).into_iter().map(|i| old.font(i).unwrap()).collect();
    let bytes: Vec<Vec<u8>> = held.iter().map(|f| f.as_slice().to_vec()).collect();
    let replacement = f.dir.join("replacement.otf");
    fs::write(&replacement, NIMBUS).unwrap();
    let old_modified = fs::metadata(&path).unwrap().modified().unwrap();
    File::options().write(true).open(&replacement).unwrap()
        .set_times(FileTimes::new().set_modified(old_modified)).unwrap();
    fs::rename(replacement, path).unwrap();
    let new = f.world(&session);
    fresh_equivalent(f, &new);
    assert_ne!(infos(&new), before);
    assert_eq!(infos(&old), before);
    for (font, expected) in held.iter().zip(bytes) { assert_eq!(font.as_slice(), expected); }
}

#[test]
fn font_session_scenarios_child() {
    let Ok(scenario) = std::env::var("TEKT_WATCH_FONT_TEST_CHILD") else { return; };
    let f = Fixture::new();
    match scenario.as_str() {
        "addition-removal" => addition_removal(&f),
        "invalid-valid" => invalid_valid(&f),
        "same-size-mtime" => valid_equal_length_restored_mtime(&f),
        "symlink" => symlink_retarget(&f),
        "live-world" => live_world_atomic_replacement(&f),
        _ => panic!("unknown test scenario"),
    }
    println!("\nTEKT_WATCH_FONT_SCENARIO_OK={scenario}");
}
#[test]
fn public_font_session_observes_every_generation_change() {
    let executable = std::env::current_exe().unwrap();
    let prefix = module_path!().split("::").skip(1).collect::<Vec<_>>().join("::");
    let child = if prefix.is_empty() { "font_session_scenarios_child".to_owned() }
        else { format!("{prefix}::font_session_scenarios_child") };
    for scenario in ["addition-removal", "invalid-valid", "same-size-mtime", "symlink", "live-world"] {
        let output = std::process::Command::new(&executable)
            .args(["--exact", child.as_str(), "--test-threads=1", "--nocapture"])
            .env("TEKT_WATCH_FONT_TEST_CHILD", scenario)
            .output().unwrap();
        assert!(output.status.success(), "scenario={scenario}\nstdout={}\nstderr={}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        let marker = format!("TEKT_WATCH_FONT_SCENARIO_OK={scenario}");
        assert_eq!(stdout.lines().filter(|line| *line == marker).count(), 1,
            "scenario child did not finish exactly once: {stdout}");
        assert_eq!(stdout.lines().filter(|line| line.starts_with("test result: ok. 1 passed; 0 failed;")).count(), 1,
            "expected exactly one executed child test: {stdout}");
    }
}
