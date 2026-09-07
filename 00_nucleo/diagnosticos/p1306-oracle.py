#!/usr/bin/env python3
"""P1306 independent public-output oracle. Never reads candidate implementation."""
import argparse
import base64
import concurrent.futures
import copy
import datetime
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
PROFILES = {"default": [], "html": ["html"], "a11y": ["a11y-extras"], "html+a11y": ["html", "a11y-extras"]}
FIXTURES = {
    "ordinary/std.typ": "#let x = 7\n#let twice(n) = n * 2\n",
    "ordinary/global.typ": "#let x = 8\n",
    "ordinary/map.typ": "#let x = 9\n",
    "ordinary/aurora.typ": "#let x = 10\n",
    "reexport/std.typ": "#import std: *\n",
    "routes/holder.typ": '#import "../ordinary/std.typ" as inner\n#let saved = inner\n',
    "routes/error.typ": '#import "../ordinary/std.typ" as inner\n#let result = inner.absent\n',
}


def cases():
    out = []
    def add(key, expression, name=None, field=None, positive=None):
        out.append(dict(id=key, expression=expression, module=name, field=field, positive_value=positive))
    for file, number in [("std", 7), ("global", 8), ("map", 9), ("aurora", 10)]:
        prefix = '{ import "ordinary/' + file + '.typ" as specimen; '
        add("ordinary-" + file, prefix + "specimen.nope }", file, "nope")
        add("positive-" + file, prefix + "(repr(specimen), specimen.x) }", positive=["<module " + file + ">", number])
    add("ordinary-bare", '{ import "ordinary/std.typ"; std.nope }', "std", "nope")
    add("ordinary-alias", '{ import "ordinary/std.typ" as original; let alias = original; alias.absent }', "std", "absent")
    add("ordinary-shadow", '{ import "ordinary/std.typ" as module; let std = module; std.absent }', "std", "absent")
    add("ordinary-nested", '{ import "ordinary/std.typ" as item; let box = (inner: (leaf: item)); box.inner.leaf.nope }', "std", "nope")
    add("ordinary-line-column", '{\n import "ordinary/std.typ" as named;\n let alias = named;\n    alias.absent\n}', "std", "absent")
    add("reexport", '{ import "reexport/std.typ" as exported; exported.nope }', "std", "nope")
    add("reexport-alias", '{ import "reexport/std.typ" as exported; let renamed = exported; renamed.absent }', "std", "absent")
    add("positive-reexport", '{ import "reexport/std.typ" as r; let renamed = r; (repr(r), repr(renamed), renamed.calc.abs(-12), repr(r.calc)) }', positive=["<module std>", "<module std>", 12, "<module calc>"])
    add("positive-alias-shadow-nested", '{ import "ordinary/std.typ" as original; let alias = original; let std = alias; let box = (inner: std); (repr(alias), repr(std), box.inner.x, box.inner.twice(6)) }', positive=["<module std>", "<module std>", 7, 12])
    add("imported-nested", '{ import "routes/holder.typ" as holder; holder.saved.absent }', "std", "absent")
    add("positive-imported-nested", '{ import "routes/holder.typ" as holder; (repr(holder.saved), holder.saved.twice(11)) }', positive=["<module std>", 22])
    add("imported-error", '{ import "routes/error.typ" as route; route.result }', "std", "absent")
    add("global", "std.nope", "global", "nope")
    add("global-alias", "{ let alias = std; alias.absent }", "global", "absent")
    add("global-renamed-import", "{ import std as renamed; renamed.nope }", "global", "nope")
    add("global-line-column", "{\n let alias = std;\n       alias.absent\n}", "global", "absent")
    add("global-bare-positive", "{ import std; (repr(std), std.calc.abs(-7)) }", positive=["<module global>", 7])
    add("global-bare-alias-positive", "{ let named = std; import named; (repr(named), named.calc.abs(-8)) }", positive=["<module global>", 8])
    add("global-import-forms", "{ import (std) as chosen; import std: calc as c; (repr(chosen), c.abs(-9)) }", positive=["<module global>", 9])
    for expression, module in [("calc", "calc"), ("sym", "sym"), ("color.map", "map")]:
        add("builtin-" + module, expression + ".absent", module, "absent")
    add("existing-lookups", "(repr(type(std.rgb)), calc.abs(-7), repr(type(sym.alpha)), repr(type(color.map.viridis)), repr(type(pdf.attach)), repr(type(pdf.artifact)))", positive=["function", 7, "symbol", "array", "function", "function"])
    add("dictionary", 'repr(type((:).nope))')
    add("float", 'repr(type(float("NaN").is-nan))')
    for field in ["data-cell", "header-cell", "table-summary"]:
        add("pdf-" + field, "repr(type(pdf." + field + "))")
    add("repr-array-boundary", "{ let a = range(41); (a.len(), a.at(40), repr(a)) }")
    return out


