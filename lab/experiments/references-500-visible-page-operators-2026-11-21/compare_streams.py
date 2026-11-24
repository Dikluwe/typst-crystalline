"""Check decoded streams before and after the original edit in fresh Tekt PDFs."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys


def pages(pdf):
    data = json.loads(subprocess.check_output(["qpdf","--json","--json-key=pages",pdf]))
    return [b"".join(subprocess.check_output(["qpdf",f"--show-object={s.split()[0]}",
              "--filtered-stream-data",pdf]) for s in row["contents"]) for row in data["pages"]]


def without_cid_numbers(data):
    # This is a diagnostic comparison only. The exporter validates the
    # mapping for each font slot before rewriting any operand.
    return re.sub(rb"^.* T[Jj]$", lambda m:re.sub(rb"<([0-9A-F]+)>",
        lambda n:b"<"+b"0"*len(n[1])+b">",m[0]),data,flags=re.M)


a,b = (pages(Path(sys.argv[1])/f"{state}.pdf") for state in ("comment-2", "visible"))
assert len(a) == len(b)
print(json.dumps([dict(page=i, before=hashlib.sha256(x).hexdigest(), after=hashlib.sha256(y).hexdigest(),
                       exact=x==y, equal_without_cid_numbers=without_cid_numbers(x)==without_cid_numbers(y))
       for i,(x,y) in enumerate(zip(a,b),1)],indent=2))
