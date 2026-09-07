#!/usr/bin/env python3
"""Independent pre-candidate observations for the P1306 L0 author."""
import datetime
import hashlib
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "00_nucleo/diagnosticos/p1306-contract-measurement.json"
PREFLIGHT = ROOT / "00_nucleo/diagnosticos/p1306-preflight.json"
BINARIES = {
    "baseline": ("/dev/shm/p1305-r2-target.F7bquUC2/release/typst", "be51045f1df75ac42ee081801ddf7f738f709029e3694b9205473ba5fa8b31d4"),
    "vanilla": ("/usr/local/bin/typst", "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"),
}
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}
FIXTURES = {"std.typ": "#let token = 7\n", "global.typ": "#let token = 8\n", "map.typ": "#let token = 9\n", "forest.typ": "#let token = 10\n", "reexport/std.typ": "#import std: *\n#let token = 11\n", "nest.typ": "#import \"std.typ\"\n"}
CASES = {
    "std-nope": '{ import "std.typ"; std.nope }',
    "std-alias-absent": '{ import "std.typ" as renamed;\n  renamed.absent }',
    "std-shadow": '{ import "std.typ" as renamed; let std = renamed; std.absent }',
    "std-reexport": '{ import "reexport/std.typ" as reexported; reexported.nope }',
    "std-nested": '{ import "nest.typ"; nest.std.absent }',
    "ordinary-global": '{ import "global.typ"; global.nope }',
    "ordinary-map": '{ import "map.typ"; map.nope }',
    "ordinary-forest": '{ import "forest.typ" as leaf; leaf.absent }',
    "real-global": 'std.nope',
    "real-global-alias": '{ let renamed = std;\n   renamed.absent }',
    "calc": 'calc.nope',
    "sym": 'sym.absent',
    "color-map": 'color.map.nope',
    "positive-ordinary": '{ import "std.typ" as renamed; (repr(renamed), renamed.token) }',
    "positive-reexport": '{ import "reexport/std.typ" as renamed; (repr(renamed), renamed.token, renamed.calc.abs(-3)) }',
    "positive-global": '(repr(std), std.calc.abs(-4), repr(calc), repr(sym), repr(color.map), repr(type(std.rgb)))',
    "dictionary": '(a: 1).nope',
    "float": 'float("NaN").is-nan',
    "pdf-gate": 'pdf.data-cell',
}

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(data):
    return hashlib.sha256(data).hexdigest()

def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=True).stdout

def state():
    return {"at": now(), "head": git("rev-parse", "HEAD"), "status": git("status", "--short", "--untracked-files=all"), "diff_stat": git("diff", "HEAD", "--stat"), "diff": git("diff", "HEAD")}

def main():
    prior = json.loads(OUT.read_text()) if OUT.exists() else None
    began = now()
    before = state()
    observed = {}
    for name, (binary, expected) in BINARIES.items():
        digest = sha(pathlib.Path(binary).read_bytes())
        if digest != expected:
            raise RuntimeError(f"Binary identity mismatch: {name}: {digest}")
        observed[name] = {"path": binary, "sha256": digest}
    workspace = pathlib.Path(tempfile.mkdtemp(prefix="p1306-contract-", dir="/dev/shm"))
    fixtures = {}
    for relative, source in FIXTURES.items():
        path = workspace / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(source)
        fixtures[relative] = {"source": source, "sha256": sha(source.encode())}
    runs = []
    for order in ("normal", "reverse"):
        cases = list(CASES.items())
        if order == "reverse":
            cases.reverse()
        for profile, flags in PROFILES.items():
            for case, expression in cases:
                for product, (binary, _) in BINARIES.items():
                    argv = [binary, "eval", *flags, expression]
                    started = now()
                    clock = time.monotonic()
                    result = subprocess.run(argv, cwd=workspace, capture_output=True, text=True)
                    runs.append({"order": order, "profile": profile, "case": case, "product": product, "argv": argv, "cwd": str(workspace), "expression_sha256": sha(expression.encode()), "started": started, "finished": now(), "seconds": time.monotonic() - clock, "exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr})
    payload = {"protocol": "tekt-materializacao-segregada/full/1", "role": "independent contract author", "preflight_sha256": sha(PREFLIGHT.read_bytes()), "script_sha256": sha(pathlib.Path(__file__).read_bytes()), "started": began, "finished": now(), "state_before": before, "state_after": state(), "vanilla_upstream": "a51e02804", "binaries": observed, "fixtures": fixtures, "expressions": CASES, "profiles": PROFILES, "runs": runs, "classification": "raw independent observations; not a verdict"}
    if prior is not None:
        payload["prior_attempt"] = prior
        payload["instrumentation_revision"] = {"cause": "Vanilla eval rejects --color; invocation failure is not RED", "hypothesis": "Remove unsupported option; captured pipes already produce plain diagnostics", "prior_artifact_sha256": sha(OUT.read_bytes()), "revision": 1}
    OUT.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"output": str(OUT), "sha256": sha(OUT.read_bytes()), "runs": len(runs), "started": began, "finished": payload["finished"]}))
    for run in runs:
        if run["order"] == "normal" and run["profile"] == "default":
            print(run["case"], run["product"], run["exit"], repr(run["stdout"]), repr(run["stderr"]))

if __name__ == "__main__":
    main()
