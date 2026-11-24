// Linux x86_64 laboratory probe. Scope totals are thread CPU rather than
// elapsed time. Each publication resets its own counters.
use std::cell::Cell;
use std::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    seconds: c_long,
    nanos: c_long,
}

unsafe extern "C" {
    fn clock_gettime(clock: c_int, output: *mut Timespec) -> c_int;
}

fn cpu_ns() -> u64 {
    let mut time = Timespec { seconds: 0, nanos: 0 };
    assert_eq!(unsafe { clock_gettime(3, &mut time) }, 0);
    time.seconds as u64 * 1_000_000_000 + time.nanos as u64
}

#[derive(Clone, Copy)]
pub(crate) enum Stage {
    FontDiscover,
    FontResolveBind,
    FontPartition,
    FontScan,
    FontSubset,
    PageOperators,
    PageCompression,
    ResourceStreams,
    TagMcids,
    TagTree,
    Serialize,
}
const N: usize = Stage::Serialize as usize + 1;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static TOTALS: Cell<[u64; N]> = const { Cell::new([0; N]) };
    static COUNTS: Cell<[u64; N]> = const { Cell::new([0; N]) };
}

pub(crate) struct Publication(u64);
impl Publication {
    pub(crate) fn begin() -> Option<Self> {
        if std::env::var_os("TEKT_LAB_PDF_CPU").is_none() {
            return None;
        }
        TOTALS.set([0; N]);
        COUNTS.set([0; N]);
        assert!(!ACTIVE.replace(true));
        Some(Self(cpu_ns()))
    }
}
impl Drop for Publication {
    fn drop(&mut self) {
        let total = cpu_ns() - self.0;
        ACTIVE.set(false);
        eprintln!("pdf-cpu {total} {:?} {:?}", TOTALS.get(), COUNTS.get());
    }
}

pub(crate) struct Scope {
    stage: Stage,
    start: u64,
}
pub(crate) fn scope(stage: Stage) -> Option<Scope> {
    ACTIVE.get().then(|| Scope { stage, start: cpu_ns() })
}
impl Drop for Scope {
    fn drop(&mut self) {
        let elapsed = cpu_ns() - self.start;
        let index = self.stage as usize;
        let mut totals = TOTALS.get();
        totals[index] += elapsed;
        TOTALS.set(totals);
        let mut counts = COUNTS.get();
        counts[index] += 1;
        COUNTS.set(counts);
    }
}
