"""Pre-C mechanical succession proof; never run against candidate source."""
import datetime
import difflib
import hashlib
import json
import pathlib
import re
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
def tokens(text):
    return re.findall(r'"(?:[^"\\]|\\.)*"|//[^\n]*|\w+|[^\s]', text)
def without_optional_trailing_comma(text):
    items = tokens(text)
    return [v for i,v in enumerate(items) if not (v == "," and i+1 < len(items) and items[i+1] == ")")]

a = (D / "p1337-tests-module-r1.rs.txt").read_text()
b = (D / "p1337-tests-module-r2.rs.txt").read_text()
assert re.findall(r'"(?:[^"\\]|\\.)*"', a) == re.findall(r'"(?:[^"\\]|\\.)*"', b)
assert without_optional_trailing_comma(a) == without_optional_trailing_comma(b)
changes = [v for v in difflib.ndiff(tokens(a), tokens(b)) if v[0] in "+-"]
assert changes == ["+ ,", "+ ,"]
m = json.loads((D / "p1337-manifest-r1.json").read_text())
source = (ROOT / m["source"]).read_text()
assert source.count(b + "\n") == 1
restored = source.replace(b + "\n", a + "\n", 1)
previous = json.loads((D / "p1337-tests-local-patch-r1.json").read_text())
assert hashlib.sha256(restored.encode()).hexdigest() == previous["tests_only_after_sha256"]
formatter = ["rustfmt", "--edition", "2021", "--config-path", "rustfmt.toml", "--emit", "stdout"]
outputs = [subprocess.check_output(formatter + [str(D / name)], cwd=ROOT).decode().split("\n\n", 1)[1] for name in ("p1337-tests-module-r1.rs.txt", "p1337-tests-module-r2.rs.txt")]
assert outputs[0] == outputs[1]
command = ["cargo", "fmt", "--all", "--", "--check"]
check = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
assert check.returncode == 0
receipt = {
    "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "manifest_sha256": sha(D / "p1337-manifest-r1.json"),
    "predecessor_freeze_sha256": sha(D / "p1337-tests-freeze-r1.json"),
    "literal_sequence_exact": True,
    "strict_token_equality": False,
    "only_token_additions": changes,
    "qualification": "rustfmt adds two optional trailing commas to existing two-element tuples; not strictly whitespace-only. All other tokens, literals, assertions and order are identical.",
    "rustfmt_canonical_outputs_equal": True,
    "rustfmt_version": subprocess.check_output(["rustfmt", "--version"]).decode(),
    "rustfmt_config_sha256": sha(ROOT / "rustfmt.toml"),
    "all_other_source_bytes_equal_r1": True,
    "tests_only_after_sha256": hashlib.sha256(source.encode()).hexdigest(),
    "module_r1_sha256": sha(D / "p1337-tests-module-r1.rs.txt"),
    "module_r2_sha256": sha(D / "p1337-tests-module-r2.rs.txt"),
    "fmt": {"argv": command, "exit": check.returncode, "stdout": check.stdout, "stderr": check.stderr},
    "mechanical_diff": "".join(difflib.unified_diff(a.splitlines(True), b.splitlines(True), fromfile="p1337-tests-module-r1.rs.txt", tofile="p1337-tests-module-r2.rs.txt")),
}
save("p1337-tests-format-proof.json", receipt)
save("p1337-tests-local-patch-r2.json", {
    "at": receipt["at"], "manifest_sha256": receipt["manifest_sha256"],
    "before_sha256": m["source_sha256"], "tests_only_after_sha256": receipt["tests_only_after_sha256"],
    "source": m["source"], "predecessor_patch_sha256": sha(D / "p1337-tests-local-patch-r1.json"),
    "format_proof_sha256": sha(D / "p1337-tests-format-proof.json"),
    "patch": "".join(difflib.unified_diff(m["pre_candidate_source"].splitlines(True), source.splitlines(True), fromfile="a/" + m["source"], tofile="b/" + m["source"])),
})
