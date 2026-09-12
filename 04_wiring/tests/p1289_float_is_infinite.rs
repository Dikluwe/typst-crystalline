//! @prompt 00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md
//! @prompt-hash 16eb6650

use std::path::PathBuf;
use std::process::Command;

#[test]
fn p1289_float_is_infinite_preserves_the_frozen_v3_oracle() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("04_wiring must have a repository parent")
        .to_path_buf();
    let runner = repo.join("lab/surface-inventory/run_p1289_oracles.py");
    let baseline = repo.join("lab/surface-inventory/p1289-oracle-baseline.json");
    let candidate = env!("CARGO_BIN_EXE_typst");

    let output = Command::new("python3")
        .arg(&runner)
        .arg("--candidate")
        .arg(candidate)
        .arg("--baseline")
        .arg(&baseline)
        .arg("--summary-only")
        .output()
        .expect("the independent P1289 oracle runner must execute");

    assert!(
        output.status.success(),
        "P1289 v3 oracle must report Preserved with zero Unknown/Violated.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"verdict\": \"Preserved\""));
    assert!(stdout.contains("\"Unknown\": 0"));
    assert!(stdout.contains("\"Violated\": 0"));
    assert!(stdout.contains("\"mutation_score\": 1.0"));
}
