"""Assemble fresh observations; never change historical expected values."""
import base64
from collections import Counter
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
record=load("rec",D/"p1309-record-r2.py")
module=load("modoracle",D/"p1306-oracle.py")
matrix=load("matrix",D/"p1309-matrix.py")
def read(n):return json.loads((D/n).read_text())
def ref(n):return {"path":"00_nucleo/diagnosticos/"+n,"sha256":record.sha(D/n)}
def envelope_class(v,c):
    if v.get("kind")=="unknown"or c.get("kind")=="unknown":return"EXECUTION_UNKNOWN"
    if v==c:return "MATCH_VALUE"if v["kind"]=="value"else"MATCH_DIAGNOSTIC"
    if v["kind"]=="value"and c["kind"]!="value":return"VANILLA_ONLY"
    if c["kind"]=="value"and v["kind"]!="value":return"CRYSTALLINE_ONLY"
    return"DIFFERENT_VALUE"if v["kind"]=="value"else"DIFFERENT_DIAGNOSTIC"

def sentinels():
    names={s:"p1309-sentinels-p1308-"+s+".json"for s in ("vanilla","crystalline")}
    payload={s:json.loads(read(n)["stdout"])for s,n in names.items()}
    sides={s:{(r["case"],r["profile"]):r for r in p["rows"]}for s,p in payload.items()}
    assert sides["vanilla"].keys()==sides["crystalline"].keys()
    rows=[]
    for key,c in sides["crystalline"].items():
        v=sides["vanilla"][key]
        row={"id":key[0],"profile":key[1],"universe":"supplement","family":"p1307-p1308-contract", "vanilla":v,"crystalline":c,
          "bilateral_class":envelope_class(v["observable"],c["observable"]),"preservation":c["verdict"]}
        rows.append(row)
    extra=read("p1309-sentinels-extra.json")
    frozen_module=read("p1306-oracle-cases.json")
    expected={(r["id"],r["profile"]):r["expected"]for r in frozen_module["expectations"]}
    modules=[];arrays=[]
    for row in extra["rows"]:
        if row["phase"]!="normal":continue
        canonical={**row,"bilateral_class":row["runtime_class"]}
        key=(row["id"].removeprefix("p1309-module-"),row["profile"])
        if key in expected:
            result=module.discriminate(expected[key],row["crystalline"])
            canonical["preservation"]=result["verdict"]
            modules.append({"id":row["id"],"profile":row["profile"],**result})
        if row["family"]=="array-integrity":
            for side in("vanilla","crystalline"):
                value=json.loads(row[side]["stdout"])
                n=int(row["id"].rsplit("-",1)[1])
                arrays.append({"id":row["id"],"profile":row["profile"],"side":side,"length":n,"values":value[1],"valid":value[:2]==[n,list(range(n))]})
        rows.append(canonical)
    extra_maps={p:{(r["id"],r["profile"]):matrix.stable_key(r)for r in extra["rows"]if r["phase"]==p}for p in("normal","repeat","reverse")}
    record.save("sentinels",{"schema":"p1309-sentinels-v1","at":record.now(),"head":record.HEAD,"tracked_diff_stat":record.state()["diff_stat"],
      "manifest_sha256":record.sha(D/"p1309-manifest-r2.json"),"inputs":[ref(n)for n in[*names.values(),"p1309-sentinels-extra.json","p1308-r2-oracle.json","p1306-oracle-cases.json"]],
      "rows":rows,"pairs":len(rows),"bilateral_counts":dict(Counter(r["bilateral_class"]for r in rows)),
      "p1308_preservation_counts":payload["crystalline"]["counts"],"module_preservation":modules,
      "module_preservation_counts":dict(Counter(r["verdict"]for r in modules)),"array_integrity":arrays,
      "extra_three_order_stable":extra_maps["normal"]==extra_maps["repeat"]==extra_maps["reverse"],
      "separate_universe":True,"principal_probe_contribution":0,
      "comparison":"P1307/P1308 uses the explicitly frozen language envelope (including error messages/hints/ranges/traces); raw stderr/stdout retained separately. Module/array supplement uses raw bilateral channels. Preservation against an authorized crystalline baseline is separate from bilateral equality; warnings never deleted from raw transcripts."})

def stability():
    phases=("normal","repeat","reverse")
    inputs={p:read("p1309-matrix-"+p+".json")for p in phases}
    maps={p:{(r["id"],r["profile"]):matrix.stable_key(r)for r in inputs[p]["results"]}for p in phases}
    failures=[]
    for p in phases[1:]:
        for key in maps["normal"].keys()|maps[p].keys():
            if maps["normal"].get(key)!=maps[p].get(key):failures.append({"phase":p,"key":key})
    unknowns=[{"phase":p,"id":r["id"],"profile":r["profile"]}for p in phases for r in inputs[p]["results"]if r["runtime_class"]=="EXECUTION_UNKNOWN"]
    record.save("stability",{"schema":"p1309-stability-v1","at":record.now(),"head":record.HEAD,"tracked_diff_stat":record.state()["diff_stat"],
      "manifest_sha256":record.sha(D/"p1309-manifest-r2.json"),"matrices":[ref("p1309-matrix-"+p+".json")for p in phases],
      "pairs_by_phase":{p:len(maps[p])for p in phases},"equal":not failures,"differences":failures,"unknowns":unknowns,"classification_ready":not failures and not unknowns,
      "comparison":"Exact per-key raw stdout/stderr bytes, exit, completeness, reason, source hash, binary hash and feature vector; times/order logistics excluded."})

if __name__=="__main__":globals()[sys.argv[1]]()
