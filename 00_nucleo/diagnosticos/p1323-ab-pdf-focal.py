#!/usr/bin/env python3
"""PDF-only focal remeasurement. Read only R1 test artifacts and binary outputs."""
import argparse
import json
from pathlib import Path
import runpy
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
R1 = runpy.run_path(str(HERE / "p1323-ab-cli.py"))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--receipt", required=True)
    args = parser.parse_args()
    prior = json.loads((HERE / "p1323-ab-process-receipt.json").read_text())
    suite = json.loads((HERE / "p1323-ab-frozen-suite.json").read_text())
    case = next(c for c in suite["cases"] if c["id"] == "preserve-pdf")
    retained = Path(tempfile.mkdtemp(prefix="ab-pdf-focal-", dir=HERE / "p1323-fixtures"))
    report = {"kind": "P1323 R2 PDF-only focal evidence", "directory": str(retained),
              "provenance": R1["provenance"](HERE / "p1323-manifest.json"),
              "focal_runner": R1["identity"](__file__), "source_suite": R1["identity"](HERE / "p1323-ab-frozen-suite.json"),
              "prior_receipt": R1["identity"](HERE / "p1323-ab-process-receipt.json"),
              "limitation": "R1 PDFs were deleted by TemporaryDirectory; cannot identify their changing bytes retrospectively",
              "cases_frozen_before_execution": [case], "observations": []}
    for role in ("baseline", "candidate"):
        binary = prior["binaries"][role]
        assert R1["sha"](binary["path"]) == binary["sha256"]
        for iteration in (1, 2):
            pdf = retained / f"{role}-{iteration}.pdf"
            obs = R1["observe"](binary["path"], case, pdf)
            row = {"role": role, "iteration": iteration, "binary": binary, "observation": obs, "inspections": {}}
            if obs.get("exit") == 0 and pdf.exists():
                for tool, argv in (
                    ("pdfinfo", ["pdfinfo", "-box", str(pdf)]),
                    ("pdffonts", ["pdffonts", str(pdf)]),
                    ("pdfimages", ["pdfimages", "-list", str(pdf)]),
                    ("pdftotext", ["pdftotext", "-layout", str(pdf), "-"]),
                    ("qpdf", ["qpdf", "--qdf", "--object-streams=disable", str(pdf), str(pdf.with_suffix(".qdf"))]),
                    ("pdftoppm", ["pdftoppm", "-r", "72", "-png", "-singlefile", str(pdf), str(pdf.with_suffix(""))]),
                ):
                    proc = subprocess.run(argv, capture_output=True, timeout=30)
                    row["inspections"][tool] = {"argv": argv, "exit": proc.returncode,
                                                "stdout": proc.stdout.decode("utf-8", errors="replace"),
                                                "stderr": proc.stderr.decode("utf-8", errors="replace")}
                row["retained_files"] = [R1["identity"](p) for p in sorted(retained.glob(pdf.stem + ".*"))]
            report["observations"].append(row)
    with Path(args.receipt).open("x") as output:
        json.dump(report, output, indent=2, ensure_ascii=False)
        output.write("\n")
    print(json.dumps({"receipt": R1["identity"](args.receipt), "directory": str(retained)}))


if __name__ == "__main__":
    main()
