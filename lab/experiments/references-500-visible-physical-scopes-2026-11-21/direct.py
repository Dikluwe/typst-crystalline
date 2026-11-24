"""Measure the verified root, finish, outline and index call sites on exact ELFs."""
from bisect import bisect_right
from collections import Counter
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
import struct
import subprocess

D = Path(__file__).resolve().parent
TABLE = struct.Struct("<51Q")
SCOPES = (
    ("root", "::layout_planned_root"),
    ("finish", "::finish_with_running"),
    ("index", "::index_physical_steps"),
    ("outline", "::prepare_outline_candidate"),
)


def prepare(bench, arm, table):
    entries,addresses=table
    exe=bench.WORK/arm
    targets={addr:(label,name) for addr,(_,name) in entries.items()
             for label,suffix in SCOPES if name.endswith(suffix)}
    callers={addr for addr,(n,name) in entries.items()
             if n and (name.endswith("::convergence::run_planned_lab")
                       or name.startswith("typst_infra::pipeline::context_stabilization::"))}
    relocs=subprocess.check_output(["readelf","-rW",exe],text=True)
    got_targets={int(fields[0],16):int(fields[3],16) for line in relocs.splitlines()
                 if len(fields:=line.split())==4 and fields[2]=="R_X86_64_RELATIVE"
                 and int(fields[3],16) in targets}
    listing=subprocess.check_output(
        ["objdump","-d","-w",f"--start-address={min(callers):#x}",
         f"--stop-address={max(a+entries[a][0] for a in callers):#x}",exe],text=True)
    points=[]
    for line in listing.splitlines():
        got=0
        relative=re.match(r"^\s*([a-f0-9]+):\s+e8 (?:[a-f0-9]{2} ){3}[a-f0-9]{2}\s+call\s+([a-f0-9]+) ",line)
        if relative:
            address,target=map(lambda s:int(s,16),relative.groups())
        else:
            indirect=re.match(r"^\s*([a-f0-9]+):\s+ff 15 .*\bcall.*# ([a-f0-9]+) ",line)
            if not indirect:
                continue
            address,got=map(lambda s:int(s,16),indirect.groups())
            target=got_targets.get(got)
        if target not in targets:
            continue
        start=addresses[bisect_right(addresses,address)-1]
        size,caller=entries[start]
        if start not in callers or address>=start+size:
            continue
        label,callee=targets[target]
        if label in ("root","finish") and not caller.endswith("::convergence::run_planned_lab"):
            continue
        points.append(dict(label=label,address=address,target=target,got=got,caller=caller,callee=callee))
    keys=sorted({(p["label"],p["target"]) for p in points})
    assert {label for label,_ in keys}=={label for label,_ in SCOPES}, (arm,points)
    assert 0<len(keys)<=6 and len(points)<128
    for p in points:
        p["slot"]=keys.index((p["label"],p["target"]))
    bench.dump(f"{arm}-direct-points.json",points)
    path=bench.WORK/f"{arm}-brackets"
    path.mkdir()
    (path/"points.inc").write_text("".join(
        f"{{{p['slot']}, {p['address']:#x}, {p['target']:#x}, {p['got']:#x}}},\n" for p in points))
    inherited=(bench.initial.D/"direct.c").read_text()
    marker="        if (originals[p.scope]) _exit(94);"
    assert inherited.count(marker)==1
    source=path/"brackets.c"
    source.write_text(inherited.replace(marker,
       "        if (originals[p.scope] && (uintptr_t)originals[p.scope] != image + p.callee) _exit(94);"))
    probe=path/"brackets.so"
    subprocess.run(["cc","-O2","-Wall","-Wextra","-Werror","-fPIC","-shared","-I",path,
                    "-o",probe,source],check=True)
    return probe,keys


