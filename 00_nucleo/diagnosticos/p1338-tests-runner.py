"""Independent P1338 A/B runner. Frozen before C; no candidate source reads."""
import argparse
import base64
import concurrent.futures
import datetime
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
MANIFEST_SHA = "14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843"
PROFILES = {"default": [], "html": ["html"], "a11y": ["a11y-extras"], "html+a11y": ["html", "a11y-extras"]}

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def read(name):
    return json.loads((D / name).read_text())

def save(name, value):
    path = D / name
    assert not path.exists(), f"immutable output already exists: {path}"
    payload = json.dumps(value, ensure_ascii=False, indent=2) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n"
    patch += "".join("+" + line + "\n" for line in payload.splitlines())
    patch += "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, cwd=ROOT, check=True, capture_output=True)

def state():
    return {"utc": utc(), **{key: subprocess.check_output(args, cwd=ROOT).decode() for key, args in {
        "head": ["git", "rev-parse", "HEAD"], "diff_stat": ["git", "diff", "HEAD", "--stat"],
        "changed_files": ["git", "diff", "HEAD", "--name-only"], "status": ["git", "status", "--short", "--", ".", ":(exclude)00_nucleo/materialization", ":(exclude)00_nucleo/context"]}.items()}}

def audit(row):
    if row.get("failure") or row.get("exit") not in (0, 1):
        return "Unknown"
    for channel in ("stdout", "stderr"):
        try:
            raw = base64.b64decode(row[channel + "_base64"], validate=True)
            if raw.decode("utf-8") != row[channel]:
                return "Unknown"
        except (KeyError, ValueError, UnicodeError):
            return "Unknown"
    if row["exit"] == 0:
        try:
            json.loads(row["stdout"])
        except ValueError:
            return "Unknown"
    elif row["stdout"] or "error: " not in row["stderr"] or "panicked at" in row["stderr"]:
        return "Unknown"
    return "Observed"

def signature(row):
    return {key: row[key] for key in ("exit", "stdout_base64", "stderr_base64")}

def observe(binary, identity, product, profile, case, order):
    argv = [binary, "--color=never", "eval", case["expr"], "--format", "json"]
    if PROFILES[profile]:
        argv += ["--features", ",".join(PROFILES[profile])]
    row = {"id": case["id"], "policy": case["policy"], "profile": profile, "product": product,
           "order": order, "argv": argv, "cwd": str(ROOT), "at": utc(), "binary_sha256": identity,
           "source": case["expr"], "source_sha256": hashlib.sha256(case["expr"].encode()).hexdigest()}
    try:
        p = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=30)
        row.update(exit=p.returncode)
        for channel in ("stdout", "stderr"):
            raw = getattr(p, channel)
            row[channel] = raw.decode("utf-8", errors="replace")
            row[channel + "_base64"] = base64.b64encode(raw).decode()
    except (OSError, subprocess.TimeoutExpired) as e:
        row["failure"] = str(e)
    row["end"] = utc()
    row["observation"] = audit(row)
    return row

def matrix(products, cases):
    rows = []
    for order in ("normal", "repeat", "reverse"):
        selected = list(reversed(cases)) if order == "reverse" else cases
        tasks = [(p["path"], p["sha256"], product, profile, c, order)
                 for profile in PROFILES for c in selected for product, p in products.items()]
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
            rows += list(pool.map(lambda a: observe(*a), tasks))
    return rows

