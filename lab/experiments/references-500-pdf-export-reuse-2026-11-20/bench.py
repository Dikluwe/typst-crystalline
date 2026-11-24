"""Same-campaign PDF exporter reuse comparisons on the original comments."""
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
spec = importlib.util.spec_from_file_location("prior_watch_cases", OLD / "bench.py")
previous = importlib.util.module_from_spec(spec)
spec.loader.exec_module(previous)
runner = previous.runner
driver = runner.driver
measure = runner.measure
REGULAR = previous.REGULAR
NOMINAL = REGULAR[:3]
DEPENDENCY = previous.DEPENDENCY
ARMS = ("tekt", "off", "fonts", "operators", "both", "vanilla")
EXES = {"tekt": WORK/"tekt", "vanilla": WORK/"vanilla"}
EXES.update({arm: WORK/"candidate" for arm in ARMS if arm not in EXES})
RESOURCE_SOURCE = REGULAR[0][1] + b"#rect(width: 30pt, height: 10pt)\n"
RESOURCES = [("resource-open", RESOURCE_SOURCE, None),
             ("resource-comment", RESOURCE_SOURCE + b"// a hidden terminal comment", None)]
LINE = re.compile(r"^pdf-reuse fonts=(true|false) operators=(\d+)/(\d+) eligible=(true|false)$",re.M)


def run_watch(case, arm, states, tagged=True, trace=False):
    case.mkdir(parents=True)
    tmp, source, pdf = case/"tmp", case/"main.typ", case/"out.pdf"
    tmp.mkdir()
    runner.setup_sources(case, *states[0][1:])
    args = [str(EXES[arm]),"watch",str(source),str(pdf),"--root",str(case),
            f"--pdf-tagged={str(tagged).lower()}"]
    env = driver.environment(tmp)
    if arm != "vanilla":
        env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1", TEKT_LAB_WATCH_INERT="1")
        if trace:
            env["TEKT_LAB_WATCH_TRACE"] = "1"
        if arm in ("fonts","operators","both"):
            env["TEKT_LAB_PDF_REUSE"] = arm
    rows = []
    log_path = case/"watch.log"
    with log_path.open("wb") as log:
        start = time.perf_counter_ns()
        child = driver.Child(args,case,env,1,log)
        try:
            publication = None
            for cycle,(state,data,leaf) in enumerate(states):
                before = 0
                if cycle:
                    before = child.cpu()
                    start = time.perf_counter_ns()
                    runner.setup_sources(case,data,leaf)
                publication,wall,_ = driver.await_pdf(child,pdf,publication)
                after = measure.settle(child)
                output=case/f"{state}.pdf"
                output.write_bytes(pdf.read_bytes())
                rows.append(dict(arm=arm,state=state,tagged=tagged,utc=measure.utc(),cpu_ns=after-before,
                                 wall_ns=wall-start,path=str(output.relative_to(WORK)),
                                 **driver.read_memory(child.process.pid)))
        finally:
            child.stop()
            child.close()
    content=log_path.read_text()
    assert content.count("compiled successfully") == len(states), (case,content[-2500:])
    if trace and arm != "tekt":
        counters=LINE.findall(content)
        assert len(counters)==len(rows),(case,content[-3000:])
        for row,(font_hit,pages,all_pages,eligible) in zip(rows,counters):
            row.update(font_hit=font_hit=="true",operator_hits=int(pages),
                       exported_pages=int(all_pages),eligible=eligible=="true")
        if states is REGULAR:
            assert sum(line.startswith("watch-retain hit inert") for line in content.splitlines())==2
    return rows


def fresh(case, arm, states, tagged):
    case.mkdir(parents=True)
    for state,data,leaf in states:
        runner.setup_sources(case,data,leaf)
        pdf=case/f"{state}.pdf"
        args=[str(EXES[arm]),"compile",str(case/"main.typ"),str(pdf),"--root",str(case),
              f"--pdf-tagged={str(tagged).lower()}"]
        with (case/f"{state}.log").open("wb") as log:
            child=driver.Child(args,case,driver.environment(case),1,log)
            try:
                child.reap()
                assert child.process.returncode==0
            finally:
                child.close()


def qdf_hash(path):
    raw=subprocess.check_output(["qpdf","--qdf","--object-streams=disable",str(path),"-"],
                                timeout=30,stderr=subprocess.PIPE)
    raw,xmp=re.subn(rb"(<xmpMM:(?:Instance|Document)ID>)[A-Za-z0-9+/=]+(</xmpMM:(?:Instance|Document)ID>)",
                    lambda m:m[1]+b"x"*(len(m[0])-len(m[1])-len(m[2]))+m[2],raw)
    raw,trailer=re.subn(rb"/ID \[<[0-9a-f]+><[0-9a-f]+>\]",b"/ID []",raw)
    assert (xmp,trailer)==(2,1),(path,xmp,trailer)
    return hashlib.sha256(raw).hexdigest()