def pin(path):
    data = pathlib.Path(path).read_bytes()
    return {"path": str(path), "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}


def state():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT).decode()
    return {"head": git("rev-parse", "HEAD").strip(), "status": git("status", "--short"), "diff_stat": git("diff", "HEAD", "--stat"), "diff_sha256": hashlib.sha256(git("diff", "HEAD", "--binary").encode()).hexdigest()}


def execute(binary, case, profile, phase, side, cwd):
    argv = [str(binary), "eval", case["expression"], "--format", "json"]
    if PROFILES[profile]:
        argv += ["--features", ",".join(PROFILES[profile])]
    row = {"id": case["id"], "expression": case["expression"], "profile": profile, "phase": phase, "side": side, "argv": argv, "cwd": str(cwd), "started_at": datetime.datetime.now(datetime.timezone.utc).isoformat()}
    start = time.monotonic_ns()
    try:
        p = subprocess.run(argv, cwd=cwd, capture_output=True, timeout=60)
        row.update(complete=True, exit_code=p.returncode)
        streams = {"stdout": p.stdout, "stderr": p.stderr}
    except (subprocess.TimeoutExpired, OSError) as e:
        row.update(complete=False, exit_code=None, execution_error=str(e))
        streams = {"stdout": getattr(e, "stdout", b"") or b"", "stderr": getattr(e, "stderr", b"") or b""}
    row["duration_ns"] = time.monotonic_ns() - start
    for name, raw in streams.items():
        row[name] = raw.decode("utf-8", errors="replace")
        row[name + "_base64"] = base64.b64encode(raw).decode()
        row[name + "_sha256"] = hashlib.sha256(raw).hexdigest()
    return row


def public(row):
    """Exact envelope; fixture cwd relocation is explicit and limited to known paths."""
    if not isinstance(row, dict) or row.get("complete") is not True or not isinstance(row.get("exit_code"), int):
        return None
    if any(not isinstance(row.get(k), str) for k in ["stdout", "stderr", "cwd"]):
        return None
    stderr = row["stderr"]
    for relative in FIXTURES:
        stderr = stderr.replace(str(pathlib.Path(row["cwd"]) / relative), "<fixture-root>/" + relative)
    return {"exit_code": row["exit_code"], "stdout": row["stdout"], "stderr": stderr}


def discriminate(expected, row):
    """Verifier API: malformed/missing observations remain Unknown, differences Violated."""
    actual = public(row)
    if actual is None or not isinstance(expected, dict) or set(expected) != {"exit_code", "stdout", "stderr"}:
        return {"verdict": "Unknown", "reason_code": "incomplete-public-observation"}
    changed = [key for key in expected if expected[key] != actual[key]]
    return {"verdict": "Violated" if changed else "Preserved", "reason_code": "exact-envelope-difference" if changed else "exact-envelope-match", "changed": changed, "expected": expected, "actual": actual}


def measure(args):
    destination = pathlib.Path(args.output)
    if destination.exists():
        raise SystemExit("Refusing to overwrite measurement")
    before = state()
    cwd = pathlib.Path(tempfile.mkdtemp(prefix="p1306-oracle-"))
    for relative, content in FIXTURES.items():
        path = cwd / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
    selected = [c for c in cases() if not args.only or c["id"] in args.only.split(",")]
    binaries = {"crystalline": args.crystalline}
    if args.vanilla:
        binaries = {"vanilla": "/usr/local/bin/typst", **binaries}
    jobs = [(binary, case, profile, side) for case in selected for profile in PROFILES for side, binary in binaries.items()]
    rows = []
    for phase in args.phases.split(","):
        ordered = list(reversed(jobs)) if phase == "reverse" else jobs
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            results = pool.map(lambda job: execute(job[0], job[1], job[2], phase, job[3], cwd), ordered)
            rows.extend(results)
    data = {"schema": "p1306-public-measurement-v1", "preflight": pin(DIAG / "p1306-preflight.json"), "runner": pin(__file__), "binaries": {s: pin(b) for s, b in binaries.items()}, "source_before": before, "source_after": state(), "fixtures": [{"relative_path": p, "content": c, **pin(cwd / p)} for p, c in FIXTURES.items()], "profiles": PROFILES, "cases": selected, "runs": rows}
    with destination.open("x") as stream:
        json.dump(data, stream, indent=2, ensure_ascii=False)
        stream.write("\n")
    print(json.dumps({"output": str(destination), "runs": len(rows), "incomplete": sum(not r["complete"] for r in rows)}))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--crystalline", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--vanilla", action="store_true")
    parser.add_argument("--only")
    parser.add_argument("--phases", default="normal,repeat,reverse")
    measure(parser.parse_args())


if __name__ == "__main__":
    main()
