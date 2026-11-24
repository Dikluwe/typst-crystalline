"""Measure the exclusive evaluation scope and paired wall/CPU costs."""
import importlib.util
import json
from pathlib import Path
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "profile",D.parent/"references-500-visible-edit-phases-2026-11-21/analyze.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
ARMS = ("lru8", "protect8", "lru2_5", "protect2_5", "vanilla")
PAIRS = (("protect8","lru8"),("protect2_5","lru2_5"),
         ("protect8","vanilla"),("protect2_5","vanilla"))


def summarize(rows, key, count, arms):
    groups = {arm:{r["block"]:r[key] for r in rows if r["arm"]==arm} for arm in arms}
    assert all(len(b)==count for b in groups.values()), groups
    return dict(absolute={arm:base.stats(list(v.values())) for arm,v in groups.items()},
                paired={f"{a}-{b}":base.stats([n-groups[b][i] for i,n in groups[a].items()])
                        for a,b in PAIRS if a in groups and b in groups})


def main(root):
    rows = [json.loads(l) for l in (root/"measurements.jsonl").read_text().splitlines()]
    assert len(rows)==150
    assert all(len(r["all_pdfs"])==4 for r in rows)
    for arm in ARMS:
        assert len({tuple(r["all_pdfs"].values()) for r in rows if r["arm"]==arm})==1
    assert len({r["pdf_hash"] for r in rows if r["arm"]!="vanilla"})==1
    for row in rows:
        row["cpu_ms"], row["wall_ms"] = row["cpu_ns"]/1e6, row["wall_ns"]/1e6
    symbols = base.symbols(root/"candidate.symbols")
    tekt = ARMS[:-1]
    tables = {arm:symbols for arm in tekt}
    nominal = [r for r in rows if r["period_ns"]==0]
    profile = []
    with tarfile.open(root/"cases.tar.gz","r:gz") as archive:
        for row in rows:
            if not row["period_ns"] or row["arm"]=="vanilla":
                continue
            stacks, detail = base.stacks(archive,row,tables)
            count = sum(n for stack,n in stacks.items()
                        if any("::eval_to_module_in_session" in frame for frame in stack))
            eval_ms = detail["main_ns"]/stacks.total()/1e6*count
            profile.append(dict(row, **detail, eval_ms=eval_ms, eval_samples=count))
    vanilla = json.loads((root/"vanilla-phases.json").read_text())
    result = dict(
        nominal={key:summarize(nominal,key,10,ARMS) for key in ("cpu_ms","wall_ms")},
        sampled_cpu=summarize([row for row in rows if row["period_ns"]],"cpu_ms",20,ARMS),
        tekt_eval=summarize(profile,"eval_ms",20,tekt),
        vanilla_eval=base.stats([r["phase_ms"]["eval"] for r in vanilla["rows"]]),
        by_period={str(p):summarize([r for r in profile if r["period_ns"]==p],"eval_ms",10,tekt)
                   for p in (577000,937000)},
        profile=profile)
    (root/"timing.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps({k:v for k,v in result.items() if k!="profile"},indent=2))


if __name__=="__main__":
    main(Path(sys.argv[1]))
