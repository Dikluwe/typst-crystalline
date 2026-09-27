//! Reject unoptimized runs before Criterion can emit measurements.

pub fn require_optimized() {
    assert!(
        matches!(env!("TYPST_BENCH_OPT_LEVEL"), "1" | "2" | "3" | "s" | "z")
            && !cfg!(debug_assertions),
        "benchmarks require optimization and disabled debug assertions; use cargo bench"
    );
}
