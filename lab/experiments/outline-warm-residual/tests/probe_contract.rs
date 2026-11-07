//! Compile independently against the laboratory collector module.
//! No compiler pipeline, source parsing or product API is used here.
#[path = "../phase_probe.rs"]
mod probe;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn prefix_child(name: &str) -> String {
    let prefix = module_path!().split("::").skip(1).collect::<Vec<_>>().join("::");
    if prefix.is_empty() { name.to_owned() } else { format!("{prefix}::{name}") }
}

fn initialize() {
    let _access = probe::enter("catalog.access");
    let _font = probe::enter("catalog.initialize");
}

#[test]
fn collector_child() {
    let Ok(mode) = std::env::var("TEKT_COLLECTOR_TEST_CHILD") else { return; };
    assert!(matches!(mode.as_str(), "off" | "on"));
    initialize();
    {
        let _eval = probe::enter("pipeline.eval");
        initialize();
        {
            let _context = probe::enter("pipeline.context");
            initialize();
        }
        initialize();
        // Unwinding must attribute the live font guard to layout, then restore
        // eval so the next initialization is not left in layout.
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let unwind = std::panic::catch_unwind(|| {
            let _layout = probe::enter("pipeline.layout");
            let _access = probe::enter("catalog.access");
            let _font = probe::enter("catalog.initialize");
            panic!("controlled unwind");
        });
        std::panic::set_hook(hook);
        assert!(unwind.is_err());
        initialize();
        // Active stage is thread-local. The new thread has no inherited stage.
        std::thread::spawn(initialize).join().unwrap();
    }
    initialize();
    let error = probe::measure("pipeline.finish", || Err::<(), _>("expected"));
    assert_eq!(error, Err("expected"));
    probe::mark("event.collector.control");
    probe::count("event.collector.control", 2);
    probe::count("event.collector.zero", 0);
    probe::flush();
    probe::flush();
    println!("\nTEKT_COLLECTOR_CHILD_OK={mode}");
}

#[test]
fn collector_off_has_no_file_on_attributes_each_initialization_once_and_restores_stage() {
    let base = PathBuf::from("/dev/shm/tekt-outline-warm-20261106");
    fs::create_dir_all(&base).unwrap();
    assert!(fs::canonicalize(&base).unwrap().starts_with(&base));
    let mut suffix = 0;
    let dir = loop {
        let candidate = base.join(format!("collector-b-{}-{suffix}", std::process::id()));
        match fs::create_dir(&candidate) {
            Ok(()) => break candidate,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => suffix += 1,
            Err(e) => panic!("RAM collector fixture: {e}"),
        }
    };
    struct Remove(PathBuf);
    impl Drop for Remove { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }
    let _remove = Remove(dir.clone());
    for mode in ["off", "on"] {
        let destination = dir.join(format!("{mode}.tsv"));
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        let child_name = prefix_child("collector_child");
        command.args(["--exact", &child_name, "--test-threads=1", "--nocapture"])
            .env("TEKT_COLLECTOR_TEST_CHILD", mode).env_remove("TEKT_JOINT_PHASE_FILE");
        if mode == "on" { command.env("TEKT_JOINT_PHASE_FILE", &destination); }
        let output = command.output().unwrap();
        assert!(output.status.success(), "mode={mode} stdout={} stderr={}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        let marker = format!("TEKT_COLLECTOR_CHILD_OK={mode}");
        assert_eq!(stdout.lines().filter(|line| *line == marker).count(), 1);
        assert_eq!(stdout.lines().filter(|line| line.starts_with("test result: ok. 1 passed; 0 failed;")).count(), 1);
        if mode == "off" {
            assert!(!destination.exists());
            assert!(output.stderr.is_empty(), "off collector must not produce diagnostics");
            continue;
        }
        let text = fs::read_to_string(&destination).unwrap();
        let mut rows = HashMap::new();
        let mut process = None;
        for line in text.lines() {
            let fields: Vec<_> = line.split('\t').collect(); assert_eq!(fields.len(), 5);
            let pid: u32 = fields[0].parse().unwrap(); assert!(pid > 0);
            if let Some(old) = process { assert_eq!(old, pid); } else { process = Some(pid); }
            let cycle: u64 = fields[1].parse().unwrap(); assert!(cycle <= 1);
            let calls: u64 = fields[3].parse().unwrap(); assert!(calls > 0);
            let ns: u128 = fields[4].parse().unwrap();
            if fields[2].starts_with("event.") { assert_eq!(ns, 0); }
            assert!(rows.insert((cycle, fields[2].to_owned()), (calls, ns)).is_none());
        }
        let get = |cycle, label: &str| rows.get(&(cycle, label.to_owned())).copied().unwrap_or((0, 0));
        assert_eq!(get(0, "catalog.initialize").0, 8);
        assert_eq!(get(0, "catalog.access").0, 8);
        assert_eq!(get(0, "font.eval").0, 3);
        assert_eq!(get(0, "font.layout").0, 1);
        assert_eq!(get(0, "font.context").0, 1);
        assert_eq!(get(0, "font.outside").0, 3);
        let font_calls: u64 = ["eval", "introspect", "context", "layout", "finish", "outside"].iter().map(|stage| get(0, &format!("font.{stage}")).0).sum();
        let font_ns: u128 = ["eval", "introspect", "context", "layout", "finish", "outside"].iter().map(|stage| get(0, &format!("font.{stage}")).1).sum();
        assert_eq!(font_calls, get(0, "catalog.initialize").0);
        assert_eq!(font_ns, get(0, "catalog.initialize").1);
        assert!(get(0, "font.eval").1 <= get(0, "pipeline.eval").1);
        assert!(get(0, "font.context").1 <= get(0, "pipeline.context").1);
        assert!(get(0, "font.layout").1 <= get(0, "pipeline.layout").1);
        assert!(get(0, "catalog.initialize").1 <= get(0, "catalog.access").1);
        assert_eq!(get(0, "pipeline.finish").0, 1, "RAII must preserve error-return measurement");
        assert_eq!(get(0, "event.collector.control"), (3, 0));
        assert_eq!(get(0, "event.collector.zero"), (0, 0));
        assert_eq!(get(0, "event.cycle"), (1, 0));
        assert_eq!(get(1, "event.cycle"), (1, 0));
        assert_eq!(rows.keys().filter(|(cycle, _)| *cycle == 1).count(), 1, "flush must drain previous totals");
    }
}
