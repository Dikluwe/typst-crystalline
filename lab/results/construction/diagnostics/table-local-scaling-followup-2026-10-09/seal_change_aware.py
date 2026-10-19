"""Seal only the three owners changed by the borrowed transformation experiment."""
from pathlib import Path
import datetime
import hashlib
import json
import re
import tomllib

D = Path(__file__).resolve().parent
R = D.parents[2]

def framed(data, domain, deps):
    digest = hashlib.sha256(data)
    digest.update(b"\0" + domain + b"\0")
    for path, pin in sorted(deps):
        name = path.encode()
        digest.update(len(name).to_bytes(8, "big"))
        digest.update(name)
        digest.update(b" ")
        digest.update(pin)
    return digest.digest()

def nucleus(path):
    data = (R / path).read_bytes()
    doc = tomllib.loads(data.decode())
    deps = []
    for ref in doc.get("depends", []):
        pin = nucleus(ref["path"])
        assert pin.hex() == ref["sha256"]
        deps.append((ref["path"], pin))
    return framed(data, b"TEKT-NUCLEUS-DEPS-V1", deps)

owners = [
    ("01_core/src/entities/content/traversal.rs", "00_nucleo/prompts/entities/content/traversal.md"),
    ("01_core/src/entities/elements/table.rs", "00_nucleo/prompts/entities/elements/table.md"),
    ("01_core/src/entities/elements/table_cell.rs", "00_nucleo/prompts/entities/elements/table_cell.md"),
]
receipt = {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "owners": []}
for consumer, owner in owners:
    source = re.sub(r"(?m)^//! @updated .*$", "//! @updated 2026-10-10", (R / consumer).read_text())
    spec = (R / owner).read_text()
    code_hash = hashlib.sha256(re.sub(r"(?m)^//! @prompt-hash [^\n]*\n", "", source).encode()).hexdigest()[:8]
    spec = re.sub(r"(?m)^Hash do Código: .*$", f"Hash do Código: {code_hash}", spec)
    deps = []
    for path, expected in re.findall(r"(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([a-f0-9]{64})$", spec):
        pin = nucleus(path)
        assert pin.hex() == expected
        deps.append((path, pin))
    clean = re.sub(r"(?m)^Hash do Código: [^\n]*\n", "", spec).encode()
    prompt_hash = framed(clean, b"TEKT-PROMPT-NUCLEI-V1", deps).hex()[:8]
    source = re.sub(r"(?m)^//! @prompt-hash .*$", f"//! @prompt-hash {prompt_hash}", source)
    (R / consumer).write_text(source)
    (R / owner).write_text(spec)
    receipt["owners"].append({"consumer": consumer, "owner": owner, "code_hash": code_hash,
                              "prompt_hash": prompt_hash, "source_sha256": hashlib.sha256(source.encode()).hexdigest(),
                              "l0_sha256": hashlib.sha256(spec.encode()).hexdigest()})
(D / "seal-change-aware.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt, indent=2))
