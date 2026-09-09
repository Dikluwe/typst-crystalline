#!/usr/bin/env python3
"""Successor to frozen R1: only PDF comparison and evidence retention change.

The CLI corpus, warning oracle and all non-PDF checks are inherited unchanged.
Only the unique validated 16-byte XMP InstanceID payload is excluded from PDF
identity; every other original PDF byte remains part of the comparison.
"""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import re
import runpy
import tempfile

HERE = Path(__file__).resolve().parent
R1_PATH = HERE / "p1323-ab-cli.py"
R1_SUITE = HERE / "p1323-ab-frozen-suite.json"
FOCAL_PATH = HERE / "p1323-ab-pdf-focal-receipt.json"
R1 = runpy.run_path(str(R1_PATH))
sha, identity, raw = R1["sha"], R1["identity"], R1["raw"]
INSTANCE = re.compile(rb"<xmpMM:InstanceID>([A-Za-z0-9+/]{22}==)</xmpMM:InstanceID>")


def pdf_identity(data):
    """Narrow observer for the uncompressed XMP envelope measured in R1 baseline."""
    matches = list(INSTANCE.finditer(data))
    if (not data.startswith(b"%PDF-") or not data.rstrip().endswith(b"%%EOF") or
            data.count(b"<xmpMM:InstanceID>") != 1 or len(matches) != 1):
        return {"status": "Unknown", "reason": "PDF or single supported InstanceID envelope unavailable"}
    match = matches[0]
    value = match.group(1)
    try:
        decoded = base64.b64decode(value, validate=True)
    except ValueError:
        return {"status": "Unknown", "reason": "invalid InstanceID base64"}
    if len(decoded) != 16 or base64.b64encode(decoded) != value:
        return {"status": "Unknown", "reason": "noncanonical 16-byte InstanceID"}
    start, end = match.span(1)
    normalized = data[:start] + b"0" * (end - start) + data[end:]
    return {"status": "Observed", "all_other_bytes_sha256": hashlib.sha256(normalized).hexdigest(),
            "excluded": {"field": "xmpMM:InstanceID", "byte_range_half_open": [start, end],
                         "original_base64_text": value.decode()},
            "original_size": len(data), "original_sha256": hashlib.sha256(data).hexdigest()}


def compare_pdf(left, right):
    if left["status"] != "Observed" or right["status"] != "Observed":
        return "Unknown"
    return "Preserved" if left["all_other_bytes_sha256"] == right["all_other_bytes_sha256"] else "Violated"


def pdf_calibration(source):
    data = Path(source).read_bytes()
    original = pdf_identity(data)
    assert original["status"] == "Observed"
    replacements = [
        ("page-width-changed", b"/MediaBox [0 0 595.2756", b"/MediaBox [0 0 596.2756", "Violated"),
        ("font-name-changed", b"LibertinusSerif-Regular", b"LibertinusSerif-Regulaz", "Violated"),
        ("document-id-changed", b"<xmpMM:DocumentID>AAAAAAAAAAAAAAAAAAAAAQ==", b"<xmpMM:DocumentID>AAAAAAAAAAAAAAAAAAAAAg==", "Violated"),
        ("instance-envelope-absent", b"<xmpMM:InstanceID>", b"<xmpMM:InstanceIx>", "Unknown"),
    ]
    samples = [("positive", data, "Preserved")]
    for name, before, after, expected in replacements:
        assert before in data, name
        samples.append((name, data.replace(before, after, 1), expected))
    start, end = original["excluded"]["byte_range_half_open"]
    samples.extend([
        ("only-instance-id-changed", data[:start] + b"AAAAAAAAAAAAAAAAAAAAAQ==" + data[end:], "Preserved"),
        ("opaque-bytes", b"opaque", "Unknown"),
        ("duplicate-instance-envelope", data.replace(b"</xmpMM:InstanceID>", b"</xmpMM:InstanceID><xmpMM:InstanceID>AAAAAAAAAAAAAAAAAAAAAQ==</xmpMM:InstanceID>", 1), "Unknown"),
    ])
    rows = [{"id": name, "expected": expected, "actual": compare_pdf(original, pdf_identity(changed))}
            for name, changed, expected in samples]
    assert all(row["expected"] == row["actual"] for row in rows), rows
    reversed_rows = [{"id": name, "actual": compare_pdf(original, pdf_identity(changed))}
                     for name, changed, _ in reversed(samples)]
    return {"kind": "copies of PDF observable, not productive mutants", "rows": rows,
            "repeat_reverse": reversed_rows, "source": identity(source)}


