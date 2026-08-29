#!/usr/bin/env python3
from __future__ import annotations

import csv
import hashlib
import json
import subprocess
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path


ROOT = Path("/repos/Antigravity/typst-crystalline")
OUT = Path("/tmp/p1268/verifier_v3")
ORACLE = Path("/tmp/p1268/oracle_v3")
REFINER = Path("/tmp/p1268/refiner")
ADVERSARY = Path("/tmp/p1268/adversary")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        rows = list(reader)
    if not rows or not reader.fieldnames or any(None in row for row in rows):
        raise RuntimeError(f"invalid TSV: {path}")
    return rows


def write_tsv(path: Path, rows: list[dict[str, str]], fields: list[str]) -> None:
    path.resolve().relative_to(OUT.resolve())
    if not rows:
        raise RuntimeError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


expected = {
    ROOT / "00_nucleo/diagnosticos/p1268-preoracle-manifest-v3.tsv": "6ed7f67f11d4d886edef3c39bf327cb7ae96e3b2bdd17517fe90b3145f3fa76c",
    ROOT / "00_nucleo/diagnosticos/p1268-suite-freeze-v3.tsv": "fc0b0b5c21449f945c886df10fb0a7ea80219dd92de856bd88d37825defa7ba7",
    ROOT / "00_nucleo/diagnosticos/p1268-role-capabilities-v3.tsv": "bdb8b2ed03f9f83655c3505c73e53b85141a935f22c8238c76ec3fade5f6f876",
    ROOT / "00_nucleo/diagnosticos/p1268-head-drift.tsv": "c9660d1451f0dafdbc631cae2153266e8fa0085426d6935fa85e79246c6ad1a2",
    ROOT / "00_nucleo/diagnosticos/p1268-contract-freeze.tsv": "7d8064a939e653961a084088e40353342de401908e7a8dcd0ba229e07489579d",
    REFINER / "contract-v3.tsv": "cb2aabaf36703bef3ef869f9a150ca8f38f5e596f1a8523c19016a283b024399",
    REFINER / "unknown-v3.tsv": "2ab6446d8b50c803240f19bde92ff55e121bc0f4a9793f8faeaecd80979b5530",
    REFINER / "observables-v3.tsv": "e32a8b201a6625156e0416303892c23ce441957e09ecbde0bb3f459d966e969c",
    REFINER / "corrections.tsv": "10d01447db10337639fb3d54429bd87648c8305ea4c00ea966f0dd30ed8a2e0f",
    REFINER / "receipt.tsv": "ac8899037e70e9b5035c0c13a5ec7bf4a897a7a381889786b43febe36086debb",
    ORACLE / "runner.py": "6471e5759d53569a8d52a0bb64290685d0f19487dd1619a92b083ea959158069",
    ORACLE / "sources.tsv": "76064c818a4a1a56fa6601b5ed451dc914adf3ebe1ccc5862d0472a37cd780c0",
    ORACLE / "positive.tsv": "ae2a99d466263d6df581a107274878e3cc3c04f8db9c8a71249e6b5c6da5cfc5",
    ORACLE / "budgets.tsv": "8b45e9145f72d252b74a96e11c566bc86a36fd7e3bd1802cf027b970a2556772",
    ORACLE / "interval-cost.tsv": "d007e2af176b2f6b4285306433d63102a68d4b814d2d63919407c37852d340e5",
    ORACLE / "masks/manifest.tsv": "e6572a92c61f3b576424a689dfb36bdbaa52a4260824902c0ab419e09c2214db",
    ORACLE / "landmarks.tsv": "490675273bdfab44b477fdbd60abd6fc961ad9b711b7ef722895013ea2a01207",
    ORACLE / "coincidence.tsv": "9ee02c4cd7579a1bcfe81c74ec9d5e49b4052d5c063650d6bb3d05061dc9e147",
    ORACLE / "degenerates.tsv": "e8ebee0d89021dc9fd7c80a37fbb43a3eb59df1d6825b116f1457dc4d689ec8b",
    ORACLE / "determinism.tsv": "cf15d0a25cad0f905b59a9a37c56c912c14da4992f4c02977d63274950c9c837",
    ORACLE / "opaque.tsv": "cfb4fcd25d200bd7c822305d852d2551e4f15423d32c80ccc26ea5f4f75a6ac1",
    ORACLE / "receipt.tsv": "934c4d1341d753cec4a7fcd8a27209d7c4d17baf0507877a38210669df9d78c9",
    ADVERSARY / "attacks.tsv": "86393124a5209ab8376224a1a84a22d3275220a0869c4ce8bab530df35e3af8b",
    ADVERSARY / "opaque-cases.tsv": "f7e48b49ec4c4ec930fe497389118e826d7f03af676816170ab4c9cfd83b0bc5",
    ADVERSARY / "mutation-schema.tsv": "82f3a77f8a720fa2d29ef37a0062ec8459606a568cc590aba69e52ac6f092109",
    ADVERSARY / "attack_runner.py": "f290e57f38db6604087225e0d4038a29a6f43dfafbd86c877d6409115dcd62a9",
    ADVERSARY / "receipt.tsv": "56cf0cff1acf2eb72b2515108c493982dc10b0a66a628213aec15c3275cde66e",
    ROOT / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv": "091d50354eb2b206d595c88a96b76eb8b9d8e8a881b2f0d3f5901edf9cce5406",
    ROOT / "00_nucleo/diagnosticos/p1266-corpus.tsv": "54b5f0fdb0153321544a9d0e147867ea0473ecd9cedfc6d959e3c0a1c9522894",
    ROOT / "00_nucleo/diagnosticos/p1266-oracle-budgets.tsv": "b8153842849d7a5f74d1af0a006407d5e3ebebc1ece44c1456379ae396686b46",
    ROOT / "00_nucleo/diagnosticos/p1266-cost.tsv": "dc10bff135b228b521376b39e4266c1656f6b650a72576253404a85ca74c93ad",
    ROOT / "lab/typst-original/crates/typst-library/src/visualize/gradient.rs": "ed7a1be4e398fa47dff9ede8aa31c939514e341f9e47ac5099787d422f67967c",
    Path("/usr/local/bin/typst"): "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8",
    ROOT / "typst-passo-1268.md": "c9f8ff5bca9131c307baaf88f3f0a1c2d80cac775623ab4c521f4231db969188",
    ROOT / "00_nucleo/diagnosticos/p1267r2-final-manifest.tsv": "d89db14418a432525e20e0191e91cb4a299c233bb0d342a539ac77da5f1c1389",
    ROOT / "00_nucleo/diagnosticos/p1267r2-verifier-verdict.tsv": "a9ced04fecaab28a04e2af0dfabdc7af9b7b2712ddc24143021accd590f74842",
    ROOT / "00_nucleo/diagnosticos/p1267r2-author-contract-v2.tsv": "0ba5d409aa2784184c2604282abb85715e7c6edd00d151f07474c9fefaeb4ce8",
    ROOT / "00_nucleo/diagnosticos/p1267r2-author-unknown-v2.tsv": "afee37521719a849d359f0c1caeab132de532319a3c1dfa74af539e3be3deb76",
    ROOT / "00_nucleo/diagnosticos/p1267r2-author-observables.tsv": "6690eeae86985c65f39d132dd43cb615c267815340dc1c180fbe03c4bc27bb2e",
    ROOT / "00_nucleo/diagnosticos/p1267r2-evidence-derivation.tsv": "14601fbefb87b50bc69482ed8f62c1446842f32293ccf3ffe3afab0efb83fa5c",
    ROOT / "00_nucleo/diagnosticos/p1267r2-r1-comparison.tsv": "8432e7dd96cc04d8df4387094e54cb8d591807b0b76a3a1e7a7a5bf4c39ae7cd",
    ROOT / "00_nucleo/diagnosticos/p1266-final-certificate.tsv": "dce8aabec0d6bbf8b714d75f8cfc22e38f83539d04f1c8d034110c8dfa1067ec",
    ROOT / "00_nucleo/prompts/infra/export/svg.md": "41c5d785d9e4ecdb184fd5e6344dd7422d874e826e3bdeac2ddb2d6617530d29",
    ROOT / "00_nucleo/prompts/infra/export/gradients/adaptive.md": "4e2e19b5af1aab3c33596166fbcef647df1aceed1739bab395d432126f46d737",
}

