"""Author-side proof of tests-only integration. Run only before P1338 C."""
import datetime
import difflib
import hashlib
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def save(name, value):
    path = D / name
    assert not path.exists()
    body = json.dumps(value, ensure_ascii=False, indent=2)
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n"
    patch += "".join("+" + line + "\n" for line in body.splitlines())
    subprocess.run(["apply_patch"], input=patch + "*** End Patch\n", text=True, cwd=ROOT, check=True)

m = json.loads((D / "p1338-manifest-r1.json").read_text())
assert sha(D / "p1338-manifest-r1.json") == "14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843"
old = m["pre_candidate_source"]
module = (D / "p1338-tests-module.rs.txt").read_text()
current = (ROOT / m["source"]).read_text()
original = '''    fn p1337_preserve_array_ast_message_and_total_span_debt() {
        // Array is an excluded baseline debt, not a P1337 parity correction.
        check_ast(
            "#((1, 2).missing)",
            "(1, 2).missing",
            "array does not contain field \\"missing\\"",
        );
    }'''
successor = '''    fn p1338_successor_array_missing_message_and_field_span() {
        // Array is an excluded baseline debt, not a P1337 parity correction.
        check_ast("#((1, 2).missing)", "missing", "cannot access fields on type array");
    }'''
assert old.count(original) == 1
expected = old.replace(original, successor, 1)
expected = expected.replace("#[cfg(test)]\nmod p1337_tests {", module + "\n#[cfg(test)]\nmod p1337_tests {", 1)
assert current == expected, "out-of-scope source edits"
command = ["cargo", "fmt", "--all", "--", "--check"]
check = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
assert check.returncode == 0, check.stdout + check.stderr
state = {k:subprocess.check_output(v,cwd=ROOT).decode() for k,v in {
    "head":["git","rev-parse","HEAD"],
    "changed_files":["git","diff","HEAD","--name-only"],
    "diff_stat":["git","diff","HEAD","--stat"],
}.items()}
save("p1338-tests-local-patch.json", {
    "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "manifest_sha256": sha(D / "p1338-manifest-r1.json"),
    "state": state, "source": m["source"],
    "before_sha256": m["source_sha256"],
    "tests_only_after_sha256": sha(ROOT / m["source"]),
    "module_sha256": sha(D / "p1338-tests-module.rs.txt"),
    "all_other_source_bytes_preserved": True,
    "legacy_comment_preserved": True,
    "legacy_call_formatter_only": "Rustfmt folds the existing check_ast call after authorized name/anchor/message succession; identical call/comparator and case, no other old test changed.",
    "fmt": {"argv": command, "exit": check.returncode, "stdout": check.stdout, "stderr": check.stderr},
    "patch": "".join(difflib.unified_diff(old.splitlines(True), current.splitlines(True), fromfile="a/"+m["source"], tofile="b/"+m["source"])),
})
