"""Attribute initial evaluation from the archived profiles and native timing scopes."""
from collections import Counter
import importlib.util
import json
from pathlib import Path
from statistics import median
import struct
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "sample_decode",D.parent/"references-500-visible-edit-phases-2026-11-21/analyze.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)


def category(stack):
    def has(*parts):
        return any(part in name for part in parts for name in stack)
    if has("::rules::textual::TextualSnapshot::same_group"):
        return "same_group"
    if has("::syntax_node::representation::linked::LinkedNode::find"):
        return "find_source_node"
    if has("::eval::operators::join::"):
        return "join"
    if has("::rules::textual::TextualSnapshot::capture"):
        return "capture_style"
    if has("::eval::markup_stream::eval_markup"):
        return "other_markup"
    return "other_eval"


def samples(source):
    rows = [json.loads(l) for l in (source/"measurements.jsonl").read_text().splitlines()]
    tables = {arm:base.symbols(source/f"{arm}.symbols") for arm in ("candidate","hybrid")}
    report = {}
    samples = []
    with tarfile.open(source/"profiles.tar.gz","r:gz") as archive:
        for row in rows:
            if row["arm"] not in tables or not row["period_ns"]:
                continue
            stacks, detail = base.stacks(archive,row,tables)
            counts, context = Counter(), Counter()
            for stack, count in stacks.items():
                if not any("::eval_to_module_in_session" in name for name in stack):
                    continue
                kind = category(stack)
                counts[kind] += count
                if any("::run_for_loop" in name for name in stack):
                    context[kind] += count
            scale = detail["main_ns"] / stacks.total() / 1e6
            phase = {k:v*scale for k,v in counts.items()}
            phase["eval_total"] = counts.total()*scale
            phase["inside_for"] = context.total()*scale
            samples.append(dict(arm=row["arm"],block=row["block"],period_ns=row["period_ns"],
                                counts=counts, for_counts=context,phase_ms=phase))
    for arm in tables:
        series = [r for r in samples if r["arm"]==arm]
        totals, within_for = Counter(), Counter()
        for r in series:
            totals.update(r["counts"])
            within_for.update(r["for_counts"])
        categories = sorted({k for r in series for k in r["phase_ms"]})
        report[arm] = dict(runs=len(series),sample_counts=totals,under_for_counts=within_for,
                           by_period={str(p):{k:median(r["phase_ms"].get(k,0) for r in series
                                                     if p==0 or r["period_ns"]==p)
                                              for k in categories} for p in (0,577000,937000)})
    return dict(results=report,raw=samples)


def vanilla(source):
    rows = []
    with tarfile.open(source/"vanilla-traces.tar.gz","r:gz") as archive:
        for i in range(12):
            path = f"vanilla-traces/{i:02d}/"
            events = json.load(archive.extractfile(path+"timings-3.json"))
            pairs = dict(struct.iter_unpack("<QQ",archive.extractfile(path+"clock.bin").read()))
            offsets = [round(event["ts"]*1000) for event in events]
            zero, = (word for word in pairs if all(word+delta in pairs for delta in offsets[:12]))
            stack = []
            totals, counts = Counter(), Counter()
            for event, delta in zip(events, offsets):
                name = event["name"]
                if event["ph"]=="B":
                    stack.append((name,pairs[zero+delta]))
                else:
                    prior, start = stack.pop()
                    assert prior==name
                    if name=="eval" or (name=="for loop" and any(name=="eval" for name,_ in stack)):
                        totals[name] += pairs[zero+delta]-start
                        counts[name] += 1
            assert not stack and counts["eval"]==1 and counts["for loop"]==20
            rows.append(dict(run=i,counts=counts,cpu_ms={k:n/1e6 for k,n in totals.items()}))
    return dict(rows=rows,phase_ms={k:median(r["cpu_ms"][k] for r in rows) for k in rows[0]["cpu_ms"]})


def capacity(source, out):
    # One immediate changed, so code locations and symbol boundaries match.
    symbols = base.symbols(source/"candidate.symbols")
    tables = {"candidate":symbols,"cap32":symbols}
    rows = [json.loads(line) for line in (out/"capacity-measurements.jsonl").read_text().splitlines()]
    assert len(rows)==32 and len({r["pdf_hash"] for r in rows})==1
    by_block = {arm:{} for arm in tables}
    with tarfile.open(out/"capacity-profiles.tar.gz","r:gz") as archive:
        for row in rows:
            stacks, detail = base.stacks(archive,row,tables)
            eval_samples = sum(count for stack,count in stacks.items()
                               if any("::eval_to_module_in_session" in name for name in stack))
            scale = detail["main_ns"] / stacks.total() / 1e6
            by_block[row["arm"]][row["block"]] = dict(period_ns=row["period_ns"],
                eval_ms=eval_samples*scale,cpu_ms=row["cpu_ns"]/1e6)
    result = dict(blocks=by_block, summary={})
    for key in ("eval_ms","cpu_ms"):
        result["summary"][key] = dict(
            arms={arm:base.stats([r[key] for r in table.values()])
                  for arm,table in by_block.items()},
            paired=base.stats([by_block["cap32"][i][key] - r[key] for i,r in by_block["candidate"].items()]))
    (out/"capacity-phases.json").write_text(json.dumps(result,indent=2)+"\n")
    print(result["summary"])


if __name__=="__main__":
    source, out = map(Path,sys.argv[1:])
    for name, data in (("eval-attribution.json",samples(source)),("vanilla-eval.json",vanilla(source))):
        (out/name).write_text(json.dumps(data,indent=2)+"\n")
        print(data.get("results",data.get("phase_ms")))
    if (out/"capacity-end.txt").is_file():
        capacity(source,out)