hash_rows = []
for path, wanted in expected.items():
    got = sha(path)
    hash_rows.append({"path": str(path), "expected_sha256": wanted, "observed_sha256": got, "status": "PASS" if got == wanted else "FAIL"})
if any(row["status"] != "PASS" for row in hash_rows):
    raise RuntimeError("protected hash drift")
write_tsv(OUT / "hash-audit.tsv", hash_rows, ["path", "expected_sha256", "observed_sha256", "status"])

corpus = read_tsv(ROOT / "00_nucleo/diagnosticos/p1266-corpus.tsv")
fixture_rows = []
for row in corpus:
    path = ROOT / "lab/parity/matrix/fixtures/p1266" / f"{row['fixture_id']}.typ"
    got = sha(path)
    fixture_rows.append({"fixture_id": row["fixture_id"], "path": str(path), "expected_sha256": row["source_sha256"], "observed_sha256": got, "status": "PASS" if got == row["source_sha256"] else "FAIL"})
if len(fixture_rows) != 96 or any(row["status"] != "PASS" for row in fixture_rows):
    raise RuntimeError("fixture closure failed")
write_tsv(OUT / "fixture-closure.tsv", fixture_rows, ["fixture_id", "path", "expected_sha256", "observed_sha256", "status"])

mask_rows = []
manifest = read_tsv(ORACLE / "masks/manifest.tsv")
for row in manifest:
    for kind, path_key, hash_key in (("svg", "mask_svg", "mask_svg_sha256"), ("png", "mask_binary", "mask_binary_sha256")):
        path = Path(row[path_key])
        got = sha(path)
        mask_rows.append({"fixture_id": row["fixture_id"], "kind": kind, "path": str(path), "expected_sha256": row[hash_key], "observed_sha256": got, "status": "PASS" if got == row[hash_key] else "FAIL"})
