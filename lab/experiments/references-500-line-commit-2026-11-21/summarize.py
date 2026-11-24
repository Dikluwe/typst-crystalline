"""Summarize recorded costs without combining a final root with provisional roots."""
from pathlib import Path
import json
import sys

out = Path(sys.argv[1])

def load(name):
    return json.loads((out / name).read_text())

def totals(rows):
    return {key: sum(row[key] for row in rows)
            for key in ("calls", "cpu_ns", "alloc", "realloc", "requested_bytes", "hits", "builds", "edge_queries_saved")}

control = load("control-pages.json")
control_finish = load("control-finish-pages.json")
proof = load("proof-pages.json")
proof_finish = load("proof-finish-pages.json")
names = ("startup", "comment", "comment-2", "visible")
summary = {}
for arm, roots, finish in (("census", control, control_finish),
                           ("cold", proof["cold"], proof_finish["cold"]),
                           ("proof", proof["proof"], proof_finish["proof"])):
    summary[arm] = {}
    for state, steps, final in zip(names, roots, finish, strict=True):
        summary[arm][state] = dict(
            roots=[totals(root) for root in steps],
            total=totals([row for root in steps for row in root]),
            finish=[totals(rows) for rows in final],
            last_root_pages_3_to_23=totals([r for r in steps[-1] if 3 <= r["page"] <= 23]),
        )
(out / "summary.json").write_text(json.dumps(summary, indent=2)+"\n")

from statistics import median

samples = [control] + [item["root"] for item in load("control-repeats.json")]
pages = []
for state_index, state in enumerate(names):
    variants = [sample[state_index] for sample in samples]
    assert len({len(roots) for roots in variants}) == 1
    for root_index, variants_of_root in enumerate(zip(*variants, strict=True), 1):
        for mechanisms in zip(*variants_of_root, strict=True):
            assert len({entry["page"] for entry in mechanisms}) == 1
            pages.append(dict(state=state, root=root_index, page=mechanisms[0]["page"],
                              **{f"{key}_median": median(r[key] for r in mechanisms)
                                 for key in ("calls","cpu_ns","alloc","realloc","requested_bytes")}))
(out / "control-page-medians.json").write_text(json.dumps(pages, indent=2)+"\n")
