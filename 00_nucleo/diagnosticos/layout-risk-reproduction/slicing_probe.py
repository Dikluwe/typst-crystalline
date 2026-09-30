"""Compile the unchanged production slicer in an external diagnostic harness."""
import json
import run
s = run.state()
b = run.call(["cargo", "build", "--locked", "-p", "typst-core", "--message-format=json"], timeout=900)
artifacts = [json.loads(x) for x in b["stdout"].splitlines() if x.startswith("{")]
rlibs = [f for a in artifacts if a.get("reason") == "compiler-artifact" and a.get("target",{}).get("name") == "typst_core" for f in a["filenames"] if f.endswith(".rlib")]
if b["exit"] != 0 or len(rlibs) != 1: raise SystemExit("No uniquely proven core artifact")
out = run.BASE / "outputs" / "slicing-probe"
out.parent.mkdir(parents=True, exist_ok=True)
source = run.BASE / "slicing_probe.rs"
c = run.call(["rustc", "--edition=2021", source, "--extern", "typst_core="+rlibs[0], "-L", "dependency="+str(run.ROOT/"target/debug/deps"), "-o", out])
r = run.call([out]) if c["exit"] == 0 else None
end = run.state()
run.save(run.BASE/"receipts"/"slicing-probe.json", dict(state=s,end_state=end,build=b,compile=c,run=r,
    source_sha256=run.sha(source), consumer_sha256=run.sha(run.ROOT/"01_core/src/compiler/layout/slicing.rs"),
    core_rlib=rlibs[0],core_rlib_sha256=run.sha(rlibs[0]), harness_sha256=run.sha(out) if r else None))
print(json.dumps(dict(compile_exit=c["exit"],run=r)))
if s["source_identity"] != end["source_identity"]: raise SystemExit("Source changed")
