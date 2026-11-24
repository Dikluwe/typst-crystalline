"""Measure the capacity-only control without debugger traps."""
import importlib.util
import json
import os
from pathlib import Path
from random import Random
import re
import shutil
import subprocess

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "original_watch",D.parent/"references-500-visible-pre-pdf-2026-11-21/sample.py")
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)
root = fixture.WORK
original = fixture.EXES["candidate"]
assert fixture.measure.sha(original) == "82d2daf8fdcb6788e59932e5b85658c495354a342d729fd6cf620d45a9c2935b"
header = subprocess.check_output(["readelf","-WS",str(original)],text=True)
vaddr, offset, size = (int(group,16) for group in re.search(
    r"\.text\s+PROGBITS\s+([0-9a-f]+)\s+([0-9a-f]+)\s+([0-9a-f]+)",header).groups())
instruction = 0x1fcaa70
assert vaddr <= instruction < vaddr + size - 4
file_offset = offset + instruction - vaddr
image = root/"cap32"
shutil.copy2(original,image)
with image.open("r+b") as exe:
    exe.seek(file_offset)
    assert exe.read(4)==bytes.fromhex("4983fc0f")
    exe.seek(file_offset+3)
    assert exe.write(bytes([31]))==1
(root/"capacity-image.json").write_text(json.dumps(
    dict(file_offset=file_offset,old=15,new=31,source=fixture.measure.sha(original),
         cap32=fixture.measure.sha(image)),indent=2)+"\n")
os.sched_setaffinity(0,{2})
rng=Random(50111)
with (root/"capacity-measurements.jsonl").open("w") as out:
    for i in range(16):
        period=577000 if i%2 else 937000
        for arm in rng.sample(("candidate","cap32"),2):
            fixture.EXES["candidate"] = original if arm=="candidate" else image
            measured=fixture.run(i*2+(arm=="cap32"),"candidate",period)
            print(json.dumps(measured|dict(block=i,arm=arm)),file=out,flush=True)
        print(i+1,flush=True)