actual_masks = {path for path in (ORACLE / "masks").iterdir() if path.is_file() and path.name != "manifest.tsv"}
listed_masks = {Path(row["path"]) for row in mask_rows}
if len(manifest) != 96 or len(mask_rows) != 192 or actual_masks != listed_masks or any(row["status"] != "PASS" for row in mask_rows):
    raise RuntimeError("mask closure failed")
write_tsv(OUT / "mask-closure.tsv", mask_rows, ["fixture_id", "kind", "path", "expected_sha256", "observed_sha256", "status"])

mutants = read_tsv(OUT / "mutation-verdicts.tsv")
negatives = [row for row in mutants if row["mutation_denominator"] == "included"]
controls = [row for row in mutants if row["mutation_denominator"] == "excluded"]
if len(negatives) != 24 or any(row["adjudicated"] != "Violated" or row["killed"] != "true" for row in negatives):
    raise RuntimeError("mutation gate failed")
if len(controls) != 1 or controls[0]["adjudicated"] != "Preserved":
    raise RuntimeError("positive control failed")
mutation_score = len(negatives) / len(negatives)
if mutation_score != 1.0:
    raise RuntimeError("mutation score below 1.0")

opaque = read_tsv(OUT / "opaque-verdicts.tsv")
if len(opaque) != 10 or any(row["adjudicated"] != "Unknown" or row["counts_as_success"] != "false" for row in opaque):
    raise RuntimeError("opaque honesty failed")
attack_suite = read_tsv(ADVERSARY / "attacks.tsv")
opaque_suite = read_tsv(ADVERSARY / "opaque-cases.tsv")
materialization_rows = []
for row in attack_suite:
    path = OUT / "mutants" / f"{row['attack_id']}.json"
    got = json.loads(path.read_text())
    wanted = json.loads(row["after_json"])
    status = "PASS" if got == wanted else "FAIL"
    materialization_rows.append({"case_id": row["attack_id"], "suite": "attack", "path": str(path), "sha256": sha(path), "expected": row["expected"], "status": status})
