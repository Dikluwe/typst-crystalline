"""Index individual measurements and verify immutable provenance receipts."""
import json, pathlib
import run
build = json.loads((run.BASE/"receipts/build.json").read_text())
baseline = json.loads((run.ROOT/build["after"]["path"]).read_text())
final_ref = run.state()
final = json.loads((run.ROOT/final_ref["path"]).read_text())
files = set(baseline["productive_documentary_sha256"]) | set(final["productive_documentary_sha256"])
changed = sorted(p for p in files if baseline["productive_documentary_sha256"].get(p) != final["productive_documentary_sha256"].get(p))
records = {}
errors = []
for path in sorted((run.BASE/"outputs").glob("*/*.json")):
    obs = json.loads(path.read_text())
    if "fixture_sha256" not in obs: continue
    refs = [obs["state"]] + ([obs["end_state"]] if "end_state" in obs else [])
    for ref in refs:
        if run.sha(run.ROOT/ref["path"]) != ref["sha256"]: errors.append("receipt hash mismatch: "+ref["path"])
        if run.sha((run.ROOT/ref["path"]).with_suffix(".patch")) != ref["patch_sha256"]: errors.append("patch hash mismatch: "+ref["path"])
    records[str(path.relative_to(run.ROOT))] = dict(sha256=run.sha(path), start=obs["run"]["started"],
        finish=obs["run"]["finished"],exit=obs["run"]["exit"],state=refs,
        has_individual_boundaries="end_state" in obs,binary_sha256=obs["binary_sha256"])
fonts = {str(p):run.sha(p) for p in sorted(pathlib.Path("/usr/share/fonts/truetype/dejavu").glob("*.ttf"))}
inputs = {str(p.relative_to(run.ROOT)):run.sha(p) for p in sorted((run.BASE/"fixtures").iterdir()) if p.is_file()}
run.save(run.BASE/"receipts/evidence-index.json",dict(head=final["head"],baseline=build["after"],final=final_ref,
    changed_since_build=changed,product_changes_since_build=[p for p in changed if p.startswith(("01_core/","02_shell/","03_infra/","04_wiring/"))],
    binaries=run.binaries(),inputs_sha256=inputs,font_files_sha256=fonts,records=records,integrity_errors=errors,
    tools={k:run.call(v) for k,v in {"cargo":["cargo","--version"],"rustc":["rustc","--version"],"mutool":["mutool","-v"],"pdftotext":["pdftotext","-v"]}.items()}))
print(json.dumps(dict(integrity_errors=errors,product_changes_since_build=[p for p in changed if p.startswith(("01_core/","02_shell/","03_infra/","04_wiring/"))],external_documentary_changes=changed)))
if errors: raise SystemExit("Evidence integrity failed")
