"""Refresh only reciprocal code-hash metadata, never sealed normative text."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
seal = json.loads((ROOT / "00_nucleo/diagnosticos/p1307-r6-seal.json").read_text())
paths = subprocess.check_output(["git", "diff", "HEAD", "--name-only"], cwd=ROOT, text=True).splitlines()
patch = "*** Begin Patch\n"
changes = []
for name in paths:
    if not name.endswith(".rs"):
        continue
    source = ROOT / name
    data = source.read_bytes()
    header = re.search(rb"(?m)^//! @prompt (.+)$", data)
    if not header:
        continue
    owner = ROOT / header.group(1).decode().strip()
    text = owner.read_text()
    semantic = re.sub(r"(?m)^Hash do Código: [^\n]*$", "Hash do Código: <metadata>", text)
    expected = seal["contract_semantic_hashes"].get(str(owner))
    assert expected == hashlib.sha256(semantic.encode()).hexdigest(), str(owner)
    assert len(re.findall(rb"(?m)^//! @prompt-hash [^\n]*\n", data)) == 1
    clean = re.sub(rb"(?m)^//! @prompt-hash [^\n]*\n", b"", data)
    digest = hashlib.sha256(clean).hexdigest()[:8]
    previous = re.search(r"(?m)^Hash do Código: (\S+)$", text).group(1)
    if previous != digest:
        patch += f"*** Update File: {owner}\n@@\n-Hash do Código: {previous}\n+Hash do Código: {digest}\n"
        changes.append({"source": name, "source_sha256": hashlib.sha256(data).hexdigest(),
                        "owner": str(owner), "before": previous, "after": digest})
patch += "*** End Patch"
print(json.dumps({"patch": patch, "changes": changes}, ensure_ascii=False))
