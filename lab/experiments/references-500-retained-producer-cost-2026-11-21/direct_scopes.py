"""Time call sites of the source-producer executor, read validator, and content builder."""
from bisect import bisect_right
from collections import Counter
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from statistics import median
import struct
import subprocess

D = Path(__file__).resolve().parent
LABELS = ("execute", "read_validation", "source_substitution")
TABLE = struct.Struct("<51Q")
POINTS = (
    "::context_stabilization::Session::execute",
    "::context_reads_valid_for",
    "::context_stabilization::Session::content",
)


def prepare(bench, arm, table):
    entries, addresses = table
    executable = bench.WORK/arm
    targets = {address: (scope, name) for address, (_,name) in entries.items()
               for scope, suffix in enumerate(POINTS) if name.endswith(suffix)}
    assert {which for which,_ in targets.values()} == set(range(3)), (arm,targets)
    relevant = [a for a, (n, name) in entries.items()
                if n and (name.startswith("typst_infra::pipeline::context_stabilization::")
                          or name.endswith("::pipeline::paged::compile_paged_in_session"))]
    listing = subprocess.check_output(["objdump","-d","-w",f"--start-address={min(relevant):#x}",
                                       f"--stop-address={max(a+entries[a][0] for a in relevant):#x}",executable],
                                      text=True)
    relocs = subprocess.check_output(["readelf","-rW",executable],text=True)
    got_targets = {int(fields[0],16):int(fields[3],16) for line in relocs.splitlines()
                   if len(fields := line.split()) == 4 and fields[2] == "R_X86_64_RELATIVE"
                   and int(fields[3],16) in targets}
    points = []
    for line in listing.splitlines():
        got = 0
        raw = re.match(r"^\s*([a-f0-9]+):\s+e8 (?:[a-f0-9]{2} ){3}[a-f0-9]{2}\s+call\s+([a-f0-9]+) ",line)
        if raw:
            address, target = [int(s,16) for s in raw.groups()]
        else:
            rip = re.match(r"^\s*([a-f0-9]+):\s+ff 15 .*\bcall.*# ([a-f0-9]+) ",line)
            if not rip:
                continue
            address, got = [int(s,16) for s in rip.groups()]
            target = got_targets.get(got)
        if target not in targets:
            continue
        which, callee = targets[target]
        start = addresses[bisect_right(addresses,address)-1]
        n, caller = entries[start]
        if not (start in relevant and start <= address < start+n):
            continue
        if which == 1 and not caller.endswith("::context_stabilization::Session::stabilize"):
            continue
        points.append(dict(which=which, label=LABELS[which], address=address, target=target, got=got,
                           caller=caller, callee=callee))
    assert len(points)<128 and {p["which"] for p in points}=={0,1,2}, (arm,points)
    assert sum(p["which"]==1 for p in points)==2, (arm,points)
    for i in range(3):
        assert len({p["target"] for p in points if p["which"]==i})==1, (arm,points)
    bench.dump(f"{arm}-direct-points.json", points)
    path = bench.WORK/f"{arm}-brackets"
    path.mkdir()
    (path/"points.inc").write_text("".join(
        f"{{{p['which']}, {p['address']:#x}, {p['target']:#x}, {p['got']:#x}}},\n" for p in points))
    old = (bench.initial.D/"direct.c").read_text()
    marker = "        if (originals[p.scope]) _exit(94);"
    assert old.count(marker)==1
    source = path/"brackets.c"
    source.write_text(old.replace(marker, """        if (originals[p.scope] &&
            (uintptr_t)originals[p.scope] != image + p.callee) _exit(94);"""))
    probe = path/"brackets.so"
    subprocess.run(["cc","-O2","-Wall","-Wextra","-Werror","-fPIC","-shared","-I",path,
                    "-o",probe,source],check=True)
    return probe


def bracket(bench,gold,tables):
    arms = ("reuse","proof")
    probes = {a:prepare(bench,a,tables[a]) for a in arms}
    rows = {a:[] for a in arms}
    order = list(permutations(arms))*6
    Random(89123).shuffle(order)
    for block, pair in enumerate(order):
        for arm in pair:
            case = bench.WORK/"direct"/f"{block:02d}-{arm}"
            with bench.direct.tracing(bench.base,probes[arm],TABLE):
                verified_pdfs = bench.watch(case,arm,gold)
            publication, clock_n, clock_ns, *cells = TABLE.unpack((case/"realization.bin").read_bytes())
            assert (publication,clock_n)==(4,256)
            clock = clock_ns/clock_n
            visible = cells[3*12:4*12]
            assert all(x==0 for x in visible[6:])
            values = {k:dict(count=n,raw_ns=ns,ms=(ns-n*clock)/1e6)
                      for k,n,ns in zip(LABELS,visible[::2],visible[1::2])}
            rows[arm].append(dict(block=block, clocks=clock, values=values, pdf=verified_pdfs))
        bench.dump("direct-progress.json",rows)
        print(f"bracket block {block+1}/{len(order)}",flush=True)
    summary = {
        a:{k:dict(counts=Counter(r["values"][k]["count"] for r in rs),
                  median_ms=median(r["values"][k]["ms"] for r in rs),
                  ci95=bench.decoder.stats([r["values"][k]["ms"] for r in rs])["ci95"]) for k in LABELS}
        for a,rs in rows.items()
    }
    total = {
        a:[sum(v["ms"] for v in r["values"].values()) for r in rs]
        for a,rs in rows.items()
    }
    paired = {k:bench.decoder.stats([old["values"][k]["ms"]-new["values"][k]["ms"]
                                      for old,new in zip(rows["reuse"],rows["proof"])]) for k in LABELS}
    paired["total"] = bench.decoder.stats([a-b for a,b in zip(total["reuse"],total["proof"])])
    bench.dump("direct.json",dict(summaries=summary, rows=rows, paired=paired))
    print("direct scoped CPU",summary,paired,flush=True)


def vanilla(bench):
    native = bench.initial.native
    source = D.parent/"references-500-pdf-opening-2026-11-20"/"trace-clock.c"
    subprocess.run(["cc","-O2","-Wall","-Wextra","-shared","-fPIC","-o",bench.WORK/"trace-clock.so",source,"-ldl"],check=True)
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    records = []
    for block in range(12):
        base = native.run(block,gold)
        case = bench.WORK/"vanilla-traces"/f"{block:02d}"
        clock = dict(struct.iter_unpack("<QQ",(case/"clock.bin").read_bytes()))
        events = json.loads((case/"timings-3.json").read_text())
        offsets = [round(e["ts"]*1000) for e in events]
        zero, = (word for word in clock if all(word+off in clock for off in offsets[:12]))
        readings = [(e["ph"],clock[zero+off]) for e,off in zip(events,offsets)
                    if e["name"]=="check stabilized"]
        assert len(readings)==2*base["counts"]["check stabilized"]
        assert all(a=="B" and b=="E" and x<=y for (a,x),(b,y) in zip(readings[::2],readings[1::2]))
        counted = len(readings)//2
        elapsed = sum(y-x for (_,x),(_,y) in zip(readings[::2],readings[1::2]))/1e6
        records.append(dict(block=block, checks=counted, check_ms=elapsed,
                            root=bench.initial.vanilla_scopes(case), phases=base["phase_ms"]))
        print(f"vanilla scope block {block+1}/12",flush=True)
    bench.dump("vanilla-native.json",dict(records=records,
                checks=Counter(r["checks"] for r in records),
                check_ms=bench.decoder.stats([r["check_ms"] for r in records])))


def main(bench,gold,tables):
    bracket(bench,gold,tables)
    vanilla(bench)
