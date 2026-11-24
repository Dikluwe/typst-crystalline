"""Validate an evaluated-content proof and benchmark four watch publications."""
from collections import Counter
from hashlib import sha256
import importlib.util
from itertools import permutations
import gzip
import json
import os
from pathlib import Path
from random import Random
import re
from statistics import median
import subprocess
import sys
import tarfile


D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("initial", D.parent /
    "references-500-visible-initial-realization-2026-11-21" / "bench.py")
initial = importlib.util.module_from_spec(spec)
spec.loader.exec_module(initial)
base, WORK, OUT, STATES = initial.base, initial.WORK, initial.OUT, initial.STATES
ARMS = ("candidate", "proof", "vanilla")
ORIGINAL_ENV = base.environment
ARCHIVE = D.parents[1] / ("results/v16-corrections-2026-10-19/figure-kind-implementation/"
                          "agents/entities/reference-acceptance")


def environment(tmp, arm, trace):
    env = ORIGINAL_ENV(tmp, arm, trace)
    if trace and arm == "proof":
        env["TEKT_LAB_HEADING_REF_TRACE"] = "tree" if os.getenv("TEKT_PROOF_TREE") else "1"
    return env


base.environment = environment


def dump(name, data):
    (OUT / name).write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")


def watch_flags(case):
    flags = []
    for entry in (case / "watch.log").read_text().split("compiled successfully")[:-1]:
        observed = re.findall(r"^lab-heading-ref initial=(true|false)$", entry, re.M)
        assert len(observed) <= 1, (case, observed)
        flags.append(observed[0] == "true" if observed else None)
    return flags


def pdf_details(path):
    qpdf = subprocess.check_output(["qpdf", "--check", path], text=True)
    info = json.loads(subprocess.check_output(["qpdf", "--json", path], text=True))
    objects = info["qpdf"][-1]
    pages = {page["object"]: page["pageposfrom1"] for page in info["pages"]}
    is_object = re.compile(r"^[0-9]+ [0-9]+ R$")

    def resolve(value):
        if isinstance(value, str) and is_object.fullmatch(value):
            return objects["obj:" + value].get("value", value)
        return value

    def normalize(value):
        if isinstance(value, str) and value in pages:
            return {"page": pages[value]}
        if isinstance(value, list):
            return list(map(normalize, value))
        if isinstance(value, dict):
            return {key: normalize(item) for key, item in value.items() if key != "object"}
        return value

    trailer = objects["trailer"]["value"]
    catalog = resolve(trailer["/Root"])
    tree = resolve(resolve(catalog.get("/Names", {})).get("/Dests"))
    names = {}

    def collect(node):
        node = resolve(node)
        if not node:
            return
        values = resolve(node.get("/Names", []))
        for name, dest in zip(values[::2], values[1::2]):
            names[name] = normalize(resolve(dest))
        for child in resolve(node.get("/Kids", [])):
            collect(child)

    collect(tree)
    for name, dest in resolve(catalog.get("/Dests", {})).items():
        names[name] = normalize(resolve(dest))
    links = []
    for page, ordinal in pages.items():
        for key in resolve(resolve(page).get("/Annots", [])):
            annot = resolve(key)
            if annot["/Subtype"] != "/Link":
                continue
            target = normalize(resolve(annot.get("/Dest", annot.get("/A"))))
            links.append([ordinal, annot["/Rect"], target])
    text = subprocess.check_output(["pdftotext", "-layout", path, "-"], text=True)
    boxes = subprocess.check_output(["pdftotext", "-bbox-layout", path, "-"])
    return dict(qpdf=qpdf, pages=len(pages), links=links, names=names, outlines=normalize(info["outlines"]),
                text=text, positions=sha256(boxes).hexdigest())


def compare(a, b):
    return {key: a[key] == b[key] for key in ("pages", "links", "names", "outlines", "text", "positions")}


