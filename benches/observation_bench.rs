//! Guarded entry point for the existing private ObservationEq Criterion test.
#[path = "support/core_runner.rs"]
mod core_runner;
#[path = "support/profile.rs"]
mod profile;

fn main() -> Result<(), String> {
    core_runner::run(
        "compiler::eval::observation::benchmarks::observation_microbenchmarks",
    )
}
