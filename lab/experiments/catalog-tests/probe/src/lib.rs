//! Laboratory-only aggregate scopes and optional filesystem observations.
//! Timing builds have no filesystem wrappers. Detail builds are not timing evidence.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Default)]
struct State {
    output: Option<PathBuf>,
    depth: usize,
    stack: Vec<&'static str>,
    scopes: BTreeMap<&'static str, (u64, u128)>,
    queries: BTreeMap<(Vec<&'static str>, &'static str, PathBuf, String), u64>,
}
thread_local! { static STATE: RefCell<State> = RefCell::new(State::default()); }
pub struct Cycle { enabled: bool, outer: bool }
pub struct Scope { label: &'static str, start: Option<Instant> }
/// One call per lazy catalog acquisition; captures configuration before phase clocks.
/// The output is emitted only when the outer cycle ends, after all scope clocks stop.
pub fn cycle() -> Cycle {
    let nested = STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.depth == 0 { false } else { s.depth += 1; true }
    });
    if nested { return Cycle { enabled: false, outer: false }; }
    let output = if std::env::var_os("CAT_PROBE").is_some_and(|v| v == "on") {
        std::env::var_os("CAT_PROBE_OUT").map(PathBuf::from)
    } else { None };
    let enabled = output.is_some();
    STATE.with(|s| *s.borrow_mut() = State { output, depth: 1, ..State::default() });
    Cycle { enabled, outer: true }
}
pub fn scope(label: &'static str) -> Scope {
    let start = STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.output.is_none() { return None; }
        s.stack.push(label);
        Some(Instant::now())
    });
    Scope { label, start }
}
pub fn measured<T>(label: &'static str, f: impl FnOnce() -> T) -> T {
    let _scope = scope(label);
    f()
}
impl Drop for Scope {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            let elapsed = start.elapsed().as_nanos();
            STATE.with(|s| {
                let mut s = s.borrow_mut();
                s.stack.pop();
                let entry = s.scopes.entry(self.label).or_default();
                entry.0 += 1;
                entry.1 += elapsed;
            });
        }
    }
}
impl Drop for Cycle {
    fn drop(&mut self) {
        if !self.outer {
            STATE.with(|s| s.borrow_mut().depth -= 1);
            return;
        }
        let state = STATE.with(|s| std::mem::take(&mut *s.borrow_mut()));
        if !self.enabled { return; }
        let Some(output) = state.output else { return; };
        let mut record = format!("cycle\tpid={}\tthread={:?}\n", std::process::id(), std::thread::current().id());
        for (label, (count, ns)) in state.scopes {
            record.push_str(&format!("scope\t{label}\t{count}\t{ns}\n"));
        }
        for ((stack, op, path, outcome), count) in state.queries {
            record.push_str(&format!("query\t{:?}\t{op}\t{:?}\t{outcome}\t{count}\n", stack, path));
        }
        record.push_str("end\n");
        // One aggregate write per cycle, never per query. Errors are explicit.
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(output)
            .expect("open CAT_PROBE_OUT");
        file.write_all(record.as_bytes()).expect("write aggregate catalog probe");
    }
}
fn record<T>(op: &'static str, path: &Path, result: &io::Result<T>) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.output.is_none() { return; }
        let outcome = match result { Ok(_) => "ok".into(), Err(e) => format!("{:?}", e.kind()) };
        let key = (s.stack.clone(), op, path.to_path_buf(), outcome);
        *s.queries.entry(key).or_default() += 1;
    });
}
macro_rules! fs_query {
    ($name:ident, $ty:ty) => {
        pub fn $name(path: impl AsRef<Path>) -> io::Result<$ty> {
            let path = path.as_ref();
            let result = std::fs::$name(path);
            record(stringify!($name), path, &result);
            result
        }
    }
}
fs_query!(metadata, std::fs::Metadata);
fs_query!(symlink_metadata, std::fs::Metadata);
fs_query!(canonicalize, PathBuf);
fs_query!(read, Vec<u8>);
fs_query!(read_dir, std::fs::ReadDir);
pub fn open(path: impl AsRef<Path>) -> io::Result<std::fs::File> {
    let path = path.as_ref();
    let result = std::fs::File::open(path);
    record("open", path, &result);
    result
}
