//! Diagnostic-only probe; existing API, no production instrumentation.
use std::hash::{Hash, Hasher};
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let (slots, book) = std::hint::black_box(typst_infra::fontdb::load_system_fonts());
    let discover_ms = start.elapsed().as_secs_f64() * 1000.0;
    // Equality control stays outside the discovery interval.
    let mut digest = std::collections::hash_map::DefaultHasher::new();
    book.infos().hash(&mut digest);
    let count = book.len();
    let metadata_hash = digest.finish();
    let start = Instant::now();
    drop((slots, book));
    let drop_ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("{{\"discover_ms\":{discover_ms},\"drop_ms\":{drop_ms},\"faces\":{count},\"metadata_control\":\"{metadata_hash:016x}\"}}");
}
