#!/usr/bin/env python3
"""P1307 R2 public-only observational adapters; no productive edits."""
import argparse
import base64
import concurrent.futures
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "00_nucleo/diagnosticos"
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def load():
    p = OUT / "p1307-r2-measurement.json"
    return json.loads(p.read_text()) if p.exists() else {"schema": "p1307-r2-public-observability-v1", "attempts": []}

def save(data):
    (OUT / "p1307-r2-measurement.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n")

def binaries():
    b = json.loads((OUT / "p1307-baseline.json").read_text())["binaries"]
    for x in b.values():
        assert sha(x["path"]) == x["sha256"], "binary identity changed"
    return {"vanilla": b["vanilla"], "baseline": b["crystalline"]}

def provenance():
    return {"at": now(), "cwd": str(ROOT), "script_sha256": sha(__file__), "preflight_sha256": sha(OUT / "p1307-r2-preflight.json"), "predecessor_baseline_sha256": sha(OUT / "p1307-baseline.json"), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True, cwd=ROOT).strip(), "diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], text=True, cwd=ROOT), "environment": {"NO_COLOR": "1", "TERM": "dumb", "TYPST_FEATURES": "unset; explicit CLI profiles"}}

def run(side, binary, case, profile):
    kind = case["route"]
    argv = [binary, kind]
    source = None
    if kind == "eval":
        argv += [case["expression"], "--format", "json"]
    elif kind == "query":
        source = case["document"]
        argv += ["-", "<out>", "--field", "value", "--one", "--format", "json"]
    elif kind == "compile":
        source = case["document"]
        argv += [case.get("fixture", "-"), case.get("pdf_output", "-")]
        if "fixture" in case:
            argv += ["--format", "pdf"]
    argv += PROFILES[profile]
    env = {k: v for k, v in os.environ.items() if k != "TYPST_FEATURES"}
    env.update(NO_COLOR="1", TERM="dumb")
    started = now()
    t = time.monotonic()
    try:
        p = subprocess.run(argv, input=source.encode() if source is not None else None, capture_output=True, cwd=ROOT, env=env, timeout=30)
        stdout, stderr = p.stdout.decode("utf-8", "replace"), p.stderr.decode("utf-8", "replace")
        obs = {"kind": "Unknown", "reason": "UNCLASSIFIED_OUTPUT"}
        if p.returncode == 0 and kind in ("eval", "query"):
            try:
                obs = {"kind": "value", "value": json.loads(stdout)}
            except ValueError:
                obs = {"kind": "Unknown", "reason": "INVALID_JSON"}
        elif p.returncode == 1 and "error: panicked with: P1307R2:" in stderr:
            captures = re.findall(r"^error: panicked with: P1307R2:([0-9,]*):END$", stderr, re.M)
            if len(captures) == 1:
                try:
                    raw = bytes(int(x) for x in captures[0].split(",") if x)
                    obs = {"kind": "transported-public-string", "value": raw.decode("utf-8"), "utf8_bytes": list(raw), "sha256": hashlib.sha256(raw).hexdigest()}
                except (ValueError, UnicodeError):
                    obs = {"kind": "Unknown", "reason": "INVALID_PUBLIC_BYTE_TRANSPORT"}
        elif p.returncode == 1 and "error:" in stderr:
            obs = {"kind": "diagnostic", "stderr_literal": stderr}
        elif p.returncode == 2:
            obs = {"kind": "Unknown", "reason": "CLI_ROUTE_OR_EXECUTION_FAILURE"}
        elif p.returncode == 0 and kind == "compile":
            obs = {"kind": "Unknown", "reason": "NO_CONTEXT_TRANSPORT_EXECUTED"}
        return {"case": case["id"], "side": side, "profile": profile, "argv": argv, "stdin": source, "source_numbered": "\n".join(f"{i+1}: {line}" for i,line in enumerate((source or case.get("expression", "")).splitlines())), "started": started, "elapsed_seconds": time.monotonic()-t, "returncode": p.returncode, "stdout": stdout if kind != "compile" else None, "stdout_base64": base64.b64encode(p.stdout).decode() if kind == "compile" else None, "stderr": stderr, "observable": obs}
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"case": case["id"], "side": side, "profile": profile, "argv": argv, "stdin": source, "started": started, "elapsed_seconds": time.monotonic()-t, "observable": {"kind": "Unknown", "reason": type(exc).__name__}, "exception": str(exc)}

