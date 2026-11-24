"""Record exact scope entry counts and bracketed CPU on a separate series."""
import json
import os
from random import Random
import subprocess

from sample import D, WORK, STATES, EXES, cycle, measure

SUFFIXES = {
    "watch_wait": "::watch::wait_for_change_since",
    "serialize": "::serialize_page_stream",
    "compression": "::export::images::compress_zlib",
    "emit": "::export::stream::page::build_page_stream",
    "page_tags": "::export::stream::PageContext::with_pdf_tags",
    "tag_tree": "::emit_tag_structure",
    "font_stream": "::export::builder::fonts::build_font_stream",
    "font_resolve_input": "::collect_fonts_from_doc",
    "font_subset": "::export::subset::subset_font_inner",
}

def symbols(exe, arm):
    nm = subprocess.check_output(["nm","-n","-S","-C","--defined-only",str(exe)],text=True)
    entries = [line.split(" ",3) for line in nm.splitlines()]
    values = {k:sorted({int(x[0],16) for x in entries if len(x)==4
         and x[3].split(".llvm.",1)[0].endswith(s)}) for k,s in SUFFIXES.items()}
    assert all(values.values()), values
    if arm == "candidate":
        assert measure.sha(exe) == "97e1bd15c908c5e7bac422b2b5c374dceb9d27420e72f4af034144664d236cd4"
        # Entry is the taken branch into from_utf8 / byte remapping. Exit is
        # before releasing the old Arc. See candidate-builder.asm.
        values.update(cid_start=[0x29676c2],cid_end=[0x2967ed9],
                      cid_map_start=[0x2966dde],cid_map_end=[0x2966e66])
    return values

def run(block,arm):
    case = WORK / "scopes" / f"{block:02d}-{arm}"
    (case / "edits").mkdir(parents=True)
    for number, (_,source,_) in enumerate(STATES):
        (case/("main.typ" if not number else f"edits/{number}.typ")).write_bytes(source)
    exe=EXES[arm]
    (case/"symbols.json").write_text(json.dumps(symbols(exe,arm)))
    cmd=["taskset","-c","1","gdb","-q","-nx","-batch","-x",str(D/"scopes.gdb"),
         "--args",str(exe),"watch","main.typ","out.pdf","--root",str(case),"--pdf-tagged=true"]
    with (case/"gdb.log").open("wb") as log:
        subprocess.run(cmd,cwd=case,env=cycle.environment(case,arm,False),
                       stdout=log,stderr=subprocess.STDOUT,timeout=90,check=True)
    measured=json.loads((case/"scopes.json").read_text())
    assert len(measured["serialize"]) == len(measured["compression"]) == 23, (arm, measured)
    assert len(measured["emit"]) == len(measured["page_tags"]) == (2 if arm=="candidate" else 23), measured
    assert len(measured.get("cid",[])) == (21 if arm=="candidate" else 0), measured
    assert len(measured.get("cid_map",[])) == (1 if arm=="candidate" else 0), measured
    for number,(state,_,_) in enumerate(STATES):
        assert cycle.cycle.key(arm,case/f"publication-{number}.pdf")==cycle.gold(arm,state)
    return dict(arm=arm,block=block,utc=measure.utc(),nanoseconds=measured)

if __name__=="__main__":
    os.sched_setaffinity(0,{2})
    order=Random(50063)
    with (WORK/"scopes.jsonl").open("w") as output:
        for block in range(8):
            for arm in order.sample(["candidate","hybrid"],2):
                print(json.dumps(run(block,arm)),file=output,flush=True)
            print(block+1,flush=True)
