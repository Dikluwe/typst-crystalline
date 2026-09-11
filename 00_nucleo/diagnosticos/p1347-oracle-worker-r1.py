#!/usr/bin/env python3
"""Answer-free P1347 worker. It emits bytes and receipts, never judgments."""

from __future__ import annotations

import hashlib
import json
import os
import signal
import sys
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
BINDING = DIAG / "p1347-contract-binding-r1.json"
STEP_SHA256 = "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c"
SPEC_SHA256 = "fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea"
BINDING_SHA256 = "de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1"
P1346_REPORT_SHA256 = "60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
PYTHON = Path("/usr/bin/python3")
PYTHON_SHA256 = "e50d468e8b0adfb05733f5b87b3cff34829c4a8c1aea50c865aa8bdfe4bb150f"
BASE_FILES = {
    "p1346-oracle-authorship-receipt-r2.json": "7343643dba523fa7716e4fc89dac7ec20492c5c41718333d9f46f67ce260d4ca",
    "p1346-oracle-caller-r2.py": "ba4fd42247eadd04ee7a8af8519043a77113205340cb60edbe79128e29ceb120",
    "p1346-oracle-checker-r2.py": "c6c8e425b9387afd2d02d20d148d407c4f2b252cef3428ccb1eb1eec91894ec1",
    "p1346-oracle-delivery-receipt-r2.json": "2b4ec685285153b185a3de6a92aec074de39731f34e0f52b35489ddfb4c0f5e0",
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(value: Any, trailing_lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if trailing_lf else b"")


def reject_duplicate(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate key")
        result[key] = value
    return result


def strict_stdin() -> dict[str, Any]:
    raw = sys.stdin.buffer.read(2_000_001)
    if len(raw) > 2_000_000 or raw.count(b"\n") != 1 or not raw.endswith(b"\n"):
        raise ValueError("request framing")
    value = json.loads(raw, object_pairs_hook=reject_duplicate, parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))
    if type(value) is not dict or canonical(value) != raw:
        raise ValueError("request canonical form")
    return value


def exact_hex(value: Any) -> str:
    if type(value) is not str or len(value) != 64 or any(char not in "0123456789abcdef" for char in value):
        raise ValueError("hex")
    bytes.fromhex(value)
    return value


def protected_ids() -> list[str]:
    raw = BINDING.read_bytes()
    if sha256(raw) != BINDING_SHA256:
        raise ValueError("binding drift")
    value = json.loads(raw, object_pairs_hook=reject_duplicate, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    ids = value["canonical_case_sequence"]["exact_ids"]
    if type(ids) is not list or len(ids) != 122 or any(type(item) is not str or not item.isascii() for item in ids):
        raise ValueError("case ids")
    if sha256(canonical(ids, trailing_lf=False)) != SEQUENCE_SHA256:
        raise ValueError("case sequence")
    return ids


def work_root() -> Path:
    supplied = os.environ.get("P1347_WORK_ROOT", "")
    if not supplied.startswith("/dev/shm/p1347-"):
        raise ValueError("work root")
    path = Path(supplied)
    observed = path.lstat()
    if path.is_symlink() or not path.is_dir() or observed.st_uid != os.getuid():
        raise ValueError("work root authority")
    return path


def invocation_digest(worker_sha256: str, challenge_hex: str, nonce_hex: str) -> str:
    values = [
        "p1347-worker-invocation-r1", STEP_SHA256, SPEC_SHA256, BINDING_SHA256,
        P1346_REPORT_SHA256, worker_sha256, PYTHON_SHA256, SEQUENCE_SHA256,
        challenge_hex, nonce_hex, "focal_transport",
    ]
    return sha256(canonical(values, trailing_lf=False))


def transport() -> int:
    request = strict_stdin()
    keys = ["case_sequence_sha256", "challenge_hex", "invocation_nonce_hex", "invocation_sha256", "mode", "schema"]
    if list(sorted(request)) != keys or request["schema"] != "p1347-worker-request-r1" or request["mode"] != "focal_transport":
        raise ValueError("transport schema")
    challenge = exact_hex(request["challenge_hex"])
    nonce = exact_hex(request["invocation_nonce_hex"])
    if challenge == nonce or request["case_sequence_sha256"] != SEQUENCE_SHA256:
        raise ValueError("transport identity")
    worker_sha = sha256(Path(__file__).read_bytes())
    invocation = invocation_digest(worker_sha, challenge, nonce)
    if exact_hex(request["invocation_sha256"]) != invocation or sha256(PYTHON.read_bytes()) != PYTHON_SHA256:
        raise ValueError("invocation authority")
    root = work_root()
    receipts = []
    challenge_sha = sha256(bytes.fromhex(challenge))
    for ordinal, case_id in enumerate(protected_ids()):
        recipe = sha256(canonical(["p1347-answer-free-recipe-r1", ordinal, case_id], trailing_lf=False))
        relative = f"case-{ordinal:03d}/mutation.bin"
        target = root / relative
        target.parent.mkdir(mode=0o700)
        payload = canonical({"case_id": case_id, "ordinal": ordinal, "recipe_sha256": recipe})
        target.write_bytes(payload)
        changed = [{"path": relative, "sha256": sha256(payload)}]
        receipts.append({
            "baseline_sha256": P1346_REPORT_SHA256,
            "case_id": case_id,
            "challenge_sha256": challenge_sha,
            "changed_paths": changed,
            "invocation_sha256": invocation,
            "mutated_tree_sha256": sha256(canonical(changed, trailing_lf=False)),
            "recipe_sha256": recipe,
        })
    operation = os.environ.get("P1347_ORACLE_MUTATION", "canonical")
    if operation == "omit_id": receipts.pop(10)
    elif operation == "extra_id": receipts.append(dict(receipts[-1], case_id="P1347-EXTRA"))
    elif operation == "duplicate_id": receipts[1] = dict(receipts[1], case_id=receipts[0]["case_id"])
    elif operation == "reorder_id": receipts[0], receipts[1] = receipts[1], receipts[0]
    elif operation == "unicode_id": receipts[0] = dict(receipts[0], case_id=receipts[0]["case_id"].replace("-", "‑", 1))
    elif operation == "timeout": time.sleep(10)
    elif operation == "signal": os.kill(os.getpid(), signal.SIGTERM)
    elif operation == "unexpected_exit": return 9
    digest = sha256(canonical(receipts, trailing_lf=False))
    response: dict[str, Any] = {
        "case_sequence_sha256": SEQUENCE_SHA256,
        "challenge_response_sha256": sha256(canonical(["p1347-worker-challenge-r1", challenge, nonce, invocation, SEQUENCE_SHA256, digest], trailing_lf=False)),
        "closed_world": {"rule": "Answer-free mutation receipts only."},
        "invocation_sha256": invocation,
        "mutation_receipts": receipts,
        "schema": "p1347-worker-response-r1",
    }
    pid_values: dict[str, Any] = {"pid_bool": True, "pid_zero": 0, "pid_negative": -1, "pid_string": str(os.getpid()), "pid_sibling": os.getppid()}
    if operation in pid_values:
        response["pid"] = pid_values[operation]
    if operation == "stderr":
        sys.stderr.write("foreign stderr\n")
    raw = canonical(response)
    if operation == "extra_line": raw += b"{}\n"
    sys.stdout.buffer.write(raw)
    return 0


def dag_tree_digest(root: Path) -> tuple[str, list[dict[str, str]]]:
    rows = []
    for path in sorted(item for item in root.iterdir() if item.is_file() and not item.is_symlink()):
        rows.append({"path": path.name, "sha256": sha256(path.read_bytes())})
    return sha256(canonical(rows, trailing_lf=False)), rows


def dag() -> int:
    request = strict_stdin()
    keys = ["case_id", "challenge_hex", "invocation_nonce_hex", "invocation_sha256", "operation", "schema"]
    if list(sorted(request)) != keys or request["schema"] != "p1347-dag-worker-request-r1":
        raise ValueError("dag schema")
    challenge = exact_hex(request["challenge_hex"])
    invocation = exact_hex(request["invocation_sha256"])
    exact_hex(request["invocation_nonce_hex"])
    if type(request["case_id"]) is not str or type(request["operation"]) is not str:
        raise ValueError("dag types")
    root = work_root()
    for name, digest in BASE_FILES.items():
        source = DIAG / name
        raw = source.read_bytes()
        if sha256(raw) != digest:
            raise ValueError("baseline drift")
        (root / name).write_bytes(raw)
    baseline, before = dag_tree_digest(root)
    operation = request["operation"]
    changed: list[dict[str, str]] = []
    if operation in {"t08", "t09"}:
        path = root / "p1346-oracle-authorship-receipt-r2.json"
        raw = path.read_bytes()
        if operation == "t08":
            raw = raw[:-2] + b',"caller_sha256":"' + b"0" * 64 + b'"}\n'
        else:
            raw = raw.replace(b"FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED", b"FAILED_NOT_SEALED")
        path.write_bytes(raw); changed.append({"path": path.name, "sha256": sha256(raw)})
    elif operation == "t10":
        path = root / "p1346-oracle-caller-r2.py"; raw = path.read_bytes() + b"\n# substitution\n"; path.write_bytes(raw); changed.append({"path": path.name, "sha256": sha256(raw)})
    elif operation == "t11":
        path = root / "p1346-oracle-delivery-receipt-r2.json"; raw = path.read_bytes(); raw = raw.replace(b"7343643dba523fa7716e4fc89dac7ec20492c5c41718333d9f46f67ce260d4ca", b"0" * 64, 1); path.write_bytes(raw); changed.append({"path": path.name, "sha256": sha256(raw)})
    elif operation in {"extra_file", "mutant_anchor"}:
        name = "extra.bin" if operation == "extra_file" else "anchor.txt"; raw = b"mutated\n"; (root / name).write_bytes(raw); changed.append({"path": name, "sha256": sha256(raw)})
    elif operation not in {"canonical", "validator_unobserved", "dag_self_labels", "receipt_answer_channel"}:
        raise ValueError("dag operation")
    mutated, after = dag_tree_digest(root)
    response: dict[str, Any] = {
        "baseline_sha256": baseline,
        "case_id": request["case_id"],
        "challenge_sha256": sha256(bytes.fromhex(challenge)),
        "changed_paths": changed,
        "invocation_sha256": invocation,
        "mutated_tree_sha256": mutated,
        "recipe_sha256": sha256(canonical(["p1347-dag-recipe-r1", request["case_id"], operation, before, after], trailing_lf=False)),
        "schema": "p1347-dag-mutation-receipt-r1",
    }
    if operation in {"dag_self_labels", "receipt_answer_channel"}:
        response["classification"] = "Violated"
    sys.stdout.buffer.write(canonical(response))
    return 0


def main() -> int:
    if sys.argv[1:] == ["--transport"]:
        return transport()
    if sys.argv[1:] == ["--dag"]:
        return dag()
    raise ValueError("closed CLI")


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        sys.stderr.write(f"WORKER_AUTHORITY: {type(exc).__name__}\n")
        raise SystemExit(2)
