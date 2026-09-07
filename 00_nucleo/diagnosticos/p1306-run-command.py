"""Record exact P1306 command receipts; never evaluates oracle expectations."""

import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def snapshot():
    diff = git("diff", "HEAD", "--binary")
    return {
        "head": git("rev-parse", "HEAD").decode().strip(),
        "status": git("status", "--short").decode(),
        "diff_stat": git("diff", "HEAD", "--stat").decode(),
        "diff_sha256": digest(diff),
        "diff": diff.decode(),
    }


def tool_identity(name):
    argv = [name, "--version"]
    proc = subprocess.run(argv, cwd=ROOT, capture_output=True)
    return {
        "path": shutil.which(name), "argv": argv, "exit_code": proc.returncode,
        "stdout": proc.stdout.decode(), "stderr": proc.stderr.decode(),
        "executable_sha256": digest(Path(shutil.which(name)).resolve().read_bytes()),
    }


def without_metadata(data, marker):
    lines = data.splitlines(keepends=True)
    matching = [i for i, line in enumerate(lines) if line.startswith(marker)]
    if len(matching) != 1:
        raise RuntimeError("Expected exactly one metadata line")
    return b"".join(line for i, line in enumerate(lines) if i != matching[0])


def write_manifest():
    preflight = DIAG / "p1306-preflight.json"
    config = json.loads(preflight.read_text())
    paths = [config["step"], "00_nucleo/diagnosticos/p1306-planning-measurement.json", "00_nucleo/diagnosticos/p1305-commit-notes.md"]
    paths.extend("00_nucleo/diagnosticos/" + name for name in [
        "p1306-preflight.json", "p1306-contract.json", "p1306-contract-measure.py",
        "p1306-contract-measurement.json", "p1306-oracle.py", "p1306-oracle-cases.json",
        "p1306-baseline-measurement.json", "p1306-attacks.json", "p1306-oracle-receipt.json",
        "p1306-run-command.py",
    ])
    source_paths = git("ls-files", "01_core", "02_shell", "03_infra", "04_wiring", "00_nucleo/prompts", "Cargo.toml", "Cargo.lock", ".cargo", "crystalline.toml").decode().splitlines()
    source_hashes = {path: digest((ROOT / path).read_bytes()) for path in source_paths if (ROOT / path).is_file()}
    manifest = {
        "protocol": config["protocol"], "regime": config["regime"],
        "created_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "preflight_sha256": digest(preflight.read_bytes()), "preflight": config,
        "source_state_before_candidate": snapshot(),
        "protected_inputs": {path: digest((ROOT / path).read_bytes()) for path in paths},
        "source_hashes_before_candidate": source_hashes,
        "toolchain": {name: tool_identity(name) for name in ("rustc", "cargo", "crystalline-lint", "python3")},
        "candidate_mutable_paths": ["01_core/src/compiler/eval/bindings/field_access.rs"],
        "metadata_reopening_policy": {
            "reason": "Preseal review identified linter V5 planning only prompt-to-header drift; source-only changes may leave stale Hash do Codigo undetected. Do not infer reciprocal freshness from dryrun alone.",
            "amends": "Preflight forbids silent postseal protected changes and requires reopening. This declares that explicit metadata-only reopening in advance; original preflight remains immutable.",
            "outputs_exact_additional": ["00_nucleo/diagnosticos/p1306-lineage-reopening.json", "00_nucleo/diagnosticos/p1306-successor-seal.json"],
            "authority": {"root": "Write lineage-reopening receipt before changing only Hash do Código metadata in field_access L0, derived from actual candidate bytes; never semantic text.", "verifier": "Independently prove exact metadata-only difference, source hash and unchanged semantic contract/oracles, rerun discriminator and affected lineage checks; write successor-seal, never repair input."},
            "sequence": "Initial raw seal -> RED -> productive patch -> GREEN -> explicit reopening -> derived metadata only -> successor seal -> final build/gates/matrix/mutants. Any semantic change blocks this narrow reopening.",
            "source_hash_algorithm": "SHA256 of source UTF-8 bytes excluding the complete //! @prompt-hash line including LF, truncated to first8 lowercasehex; confirmed in tekt-linter/03_infra/hash_writer.rs:15-19.",
        },
        "l0_before_candidate": {
            path: {"raw": (ROOT / path).read_text(), "raw_sha256": digest((ROOT / path).read_bytes()), "semantic_sha256": digest(without_metadata((ROOT / path).read_bytes(), "Hash do Código: ".encode()))}
            for path in ("00_nucleo/prompts/compiler/eval/bindings/field_access.md", "00_nucleo/prompts/compiler/eval/tests.md")
        },
        "final_artifact_policy": "Verifier seal precedes productive patch. Certificate pins completed receipts; report pins certificate; no circular hashes.",
    }
    dest = DIAG / "p1306-manifest.json"
    if dest.exists():
        raise RuntimeError("Manifest already exists; reopening must be explicit")
    dest.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"path": str(dest), "sha256": digest(dest.read_bytes()), "source_files": len(source_hashes)}))


