"""Exploratory causal show witness, no L0 or productive source access."""
import argparse
from collections import Counter
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
PREFIX = "p1339-show-witness-"
OLD_MANIFEST = BASE / "p1339-full-show-final-manifest.json"
OLD_RUNS = BASE / "p1339-full-show-final-vanilla-runs.json"
BINARIES = {
    "vanilla": {"path": "/usr/local/bin/typst", "sha256": "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8", "upstream": "a51e02804"},
    "baseline": {"path": "/tmp/p1338-target.vlNAmp/release/typst", "sha256": "f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1"},
}
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, "rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()

def read(path):
    return json.loads(Path(path).read_text())

def save_text(path, text):
    path = Path(path).resolve()
    assert path.parent == BASE and path.name.startswith(PREFIX)
    assert not path.exists(), ("immutable prior artifact", str(path))
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n" + "".join("+" + line + "\n" for line in text.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, check=True, capture_output=True)

def save(path, data):
    save_text(path, json.dumps(data, ensure_ascii=False, indent=2) + "\n")

def state():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT, text=True)
    return {"utc": now(), "head": git("rev-parse", "HEAD").strip(), "status_short": git("status", "--short"), "diff_stat": git("diff", "HEAD", "--stat")}

def manifest_path(phase):
    return BASE / (PREFIX + phase + "-manifest.json")

def runs_path(phase, product):
    return BASE / (PREFIX + phase + "-" + product + "-runs.json")

def prepare(phase):
    prior = read(OLD_MANIFEST)
    observations = read(OLD_RUNS)
    assert observations["manifest_sha256"] == sha(OLD_MANIFEST)
    inputs = {str(p.relative_to(ROOT)): sha(p) for p in (OLD_MANIFEST, OLD_RUNS, BASE / "p1339-full-a2.md", BASE / "p1339-full-cli-capabilities.json", BASE / "p1339-a0.json", ROOT / "00_nucleo/materialization/typst-passo-1339.md")}
    if phase == "final":
        focal_path = BASE / (PREFIX + "focal-observation.json")
        focal = read(focal_path)
        assert focal["focal_witness_valid"] is True
        inputs[str(focal_path.relative_to(ROOT))] = sha(focal_path)
    cases = []
    for original in prior["cases"]:
        if phase == "focal" and original["id"] not in ("strong-static-match", "strong-static-miss"):
            continue
        original_path = Path(original["path"])
        assert sha(original_path) == original["sha256"]
        assert original_path.read_text() == original["source"]
        inputs[str(original_path.relative_to(ROOT))] = sha(original_path)
        rows = [r for r in observations["rows"] if r["id"] == original["id"]]
        assert len(rows) == 8
        should_match = not original["id"].endswith("-miss")
        expected_stdout = '["MATCH"]\n' if should_match else '[]\n'
        assert all(r["exit"] == 0 and r["stdout"] == expected_stdout for r in rows)
        marker = "P1339_SHOW_WITNESS_" + original["id"].replace("-", "_").upper()
        source = original["source"].replace('metadata("MATCH")', 'panic("' + marker + '")')
        assert source != original["source"]
        path = BASE / (PREFIX + phase + "-" + original["id"] + ".typ")
        save_text(path, source)
        cases.append({"id": original["id"], "kind": "selector", "source": source, "path": str(path), "sha256": sha(path), "marker": marker, "prior_id": original["id"], "prior_source_sha256": original["sha256"], "prior_vanilla_stdout": expected_stdout, "prior_Unknown_unchanged": True, "expected_effect": "CallbackExecuted" if should_match else "CompileSucceededNoCallback", "expected_primary": 'error: panicked with: "' + marker + '"' if should_match else None, "basis": "Prior independent vanilla metadata observation, same selector and subject; callback body alone replaced. Prediction frozen before running either product."})
    for element in (["strong"] if phase == "focal" else ["strong", "emph", "text"]):
        case_id = element + "-direct-control"
        marker = "P1339_SHOW_WITNESS_" + element.upper() + "_DIRECT_CONTROL"
        subject = '#text("BASE")' if element == "text" else '#' + element + '[BASE]'
        source = '#show ' + element + ': it => panic("' + marker + '")\n' + subject + '\n'
        path = BASE / (PREFIX + phase + "-" + case_id + ".typ")
        save_text(path, source)
        cases.append({"id": case_id, "kind": "direct-control", "source": source, "path": str(path), "sha256": sha(path), "marker": marker, "expected_effect": "CallbackExecuted", "expected_primary": 'error: panicked with: "' + marker + '"', "basis": "Exploratory native element show control without where: inferred to invoke callback, explicitly refutable by either product's compile observation."})
    manifest = {"schema": "p1339-show-witness-exploratory-manifest-v1", "phase": phase, "state": state(), "runner_sha256": sha(__file__), "inputs": inputs, "binaries": BINARIES, "profiles": PROFILES, "orders": ["normal"] if phase == "focal" else ["normal", "reverse"], "cases": cases, "regime": "Exploratory independent observation; executed without isolation attestation; no contract, seal, implementation or final verdict", "capabilities": {"executor": "/root/p1312_tests", "historical_context": "Reused agent retains P1312 test-author context. Those prior functions/implementations are not inputs or evidence for P1339. No P1339 L0/productive/candidate source read.", "reads": list(inputs) + ["CLAUDE.md", "01_core/CLAUDE.md", "skill and refs", "own witness artifacts", "binary observations", "git HEAD/status/diffstat"], "writes": ["00_nucleo/diagnosticos/p1339-show-witness-*", "dedicated /tmp compile output"], "limitation": "Shared filesystem does not enforce the role allowlist."}, "observation_policy": {"callback_executed": "Nonzero exit with the first diagnostic line exactly equal to expected_primary; a marker appearing only in printed source is never a witness.", "mismatch": "Zero exit plus produced PDF and no stderr error is only negative observation; paired matching/direct controls must establish callback execution capability.", "other_diagnostic": "Record exact primary and full channels. A supported compile semantic rejection before callback is an observation of unavailable route, not proof of matching.", "Unknown": "Timeout, missing output with success, no recognizable diagnostic, CLI transport rejection, missing identity or unverified fixture remain Unknown and prevent expansion."}, "budget": "One initial focal of 3 strong cases in four profiles. At most one focal revision if needed; expand only after valid focal. Expanded 18 selectors plus 3 direct controls in four profiles, normal/reverse. No implementation or final PASS permitted.", "fragment": "Selector acceptance/rejection and causal callback invocation or non-invocation for fixed strong/emph/text empty/equal/unequal fields. Does not compare PDF bytes or callback return morphology."}
    save(manifest_path(phase), manifest)
    print(json.dumps({"manifest": str(manifest_path(phase)), "sha256": sha(manifest_path(phase)), "cases": len(cases)}))

