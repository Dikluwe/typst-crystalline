"""P1309 read-only product provenance and bounded command receipts."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
STEP = ROOT / "00_nucleo/materialization/typst-passo-1309.md"
HEAD = "eb24cd657fc2333dc7ea5393f7cfebf8c7192d39"
TARGET = "/dev/shm/p1309-r2-target.R2ZoSj"
VANILLA = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT).decode()

def state():
    values = {"head": git("rev-parse", "HEAD").strip(), "branch": git("branch", "--show-current").strip(),
              "status": git("status", "--short", "--untracked-files=all"),
              "diff_stat": git("diff", "HEAD", "--stat"), "diff": git("diff", "HEAD", "--binary"),
              "staged": git("diff", "--cached", "--binary")}
    return {**values, "sha256": {k: hashlib.sha256(v.encode()).hexdigest() for k,v in values.items()}}

def save(name, data):
    path = D / ("p1309-" + name + ".json")
    if path.exists():
        raise FileExistsError(path)
    payload = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n" + "".join("+"+line+"\n" for line in payload.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, cwd=ROOT, check=True, capture_output=True)
    print(json.dumps({"path":str(path),"sha256":sha(path)}),flush=True)

def inventory():
    paths = git("ls-files", "-z", "00_nucleo/prompts", "01_core", "02_shell", "03_infra", "04_wiring", "benches", "Cargo.toml", "Cargo.lock").split("\0")
    return {p:sha(ROOT/p) for p in paths if p and (ROOT/p).is_file()}

def init():
    assert not (D/"__pycache__/p1309-inventory-runner.cpython-312.pyc").exists()
    s=state()
    assert s["head"]==HEAD and not s["diff"] and not s["staged"], "Unclean or wrong baseline"
    assert sha("/usr/local/bin/typst")==VANILLA
    inputs={
      "p1299-probe-catalog.json":"649dd46f05e67d376a8096756a31207a6a7d4087acf5fcbdd6688a59c4f934ae",
      "p1304-owner-ledger.tsv":"01e038177dde246c3196af79bdb36eb13eb2e03364b0b091056cb2af8f4d6770",
      "p1304-decision-report.md":"92e7ecf2ac895c56831a0d4619ba79d703372cff029885bbe1101511f13425bd",
      "p1304-encode-readiness.json":"156f56b9f65ad5fd4310aa7785d4667c1f2c20e13f3f5c8be43b280700e7b950",
      "p1308-r2-final-report.md":"fc114f4892545ecc815e0b252f0190dcb27f569d9044ba4e34640a5b9804568a",
      "p1308-verification-r2-final.json":"6f6d9cfd4d3cb8233915c0cf59f9734a1b021139486e09fb743f2646fa5c37c0"}
    for p,h in inputs.items():
        assert sha(D/p)==h, p
    untracked=git("ls-files","--others","--exclude-standard","-z").split("\0")
    untracked={p:sha(ROOT/p) for p in untracked if p and (p==str(STEP.relative_to(ROOT)) or p.startswith("00_nucleo/diagnosticos/p1309-"))}
    save("baseline-r2",{"schema":"p1309-baseline-r2-v1","supersedes_invalid_attempt":sha(D/"p1309-baseline.json"),"incident_sha256":sha(D/"p1309-inventory-incident.json"),"at":now(),"local":datetime.datetime.now().astimezone().isoformat(),
       "state":s,"product_inventory":inventory(),"initial_user_untracked":[str(STEP.relative_to(ROOT))],
       "untracked_at_freeze":untracked,"inputs":inputs,"step_sha256":sha(STEP),"recorder_sha256":sha(__file__),
       "vanilla":{"path":"/usr/local/bin/typst","sha256":VANILLA,"upstream":"a51e02804"},
       "fresh_target":TARGET,"candidate_reuse_forbidden":True,
       "unknown_policy":"timeout/crash/unparseable output/fixture or binary mismatch/missing mandatory observable => EXECUTION_UNKNOWN; blocks classification and selection"})
    save("manifest-r2",{"schema":"p1309-manifest-r2-v1","supersedes_invalid_attempt":sha(D/"p1309-manifest.json"),"retry_policy":"one clean successor after restored cache incident; a second allowlist leak blocks; no earlier receipt is a valid R2 gate","at":now(),"baseline_sha256":sha(D/"p1309-baseline-r2.json"),
      "step":{"path":str(STEP.relative_to(ROOT)),"sha256":sha(STEP)},"regime":"executado sem atestação de isolamento técnico",
      "roles":{"A":{"executor":"/root/p1309_inventory","inherited_context":"none; task instructions only","write":"p1309-inventory-* and p1309-probe-catalog.json","read":"step, skill, historical method/catalog, pinned binaries; no candidate matrix"},
       "B":{"executor":"/root","inherited_context":"conversation history; coordinator","write":"p1309 provenance, runners, matrices, sentinels, gates, final report","read":"step, frozen catalog, binaries, prior methods and sealed artifacts"},
       "C":{"executor":"/root/p1309_classifier","inherited_context":"none; task instructions only","write":"p1309 ledgers, classification, selection, certification debt","read":"step, catalog, matrices, relevant full L0 and source"},
       "D":{"executor":"/root/p1309_adversary","inherited_context":"none; task instructions only","write":"p1309-adversarial-* and p1309-audit-attacks.py","read":"step, frozen artifacts and historical auditor methods"},
       "E":{"executor":"separate fresh verifier after A finishes","inherited_context":"none","write":"p1309-verification.json and p1309-certificate.json","read":"sealed artifacts and receipts only; cannot edit judged artifacts"}},
      "policy":{"product_read_only":True,"no_commit":True,"allowed_writes":"00_nucleo/diagnosticos/p1309-*; disposable dedicated build target outside repo",
                "command_timeout_seconds":1200,"probe_timeout_seconds":30,"full_orders":["normal","repeat","reverse"],
                "calibration_limit":"two focal revisions per failure class; two without gain stop, never turn Unknown into success"}})

def command(name,argv):
    before=state(); start=now(); tick=time.monotonic()
    env={"CARGO_TARGET_DIR":TARGET,"PYTHONDONTWRITEBYTECODE":"1"}
    p=subprocess.run(argv,cwd=ROOT,env={**os.environ,**env},capture_output=True,text=True,timeout=1200)
    data={"schema":"p1309-command-v1","at":start,"end":now(),"seconds":time.monotonic()-tick,"argv":argv,"cwd":str(ROOT),"env":env,
          "exit":p.returncode,"stdout":p.stdout,"stderr":p.stderr,"before":before,"after":state(),
          "manifest_sha256":sha(D/"p1309-manifest-r2.json"),"recorder_sha256":sha(__file__)}
    binary=Path(TARGET)/"release/typst"
    if binary.exists(): data["candidate"]={"path":str(binary),"sha256":sha(binary)}
    save(name,data)
    print(p.stdout[-1200:]);print(p.stderr[-1200:])
    raise SystemExit(p.returncode)

if __name__=="__main__":
    if sys.argv[1]=="init": init()
    else: command(sys.argv[1],sys.argv[2:])
