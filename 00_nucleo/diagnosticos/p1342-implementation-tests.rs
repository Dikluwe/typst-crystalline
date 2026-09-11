// P1342 independent A/B RED tests.
//
// This file is included only by `typst-infra` under
// `cfg(all(test, p1339_observation))`. It intentionally defines no ledger,
// carrier, hook, projection, or production facade. The implementation must
// provide the one test-only facade named below from the real pipeline owner.

use super::*;
use serde_json::Value;
use std::fs;
use std::process::Command;

const FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../00_nucleo/diagnosticos/p1342-contract-fixture-r1.typ"
));

const CHECKER_R3_SHA256: &str =
    "047dd04c811fd7672143bdc253b759450d2f61d548cc3b49dd3fbcbce71791b7";
const CONTRACT_R2_SHA256: &str =
    "62d656241ecd96cd3d984025e59990d607a0efd75c3c326503c7ec07a3486c4d";
const BINDING_R2_SHA256: &str =
    "3b522cf1b0f288fe1e472f21f775089541835ede754250a14a3c2d17c17e9d48";
const FIXTURE_R1_SHA256: &str =
    "98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714";
const L0_FREEZE_R1_SHA256: &str =
    "2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e";

fn fresh_challenges() -> [[u8; 32]; 3] {
    let mut challenges = [[0_u8; 32]; 3];
    for challenge in &mut challenges {
        getrandom::getrandom(challenge).expect("the independent test driver needs 32 random bytes");
    }
    assert_ne!(challenges[0], challenges[1]);
    assert_ne!(challenges[0], challenges[2]);
    assert_ne!(challenges[1], challenges[2]);
    challenges
}

fn run_sealed_checker(dto: &Value, evidence: &Value) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let nonce = std::process::id();
    let temp = std::env::temp_dir().join(format!("p1342-ab-{nonce}"));
    fs::create_dir_all(&temp).expect("create isolated P1342 checker directory");
    let dto_path = temp.join("dto.json");
    let evidence_path = temp.join("evidence.json");
    fs::write(&dto_path, serde_json::to_vec(dto).unwrap()).unwrap();
    fs::write(&evidence_path, serde_json::to_vec(evidence).unwrap()).unwrap();

    // Import and call the sealed R3 judge directly. This deliberately does not
    // duplicate its evolving schema, carrier, cardinality, or opacity rules.
    let script = r#"
import hashlib, importlib.util, json, pathlib, sys
root, dto_path, evidence_path = map(pathlib.Path, sys.argv[1:4])
checker = root / '00_nucleo/diagnosticos/p1342-oracle-checker-r3.py'
contract_path = root / '00_nucleo/diagnosticos/p1342-contract-spec-r2.json'
binding_path = root / '00_nucleo/diagnosticos/p1342-contract-binding-r2.json'
fixture_path = root / '00_nucleo/diagnosticos/p1342-contract-fixture-r1.typ'
freeze_path = root / '00_nucleo/diagnosticos/p1342-l0-freeze-r1.json'
expected = {
  checker: sys.argv[4], contract_path: sys.argv[5], binding_path: sys.argv[6],
  fixture_path: sys.argv[7], freeze_path: sys.argv[8],
}
for path, digest in expected.items():
  actual = hashlib.sha256(path.read_bytes()).hexdigest()
  if actual != digest: raise SystemExit(f'protected input changed: {path}: {actual}')
spec = importlib.util.spec_from_file_location('p1342_checker_r3_sealed', checker)
module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
dto = json.loads(dto_path.read_text()); evidence = json.loads(evidence_path.read_text())
contract = json.loads(contract_path.read_text()); binding = json.loads(binding_path.read_text())
pins = {'contract': sys.argv[5], 'binding': sys.argv[6], 'fixture': sys.argv[7], 'l0_freeze': sys.argv[8]}
actual = module.judge(dto, evidence, module.digest(evidence), contract, binding, pins)
if actual != 'Preserved': raise SystemExit(f'expected Preserved, got {actual}')
"#;
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .arg(&root)
        .arg(&dto_path)
        .arg(&evidence_path)
        .arg(CHECKER_R3_SHA256)
        .arg(CONTRACT_R2_SHA256)
        .arg(BINDING_R2_SHA256)
        .arg(FIXTURE_R1_SHA256)
        .arg(L0_FREEZE_R1_SHA256)
        .output()
        .expect("execute sealed P1342 R3 checker");
    let _ = fs::remove_dir_all(&temp);
    assert!(
        output.status.success(),
        "sealed checker rejected the productive ledger: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn p1342_fixture_reaches_real_pipeline_with_fresh_raw_evidence() {
    // Three different World/Source owners make normal, repeat and reverse real
    // executions, not three projections of one captured run.
    let normal = mock_world(FIXTURE);
    let repeat = mock_world(FIXTURE);
    let reverse = mock_world(FIXTURE);
    let challenges = fresh_challenges();

    // Required cfg-only facade contract:
    //   fn p1342_run_fixture_for_test(
    //       runs: [(&dyn World, &Source); 3],
    //       challenges: [[u8; 32]; 3],
    //   ) -> Result<String, String>
    //
    // It must compile each supplied source through the real paged pipeline in
    // normal/repeat/reverse order and return one closed JSON envelope with
    // exactly: schema, raw_before_projection, dto, evidence. The raw array is
    // captured before projection and corresponds 1:1 to dto.runs.
    let encoded = context_stabilization::p1342_run_fixture_for_test(
        [
            (&normal as &dyn World, &normal.source),
            (&repeat as &dyn World, &repeat.source),
            (&reverse as &dyn World, &reverse.source),
        ],
        challenges,
    )
    .expect("P1342 productive fixture run");

    let envelope: Value = serde_json::from_str(&encoded).expect("closed P1342 facade JSON");
    let object = envelope.as_object().expect("P1342 envelope object");
    let keys: std::collections::BTreeSet<_> = object.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        ["dto", "evidence", "raw_before_projection", "schema"]
            .into_iter()
            .collect()
    );
    assert_eq!(envelope["schema"], "p1342-ab-facade-v1");

    let runs = envelope["dto"]["runs"].as_array().expect("three DTO runs");
    let raw = envelope["raw_before_projection"]
        .as_array()
        .expect("three preprojection raw snapshots");
    assert_eq!(runs.len(), 3);
    assert_eq!(raw.len(), 3);
    assert_eq!(
        runs.iter().map(|run| run["mode"].as_str().unwrap()).collect::<Vec<_>>(),
        ["normal", "repeat", "reverse"]
    );
    for (run, before) in runs.iter().zip(raw) {
        assert_eq!(&run["raw_snapshot"], before);
        assert_eq!(run["projection"]["raw_digest_before"], before["raw_digest"]);
        assert_eq!(run["projection"]["raw_digest_after"], before["raw_digest"]);
    }

    run_sealed_checker(&envelope["dto"], &envelope["evidence"]);
}
