//! Standalone allocation observations of the real public query serializer.
//! Requires only std, typst_core and typst_shell. Counter overhead precludes
//! timing claims; all allocation counters are process-global and never reset.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use typst_core::entities::value::Value;
use typst_shell::cli::{serialize_query_with_format, QueryFormat};

struct CountingSystem;
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static REALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static REQUESTED: AtomicU64 = AtomicU64::new(0);
static RELEASED: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);

fn acquired(bytes: usize) {
    REQUESTED.fetch_add(bytes as u64, Relaxed);
    let live = LIVE.fetch_add(bytes as u64, Relaxed) + bytes as u64;
    PEAK.fetch_max(live, Relaxed);
}
fn released(bytes: usize) {
    RELEASED.fetch_add(bytes as u64, Relaxed);
    LIVE.fetch_sub(bytes as u64, Relaxed);
}
// All operations delegate to System unchanged. Only successful requests affect
// counters. Realloc records old release/new request and updates live by the net
// delta, avoiding a synthetic peak where both buffers appear simultaneously live.
unsafe impl GlobalAlloc for CountingSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() { ALLOCATIONS.fetch_add(1, Relaxed); acquired(layout.size()); }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() { ALLOCATIONS.fetch_add(1, Relaxed); acquired(layout.size()); }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCATIONS.fetch_add(1, Relaxed);
        released(layout.size());
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, size) };
        if !new_ptr.is_null() {
            REALLOCATIONS.fetch_add(1, Relaxed);
            REQUESTED.fetch_add(size as u64, Relaxed);
            RELEASED.fetch_add(layout.size() as u64, Relaxed);
            let live = if size >= layout.size() {
                LIVE.fetch_add((size - layout.size()) as u64, Relaxed) + (size - layout.size()) as u64
            } else {
                LIVE.fetch_sub((layout.size() - size) as u64, Relaxed) - (layout.size() - size) as u64
            };
            PEAK.fetch_max(live, Relaxed);
        }
        new_ptr
    }
}
#[global_allocator]
static ALLOCATOR: CountingSystem = CountingSystem;

#[derive(Clone, Copy)]
struct Snapshot { allocs: u64, deallocs: u64, reallocs: u64, requested: u64, released: u64, live: u64, peak: u64 }
fn snapshot() -> Snapshot {
    Snapshot { allocs: ALLOCATIONS.load(Relaxed), deallocs: DEALLOCATIONS.load(Relaxed),
        reallocs: REALLOCATIONS.load(Relaxed), requested: REQUESTED.load(Relaxed),
        released: RELEASED.load(Relaxed), live: LIVE.load(Relaxed), peak: PEAK.load(Relaxed) }
}
fn report(label: &str, before: Snapshot, after: Snapshot) {
    println!("alloc\t{label}\tallocs={}\tdeallocs={}\treallocs={}\trequested_bytes={}\treleased_bytes={}\tlive_before={}\tlive_after={}\tlive_delta={}\tprocess_peak_before={}\tprocess_peak_after={}",
        after.allocs - before.allocs, after.deallocs - before.deallocs, after.reallocs - before.reallocs,
        after.requested - before.requested, after.released - before.released, before.live, after.live,
        i128::from(after.live) - i128::from(before.live), before.peak, after.peak);
}
#[path = "../allocator_selftest.rs"]
mod allocator_selftest;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|arg| arg == "allocator-selftest") {
        allocator_selftest::run();
        return;
    }
    if args.iter().any(|arg| arg == "--help") {
        println!("profile_query [empty|allocator-selftest]\nNo arguments: real serialize_query_with_format on 16/256/4096 Unicode dictionaries,\n100 calls per JSON compact/JSON pretty/YAML and full-value/index-field case.\nempty: snapshot overhead control. Counters are global; no durations are reported.\nSnapshot deltas exclude input/label construction and output printing.\nPeak is process-cumulative; output byte totals sum actual serializer outputs.\nOnly std, typst_core and typst_shell are linked directly.");
        return;
    }
    if args.get(1).is_some_and(|arg| arg == "empty") {
        let before = snapshot();
        let after = snapshot();
        report("empty", before, after);
        return;
    }
    if args.len() != 1 {
        eprintln!("unknown argument; use --help");
        std::process::exit(2);
    }
    for entries in [16usize, 256, 4096] {
        let elements: Vec<Value> = (0..entries).map(|index| {
            Value::Dict([
                ("index".into(), Value::Int(index as i64)),
                ("text".into(), Value::Str(format!("ação αβ 😀 <&>\"quote\" {index}").into())),
            ].into_iter().collect())
        }).collect();
        for field in [None, Some("index")] {
            for (name, format, pretty) in [
                ("json_compact", QueryFormat::Json, false),
                ("json_pretty", QueryFormat::Json, true),
                ("yaml", QueryFormat::Yaml, false),
            ] {
                let label = format!("query.entries={entries}.format={name}.field={}.one=false.calls=100", field.unwrap_or("none"));
                let mut output_bytes = 0u64;
                let before = snapshot();
                for _ in 0..100 {
                    let bytes = serialize_query_with_format(&elements, field, false, pretty, format)
                        .expect("serialize real query values");
                    output_bytes = output_bytes.checked_add(bytes.len() as u64).expect("output length overflow");
                    drop(std::hint::black_box(bytes));
                }
                let after = snapshot();
                report(&label, before, after);
                println!("output\t{label}\tbytes_total={output_bytes}");
            }
        }
    }
}
