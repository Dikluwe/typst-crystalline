"""Paired watch experiment; direct CPU scopes and tagged/untagged ablations."""
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
from random import Random
from statistics import median
import subprocess
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
ROOT = D.parents[2]
OLD = D.parent / "references-500-inert-comment-tail-2026-11-20"
spec = importlib.util.spec_from_file_location("known_watch_cases", OLD / "bench.py")
previous = importlib.util.module_from_spec(spec)
spec.loader.exec_module(previous)
runner = previous.runner
driver = runner.driver
measure = runner.measure
STATES = previous.REGULAR[:3]
ENGINES = {"tekt": WORK / "tekt", "probe": WORK / "probe", "vanilla": WORK / "vanilla"}
ARMS = tuple((engine, tagged) for engine in ENGINES for tagged in (True, False))
STAGES = ("font_discover", "font_resolve_bind", "font_partition", "font_scan", "font_subset",
          "page_operators", "page_compression", "resource_streams", "tag_mcids", "tag_tree",
          "serialize")
CPU_LINE = re.compile(r"^pdf-cpu (\d+) (\[[^\n]+?\]) (\[[^\n]+?\])$", re.M)


def run_watch(case, engine, tagged, trace=False):
    case.mkdir(parents=True)
    tmp, source, pdf = case / "tmp", case / "main.typ", case / "out.pdf"
    tmp.mkdir()
    runner.write_source(source, STATES[0][1])
    args = [str(ENGINES[engine]), "watch", str(source), str(pdf), "--root", str(case),
            f"--pdf-tagged={str(tagged).lower()}"]
    env = driver.environment(tmp)
    if engine != "vanilla":
        env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1", TEKT_LAB_WATCH_INERT="1")
        if trace:
            env["TEKT_LAB_WATCH_TRACE"] = "1"
    if engine == "probe":
        env["TEKT_LAB_PDF_CPU"] = "1"
    log_path = case / "watch.log"
    rows = []
    with log_path.open("wb") as log:
        start = time.perf_counter_ns()
        child = driver.Child(args, case, env, 1, log)
        try:
            previous_pdf = None
            for cycle, (state, data, _) in enumerate(STATES):
                before = 0
                if cycle:
                    before = child.cpu()
                    start = time.perf_counter_ns()
                    runner.write_source(source, data)
                previous_pdf, wall, publish_cpu = driver.await_pdf(child, pdf, previous_pdf)
                after = measure.settle(child)
                output = case / f"{state}.pdf"
                output.write_bytes(pdf.read_bytes())
                row = dict(engine=engine, tagged=tagged, state=state,
                           utc=measure.utc(), cpu_ns=after-before, wall_ns=wall-start,
                           publish_cpu_ns=publish_cpu-before, path=str(output.relative_to(WORK)))
                rows.append(row)
        finally:
            child.stop()
            child.close()
    content = log_path.read_text()
    if engine == "probe":
        lines = CPU_LINE.findall(content)
        assert len(lines) == len(rows), (case, content[-1500:])
        for row, (wall_cpu, raw_stages, raw_counts) in zip(rows, lines):
            times, counts = json.loads(raw_stages), json.loads(raw_counts)
            assert len(times) == len(counts) == len(STAGES)
            assert sum(times) <= int(wall_cpu)
            stage_cpu = dict(zip(STAGES, times))
            observed_calls = dict(zip(STAGES, counts))
            assert observed_calls["page_operators"] == observed_calls["page_compression"] == 23
            assert observed_calls["font_partition"] == observed_calls["tag_tree"] == 1
            assert observed_calls["tag_mcids"] == 23 and observed_calls["font_scan"] > 1
            row.update(export_thread_cpu_ns=int(wall_cpu), stage_cpu_ns=stage_cpu,
                       stage_calls=observed_calls)
    if trace:
        inert_hits = [line for line in content.splitlines() if line.startswith("watch-retain hit inert")]
        assert len(inert_hits) == 2, content[-2500:]
    return rows


def fresh(case, engine, tagged, name, data):
    case.mkdir(parents=True, exist_ok=True)
    source = case / "main.typ"
    runner.write_source(source, data)
    pdf = case / f"{name}.pdf"
    command = [str(ENGINES[engine]), "compile", str(source), str(pdf), "--root", str(case),
               f"--pdf-tagged={str(tagged).lower()}"]
    with (case / f"{name}.log").open("wb") as log:
        child = driver.Child(command, case, driver.environment(case), 1, log)
        try:
            child.reap()
            assert child.process.returncode == 0
        finally:
            child.close()
    return pdf


def bootstrap(values):
    rng = Random(500)
    samples = sorted(median(rng.choices(values, k=len(values))) for _ in range(10000))
    return dict(median=median(values), ci95=[samples[250], samples[9749]])


def bucket(row):
    t = row["stage_cpu_ns"]
    scans = t["font_discover"] + t["font_partition"] + t["font_scan"]
    streams = t["page_operators"] + t["page_compression"] + t["resource_streams"]
    explicit_tags = t["tag_mcids"] + t["tag_tree"]
    total = row["export_thread_cpu_ns"]
    return dict(**{s: v / 1e6 for s,v in t.items()}, scans=scans/1e6,
                streams=streams/1e6, explicit_tags=explicit_tags/1e6, export_total=total/1e6,
                remainder=(total-sum(t.values()))/1e6,
                outside_export=(row["cpu_ns"]-total)/1e6)


