"""Capture approved R6 inputs and command receipts; does not judge implementation."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def state():
    return {k: subprocess.check_output(["git", *v], cwd=ROOT, text=True) for k,v in {
        "head": ["rev-parse","HEAD"], "status": ["status","--short"],
        "diff_stat": ["diff","HEAD","--stat"], "diff": ["diff","HEAD","--binary"],
        "staged": ["diff","--cached","--binary"]}.items()}

def read(n):
    return json.loads((D/n).read_text())

def write_new(n, v):
    with (D/n).open("x") as f:
        json.dump(v,f,ensure_ascii=False,indent=2)
        f.write("\n")
    print(json.dumps({"path":str(D/n),"sha256":sha(D/n)}))

if sys.argv[1] == "baseline":
    old=read("p1307-r5-baseline.json")
    review=read("p1307-r5-review.json")
    assert review["verdict"] == "READY_FOR_PUBLIC_API_REVIEW"
    for row in review["contracts"]:
        assert sha(row["path"]) == row["sha256"], row["path"]
    inventory={p:sha(ROOT/p) for p in old["source_inventory"]}
    inventory["00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml"]=sha(ROOT/"00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml")
    for p,digest in old["source_inventory"].items():
        if p.endswith(".rs"):
            assert inventory[p]==digest,p
    protected={p:sha(ROOT/p) for p in old["protected_predecessors"]}
    for p in D.glob("p1307-*"):
        if not p.name.startswith("p1307-r6-"):
            protected[str(p.relative_to(ROOT))]=sha(p)
    write_new("p1307-r6-baseline.json", {
        "schema":"p1307-r6-approved-baseline-v1","at":now(),"state":state(),
        "authorization":{"message":"autorizo","scope":"Concrete R5 API and observable changes plus previously approved R3/R4 encoders and Args","commit_stage_push_delete":False},
        "regime":"executado sem atestacao de isolamento tecnico",
        "review_sha256":sha(D/"p1307-r5-review.json"),
        "source_inventory":inventory,"protected_predecessors":protected,
        "binaries":old["binaries"],
        "roles":{"root":"coordinator/implementation; no oracle or verdict writes after seal",
                 "p1307_oracle":"oracle author; no candidate access; r6-oracle* only",
                 "p1307_contract":"independent tests; baseline and L0 only; r6-tests.patch/note",
                 "p1306_oracle":"verifier/adversary; no judged-input edits; r6-verify/discriminator/seal/verification/certificate"},
        "budget":{"preseal_minutes":15,"same_cause_retries":2,"full_corpus_rule":"focal first; full only after focal success and final gates"},
        "script_sha256":sha(__file__)})
else:
    name=sys.argv[1]
    argv=sys.argv[2:]
    before=state(); start=now(); tick=time.monotonic()
    p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True)
    write_new("p1307-r6-"+name+".json",{"argv":argv,"cwd":str(ROOT),"at":start,"end":now(),
        "seconds":time.monotonic()-tick,"exit":p.returncode,"stdout":p.stdout,"stderr":p.stderr,
        "before":before,"after":state(),"baseline_sha256":sha(D/"p1307-r6-baseline.json")})
    print(p.stdout[-4000:]);print(p.stderr[-4000:])
    raise SystemExit(p.returncode)
