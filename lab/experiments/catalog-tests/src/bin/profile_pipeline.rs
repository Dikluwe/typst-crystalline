//! Whole-pipeline allocation observations through public, real APIs.
//! This binary's allocation counters perturb execution: its durations are never
//! performance evidence. There is no alternative implementation or optimization.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use typst_core::contracts::world::World;
use typst_core::entities::{content::Content, layout_types::{FrameItem, TextStyle},
    module::Module, scope::Scope, style_chain::StyleChain, value::Value};
use typst_infra::{export::{PdfTags, StreamMode}, pipeline, world::SystemWorld};

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
fn sizes() {
    macro_rules! size { ($($t:ty),*) => { $(println!("carrier\t{}\tsize={}\talign={}", stringify!($t), std::mem::size_of::<$t>(), std::mem::align_of::<$t>());)* }; }
    size!(Content, Value, FrameItem, TextStyle, StyleChain, Scope, Module);
}
fn clone_cases() {
    for entries in [16usize, 256, 4096] {
        let base = Value::Dict((0..entries).map(|i| (
            format!("entry-{i}").into(),
            Value::Array(vec![Value::Int(i as i64), Value::Str(format!("ação-αβ-😀-{i}").into())]),
        )).collect());
        let label = format!("dict.entries={entries}.clones=100.array_values=2");
        let before = snapshot();
        for _ in 0..100 { drop(std::hint::black_box(base.clone())); }
        let after = snapshot();
        report(&label, before, after);
    }
}
fn cst_cases() {
    use typst_core::entities::{syntax_kind::SyntaxKind, syntax_node::SyntaxNode};
    for depth in [0usize, 16, 64, 256] {
        let leaves = (0..1024).map(|_| SyntaxNode::leaf(SyntaxKind::Text, "ação αβ 😀\n")).collect();
        let mut base = SyntaxNode::inner(SyntaxKind::Markup, leaves);
        for _ in 0..depth { base = SyntaxNode::inner(SyntaxKind::Markup, vec![base]); }
        let label = format!("cst.wrappers={depth}.unicode_leaves=1024.clone_into_text=100");
        let before = snapshot();
        for _ in 0..100 { drop(std::hint::black_box(base.clone()).into_text()); }
        let after = snapshot();
        report(&label, before, after);
    }
}
fn diagnostic_cases() {
    use typst_core::entities::{source::Source, source_result::SourceDiagnostic, span::Span};
    use typst_shell::diagnostic::{DiagnosticSource, format_diagnostic};
    const LINE: &str = "ação αβ 😀\n";
    for lines in [64usize, 4096, 65536] {
        let source = Source::detached(LINE.repeat(lines)).expect("parse diagnostic source");
        let start = (lines / 2) * LINE.len();
        let span = Span::from_range(source.id(), start..start + "ação".len());
        let diagnostic = SourceDiagnostic::error(span, "diagnóstico Unicode αβ 😀")
            .with_hint("verifique ação, αβ e 😀");
        let sources = [DiagnosticSource::new(source, "diagnóstico-αβ.typ")];
        let label = format!("diagnostic.lines={lines}.unicode_hint=true.formats=100");
        let before = snapshot();
        for _ in 0..100 {
            drop(std::hint::black_box(format_diagnostic(&diagnostic, &sources, false)));
        }
        let after = snapshot();
        report(&label, before, after);
    }
}
#[path = "../allocator_selftest.rs"]
mod allocator_selftest;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|arg| arg == "allocator-selftest") {
        allocator_selftest::run();
        return;
    }
    if args.len() == 1 || args.iter().any(|a| a == "--help") {
        println!("profile_pipeline MODE [INPUT] [ROUNDS] [embedded|system] [same|fresh]\nMODE: empty|sizes|clones|cst|diagnostic|eval|pdf|png|svg|html\nallocator-selftest: explicit zeroed/grow/shrink/free controls\nempty: snapshot overhead control; sizes: public carrier size/alignment\nclones/cst/diagnostic: fixed dimensioned allocation cases, no INPUT required\nOther modes use real public APIs; default 1 round, embedded fonts, same World.\nhtml explicitly enables Features::html() in this experiment.\nAll snapshots are whole-process allocator totals; deltas include concurrent work.\nCounts exclude mmap/foreign allocator memory. No duration is a performance result.\nPeak is process cumulative; live_delta is net bytes, not proof of leaked ownership.\nOutput allocations occur after each snapshot; no live counter is reset.");
        return;
    }
    if args[1] == "sizes" { sizes(); return; }
    if args[1] == "clones" { clone_cases(); return; }
    if args[1] == "cst" { cst_cases(); return; }
    if args[1] == "diagnostic" { diagnostic_cases(); return; }
    if args[1] == "empty" {
        let before = snapshot();
        let after = snapshot();
        report("empty", before, after);
        return;
    }
    if !matches!(args[1].as_str(), "eval" | "pdf" | "png" | "svg" | "html") {
        eprintln!("unknown mode; use --help"); std::process::exit(2);
    }
    let input = std::fs::canonicalize(args.get(2).expect("INPUT required")).expect("resolve INPUT");
    let rounds: usize = args.get(3).map(|s| s.parse().expect("integer ROUNDS")).unwrap_or(1);
    assert!((1..=1000).contains(&rounds), "ROUNDS must be 1..1000");
    let fonts = args.get(4).map(String::as_str).unwrap_or("embedded");
    assert!(matches!(fonts, "embedded" | "system"), "fonts must be embedded|system");
    let world_policy = args.get(5).map(String::as_str).unwrap_or("same");
    assert!(matches!(world_policy, "same" | "fresh"), "world must be same|fresh");
    let make_world = || {
        let world = SystemWorld::new(input.parent().unwrap(), input.file_name().unwrap()).expect("create real World");
        if fonts == "system" { world.with_fonts_and_system(&[]) } else { world.with_embedded_fonts() }
    };
    let before = snapshot();
    let mut world = make_world();
    let after = snapshot();
    report("initial_world", before, after);
    for round in 0..rounds {
        if round > 0 && world_policy == "fresh" {
            let before = snapshot();
            world = make_world();
            let after = snapshot();
            report("replace_world", before, after);
        }
        let before = snapshot();
        let source = world.source(world.main()).expect("read real Source");
        let after = snapshot();
        report("source", before, after);
        let before = snapshot();
        // Retain result through the middle snapshot; then drop it and observe
        // residual global allocations without resetting allocator live state.
        let (output_bytes, errors, warnings, after_output, after_drop) = match args[1].as_str() {
            "eval" => {
                let (result, warnings) = pipeline::eval_to_module_with_sink(&world, &source);
                let errors = result.as_ref().err().map_or(0, Vec::len);
                let after_output = snapshot();
                let warnings_len = warnings.len();
                drop((result, warnings));
                (0, errors, warnings_len, after_output, snapshot())
            }
            mode @ ("pdf" | "png") => {
                let (result, warnings) = if mode == "png" {
                    pipeline::compile_to_png_bytes(&world, &source)
                } else {
                    pipeline::compile_to_pdf_bytes(&world, &source, StreamMode::Verbose, PdfTags::Enabled)
                };
                let bytes = result.as_ref().map_or(0, Vec::len);
                let errors = result.as_ref().err().map_or(0, Vec::len);
                let after_output = snapshot();
                let warnings_len = warnings.len();
                drop((result, warnings));
                (bytes, errors, warnings_len, after_output, snapshot())
            }
            mode => {
                let (result, warnings) = if mode == "svg" {
                    pipeline::compile_to_svg_string(&world, &source)
                } else { pipeline::compile_to_html_string_with_features(
                    &world, &source, typst_core::entities::compiler_features::Features::html(),
                ) };
                let bytes = result.as_ref().map_or(0, String::len);
                let errors = result.as_ref().err().map_or(0, Vec::len);
                let after_output = snapshot();
                let warnings_len = warnings.len();
                drop((result, warnings));
                (bytes, errors, warnings_len, after_output, snapshot())
            }
        };
        report("compile_result_live", before, after_output);
        report("compile_result_dropped", before, after_drop);
        println!("result\tround={round}\tbytes={output_bytes}\terrors={errors}\twarnings={warnings}");
        if errors != 0 { std::process::exit(1); }
    }
    let before = snapshot();
    drop(world);
    let after = snapshot();
    report("world_dropped", before, after);
}
