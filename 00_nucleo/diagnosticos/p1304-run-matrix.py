#!/usr/bin/env python3
"""P1304: mechanical A/B evidence runner; no semantic or ownership decisions.

The historical catalog is read only. Run --prepare first, then provide the fresh
coordinator-built binary and its expected SHA256. Exactly one complete corpus is
executed, followed by one reversed, explicitly bounded subset. Output replacement
is refused so a second run cannot silently erase evidence.
"""

import argparse
import base64
import collections
import datetime
import hashlib
import json
import math
import pathlib
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
HERE = pathlib.Path(__file__).resolve().parent
HISTORICAL = HERE / "p1299-probe-catalog.json"
CATALOG = HERE / "p1304-probe-catalog.json"
OUTPUT = HERE / "p1304-feature-matrix.json"
HISTORICAL_SHA = "649dd46f05e67d376a8096756a31207a6a7d4087acf5fcbdd6688a59c4f934ae"
VANILLA_SHA = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
PROFILES = {"default": [], "html": ["html"], "a11y": ["a11y-extras"],
            "html+a11y": ["html", "a11y-extras"]}
MATCH = {"MATCH_VALUE", "MATCH_DIAGNOSTIC"}
REPEAT_PATHS = {"hsl", "hsv", "linear_rgb", "std.hsl", "std.hsv", "std.linear_rgb",
                "calc.nope", "sym.nope", "color.map.nope", "std.rgb", "calc.abs",
                "sym.arrow", "color.map.turbo", "color.hsl", "color.hsv",
                "color.linear-rgb", "pdf.data-cell", "pdf.header-cell", "pdf.table-summary"}
SUPPLEMENT_PATHS = {"calc.nope", "color.map.nope", "std.hsl", "std.hsv", "std.linear_rgb",
                    "std.rgb", "sym.arrow", "sym.nope"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def sha256(path):
    h = hashlib.sha256()
    with pathlib.Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write_new(path, data):
    with path.open("x", encoding="utf-8") as stream:
        json.dump(data, stream, ensure_ascii=False, indent=2, sort_keys=True)
        stream.write("\n")


def source_state():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT)
    return {"captured_at": utc(), "head": git("rev-parse", "HEAD").decode().strip(),
            "status_porcelain": git("status", "--porcelain=v1", "--untracked-files=all").decode(),
            "diff_head_stat": git("diff", "HEAD", "--stat").decode(),
            "tracked_diff_sha256": digest(git("diff", "HEAD", "--binary"))}


def build_catalog():
    if sha256(HISTORICAL) != HISTORICAL_SHA:
        raise ValueError("historical catalog changed")
    old = json.loads(HISTORICAL.read_text())
    probes = old["probes"]
    if len(probes) != 626 or len({p["id"] for p in probes}) != 626:
        raise ValueError("historical probe cardinality or IDs invalid")
    if not REPEAT_PATHS - SUPPLEMENT_PATHS <= {p["path"] for p in probes}:
        raise ValueError("missing required controls: " + repr(REPEAT_PATHS - {p["path"] for p in probes}))
    result = {"schema_version": "p1304-probe-catalog-v1", "created_at": utc(),
              "historical_input": {"path": str(HISTORICAL.relative_to(ROOT)), "sha256": HISTORICAL_SHA},
              "preservation": "All 626 historical probe records retained verbatim as JSON values, in original order.",
              "counts": {"historical_probes": 626, "added_probes": 1, "probes": 627},
              "profiles": PROFILES, "probes": probes + [
                  {"id": "p1304-repr-std", "path": "std", "expression": "repr(std)",
                   "profiles": list(PROFILES), "origins": ["p1304:explicit-repr-std"],
                   "roles": ["explicit-repr-probe"]}]}
    if len({p["id"] for p in result["probes"]}) != 627:
        raise ValueError("duplicate added probe ID")
    return result


def feature_argv(profile):
    return ["--features", ",".join(PROFILES[profile])] if PROFILES[profile] else []


def run_side(binary, binary_sha, probe, profile, side, phase, timeout):
    argv = [str(binary), "eval", probe["expression"], "--format", "json", *feature_argv(profile)]
    started_at = utc()
    start = time.monotonic_ns()
    reason = None
    execution_error = None
    try:
        process = subprocess.run(argv, cwd=ROOT, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, timeout=timeout)
        code, stdout, stderr = process.returncode, process.stdout, process.stderr
        if code < 0:
            reason = "SIGNAL"
    except subprocess.TimeoutExpired as error:
        code, stdout, stderr = None, error.stdout or b"", error.stderr or b""
        reason = "TIMEOUT"
    except OSError as error:
        code, stdout, stderr = None, b"", b""
        reason, execution_error = "SPAWN_OS_ERROR", str(error)
    duration = time.monotonic_ns() - start
    return {"run_id": f"{phase}/{probe['id']}/{profile}/{side}", "id": probe["id"],
            "path": probe["path"], "expression": probe["expression"], "profile": profile,
            "features": PROFILES[profile], "side": side, "phase": phase,
            "argv": argv, "cwd": str(ROOT), "binary_path": str(binary),
            "binary_sha256": binary_sha, "started_at": started_at, "exit_code": code,
            "stdout": stdout.decode("utf-8", errors="replace"),
            "stderr": stderr.decode("utf-8", errors="replace"),
            "stdout_base64": base64.b64encode(stdout).decode("ascii"),
            "stderr_base64": base64.b64encode(stderr).decode("ascii"),
            "stdout_sha256": digest(stdout), "stderr_sha256": digest(stderr),
            "duration_ns": duration, "complete": reason is None, "reason_code": reason,
            "execution_error": execution_error}


