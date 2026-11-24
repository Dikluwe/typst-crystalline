"""Reduce the verified t0..t5 and PDF boundary sequence into paired CPU intervals."""
import json
from pathlib import Path
from random import Random
from statistics import mean, median
import sys

PHASES = ("evaluation", "module_index", "context_preparation", "remaining_layout",
          "bidi_shape_positions", "after_layout_before_export", "pdf_export")
ARMS = ("candidate", "bridge", "proof")


def summary(values):
    rng = Random(59214)
    draws = sorted(mean(rng.choices(values, k=len(values))) for _ in range(10000))
    return dict(median=median(values), mean=mean(values), mean_ci95=[draws[250], draws[9749]])


def main(root):
    raw = json.loads((root / "phase-clock.json").read_text())
    table = {}
    for arm in ARMS:
        probe = raw[arm]
        paged, pdf = probe["ranges"].values()
        pcs = [int(pc, 16) for pc in probe["pcs"]]
        assert len(pcs) == len(PHASES) + 1, (arm, probe["pcs"])
        assert all(paged[0] <= pc < paged[1] for pc in pcs[:6]), (arm, pcs, paged)
        assert all(pdf[0] <= pc < pdf[1] for pc in pcs[6:]), (arm, pcs, pdf)
        rows = {}
        for row in probe["records"]:
            times = [ns for _, ns in row["marks"]["4"]]
            ms = {name: (end-begin)/1e6 for name, begin, end in zip(PHASES, times, times[1:])}
            rows[row["block"]] = ms
        table[arm] = rows
    labels = PHASES
    formatted = {arm: {name: summary([row[name] for row in values.values()]) for name in labels}
                 for arm, values in table.items()}
    pairs = {f"{a}-{b}": {name: summary([row[name]-table[b][block][name]
                                          for block, row in table[a].items()]) for name in labels}
             for a,b in (("candidate","bridge"),("bridge","proof"),("candidate","proof"))}
    result = dict(arms=formatted, paired=pairs, per_block=table)
    (root / "phase-reduced.json").write_text(json.dumps(result, indent=2) + "\n")
    for arm, vals in formatted.items():
        print(arm, {name: round(vals[name]["median"], 4) for name in labels})


if __name__ == "__main__":
    main(Path(sys.argv[1]))
