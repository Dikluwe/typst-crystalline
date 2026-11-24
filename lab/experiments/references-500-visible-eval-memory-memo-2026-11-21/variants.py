"""Prepare the exact count and byte-budget controls on a disposable ELF copy."""
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess

ORIGINAL_SHA = "82d2daf8fdcb6788e59932e5b85658c495354a342d729fd6cf620d45a9c2935b"
COUNT_TEST = 0x1FCAA70
BUDGET_TEST = 0x1FCAA82


def sha(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()


def make_variants(work: Path):
    source = work / "cap16"
    assert sha(source) == ORIGINAL_SHA
    header = subprocess.check_output(["readelf", "-WS", str(source)], text=True)
    vaddr, offset, size = (int(x, 16) for x in re.search(
        r"\.text\s+PROGBITS\s+([0-9a-f]+)\s+([0-9a-f]+)\s+([0-9a-f]+)", header).groups())

    def patch(arm, edits):
        exe = work / arm
        shutil.copy2(source, exe)
        receipt = []
        with exe.open("r+b") as f:
            for address, before, after in edits:
                assert len(before) == len(after) and vaddr <= address <= vaddr + size - len(before)
                position = offset + address - vaddr
                f.seek(position)
                assert f.read(len(before)) == before
                f.seek(position)
                assert f.write(after) == len(after)
                receipt.append(dict(address=hex(address),offset=position,before=before.hex(),after=after.hex()))
        return dict(sha256=sha(exe), patches=receipt)

    # cmp $15, entry_count; ja evict. Disable only the count jump.
    # The charge addition, cmp $8 MiB and conditional eviction stay intact.
    memory_only = (COUNT_TEST, bytes.fromhex("4983fc0f771d"), bytes.fromhex("4983fc0f6690"))
    byte_pressure = (BUDGET_TEST, bytes.fromhex("483d00008000"), bytes.fromhex("483d00000400"))
    result = {
        "cap16": dict(sha256=sha(source), patches=[]),
        "cap32": patch("cap32", [(COUNT_TEST, bytes.fromhex("4983fc0f771d"), bytes.fromhex("4983fc1f771d"))]),
        "memory": patch("memory", [memory_only]),
        "pressure": patch("pressure", [memory_only, byte_pressure]),
        "vanilla": dict(sha256=sha(work/"vanilla"), patches=[]),
    }
    assert result["cap32"]["sha256"] == "33a22dc21518af328a8c57ee3606dfdbfb1e41a1ebddb836ec393558168d00d1"
    assert result["vanilla"]["sha256"] == "e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b"
    (work/"variants.json").write_text(json.dumps(result, indent=2)+"\n")
