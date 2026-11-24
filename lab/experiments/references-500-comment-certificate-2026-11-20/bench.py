"""Paired watch comparison and directed tests of the syntax-based probe."""
from collections import Counter
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess

D = Path(__file__).resolve().parent
PREVIOUS = D.parent / "references-500-watch-physical-2026-11-20"
spec = importlib.util.spec_from_file_location("previous_watch_driver", PREVIOUS / "bench.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)

measure = runner.measure
describe = runner.describe
write_json = measure.write_json
command = runner.command
WORK = Path(os.environ.get("TEKT_LAB_WORK", str(D)))
EXES = runner.EXES = {"control": WORK / "control.elf", "off": WORK / "candidate.elf",
                       "retain": WORK / "candidate.elf"}
runner.D = WORK
BASE = runner.BASE
cat = lambda text: text.encode("utf-8")
STATES = runner.STATES = {
    "startup": BASE,
    "comment": BASE + cat("// nota de revisao"),
    "comment-2": BASE + cat("// anotação arbitrária, com tamanho diferente na revisão final"),
    "visible": (BASE + cat("// anotação arbitrária, com tamanho diferente na revisão final"))
        .replace(b"[Paragraph", b"[Visible paragraph", 1),
}
REGULAR = [(state, data, None) for state, data in STATES.items()]
LEGACY = [(state, data, None) for state, data in runner.prior.STATES.items()]
SPAN_BASE = cat("""// primeira nota
#set page(width:400pt,height:600pt)
#set heading(numbering: "1.")
#outline(title: none)
= Alpha <a>
See @b.
#pagebreak()
= Beta <b>
#context {
  let pos = query(<b>).first().location().position()
  /* blue */ /* az */ [Observed page #pos.page.]
}
""")
SPAN_KEEP = SPAN_BASE.replace(b"primeira nota", b"segunda nota ").replace(b"/* blue */", b"/* cyan */")
SPAN_UNICODE = SPAN_KEEP.replace(b"/* az */", cat("/* é */"))
assert len(SPAN_BASE) == len(SPAN_KEEP) == len(SPAN_UNICODE)
SPAN_SHIFT = SPAN_UNICODE.replace(b"segunda nota ", b"comentario mais longo antes dos titulos")
SPAN_MOVE = SPAN_SHIFT.replace(b"#pagebreak()", b"#pagebreak()\n#pagebreak()")
SPANS = [
    ("span-open", SPAN_BASE, None),
    ("span-kept", SPAN_KEEP, None),
    ("span-unicode", SPAN_UNICODE, None),
    ("span-shifted", SPAN_SHIFT, None),
    ("span-position-moved", SPAN_MOVE, None),
    ("span-eof-insert", SPAN_MOVE + b"// another independent input", None),
    ("span-eof-remove", SPAN_MOVE, None),
]
DEPENDENCY = [
    ("dep-start", runner.INCLUDE + STATES["startup"], runner.LEAF_FIRST),
    ("dep-comment", runner.INCLUDE + STATES["comment"], runner.LEAF_FIRST),
    ("dep-leaf-edit", runner.INCLUDE + STATES["comment"], runner.LEAF_EDIT),
    ("dep-leaf-restore", runner.INCLUDE + STATES["comment"], runner.LEAF_FIRST),
]
RAW_BASE = b"#set page(width:400pt,height:600pt)\n#raw(read(\"main.typ\"))\n"
RAW = [("raw-open", RAW_BASE, None), ("raw-comment", RAW_BASE + b"// raw bytes changed", None)]
CASE_STATES = {"regular": REGULAR, "legacy": LEGACY, "spans": SPANS, "dependency": DEPENDENCY, "raw": RAW}


def status_watch(log, expected):
    content = log.read_text()
    assert content.count("compiled successfully") == expected, (log, content)


def watch_events(log):
    events = []
    current = None
    for line in log.read_text().splitlines():
        if "compiling ..." in line:
            current = []
            events.append(current)
        elif line.startswith(("watch-retain", "watch-memo")):
            current.append(line)
    return events


def pdf_text(path):
    return subprocess.check_output(["pdftotext", str(path), "-"], text=True)


def validate(runs, nominal):
    checks = []
    fingerprints = {}
    described = {}
    targeted_arms = ("control", "off", "retain")
    expected_hits = {
        "regular": {"comment", "comment-2"},
        "spans": {"span-kept", "span-eof-insert", "span-eof-remove"},
        "legacy": set(), "dependency": set(), "raw": set(),
    }

    def inspect(path):
        digest = measure.sha(path)
        if digest not in described:
            described[digest] = describe(path)
        return described[digest]

    for case_name, states in CASE_STATES.items():
        for arm in targeted_arms:
            if arm == "off" and case_name != "regular":
                continue
            case = runs / "validation" / case_name / arm
            rows = runner.run_watch(case, arm, states, trace=arm != "control")
            status_watch(case / "watch.log", len(states))
            runner.run_fresh(case, arm, states, trace=arm != "control")
            if arm == "retain":
                events = watch_events(case / "watch.log")
                assert len(events) == len(states), events
            if arm != "control":
                observed_positions = runner.positions(case / "watch.log")
                assert len(observed_positions) == len(states)
            for index, row in enumerate(rows):
                state = row["state"]
                path = WORK / row["path"]
                fresh_path = case / (state + "-fresh.pdf")
                actual, fresh = inspect(path), inspect(fresh_path)
                key = case_name, state
                if arm == "control":
                    fingerprints[key] = fresh
                assert actual == fresh == fingerprints[key], (case_name, arm, state)
                record = dict(case=case_name, arm=arm, state=state, checks=list(actual))
                if arm != "control":
                    position, = runner.positions(case / (state + "-fresh.log"))
                    assert observed_positions[index] == position, (case_name, arm, state)
                    record["positions"] = position
                if arm == "retain":
                    case_events = events[index]
                    hits = [event for event in case_events if event.startswith("watch-retain hit")]
                    assert bool(hits) == (state in expected_hits[case_name]), (state, case_events)
                    if hits:
                        assert len(hits) == 1 and "spans=true" in hits[0], (state, case_events)
                        assert not any(event.startswith("watch-memo") for event in case_events)
                    if case_name in ("dependency", "raw"):
                        assert any("watch-retain miss sealed=false" in event for event in case_events), (state, case_events)
                    if state in ("span-unicode", "span-shifted"):
                        assert any("watch-retain source-miss spans=false" in event for event in case_events), (state, case_events)
                    record["events"] = case_events
                    if case_name == "regular":
                        navigation = runner.prior.navigation(path, actual, state == "visible")
                        wrong = navigation["wrong_pages"]
                        assert len(wrong["toc_links"]) == len(wrong["bookmarks"]) == 19
                        assert not wrong["references"] and not wrong["toc_printed"]
                        record["navigation"] = navigation
                    if case_name == "dependency":
                        target = actual["named_destinations"]["u:external"]
                        assert target[0][1] == (2 if state == "dep-leaf-edit" else 1), (state, target)
                        assert ("External omega." in pdf_text(path)) == (state == "dep-leaf-edit")
                        record["dependency_destination"] = target
                    if case_name == "spans":
                        expected = 3 if state in ("span-position-moved", "span-eof-insert", "span-eof-remove") else 2
                        rendered = pdf_text(path)
                        assert f"Observed page {expected}." in rendered, (state, rendered)
                        position = actual["named_destinations"]["u:b"]
                        assert position[0][1] == expected, (state, position)
                        record["observed_heading_position"] = position
                    if case_name == "raw":
                        assert ("raw bytes changed" in pdf_text(path)) == (state == "raw-comment")
                checks.append(record)

    for row in nominal:
        assert inspect(WORK / row["path"]) == fingerprints["regular", row["state"]], row
    return dict(utc=measure.utc(), nominal_verified=len(nominal), checks=checks)


def run():
    os.sched_setaffinity(0, {2})
    blocks = int(os.environ.get("BLOCKS", "12"))
    assert (D / "control-before.sha256").read_bytes() == (D / "control-input-manifest.sha256").read_bytes()
    hashes = runner.variants()
    for arm, path in (("control", "control"), ("retain", "candidate")):
        assert hashes[arm] == (D / (path + ".sha256")).read_text().split()[0]
    runs = WORK / "runs"
    runs.mkdir()
    provenance = {name: measure.sha(D / name) for name in (
        "control-diffstat.txt", "control-input-manifest.sha256", "candidate-input-manifest.sha256",
        "prototype.py", "watch_probe.inc.rs", "build-control.sh", "build-candidate.sh", "bench.py", "run-in-ram.sh")}
    provenance["previous_bench.py"] = measure.sha(PREVIOUS / "bench.py")
    write_json(D / "receipt.json", dict(utc=measure.utc(), head=(D / "head.txt").read_text().strip(),
        elves=hashes, tmp_work=str(WORK), blocks=blocks, cpus=dict(child=1, controller=2), provenance=provenance,
        inputs={kind: {state: dict(main=hashlib.sha256(main).hexdigest(),
                                  middle=hashlib.sha256(runner.MIDDLE).hexdigest() if leaf else None,
                                  leaf=hashlib.sha256(leaf).hexdigest() if leaf else None)
                       for state, main, leaf in states}
                for kind, states in CASE_STATES.items()}))
    orders = (("control", "off", "retain"), ("off", "retain", "control"), ("retain", "control", "off"),
              ("retain", "off", "control"), ("control", "retain", "off"), ("off", "control", "retain"))
    rows = []
    for block in range(blocks):
        for arm in orders[block % len(orders)]:
            case = runs / str(block) / arm
            rows.extend(dict(block=block, **r) for r in runner.run_watch(case, arm, REGULAR))
            status_watch(case / "watch.log", len(REGULAR))
        print("block", block, flush=True)
    with gzip.open(D / "nominal.json.gz", "wt") as out:
        json.dump(rows, out, indent=2)
    write_json(D / "timing.json", runner.timing(rows, blocks))
    write_json(D / "verification.json", validate(runs, rows))
    assert hashes == runner.variants()


if __name__ == "__main__":
    run()
