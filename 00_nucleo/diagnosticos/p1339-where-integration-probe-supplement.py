#!/usr/bin/env python3
"""Focal supplement: locatability discovered in initial measurement."""
import datetime, hashlib, json, pathlib, shlex, subprocess
ROOT = pathlib.Path("/repos/Antigravity/typst-crystalline")
BIN = "/usr/local/bin/typst"
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(cmd, stdin=None):
    start = utc()
    p = subprocess.run(cmd, input=stdin, text=True, capture_output=True, cwd=ROOT, timeout=45)
    return dict(command=cmd, shell_command=shlex.join(cmd), stdin=stdin, utc_start=start, utc_end=utc(), exit_code=p.returncode, stdout=p.stdout, stderr=p.stderr)
def sha(path): return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def state(): return {key: run(cmd) for key, cmd in [("head", ["git","rev-parse","HEAD"]), ("diff_stat", ["git","diff","HEAD","--stat"]), ("status", ["git","status","--short"])]}
cases=[]
prefix="*Bold* _Emph_ Text\n"
for name, body in [("strong","Bold"), ("emph","Emph")]:
    expr="(query(" + name + ").map(it => repr(it.body)), query(" + name + ".where()).map(it => repr(it.body)), query(" + name + ".where(body: [" + body + "])).map(it => repr(it.body)), query(" + name + ".where(body: [Miss])).len())"
    cases.append(dict(id="query_" + name + "_filtered", source=prefix+"#context metadata("+expr+")\n"))
    expr="(counter(" + name + ").get(), counter(" + name + ".where()).get(), counter(" + name + ".where(body: [" + body + "])).get(), counter(" + name + ".where(body: [Miss])).get(), counter(" + name + ".where()).display())"
    cases.append(dict(id="counter_" + name + "_filtered", source=prefix+"#context metadata("+expr+")\n"))
cases.append(dict(id="counter_empty_distinct_manual_update", source="#set heading(numbering: \"1\")\n= First\n#counter(heading).update(42)\n#context metadata((counter(heading).get(), counter(heading.where()).get()))\n"))
sources=["foundations/selector.rs", "introspection/counter.rs", "introspection/query.rs", "model/strong.rs", "model/emph.rs", "model/heading.rs"]
receipt=dict(regime="executado sem atestacao de isolamento", executor="/root/p1339_where_integration_probe", predecessor_sha256=sha(ROOT/"00_nucleo/diagnosticos/p1339-where-integration-probe-runs.json"), binary=BIN,binary_sha256=sha(BIN), runner_sha256=sha(__file__), baseline_upstream="a51e02804", utc_start=utc(), before=state(), sources={p: sha(ROOT/"lab/typst-original/crates/typst-library/src"/p) for p in sources}, cases=cases)
assert receipt["binary_sha256"] == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
receipt["runs"]=[dict(id=c["id"], **run([BIN,"query","-","metadata","--field","value"], c["source"])) for c in cases]
receipt["after"]=state()
receipt["utc_end"]=utc()
print(json.dumps(receipt, ensure_ascii=False, indent=2))

