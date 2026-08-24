#!/usr/bin/env python3
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
PROBES = json.loads((pathlib.Path(__file__).with_name("probes.json")).read_text())
BINARIES = {
    "vanilla": ROOT / "lab/typst-original/target/release/typst",
    "crystalline": ROOT / "target/release/typst",
}

results = []
for probe in PROBES:
    row = dict(probe)
    expression = f'repr(type({probe["path"]}))'
    row["expression"] = expression
    for side, binary in BINARIES.items():
        command = [str(binary), "eval", expression, "--format", "json"]
        if side == "vanilla" and probe["path"] == "html":
            command.extend(["--features", "html"])
        run = subprocess.run(
            command,
            cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        )
        row[side] = {
            "exit_code": run.returncode,
            "stdout": run.stdout,
            "stderr": run.stderr,
        }
    row["same"] = row["vanilla"] == row["crystalline"]
    results.append(row)

output = ROOT / "00_nucleo/diagnosticos/superficie-linguagem-p1140-probes.json"
output.write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"total": len(results), "same": sum(r["same"] for r in results)}))
