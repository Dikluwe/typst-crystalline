#!/usr/bin/env python3
"""P1323 A/B external CLI observer. No productive source inspection.

freeze: observe only pinned vanilla, freeze cases and exact expectations.
run: consume the frozen suite and execute baseline then candidate in normal,
repeated and reversed case order. Outputs are evidence for the verifier.
The observer mutation calibration is not productive mutation testing.
"""
import argparse
import base64
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
FIXTURE = DIAG / "p1323-fixtures/hello.typ"
ERROR = DIAG / "p1323-fixtures/ab-error.typ"
VANILLA = Path("/usr/local/bin/typst")
VANILLA_SHA = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
SPEC_WARNING = (
    b"warning: html export is under active development and incomplete\n"
    b" = hint: its behaviour may change at any time\n"
    b" = hint: do not rely on this feature for production use cases\n"
    b" = hint: see https://github.com/typst/typst/issues/5512 for more information\n\n"
)
HEADLINE = SPEC_WARNING.splitlines(keepends=True)[0]
PROTECTED = [Path(__file__).resolve(), DIAG / "p1323-ab-unit.rs", FIXTURE, ERROR]
NUCLEI = [ROOT / "00_nucleo/prompts/_nuclei" / p for p in (
    "compiler-feature-gates.toml", "network/custom-ca-cert.toml", "wiring/cli-observables.toml")]
PROMPT = ROOT / "00_nucleo/prompts/wiring.md"


def obligation():
    content = PROMPT.read_bytes()
    body = b"".join(line for line in content.splitlines(keepends=True)
                    if not line.startswith(b"Hash do C\xc3\xb3digo:"))
    return {"path": str(PROMPT), "raw_sha256": sha(PROMPT),
            "normative_sha256": hashlib.sha256(body).hexdigest(),
            "algorithm": "SHA256 of raw UTF-8 bytes excluding only lines starting Hash do Código:",
            "nuclei": [identity(p) for p in NUCLEI]}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def b64(data):
    return base64.b64encode(data).decode("ascii")


def raw(obs, stream):
    return base64.b64decode(obs[stream + "_base64"])


def identity(path):
    path = Path(path).resolve()
    return {"path": str(path), "sha256": sha(path)}


def provenance(manifest):
    def git(*argv):
        return subprocess.check_output(["git", *argv], cwd=ROOT).decode()
    return {"utc": utc(), "head": git("rev-parse", "HEAD").strip(),
            "working_tree_diff_stat": git("diff", "HEAD", "--stat"),
            "status_porcelain": git("status", "--porcelain=v1"),
            "manifest": identity(manifest), "runner": identity(__file__),
            "regime": "A/B executed without technical isolation attestation"}


def environment():
    # Preserve the host's font environment consistently; remove CLI overrides.
    env = {k: v for k, v in os.environ.items() if not k.startswith("TYPST_")}
    env.update(NO_COLOR="1", TERM="dumb", LC_ALL="C", SOURCE_DATE_EPOCH="1700000000")
    return env


def cases():
    result = []
    def compile_case(name, features=None, mode=None, legacy=False, fmt="html", error=False):
        args = ["--color", "never"] + ([] if legacy else ["compile"])
        args += [str(ERROR if error else FIXTURE), "{output}", "--format", fmt]
        if features:
            args += ["--features", features]
        if mode:
            args += ["--html-serialization", mode]
        if fmt == "pdf":
            args += ["--document-id", "00000000-0000-0000-0000-000000000001"]
        enabled = fmt == "html" and features is not None and "html" in features.split(",")
        result.append({"id": name, "argv": args, "format": fmt,
                       "warning_expected": enabled, "error_expected": error or (fmt == "html" and not enabled),
                       "stdin_base64": b64(b"")})
    compile_case("html-legacy", "html", legacy=True)
    compile_case("html-compile", "html")
    compile_case("html-combined", "html,a11y-extras")
    compile_case("html-crystalline", "html", mode="crystalline")
    compile_case("html-vanilla", "html", mode="vanilla")
    compile_case("gate-default")
    compile_case("gate-a11y-only", "a11y-extras")
    compile_case("html-error-order", "html", error=True)
    for fmt in ("pdf", "png", "svg"):
        compile_case("preserve-" + fmt, "html,a11y-extras", fmt=fmt)
    for name, args, stdin in (
        ("query-file", ["--color", "never", "query", str(FIXTURE), "heading"], b""),
        ("query-stdin", ["--color", "never", "query", "-", "heading"], FIXTURE.read_bytes()),
        ("eval", ["--color", "never", "eval", "1 + 1"], b""),
        ("help", ["--help"], b""),
    ):
        result.append({"id": name, "argv": args, "format": None,
                       "warning_expected": False, "error_expected": False,
                       "stdin_base64": b64(stdin)})
    return result


