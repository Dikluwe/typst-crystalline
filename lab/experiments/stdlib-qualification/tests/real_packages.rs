//! Public SystemWorld package integration, separate from pure eval/MockWorld.
//! C prepares pinned package sources and wrappers in task RAM before execution.
use comemo::Track;
use std::path::{Path, PathBuf};
use typst_core::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
use typst_core::entities::{compiler_features::Features, element_registry::ElementRegistry,
    sink::Sink, value::Value, world_types::{Route, Routines, Traced}};
use typst_core::contracts::world::World;
use typst_infra::world::SystemWorld;
const RAM: &str = "/dev/shm/tekt-stdlib-qualification-20261106";
fn checked_ram(raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    assert!(path.is_absolute() && path.starts_with(RAM));
    assert!(path.canonicalize().unwrap().starts_with(RAM));
    path
}

#[test]
fn real_packages_child() {
    if std::env::var_os("TEKT_QUAL_PACKAGE_CHILD").is_none() { return; }
    let root = checked_ram(&std::env::var("TEKT_QUAL_PACKAGE_ROOT").expect("prepared corpus root"));
    let cache = checked_ram(&std::env::var("TEKT_QUAL_PACKAGE_CACHE").expect("prepared typst/packages cache"));
    for (package, version) in [("cetz", "0.5.2"), ("oxifmt", "1.0.0")] {
        assert!(cache.join("preview").join(package).join(version).join("typst.toml").is_file(), "missing pinned local package {package}:{version}");
    }
    assert_eq!(cache.file_name().unwrap(), "packages");
    assert_eq!(cache.parent().unwrap().file_name().unwrap(), "typst");
    let xdg = cache.parent().unwrap().parent().unwrap();
    std::env::set_var("XDG_CACHE_HOME", xdg);
    std::env::set_var("XDG_DATA_HOME", xdg);
    let fixtures = std::env::var("TEKT_QUAL_PACKAGE_FIXTURES").unwrap_or_else(|_| "cetz.typ:oxifmt.typ".into());
    for fixture in fixtures.split(':') {
        let relative = Path::new(fixture);
        assert!(!relative.is_absolute() && !relative.components().any(|c| c == std::path::Component::ParentDir));
        assert!(root.join(relative).canonicalize().unwrap().starts_with(&root));
        for tag in ["A-á中🦀", "B-é中🦀", "A-á中🦀"] {
            let world = SystemWorld::new(&root, fixture).unwrap().with_fonts(vec![])
                .with_inputs(vec![("qualification-tag".into(), tag.into())]);
            let source = world.source(world.main()).unwrap();
            let mut sink = Sink::new();
            let result = eval_with_full_error_target_and_features(&Routines::new(), &world,
                Traced::default().track(), sink.track_mut(), Route::root().track(), &source,
                &ElementRegistry::new(), false, EvalTarget::Paged, Features::empty());
            let module = result.unwrap_or_else(|errors| panic!("package wrapper {fixture} failed: {errors:?}"));
            let _diagnostics = sink.into_diagnostics(); // compared separately by C, without rewriting warnings
            assert!(matches!(module.scope().get("qualification_ok"), Some(Value::Bool(true))), "wrapper must export qualification_ok=true");
            let Some(Value::Dict(inputs)) = module.scope().get("qualification_inputs") else { panic!("wrapper must export qualification_inputs=sys.inputs"); };
            assert!(matches!(inputs.get("qualification-tag"), Some(Value::Str(value)) if value.as_str() == tag));
        }
    }
    println!("\nTEKT_QUAL_PACKAGE_OK");
}

#[test]
#[ignore = "requires C's pinned package cache and prepared wrappers in task RAM"]
fn real_local_packages_observe_inputs_across_fresh_system_worlds() {
    let prefix = module_path!().split("::").skip(1).collect::<Vec<_>>().join("::");
    let child = if prefix.is_empty() { "real_packages_child".into() } else { format!("{prefix}::real_packages_child") };
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", &child, "--test-threads=1", "--nocapture"])
        .env("TEKT_QUAL_PACKAGE_CHILD", "1").output().unwrap();
    assert!(output.status.success(), "{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().filter(|line| *line == "TEKT_QUAL_PACKAGE_OK").count(), 1);
    assert_eq!(stdout.lines().filter(|line| line.starts_with("test result: ok. 1 passed; 0 failed;")).count(), 1);
}
