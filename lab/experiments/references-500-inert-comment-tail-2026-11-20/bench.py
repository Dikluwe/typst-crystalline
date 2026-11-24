"""Paired watch comparison and directed tests of the syntax-based probe."""
from itertools import permutations
import gzip
import hashlib
import importlib.util
import json
import os
import re
from pathlib import Path
from random import Random
from statistics import median
import subprocess

D = Path(__file__).resolve().parent
PREVIOUS = D.parent / "references-500-watch-physical-2026-11-20"
spec = importlib.util.spec_from_file_location("previous_watch_driver", PREVIOUS / "bench.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)

measure = runner.measure
describe = runner.describe
write_json = measure.write_json
original_command = runner.command
WORK = Path(os.environ.get("TEKT_LAB_WORK", str(D)))
EXES = runner.EXES = {"control": WORK / "control.elf", "inert": WORK / "candidate.elf",
                       "lazy": WORK / "candidate.elf"}


def command(case, arm, mode, dest, trace):
    args, env = original_command(case, arm, mode, dest, trace)
    if arm in ("inert", "lazy"):
        env["TEKT_LAB_WATCH_RETAIN"] = "1"
        env["TEKT_LAB_WATCH_LAZY"] = "1"
    if arm == "inert":
        env["TEKT_LAB_WATCH_INERT"] = "1"
    return args, env


runner.command = command
runner.D = WORK
BASE = runner.BASE
cat = lambda text: text.encode("utf-8")
STATES = runner.STATES = dict(runner.prior.STATES)
REGULAR = [(state, data, None) for state, data in STATES.items()]
STRICT = [
    ("strict-open", BASE, None),
    ("strict-add", BASE + b"// note without extra newline", None),
    ("strict-edit", BASE + cat("// outra anotação na cauda"), None),
]
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
WHOLE_BASE = b"#show: it => [#repr(it)]\nBody\n"
WHOLE = [
    ("whole-open", WHOLE_BASE, None),
    ("whole-added", WHOLE_BASE + b"\n// first arbitrary note\n", None),
    ("whole-edited", WHOLE_BASE + b"\n// long note with a different width\n", None),
    ("whole-removed", WHOLE_BASE, None),
]
CONDITIONAL_BASE = (
    b'#show: it => if repr(it).contains("parbreak") { read("leaf.typ") } else { [Initial body.] }\n'
    b'Body\n'
)
CONDITIONAL_ALT = CONDITIONAL_BASE + b"\n// Activate the alternate body.\n"
CONDITIONAL = [
    ("conditional-open", CONDITIONAL_BASE, b"External alpha."),
    ("conditional-comment", CONDITIONAL_ALT, b"External alpha."),
    ("conditional-leaf", CONDITIONAL_ALT, b"External omega."),
]
CASE_STATES = {"regular": REGULAR, "strict": STRICT, "spans": SPANS, "dependency": DEPENDENCY,
               "raw": RAW, "whole": WHOLE, "conditional": CONDITIONAL}


def family_document(width, height, count, binding, prose, close):
    return cat(f"""#set page(width:{width}pt,height:{height}pt)
#set heading(numbering:"1.")
#outline(title: none)
= Unlike the benchmark <p>
See @p.
#for {binding} in range({count}) [{prose}{close}]
""")


def tail_edits(base):
    return [("family-open", base, None),
            ("family-comment", base + b"\n// separate comment\n", None),
            ("family-comment-2", base + cat("\n// comentário com novo tamanho\n"), None)]


for index, (width, height, count, binding, prose) in enumerate((
    (300, 300, 1, "idx", "One compact paragraph #idx."),
    (225, 180, 8, "n", "A narrow page breaks this paragraph apart across more than one line #n."),
    (560, 430, 13, "step", "A wider document keeps this paragraph on one line."),
    (340, 200, 44, "k", "Iteration #k ends in an ordinary paragraph."),
    (430, 260, 5, "x", "#x is an inline integer at the beginning of a paragraph."),
    (750, 100, 12, "i", "Short height crosses pages quickly at step #i."),
)):
    CASE_STATES[f"family-{index}"] = tail_edits(family_document(width,height,count,binding,prose,"\n\n"))
CASE_STATES["unclosed"] = tail_edits(family_document(400, 600, 1, "item", "Open paragraph #item.", ""))


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
    lazy_hits = {
        "regular": {"comment-2"},
        "spans": {"span-kept", "span-eof-insert", "span-eof-remove"},
        "strict": {"strict-add", "strict-edit"}, "dependency": set(), "raw": set(),
        "whole": {"whole-edited"}, "conditional": set(), "unclosed": {"family-comment-2"},
    }
    for case in CASE_STATES:
        if case.startswith("family-"):
            lazy_hits[case] = {"family-comment-2"}
    inert_hits = dict(lazy_hits, regular={"comment","comment-2"})
    for case in CASE_STATES:
        if case.startswith("family-"):
            inert_hits[case] = {"family-comment","family-comment-2"}
    complete_pdf = {}

    def pdf_structure(path):
        raw = subprocess.check_output(["qpdf", "--qdf", "--object-streams=disable", str(path), "-"],
                                      timeout=30, stderr=subprocess.PIPE)
        # Only publication identifiers vary even with a fixed compilation date.
        raw, xmp = re.subn(rb"(<xmpMM:(?:Instance|Document)ID>)[A-Za-z0-9+/=]+(</xmpMM:(?:Instance|Document)ID>)",
                          lambda m: m[1] + b"x" * (len(m[0]) - len(m[1]) - len(m[2])) + m[2], raw)
        raw, trailer = re.subn(rb"/ID \[<[0-9a-f]+><[0-9a-f]+>\]", b"/ID []", raw)
        assert (xmp,trailer) == (2,1), (xmp,trailer,path)
        return hashlib.sha256(raw).hexdigest()

    def inspect(path):
        digest = measure.sha(path)
        if digest not in described:
            described[digest] = describe(path)
        return described[digest]

    for case_name, states in CASE_STATES.items():
        for arm in EXES:
            case = runs / "validation" / case_name / arm
            rows = runner.run_watch(case, arm, states, trace=arm != "control")
            status_watch(case / "watch.log", len(states))
            runner.run_fresh(case, arm, states, trace=arm != "control")
            if arm != "control":
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
                if arm != "control":
                    case_events = events[index]
                    hits = [event for event in case_events if event.startswith("watch-retain hit")]
                    expected = lazy_hits if arm == "lazy" else inert_hits
                    assert bool(hits) == (state in expected[case_name]), (arm, state, case_events)
                    assert not any("alternative-" in e or "hit alternative" in e for e in case_events)
                    positive = case_name == "regular" or case_name.startswith("family-")
                    if arm == "inert" and positive and index == 0:
                        assert any("inert-ready=true" in e for e in case_events), (case_name, case_events)
                    if arm == "inert" and positive and index in (1,2):
                        assert any("watch-retain hit inert" in e for e in case_events), (case_name, case_events)
                    if arm == "inert" and case_name in ("whole","unclosed") and index == 0:
                        assert any("inert-ready=false" in e for e in case_events), (case_name,case_events)
                    if arm == "lazy" and case_name == "regular" and state == "comment":
                        assert any(e.startswith("watch-retain miss sealed=true") for e in case_events)
                        assert any(e.startswith("watch-memo") for e in case_events)
                    if hits:
                        assert len(hits) == 1 and "spans=true" in hits[0], (state, case_events)
                        assert not any(event.startswith("watch-memo") for event in case_events)
                    if case_name in ("dependency", "raw"):
                        assert any("watch-retain miss sealed=false" in event for event in case_events), (state, case_events)
                    if state in ("span-unicode", "span-shifted"):
                        assert any("watch-retain source-miss spans=false" in event for event in case_events), (state, case_events)
                    record["events"] = case_events
                    if case_name == "whole":
                        assert ("parbreak()" in pdf_text(path)) == (state in ("whole-added", "whole-edited"))
                    if case_name == "conditional":
                        expected_text = {
                            "conditional-open": "Initial body.",
                            "conditional-comment": "External alpha.",
                            "conditional-leaf": "External omega.",
                        }[state]
                        assert expected_text in pdf_text(path), (state, pdf_text(path))
                        if state != "conditional-open":
                            assert any("watch-retain miss sealed=false" in e for e in case_events), case_events
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
                if case_name == "regular" or case_name.startswith("family-"):
                    structure = pdf_structure(path)
                    fresh_structure = pdf_structure(fresh_path)
                    if arm == "control":
                        complete_pdf[key] = fresh_structure
                    assert structure == fresh_structure == complete_pdf[key], (case_name, state, arm)
                    record["pdf_qdf_sha256"] = structure
                checks.append(record)

    for row in nominal:
        assert inspect(WORK / row["path"]) == fingerprints["regular", row["state"]], row
    return dict(utc=measure.utc(), nominal_verified=len(nominal), checks=checks)


def summarize(rows, blocks):
    def stats(by_arm):
        result = dict(median_ms={arm: median(values) / 1e6 for arm, values in by_arm.items()})
        for arm, baseline in (("inert", "control"), ("lazy", "control"), ("inert", "lazy")):
            ratios = [a / b for a, b in zip(by_arm[arm], by_arm[baseline])]
            rng = Random(500)
            sampled = sorted(median(rng.choices(ratios, k=blocks)) for _ in range(10_000))
            result[arm + "_over_" + baseline] = dict(median=median(ratios),
                                                      ci95=[sampled[250], sampled[9749]])
        return result

    index = {(row["arm"], row["block"], row["state"]): row for row in rows}
    assert len(index) == len(EXES) * blocks * len(STATES)
    by_state = {
        state: {
            metric: stats({arm: [index[arm, b, state][metric] for b in range(blocks)]
                           for arm in EXES})
            for metric in ("cpu_ns", "wall_ns")
        } for state in STATES
    }
    prefixes = {"through_first": list(STATES)[:2], "through_second": list(STATES)[:3],
                "all_four": list(STATES)}
    session = {
        name: stats({arm: [sum(index[arm, b, state]["cpu_ns"] for state in states)
                           for b in range(blocks)] for arm in EXES})
        for name, states in prefixes.items()
    }
    memory = {arm: max(row["hwm_bytes"] for row in rows if row["arm"] == arm) / 2**20
              for arm in EXES}
    return dict(by_state=by_state, session_cpu=session, max_hwm_mib=memory)


def run():
    os.sched_setaffinity(0, {2})
    blocks = int(os.environ.get("BLOCKS", "12"))
    assert (D / "control-before.sha256").read_bytes() == (D / "control-input-manifest.sha256").read_bytes()
    hashes = runner.variants()
    for arm, path in (("control", "control"), ("inert", "candidate"), ("lazy", "candidate")):
        assert hashes[arm] == (D / (path + ".sha256")).read_text().split()[0]
    runs = WORK / "runs"
    runs.mkdir()
    provenance = {name: measure.sha(D / name) for name in (
        "control-diffstat.txt", "control-input-manifest.sha256", "candidate-input-manifest.sha256",
        "prototype.py", "watch_probe.inc.rs", "control-original-build.log", "build-candidate.sh", "bench.py", "run-in-ram.sh")}
    provenance["previous_bench.py"] = measure.sha(PREVIOUS / "bench.py")
    previous_receipt = json.loads((D.parent / "references-500-comment-tail-lazy-2026-11-20" / "receipt.json").read_text())
    assert hashes["control"] == previous_receipt["elves"]["control"]
    assert {state: hashlib.sha256(data).hexdigest() for state, data, _ in REGULAR} == {
        state: value["main"] for state, value in previous_receipt["inputs"]["regular"].items()
    }
    write_json(D / "receipt.json", dict(utc=measure.utc(), head=(D / "head.txt").read_text().strip(),
        elves=hashes, tmp_work=str(WORK), blocks=blocks, cpus=dict(child=1, controller=2), provenance=provenance,
        inputs={kind: {state: dict(main=hashlib.sha256(main).hexdigest(),
                                  middle=hashlib.sha256(runner.MIDDLE).hexdigest() if leaf else None,
                                  leaf=hashlib.sha256(leaf).hexdigest() if leaf else None)
                       for state, main, leaf in states}
                for kind, states in CASE_STATES.items()}))
    orders = tuple(permutations(EXES))
    rows = []
    for block in range(blocks):
        for arm in orders[block % len(orders)]:
            case = runs / str(block) / arm
            rows.extend(dict(block=block, **r) for r in runner.run_watch(case, arm, REGULAR))
            status_watch(case / "watch.log", len(REGULAR))
        print("block", block, flush=True)
    with gzip.open(D / "nominal.json.gz", "wt") as out:
        json.dump(rows, out, indent=2)
    write_json(D / "timing.json", summarize(rows, blocks))
    write_json(D / "verification.json", validate(runs, rows))
    assert hashes == runner.variants()


if __name__ == "__main__":
    run()