def observe(binary, case, output):
    args = [str(binary)] + [str(output) if a == "{output}" else a for a in case["argv"]]
    begin = utc()
    record = {"case": case["id"], "argv": args, "cwd": str(ROOT), "started_utc": begin,
              "stdin_base64": case["stdin_base64"], "environment_overrides":
              {"NO_COLOR": "1", "TERM": "dumb", "LC_ALL": "C", "SOURCE_DATE_EPOCH": "1700000000",
               "TYPST_*": "removed"}}
    try:
        run = subprocess.run(args, input=base64.b64decode(case["stdin_base64"]),
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             cwd=ROOT, env=environment(), timeout=45)
        record.update(status="observed", exit=run.returncode,
                      stdout_base64=b64(run.stdout), stderr_base64=b64(run.stderr))
    except (OSError, subprocess.TimeoutExpired) as exc:
        record.update(status="opaque", reason=type(exc).__name__)
    if case["format"] and Path(output).is_file():
        record["artifact"] = {**identity(output), "size": Path(output).stat().st_size}
    else:
        record["artifact"] = None
    record["ended_utc"] = utc()
    return record


def classify_warning(obs, expected, enabled=True, clean=True):
    if obs.get("status") != "observed":
        return "Unknown"
    stderr, stdout = raw(obs, "stderr"), raw(obs, "stdout")
    if HEADLINE in stdout:
        return "Violated"
    if not enabled:
        return "Violated" if HEADLINE in stderr else "Preserved"
    if stderr.count(HEADLINE) != 1 or not stderr.startswith(expected):
        return "Violated"
    if clean and stderr != expected:
        return "Violated"
    return "Preserved"


def calibrate(expected):
    positive = {"status": "observed", "stdout_base64": b64(b""), "stderr_base64": b64(expected)}
    samples = [("positive", positive, "Preserved")]
    lines = expected.splitlines(keepends=True)
    changes = {
        "hint-removed": b"".join(lines[:1] + lines[2:]),
        "hint-reordered": b"".join([lines[0], lines[2], lines[1], *lines[3:]]),
        "duplicated": expected + expected,
        "termination-removed": expected[:-1],
        "termination-extra": expected + b"\n",
    }
    for name, changed in changes.items():
        obs = copy.deepcopy(positive)
        obs["stderr_base64"] = b64(changed)
        samples.append((name, obs, "Violated"))
    wrong_channel = dict(positive, stdout_base64=b64(expected), stderr_base64=b64(b""))
    samples += [("wrong-channel", wrong_channel, "Violated"),
                ("opaque", {"status": "opaque"}, "Unknown")]
    rows = [{"id": name, "expected": want, "actual": classify_warning(obs, expected)}
            for name, obs, want in samples]
    assert all(row["actual"] == row["expected"] for row in rows), rows
    return {"kind": "mutations of copied observable; not productive mutants", "revision": 0,
            "budget": "two focal revisions without discriminatory gain; then stop",
            "rows": rows, "repeat_and_reverse": [classify_warning(obs, expected)
                                                    for _, obs, _ in reversed(samples)]}


def freeze(args):
    assert sha(VANILLA) == VANILLA_SHA, "ratified vanilla identity mismatch"
    fixed = cases()
    # Oracle probe is deliberately vanilla-only and precedes any candidate read.
    probe = {"id": "oracle-vanilla", "argv": ["--color", "never", "compile", str(FIXTURE), "-", "--format", "html",
             "--features", "html"], "format": None, "stdin_base64": b64(b"")}
    obs = observe(VANILLA, probe, "-")
    assert obs["status"] == "observed" and obs["exit"] == 0 and raw(obs, "stdout"), obs
    expected = raw(obs, "stderr")
    assert expected == SPEC_WARNING, "vanilla does not match the frozen L0 envelope"
    return {"kind": "frozen-suite", "provenance": provenance(args.manifest),
            "vanilla": identity(VANILLA), "oracle_observation": obs,
            "warning_base64": b64(expected), "cases": fixed,
            "protected": [identity(p) for p in PROTECTED], "obligation": obligation(),
            "calibration": calibrate(expected),
            "pre_freeze_revision": {"revision": 1, "source": "vanilla-only argv probe",
                                    "failed_utc": "2026-09-08T23:23:06.458375+00:00",
                                    "cause": "--color after subcommand rejected with exit 2",
                                    "correction": "move --color never before subcommand in every fixed case",
                                    "candidate_or_baseline_read": False, "cost": "one failed vanilla process"},
            "scope": "exact colorless warning only; baseline preservation controls; no global HTML parity"}