def classify(vanilla, crystalline):
    if not vanilla["complete"] or not crystalline["complete"]:
        return "EXECUTION_UNKNOWN"
    v, c = vanilla["exit_code"], crystalline["exit_code"]
    if v == 0 and c == 0:
        return "MATCH_VALUE" if vanilla["stdout_base64"] == crystalline["stdout_base64"] else "DIFFERENT_VALUE"
    if v != 0 and c == 0:
        return "CRYSTALLINE_ONLY"
    if v == 0 and c != 0:
        return "VANILLA_ONLY"
    if v == c and vanilla["stderr_base64"] == crystalline["stderr_base64"]:
        return "MATCH_DIAGNOSTIC"
    return "DIFFERENT_DIAGNOSTIC"


def execute(catalog, binaries, timeout):
    normal = []
    inverse = []
    jobs = [(p, profile) for profile in PROFILES for p in catalog["probes"]]
    supplement_probes = [{"id": "p1304-control-" + path, "path": path,
                          "expression": f"repr((type({path}), repr({path})))"}
                         for path in sorted(SUPPLEMENT_PATHS)]
    supplement_jobs = [(p, profile) for profile in PROFILES for p in supplement_probes]

    def pair(probe, profile, phase, side_order):
        result = {"id": probe["id"], "path": probe["path"], "expression": probe["expression"],
                  "profile": profile, "features": PROFILES[profile], "phase": phase}
        for side in side_order:
            binary = binaries[side]
            result[side] = run_side(pathlib.Path(binary["path"]), binary["sha256"],
                                    probe, profile, side, phase, timeout)
        result["runtime_class"] = classify(result["vanilla"], result["crystalline"])
        return result

    for i, (probe, profile) in enumerate(jobs, 1):
        normal.append(pair(probe, profile, "normal", ("vanilla", "crystalline")))
        if i % 100 == 0:
            print(json.dumps({"phase": "normal", "completed_pairs": i}), flush=True)
    supplement_normal = [pair(p, profile, "supplement_normal", ("vanilla", "crystalline"))
                         for p, profile in supplement_jobs]
    nonmatch_paths = {r["path"] for r in normal if r["runtime_class"] not in MATCH}
    classes_by_id = collections.defaultdict(set)
    for r in normal:
        classes_by_id[r["id"]].add(r["runtime_class"])
    match_ids = {i for i, classes in classes_by_id.items() if classes <= MATCH}
    sample = sorted(match_ids, key=lambda i: (digest(i.encode()), i))[:math.ceil(len(match_ids) * .05)]
    reasons = {}
    for probe in catalog["probes"]:
        tags = []
        if probe["path"] in nonmatch_paths:
            tags.append("non_match_path_in_normal")
        if probe["path"] in REPEAT_PATHS:
            tags.append("mandatory_control_path")
        if probe["id"] == "p1304-repr-std":
            tags.append("explicit_repr_std")
        if probe["id"] in sample:
            tags.append("deterministic_match_sample")
        if tags:
            reasons[probe["id"]] = tags
    repeated_jobs = [(p, profile) for p, profile in reversed(jobs) if p["id"] in reasons]
    for i, (probe, profile) in enumerate(repeated_jobs, 1):
        inverse.append(pair(probe, profile, "inverted", ("crystalline", "vanilla")))
        if i % 100 == 0:
            print(json.dumps({"phase": "inverted", "completed_pairs": i}), flush=True)
    supplement_inverse = [pair(p, profile, "supplement_inverted", ("crystalline", "vanilla"))
                          for p, profile in reversed(supplement_jobs)]
    initial = {(r["id"], r["profile"]): r for r in normal + supplement_normal}
    comparisons = []
    fields = ("exit_code", "stdout_base64", "stderr_base64", "complete", "reason_code")
    for r in inverse + supplement_inverse:
        old = initial[r["id"], r["profile"]]
        for side in ("vanilla", "crystalline"):
            changed = [key for key in fields if old[side][key] != r[side][key]]
            comparisons.append({"id": r["id"], "profile": r["profile"], "side": side,
                                "binary_sha256": r[side]["binary_sha256"],
                                "normal_run_id": old[side]["run_id"], "inverted_run_id": r[side]["run_id"],
                                "identical": not changed, "changed_fields": changed})
    all_results = normal + inverse + supplement_normal + supplement_inverse
    return {"results": normal, "inverted_results": inverse,
            "supplement": {"supplement_reason": "Eight explicit Fase C controls absent from the historical corpus; coordinator authorized normal and inverted observations outside the 627-probe corpus.",
                           "probe_ids": [p["id"] for p in supplement_probes],
                           "normal_results": supplement_normal, "inverted_results": supplement_inverse,
                           "pairs": len(supplement_normal) + len(supplement_inverse),
                           "processes": 2 * (len(supplement_normal) + len(supplement_inverse))},
            "repetition": {"order": "reverse all selected profile/probe jobs and reverse binary order",
                           "mandatory_paths": sorted(REPEAT_PATHS), "non_match_paths": sorted(nonmatch_paths),
                           "match_sample_population": len(match_ids), "match_sample_size": len(sample),
                           "sample_rule": "ceil(5% of IDs matching in all four profiles), ascending SHA256(UTF-8 ID)",
                           "sample_ids": sample, "selected_id_reasons": reasons, "comparisons": comparisons},
            "counts": {"normal_pairs": len(normal), "inverted_pairs": len(inverse),
                       "corpus_processes": 2 * (len(normal) + len(inverse)),
                       "supplement_pairs": len(supplement_normal) + len(supplement_inverse),
                       "supplement_processes": 2 * (len(supplement_normal) + len(supplement_inverse)),
                       "processes": 2 * len(all_results),
                       "normal_by_class": dict(collections.Counter(r["runtime_class"] for r in normal)),
                       "normal_by_profile": {p: dict(collections.Counter(r["runtime_class"] for r in normal if r["profile"] == p)) for p in PROFILES},
                       "unknown_pairs": sum(r["runtime_class"] == "EXECUTION_UNKNOWN" for r in all_results),
                       "nonrepeatable_sides": sum(not c["identical"] for c in comparisons)}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--crystalline-bin", type=pathlib.Path)
    parser.add_argument("--crystalline-sha256")
    parser.add_argument("--timeout-seconds", type=float, default=30.0)
    args = parser.parse_args()
    if args.prepare:
        write_new(CATALOG, build_catalog())
        print(json.dumps({"catalog": str(CATALOG), "sha256": sha256(CATALOG), "probes": 627}))
        return
    if args.crystalline_bin is None or not args.crystalline_sha256:
        parser.error("fresh --crystalline-bin and --crystalline-sha256 are required")
    if OUTPUT.exists():
        raise FileExistsError("refusing to overwrite existing run evidence")
    catalog = json.loads(CATALOG.read_text())
    reference = build_catalog()
    if catalog["probes"] != reference["probes"] or catalog["profiles"] != PROFILES:
        raise ValueError("prepared catalog is not the declared corpus")
    vanilla = pathlib.Path("/usr/local/bin/typst")
    crystalline = args.crystalline_bin.resolve(strict=True)
    if crystalline == (ROOT / "target/release/typst").resolve():
        raise ValueError("root target/release reuse is forbidden")
    binaries = {"vanilla": {"path": str(vanilla), "sha256": sha256(vanilla)},
                "crystalline": {"path": str(crystalline), "sha256": sha256(crystalline)}}
    if binaries["vanilla"]["sha256"] != VANILLA_SHA or binaries["crystalline"]["sha256"] != args.crystalline_sha256:
        raise ValueError("binary identity mismatch")
    inputs = {str(p.relative_to(ROOT)): sha256(p) for p in (HISTORICAL, CATALOG, pathlib.Path(__file__).resolve())}
    started_at, started_ns, before = utc(), time.monotonic_ns(), source_state()
    result = execute(catalog, binaries, args.timeout_seconds)
    after = source_state()
    stable = {"source_head": before["head"] == after["head"],
              "tracked_diff": before["tracked_diff_sha256"] == after["tracked_diff_sha256"],
              "inputs": all(sha256(ROOT / p) == h for p, h in inputs.items()),
              "binaries": all(sha256(b["path"]) == b["sha256"] for b in binaries.values())}
    result.update({"schema_version": "p1304-feature-matrix-v1", "regime": "executado sem atestacao de isolamento tecnico",
                   "runner_role": "AUTOR/RUNNER A/B; no semantic classification, ownership or cohort selection",
                   "started_at": started_at, "finished_at": utc(), "duration_ns": time.monotonic_ns() - started_ns,
                   "cwd": str(ROOT), "binaries": binaries, "profiles": PROFILES, "input_sha256": inputs,
                   "source_before": before, "source_after": after, "identity_stability": stable,
                   "timeout_seconds": args.timeout_seconds, "unknown_policy": "No Unknown accepted",
                   "budget": "one full 627 x 4 bilateral corpus, one bounded inverted subset; eight supplementary controls x four profiles x two orders x two binaries = 128 supplementary processes"})
    result["evidence_complete"] = (result["counts"]["unknown_pairs"] == 0 and
                                   result["counts"]["nonrepeatable_sides"] == 0 and all(stable.values()))
    write_new(OUTPUT, result)
    print(json.dumps({"output": str(OUTPUT), "sha256": sha256(OUTPUT), "counts": result["counts"],
                      "evidence_complete": result["evidence_complete"]}), flush=True)
    if not result["evidence_complete"]:
        raise SystemExit(2)


if __name__ == "__main__":
    main()
