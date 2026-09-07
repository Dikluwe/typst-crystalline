"""P1307 provenance and pre-gate orchestration; no product implementation."""
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def pin(path):
    path = Path(path)
    return {"path": str(path), "sha256": sha(path.read_bytes()), "bytes": path.stat().st_size}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def state():
    commands = {
        "head": ["rev-parse", "HEAD"], "status": ["status", "--short"],
        "diff_stat": ["diff", "HEAD", "--stat"], "diff": ["diff", "HEAD", "--binary"],
    }
    result = {"at": now()}
    for name, args in commands.items():
        data = git(*args)
        result[name] = data.decode()
        result[name + "_sha256"] = sha(data)
    return result


def source_inventory():
    paths = git("ls-files", "01_core", "02_shell", "03_infra", "04_wiring", "00_nucleo/prompts", "Cargo.toml", "Cargo.lock", ".cargo", "crystalline.toml").decode().splitlines()
    return {path: sha((ROOT / path).read_bytes()) for path in paths if (ROOT / path).is_file()}


def command(argv, cwd=ROOT, env=None):
    started = now()
    tick = time.monotonic()
    proc = subprocess.run(argv, cwd=cwd, env=env, capture_output=True)
    record = {"argv": list(map(str, argv)), "cwd": str(cwd), "started_at": started, "finished_at": now(), "seconds": time.monotonic() - tick, "exit_code": proc.returncode}
    for name, data in (("stdout", proc.stdout), ("stderr", proc.stderr)):
        record[name] = data.decode(errors="replace")
        record[name + "_base64"] = base64.b64encode(data).decode()
        record[name + "_sha256"] = sha(data)
    return record


def save(name, payload):
    path = DIAG / name
    with path.open("x") as stream:
        json.dump(payload, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
    print(json.dumps(pin(path)), flush=True)


def baseline():
    before = state()
    sources = source_inventory()
    preflight = json.loads((DIAG / "p1307-preflight.json").read_text())
    closure = sorted(DIAG.glob("p1306-*")) + [DIAG / "p1305-commit-notes.md", ROOT / "00_nucleo/materialization/typst-passo-1306.md"]
    frozen_closure = {str(path.relative_to(ROOT)): pin(path) for path in closure if path.is_file()}
    vanilla = pin("/usr/local/bin/typst")
    assert vanilla["sha256"] == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
    certificate = pin(DIAG / "p1306-certificate.json")
    assert certificate["sha256"] == "9c7008973799dcf02cb3f02b08a76707c5e52a0a63b7738260fad2c06664733d"
    target = Path(tempfile.mkdtemp(prefix="p1307-baseline-target.", dir="/dev/shm"))
    print(json.dumps({"target": str(target), "source_state": before["diff_stat"], "phase": "fresh baseline build"}), flush=True)
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(target)
    build = command(["cargo", "build", "--release", "-p", "typst-wiring", "--bin", "typst"], env=env)
    after = state()
    assert sources == source_inventory(), "Source changed during baseline build"
    for relative, record in frozen_closure.items():
        assert sha((ROOT / relative).read_bytes()) == record["sha256"], relative
    payload = {
        "schema": "p1307-composite-baseline-v1", "generated_at": now(),
        "preflight": pin(DIAG / "p1307-preflight.json"), "script": pin(__file__),
        "step": pin(ROOT / preflight["step"]), "source_before": before, "source_after": after,
        "source_hashes": sources, "p1306_closure": frozen_closure,
        "p1306_certificate": certificate, "owners": {p: {"prompt": pin(ROOT / p), "consumer": pin(ROOT / c)} for p, c in preflight["owners"].items()},
        "binaries": {"vanilla": vanilla, "crystalline": pin(target / "release/typst") if build["exit_code"] == 0 else None},
        "vanilla_upstream": "a51e02804", "build": build,
        "env": {"CARGO_TARGET_DIR": str(target)}, "target": str(target),
        "classification": "Composite uncommitted P1306 baseline; no P1307 L0/Rust changes during build",
    }
    save("p1307-baseline.json", payload)
    print(json.dumps({"build_exit": build["exit_code"], "seconds": build["seconds"], "binary": payload["binaries"]["crystalline"]}), flush=True)
    raise SystemExit(build["exit_code"])


def gate(argv):
    before = state()
    result = command(argv)
    result.update(source_before=before, source_after=state(), preflight=pin(DIAG / "p1307-preflight.json"))
    path = DIAG / "p1307-gates.json"
    rows = json.loads(path.read_text()) if path.exists() else []
    rows.append(result)
    path.write_text(json.dumps(rows, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({k: result[k] for k in ("argv", "exit_code", "seconds")}), flush=True)
    print(result["stdout"][-1600:])
    print(result["stderr"][-800:])
    raise SystemExit(result["exit_code"])


if __name__ == "__main__":
    if sys.argv[1:] == ["baseline"]:
        baseline()
    elif sys.argv[1] == "gate":
        gate(sys.argv[2:])
    else:
        raise SystemExit("usage: coordinator.py baseline | gate <argv...>")