def original():
    evidence = []
    runs = {arm: base.watch(WORK / "original" / arm, arm, STATES, trace=True, keep=True) for arm in ARMS}
    for i, (name, *_unused) in enumerate(STATES):
        paths = {arm: WORK / "original" / arm / f"{name}.pdf" for arm in ARMS}
        digests = {arm: runs[arm][i]["pdf_hash"] for arm in ARMS}
        assert all(digests[arm] == base.golden(arm, name) for arm in ("candidate", "vanilla"))
        a, b, v = [pdf_details(paths[arm]) for arm in ARMS]
        rasters_equal = None
        if name == "visible" or digests["candidate"] != digests["proof"]:
            rasters_equal = base.render(paths["candidate"], paths["candidate"].parent, "render-" + name) ==                             base.render(paths["proof"], paths["proof"].parent, "render-" + name)
        evidence.append(dict(state=name, digests=digests, same=compare(a, b), rasters_equal=rasters_equal,
                             links={arm: data["links"] for arm, data in zip(ARMS, (a,b,v))},
                             destinations={arm:data["names"] for arm,data in zip(ARMS, (a,b,v))},
                             outlines={arm:data["outlines"] for arm,data in zip(ARMS, (a,b,v))}))
    flags = watch_flags(WORK / "original" / "proof")
    assert len(flags) == 4
    physical = {arm: base.cycle.opening.log_groups(WORK / "original" / arm / "watch.log")
                for arm in ("candidate", "proof")}
    fresh = {arm: compile_case(WORK / "original" / ("fresh-" + arm), arm, *STATES[-1][1:]) for arm in ARMS}
    fresh_receipt = {arm: dict(qdf=result["qdf"], same_watch=result["qdf"] == evidence[-1]["digests"][arm],
                               initial=result["initial"], code=result["code"], diagnostics=result["diag"])
                     for arm, result in fresh.items()}
    dump("original.json", dict(evidence=evidence, initial=flags, physical=physical, fresh=fresh_receipt))
    if all(flag is not False for flag in flags):
        os.environ["TEKT_PROOF_TREE"] = "1"
        try:
            path = WORK / "proof-tree"
            base.watch(path, "proof", STATES[:1], trace=True, keep=True)
            with gzip.open(OUT / "proof-tree.log.gz", "wb") as output:
                output.write((path / "watch.log").read_bytes())
        finally:
            del os.environ["TEKT_PROOF_TREE"]
    print("original", flags, [(e["state"], e["same"], e["rasters_equal"]) for e in evidence], flush=True)
    return {state["state"]: state["digests"]["proof"] for state in evidence}


def compile_case(folder, arm, source, leaf=None, bibliography=None):
    folder.mkdir(parents=True)
    tmp = folder / "tmp"
    tmp.mkdir()
    base.cycle.base.runner.setup_sources(folder, source, leaf)
    if bibliography is not None:
        (folder / "refs.yml").write_text(bibliography)
    output = folder / "out.pdf"
    command = [WORK / arm, "compile", folder / "main.typ", output, "--root", folder, "--pdf-tagged=true"]
    run = subprocess.run(command, cwd=folder, env=environment(tmp, arm, trace=True), text=True,
                         capture_output=True, timeout=45)
    output_log = (run.stdout + run.stderr).replace(str(folder), "<case>")
    (folder / "compile.log").write_text(output_log)
    flags = re.findall(r"^lab-heading-ref initial=(true|false)$", output_log, re.M)
    diagnostic = "\n".join(l for l in output_log.splitlines()
                           if not l.startswith(("lab-heading-ref", "lab-", "pre-bidi-pages")))
    digest = base.cycle.cycle.key("vanilla" if arm == "vanilla" else "candidate", output) if output.exists() else None
    return dict(code=run.returncode, diag=diagnostic, qdf=digest, initial=flags,
                content=pdf_details(output) if output.exists() else None)


