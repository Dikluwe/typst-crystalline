#!/usr/bin/env python3
"""E final offline replay: reconstruct mutation inputs, authenticate receipts."""
import sys
sys.dont_write_bytecode = True
import copy
import hashlib
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
pins, failures, checks = {}, [], []
def sha(b): return hashlib.sha256(b).hexdigest()
def canon(v): return json.dumps(v,sort_keys=True,ensure_ascii=False,separators=(",",":"),allow_nan=False).encode()
def check(name,ok,witness=None):
    checks.append(name)
    if not ok: failures.append({"name":name,"witness":witness})
def load(name,expected=None):
    raw=(D/name).read_bytes();pins[name]=sha(raw)
    if expected: check("pin "+name,pins[name]==expected)
    return json.loads(raw)
adv=load("p1309-adversarial-ledger.json","1c693fffb797a0b49d69b8ac62e8de9f815aa68dc73b03c6920aaa0c97cf351d")
cases=load("p1309-adversarial-cases.json","e8663dc60676d92867056de5b34b9e67e5fed0f64e8c809d2d8106682ef68cad")
bundle=load("p1309-audit-bundle.json","eeaa851c41ce226436ccb6ef729aacac61e59bdaed444927d7d894ca4dd3e4b2")
check("case source bundle",cases["bundle_sha256"]==pins["p1309-audit-bundle.json"])
for path,h in adv["input_sha256"].items(): check("D input remains sealed "+path,sha(Path(path).read_bytes())==h)
cached={k:canon(v) for k,v in bundle.items()}
def root_hash(value,changed):
    return sha(b"{"+b",".join(canon(k)+b":"+(canon(value[k]) if k in changed else cached[k]) for k in sorted(value))+b"}")
def apply(value,operations):
    roots={op["path"].split("/")[1].replace("~1","/").replace("~0","~") for op in operations}
    result=dict(value)
    for k in roots:result[k]=copy.deepcopy(value[k])
    changes=[]
    for op in operations:
        route=[x.replace("~1","/").replace("~0","~") for x in op["path"][1:].split("/")]
        parent=result
        for token in route[:-1]:parent=parent[int(token)] if isinstance(parent,list) else parent[token]
        key=int(route[-1]) if isinstance(parent,list) and route[-1]!="-" else route[-1]
        if op["op"]=="test":
            check("mutation witness test "+op["path"],parent[key]==op["value"])
            continue
        before=copy.deepcopy(parent[key]) if key!="-" else None
        changes.append({"before":before,"operation":op})
        if op["op"]=="remove":del parent[key]
        elif op["op"]=="replace":parent[key]=copy.deepcopy(op["value"])
        elif op["op"]=="add":
            if isinstance(parent,list):parent.insert(len(parent) if key=="-" else key,copy.deepcopy(op["value"]))
            else:parent[key]=copy.deepcopy(op["value"])
        else:raise ValueError(op["op"])
    return result,changes,roots
def receipt(name,r,expected_hash,positive,target_codes=None):
    check(name+" canonical input",r["bundle_canonical_sha256"]==expected_hash)
    check(name+" successful actual evaluator",r["exit"]==0 and not r.get("execution_unknown") and "-B" in r["argv"] and "--evaluate" in r["argv"])
    check(name+" stdout witness parsed",json.loads(r["stdout"])["failures"]==r["failures"])
    check(name+" discriminates",not r["failures"] if positive else bool({f["code"] for f in r["failures"]}&set(target_codes)))