def selftest():
    ok = {"exit": 0, "stdout": "1\n", "stdout_base64": "MQo=", "stderr": "", "stderr_base64": ""}
    tests = [("valid", ok, "Observed"), ("crash", {**ok, "exit": -11}, "Unknown"),
             ("missing-bytes", {k:v for k,v in ok.items() if k != "stderr_base64"}, "Unknown"),
             ("opaque-transport", {"failure": "planned opaque source"}, "Unknown"),
             ("malformed-json", {**ok, "stdout": "a", "stdout_base64": "YQ=="}, "Unknown"),
             ("mismatched-bytes", {**ok, "stdout_base64": "MAo="}, "Unknown")]
    rows = [{"id": name, "expected": expected, "actual": audit(r), "input": r} for name,r,expected in tests]
    assert all(r["expected"] == r["actual"] for r in rows)
    return rows

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["baseline", "candidate"])
    parser.add_argument("--binary")
    parser.add_argument("--finalize-baseline", action="store_true")
    args = parser.parse_args()
    assert sha(D / "p1338-manifest-r1.json") == MANIFEST_SHA
    m, cases = read("p1338-manifest-r1.json"), read("p1338-tests-cases.json")
    started = state()
    hashes = {name: sha(D/name) for name in ["p1338-tests-cases.json", "p1338-tests-runner.py", "p1338-tests-module.rs.txt", "p1338-tests-local-patch.json", "p1338-tests-scope.md"]}
    if args.mode == "baseline":
        products = {"vanilla": m["vanilla"], "baseline": m["baseline_binary"]}
    else:
        freeze = read("p1338-tests-freeze.json")
        assert hashes == freeze["protected"]
        assert sha(D/"p1338-tests-baseline.json") == freeze["baseline_runs_sha256"]
        assert sha(D/"p1338-tests-expectations.json") == freeze["expectations_sha256"]
        products = {"candidate": {"path": str(pathlib.Path(args.binary).resolve()), "sha256": sha(args.binary)}}
    for p in products.values():
        assert sha(p["path"]) == p["sha256"]
    if args.finalize_baseline:
        assert args.mode == "baseline"
        measured = read("p1338-tests-baseline.json")
        assert measured["manifest_sha256"] == MANIFEST_SHA and measured["products"] == products
        rows = measured["rows"]
        declared = {c["id"]: c for c in cases}
        assert all(r["source"] == declared[r["id"]]["expr"] for r in rows)
    else:
        rows = matrix(products, cases)
    result = {"manifest_sha256": MANIFEST_SHA, "before": started, "after": state(), "protected": hashes,
              "products": products, "harness_tests": selftest(), "rows": rows}
    if not args.finalize_baseline:
        save("p1338-tests-" + args.mode + ".json", result)
    assert all(r["observation"] == "Observed" for r in rows), "Unknown blocks"
    by_key = {(r["id"],r["profile"],r["product"],r["order"]):r for r in rows}
    assert all(signature(r) == signature(by_key[r["id"],r["profile"],r["product"],"normal"]) for r in rows), "unstable order"
    if args.mode == "baseline":
        expectations = []
        for c in cases:
            for profile in PROFILES:
                v = by_key[c["id"],profile,"vanilla","normal"]
                b = by_key[c["id"],profile,"baseline","normal"]
                match = signature(v) == signature(b)
                if c["policy"] == "converge":
                    assert not match and v["exit"] == 1 and b["exit"] == 1
                    assert "cannot access fields on type " in v["stderr"]
                expectations.append({"id":c["id"], "profile":profile, "policy":c["policy"],
                    "baseline_class": "CONVERGENCE_REQUIRED" if c["policy"] == "converge" else ("PRESERVE_PARITY" if match else "PRESERVE_EXISTING_DEBT"),
                    "baseline":signature(b), "vanilla":signature(v), "expected":signature(v if c["policy"] == "converge" else b)})
        save("p1338-tests-expectations.json", expectations)
        save("p1338-tests-freeze.json", {"at":utc(), "manifest_sha256":MANIFEST_SHA,"protected":hashes,
             "baseline_runs_sha256":sha(D/"p1338-tests-baseline.json"), "expectations_sha256":sha(D/"p1338-tests-expectations.json"),
             "regime":"A/B executed without technical isolation attestation; prior P1337 test-author context retained; P1338 baseline source read only before freeze for local integration",
             "rust_tests":"01_core/src/compiler/eval/bindings/field_access.rs::p1338_tests; frozen before implementation",
             "planned_opaque":"harness-only Unknown controls, not product parity or success"})
        print(json.dumps({"frozen":True,"runs":len(rows),"cases":len(cases),"classes":{k:sum(e["baseline_class"]==k for e in expectations) for k in {e["baseline_class"] for e in expectations}}}))
    else:
        expectations = {(e["id"],e["profile"]):e for e in read("p1338-tests-expectations.json")}
        comparisons = [{"id":r["id"],"profile":r["profile"],"order":r["order"],"class":expectations[r["id"],r["profile"]]["baseline_class"],
                        "verdict":"Satisfied" if signature(r)==expectations[r["id"],r["profile"]]["expected"] else "Violated"} for r in rows]
        save("p1338-tests-comparison.json", {"at":utc(),"manifest_sha256":MANIFEST_SHA,
             "candidate_runs_sha256":sha(D/"p1338-tests-candidate.json"),"freeze_sha256":sha(D/"p1338-tests-freeze.json"),"rows":comparisons})
        failed = [r for r in comparisons if r["verdict"] != "Satisfied"]
        print(json.dumps({"runs":len(rows),"failed":failed}))
        if failed:
            sys.exit(1)

if __name__ == "__main__":
    main()