def write_lineage_reopening():
    manifest = json.loads((DIAG / "p1306-manifest.json").read_text())
    source = ROOT / "01_core/src/compiler/eval/bindings/field_access.rs"
    prompt = ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md"
    original = prompt.read_bytes()
    raw_before = manifest["l0_before_candidate"][str(prompt.relative_to(ROOT))]["raw_sha256"]
    if digest(original) != raw_before:
        raise RuntimeError("L0 drift before explicit reopening")
    code_hash = digest(without_metadata(source.read_bytes(), b"//! @prompt-hash "))[:8]
    replacement = ("Hash do Código: " + code_hash + "\n").encode()
    expected = b"".join(replacement if line.startswith("Hash do Código: ".encode()) else line for line in original.splitlines(keepends=True))
    payload = {
        "created_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "manifest_sha256": digest((DIAG / "p1306-manifest.json").read_bytes()),
        "initial_seal_sha256": digest((DIAG / "p1306-seal.json").read_bytes()),
        "source_state": snapshot(), "source_path": str(source.relative_to(ROOT)),
        "candidate_source_sha256": digest(source.read_bytes()), "derived_code_hash": code_hash,
        "prompt_path": str(prompt.relative_to(ROOT)), "before_raw_sha256": digest(original),
        "expected_after_raw_sha256": digest(expected),
        "semantic_sha256": digest(without_metadata(original, "Hash do Código: ".encode())),
        "authorized_change": "Replace only canonical Hash do Código metadata line; no candidate semantic or oracle change. Independent successor seal required before final gates.",
        "algorithm": manifest["metadata_reopening_policy"]["source_hash_algorithm"],
    }
    dest = DIAG / "p1306-lineage-reopening.json"
    if dest.exists():
        raise RuntimeError("Reopening receipt already exists")
    dest.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(payload, ensure_ascii=False))


def record_command(dest, argv):
    before = snapshot()
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    tick = time.monotonic_ns()
    proc = subprocess.run(argv, cwd=ROOT, capture_output=True)
    after = snapshot()
    record = {
        "argv": argv, "cwd": str(ROOT), "started_at": start,
        "finished_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "duration_ns": time.monotonic_ns() - tick, "exit_code": proc.returncode,
        "preflight_sha256": digest((DIAG / "p1306-preflight.json").read_bytes()),
        "manifest_sha256": digest((DIAG / "p1306-manifest.json").read_bytes()) if (DIAG / "p1306-manifest.json").exists() else None,
        "source_before": before, "source_after": after,
        "selected_env": {key: os.environ[key] for key in ("CARGO_TARGET_DIR", "RUST_MIN_STACK", "RUSTFLAGS") if key in os.environ},
        "stdout": proc.stdout.decode(errors="replace"), "stderr": proc.stderr.decode(errors="replace"),
        "stdout_base64": base64.b64encode(proc.stdout).decode(), "stderr_base64": base64.b64encode(proc.stderr).decode(),
        "stdout_sha256": digest(proc.stdout), "stderr_sha256": digest(proc.stderr),
    }
    existing = json.loads(dest.read_text()) if dest.exists() else []
    existing.append(record)
    dest.write_text(json.dumps(existing, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({key: record[key] for key in ("argv", "started_at", "finished_at", "duration_ns", "exit_code")}, ensure_ascii=False))
    print(record["stdout"][-3500:])
    print(record["stderr"][-1500:])
    return proc.returncode


if __name__ == "__main__":
    if sys.argv[1:] == ["--manifest"]:
        write_manifest()
    elif sys.argv[1:] == ["--lineage-reopening"]:
        write_lineage_reopening()
    else:
        raise SystemExit(record_command(Path(sys.argv[1]), sys.argv[2:]))