def verify_manifest(manifest):
    assert manifest["runner_sha256"] == sha(__file__)
    for path, digest in manifest["inputs"].items():
        assert sha(ROOT / path) == digest, ("input drift", path)
    for case in manifest["cases"]:
        assert sha(case["path"]) == case["sha256"]
    for binary in BINARIES.values():
        assert sha(binary["path"]) == binary["sha256"]

def observe(proc, case, pdf):
    lines = proc["stderr"].splitlines()
    primary = next((line for line in lines if line.startswith("error:")), None)
    if proc["timeout"]:
        return "Unknown", primary
    if proc["exit"] != 0 and primary == 'error: panicked with: "' + case["marker"] + '"':
        return "CallbackExecuted", primary
    if proc["exit"] == 0 and pdf.exists() and primary is None:
        return "CompileSucceededNoCallback", primary
    if primary and ("unexpected argument '--features'" in primary or "unrecognized subcommand" in primary):
        return "Unknown", primary
    if proc["exit"] != 0 and primary:
        return "OtherDiagnosticBeforeCallback", primary
    return "Unknown", primary

def run(phase, product):
    manifest = read(manifest_path(phase))
    verify_manifest(manifest)
    if product == "baseline":
        assert runs_path(phase, "vanilla").exists()
    output_dir = Path(tempfile.mkdtemp(prefix="p1339-show-witness-"))
    result = {"schema": "p1339-show-witness-runs-v1", "start": state(), "manifest_sha256": sha(manifest_path(phase)), "runner_sha256": sha(__file__), "binary": BINARIES[product], "product": product, "output_dir": str(output_dir), "rows": []}
    env = dict(os.environ)
    for key in ("TYPST_FEATURES", "TYPST_ROOT", "TYPST_DIAGNOSTIC_FORMAT"):
        env.pop(key, None)
    env.update({"NO_COLOR": "1", "TERM": "dumb", "PYTHONDONTWRITEBYTECODE": "1"})
    for order in manifest["orders"]:
        for case in reversed(manifest["cases"]) if order == "reverse" else manifest["cases"]:
            for profile, flags in manifest["profiles"].items():
                pdf = output_dir / (case["id"] + "-" + profile + "-" + order + ".pdf")
                argv = [BINARIES[product]["path"], "compile", case["path"], str(pdf), *flags]
                stamp, tick = now(), time.monotonic()
                try:
                    p = subprocess.run(argv, input="", cwd=ROOT, env=env, capture_output=True, text=True, timeout=45)
                    proc = {"exit": p.returncode, "stdout": p.stdout, "stderr": p.stderr, "timeout": False}
                except subprocess.TimeoutExpired as error:
                    proc = {"exit": None, "stdout": error.stdout.decode() if isinstance(error.stdout, bytes) else error.stdout or "", "stderr": error.stderr.decode() if isinstance(error.stderr, bytes) else error.stderr or "", "timeout": True}
                effect, primary = observe(proc, case, pdf)
                result["rows"].append({"id": case["id"], "kind": case["kind"], "profile": profile, "order": order, "argv": argv, "cwd": str(ROOT), "stdin": "", "start": stamp, "end": now(), "seconds": time.monotonic() - tick, "source_sha256": case["sha256"], **proc, "primary_diagnostic": primary, "observed_effect": effect, "expected_effect": case["expected_effect"], "pdf_exists": pdf.exists(), "pdf_sha256": sha(pdf) if pdf.exists() else None})
    result["end"] = state()
    verify_manifest(manifest)
    save(runs_path(phase, product), result)
    print(json.dumps({"output": str(runs_path(phase, product)), "sha256": sha(runs_path(phase, product)), "observations": dict(Counter(r["observed_effect"] for r in result["rows"]))}))