def summarize(rows, blocks):
    out = {}
    index = {(row["state"],row["engine"],row["tagged"],row["block"]):row for row in rows}
    for state,_,_ in STATES:
        cpu = {engine: {tag: [index[state,engine,tag,i]["cpu_ns"]/1e6 for i in range(blocks)]
                         for tag in (False,True)} for engine in ENGINES}
        def paired(engine, baseline, tagged):
            return bootstrap([x/y for x,y in zip(cpu[engine][tagged],cpu[baseline][tagged])])
        summaries = {engine: dict(tagged_ms=median(v[True]), untagged_ms=median(v[False]),
                                  tag_delta_ms=bootstrap([a-b for a,b in zip(v[True],v[False])]))
                     for engine,v in cpu.items()}
        stages = {tag:[bucket(index[state,"probe",tag,i]) for i in range(blocks)]
                  for tag in (False,True)}
        keys = tuple(stages[True][0])
        ratios = {k: bootstrap([a[k]-b[k] for a,b in zip(stages[True],stages[False])]) for k in keys}
        by_tag = {str(tag).lower():{k:median(r[k] for r in data) for k in keys}
                  for tag,data in stages.items()}
        out[state] = dict(process_cpu=summaries, tekt_over_vanilla_tagged=paired("tekt","vanilla",True),
                          probe_over_tekt_tagged=paired("probe","tekt",True),
                          probe_over_tekt_untagged=paired("probe","tekt",False),
                          tekt_stage_cpu_ms=by_tag, stage_tag_delta_ms=ratios)
    return out


def verify(rows):
    ref, fingerprints = {}, {}
    def fingerprint(path):
        digest = measure.sha(path)
        if digest not in fingerprints:
            fingerprints[digest] = runner.describe(path)
        return fingerprints[digest]
    for engine, tagged in ARMS:
        case = WORK / "fresh" / f"{engine}-{tagged}"
        for name,data,_ in STATES:
            pdf = fresh(case,engine,tagged,name,data)
            observed = fingerprint(pdf)
            assert observed["tags"] is tagged, (engine,name,tagged,observed["tags"])
            ref[engine,tagged,name] = observed
            if engine == "probe":
                assert observed == ref["tekt",tagged,name], (name,tagged)
    directed = []
    for engine,tagged in ARMS:
        if engine == "vanilla":
            continue
        case = WORK / "directed" / f"{engine}-{tagged}"
        directed.extend(run_watch(case,engine,tagged,trace=True))
    for row in rows + directed:
        actual = fingerprint(WORK / row["path"])
        assert actual == ref[row["engine"],row["tagged"],row["state"]], row
    return dict(utc=measure.utc(), nominal=len(rows), directed=len(directed))


def main():
    os.sched_setaffinity(0,{2})
    blocks = int(os.environ.get("BLOCKS","12"))
    assert blocks % 6 == 0
    active = json.loads((OLD/"receipt.json").read_text())
    vanilla = json.loads((ROOT/"lab/parity/vanilla.json").read_text())
    hashes = {arm:measure.sha(exe) for arm,exe in ENGINES.items()}
    assert hashes["vanilla"] == vanilla["local_binary_sha256"]
    assert hashes["tekt"] == active["elves"]["inert"]
    assert hashes["probe"] == (D/"candidate.sha256").read_text().strip()
    assert (D/"control-input-manifest.sha256").read_bytes() == (D/"control-before.sha256").read_bytes()
    inputs = {state:hashlib.sha256(data).hexdigest() for state,data,_ in STATES}
    assert all(value == active["inputs"]["regular"][state]["main"] for state,value in inputs.items())
    files = ("control-diffstat.txt","control-input-manifest.sha256","candidate-input-manifest.sha256",
             "build-candidate.sh","prototype.py","watch_probe.inc.rs","pdf_cpu.rs","measure.py","run-ram.sh")
    measure.write_json(D/"receipt.json",dict(utc=measure.utc(), head=active["head"], elves=hashes,
         vanilla_pin=vanilla["commit"], inputs=inputs, blocks=blocks, cpus=dict(child=1,controller=2),
         provenance={name:measure.sha(D/name) for name in files},
         prior_tekt_receipt=measure.sha(OLD/"receipt.json")))
    rows = []
    order = (0,1,5,2,4,3)
    for block in range(blocks):
        for position in order:
            engine,tagged = ARMS[(position+block)%6]
            case = WORK / "runs" / str(block) / f"{engine}-{tagged}"
            rows.extend(dict(block=block,**row) for row in run_watch(case,engine,tagged))
        print("block",block,flush=True)
    with gzip.open(D/"nominal.json.gz","wt") as output:
        json.dump(rows,output,indent=2)
    measure.write_json(D/"timing.json",summarize(rows,blocks))
    measure.write_json(D/"verification.json",verify(rows))


if __name__ == "__main__":
    main()
