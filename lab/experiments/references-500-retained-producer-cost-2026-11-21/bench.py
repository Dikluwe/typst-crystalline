"""Compare retained-producer and no-producer watch editions in the same workload."""
from collections import Counter
from functools import lru_cache
from hashlib import sha256
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from shutil import copy2
import subprocess
import sys
import tarfile

from scopes import TEKT, VANILLA, category

D = Path(__file__).resolve().parent


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


heading = load("prior_heading", D.parent / "references-500-heading-reference-proof-2026-11-21" / "bench.py")
initial, base = heading.initial, heading.base
WORK, OUT, STATES = heading.WORK, heading.OUT, heading.STATES
decoder = initial.decoder
direct = load("prior_direct", initial.D / "direct.py")
ARMS = ("reuse", "proof", "vanilla")
# The exact retained ELF in binaries.sha256 inlines both replay-engine wrappers.
# These blocks start after the location comparison and end before the match on
# the two results. The inner context_reads_valid_for has its own symbol.
REPLAY_ENGINE_RANGES = ((0x29AE2F0, 0x29AEC0D),)


def dump(name, payload):
    (OUT / name).write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def environment(tmp, arm, trace):
    env = heading.ORIGINAL_ENV(tmp, arm, trace)
    if trace and arm == "reuse":
        env["TEKT_LAB_TOPOLOGY_INDEX_TRACE"] = "verify"
    if trace and arm == "proof":
        env["TEKT_LAB_HEADING_REF_TRACE"] = "1"
    return env


base.environment = environment
heading.environment = environment

original_key = base.cycle.cycle.key
known_qdf = {}


def cached_key(engine, pdf):
    if engine == "vanilla":
        return original_key(engine, pdf)
    raw = sha256(pdf.read_bytes()).digest()
    if raw not in known_qdf:
        known_qdf[raw] = original_key(engine, pdf)
    return known_qdf[raw]


base.cycle.cycle.key = cached_key


def watch(folder, arm, gold, *, trace=False, period=0):
    rows = base.watch(folder, arm, STATES, trace=trace, period=period, keep=True)
    assert all(r["pdf_hash"] == gold[arm][r["state"]] for r in rows), (arm, folder, rows)
    if not trace:
        for row in rows:
            (folder / (row["state"] + ".pdf")).unlink()
    return rows


def validate(gold):
    folder = WORK / "validation"
    checks = {a: watch(folder/a, a, gold, trace=True) for a in ARMS}
    trace = (folder/"reuse/watch.log").read_text()
    publication = trace.split("compiled successfully")[-2]
    assert re.search(r"attempt=1 roots=20 builds=1 hits=19 physical=0 invalidations=0 checked=19", publication)
    assert [False, None, None, False] == heading.watch_flags(folder/"proof")
    comparisons = {}
    for name, *_ in STATES:
        pdfs = {a: heading.pdf_details(folder/a/(name+".pdf")) for a in ARMS}
        same = heading.compare(pdfs["reuse"], pdfs["proof"])
        assert all(same.values()), (name,same)
        comparisons[name] = dict(same=same, pages=pdfs["reuse"]["pages"],
                                 links={a:len(pdf["links"]) for a,pdf in pdfs.items()})
    rasters = [base.render(folder/a/"visible.pdf", folder/a, "visible-page") for a in ARMS[:2]]
    assert rasters[0] == rasters[1]
    figures = b'#figure([ART], kind: image, caption: [CAP], supplement: it => [CALLBACK]) <fig>\nSee @fig.\n'
    controls = {a: heading.compile_case(folder/("figure-"+a), a, figures) for a in ARMS}
    assert all(r["code"] == 0 and "CALLBACK" in r["content"]["text"] for r in controls.values())
    assert controls["reuse"]["qdf"] == controls["proof"]["qdf"]
    assert controls["proof"]["initial"] == ["true"], controls["proof"]["initial"]
    dump("validation.json", dict(original=checks, semantic_pdf_checks=comparisons,
                                  figure=controls, rasters=len(rasters[0]),
                                  reuse_census=[line for line in publication.splitlines()
                                                if line.startswith("lab-topology-index")]))
    print("PDFs and figure callback validated", flush=True)


def configure_symbols():
    copy2(initial.prior.residual.TABLES["vanilla"], WORK/"vanilla.symbols")
    for a in ARMS[:2]:
        with (WORK/f"{a}.symbols").open("wb") as file:
            subprocess.run(["nm","-n","-S","-C","--defined-only",WORK/a], stdout=file, check=True)
    return {a: decoder.symbols(WORK/f"{a}.symbols") for a in ARMS}


def measure(gold, mode):
    orders = list(permutations(ARMS))
    queue = [(0, order) for order in orders*3] if mode == "nominal" else [
        (period, order) for period in (93000, 227000) for order in orders
    ]
    Random(61346).shuffle(queue)
    rows = []
    filename = f"measurements-{mode}.jsonl"
    with (WORK / filename).open("w") as result:
        for block, (period, order) in enumerate(queue):
            for arm in order:
                for row in watch(WORK/mode/f"{block:02d}-{arm}", arm, gold, period=period):
                    rows.append(dict(row, block=block))
                    print(json.dumps(rows[-1]), file=result, flush=True)
            copy2(WORK/filename, OUT/filename)
            print(f"{mode} block {block+1}/{len(queue)}", flush=True)
    return rows


