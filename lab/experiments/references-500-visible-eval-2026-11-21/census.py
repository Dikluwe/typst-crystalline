"""Count loop execution and actual memo outcomes on the frozen stable-CID ELF."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "original_watch",D.parent/"references-500-visible-pre-pdf-2026-11-21/sample.py")
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)
case = fixture.WORK
exe = fixture.EXES["candidate"]
assert fixture.measure.sha(exe) == "82d2daf8fdcb6788e59932e5b85658c495354a342d729fd6cf620d45a9c2935b"
(case/"edits").mkdir()
for i,(_,source,leaf) in enumerate(fixture.STATES):
    assert leaf is None
    (case/("main.typ" if i==0 else f"edits/{i}.typ")).write_bytes(source)
os.sched_setaffinity(0,{2})
args=["taskset","-c","1","gdb","-q","-nx","-batch","-x",str(D/"census.gdb"),
      "--args",str(exe),"watch","main.typ","out.pdf","--root",str(case),"--pdf-tagged=true"]
env = fixture.cycle.environment(case,"candidate",False)
if "TEKT_EVAL_CAPACITY" in os.environ:
    env["TEKT_EVAL_CAPACITY"] = os.environ["TEKT_EVAL_CAPACITY"]
with (case/"gdb.log").open("wb") as log:
    subprocess.run(args,cwd=case,env=env,
                   stdout=log,stderr=subprocess.STDOUT,timeout=60,check=True)
for i,(name,_,_) in enumerate(fixture.STATES):
    assert fixture.cycle.cycle.key("candidate",case/f"pub-{i}.pdf")==fixture.golden("candidate",name)
rows_by_publication = json.loads((case/"census.json").read_text())
if env.get("TEKT_EVAL_CAPACITY") == "32":
    assert rows_by_publication[0] and all(not r["evictions"] for r in rows_by_publication[0])
for rows in rows_by_publication:
    print("loops",len(rows), "body",sum(r["body"] for r in rows),
          "miss",sum(r["miss"] for r in rows),"hit",sum(r["hit"] for r in rows),
          "store",sum(r["store"] for r in rows), "evict",sum(len(r["evictions"]) for r in rows))