def matrix(cases, profiles):
    bins = binaries()
    jobs = [(side, info["path"], c, p) for c in cases for p in profiles for side,info in bins.items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        return list(pool.map(lambda x: run(*x), jobs))

def transport(expression, contextual=False):
    rendered = f'{{ let raw = bytes({expression}); panic("P1307R2:" + range(raw.len()).map(i => str(raw.at(i))).join(",") + ":END") }}'
    if contextual:
        return '= Probe <probe>\n#context { let found = query(<probe>); if found.len() > 0 { ' + rendered + ' } }\n'
    return "#" + rendered + "\n"

def initial(args):
    data = load()
    assert not data["attempts"], "initial attempt already recorded"
    cases = [
        {"id": "byte-array-cast", "route": "eval", "expression": 'array(bytes("α"))'},
        {"id": "query-direct-metadata", "route": "query", "document": '#metadata("ok") <out>\n'},
        {"id": "query-context-location", "route": "query", "document": '= Probe <probe>\n#context metadata(repr(query(<probe>).first().location())) <out>\n'},
    ]
    rows = matrix(cases, list(PROFILES))
    data.update(provenance=provenance(), binaries=binaries(), revision_budget=2)
    data["attempts"].append({"revision": 0, "hypothesis": "Existing query metadata can carry contextual Values and profiles; array(Bytes) exposes complete payload.", "cases": cases, "rows": rows, "finished": now()})
    save(data)
    print(json.dumps([{k:r[k] for k in ("case","side","profile","observable")} for r in rows],ensure_ascii=False),flush=True)

def contextual(args):
    data = load()
    assert len(data["attempts"]) == 1
    cases = [{"id": key, "route": "compile", "document": transport(expr, contextual=ctx)} for key,expr,ctx in [
        ("transport-calibration", '"α\\n\\\"x\\\""', False),
        ("construct-Location", "repr(found.first().location())", True),
        ("construct-LocatedContent", "repr(found.first())", True),
        ("feature-html", "repr(html.elem)", False),
        ("feature-a11y", 'repr(pdf.table-summary)', False),
    ]]
    rows = matrix(cases, list(PROFILES))
    data["attempts"].append({"revision": 1, "hypothesis": "compile supports all feature flags and runs layout context. Intentional panic of decimal UTF8 bytes via bytes.len/at transports complete public Str without files or encoder dependency; array(Bytes) constructor absent in baseline is avoided using existing public byte accessors. Feature sentinels prove profile effects.", "cases": cases, "rows": rows, "provenance": provenance(), "finished": now()})
    save(data)
    print(json.dumps([{k:r[k] for k in ("case","side","profile","observable")} for r in rows],ensure_ascii=False),flush=True)

def materialize_fixtures(cases):
    folder = Path(tempfile.mkdtemp(prefix="p1307-r2-probes-", dir="/tmp"))
    patches = []
    for i,c in enumerate(cases):
        if c["route"] != "compile":
            continue
        path = folder / f"{i:03d}-{c['id']}.typ"
        c.update(fixture=str(path), pdf_output=str(path.with_suffix(".pdf")), fixture_sha256=hashlib.sha256(c["document"].encode()).hexdigest())
        patches.append(f"*** Add File: {path}\n" + "".join("+"+line+"\n" for line in c["document"].splitlines()))
    subprocess.run(["apply_patch"], input="*** Begin Patch\n" + "".join(patches) + "*** End Patch\n", text=True, check=True, capture_output=True, cwd=ROOT)
    return str(folder)

def file_contextual(args):
    data=load()
    assert len(data["attempts"]) == 2
    cases=[dict(c) for c in data["attempts"][1]["cases"]]
    folder=materialize_fixtures(cases)
    rows=matrix(cases,list(PROFILES))
    data["attempts"].append({"revision":2,"hypothesis":"Named temporary .typ files replace unsupported baseline compile stdin. Keep existing compile --features and public decimal UTF8 transport; parse only actual panic message, not marker text printed in source context.","fixture_directory":folder,"cases":cases,"rows":rows,"provenance":provenance(),"finished":now()})
    save(data)
    print(json.dumps([{k:r[k] for k in ("case","side","profile","observable")} for r in rows],ensure_ascii=False),flush=True)

def complete_cases():
    cases=[]
    for typ,value in (("Location","found.first().location()"),("LocatedContent","found.first()")):
        cases.append({"id":f"type-{typ}","route":"compile","document":transport(f"repr(type({value}))",True)})
        for fmt in ("json","toml","yaml"):
            arg=f"(value: {value})" if fmt=="toml" else value
            cases.append({"id":f"context-{fmt}-{typ}","route":"compile","document":transport(f"{fmt}.encode({arg})",True)})
    for tag,value in {"primitive":"(z: 1, a: 2)","bytes":"bytes((0,255,10))","Symbol":"sym.alpha","Content":"[hi]","empty-bytes":"bytes(())","none":"none","float-negative-zero":"-0.0"}.items():
        cases.append({"id":f"cbor-payload-{tag}","route":"eval","expression":f"{{ let raw = cbor.encode({value}); range(raw.len()).map(i => raw.at(i)) }}"})
    for fmt in ("json","toml","yaml"):
        value="(z: (1, 2), a: 3)"
        routes={
            "parent-invalid-named":f"{fmt}.with(nope: 9).encode({value})",
            "parent-bound-positional-named":f'{fmt}.with(bytes("ignored"), nope: 9).encode({value})',
            "encoder-bound-invalid-named":f"{fmt}.encode.with(nope: 9)({value})",
            "encoder-bound-pretty":f"{fmt}.encode.with(pretty: false)({value})",
            "encoder-override":f"{fmt}.encode.with(pretty: false)({value}, pretty: true)",
            "invalid-first-occurrence":f'{fmt}.encode.with(pretty: "bad")({value}, pretty: true)',
            "invalid-last-occurrence":f'{fmt}.encode.with(pretty: true)({value}, pretty: "bad")',
            "encoder-with-identity":f"(repr(type({fmt}.encode.with())), repr({fmt}.encode.with()))",
        }
        for tag,expr in routes.items():
            cases.append({"id":f"with-{fmt}-{tag}","route":"eval","expression":expr})
    for carrier in ("arguments", "closure"):
        ctor="arguments(nope: 1)" if carrier=="arguments" else "((..xs) => xs)(nope: 1)"
        for final in ("f", "g"):
            expr="{\n  let make(which) = {\n    let a = "+ctor+";\n    let b = "+ctor+";\n    yaml.encode.with(..(if which { a } else { b }))\n  };\n  let f = make(true);\n  let g = make(false);\n  "+final+"((:))\n}"
            cases.append({"id":f"with-spread-{carrier}-{final}","route":"eval","expression":expr})
    return cases

def focal_extra(args):
    data=load()
    assert len(data["attempts"])==3 and "extra_focal" not in data
    assert all(r["observable"]["kind"]!="Unknown" for r in data["attempts"][2]["rows"]),"context adapter not observable"
    cases=complete_cases()
    folder=materialize_fixtures(cases)
    rows=matrix(cases,["default"])
    data["extra_focal"]={"provenance":provenance(),"fixture_directory":folder,"cases":cases,"rows":rows,"finished":now()}
    save(data)
    print(json.dumps([{k:r[k] for k in ("case","side","observable")} for r in rows],ensure_ascii=False),flush=True)

def full(args):
    data=load()
    assert "extra_focal" in data and "matrix" not in data
    assert all(r["observable"]["kind"]!="Unknown" for r in data["extra_focal"]["rows"]),"new focal not observable"
    cases=data["attempts"][2]["cases"]+data["extra_focal"]["cases"]
    # Cases carry immutable fixture paths reused unchanged across every order.
    for c in cases:
        if "fixture" in c:
            assert sha(c["fixture"])==c["fixture_sha256"]
    orders={}
    for name,cs in (("normal",cases),("repeat",cases),("reverse",list(reversed(cases)))):
        print(f"measuring {name}: {len(cases)} cases x4profiles x2binaries",flush=True)
        orders[name]=matrix(cs,list(PROFILES))
    lookup={(r["case"],r["side"],r["profile"]):r for r in orders["normal"]}
    instability=[]
    for order,rows in orders.items():
        for r in rows:
            old=lookup[r["case"],r["side"],r["profile"]]
            # Complete output stability, including raw diagnostics; never
            # normalize a serialized Str or silently discard profiles.
            if any(old.get(k)!=r.get(k) for k in ("returncode","stdout","stdout_base64","stderr","observable")):
                instability.append({"order":order,"case":r["case"],"side":r["side"],"profile":r["profile"]})
    data["matrix"]={"provenance":provenance(),"cases":cases,"orders":orders,"instability":instability,"unknowns":[{"order":o,"case":r["case"],"side":r["side"],"profile":r["profile"]} for o,rs in orders.items() for r in rs if r["observable"]["kind"]=="Unknown"],"finished":now()}
    save(data)
    print(json.dumps({"runs":sum(map(len,orders.values())),"unknowns":data["matrix"]["unknowns"],"instability":instability}),flush=True)

if __name__ == "__main__":
    p=argparse.ArgumentParser()
    p.add_argument("command", choices=["initial", "contextual", "file-contextual", "focal-extra", "full"])
    a=p.parse_args()
    {"initial":initial,"contextual":contextual,"file-contextual":file_contextual,"focal-extra":focal_extra,"full":full}[a.command](a)