def nominal(rows):
    vals = {a:{} for a in ARMS}
    for row in rows:
        if row["state"] == "visible":
            vals[row["arm"]][row["block"]] = row["cpu_ns"]/1e6
    absolute = {a:decoder.stats(list(g.values())) for a,g in vals.items()}
    paired = {a:decoder.stats([v-vals[a][k] for k,v in vals["reuse"].items()])
              for a in ("proof","vanilla")}
    dump("nominal.json", dict(absolute=absolute, paired_reuse_minus=paired))
    print("nominal", absolute, paired, flush=True)


class Scratch:
    def extractfile(self, name):
        return (WORK/name).open("rb")


def resolver_with_replay(maps, exe, table):
    base_resolve = original_resolver(maps, exe, table)
    if Path(exe).name != "reuse":
        return base_resolve
    image = next(int(fields[0].split("-")[0],16) for line in maps.splitlines()
                 if len(fields := line.split(maxsplit=5)) == 6
                 and int(fields[2],16) == 0 and fields[5] == exe)

    @lru_cache(maxsize=None)
    def resolve(pc):
        relative = pc-image
        if any(start <= relative < end for start,end in REPLAY_ENGINE_RANGES):
            return "[producer-replay-engine]"
        return base_resolve(pc)
    return resolve


original_resolver = decoder.resolver


def profile(rows, tables, archive=None):
    decoder.resolver = resolver_with_replay
    source = archive if archive is not None else Scratch()
    by_case = {a:[] for a in ARMS}
    hot = Counter()
    for row in rows:
        if row["state"] != "visible":
            continue
        a = row["arm"]
        stacks, detail = decoder.stacks(source, row, tables)
        counts = Counter()
        for stack, n in stacks.items():
            label = category(a, stack)
            counts[label] += n
            if label in ("execute", "validate_reads", "validation_engine", "validate_other", "substitute_source",
                         "substitute_other"):
                hot[a,label,stack] += n
        unit = detail["main_ns"]/detail["samples"]/1e6
        by_case[a].append(dict(**detail, counts=dict(counts), period_ns=row["period_ns"], block=row["block"],
                               ms={k: counts[k]*unit for k in (TEKT if a != "vanilla" else VANILLA)}))
    summary = {a:{k:dict(**initial.prior.residual.mean_ci([r["ms"][k] for r in runs]),
                          ticks=sum(r["counts"].get(k,0) for r in runs))
                  for k in runs[0]["ms"]} for a,runs in by_case.items()}
    with (OUT/"scope-stacks.txt").open("w") as stream:
        for (a,label,stack), n in hot.most_common():
            print(a,label,n," | ".join(stack),file=stream)
    dump("profile.json",dict(summary=summary,cases=by_case))
    print("profile scopes", {a:{k:x["mean"] for k,x in v.items()} for a,v in summary.items()}, flush=True)


def archive(mode):
    with tarfile.open(OUT/f"{mode}-cases.tar.gz","w:gz") as bundle:
        for root in (mode, "validation", "vanilla-traces"):
            for p in sorted((WORK/root).rglob("*")):
                if p.name in ("maps","samples.bin","watch.log","compile.log","clock.bin") or p.name.startswith("timings-"):
                    bundle.add(p,arcname=str(p.relative_to(WORK)))
        for a in ARMS:
            path = WORK/f"{a}.symbols"
            if path.exists():
                bundle.add(path,arcname=path.name)


def main():
    mode = sys.argv[1]
    if mode=="decode-profile":
        with tarfile.open(OUT/"profile-cases.tar.gz") as source:
            tables = {}
            for a in ARMS:
                path = WORK/f"{a}.symbols"
                path.write_bytes(source.extractfile(path.name).read())
                tables[a] = decoder.symbols(path)
            rows = [json.loads(line) for line in (OUT/"measurements-profile.jsonl").read_text().splitlines()]
            profile(rows,tables,source)
        return
    os.sched_setaffinity(0,{2})
    gold = {a:{s:base.golden("vanilla" if a=="vanilla" else "candidate",s) for s,*_ in STATES} for a in ARMS}
    tables = configure_symbols()
    try:
        if mode=="nominal":
            validate(gold)
            nominal(measure(gold,mode))
        elif mode=="profile":
            copy2(D.parent/"unicode-pdf-cost-2026-11-19/profiler.so",WORK/"profiler.so")
            profile(measure(gold,mode),tables)
        elif mode=="direct":
            run = load("direct_scopes", D/"direct_scopes.py")
            run.main(sys.modules[__name__],gold,tables)
        else:
            raise ValueError(mode)
    finally:
        archive(mode)


if __name__=="__main__":
    main()
