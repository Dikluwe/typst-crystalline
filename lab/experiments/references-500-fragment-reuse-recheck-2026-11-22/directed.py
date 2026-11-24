"""Compare guarded suffix reuse with a fresh full compile after directed edits."""
import json
import os
from pathlib import Path
import subprocess
import sys

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / "references-500-visible-eval-residual-2026-11-21"))
from watch import EXES, STATES, WORK, cycle, run

sys.path.insert(0, str(here.parent / "references-500-visible-stable-cids-2026-11-21"))
from pdf_check import render, structure, text_layout

EXES["candidate"] = WORK / "candidate"
opts = {"TEKT_LAB_MEMO_PROTECT": "1", "TEKT_LAB_MEMO_BYTES": str(8 * 1024 * 1024)}
setup = cycle.base.runner.setup_sources
describe = cycle.cycle.original.compare.describe
source = STATES[-1][1]


def edit(before, after):
    assert before in source
    return source.replace(before, after, 1)


prefix, separator, suffix = source.rpartition(b"Paragraph")
assert separator

cases = {
    "height": [*STATES[:3], ("changed", edit(b"height:600pt", b"height:440pt"), None)],
    "outline": [*STATES[:3], ("changed", edit(b"#outline()", b"#outline(title: [Probe contents])"), None)],
    "reference": [*STATES[:3], ("changed", edit(b"See @s19.", b"See @s13."), None)],
    "suffix-text": [*STATES[:3], ("changed", prefix + b"Suffix paragraph" + suffix, None)],
}
included = source + b'\n#include "middle.typ"\n'
first = b'= Original included suffix\nSuffix dependency.\n'
changed = b'= Changed included suffix\nUpdated dependency.\n'
cases["suffix-dependency"] = [("open", included, first), ("changed", included, changed),
                               ("restored", included, first)]

results = {}
for name, states in cases.items():
    if name == "suffix-dependency":
        # Use separate sessions so the final watch PDF is available for both transitions.
        sequences = [states[:2], states]
    else:
        sequences = [states]
    readings = []
    for n, sequence in enumerate(sequences):
        publication, data, leaf = sequence[-1]
        row = run(n, "candidate", 0, states=sequence, check=False, overrides=opts,
                  directory=f"semantic-{name}")
        hot = WORK / row["directory"] / "out.pdf"
        cold = WORK / row["directory"] / "cold"
        cold.mkdir()
        setup(cold, data, leaf)
        env = cycle.environment(cold, "candidate", trace=False)
        env.update(opts)
        with (cold / "compile.log").open("wb") as log:
            subprocess.run(["taskset", "-c", "1", str(WORK / "control"), "compile", "main.typ",
                            "out.pdf", "--root", str(cold), "--pdf-tagged=true"], cwd=cold,
                           env=env, stdout=log, stderr=subprocess.STDOUT, check=True, timeout=60)
        fresh = cold / "out.pdf"
        qdf_equal = cycle.cycle.key("candidate", hot) == cycle.cycle.key("candidate", fresh)
        doc = describe(hot)
        assert doc == describe(fresh), (name, publication, "PDF description")
        assert text_layout(hot) == text_layout(fresh), (name, publication, "text geometry")
        assert structure(hot) == structure(fresh), (name, publication, "PDF structure")
        rasters = render(hot, cold, "hot")
        assert rasters == render(fresh, cold, "fresh"), (name, publication, "raster")
        assert row["all_pdfs"][publication] != row["all_pdfs"][sequence[0][0]] or publication == "restored", name
        readings.append({"publication": publication, "qdf_equal": qdf_equal,
                         "semantic_equal": True, "pages": doc["page_count"],
                         "links": sum(map(len, doc["links"])), "rasters": rasters, "watch": row})
        print(name, publication, "ok", flush=True)
    results[name] = readings
(Path(os.environ["TEKT_LAB_TIMING_OUT"]) / "directed.json").write_text(json.dumps(results, indent=2) + "\n")
