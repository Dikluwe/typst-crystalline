"""Raw adversarial observations only. Independent verifier owns verdict/seal."""
import argparse
import base64
import datetime
import hashlib
import json
import os
import pathlib
import resource
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
AREA = pathlib.Path("/tmp/p1339-mutants.oPfqdA")
AUTHORITY = "842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b"

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def no_core():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

def observation(binary, mode, expression, case, profile="default"):
    argv = [str(binary), "--color=never", "eval", expression, "--format", "json"]
    features = {"default": "", "html":"html", "a11y":"a11y-extras", "html+a11y":"html,a11y-extras"}[profile]
    if features:
        argv += ["--features", features]
    env_delta = {"P1339_MUTANT":str(mode),"LC_ALL":"C.UTF-8","NO_COLOR":"1"}
    row = {"case":case,"source":expression,"source_sha256":hashlib.sha256(expression.encode()).hexdigest(),
           "profile":profile,"mode":str(mode),"argv":argv,"cwd":str(ROOT),"env_delta":env_delta,
           "inherited_environment":"host environment plus recorded env_delta; no fixture-dependent values",
           "rlimit_core":[0,0],"at":utc(),"binary_sha256":sha(binary)}
    try:
        p = subprocess.run(argv,cwd=ROOT,env={**os.environ,**env_delta},capture_output=True,timeout=30,preexec_fn=no_core)
        row["exit"] = p.returncode
        row["signal"] = -p.returncode if p.returncode < 0 else None
        for channel in ("stdout","stderr"):
            value = getattr(p,channel)
            row[channel] = value.decode(errors="replace")
            row[channel+"_base64"] = base64.b64encode(value).decode()
        row["transport"] = "terminated_by_signal" if p.returncode < 0 else "completed"
    except (subprocess.TimeoutExpired,OSError) as e:
        row["transport"] = type(e).__name__
        row["error"] = str(e)
        if isinstance(e,subprocess.TimeoutExpired):
            for channel in ("stdout","stderr"):
                value = getattr(e,channel) or b""
                row[channel] = value.decode(errors="replace")
                row[channel+"_base64"] = base64.b64encode(value).decode()
    row["end"] = utc()
    return row

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("label")
    parser.add_argument("--binary",required=True)
    parser.add_argument("--duplicate")
    parser.add_argument("--families",default="all")
    args = parser.parse_args()
    assert args.label.replace("-","").isalnum()
    output = D / ("p1339-mutant-"+args.label+".json")
    assert not output.exists()
    artifact = D/"p1339-negative-oracles.json"
    cases = json.loads(artifact.read_text())["cases"]
    if args.families != "all":
        selected = {int(x) for x in args.families.split(",")}
        cases = [c for c in cases if c["family"] in selected]
    binary = pathlib.Path(args.binary).resolve()
    pinned = pathlib.Path("/usr/local/bin/typst")
    uninstrumented = AREA/"binaries/typst-uninstrumented"
    start = utc()
    rows = []
    for c in cases:
        family = c["family"]
        rows.append({"family":family,"role":"pinned",**observation(pinned,0,c["expr"],c["id"])})
        rows.append({"family":family,"role":"rebuild",**observation(uninstrumented,0,c["expr"],c["id"])})
        rows.append({"family":family,"role":"control",**observation(binary,0,c["expr"],c["id"])})
        if family == 12:
            if args.duplicate:
                rows.append({"family":family,"role":"mutant",**observation(pathlib.Path(args.duplicate),0,c["expr"],c["id"])})
        else:
            rows.append({"family":family,"role":"mutant",**observation(binary,family,c["expr"],c["id"])})
    if args.families == "all":
        rows.append({"family":0,"role":"invalid-mode",**observation(binary,99,"1","mode-invalid")})
    record = {"authority_manifest_sha256":AUTHORITY,"start":start,"end":utc(),
              "negative_oracles_sha256":sha(artifact),"runner_sha256":sha(__file__),"rows":rows,
              "purpose":"focal raw observations; no discrimination verdict or seal"}
    with output.open("x") as f:
        json.dump(record,f,ensure_ascii=False,indent=2)
        f.write("\n")
    print(json.dumps({"output":str(output),"rows":len(rows),"transports":{t:sum(r["transport"]==t for r in rows) for t in {r["transport"] for r in rows}}}))

if __name__ == "__main__":
    main()
