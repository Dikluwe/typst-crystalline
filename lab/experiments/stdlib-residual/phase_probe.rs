//! LAB ONLY. Inclusive L3 scopes, deliberately independent of pipeline Timings.
//! TEKT_LAB_PHASE_FILE enables append-only TSV flushes after publication. Unset
//! means no clocks, mutex, file creation or output. The OnceLock opt-in check
//! remains, and must be calibrated with the same executable on/off.
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

static DESTINATION: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();
static TOTALS: Mutex<BTreeMap<&'static str, (u64, u128)>> = Mutex::new(BTreeMap::new());
static CYCLE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn destination() -> Option<&'static std::path::Path> {
    DESTINATION.get_or_init(|| std::env::var_os("TEKT_LAB_PHASE_FILE").map(Into::into)).as_deref()
}
pub struct Guard { label: &'static str, start: Option<Instant> }
pub fn enter(label: &'static str) -> Guard {
    Guard { label, start: destination().map(|_| Instant::now()) }
}
impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            let ns = start.elapsed().as_nanos();
            let mut all = TOTALS.lock().unwrap();
            let entry = all.entry(self.label).or_default();
            entry.0 += 1;
            entry.1 += ns;
        }
    }
}
/// Called outside all measured scopes, after output publication/diagnostics.
/// Inclusive labels overlap: never sum them into a purported exclusive total.
pub fn flush() {
    let Some(path) = destination() else { return };
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
