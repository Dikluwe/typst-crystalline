//! Public-API catalog comparison, not a measurement of the entire watch loop.
//! Arguments: INPUT [ROUNDS=25]. Each sample uses a fresh real SystemWorld.
//! Mode fresh configures with_fonts_and_system; mode session shares one
//! FontCatalogSession across worlds (first session sample cold, later warm).
//! Rounds alternate mode order. No pipeline Timings/oracle entrypoint is used.
use std::hint::black_box;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use typst_core::contracts::world::World;
use typst_infra::world::{FontCatalogSession, SystemWorld};

fn measure<T>(operation: impl FnOnce() -> T) -> (T, u128) {
    let start = Instant::now();
    let value = operation();
    (value, start.elapsed().as_nanos())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::canonicalize(args.get(1).expect("usage: catalog_harness INPUT [ROUNDS]"))
        .expect("canonical input");
    let rounds: usize = args.get(2).map(|v| v.parse().expect("numeric rounds")).unwrap_or(25);
    let session = Arc::new(FontCatalogSession::new());
    let mut results = Vec::with_capacity(rounds * 2);
    for round in 0..rounds {
        for session_mode in if round % 2 == 0 { [false, true] } else { [true, false] } {
            let (world, config_ns) = measure(|| {
                let world = SystemWorld::new(input.parent().unwrap_or(Path::new(".")), &input)
                    .expect("World");
                if session_mode {
                    world.with_font_catalog_session(Arc::clone(&session), &[])
                } else {
                    world.with_fonts_and_system(&[])
                }
            });
            let (source, source_ns) = measure(|| world.source(world.main()).expect("Source"));
            let (faces, catalog_ns) = measure(|| black_box(world.book()).len());
            black_box(&source);
            results.push((round, if session_mode { "session" } else { "fresh" },
                config_ns, source_ns, catalog_ns, faces));
            // Optional only in builds that contain the independent probe
            // overlay. Flush is outside the measured catalog/config/Source.
            #[cfg(watch_phase_probe)]
            typst_infra::lab_watch_phase_probe::flush();
        }
    }
    println!("round\tmode\tworld_config_ns\tsource_ns\tfirst_book_ns\tfaces");
    for (round, mode, config, source, catalog, faces) in results {
        println!("{round}\t{mode}\t{config}\t{source}\t{catalog}\t{faces}");
    }
}
