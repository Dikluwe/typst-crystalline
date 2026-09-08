"""Fresh P1309 bilateral observations. No semantic classification or product writes."""
import argparse
import base64
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time
import importlib.util

sys.dont_write_bytecode=True

ROOT=Path(__file__).resolve().parents[2]
D=ROOT/"00_nucleo/diagnosticos"
spec=importlib.util.spec_from_file_location("p1309_record",D/"p1309-record-r2.py")
record=importlib.util.module_from_spec(spec)
spec.loader.exec_module(record)
PROFILES={"default":[],"html":["html"],"a11y":["a11y-extras"],"html+a11y":["html","a11y-extras"]}
SIDES=("vanilla","crystalline")

def features(profile):
    return ["--features",",".join(PROFILES[profile])] if PROFILES[profile] else []

def observe(binary,probe,profile,side,phase,timeout=30):
    expression=probe["expression"]
    argv=[binary["path"],"eval",expression,"--format","json",*features(profile)]
    started=record.now(); tick=time.monotonic(); reason=None
    try:
        p=subprocess.run(argv,cwd=ROOT,capture_output=True,timeout=timeout)
        code,out,err=p.returncode,p.stdout,p.stderr
        if code not in (0,1): reason="CRASH_OR_UNSUPPORTED_EXIT"
    except subprocess.TimeoutExpired as e:
        code,out,err=None,e.stdout or b"",e.stderr or b"";reason="TIMEOUT"
    except OSError as e:
        code,out,err=None,b"",str(e).encode();reason="SPAWN_ERROR"
    value=None
    if code==0:
        try:
            value=json.loads(out.decode("utf-8"),parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))
        except (ValueError,UnicodeError): reason="UNPARSEABLE_JSON"
    elif code==1 and not err.strip(): reason="MISSING_DIAGNOSTIC"
    return {"id":probe["id"],"profile":profile,"side":side,"phase":phase,"features":PROFILES[profile],
      "argv":argv,"cwd":str(ROOT),"expression":expression,"source_sha256":hashlib.sha256(expression.encode()).hexdigest(),
      "binary_path":binary["path"],"binary_sha256":binary["sha256"],"started_at":started,"duration_seconds":time.monotonic()-tick,
      "exit_code":code,"stdout":out.decode("utf-8",errors="replace"),"stderr":err.decode("utf-8",errors="replace"),
      "stdout_base64":base64.b64encode(out).decode(),"stderr_base64":base64.b64encode(err).decode(),
      "stdout_sha256":hashlib.sha256(out).hexdigest(),"stderr_sha256":hashlib.sha256(err).hexdigest(),
      "parsed_value":value,"diagnostics":{"raw_stderr":err.decode("utf-8",errors="replace"),"structured_api_available":False},
      "complete":reason is None,"reason_code":reason}

def classify(v,c):
    if not v["complete"] or not c["complete"]: return "EXECUTION_UNKNOWN"
    if v["exit_code"]==0 and c["exit_code"]==0:
        if v["stdout_base64"]!=c["stdout_base64"]: return "DIFFERENT_VALUE"
        return "MATCH_VALUE" if v["stderr_base64"]==c["stderr_base64"] else "DIFFERENT_DIAGNOSTIC"
    if v["exit_code"]==0: return "VANILLA_ONLY"
    if c["exit_code"]==0: return "CRYSTALLINE_ONLY"
    same=all(v[k]==c[k] for k in ("exit_code","stdout_base64","stderr_base64"))
    return "MATCH_DIAGNOSTIC" if same else "DIFFERENT_DIAGNOSTIC"

def stable_key(row):
    return {"runtime_class":row["runtime_class"],**{s:{k:row[s][k] for k in
       ("exit_code","stdout_base64","stderr_base64","complete","reason_code","source_sha256","binary_sha256","features")} for s in SIDES}}

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("phase",choices=["normal","repeat","reverse"])
    parser.add_argument("--workers",type=int,default=8)
    args=parser.parse_args()
    catalog=json.loads((D/"p1309-probe-catalog.json").read_text())
    baseline=json.loads((D/"p1309-baseline-r2.json").read_text())
    build=json.loads((D/"p1309-build-r2.json").read_text())
    binaries={"vanilla":baseline["vanilla"],"crystalline":build["candidate"]}
    assert build["exit"]==0 and build["before"]["head"]==record.HEAD and not build["before"]["diff"]
    assert binaries["crystalline"]["path"]==record.TARGET+"/release/typst"
    for b in binaries.values(): assert record.sha(b["path"])==b["sha256"]
    assert catalog["profiles"]==PROFILES
    ids=[p["id"]for p in catalog["probes"]];assert ids and len(ids)==len(set(ids))
    old=json.loads((D/"p1304-probe-catalog.json").read_text())["probes"]
    lookup={p["id"]:p for p in catalog["probes"]}
    assert len(old)==627 and all(lookup.get(p["id"])==p for p in old),"Historical probe loss/change"
    before=record.state();assert not before["diff"] and not before["staged"]
    jobs=[(p,profile) for p in sorted(catalog["probes"],key=lambda p:p["id"])for profile in PROFILES]
    if args.phase=="reverse": jobs.reverse()
    tick=time.monotonic();started=record.now()
    def pair(job):
        probe,profile=job
        row={"id":probe["id"],"path":probe["path"],"expression":probe["expression"],"profile":profile,"phase":args.phase,"universe":"principal"}
        order=SIDES if args.phase!="reverse" else tuple(reversed(SIDES))
        for side in order: row[side]=observe(binaries[side],probe,profile,side,args.phase)
        row["runtime_class"]=classify(row["vanilla"],row["crystalline"])
        return row
    rows=[]
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        for row in pool.map(pair,jobs):
            rows.append(row)
            if len(rows)%500==0: print(json.dumps({"phase":args.phase,"pairs":len(rows),"total":len(jobs)}),flush=True)
    for b in binaries.values(): assert record.sha(b["path"])==b["sha256"]
    after=record.state();assert before["head"]==after["head"] and not after["diff"] and not after["staged"]
    counts=dict(Counter(r["runtime_class"] for r in rows))
    record.save("matrix-"+args.phase,{"schema":"p1309-global-matrix-v1","at":started,"end":record.now(),"seconds":time.monotonic()-tick,
      "manifest_sha256":record.sha(D/"p1309-manifest-r2.json"),"catalog_sha256":record.sha(D/"p1309-probe-catalog.json"),"runner_sha256":record.sha(__file__),
      "build_receipt_sha256":record.sha(D/"p1309-build-r2.json"),"binaries":binaries,"before":before,"after":after,
      "execution_order":"Canonical ID/profile submission; bounded independent process pairs; output retained in submission order. Reverse reverses full submission sequence and side order.",
      "workers":args.workers,"probes":len(ids),"pairs":len(rows),"counts":counts,"results":rows,
      "comparison":"Exact stdout and stderr bytes, with successful JSON parse required. No warning suppression or diagnostic normalization."})
    print(json.dumps(counts))

if __name__=="__main__":main()