def brackets(bench,gold,tables):
    arms=("reuse","proof")
    probes={a:prepare(bench,a,tables[a]) for a in arms}
    results={a:[] for a in arms}
    order=list(permutations(arms))*6
    Random(29555).shuffle(order)
    for block,pair in enumerate(order):
        for arm in pair:
            case=bench.WORK/"direct"/f"{block:02d}-{arm}"
            probe,keys=probes[arm]
            with bench.direct.tracing(bench.base,probe,TABLE):
                checks=bench.watch(case,arm,gold)
            publication,calibration,ns,*cells=TABLE.unpack((case/"realization.bin").read_bytes())
            assert (publication,calibration)==(4,256)
            values=[]
            for pub in range(4):
                sums={label:dict(calls=0,ms=0.0) for label,_ in SCOPES}
                for slot,(label,_) in enumerate(keys):
                    count,elapsed=cells[pub*12+slot*2:pub*12+slot*2+2]
                    sums[label]["calls"]+=count
                    sums[label]["ms"]+=(elapsed-count*ns/calibration)/1e6
                values.append(sums)
            assert [v["root"]["calls"] for v in values]==[1,0,0,1], (arm,values)
            assert [v["finish"]["calls"] for v in values]==[1,0,0,1], (arm,values)
            results[arm].append(dict(block=block,values=values,clock_ns=ns/calibration,pdfs=checks))
        bench.dump("direct-progress.json",results)
        print(f"Tekt physical clocks {block+1}/{len(order)}",flush=True)
    summary={a:{label:dict(**bench.decoder.stats([r["values"][-1][label]["ms"] for r in rows]),
                          calls=Counter(r["values"][-1][label]["calls"] for r in rows))
                for label,_ in SCOPES} for a,rows in results.items()}
    paired={label:bench.decoder.stats([b["values"][-1][label]["ms"]-a["values"][-1][label]["ms"]
                for a,b in zip(results["reuse"],results["proof"])]) for label,_ in SCOPES}
    bench.dump("direct.json",dict(summary=summary,paired_proof_minus_reuse=paired,rows=results))


def vanilla_phases(case):
    clock=dict(struct.iter_unpack("<QQ",(case/"clock.bin").read_bytes()))
    events=json.loads((case/"timings-3.json").read_text())
    offsets=[round(e["ts"]*1000) for e in events]
    zero,=(word for word in clock if all(word+x in clock for x in offsets[:12]))
    totals,names,stack=Counter(),Counter(),[]
    for e,offset in zip(events,offsets):
        end=clock[zero+offset]
        if e["ph"]=="B":
            stack.append([e["name"],end,0])
            continue
        name,begin,child=stack.pop()
        assert name==e["name"] and end>=begin and end-begin>=child
        if stack:
            stack[-1][2]+=end-begin
        ancestors=[v[0] for v in stack]+[name]
        if name in ("layout document","check stabilized"):
            totals[name]+=end-begin
        if "layout document" not in ancestors:
            continue
        names[name]+=1
        if "introspect pages" in ancestors:
            label="physical_index"
        elif "realize" in ancestors:
            label="document_realization" if ancestors[ancestors.index("realize")-1]=="layout document" else "nested_realization"
        elif "prepare" in ancestors:
            label="paragraph_prepare"
        elif any(x.startswith("linebreak") for x in ancestors):
            label="paragraph_break"
        elif "finalize" in ancestors:
            label="paragraph_finalize"
        elif "page run" in ancestors:
            label="page_flow_other"
        else:
            label="layout_other"
        totals[label]+=end-begin-child
    assert not stack
    assert totals["layout document"]==sum(v for k,v in totals.items() if k not in ("layout document","check stabilized"))
    return dict(ms={k:v/1e6 for k,v in totals.items()},counts=names)


def vanilla(bench):
    source=D.parent/"references-500-pdf-opening-2026-11-20/trace-clock.c"
    subprocess.run(["cc","-O2","-Wall","-Wextra","-shared","-fPIC","-o",bench.WORK/"trace-clock.so",
                    source,"-ldl"],check=True)
    gold=json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    rows=[]
    for block in range(12):
        observation=bench.initial.native.run(block,gold)
        summary=vanilla_phases(bench.WORK/"vanilla-traces"/f"{block:02d}")
        rows.append(dict(block=block,observation=observation,**summary))
        print(f"vanilla native clocks {block+1}/12",flush=True)
    keys={k for r in rows for k in r["ms"]}
    stats={k:bench.decoder.stats([r["ms"].get(k,0) for r in rows]) for k in sorted(keys)}
    bench.dump("vanilla-native.json",dict(summary=stats,rows=rows))


def main(bench,gold,tables):
    brackets(bench,gold,tables)
    vanilla(bench)