def focal_proof():
    focal = json.loads(FOCAL_PATH.read_text())
    proofs, bytes_by_run = [], []
    for row in focal["observations"]:
        artifact = row["observation"]["artifact"]
        assert artifact and sha(artifact["path"]) == artifact["sha256"]
        data = Path(artifact["path"]).read_bytes()
        bytes_by_run.append(data)
        proof = pdf_identity(data)
        assert proof["status"] == "Observed"
        proofs.append({"role": row["role"], "iteration": row["iteration"], "identity": proof})
    reference = bytes_by_run[0]
    span = proofs[0]["identity"]["excluded"]["byte_range_half_open"]
    differences = []
    for index, data in enumerate(bytes_by_run[1:], start=1):
        assert len(reference) == len(data)
        offsets = [i for i, (a, b) in enumerate(zip(reference, data)) if a != b]
        assert offsets and all(span[0] <= i < span[1] for i in offsets)
        differences.append({"reference": 0, "other": index, "changed_offsets_zero_based": offsets,
                            "all_changes_inside_instance_payload": True})
    assert len({row["identity"]["all_other_bytes_sha256"] for row in proofs}) == 1
    inspections = {}
    for tool in ("pdfinfo", "pdftotext", "pdffonts", "pdfimages"):
        values = [row["inspections"][tool] for row in focal["observations"]]
        assert all(v["exit"] == 0 for v in values)
        inspections[tool] = {"all_equal": len({v["stdout"] for v in values}) == 1,
                             "value": values[0]["stdout"]}
        assert inspections[tool]["all_equal"]
    render_hashes = [sha(Path(row["observation"]["artifact"]["path"]).with_suffix(".png")) for row in focal["observations"]]
    assert len(set(render_hashes)) == 1
    return {"receipt": identity(FOCAL_PATH), "pdf_identities": proofs, "byte_differences": differences,
            "poppler": inspections, "render_png_sha256": render_hashes,
            "explanation": "original PDFs differ exclusively in XMP InstanceID; qpdf adds its own /ID during re-export and that added ID is irrelevant to original-byte comparison"}


def validate_r1(frozen):
    assert frozen["provenance"]["manifest"] == identity(HERE / "p1323-manifest.json")
    for item in frozen["protected"]:
        assert sha(item["path"]) == item["sha256"]
    obligation = R1["obligation"]()
    assert frozen["obligation"]["normative_sha256"] == obligation["normative_sha256"]
    assert frozen["obligation"]["nuclei"] == obligation["nuclei"]


def freeze():
    frozen = json.loads(R1_SUITE.read_text())
    validate_r1(frozen)
    proof = focal_proof()
    source = json.loads(FOCAL_PATH.read_text())["observations"][0]["observation"]["artifact"]["path"]
    frozen.update(kind="P1323 successor frozen suite R2", successor_provenance=R1["provenance"](HERE / "p1323-manifest.json"),
                  runner_r2=identity(__file__), predecessor_suite=identity(R1_SUITE),
                  pdf_focal=proof, pdf_calibration=pdf_calibration(source),
                  revision={"number": 2, "cause": "R1 PDF byte identity failed because baseline InstanceID is volatile",
                            "scope": "PDF comparator and retention only; fixed cases/argv/warning/non-PDF checks inherited unchanged",
                            "candidate_source_read": False, "cost": "4 PDF compile processes before freeze; 0 full reruns",
                            "limitation": "R1 temporary PDFs unavailable; fresh retained focal proves volatility in the same pinned baseline, not original discarded bytes"})
    return frozen