receipt("full positive",adv["base_control"],root_hash(bundle,set()),True)
check("18 distinct attacks",len(adv["attacks"])==len(cases["cases"])==18 and {r["id"] for r in adv["attacks"]}=={f"A{i:02}" for i in range(1,19)})
case_by_id={c["id"]:c for c in cases["cases"]}
witnesses=[]
for attack in adv["attacks"]:
    case=case_by_id[attack["id"]]
    positive,changes,roots=apply(bundle,case.get("control_operations",[]))
    check(attack["id"]+" exact control operations",changes==attack["control_operations"])
    positive_hash=root_hash(positive,roots)
    receipt(attack["id"]+" positive",attack["positive"],positive_hash,True)
    negative,changes,mutation_roots=apply(positive,case["operations"])
    check(attack["id"]+" exact mutation operations",changes==attack["mutation_operations"])
    negative_hash=root_hash(negative,roots|mutation_roots)
    check(attack["id"]+" nontrivial mutation",positive_hash!=negative_hash)
    check(attack["id"]+" frozen anchors unmodified",positive["anchors"]==negative["anchors"]==bundle["anchors"])
    receipt(attack["id"]+" negative",attack["negative"],negative_hash,False,case["target_codes"])
    check(attack["id"]+" status witness",attack["status"]=="REJECTED_WITH_WITNESS" and attack["witness"]==case["witness"] and attack["target_codes"]==case["target_codes"])
    witnesses.append({"id":attack["id"],"description":attack["description"],"target_codes":case["target_codes"],"observed_codes":[f["code"] for f in attack["negative"]["failures"]],"positive_canonical_sha256":positive_hash,"negative_canonical_sha256":negative_hash})
check("D score arithmetic",adv["rejected"]==18 and adv["survived"]==0 and adv["mutation_score"]==18/18)
for name in ("p1309-verification-preliminary.json","p1309-verification-runtime.json","p1309-verification-semantic.json"):
    r=load(name)
    check("predecessor checks "+name,not r.get("failures",r.get("failed",[])))
    for path,h in r["pins"].items():check("predecessor input "+path,sha((D/path).read_bytes())==h)
baseline=load("p1309-baseline-r2.json")
changes=[p for p,h in baseline["product_inventory"].items() if sha((ROOT/p).read_bytes())!=h]
check("final product unchanged",not changes,changes)
git=lambda *a:subprocess.check_output(["git",*a],cwd=ROOT,text=True)
status=git("status","--short")
outside=[r for r in status.splitlines() if not r[3:].startswith("00_nucleo/diagnosticos/p1309-") and r[3:] not in baseline["initial_user_untracked"]]
check("final allowlist",not outside,outside)
check("final tracked and index clean",not git("diff","HEAD","--stat") and not git("diff","--cached","--stat"))
check("final head",git("rev-parse","HEAD").strip()==baseline["state"]["head"])
for b in (baseline["vanilla"],load("p1309-build-r2.json")["candidate"]):check("final binary "+b["path"],sha(Path(b["path"]).read_bytes())==b["sha256"])
incident=load("p1309-inventory-incident.json")
check("incident preserved cache",sha(Path(incident["recovery"]["recoverable_path"]).read_bytes())==incident["sha256"] and not (ROOT/incident["out_of_allowlist_path"]).exists())
debt=load("p1309-certification-debt.json")
check("product certification debt remains separate",len(debt["families"])==37 and debt["p1307_mutation_score"] is None and adv["policy"]["product_mutations_p1307_executed"]==0)
print(json.dumps({"schema":"p1309-final-independent-checks-v1","role":"E","at":datetime.now(timezone.utc).isoformat(),"head":baseline["state"]["head"],"manifest_sha256":adv["manifest_r2_sha256"],"pins":pins,"checks":len(checks),"failures":failures,"attack_witnesses":witnesses,"valid_attacks":18,"rejected_attacks":18,"positive_controls":19,"mutation_score":1.0,"product_mutants_executed":0,"product_files_compared":len(baseline["product_inventory"]),"git_diff_HEAD_stat":git("diff","HEAD","--stat"),"git_status_short":status,"outside_allowlist":outside,"method":"Offline independent reconstruction of every JSON mutation and canonical evaluator-input hash; authenticate actual positive and negative receipts; no rerun of the attacked evaluator or product matrices.","regime":"executado sem atestação de isolamento técnico","verdict":"P1309_PASS_P1310_COHORT_SELECTED" if not failures else "P1309_BLOCKED_AUDIT_INTEGRITY"},ensure_ascii=False,indent=2))
