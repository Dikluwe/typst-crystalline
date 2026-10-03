"""A/B geometry and timing for Auto text width versus pinned vanilla."""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
OUT = Path("/tmp/tekt-grid-auto-space")
OUT.mkdir(exist_ok=True)
BIN = {
    "before": Path("/tmp/tekt-before-auto-space"),
    "after": ROOT / "target/release/typst",
    "vanilla": ROOT / "lab/typst-original/target/release/typst",
}
ENV = dict(os.environ, SOURCE_DATE_EPOCH="0", RAYON_NUM_THREADS="1")
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def source(rows, mode):
    if mode == "multi":
        cells = "[AB CD], [EF GH], [IJ KL], [MN OP]"
    else:
        cells = ",\n".join(f"[C{r:02d}{c}]" for r in range(rows) for c in range(4))
    columns = "(100pt, 100pt, 100pt, 100pt)" if mode == "fixed" else "4"
    path = OUT / f"{mode}-{rows}.typ"
    path.write_text(
        '#set page(width: 500pt, height: 3500pt, margin: 30pt)\n'
        '#set text(font: "Libertinus Serif", size: 10pt)\n'
        f'#table(columns: {columns}, rows: {rows},\n{cells}\n)\n'
    )
    return path


def run(src, engine, suffix):
    pdf = OUT / f"{src.stem}-{engine}-{suffix}.pdf"
    cmd = [str(BIN[engine]), "compile", str(src), str(pdf), "--root", str(OUT),
           "--pdf-tagged=false", "--pretty=false"]
    if engine == "vanilla":
        cmd += ["--jobs", "1"]
    start = time.perf_counter_ns()
    proc = subprocess.run(cmd, cwd=ROOT, env=ENV, capture_output=True,
                          preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
    elapsed = (time.perf_counter_ns() - start) / 1e6
    if proc.returncode:
        raise RuntimeError(f"{cmd}: {proc.stderr.decode(errors='replace')}")
    return pdf, elapsed


def words(pdf):
    xml = subprocess.check_output(["pdftotext", "-bbox", str(pdf), "-"])
    tree = ET.fromstring(xml)
    return {e.text: [round(float(e.attrib[axis]), 4) for axis in ("xMin", "yMin")]
            for e in tree.iter() if e.tag.endswith("word") and e.text}


def main():
    fixtures = {
        "auto1": source(1, "auto"),
        "auto80": source(80, "auto"),
        "fixed80": source(80, "fixed"),
        "multi1": source(1, "multi"),
    }
    geometry = {}
    for name, src in fixtures.items():
        positions = {}
        for engine in BIN:
            pdf, _ = run(src, engine, "geometry")
            positions[engine] = words(pdf)
        common = set.intersection(*(set(p) for p in positions.values()))
        geometry[name] = {
            "word_counts": {k: len(v) for k, v in positions.items()},
            "common_words": len(common),
            "x_mismatch_gt_0_001": {
                "before": sum(abs(positions["before"][w][0] - positions["vanilla"][w][0]) > 0.001 for w in common),
                "after": sum(abs(positions["after"][w][0] - positions["vanilla"][w][0]) > 0.001 for w in common),
            },
            "max_abs_x_delta": {
                k: max((abs(positions[k][w][0] - positions["vanilla"][w][0]) for w in common), default=0)
                for k in ("before", "after")
            },
            "sample_x": {w: {k: positions[k].get(w) for k in BIN} for w in sorted(common)[:16]},
        }
    samples = {k: [] for k in BIN}
    src = fixtures["auto80"]
    for _ in range(2):
        for engine in BIN:
            run(src, engine, "warm")
    for i in range(12):
        order = list(BIN)
        if i % 2:
            order.reverse()
        for engine in order:
            _, elapsed = run(src, engine, f"time{i}")
            samples[engine].append(elapsed)
    result = {
        "utc": time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime()),
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "status": subprocess.check_output(["git", "status", "--short"], cwd=ROOT, text=True),
        "diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=ROOT, text=True),
        "cpu": CPU,
        "binary_sha256": {k: sha(v) for k, v in BIN.items()},
        "fixture_sha256": {k: sha(v) for k, v in fixtures.items()},
        "geometry": geometry,
        "timing_ms": {k: {"median": statistics.median(v), "samples": v} for k, v in samples.items()},
    }
    (OUT / "results.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"geometry": geometry, "median_ms": {k: v["median"] for k, v in result["timing_ms"].items()}}, indent=2))


if __name__ == "__main__":
    main()
