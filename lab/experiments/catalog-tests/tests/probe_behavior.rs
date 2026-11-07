// Standalone producer behavior tests. C copies this file and probe/ into RAM
// with this relative layout and builds using rustc --test --edition=2021.
#[path = "../probe/src/lib.rs"]
mod catalog_probe;

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const SCENARIO: &str = "CATALOG_PROBE_TEST_SCENARIO";
const ROOT: &str = "CATALOG_PROBE_TEST_ROOT";
static NEXT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default)]
struct Block {
    scopes: BTreeMap<String, (u64, u128)>,
    query_counts: BTreeMap<String, u64>,
}

fn blocks(path: &Path) -> Vec<Block> {
    let text = fs::read_to_string(path).expect("On must emit a readable dump");
    assert!(text.ends_with("end\n"), "truncated producer dump: {text:?}");
    let mut current: Option<Block> = None;
    let mut result = Vec::new();
    for line in text.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        match fields.as_slice() {
            ["cycle", pid, thread] => {
                assert!(current.is_none(), "interleaved cycle: {line:?}");
                let pid = pid.strip_prefix("pid=").unwrap().parse::<u64>().unwrap();
                assert!(pid > 0);
                assert!(!thread.strip_prefix("thread=").unwrap().is_empty());
                current = Some(Block::default());
            }
            ["scope", label, count, ns] => {
                let block = current.as_mut().expect("scope outside cycle");
                let count = count.parse::<u64>().unwrap();
                let ns = ns.parse::<u128>().unwrap();
                assert!(count > 0);
                assert!(!label.is_empty());
                assert!(block.scopes.insert((*label).into(), (count, ns)).is_none(),
                    "duplicate scope within acquisition: {label}");
            }
            ["query", stack, op, path, outcome, count] => {
                let block = current.as_mut().expect("query outside cycle");
                assert!(!stack.is_empty() && !op.is_empty() && !path.is_empty() && !outcome.is_empty());
                let count = count.parse::<u64>().unwrap();
                assert!(count > 0);
                *block.query_counts.entry((*op).into()).or_default() += count;
            }
            ["end"] => result.push(current.take().expect("end outside cycle")),
            _ => panic!("malformed or interleaved record: {line:?}"),
        }
    }
    assert!(current.is_none(), "unclosed cycle");
    assert!(!result.is_empty());
    result
}

fn root() -> PathBuf { std::env::var_os(ROOT).unwrap().into() }
fn output() -> PathBuf { std::env::var_os("CAT_PROBE_OUT").unwrap().into() }

