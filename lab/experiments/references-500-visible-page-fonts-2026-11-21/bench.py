"""Compare page font scans against the existing operator probe and pinned Typst."""
import importlib.util
import json
import re
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
prior = D.parent / "references-500-visible-page-operators-2026-11-21" / "bench.py"
spec = importlib.util.spec_from_file_location("operator_bench", prior)
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)

FONTS = re.compile(r"^pdf-page-fonts pages=(\d+) recomputed=(\[.*?\])$", re.M)
environment = bench.environment
check_operators = bench.check_operators
directed = bench.directed


def candidate_environment(tmp, arm, trace):
    env = environment(tmp, arm, trace)
    if trace and arm == "candidate":
        env["TEKT_LAB_FONT_SCAN_CHECK"] = "1"
    return env


def compare_operators(case):
    check_operators(case)
    for arm in ("hybrid", "candidate"):
        log = (case / arm / "watch.log").read_text()
        data = bench.OPERATORS.findall(log)
        assert len(data) == 4 and data[-1] == ("21", "21", "[1, 2]"), (arm, data)


def font_counts():
    directed()
    evidence = {}
    for case in sorted((bench.WORK/"directed").iterdir()):
        log = (case/"candidate"/"watch.log").read_text()
        publications = []
        for part in log.split("compiled successfully")[:-1]:
            records = FONTS.findall(part)
            assert len(records) <= 1, (case, records)
            if records:
                pages, recomputed = records[0]
                pages, recomputed = int(pages), json.loads(recomputed)
                publications.append(dict(pages=pages, reused=pages-len(recomputed), recomputed=recomputed))
            else:
                publications.append(None)
        evidence[case.name] = publications
    start, comment, comment_2, visible = evidence["original"]
    assert start == dict(pages=23, reused=0, recomputed=list(range(1, 24)))
    assert comment is None and comment_2 is None
    assert visible == dict(pages=23, reused=21, recomputed=[1,2]), visible
    assert evidence["font-change"][-1] is None or evidence["font-change"][-1]["reused"] == 0
    assert evidence["resource"][-1] is None
    bench.cycle.dump("fonts.json", evidence)
    print("font scan counts", visible, flush=True)

    for arm in bench.ARMS:
        bench.base.EXES[arm] = bench.EXES[arm]
        dest = bench.WORK / "fresh-verification" / arm
        bench.base.fresh(dest, arm, bench.STATES, True)
        for state, _, _ in bench.STATES:
            assert bench.cycle.key(arm, dest / f"{state}.pdf") == bench.gold(arm, state)
    print("fresh compilation: all four states across all three variants", flush=True)
    validate_render()


def validate_render():
    directory = bench.WORK / "render-validation"
    directory.mkdir()
    watch = bench.WORK / "directed" / "original"
    cases = {
        "candidate-watch": watch / "candidate" / "visible.pdf",
        "control-watch": watch / "hybrid" / "visible.pdf",
        "candidate-fresh": bench.WORK / "fresh-verification" / "candidate" / "visible.pdf",
    }
    rendered = {}
    for name, pdf in cases.items():
        subprocess.run(["qpdf", "--check", str(pdf)], check=True, capture_output=True)
        subprocess.run(["pdftoppm", "-gray", "-r", "72", str(pdf), str(directory/name)], check=True)
        images = sorted(directory.glob(name + "-*.pgm"))
        assert len(images) == 23, (name, images)
        text = subprocess.check_output(["pdftotext", "-layout", str(pdf), "-"])
        rendered[name] = dict(pages=[bench.cycle.measure.sha(image) for image in images],
                              text_sha256=bench.cycle.hashlib.sha256(text).hexdigest())
        assert " ".join(text.decode().split())
        assert rendered[name] == rendered["candidate-watch"], name
    bench.cycle.dump("render-validation.json", rendered)
    print("visible PDF: 23 identical renders plus extracted text", flush=True)


if __name__ == "__main__":
    bench.cycle.environment = candidate_environment
    bench.check_operators = compare_operators
    bench.directed = font_counts
    bench.main()