def summarize(phase):
    manifest = read(manifest_path(phase))
    verify_manifest(manifest)
    products = {p: read(runs_path(phase, p)) for p in BINARIES}
    rows = {p: d["rows"] for p, d in products.items()}
    for product, data in products.items():
        assert data["manifest_sha256"] == sha(manifest_path(phase))
        assert len(data["rows"]) == len(manifest["cases"]) * 4 * len(manifest["orders"])
    vanilla_valid = all(r["observed_effect"] == r["expected_effect"] for r in rows["vanilla"])
    controls_valid = all(r["observed_effect"] == "CallbackExecuted" for r in rows["baseline"] if r["kind"] == "direct-control")
    unknown = [dict(product=p, id=r["id"], profile=r["profile"], order=r["order"]) for p in rows for r in rows[p] if r["observed_effect"] == "Unknown"]
    stability = []
    if phase == "final":
        for product in products:
            for case in manifest["cases"]:
                for profile in PROFILES:
                    pair = [r for r in rows[product] if r["id"] == case["id"] and r["profile"] == profile]
                    assert len(pair) == 2
                    keys = ("exit", "stdout", "stderr", "observed_effect", "primary_diagnostic")
                    if any(pair[0][k] != pair[1][k] for k in keys):
                        stability.append({"product": product, "id": case["id"], "profile": profile})
    result = {"schema": "p1339-show-witness-observation-v1", "state": state(), "manifest_sha256": sha(manifest_path(phase)), "runs_sha256": {p: sha(runs_path(phase, p)) for p in products}, "counts": {p: dict(Counter(r["observed_effect"] for r in rows[p])) for p in rows}, "vanilla_predictions_observed": vanilla_valid, "direct_controls_observed_in_baseline": controls_valid, "unknown": unknown, "instability": stability, "focal_witness_valid": vanilla_valid and controls_valid and not unknown, "final_verdict": "Not issued; exploratory witness only", "prior_Unknown": "Original query infrastructure Unknown cells are immutable; this successor records different compile fixtures and an explicit causal bridge. It does not prove the original metadata-producing outputs or matching in a baseline that rejects where before callback.", "other_limits": ["Angle NaN is outside this witness and unchanged.", "Compile uses PDF target in all four feature profiles; this is not an HTML-target witness.", "PDF bytes are not compared."]}
    save(BASE / (PREFIX + phase + "-observation.json"), result)
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=("prepare", "run", "summarize"))
    parser.add_argument("phase", choices=("focal", "final"))
    parser.add_argument("--product", choices=tuple(BINARIES))
    args = parser.parse_args()
    if args.action == "prepare":
        prepare(args.phase)
    elif args.action == "run":
        assert args.product
        run(args.phase, args.product)
    else:
        summarize(args.phase)