def directed():
    head = b'#set heading(numbering: "1.")\n'
    figure = b'#figure([ART], kind: image, caption: [CAPTION], supplement: [FIG-SUPPLEMENT]) <fig>\n'
    callback = figure.replace(b"supplement: [FIG-SUPPLEMENT]", b"supplement: it => [CALLBACK-SUPPLEMENT]")
    bibliography = """collision:
  type: Book
  title: A citation
  author: Doe, Jane
  date: 2020
"""
    inputs = {
        "forward-heading": (head + b"See @target.\n\n= Target <target>\n", None, None, False),
        "missing-heading": (head + b"= Target <other>\n\nSee @target.\n", None, None, True),
        "duplicates": (head + b"= First <target>\n\n= Second <target>\n\nSee @target.\n", None, None, True),
        "unnumbered-heading": (b"= Heading <target>\n\nSee @target.\n", None, None, True),
        "figure-explicit": (head + figure + b"See @fig.\n", None, None, True),
        "figure-callback": (head + callback + b"See @fig.\n", None, None, True),
        "figure-ref-override": (head + callback + b'See #ref(<fig>, supplement: [OVERRIDE]).\n', None, None, True),
        "nested-figure-supplement": ((ARCHIVE / "nested-references.typ").read_bytes(), None, None, True),
        "figure-unnumbered": ((ARCHIVE / "unnumbered-target.typ").read_bytes(), None, None, True),
        "figure-supplement-error": ((ARCHIVE / "inner-panic-reference.typ").read_bytes(), None, None, True),
        "bibliography-collision": (head + b'= Heading <collision>\n\nSee @collision.\n\n#bibliography("refs.yml")\n',
                                   None, bibliography, True),
        "show-heading": (head + b'#show heading: it => emph(it.body)\n= Heading <target>\nSee @target.\n',
                         None, None, True),
        "dynamic-label": (head + b'#context [= Generated <target>]\n\nSee @target.\n', None, None, True),
    }
    output = {}
    for name, (source, leaf, bib, expected) in inputs.items():
        folder = WORK / "directed" / name
        a, b, v = [compile_case(folder / arm, arm, source, leaf, bib) for arm in ARMS]
        for row in (a,b,v):
            if row["content"] is not None:
                row["text_excerpt"] = row["content"]["text"].strip()[:700]
        same = compare(a["content"], b["content"]) if a["content"] and b["content"] else None
        token = {
            "figure-explicit": "FIG-SUPPLEMENT",
            "figure-callback": "CALLBACK-SUPPLEMENT",
            "figure-ref-override": "OVERRIDE",
            "figure-supplement-error": "REF_INNER",
        }.get(name)
        token_seen = ({arm: token in (r.get("text_excerpt", "") + r["diag"])
                       for arm, r in zip(ARMS, (a, b, v))} if token else None)
        output[name] = dict(expected_initial=expected, initial=b["initial"], same=same,
                            token=token, token_seen=token_seen,
                            status_equal=a["code"] == b["code"], diagnostics_equal=a["diag"] == b["diag"],
                            qdf_equal=a["qdf"] == b["qdf"], results=dict(zip(ARMS, (a,b,v))))
        print("directed", name, a["code"], b["code"], v["code"], b["initial"],
              same, output[name]["qdf_equal"], flush=True)
        dump("directed.json", output)

    visible = STATES[-1][1]
    switched = visible.replace(b"See @s0.", b"See @s13.", 1)
    assert switched != visible
    to_figure = switched.replace(b"= Section 13 <s13>", figure.rstrip().replace(b"<fig>", b"<s13>"), 1)
    assert to_figure != switched
    sources = [("heading", visible, None), ("retarget", switched, None), ("figure", to_figure, None),
               ("restore", switched, None)]
    evidence = {}
    folder = WORK / "directed" / "watch-target-edit"
    for arm in ARMS:
        base.watch(folder / arm, arm, sources, trace=True, keep=True)
    flags = watch_flags(folder / "proof")
    for name, *_ in sources:
        a, b, v = [pdf_details(folder / arm / f"{name}.pdf") for arm in ARMS]
        evidence[name] = dict(same=compare(a,b), links={arm: item["links"] for arm, item in zip(ARMS, (a,b,v))})
    def target_counts(links):
        return Counter(json.dumps(item[-1], sort_keys=True) for item in links)

    changes = {}
    for arm in ARMS:
        before = target_counts(evidence["heading"]["links"][arm])
        after = target_counts(evidence["retarget"]["links"][arm])
        changes[arm] = dict(removed=before-after, added=after-before)
    dump("target-edit.json", dict(initial=flags, states=evidence, ref_change=changes))
    print("target-edit", flags, {name: e["same"] for name,e in evidence.items()}, flush=True)


def time_cycles(proof_gold):
    rows, blocks = [], {}
    orders = list(permutations(ARMS)) * 3
    Random(95401).shuffle(orders)
    for number, order in enumerate(orders):
        blocks[number] = {}
        for arm in order:
            group = base.watch(WORK / "nominal" / f"{number:02d}-{arm}", arm, STATES, keep=(arm=="proof"))
            if arm == "proof":
                assert all(r["pdf_hash"] == proof_gold[r["state"]] for r in group)
            blocks[number][arm] = {r["state"]: r["cpu_ns"]/1e6 for r in group}
            rows.extend(dict(r, block=number) for r in group)
        print(f"time {number+1}/{len(orders)}", flush=True)
    (WORK / "measurements.jsonl").write_text("".join(json.dumps(row) + "\n" for row in rows))
    output = {}
    for name in [s[0] for s in STATES] + ["cycle"]:
        values = {arm: [sum(b[arm].values()) if name=="cycle" else b[arm][name] for b in blocks.values()]
                  for arm in ARMS}
        output[name] = dict(absolute={a: initial.decoder.stats(v) for a,v in values.items()},
                            paired={a: initial.decoder.stats([p-c for p,c in zip(values["proof"],values[a])])
                                    for a in ("candidate", "vanilla")})
    dump("timing.json", output)


def main():
    os.sched_setaffinity(0, {2})
    try:
        proof_gold = original()
        if "--timing-only" not in sys.argv:
            directed()
        if "--checks-only" not in sys.argv:
            time_cycles(proof_gold)
    finally:
        with tarfile.open(OUT / "validation-logs.tar.gz", "w:gz") as archive:
            for prefix in ("original", "directed"):
                for path in sorted((WORK / prefix).rglob("*.log")):
                    archive.add(path, arcname=str(path.relative_to(WORK)))


if __name__ == "__main__":
    main()
