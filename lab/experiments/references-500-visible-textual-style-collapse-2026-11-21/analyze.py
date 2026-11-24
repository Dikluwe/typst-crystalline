"""Reduce watch CPU measurements and evaluation stack samples."""
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
ARMS = ("candidate","control","vanilla")


def summarize(rows,key,arms,n):
    groups = {arm:{r["block"]:r[key] for r in rows if r["arm"]==arm} for arm in arms}
    assert all(len(group)==n for group in groups.values())
    return dict(absolute={arm:base.stats(list(group.values())) for arm,group in groups.items()},
                paired={arm:base.stats([v-groups[arm][i] for i,v in groups["candidate"].items()])
                        for arm in arms[1:]})


def main(root):
    rows = [json.loads(line) for line in (root/"measurements.jsonl").read_text().splitlines()]
    assert len(rows)==108 and all(len(r["all_pdfs"])==4 for r in rows)
    assert len({r["pdf_hash"] for r in rows if r["arm"]!="vanilla"})==1
    for row in rows:
        row["cpu_ms"],row["wall_ms"] = row["cpu_ns"]/1e6,row["wall_ns"]/1e6
    tables = {arm:base.symbols(root/f"{arm}.symbols") for arm in ARMS[:2]}
    profile = []
    with tarfile.open(root/"cases.tar.gz","r:gz") as archive:
        for row in rows:
            if not row["period_ns"] or row["arm"]=="vanilla":
                continue
            stacks, detail = base.stacks(archive,row,tables)
            full_eval, group, collapse = 0,0,0
            for stack,n in stacks.items():
                if not any("::eval_to_module_in_session" in frame for frame in stack):
                    continue
                full_eval += n
                if any("::TextualSnapshot::same_group" in frame for frame in stack):
                    group += n
                    if any("::StyleChain::collapse" in frame for frame in stack):
                        collapse += n
            scale = detail["main_ns"]/stacks.total()/1e6
            profile.append(dict(row, **detail, eval_ms=full_eval*scale, group_ms=group*scale,
                                group_collapse_ms=collapse*scale))
    nominal = [r for r in rows if not r["period_ns"]]
    native = json.loads((root/"vanilla-phases.json").read_text())
    result = dict(
        nominal={key:summarize(nominal,key,ARMS,12) for key in ("cpu_ms","wall_ms")},
        sampled_cpu=summarize([r for r in rows if r["period_ns"]],"cpu_ms",ARMS,24),
        tekt_eval={key:summarize(profile,key,ARMS[:2],24) for key in ("eval_ms","group_ms","group_collapse_ms")},
        vanilla_eval=base.stats([r["phase_ms"]["eval"] for r in native["rows"]]),
        by_period={str(p):summarize([r for r in profile if r["period_ns"]==p],"eval_ms",ARMS[:2],12)
                   for p in (577000,937000)},
        profile=profile)
    (root/"timing.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps({k:v for k,v in result.items() if k!="profile"},indent=2))


if __name__=="__main__":
    main(Path(sys.argv[1]))