def run(args):
    frozen = json.loads(Path(args.suite).read_text())
    validate_r1(frozen)
    assert frozen["runner_r2"] == identity(__file__)
    assert frozen["predecessor_suite"] == identity(R1_SUITE)
    assert frozen["pdf_focal"]["receipt"] == identity(FOCAL_PATH)
    expected = base64.b64decode(frozen["warning_base64"])
    binaries = {"baseline": Path(args.baseline).resolve(), "candidate": Path(args.candidate).resolve()}
    retained = Path(tempfile.mkdtemp(prefix="ab-cli-r2-", dir=HERE / "p1323-fixtures"))
    result = {"kind": "P1323 A/B observations R2", "provenance": R1["provenance"](HERE / "p1323-manifest.json"),
              "runner_r2": identity(__file__), "suite": identity(args.suite),
              "binaries": {k: identity(v) for k, v in binaries.items()}, "retained_directory": str(retained),
              "rounds": [], "scope": frozen["scope"], "obligation": R1["obligation"](),
              "product_verdict": "reserved for independent verifier"}
    for order, fixed in (("normal", frozen["cases"]), ("repeat", frozen["cases"]),
                         ("reverse", list(reversed(frozen["cases"])))):
        rows = []
        for case in fixed:
            obs = {}
            for role, binary in binaries.items():
                output = retained / (order + "-" + role + "-" + case["id"] + "." + str(case["format"]))
                obs[role] = R1["observe"](binary, case, output)
                if case["format"] == "pdf" and obs[role]["artifact"]:
                    obs[role]["artifact"]["pdf_identity"] = pdf_identity(output.read_bytes())
            base, candidate = obs["baseline"], obs["candidate"]
            checks = {"warning": R1["classify_warning"](candidate, expected, case["warning_expected"], not case["error_expected"])}
            if any(o["status"] != "observed" for o in obs.values()):
                checks["preservation"] = "Unknown"
            else:
                base_stderr, candidate_stderr = raw(base, "stderr"), raw(candidate, "stderr")
                if case["warning_expected"]:
                    old_prefix = expected if base_stderr.startswith(expected) else R1["HEADLINE"]
                    base_stderr = base_stderr.removeprefix(old_prefix)
                    candidate_stderr = candidate_stderr.removeprefix(expected)
                equal_artifact = (base["artifact"] is None and candidate["artifact"] is None) or (
                    base["artifact"] is not None and candidate["artifact"] is not None and
                    base["artifact"]["sha256"] == candidate["artifact"]["sha256"])
                pdf_state = None
                if case["format"] == "pdf" and base["artifact"] and candidate["artifact"]:
                    pdf_state = compare_pdf(base["artifact"]["pdf_identity"], candidate["artifact"]["pdf_identity"])
                    equal_artifact = pdf_state == "Preserved"
                success = candidate["exit"] != 0 if case["error_expected"] else candidate["exit"] == 0
                artifact_ok = (candidate["artifact"] is None if case["error_expected"] else
                               (not case["format"] or (candidate["artifact"] is not None and candidate["artifact"]["size"] > 0)))
                preserved = (base["exit"] == candidate["exit"] and raw(base, "stdout") == raw(candidate, "stdout") and
                             base_stderr == candidate_stderr and equal_artifact and success and artifact_ok)
                checks["preservation"] = "Unknown" if pdf_state == "Unknown" else ("Preserved" if preserved else "Violated")
                checks["details"] = {"exit_equal": base["exit"] == candidate["exit"],
                                     "stdout_equal": raw(base, "stdout") == raw(candidate, "stdout"),
                                     "remaining_stderr_equal": base_stderr == candidate_stderr,
                                     "artifact_equal_between_crystalline_builds": equal_artifact,
                                     "pdf_comparator": pdf_state, "expected_success_or_failure": success,
                                     "artifact_presence_valid": artifact_ok}
            rows.append({"case": case, "observations": obs, "checks": checks})
        result["rounds"].append({"order": order, "rows": rows})
    def stable_view(row, role):
        obs = row["observations"][role]
        artifact = obs["artifact"]
        digest = artifact["sha256"] if artifact else None
        if artifact and row["case"]["format"] == "pdf":
            if artifact["pdf_identity"]["status"] != "Observed":
                return None
            digest = artifact["pdf_identity"]["all_other_bytes_sha256"]
        return {k: obs.get(k) for k in ("status", "exit", "stdout_base64", "stderr_base64")} | {"artifact_identity": digest}
    normal = {row["case"]["id"]: row for row in result["rounds"][0]["rows"]}
    result["order_checks"] = [{"order": r["order"], "case": row["case"]["id"], "role": role,
                               "stable": stable_view(row, role) is not None and stable_view(row, role) == stable_view(normal[row["case"]["id"]], role)}
                              for r in result["rounds"][1:] for row in r["rows"] for role in binaries]
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("phase", choices=("freeze", "run"))
    parser.add_argument("--receipt", required=True)
    parser.add_argument("--suite")
    parser.add_argument("--baseline")
    parser.add_argument("--candidate")
    args = parser.parse_args()
    if args.phase == "run" and not all((args.suite, args.baseline, args.candidate)):
        parser.error("run requires --suite, --baseline and --candidate")
    report = freeze() if args.phase == "freeze" else run(args)
    with Path(args.receipt).open("x") as output:
        json.dump(report, output, indent=2, ensure_ascii=False)
        output.write("\n")
    print(json.dumps({"receipt": identity(args.receipt), "kind": report["kind"]}))


if __name__ == "__main__":
    main()