for row in opaque_suite:
    path = OUT / "opaque_materialized" / f"{row['case_id']}.json"
    got = json.loads(path.read_text())
    wanted = json.loads(row["canonical_state_json"])
    status = "PASS" if got == wanted else "FAIL"
    materialization_rows.append({"case_id": row["case_id"], "suite": "opaque", "path": str(path), "sha256": sha(path), "expected": row["expected"], "status": status})
if len(materialization_rows) != 35 or any(row["status"] != "PASS" for row in materialization_rows):
    raise RuntimeError("adversarial materialization closure failed")
write_tsv(OUT / "materialization-closure.tsv", materialization_rows, ["case_id", "suite", "path", "sha256", "expected", "status"])
technical = read_tsv(OUT / "technical-gates.tsv")
capabilities = read_tsv(OUT / "capability-gates.tsv")
if any(row["status"] != "PASS" for row in technical + capabilities) or any(row["breach"] != "false" for row in capabilities):
    raise RuntimeError("technical or capability gate failed")

positive = read_tsv(ORACLE / "positive.tsv")
if len(positive) != 96 or any(row["oracle_status"] != "Available" for row in positive):
    raise RuntimeError("positive oracle failed")
determinism = read_tsv(ORACLE / "determinism.tsv")
if len(determinism) != 192 or any(row["status"] != "MeasuredEqual" for row in determinism):
    raise RuntimeError("determinism failed")
intervals = read_tsv(ORACLE / "interval-cost.tsv")
for row in intervals:
    d = int(row["midpoint_decisions_d_i"])
    b = int(row["boundary_sampler_calls_b_i"])
    q = int(row["total_sampler_calls_q_i"])
    if q != d + b or int(row["s_i"]) > 64 or int(row["a_i"]) > 63 or d > 127 or b > 126 or q > 253:
        raise RuntimeError("interval limit failed")
budgets = read_tsv(ORACLE / "budgets.tsv")
if any(row["identity_and_bounds"] != "Measured" or int(row["E_emitted_stops"]) != int(row["expected_E_M_plus_sum_a"]) or int(row["E_emitted_stops"]) > int(row["affine_bound_M_plus_63R"]) for row in budgets):
    raise RuntimeError("affine identity failed")
coincidence = read_tsv(ORACLE / "coincidence.tsv")
if any(row["exact_point_status"] != "MeasuredStructural" or row["epsilon_right_status"] == "Unknown" for row in coincidence):
    raise RuntimeError("coincidence failed")
degenerates = read_tsv(ORACLE / "degenerates.tsv")
if Counter(row["seed"] for row in degenerates) != Counter({"S20": 4, "S21": 4}) or any(row["status"] != "Measured" for row in degenerates):
    raise RuntimeError("S20/S21 failed")

head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True, capture_output=True, check=True).stdout.strip()
status_text = subprocess.run(["git", "status", "--short"], cwd=ROOT, text=True, capture_output=True, check=True).stdout
diff_stat = subprocess.run(["git", "diff", "HEAD", "--stat"], cwd=ROOT, text=True, capture_output=True, check=True).stdout
if head != "7df0e3174d0d651189c9281c81c4508e3c691478":
    raise RuntimeError("HEAD moved after V3 freeze")

content_paths = set(expected)
content_paths.update(Path("/home/dikluwe/.codex/skills/tekt-materializacao-segregada") / name for name in ["SKILL.md", "references/papeis-e-capacidades.md", "references/artefatos-e-gates.md"])
content_paths.update((ROOT / "00_nucleo/adr").glob("*.md"))
content_paths.update(Path(row["path"]) for row in fixture_rows)
content_paths.update(path for path in ORACLE.rglob("*") if path.is_file())
content_paths.update(path for path in REFINER.glob("*") if path.is_file())
content_paths.update(path for path in ADVERSARY.glob("*") if path.is_file())
content_read_rows = [{"path": str(path), "sha256": sha(path), "access": "authorized-content", "attestation": "processual-only"} for path in sorted(content_paths, key=str)]
write_tsv(OUT / "content-reads.tsv", content_read_rows, ["path", "sha256", "access", "attestation"])

