"""Controles linguísticos da auditoria, sem alteração de código produtivo."""
import datetime
import gzip
import hashlib
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[3]
HERE = pathlib.Path(__file__).resolve().parent
ENGINES = {
    "tekt": ROOT / "target/release/typst",
    "vanilla": ROOT / "lab/typst-original/target/release/typst",
}
CASES = {
    "original": '#show heading.where(level: 1): it => upper(it.body)\n= Original\n',
    "generated_one": '#show heading.where(level: 1): it => heading(level: 2)[Generated A]\n= Original\n',
    "generated_two": '#show heading.where(level: 1): it => [#heading(level: 2)[Generated A] #heading(level: 2)[Generated B]]\n= Original\n',
    "matrix_mutation": '#let calls = (1,)\n#let m = math.mat({ calls += calls; [a] })\n#assert(calls.len() == 2)\n#m\n',
    "matrix_once": '#let calls = ()\n#let m = math.mat({ calls.push("once"); [a] })\n#assert(calls == ("once",))\n#m\n',
}

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)

record = {
    "started": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "head": git("rev-parse", "HEAD").strip(),
    "diff_stat": git("diff", "HEAD", "--stat"),
    "status": git("status", "--short"),
    "vanilla_pin": "9f2b6e8715237cb086899a42873660fe744622e8",
    "binaries": {name: {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()} for name, path in ENGINES.items()},
    "cases": [],
}
for name, source in CASES.items():
    fixture = HERE / (name + ".typ")
    fixture.write_text(source)
    for engine, binary in ENGINES.items():
        args = (["query", str(fixture), "heading", "--field", "body"]
                if name.startswith("original") or name.startswith("generated")
                else ["compile", str(fixture), str(pathlib.Path("/tmp") / ("tekt-audit-" + name + "-" + engine + ".pdf"))])
        command = [str(binary), *args]
        result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, timeout=60)
        record["cases"].append({"name": name, "engine": engine, "source": source,
                                "command": command, "returncode": result.returncode,
                                "stdout": result.stdout, "stderr": result.stderr})
record["finished"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE / "results.json.gz", "wt") as output:
    json.dump(record, output, ensure_ascii=False, indent=2)
for item in record["cases"]:
    print(item["name"], item["engine"], item["returncode"], item["stdout"].strip(), item["stderr"].strip())
