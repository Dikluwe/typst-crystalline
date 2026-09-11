"""Build only isolated adversarial reference copies; never productive source."""
import argparse
import base64
import datetime
import hashlib
import json
import pathlib
import shutil
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
AREA = pathlib.Path("/tmp/p1339-mutants.oPfqdA")
AUTHORITY = "842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b"

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source", choices=["instrumented", "duplicate"])
    parser.add_argument("label")
    args = parser.parse_args()
    assert args.label.replace("-", "").isalnum()
    source = AREA / args.source
    output = D / f"p1339-mutant-build-{args.label}.json"
    assert not output.exists()
    argv = ["cargo", "build", "--manifest-path", str(source / "Cargo.toml"), "-p", "typst-cli",
            "--bin", "typst", "--release", "--locked", "--offline", "--target-dir", str(AREA / "target"),
            "--config", "profile.release.lto=false", "--config", "profile.release.codegen-units=16", "-j", "3"]
    files = sorted(p for p in source.rglob("*") if p.is_file() and (p.suffix == ".rs" or p.name in ("Cargo.toml", "Cargo.lock")))
    inputs = {str(p.relative_to(source)):sha(p) for p in files}
    start = utc()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=2700)
    record = {"authority_manifest_sha256":AUTHORITY,"start":start,"end":utc(),"argv":argv,"cwd":str(ROOT),
              "source_root":str(source),"source_sha256":inputs,"exit":p.returncode,
              "stdout":p.stdout.decode(errors="replace"),"stderr":p.stderr.decode(errors="replace"),
              "stdout_base64":base64.b64encode(p.stdout).decode(),"stderr_base64":base64.b64encode(p.stderr).decode(),
              "runner_sha256":sha(__file__)}
    if p.returncode == 0:
        binaries = AREA / "binaries"
        binaries.mkdir(exist_ok=True)
        binary = binaries / ("typst-" + args.label)
        assert not binary.exists()
        shutil.copy2(AREA / "target/release/typst", binary)
        record["binary"] = {"path":str(binary),"sha256":sha(binary)}
    with output.open("x") as f:
        json.dump(record,f,ensure_ascii=False,indent=2)
        f.write("\n")
    print(json.dumps({"output":str(output),"exit":p.returncode,"binary":record.get("binary")}))

if __name__ == "__main__":
    main()
