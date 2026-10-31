#!/usr/bin/env python3
"""Sondas do catálogo P1495, sem alterar fontes nem artefatos.

O oráculo é o binário vanilla 64c5e8c0 ratificado no re-sync 2026-10-26. Um exit 1 indica falha de paridade ou do contrato das extensões;
exit 2 denuncia defeito do oráculo/harness. Extensões legadas são reportadas,
mas não contam como violação contra o novo vanilla.
"""

import argparse
import hashlib
import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[3]
VANILLA_SHA256 = "22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34"

CHILDREN = {
    "chess": ("king", "queen", "rook", "bishop", "knight", "pawn"),
    "keyboard": (
        "tab", "pageup", "pagedown", "shift", "capslock", "space", "erase",
        "cmd", "option", "clear", "alt", "esc", "enter",
    ),
    "ordinal": ("feminine", "masculine"),
}
VARIANTS = tuple(
    f"chess.{piece}.{color}"
    for piece in CHILDREN["chess"]
    for color in ("white", "black")
) + (
    "keyboard.tab.r", "keyboard.pageup.alt", "keyboard.pagedown.alt",
    "keyboard.capslock.pedestal", "keyboard.erase.l", "keyboard.erase.r",
    "keyboard.enter.alt",
)
FAMILY_WITNESSES = {
    "eq": "eq.gt.slant",
    "equiv": "equiv.lt",
    "gt": "gt.eq.lt.slant",
    "lat": "lat.eq.slant",
    "lt": "lt.eq.gt.slant",
    "not": "not.inv",
    "prec": "prec.eq.slant",
    "smt": "smt.eq.slant",
    "succ": "succ.eq.slant",
    "tack": "tack.r.double",
    "tilde": "tilde.lt",
    "trademark": "trademark.mc",
}
TACK_VALUES = (
    "tack.r.double", "tack.r.double.not", "tack.l.double",
    "tack.t.double", "tack.b.double",
)
LEGACY = ("join", "join.r", "join.l", "join.l.r") + tuple(
    f"{side}.tri{suffix}"
    for side in ("gt", "lt")
    for suffix in ("", ".eq", ".eq.not", ".not")
)

JOIN_WARNING = "`join` is deprecated, use `bowtie.big` instead"
LEGACY_VALUES = {
    "join": "⨝",
    "join.r": "⟖",
    "join.l": "⟕",
    "join.l.r": "⟗",
    "gt.tri": "⊳",
    "gt.tri.eq": "⊵",
    "gt.tri.eq.not": "⋭",
    "gt.tri.not": "⋫",
    "lt.tri": "⊲",
    "lt.tri.eq": "⊴",
    "lt.tri.eq.not": "⋬",
    "lt.tri.not": "⋪",
}
LEGACY_REPRS = {
    "join": "symbol(\"⨝\", (\"r\", \"⟖\"), (\"l\", \"⟕\"), (\"l.r\", \"⟗\"))",
    "join.r": "symbol(\"⟖\", (\"l\", \"⟗\"))",
    "join.l": "symbol(\"⟕\", (\"r\", \"⟗\"))",
    "join.l.r": "symbol(\"⟗\")",
    "gt.tri": "symbol(\n  \"⊳\",\n  (\"eq\", \"⊵\"),\n  (\"eq.not\", \"⋭\"),\n  (\"not\", \"⋫\"),\n)",
    "gt.tri.eq": "symbol(\"⊵\", (\"not\", \"⋭\"))",
    "gt.tri.eq.not": "symbol(\"⋭\")",
    "gt.tri.not": "symbol((\"eq\", \"⋭\"), \"⋫\")",
    "lt.tri": "symbol(\n  \"⊲\",\n  (\"eq\", \"⊴\"),\n  (\"eq.not\", \"⋬\"),\n  (\"not\", \"⋪\"),\n)",
    "lt.tri.eq": "symbol(\"⊴\", (\"not\", \"⋬\"))",
    "lt.tri.eq.not": "symbol(\"⋬\")",
    "lt.tri.not": "symbol((\"eq\", \"⋬\"), \"⋪\")",
}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def cases():
    for namespace in ("sym", "math"):
        for parent, children in CHILDREN.items():
            yield ("parity", f"{namespace}.{parent}", f"repr({namespace}.{parent})")
            for child in children:
                path = f"{parent}.{child}"
                yield ("parity", f"{namespace}.{path}", f"repr({namespace}.{path})")
        for path in VARIANTS:
            yield ("parity", f"{namespace}.{path}", f"repr({namespace}.{path})")
        for group, witness in FAMILY_WITNESSES.items():
            yield ("parity", f"{namespace}.{group}", f"repr({namespace}.{group})")
            yield ("parity", f"{namespace}.{witness}", f"repr({namespace}.{witness})")
        for path in TACK_VALUES:
            yield ("parity", f"{namespace}.{path}.value", f"str({namespace}.{path})")
        yield ("warning", f"{namespace}.spacebar", f"repr({namespace}.spacebar)")
        for path in LEGACY:
            yield ("extension", f"{namespace}.{path}", f"repr({namespace}.{path})")

    for path in (
        "chess.king.white", "keyboard.tab.r", "ordinal.feminine",
        *FAMILY_WITNESSES.values(),
    ):
        yield ("bare", f"math-bare.{path}", f'eval("{path}", mode: "math").body')
    yield ("bare-warning", "math-bare.spacebar", 'eval("spacebar", mode: "math").body')
    for path in LEGACY:
        yield ("bare-extension", f"math-bare.{path}", f'eval("{path}", mode: "math").body')


