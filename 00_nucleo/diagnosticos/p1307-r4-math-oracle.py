#!/usr/bin/env python3
"""Additive preseal math-call missing oracle; frozen principal runner unchanged."""
import argparse
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location("p1307_r4_frozen",HERE/"p1307-r4-oracle.py")
base=importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
BASE_RUNNER_PIN="ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68"

def cases():
    result=[]
    for fmt in ("json","toml","yaml"):
        for route,expression in {"qualified":f"$std.{fmt}.encode()$", "bare":f"{{ let enc = {fmt}.encode; $enc()$ }}", "bare-with":f"{{ let enc = {fmt}.encode.with(); $enc()$ }}"}.items():
            result.append({"id":f"math.missing.{fmt}.{route}","expression":expression,"route":"eval","source_sha256":base.source_hash(expression),"classes":["math","missing","whole-call",route],"mandatory":True,"policy":"vanilla","expected_shape":"diagnostic","required_message":"missing argument: value"})
    for route,expression in {"qualified":"$std.calc.abs(-1)$", "bare":"{ let enc = calc.abs; $enc(-1)$ }"}.items():
        result.append({"id":f"math.control.{route}","expression":expression,"route":"eval","source_sha256":base.source_hash(expression),"classes":["math","control",route],"mandatory":True,"policy":"baseline","expected_shape":"diagnostic","required_message":"expected integer, float, length, angle, ratio, fraction, or decimal, found content","scope_out":"The mathematical -1 argument is Content, not a code Int. Preserve this existing nonencoder type-error diagnostic as a negative boundary control, not a successful numeric call or general math diagnostic parity claim."})
    return result