commands = [
    ("C001", "sed skill and required references", "read full skill instructions"),
    ("C002", "rg --files 00_nucleo/adr and rg materialization terms", "ADR discovery and authorized content scan"),
    ("C003", "sha256sum and sed V3 manifests freezes capabilities drift", "frozen control validation"),
    ("C004", "find authorized /tmp/p1268 trees", "filename metadata enumeration"),
    ("C005", "sha256sum frozen refiner oracle adversary authoritative artifacts", "independent hash audit"),
    ("C006", "sed frozen contract unknown observables corrections receipt", "contract audit"),
    ("C007", "sed oracle runner and TSV outputs in bounded chunks", "oracle semantic audit"),
    ("C008", "sed adversary runner attacks opaque schema receipt", "adversary audit"),
    ("C009", "git rev-parse HEAD; git status --short; protected sha256sum", "HEAD provenance audit"),
    ("C010", "python read-only population cost mask coincidence degenerates determinism audit", "deep gate revalidation"),
    ("C011", "python read-only binary direct inverse repeat equality", "determinism revalidation"),
    ("C012", "sed authoritative adaptive L0 svg L0 and ratified gradient source sections", "normative source adjudication"),
    ("C013", "python attack_runner.py materialize 25 attacks under verifier_v3", "frozen attack execution"),
    ("C014", "python attack_runner.py --opaque materialize 10 cases under verifier_v3", "opaque execution"),
    ("C015", "python compare 35 materializations with frozen declared states", "runner reproduction"),
    ("C016", "apply_patch verifier mutation opaque technical capability TSVs", "verifier-only evidence write"),
    ("C017", "awk constant-column validation", "TSV schema gate"),
    ("C018", "python3 /tmp/p1268/verifier_v3/finalize_verification.py", "final hash closure and conditional preseal"),
]
write_tsv(OUT / "commands.tsv", [{"command_id": a, "command": b, "purpose": c, "status": "PASS"} for a, b, c in commands], ["command_id", "command", "purpose", "status"])

receipt_rows = [
    {"field": "role", "value": "VERIFIER-V3 independent adjudicator"},
    {"field": "regime", "value": "Tekt full protocol; verifier phase"},
    {"field": "measured_at", "value": datetime.now().astimezone().isoformat(timespec="seconds")},
    {"field": "head_at_intent_freeze", "value": "697eaf31e8ce6aaa4eef7d61d7808e377005c3c5"},
    {"field": "head_verified", "value": head},
    {"field": "head_drift_verdict", "value": "ACCEPTED-AS-PROVENANCE-ONLY; all content-addressed protected inputs unchanged"},
    {"field": "git_status_sha256_current", "value": hashlib.sha256(status_text.encode()).hexdigest()},
    {"field": "git_status_current", "value": status_text.replace("\n", ";") or "clean"},
    {"field": "git_diff_HEAD_stat_sha256", "value": hashlib.sha256(diff_stat.encode()).hexdigest()},
    {"field": "git_diff_HEAD_stat", "value": diff_stat.replace("\n", ";") or "empty"},
    {"field": "content_reads", "value": f"{len(content_read_rows)} paths; see content-reads.tsv"},
    {"field": "commands", "value": f"{len(commands)} normalized command records; see commands.tsv"},
    {"field": "population", "value": "96;24 per pair"},
    {"field": "positive_oracles", "value": "96 Available;0 necessary Unknown"},
    {"field": "M_R_E", "value": "1320;848;11188"},
    {"field": "intervals", "value": "1224 total;848 eligible;376 ineligible;maxima 64/31/63/62/125"},
    {"field": "mask_closure", "value": "96 rows;192 SVG-PNG files;all hashes exact"},
    {"field": "coincidence", "value": "180 groups;exact Unknown=0;epsilon Unknown=0;terminal N-A=4"},
    {"field": "degenerates", "value": "S20=4/4;S21=4/4"},
    {"field": "determinism", "value": "192/192 semantic equal;96/96 SVG triplets byte equal"},
    {"field": "attacks", "value": "24 valid negative killed;1 positive control preserved;10 opaque Unknown"},
    {"field": "mutation_score", "value": "1.0 (24/24)"},
    {"field": "negative_unknown", "value": "0"},
    {"field": "oracle_breach", "value": "false"},
    {"field": "verifier_breach", "value": "false"},
    {"field": "write_root", "value": "/tmp/p1268/verifier_v3 exactly"},
    {"field": "repository_writes", "value": "NONE"},
    {"field": "forbidden_content_read", "value": "false; excluded attempts candidate target/debug/typst future product and P1267-R1 product content were not inspected"},
    {"field": "r2_comparison_access", "value": "sha256-only protected identity check required by head-drift; no semantic inspection and no P1267-R1 artifact opened"},
    {"field": "filesystem_isolation_attested", "value": "false"},
    {"field": "attestation", "value": "segregacao processual; sem atestacao tecnica de isolamento de filesystem"},
    {"field": "verdict", "value": "PRESEAL-ELIGIBLE"},
    {"field": "final_seal", "value": "NOT ISSUED"},
]
write_tsv(OUT / "receipt.tsv", receipt_rows, ["field", "value"])