fn run(scenario: &str, enabled: bool) {
    let base = PathBuf::from(std::env::var_os("TMPDIR").expect("C must set TMPDIR to task RAM"));
    assert!(base.is_absolute() && base.starts_with("/dev/shm"), "TMPDIR must be task RAM");
    let unique = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = base.join(format!("producer-test-{}-{unique}-{scenario}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let result = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "producer_child", "--nocapture", "--test-threads=1"])
        .env(SCENARIO, scenario).env(ROOT, &dir)
        .env("CAT_PROBE", if enabled { "on" } else { "off" })
        .env("CAT_PROBE_OUT", dir.join("output.tsv"))
        .env("TMPDIR", &dir).output().unwrap();
    assert!(result.status.success(), "scenario={scenario} status={}\nstdout={}\nstderr={}",
        result.status, String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
    if enabled { blocks(&dir.join("output.tsv")); }
    else { assert!(!dir.join("output.tsv").exists(), "Off created a dump file"); }
    fs::remove_dir_all(dir).unwrap();
}

#[test] fn single_cycle_produces_one_complete_block() { run("single", true); }
#[test] fn nested_cycle_does_not_reset_or_double_flush() { run("nested", true); }
#[test] fn two_cycles_append_independent_blocks() { run("append", true); }
#[test] fn measured_preserves_return_values_and_returned_guard() { run("returned", true); }
#[test] fn off_does_not_create_output() { run("off", false); }
#[test] fn filesystem_wrappers_preserve_success_errors_and_symlinks() { run("wrappers", true); }
#[test] fn panic_unwind_leaves_following_cycle_valid() { run("panic", true); }

#[test]
fn producer_child() {
    let Ok(scenario) = std::env::var(SCENARIO) else { return };
    match scenario.as_str() {
        "single" => {
            {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("single_scope");
                std::hint::black_box((0..1000u64).sum::<u64>());
            }
            let records = blocks(&output());
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].scopes["single_scope"].0, 1);
        }
        "nested" => {
            {
                let _outer = catalog_probe::cycle();
                { let _before = catalog_probe::scope("before_nested"); }
                {
                    let _nested = catalog_probe::cycle();
                    let _inside = catalog_probe::scope("inside_nested");
                }
                assert!(fs::read(&output()).map_or(true, |bytes| bytes.is_empty()),
                    "nested drop flushed before outer acquisition ended");
                { let _after = catalog_probe::scope("after_nested"); }
            }
            let records = blocks(&output());
            assert_eq!(records.len(), 1);
            for label in ["before_nested", "inside_nested", "after_nested"] {
                assert_eq!(records[0].scopes[label].0, 1, "nested cycle reset {label}");
            }
        }
        "append" => {
            {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("first_cycle");
                { let _same = catalog_probe::scope("repeated_across_cycles"); }
                { let _same = catalog_probe::scope("repeated_across_cycles"); }
            }
            let first = fs::read(output()).unwrap();
            assert_eq!(blocks(&output()).len(), 1);
            {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("second_cycle");
                let _same = catalog_probe::scope("repeated_across_cycles");
            }
            assert!(fs::read(output()).unwrap().starts_with(&first), "second acquisition overwrote first");
            let records = blocks(&output());
            assert_eq!(records.len(), 2);
            assert_eq!(records[0].scopes["repeated_across_cycles"].0, 2);
            assert_eq!(records[1].scopes["repeated_across_cycles"].0, 1);
            assert!(records[0].scopes.contains_key("first_cycle"));
            assert!(!records[0].scopes.contains_key("second_cycle"));
            assert!(records[1].scopes.contains_key("second_cycle"));
            assert!(!records[1].scopes.contains_key("first_cycle"));
        }
        "returned" => {
            {
                let _cycle = catalog_probe::cycle();
                let calls = std::cell::Cell::new(0);
                let expected = vec![3u8, 1, 4];
                let ptr = expected.as_ptr();
                let actual = catalog_probe::measured("move_return", || {
                    calls.set(calls.get() + 1);
                    expected
                });
                assert_eq!(calls.get(), 1, "measured must execute closure exactly once");
                assert_eq!(actual, [3, 1, 4]);
                assert_eq!(actual.as_ptr(), ptr, "measured must return original moved value");
                let guard = catalog_probe::measured("guard_constructor", || catalog_probe::scope("returned_guard"));
                drop(guard);
                let error: Result<u8, &str> = catalog_probe::measured("error_return", || Err("exact error"));
                assert_eq!(error, Err("exact error"));
            }
            let records = blocks(&output());
            assert_eq!(records.len(), 1);
            for label in ["move_return", "guard_constructor", "returned_guard", "error_return"] {
                assert_eq!(records[0].scopes[label].0, 1);
            }
        }
        "off" => {
            let fixture = root().join("off-input");
            fs::write(&fixture, b"off fixture").unwrap();
            {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("off_scope");
                assert_eq!(catalog_probe::measured("off_return", || 23), 23);
                assert_eq!(catalog_probe::read(&fixture).unwrap(), b"off fixture");
                { let _nested = catalog_probe::cycle(); }
            }
            assert!(!output().exists());
        }
        "wrappers" => wrappers(),
        "panic" => {
            let failed = std::panic::catch_unwind(|| {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("unwound_scope");
                catalog_probe::measured("unwound_measured", || panic!("intentional test unwind"));
            });
            assert!(failed.is_err());
            {
                let _cycle = catalog_probe::cycle();
                let _scope = catalog_probe::scope("after_unwind");
                assert_eq!(catalog_probe::measured("after_unwind_return", || 71), 71);
            }
            let records = blocks(&output());
            let last = records.last().unwrap();
            assert_eq!(last.scopes["after_unwind"].0, 1);
            assert_eq!(last.scopes["after_unwind_return"].0, 1);
            assert!(!last.scopes.contains_key("unwound_scope"), "unwind left stale acquisition state");
            assert!(!last.scopes.contains_key("unwound_measured"));
            assert_eq!(records.iter().filter(|b| b.scopes.contains_key("after_unwind")).count(), 1);
        }
        other => panic!("unknown test scenario: {other}"),
    }
}

fn names(entries: fs::ReadDir) -> HashSet<std::ffi::OsString> {
    entries.map(|entry| entry.unwrap().file_name()).collect()
}

fn wrappers() {
    let dir = root().join("inputs");
    fs::create_dir(&dir).unwrap();
    // Debug formatting must escape these characters rather than corrupt TSV.
    let file = dir.join("file\twith\nnewline.bin");
    let missing = dir.join("missing");
    let payload = b"exact\0binary\xffpayload";
    fs::write(&file, payload).unwrap();
    #[cfg(unix)]
    let link = {
        let link = dir.join("link");
        std::os::unix::fs::symlink(&file, &link).unwrap();
        link
    };
    {
        let _cycle = catalog_probe::cycle();
        let _scope = catalog_probe::scope("wrapper_contract");
        assert_eq!(catalog_probe::metadata(&file).unwrap().len(), fs::metadata(&file).unwrap().len());
        assert_eq!(catalog_probe::symlink_metadata(&file).unwrap().len(), fs::symlink_metadata(&file).unwrap().len());
        assert_eq!(catalog_probe::read(&file).unwrap(), fs::read(&file).unwrap());
        assert_eq!(names(catalog_probe::read_dir(&dir).unwrap()), names(fs::read_dir(&dir).unwrap()));
        assert_eq!(catalog_probe::canonicalize(&file).unwrap(), fs::canonicalize(&file).unwrap());
        let mut opened = Vec::new();
        catalog_probe::open(&file).unwrap().read_to_end(&mut opened).unwrap();
        assert_eq!(opened, payload);
        assert_eq!(catalog_probe::metadata(&missing).unwrap_err().kind(), fs::metadata(&missing).unwrap_err().kind());
        assert_eq!(catalog_probe::symlink_metadata(&missing).unwrap_err().kind(), fs::symlink_metadata(&missing).unwrap_err().kind());
        assert_eq!(catalog_probe::read(&missing).unwrap_err().kind(), fs::read(&missing).unwrap_err().kind());
        assert_eq!(catalog_probe::read_dir(&missing).unwrap_err().kind(), fs::read_dir(&missing).unwrap_err().kind());
        assert_eq!(catalog_probe::canonicalize(&missing).unwrap_err().kind(), fs::canonicalize(&missing).unwrap_err().kind());
        assert_eq!(catalog_probe::open(&missing).unwrap_err().kind(), fs::File::open(&missing).unwrap_err().kind());
        #[cfg(unix)]
        {
            assert_eq!(catalog_probe::metadata(&link).unwrap().is_file(), fs::metadata(&link).unwrap().is_file());
            assert!(catalog_probe::symlink_metadata(&link).unwrap().file_type().is_symlink());
        }
    }
    let records = blocks(&output());
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].scopes["wrapper_contract"].0, 1);
    for op in ["metadata", "symlink_metadata", "read", "read_dir", "canonicalize", "open"] {
        assert!(records[0].query_counts.get(op).copied().unwrap_or(0) >= 2,
            "wrapper success/error operations absent for {op}: {:?}", records[0]);
    }
}
