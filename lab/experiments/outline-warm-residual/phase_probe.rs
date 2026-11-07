//! LAB ONLY. Inclusive L3 scopes, deliberately independent of pipeline Timings.
//! TEKT_JOINT_PHASE_FILE enables append-only TSV flushes after publication. Unset
//! means no clocks, mutex, file creation or output. The OnceLock opt-in check
//! remains, and must be calibrated with the same executable on/off.
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
thread_local! { static ACTIVE_STAGE: Cell<Option<&'static str>> = const { Cell::new(None) }; }

static DESTINATION: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();
static TOTALS: Mutex<BTreeMap<&'static str, (u64, u128)>> = Mutex::new(BTreeMap::new());
static CYCLE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn destination() -> Option<&'static std::path::Path> {
    DESTINATION.get_or_init(|| std::env::var_os("TEKT_JOINT_PHASE_FILE").map(Into::into)).as_deref()
}
pub struct Guard { label: &'static str, start: Option<Instant>, previous: Option<Option<&'static str>>, thread_bound: PhantomData<Rc<()>> }
pub fn enter(label: &'static str) -> Guard {
    if destination().is_none() {
        return Guard { label, start: None, previous: None, thread_bound: PhantomData };
    }
    let stage = match label {
        "pipeline.eval" => Some("font.eval"),
        "pipeline.introspect" => Some("font.introspect"),
        "pipeline.context" => Some("font.context"),
        "pipeline.layout" => Some("font.layout"),
        "pipeline.finish" => Some("font.finish"),
        _ => None,
    };
    let previous = stage.map(|stage| ACTIVE_STAGE.with(|active| active.replace(Some(stage))));
    Guard { label, start: Some(Instant::now()), previous, thread_bound: PhantomData }
}
impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            let ns = start.elapsed().as_nanos();
            let font_stage = if self.label == "catalog.initialize" {
                Some(ACTIVE_STAGE.with(|active| active.get().unwrap_or("font.outside")))
            } else { None };
            let mut all = TOTALS.lock().unwrap();
            let entry = all.entry(self.label).or_default();
            entry.0 += 1;
            entry.1 += ns;
            if let Some(stage) = font_stage {
                let attributed = all.entry(stage).or_default();
                attributed.0 += 1;
                attributed.1 += ns;
            }
        }
        if let Some(previous) = self.previous {
            ACTIVE_STAGE.with(|active| active.set(previous));
        }
    }
}
/// Called outside all measured scopes, after output publication/diagnostics.
/// Inclusive labels overlap: never sum them into a purported exclusive total.
pub fn measure<T>(label: &'static str, operation: impl FnOnce() -> T) -> T {
    let _guard = enter(label);
    operation()
}
/// Event counts have zero nanoseconds. No clock is read.
pub fn count(label: &'static str, calls: u64) {
    if calls == 0 || destination().is_none() { return }
    let mut all = TOTALS.lock().unwrap();
    all.entry(label).or_default().0 += calls;
}
pub fn mark(label: &'static str) {
    if destination().is_none() { return }
    let mut all = TOTALS.lock().unwrap();
    let entry = all.entry(label).or_default();
    entry.0 += 1;
}
pub fn flush() {
    let Some(path) = destination() else { return };
    mark("event.cycle");
    let totals = std::mem::take(&mut *TOTALS.lock().unwrap());
    let cycle = CYCLE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let output = (|| -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        for (label, (count, ns)) in totals {
            writeln!(file, "{}\t{}\t{}\t{}\t{}", std::process::id(), cycle, label, count, ns)?;
        }
        Ok(())
    })();
    if let Err(error) = output { eprintln!("lab phase probe: {error}"); }
}
