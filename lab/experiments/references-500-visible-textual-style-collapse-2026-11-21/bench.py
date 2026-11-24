"""Compare the original edit and validate focused textual-style documents."""
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import subprocess

from watch import D, EXES, STATES, WORK, cycle, run

ARMS = ("candidate", "control", "vanilla")
PROTECT = {"TEKT_LAB_MEMO_PROTECT":"1", "TEKT_LAB_MEMO_BYTES":str(8*1024*1024)}
CHECK = D.parent/"references-500-visible-stable-cids-2026-11-21/pdf_check.py"
spec = importlib.util.spec_from_file_location("pdf_check", CHECK)
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)

DIRECTED = {
    "literal": ('''
#show "hello world": it => [LITERAL]
hello #[world]

#show "red blue": it => [MATCHED]
red blue
''', ["LITERAL", "MATCHED"]),
    "regex": ('''
#show regex("ab[0-9]+"): it => strong[REGEX]
ab12, ab7, abx.
''', ["REGEX", "abx"]),
    "lexical-styles": ('''
#set text(fill: blue, size: 9pt)
First #emph[part].
#[#set text(size: 1.5em, fill: red); Nested #strong[page].]
#set text(size: 13pt, fill: black)
Final page.
''', ["First", "Nested", "Final"]),
    "scope-and-guards": ('''
#show "red": it => [red LOCAL]
red
#[#show "blue": it => [blue INNER]; red blue]
blue
''', ["LOCAL", "INNER"]),
    "unicode-and-spacing": ('''
#show regex("éclair"): it => [ÉCLAIR]
An éclair #sym.space.nobreak récit.

A #emph[second line] with #sym.space.thin a space.
''', ["ÉCLAIR", "récit"]),
    "whole-tail": ('''
Before.
#[#show: it => block(fill: yellow)[Wrapped #it]; Body #strong[tail].]
After.
''', ["Before", "Wrapped", "After"]),
}


def directed():
    results = {}
    budget = dict(PROTECT, TEKT_LAB_MEMO_BYTES=str(5*1024*1024//2))
    pressure = {}
    raster_dir = WORK/"directed"/"pressure-raster"
    raster_dir.mkdir(parents=True)
    for i, arm in enumerate(ARMS[:2]):
        EXES["candidate"] = WORK / arm
        row = run(i, "candidate", 0, overrides=budget, directory="directed")
        final = WORK/row["directory"]/"out.pdf"
        pressure[arm] = dict(pdfs=row["all_pdfs"], text_positions=check.text_layout(final),
                             raster=check.render(final,raster_dir,arm))
    assert pressure["candidate"] == pressure["control"]
    assert len(pressure["candidate"]["raster"])==23
    results["original-pressure"] = pressure
    for name,(content,phrases) in DIRECTED.items():
        folder = WORK/"directed"/name
        folder.mkdir(parents=True)
        source = folder/"main.typ"
        source.write_text(content)
        results[name] = {}
        for arm in ARMS[:2]:
            pdf = folder/f"{arm}.pdf"
            with (folder/f"{arm}.log").open("wb") as log:
                subprocess.run(["taskset","-c","1",str(WORK/arm),"compile",str(source),str(pdf),
                                "--root",str(folder),"--pdf-tagged=true"],
                               cwd=folder,env=cycle.environment(folder,"candidate",False),
                               stdout=log,stderr=subprocess.STDOUT,timeout=25,check=True)
            content = subprocess.check_output(["pdftotext",str(pdf),"-"],text=True)
            assert all(phrase in content for phrase in phrases), (name,arm,content)
            results[name][arm] = dict(qdf=cycle.cycle.key("candidate",pdf),
                                     text_positions=check.text_layout(pdf))
            if name in ("lexical-styles", "scope-and-guards"):
                results[name][arm]["raster"] = check.render(pdf,folder,arm)
        assert results[name]["candidate"] == results[name]["control"], name
        print("directed",name,flush=True)
    (WORK/"directed.json").write_text(json.dumps(results,indent=2)+"\n")


def main():
    os.sched_setaffinity(0,{2})
    directed()
    rng = Random(50150)
    periods = [0]*12 + [577000]*12 + [937000]*12
    groups = [periods[i:i+6] for i in range(0,len(periods),6)]
    rng.shuffle(groups)
    with (WORK/"measurements.jsonl").open("w") as output:
        for group, (period,*_) in enumerate(groups):
            for i, order in enumerate(rng.sample(list(permutations(ARMS)),6)):
                block = 6*group+i
                for j, arm in enumerate(order):
                    engine = "vanilla" if arm=="vanilla" else "candidate"
                    EXES[engine] = WORK/arm
                    row = run(3*block+j,engine,period,overrides=PROTECT if engine!="vanilla" else None)
                    print(json.dumps(row|dict(block=block,arm=arm)),file=output,flush=True)
                print(f"{block+1}/36 {period}",flush=True)


if __name__=="__main__":
    main()
