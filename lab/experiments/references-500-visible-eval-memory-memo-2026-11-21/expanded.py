"""Verify the byte policy above 32 live entries, outside the timed fixture."""
import importlib.util
import json
import os

from bench import D, STATES, WORK, census, cycle, make_variants

checker = D.parent/"references-500-visible-stable-cids-2026-11-21/pdf_check.py"
spec = importlib.util.spec_from_file_location("pdf_check",checker)
pdf_check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pdf_check)


def extend(source):
    marker = b"= Section 19 <s19>"
    start = source.index(marker)
    template_end = source.index(b"across lines.\n\n]\n",start)+len(b"across lines.\n\n]\n")
    template = source[start:template_end]
    extra = [template.replace(b"Section 19",f"Section {i}".encode())
                      .replace(b"s19",f"s{i}".encode()) for i in range(20,36)]
    return source[:template_end]+b"".join(extra)+source[template_end:]


os.sched_setaffinity(0,{2})
states = [(name,extend(source),leaf) for name,source,leaf in STATES]
make_variants(WORK)
extended = {}
for arm in ("cap32","memory"):
    entries = census(arm,states,verify_reference=False)
    pdfs = [WORK/"census"/arm/f"pub-{i}.pdf" for i in range(4)]
    hashes = [cycle.cycle.key("candidate",pdf) for pdf in pdfs]
    semantic = [pdf_check.text_layout(pdf) for pdf in pdfs]
    assert entries[0]["loops"]==36, entries
    extended[arm] = dict(entries=entries,qdf=hashes,text_positions=semantic)
a,b = extended["cap32"],extended["memory"]
assert a["qdf"]==b["qdf"] and a["text_positions"]==b["text_positions"]
assert a["entries"][-1]["hit"]==0 and b["entries"][-1]["hit"]==35
assert a["entries"][-1]["body"]==900 and b["entries"][-1]["body"]==25
assert b["entries"][-1]["max_entries"]==37 and b["entries"][-1]["evictions"]==0
(WORK/"expanded.json").write_text(json.dumps(extended,indent=2)+"\n")