def run_suite(args):
    frozen = json.loads(Path(args.suite).read_text())
    assert frozen["provenance"]["manifest"] == identity(args.manifest), "manifest changed"
    for item in frozen["protected"]:
        assert sha(item["path"]) == item["sha256"], "suite input changed: " + item["path"]
    current_obligation = obligation()
    assert frozen["obligation"]["normative_sha256"] == current_obligation["normative_sha256"], "normative L0 changed"
    assert frozen["obligation"]["nuclei"] == current_obligation["nuclei"], "nucleus changed"
    expected = base64.b64decode(frozen["warning_base64"])
    binaries = {"baseline": Path(args.baseline).resolve(), "candidate": Path(args.candidate).resolve()}
    result = {"kind": "A/B observations", "provenance": provenance(args.manifest),
              "suite": identity(args.suite), "binaries": {k: identity(v) for k, v in binaries.items()},
              "rounds": [], "obligation": current_obligation, "scope": frozen["scope"],
              "product_verdict": "reserved for independent verifier"}
    with tempfile.TemporaryDirectory(prefix="p1323-ab-") as temporary:
        for order, fixed in (("normal", frozen["cases"]), ("repeat", frozen["cases"]),
                             ("reverse", list(reversed(frozen["cases"])))):
            rows = []
            for case in fixed:
                obs = {}
                for role, binary in binaries.items():
                    output = Path(temporary) / (order + "-" + role + "-" + case["id"] + "." + str(case["format"]))
                    obs[role] = observe(binary, case, output)
                base, candidate = obs["baseline"], obs["candidate"]
                checks = {"warning": classify_warning(candidate, expected, case["warning_expected"],
                                                     not case["error_expected"])}
                if any(o["status"] != "observed" for o in obs.values()):
                    checks["preservation"] = "Unknown"
                else:
                    base_stderr = raw(base, "stderr")
                    candidate_stderr = raw(candidate, "stderr")
                    if case["warning_expected"]:
                        # Remove exactly the old headline (or complete envelope) only at its existing position.
                        old_prefix = expected if base_stderr.startswith(expected) else HEADLINE
                        base_stderr = base_stderr.removeprefix(old_prefix)
                        candidate_stderr = candidate_stderr.removeprefix(expected)
                    equal_artifact = (base["artifact"] is None and candidate["artifact"] is None) or (
                        base["artifact"] is not None and candidate["artifact"] is not None and
                        base["artifact"]["sha256"] == candidate["artifact"]["sha256"])
                    success = candidate["exit"] != 0 if case["error_expected"] else candidate["exit"] == 0
                    artifact_ok = (candidate["artifact"] is None if case["error_expected"] else
                                   (not case["format"] or (candidate["artifact"] is not None and candidate["artifact"]["size"] > 0)))
                    preserved = (base["exit"] == candidate["exit"] and
                                 raw(base, "stdout") == raw(candidate, "stdout") and
                                 base_stderr == candidate_stderr and equal_artifact and success and artifact_ok)
                    checks["preservation"] = "Preserved" if preserved else "Violated"
                    checks["details"] = {"exit_equal": base["exit"] == candidate["exit"],
                                         "stdout_equal": raw(base, "stdout") == raw(candidate, "stdout"),
                                         "remaining_stderr_equal": base_stderr == candidate_stderr,
                                         "artifact_equal_between_crystalline_builds": equal_artifact,
                                         "expected_success_or_failure": success, "artifact_presence_valid": artifact_ok}
                rows.append({"case": case, "observations": obs, "checks": checks})
            result["rounds"].append({"order": order, "rows": rows})
    def stable_view(row, role):
        obs = row["observations"][role]
        return {k: obs.get(k) for k in ("status", "exit", "stdout_base64", "stderr_base64")} | {
            "artifact_sha256": obs["artifact"]["sha256"] if obs["artifact"] else None}
    normal = {row["case"]["id"]: row for row in result["rounds"][0]["rows"]}
    result["order_checks"] = [{"order": round_["order"], "case": row["case"]["id"],
                              "role": role, "stable": stable_view(row, role) == stable_view(normal[row["case"]["id"]], role)}
                             for round_ in result["rounds"][1:] for row in round_["rows"] for role in binaries]
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("phase", choices=["freeze", "run"])
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--receipt", required=True)
    parser.add_argument("--suite")
    parser.add_argument("--baseline")
    parser.add_argument("--candidate")
    args = parser.parse_args()
    if args.phase == "run" and not all((args.suite, args.baseline, args.candidate)):
        parser.error("run requires --suite, --baseline and --candidate")
    report = freeze(args) if args.phase == "freeze" else run_suite(args)
    # Exclusive output creation prevents accidental replacement of evidence.
    with Path(args.receipt).open("x") as output:
        json.dump(report, output, indent=2, ensure_ascii=False)
        output.write("\n")
    print(json.dumps({"receipt": identity(args.receipt), "kind": report["kind"]}))


if __name__ == "__main__":
    main()
