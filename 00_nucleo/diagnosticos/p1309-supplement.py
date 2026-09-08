"""Fresh module, repr-integrity and channel sentinels, counted outside catalog."""
import base64
from concurrent.futures import ThreadPoolExecutor
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
record=load("p1309_record",D/"p1309-record-r2.py")
oracle=load("p1306_frozen",D/"p1306-oracle.py")
matrix=load("p1309_matrix",D/"p1309-matrix.py")

def main():
    start=record.now();before=record.state()
    build=json.loads((D/"p1309-build-r2.json").read_text())
    baseline=json.loads((D/"p1309-baseline-r2.json").read_text())
    binaries={"crystalline":build["candidate"],"vanilla":baseline["vanilla"]}
    fixture=Path(tempfile.mkdtemp(prefix="p1309-sentinels-"))
    for path,body in oracle.FIXTURES.items():
        patch="*** Begin Patch\n*** Add File: "+str(fixture/path)+"\n"+"".join("+"+line+"\n"for line in body.splitlines())+"*** End Patch\n"
        subprocess.run(["apply_patch"],input=patch,text=True,check=True,capture_output=True)
    cases=[{**c,"id":"p1309-module-"+c["id"],"family":"module"}for c in oracle.cases()]
    for n in (39,40,41,42,81,256):
        cases.append({"id":f"p1309-array-{n}","expression":f"{{ let a = range({n}); (a.len(), a, repr(a)) }}","family":"array-integrity","length":n})
    # The step explicitly includes all seven boundary sizes; 39..42 are four,
    # followed by 81 and 256: six sizes, not seven.
    cases.append({"id":"p1309-small-large-map","expression":"(repr(std), repr(color.map), repr(color.map.viridis.slice(0, 2)), repr(color.map.viridis))","family":"repr"})
    for path in ("csv.encode","xml.encode","read.encode"):
        cases.append({"id":"p1309-negative-"+path,"expression":f"repr(type({path}))","family":"negative-encoder"})
    jobs=[(c,p)for c in cases for p in matrix.PROFILES]
    rows=[]
    for phase in ("normal","repeat","reverse"):
        ordered=jobs if phase!="reverse"else list(reversed(jobs))
        def pair(job):
            c,profile=job;row={"id":c["id"],"profile":profile,"expression":c["expression"],"family":c["family"],"phase":phase,"universe":"supplement"}
            for side in ("vanilla","crystalline"):
                o=oracle.execute(binaries[side]["path"],c,profile,phase,side,fixture)
                reason=None
                if not o["complete"]or o["exit_code"]not in(0,1):reason="EXECUTION_FAILURE"
                if o["exit_code"]==0:
                    try:json.loads(o["stdout"])
                    except ValueError:reason="UNPARSEABLE_JSON"
                elif not o["stderr"].strip():reason="MISSING_DIAGNOSTIC"
                o.update(binary_path=binaries[side]["path"],binary_sha256=binaries[side]["sha256"],reason_code=reason,
                  complete=reason is None,features=matrix.PROFILES[profile],source_sha256=hashlib.sha256(c["expression"].encode()).hexdigest())
                row[side]=o
            row["runtime_class"]=matrix.classify(row["vanilla"],row["crystalline"])
            return row
        with ThreadPoolExecutor(max_workers=8)as pool:rows.extend(pool.map(pair,ordered))
    for b in binaries.values():assert record.sha(b["path"])==b["sha256"]
    record.save("sentinels-extra",{"schema":"p1309-extra-sentinels-v1","at":start,"end":record.now(),"before":before,"after":record.state(),
       "manifest_sha256":record.sha(D/"p1309-manifest-r2.json"),"runner_sha256":record.sha(__file__),"frozen_module_oracle_sha256":record.sha(D/"p1306-oracle.py"),
       "fixtures":oracle.FIXTURES,"fixture_root":str(fixture),"binaries":binaries,"cases":cases,"rows":rows,
       "comparison":"Raw stdout/stderr retained, common physical fixtures across both sides and all orders; known debts remain differences."})

if __name__=="__main__":main()
