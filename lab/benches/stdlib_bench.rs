//! Guarded entry point for the existing private stdlib Criterion test.
#[path = "support/core_runner.rs"]
mod core_runner;
#[path = "support/profile.rs"]
mod profile;

fn main() -> Result<(), String> {
    core_runner::run(
        "compiler::stdlib::registry::microbenchmarks::benchmarks::stdlib_microbenchmarks",
    )
}
