"""Collect completed P1309 R2 gates with exact source-state provenance."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location("rec",D/"p1309-record-r2.py");rec=importlib.util.module_from_spec(spec);spec.loader.exec_module(rec)
def read(n):return json.loads((D/n).read_text())

def main():
    names=["build-r2","fmt-r2","lint-r2","workspace-tests-r2","diff-check-r2"]
    records={n:read("p1309-"+n+".json")for n in names}
    tests=re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored",records["workspace-tests-r2"]["stdout"])
    totals={k:sum(int(r[i])for r in tests)for i,k in enumerate(("passed","failed","ignored"))}
    ignored=[l for l in records["workspace-tests-r2"]["stdout"].splitlines()if l.startswith("test ")and l.endswith(" ... ignored")]
    lint=records["lint-r2"]["stdout"]+records["lint-r2"]["stderr"]
    severities={s:len(re.findall(r"^"+s+r":",lint,re.M))for s in("error","warning","info")}
    source=rec.inventory();baseline=read("p1309-baseline-r2.json")
    changed=[p for p in source.keys()|baseline["product_inventory"].keys()if source.get(p)!=baseline["product_inventory"].get(p)]
    state=rec.state()
    allowed=[line for line in state["status"].splitlines()if line.startswith("?? 00_nucleo/diagnosticos/p1309-")or line=="?? 00_nucleo/materialization/typst-passo-1309.md"]
    outside=[line for line in state["status"].splitlines()if line not in allowed]
    caches=[str(p)for p in (D/"__pycache__").glob("p1309*.pyc")]
    rec.save("gates",{"schema":"p1309-gates-v1","at":rec.now(),"source_state":state,"manifest_sha256":rec.sha(D/"p1309-manifest-r2.json"),
      "commands":{n:{"path":"00_nucleo/diagnosticos/p1309-"+n+".json","sha256":rec.sha(D/("p1309-"+n+".json")),"exit":r["exit"],"at":r["at"],"end":r["end"]}for n,r in records.items()},
      "workspace_tests":totals,"ignored_test_names":ignored,"lint_severities":severities,"lint_policy":"Warnings/info retained verbatim in linked receipt; zero exit is not zero findings.",
      "product_files_compared":len(source),"product_changes":changed,"outside_write_allowlist":outside,"p1309_cache_files_in_repository":caches,
      "tracked_clean":not state["diff"],"index_clean":not state["staged"],"no_commit":state["head"]==rec.HEAD,
      "pass":all(r["exit"]==0 for r in records.values())and not changed and not outside and not caches and not state["diff"]and not state["staged"],
      "prior_attempt":"Initial attempt invalidated by cache incident; only -r2 gates count. Initial lint/test receipt save failed due argv limit; successor saves via stdin. No success is inferred from those missing initial receipts."})

if __name__=="__main__":main()