def run(binary, expression):
    try:
        result = subprocess.run(
            [str(binary), "eval", expression, "--format", "json"],
            capture_output=True, text=True, timeout=15, check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"exit": None, "error": str(error)}
    try:
        value = json.loads(result.stdout) if result.returncode == 0 else None
    except json.JSONDecodeError:
        return {"exit": result.returncode, "error": "invalid JSON", "stdout": result.stdout}
    return {
        "exit": result.returncode,
        "value": value,
        "warnings": [
            line.removeprefix("warning: ")
            for line in result.stderr.splitlines()
            if line.startswith("warning: ")
        ],
        "error": next(
            (line.removeprefix("error: ") for line in result.stderr.splitlines()
             if line.startswith("error: ")),
            None,
        ),
    }


def verdict(kind, name, oracle, crystal):
    if oracle.get("exit") is None:
        return "BASELINE_DEFECT"
    if kind in ("extension", "bare-extension"):
        bare = kind == "bare-extension"
        path = name.removeprefix("math-bare.") if bare else name.split(".", 1)[1]
        if path.startswith("join"):
            expected_error = (
                "unknown variable `join`" if bare
                else f"module `{name.split('.', 1)[0]}` does not contain `join`"
            )
        else:
            expected_error = "unknown symbol modifier"
        if (oracle["exit"] == 0 or oracle.get("error") != expected_error
                or oracle.get("warnings") != []):
            return "BASELINE_DEFECT"
        if crystal.get("exit") != 0 or crystal.get("error") is not None:
            return "EXTENSION_MISSING"
        # Approved batch E insertion preserves native Symbol identity, including legacy extensions.
        if bare and (not isinstance(crystal["value"], dict) or crystal["value"].get("func") != "symbol"):
            return "EXTENSION_DRIFT"
        observed = (
            crystal["value"].get("text")
            if bare and isinstance(crystal["value"], dict)
            else crystal["value"]
        )
        expected = LEGACY_VALUES[path] if bare else LEGACY_REPRS[path]
        if observed != expected:
            return "EXTENSION_DRIFT"
        # A extensão conserva a política histórica: sym e bare avisam;
        # math.join permanece silencioso, sem criar novo default P1495.
        warnings = [JOIN_WARNING] if path.startswith("join") and (bare or name.startswith("sym.")) else []
        return "EXTENSION" if crystal["warnings"] == warnings else "EXTENSION_WARNING_RED"
    if oracle["exit"] != 0 or oracle.get("error") is not None:
        return "BASELINE_DEFECT"
    if crystal.get("exit") != 0 or crystal.get("error") is not None:
        return "RED"
    if kind.startswith("bare"):
        # Compara o glifo, mas não declara MATCH integral se o carrier
        # público ainda divergir (`symbol` vanilla vs `text` cristalino).
        left_body, right_body = oracle["value"], crystal["value"]
        if (not isinstance(left_body, dict) or "text" not in left_body
                or not isinstance(right_body, dict) or "text" not in right_body):
            return "RED"
        left, right = left_body["text"], right_body["text"]
    else:
        left, right = oracle["value"], crystal["value"]
    if left != right:
        return "RED"
    if oracle["warnings"] != crystal["warnings"]:
        return "RED"
    if kind.startswith("bare") and left_body != right_body:
        return "TEXT_MATCH_CARRIER_DIFF"
    return "MATCH"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vanilla", type=pathlib.Path,
                        default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline", type=pathlib.Path,
                        default=ROOT / "target/release/typst")
    parser.add_argument("--details", action="store_true", help="mostrar cada caso")
    args = parser.parse_args()
    vanilla, crystal = args.vanilla.resolve(), args.crystalline.resolve()
    if sha256(vanilla) != VANILLA_SHA256:
        parser.error("binário vanilla não é o build 64c5e8c0 ratificado em 2026-10-26")
    crystal_hash = sha256(crystal)

    results = []
    counts = {}
    for kind, name, expression in cases():
        oracle = run(vanilla, expression)
        current = run(crystal, expression)
        status = verdict(kind, name, oracle, current)
        counts[status] = counts.get(status, 0) + 1
        results.append({
            "name": name, "kind": kind, "verdict": status,
            "expression": expression, "oracle": oracle, "crystalline": current,
        })
    if sha256(vanilla) != VANILLA_SHA256 or sha256(crystal) != crystal_hash:
        print(json.dumps({
            "error": "binary changed during probe",
            "vanilla_sha256": VANILLA_SHA256,
            "crystalline_sha256": crystal_hash,
        }, sort_keys=True))
        return 2
    output = {
        "vanilla_sha256": VANILLA_SHA256,
        "crystalline_sha256": crystal_hash,
        "total": len(results), "counts": counts,
        "first_red": [r["name"] for r in results if r["verdict"] in ("RED", "EXTENSION_WARNING_RED", "EXTENSION_DRIFT", "EXTENSION_MISSING")][:20],
    }
    if args.details:
        output["results"] = results
    print(json.dumps(output, ensure_ascii=False, sort_keys=True))
    if counts.get("BASELINE_DEFECT"):
        return 2
    return 1 if any(counts.get(key) for key in ("RED", "EXTENSION_MISSING", "EXTENSION_DRIFT", "EXTENSION_WARNING_RED")) else 0


if __name__ == "__main__":
    sys.exit(main())
