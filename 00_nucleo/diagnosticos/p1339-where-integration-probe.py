#!/usr/bin/env python3
"""Read-only vanilla measurement. All artifacts printed to stdout."""
import datetime, hashlib, json, pathlib, shlex, subprocess

ROOT = pathlib.Path("/repos/Antigravity/typst-crystalline")
BIN = "/usr/local/bin/typst"
EXPECTED = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path): return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def run(cmd, stdin=None):
    t = utc()
    p = subprocess.run(cmd, input=stdin, text=True, capture_output=True, cwd=ROOT, timeout=45)
    return dict(command=cmd, shell_command=shlex.join(cmd), stdin=stdin, utc_start=t, utc_end=utc(), exit_code=p.returncode, stdout=p.stdout, stderr=p.stderr)
def state():
    return {name: run(cmd) for name, cmd in (
        ("head", ["git", "rev-parse", "HEAD"]),
        ("diff_stat", ["git", "diff", "HEAD", "--stat"]),
        ("status", ["git", "status", "--short"]),
    )}
cases = []
def ev(id, expr):
    cases.append(dict(id=id, expression=expr, command=[BIN, "eval", expr, "--format", "json"], stdin=None))
prefix = "#set heading(numbering: \"1.1\")\n= First\n== Child\n= Last\n*Bold* _Emph_ Text\n"
def fixture(id, expr):
    src = prefix + "#context metadata(" + expr + ")\n"
    cases.append(dict(id=id, expression=expr, command=[BIN, "query", "-", "metadata", "--field", "value"], stdin=src))
ev("selector_heading_empty", "{ let s = selector(heading.where()); (repr(type(s)), repr(s), s == heading.where(), s == selector(heading)) }")
ev("selector_heading_filtered", "{ let s = selector(heading.where(level: 1)); (repr(type(s)), repr(s), s == heading.where(level: 1)) }")
ev("selector_strong_empty", "{ let s = selector(strong.where()); (repr(type(s)), repr(s)) }")
ev("selector_text_filtered", "{ let s = selector(text.where(text: \"Hi\")); (repr(type(s)), repr(s)) }")
for id, key in [("counter_heading_bare", "heading"), ("counter_heading_empty", "heading.where()"), ("counter_heading_level1", "heading.where(level: 1)"), ("counter_heading_level2", "heading.where(level: 2)")]:
    ev(id, "{ let c = counter(" + key + "); (repr(type(c)), repr(c)) }")
ev("counter_key_equality", "(counter(heading) == counter(heading.where()), counter(heading.where(level: 1)) == counter(heading.where(level: 2)))")
for name in ("strong", "emph", "text"):
    ev("counter_reject_" + name, "repr(counter(" + name + ".where()))")
for id, sel in [("query_heading_bare", "heading"), ("query_heading_empty", "heading.where()"), ("query_heading_level1", "heading.where(level: 1)"), ("query_heading_level2", "heading.where(level: 2)"), ("query_heading_miss", "heading.where(level: 9)"), ("query_heading_wrong_type", "heading.where(level: \"bad\")")]:
    fixture(id, "query(" + sel + ").map(it => (it.level, repr(it.body)))")
for name in ("strong", "emph", "text"):
    fixture("query_reject_" + name, "query(" + name + ".where()).len()")
fixture("counter_get", "(counter(heading).get(), counter(heading.where()).get(), counter(heading.where(level: 1)).get(), counter(heading.where(level: 2)).get(), counter(heading.where(level: 9)).get())")
fixture("counter_display", "(repr(counter(heading).display()), repr(counter(heading.where()).display()), repr(counter(heading.where(level: 1)).display()))")
# A contextual display forced through a show rule observes realized text as metadata.
src = prefix + """#show text: it => { metadata(it.text); it }
#context counter(heading.where(level: 1)).display()
"""
cases.append(dict(id="counter_display_realized", expression=None, command=[BIN, "query", "-", "metadata", "--field", "value"], stdin=src))
inputs = ["00_nucleo/materialization/typst-passo-1339.md", "00_nucleo/diagnosticos/p1339-where-l0-gate.md", "00_nucleo/diagnosticos/p1339-where-l0-review.md", "00_nucleo/diagnosticos/p1339-full-a2.md", str(pathlib.Path(__file__).relative_to(ROOT))]
receipt = dict(regime="executado sem atestacao de isolamento", executor="/root/p1339_where_integration_probe", baseline_upstream="a51e02804", binary=BIN, binary_sha256=sha(BIN), utc_start=utc(), inputs={p: sha(ROOT / p) for p in inputs}, before=state(), cases=cases)
assert receipt["binary_sha256"] == EXPECTED
receipt["runs"] = [dict(id=c["id"], **run(c["command"], c["stdin"])) for c in cases]
receipt["after"] = state()
receipt["utc_end"] = utc()
print(json.dumps(receipt, ensure_ascii=False, indent=2))