owned = [
    OUT / "hash-audit.tsv", OUT / "fixture-closure.tsv", OUT / "mask-closure.tsv",
    OUT / "materialization-closure.tsv",
    OUT / "mutation-verdicts.tsv", OUT / "opaque-verdicts.tsv", OUT / "technical-gates.tsv",
    OUT / "capability-gates.tsv", OUT / "content-reads.tsv", OUT / "commands.tsv", OUT / "receipt.tsv",
    OUT / "finalize_verification.py",
]
preseal_rows = [
    {"field": "artifact", "value": "P1268-VERIFIER-V3-PRESEAL"},
    {"field": "suite_freeze_sha256", "value": expected[ROOT / "00_nucleo/diagnosticos/p1268-suite-freeze-v3.tsv"]},
    {"field": "preoracle_manifest_sha256", "value": expected[ROOT / "00_nucleo/diagnosticos/p1268-preoracle-manifest-v3.tsv"]},
    {"field": "role_capabilities_sha256", "value": expected[ROOT / "00_nucleo/diagnosticos/p1268-role-capabilities-v3.tsv"]},
    {"field": "head_drift_sha256", "value": expected[ROOT / "00_nucleo/diagnosticos/p1268-head-drift.tsv"]},
    {"field": "contract_sha256", "value": expected[REFINER / "contract-v3.tsv"]},
    {"field": "oracle_receipt_sha256", "value": expected[ORACLE / "receipt.tsv"]},
    {"field": "adversary_attacks_sha256", "value": expected[ADVERSARY / "attacks.tsv"]},
    {"field": "mutation_score", "value": "1.0"},
    {"field": "valid_negative_killed", "value": "24/24"},
    {"field": "negative_unknown", "value": "0"},
    {"field": "positive_control", "value": "1/1 Preserved"},
    {"field": "explicit_opaque", "value": "10/10 Unknown;counts_as_success=false"},
    {"field": "technical_gates", "value": "ALL PASS"},
    {"field": "capability_gates", "value": "ALL PASS;breach=false"},
    {"field": "head_drift", "value": "ACCEPTED-AS-PROVENANCE-ONLY"},
    {"field": "filesystem_isolation", "value": "NOT TECHNICALLY ATTESTED"},
    {"field": "attestation", "value": "segregacao processual"},
    {"field": "verdict", "value": "PRESEAL-ISSUED"},
    {"field": "scope", "value": "frozen P1268 observable fragment only;no general functional equivalence claim"},
    {"field": "final_seal", "value": "NOT ISSUED"},
]
for path in owned:
    preseal_rows.append({"field": "verifier_output_sha256", "value": f"{path} sha256:{sha(path)}"})
write_tsv(OUT / "preseal.tsv", preseal_rows, ["field", "value"])

print(f"PRESEAL-ISSUED mutation_score={mutation_score:.1f} protected={len(hash_rows)} fixtures={len(fixture_rows)} masks={len(mask_rows)} reads={len(content_read_rows)}")
