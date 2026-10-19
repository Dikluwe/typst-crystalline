"""Read-only outside-scope comparison; write only a diagnostic handoff receipt."""
import datetime, hashlib, json, pathlib, subprocess
here = pathlib.Path(__file__).resolve().parent
root = here.parents[2]
before = json.loads((here / "results/provenance-before.json").read_text())
changes = {}
for name, digest in before["protected_sha256"].items():
    path = root / name
    current = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
    if current != digest:
        changes[name] = dict(before=digest, handoff=current)
receipt = dict(time=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    head=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
    status=subprocess.check_output(["git", "status", "--porcelain=v1", "-uall"], cwd=root, text=True),
    outside_scope_changes=changes,
    productive_source_changes=[p for p in changes if p.startswith(("01_core/src/", "02_shell/src/", "03_infra/src/", "04_wiring/src/"))],
    qualification="Later concurrent changes, not changes observed during the successful measurement; left untouched.")
(here / "results/provenance-handoff.json").write_text(json.dumps(receipt, ensure_ascii=False, indent=2)+"\n")
print(json.dumps({"productive_source_changes":receipt["productive_source_changes"],
                  "full_receipt":"results/provenance-handoff.json"}))
