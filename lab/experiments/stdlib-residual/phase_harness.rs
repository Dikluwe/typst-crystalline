//! Standalone LAB harness against original/eager/diagnostic public rlibs.
//! Arguments: INPUT [ROUNDS=25]. All clocks/I/O are outside L1.
//! Each sample uses two independent, newly configured SystemWorlds:
//! 1. config → Source → first book() → standalone eval/drop;
//! 2. config → Source → DEFAULT PDF compilation/drop, with lazy fonts inside.
//! These lanes are NOT a partition: default PDF repeats eval and font work.
//! No pipeline with_timings/oracle entrypoint is used. No PDF file is written.
//! Reports are buffered until all measured phases finish. System/page caches
//! remain naturally warm across rounds, as in the paired process benchmark.
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;
use typst_core::contracts::world::World;
use typst_infra::export::{PdfTags, StreamMode};
use typst_infra::pipeline::{compile_to_pdf_bytes, eval_to_module_with_sink};
use typst_infra::world::SystemWorld;

fn measured<T>(operation: impl FnOnce() -> T) -> (T, u128) {
    let start = Instant::now();
    let value = operation();
    (value, start.elapsed().as_nanos())
}
fn configured(input: &Path) -> SystemWorld {
    SystemWorld::new(input.parent().expect("input parent"), input)
        .expect("SystemWorld construction")
        .with_fonts_and_system(&[])
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::canonicalize(args.get(1).expect("usage: phase_harness INPUT [ROUNDS]"))
        .expect("canonical input");
    let rounds: usize = args.get(2).map(|s| s.parse().expect("numeric rounds")).unwrap_or(25);
    let mut records = Vec::with_capacity(rounds);
    for round in 0..rounds {
        let (world, config) = measured(|| configured(&input));
        let (source, source_ns) = measured(|| world.source(world.main()).expect("main Source"));
        let (_, catalog) = measured(|| { black_box(world.book()); });
        let ((eval_ok, eval_warnings), eval_drop) = measured(|| {
            let (result, warnings) = eval_to_module_with_sink(&world, &source);
            let ok = result.is_ok();
            let count = warnings.len();
            black_box(&result);
            drop(result);
            drop(warnings);
            (ok, count)
        });
        assert!(eval_ok, "standalone eval failed");
        drop(source);
        drop(world);
        // Fresh world: default compile includes its own lazy catalog and imports.
        let (world, pdf_config) = measured(|| configured(&input));
        let (source, pdf_source) = measured(|| world.source(world.main()).expect("PDF Source"));
        let ((pdf_ok, pdf_len, pdf_warnings), default_pdf_drop) = measured(|| {
            let (result, warnings) = compile_to_pdf_bytes(
                &world, &source, StreamMode::Verbose, PdfTags::Disabled,
            );
            let ok = result.is_ok();
            let len = result.as_ref().map_or(0, Vec::len);
            let count = warnings.len();
            black_box(&result);
            drop(result);
            drop(warnings);
            (ok, len, count)
        });
        assert!(pdf_ok, "default PDF compilation failed");
        records.push((round, config, source_ns, catalog, eval_drop, pdf_config,
            pdf_source, default_pdf_drop, eval_warnings, pdf_warnings, pdf_len));
    }
    println!("round\teval_world_config_ns\teval_source_ns\teval_first_catalog_ns\tstandalone_eval_and_drop_ns\tpdf_world_config_ns\tpdf_source_ns\tdefault_pdf_inclusive_and_drop_ns\teval_warnings\tpdf_warnings\tpdf_bytes");
    for (r, c, s, f, e, pc, ps, p, ew, pw, n) in records {
        println!("{r}\t{c}\t{s}\t{f}\t{e}\t{pc}\t{ps}\t{p}\t{ew}\t{pw}\t{n}");
    }
}
