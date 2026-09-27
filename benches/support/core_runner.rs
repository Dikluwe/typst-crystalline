//! Access private Criterion tests through verified core test executables.

pub fn run(test: &str) -> Result<(), String> {
    super::profile::require_optimized();
    let script =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("run_core_benchmark.py");
    let status = std::process::Command::new("python3")
        .arg(script)
        .arg(test)
        .args(std::env::args().skip(1))
        .status()
        .map_err(|error| format!("cannot start guarded core runner: {error}"))?;
    if !status.success() {
        return Err(format!("guarded core runner failed: {status}"));
    }
    Ok(())
}