def freeze(args):
    assert base.sha(HERE/"p1307-r4-oracle.py")==BASE_RUNNER_PIN
    cs=cases()
    if (HERE/"p1307-r4-math-measurement.json").exists():
        previous=base.read("p1307-r4-math-measurement.json")
        assert previous["focal"]["issues"] and "previous_attempt" not in previous,"focal revision budget exhausted"
        affected=[c for c in cs if c["id"].startswith("math.control.")]
        retry=base.measure(affected,["default"])
        focal=[r for r in previous["focal"]["rows"] if not r["case"].startswith("math.control.")]+retry
        revision={"previous_attempt":previous,"hypothesis":"Numeric-looking math argument -1 is Content. The two controls are valid existing type-error boundaries, not positive numerical calls; preserve their measured baseline debt explicitly. Missing cases remain unchanged.","new_rows":retry}
    else:
        focal=base.measure(cs,["default"]);revision={}
    data={"schema":"p1307-r4-independent-math-measurement-v1","at":base.now(),"baseline_sha256":base.BASELINE_PIN,"script_sha256":base.sha(__file__),"runner_sha256":BASE_RUNNER_PIN,"provenance":base.provenance(),"cases":cs,"focal":{"rows":focal,"issues":base.issues(cs,focal),"finished":base.now()},**revision}
    base.save("p1307-r4-math-measurement.json",data)
    if data["focal"]["issues"]:
        print(json.dumps(data["focal"]["issues"]));return
    orders={}
    for name,ordered in (("normal",cs),("repeat",cs),("reverse",list(reversed(cs)))):
        orders[name]=base.measure(ordered,list(base.PROFILES))
    lookup={(r["case"],r["side"],r["profile"]):r for r in orders["normal"]}
    instability=[{"order":name,"case":r["case"],"side":r["side"],"profile":r["profile"]} for name,rs in orders.items() for r in rs if any(r.get(k)!=lookup[r["case"],r["side"],r["profile"]].get(k) for k in ("returncode","stdout","stderr","observable"))]
    data["matrix"]={"orders":orders,"issues":{name:base.issues(cs,rs) for name,rs in orders.items()},"instability":instability,"finished":base.now()}
    base.save("p1307-r4-math-measurement.json",data)
    assert not instability and not any(data["matrix"]["issues"].values())
    for c in cs:
        c["observations"]={p:base.cell(lookup[c["id"],"vanilla",p]["observable"],lookup[c["id"],"baseline",p]["observable"],c["policy"]) for p in base.PROFILES}
        c["measurement_ref"]={"artifact":"p1307-r4-math-measurement.json","collection":"matrix.orders.normal","case_key":"case","case_id":c["id"],"order":"normal","profile_key":"profile","side_key":"side"}
    obj={"schema":"p1307-r4-additive-math-oracle-v1","regime":"executado sem atestacao de isolamento tecnico","candidate_read":False,"at":base.now(),"inputs":{name:base.sha(HERE/name) for name in ("p1307-r4-oracle.py","p1307-r4-oracle.json","p1307-r4-baseline.json","p1307-r4-call-span-refinement.json","p1307-r4-math-measurement.json")},"script_sha256":base.sha(__file__),"contract_pins":{p:base.sha(base.ROOT/p) for p in ("00_nucleo/prompts/compiler/eval/math.md","00_nucleo/prompts/compiler/eval/call_dispatch.md","00_nucleo/prompts/compiler/stdlib/loading.md")},"profiles":list(base.PROFILES),"cases":cs,"comparison":"Principal frozen runner classify/envelope; complete public values and diagnostics; source SHA recorded for all inputs.","unknown_policy":"Mandatory Unknown blocks; no fake opaque cases.","attack_obligations":[{"owner":"01_core/src/compiler/eval/math.rs","defect":"Omit whole-call transport on qualified math route","witnesses":["math.missing.json.qualified","math.missing.toml.qualified","math.missing.yaml.qualified"]},{"owner":"01_core/src/compiler/eval/math.rs","defect":"Omit transport on bare Func or fail unwrapping With","witnesses":["math.missing.json.bare","math.missing.json.bare-with","math.missing.toml.bare-with","math.missing.yaml.bare-with"]},{"owner":"01_core/src/compiler/eval/math.rs","defect":"Apply encoder-only span transport to other functions or alter their value","witnesses":["math.control.qualified","math.control.bare"]}],"source_mutants_executed":0,"mutation_score":None,"limitations":"Additive math fragment only. Run together with principal 455-case suite; not standalone parity certificate."}
    base.save("p1307-r4-math-oracle.json",obj)
    print(json.dumps({"cases":len(cs),"profile_cells":len(cs)*len(base.PROFILES),"focal_runs":len(focal),"matrix_runs":sum(map(len,orders.values())),"unknowns":[],"instability":instability,"vanilla_default":[{"case":r["case"],"observable":r["observable"]} for r in focal if r["side"]=="vanilla"]},ensure_ascii=False))

def replay(args):
    o=base.read("p1307-r4-math-oracle.json")
    assert base.sha(__file__)==o["script_sha256"]
    for name,pin in o["inputs"].items():assert base.sha(HERE/name)==pin
    cs=list(reversed(o["cases"])) if args.reverse else o["cases"]
    rows=[]
    for c in cs:
        for p in ([args.profile] if args.profile else base.PROFILES):
            row=base.run(args.binary,"candidate",c,p)
            row["verdict"]=base.classify(c["observations"][p]["future_expected"],row["observable"])
            rows.append(row)
    print(json.dumps({"schema":"p1307-r4-math-replay-v1","binary":args.binary,"binary_sha256":base.sha(args.binary),"oracle_sha256":base.sha(HERE/"p1307-r4-math-oracle.json"),"rows":rows,"counts":{v:sum(r["verdict"]==v for r in rows) for v in ("Preserved","Violated","Unknown")}},ensure_ascii=False))

if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("command",choices=["freeze","replay"]);p.add_argument("--binary");p.add_argument("--profile",choices=list(base.PROFILES));p.add_argument("--reverse",action="store_true");a=p.parse_args();{"freeze":freeze,"replay":replay}[a.command](a)
