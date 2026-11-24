"""Reduce the four-arm comparison without mixing sampled and nominal times."""
import importlib.util
import json
from pathlib import Path
from statistics import median
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "decoder",D.parent/"references-500-visible-edit-phases-2026-11-21/analyze.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
ARMS = ("cap16","cap32","memory","vanilla")


def comparisons(rows,key,arms):
    by_arm = {arm:{r["block"]:r[key] for r in rows if r["arm"]==arm} for arm in arms}
    assert all(len(data)==16 for data in by_arm.values())
    return dict(absolute={arm:base.stats(list(data.values())) for arm,data in by_arm.items()},
                paired={arm:base.stats([value-by_arm[arm][block] for block,value in by_arm["memory"].items()])
                        for arm in arms if arm!="memory"})


def main(result):
    rows = [json.loads(l) for l in (result/"measurements.jsonl").read_text().splitlines()]
    assert len(rows)==128
    assert all(len(r["all_pdfs"])==4 for r in rows)
    for arm in ARMS:
        totals = {tuple(r["all_pdfs"].values()) for r in rows if r["arm"]==arm}
        assert len(totals)==1, arm
    for r in rows:
        r["cpu_ms"], r["wall_ms"] = r["cpu_ns"]/1e6,r["wall_ns"]/1e6
    nominal = [r for r in rows if not r["period_ns"]]
    profile = []
    # Both lab patches preserve the addresses and lengths of all symbols.
    control = result.parent/"references-500-visible-pre-pdf-2026-11-21"
    receipt = json.loads((result/"variants.json").read_text())
    assert receipt["cap16"]["sha256"] == (control/"binaries.sha256").read_text().split()[0]
    symbols = base.symbols(control/"candidate.symbols")
    tables = {arm:symbols for arm in ARMS if arm!="vanilla"}
    with tarfile.open(result/"cases.tar.gz","r:gz") as archive:
        for row in rows:
            if not row["period_ns"] or row["arm"]=="vanilla":
                continue
            stacks, detail = base.stacks(archive,row,tables)
            evaluation = sum(n for stack,n in stacks.items()
                             if any("::eval_to_module_in_session" in frame for frame in stack))
            scale = detail["main_ns"]/stacks.total()/1e6
            profile.append(dict(row, **detail, eval_ms=evaluation*scale, eval_samples=evaluation))
    native = json.loads((result/"vanilla-phases.json").read_text())
    assert len(native["rows"])==12
    summary = dict(nominal={key:comparisons(nominal,key,ARMS) for key in ("cpu_ms","wall_ms")},
                   tekt_eval=comparisons(profile,"eval_ms",tuple(tables)),
                   vanilla_eval=base.stats([r["phase_ms"]["eval"] for r in native["rows"]]),
                   period_medians={str(p):{arm:median(r["eval_ms"] for r in profile if r["arm"]==arm and r["period_ns"]==p)
                                           for arm in tables} for p in (577000,937000)})
    (result/"timing.json").write_text(json.dumps(summary|dict(profile=profile),indent=2)+"\n")
    print(json.dumps(summary,indent=2))


if __name__=="__main__":
    main(Path(sys.argv[1]))
