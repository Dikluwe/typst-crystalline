#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
for n in $(seq 1 240); do
    if test -s "$d/verification.json" && test -s "$d/fresh/probe-True/comment-2.pdf"; then
        break
    fi
    sleep 1
done
test -s "$d/verification.json"
ram="$(mktemp -d /dev/shm/tekt-pdf-cpu-overhead.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
export TEKT_LAB_WORK="$ram" TMPDIR="$ram"
findmnt -T "$ram" > "$d/overhead-ram-mount.txt"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/candidate.elf" "$ram/probe"
python3 - "$d" <<'PY'
from pathlib import Path
import importlib.util,json,os,sys
d=Path(sys.argv[1])
spec=importlib.util.spec_from_file_location("pdf_cost_runner",d/"measure.py")
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
ENGINES,WORK,bootstrap,measure,runner,run_watch=(m.ENGINES,m.WORK,m.bootstrap,m.measure,m.runner,m.run_watch)
os.sched_setaffinity(0,{2})
ENGINES["quiet"]=ENGINES["probe"]
digest=measure.sha(ENGINES["probe"])
assert digest == (d/"candidate.sha256").read_text().strip()
rows=[]
for block in range(12):
    for arm in (("probe","quiet") if block%2==0 else ("quiet","probe")):
        case=WORK/f"{block}-{arm}"
        current=run_watch(case,arm,True)
        rows.extend(dict(block=block,**row) for row in current)
        if block==0:
            for row in current:
                fresh=d/"fresh"/"probe-True"/(row["state"]+".pdf")
                assert runner.describe(WORK/row["path"])==runner.describe(fresh)
summary={}
for state in ("comment","comment-2"):
    select=lambda arm: [r["cpu_ns"]/1e6 for r in rows if r["engine"]==arm and r["state"]==state]
    active, quiet=select("probe"),select("quiet")
    summary[state]=dict(delta_ms=bootstrap([a-b for a,b in zip(active,quiet)]),
                        ratio=bootstrap([a/b for a,b in zip(active,quiet)]))
measure.write_json(d/"instrumentation-overhead.json",dict(utc=measure.utc(),elf=digest,blocks=12,
             script_sha256=measure.sha(d/"check-overhead.sh"),rows=rows,summary=summary))
print(json.dumps(summary,indent=2))
PY