def verify(rows):
    fresh_root=WORK/"fresh"
    targets = (("regular", REGULAR, True, ARMS),
               ("dependency", DEPENDENCY, True, ("tekt","both")),
               ("resources", RESOURCES, True, ("tekt","both")),
               ("untagged", REGULAR, False, ("tekt","both")))
    descriptions={}
    checked=[]
    def describe(path):
        digest=measure.sha(path)
        if digest not in descriptions:
            descriptions[digest]=runner.describe(path)
        return descriptions[digest]
    for kind,states,tagged,arms in targets:
        tekt=fresh_root/kind/"tekt"
        fresh(tekt,"tekt",states,tagged)
        for engine in ("off","vanilla"):
            if engine in arms:
                directory = fresh_root/kind/engine
                fresh(directory,engine,states,tagged)
                if engine == "off":
                    for state,_,_ in states:
                        assert describe(directory/f"{state}.pdf") == describe(tekt/f"{state}.pdf")
                        assert qdf_hash(directory/f"{state}.pdf") == qdf_hash(tekt/f"{state}.pdf")
        for arm in arms:
            # Vanilla nominal outputs have their own fresh references.
            if arm == "vanilla":
                continue
            case=WORK/"directed"/kind/arm
            actual=run_watch(case,arm,states,tagged,trace=True)
            for row in actual:
                state=row["state"]
                path=WORK/row["path"]
                expected=tekt/f"{state}.pdf"
                assert describe(path)==describe(expected), (kind,arm,state)
                assert qdf_hash(path)==qdf_hash(expected), (kind,arm,state,"PDF structure")
                if arm != "tekt":
                    hit=(kind in ("regular","untagged") and state in ("comment","comment-2"))
                    assert row["font_hit"]==(hit and arm in ("fonts","both")),row
                    assert row["operator_hits"]==(23 if hit and arm in ("operators","both") else 0),row
                    if kind=="resources":
                        assert not row["eligible"],row
                checked.append(dict(kind=kind,**row))
        print("validated",kind,flush=True)
    expected = {
        arm:{state:describe(fresh_root/"regular"/("vanilla" if arm=="vanilla" else "tekt")/f"{state}.pdf")
             for state,_,_ in NOMINAL}
        for arm in ARMS
    }
    for row in rows:
        assert describe(WORK/row["path"])==expected[row["arm"]][row["state"]],row
    return dict(utc=measure.utc(),nominal=len(rows),directed=len(checked),checks=checked)


def bootstrap(values):
    rng=Random(500)
    samples=sorted(median(rng.choices(values,k=len(values))) for _ in range(10000))
    return dict(median=median(values),ci95=[samples[250],samples[9749]])


def summarize(rows, blocks):
    index={(r["arm"],r["state"],r["block"]):r for r in rows}
    assert len(index)==len(ARMS)*len(NOMINAL)*blocks
    def stats(by_arm):
        out=dict(median_ms={arm:median(v)/1e6 for arm,v in by_arm.items()})
        out["paired_ratios"]={
            f"{a}_over_{b}":bootstrap([x/y for x,y in zip(by_arm[a],by_arm[b])])
            for a,b in (("off","tekt"),("fonts","tekt"),("operators","tekt"),("both","tekt"),
                        ("fonts","off"),("operators","off"),("both","off"),("both","vanilla"),
                        ("both","fonts"),("both","operators"),("tekt","vanilla"))
        }
        return out
    by_state={state:{metric:stats({arm:[index[arm,state,b][metric] for b in range(blocks)]
                                  for arm in ARMS})
                    for metric in ("cpu_ns","wall_ns")}
              for state,_,_ in NOMINAL}
    names=[state for state,_,_ in NOMINAL]
    totals=stats({arm:[sum(index[arm,state,b]["cpu_ns"] for state in names)
                       for b in range(blocks)] for arm in ARMS})
    peak={arm:max(row["hwm_bytes"] for row in rows if row["arm"]==arm)/2**20 for arm in ARMS}
    return dict(by_state=by_state,through_second_cpu=totals,max_hwm_mib=peak)


def main():
    os.sched_setaffinity(0,{2})
    blocks=int(os.environ.get("BLOCKS","12"))
    assert blocks % 6==0
    base=json.loads((OLD/"receipt.json").read_text())
    pin=json.loads((ROOT/"lab/parity/vanilla.json").read_text())
    hashes={arm:measure.sha(path) for arm,path in EXES.items()}
    assert hashes["tekt"]==base["elves"]["inert"]
    assert hashes["vanilla"]==pin["local_binary_sha256"]
    assert hashes["both"]==(D/"candidate.sha256").read_text().strip()
    assert (D/"control-input-manifest.sha256").read_bytes()==(D/"control-before.sha256").read_bytes()
    inputs={state:hashlib.sha256(data).hexdigest() for state,data,_ in REGULAR}
    assert inputs=={s:a["main"] for s,a in base["inputs"]["regular"].items()}
    files=("control-diffstat.txt","control-input-manifest.sha256","candidate-input-manifest.sha256",
           "prototype.py","watch_probe.inc.rs","pdf_reuse.rs","bench.py","build-candidate.sh","run-ram.sh")
    measure.write_json(D/"receipt.json",dict(utc=measure.utc(),head=base["head"],blocks=blocks,
        elves=hashes,vanilla_pin=pin["commit"],cpus=dict(compiler=1,driver=2),inputs=inputs,
        parent_receipt=measure.sha(OLD/"receipt.json"),
        provenance={name:measure.sha(D/name) for name in files}))
    order=(0,1,5,2,4,3)
    rows=[]
    for b in range(blocks):
        for i in order:
            arm=ARMS[(i+b)%6]
            case=WORK/"runs"/str(b)/arm
            rows.extend(dict(block=b,**r) for r in run_watch(case,arm,NOMINAL))
        print("block",b,flush=True)
    with gzip.open(D/"nominal.json.gz","wt") as stream:
        json.dump(rows,stream,indent=2)
    measure.write_json(D/"timing.json",summarize(rows,blocks))
    measure.write_json(D/"verification.json",verify(rows))


if __name__=="__main__":
    main()
