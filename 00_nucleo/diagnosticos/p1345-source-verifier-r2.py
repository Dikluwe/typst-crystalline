#!/usr/bin/env python3
"""P1345 R2 source verifier: pinned R1 semantics plus root-identity closure.

R1 remains the canonical implementation of token, marker, owner, anchor and
inverse-normalization verification.  This revision changes only the executable
trust boundary: an authority root must be a real directory identity, never a
symlink (including a symlink in an ancestor component).
"""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R1_PATH = DIAG / "p1345-source-verifier-r1.py"
EXPECTED_R1_SHA256 = "a787f52fbcc2590255d50d982c8f0ccbca84f0c750062ede05aa6b171744d8b7"


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


if _sha256(R1_PATH.read_bytes()) != EXPECTED_R1_SHA256:
    raise RuntimeError("AUTHORITY_ROOT: P1345 source verifier R1 hash drift")
_spec = importlib.util.spec_from_file_location("p1345_source_verifier_r1_pinned_by_r2", R1_PATH)
if _spec is None or _spec.loader is None:
    raise RuntimeError("AUTHORITY_ROOT: cannot load P1345 source verifier R1")
_r1 = importlib.util.module_from_spec(_spec)
sys.modules[_spec.name] = _r1
_spec.loader.exec_module(_r1)

# Re-export the immutable R1 lexical/canonical implementation.  The explicit
# override below is the sole semantic delta in this file.
for _name in dir(_r1):
    if not _name.startswith("__") and _name not in globals():
        globals()[_name] = getattr(_r1, _name)


def verify_root(root: Path, case_id: str = "candidate") -> dict[str, Any]:
    root = Path(root)
    absolute = root.absolute()
    try:
        resolved = root.resolve(strict=True)
    except (FileNotFoundError, OSError) as exc:
        raise VerificationFailure("PATH", f"authority root is not a real directory: {exc}") from exc
    if root.is_symlink() or absolute != resolved or not root.is_dir():
        raise VerificationFailure("PATH", "authority root identity traverses a symlink or is not a directory")
    return _r1.verify_root(root, case_id)


if __name__ == "__main__":
    import argparse
    import json

    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify_root(args.root), ensure_ascii=False, indent=2))
