"""Symbolize CPU samples from the two exact watch ELFs."""
from bisect import bisect_right
from collections import Counter
from functools import lru_cache
import json
from pathlib import Path
from random import Random
from statistics import median
import struct
import sys

D = Path(sys.argv[1])
RECORD = struct.Struct("<101Q")


def symbols(path):
    entries = {}
    for line in path.read_text().splitlines():
        fields = line.split(maxsplit=3)
        if len(fields) == 4 and fields[2] in ("t", "T", "w", "W"):
            address, size = (int(s, 16) for s in fields[:2])
            entries[address] = (size, fields[3])
    return entries, sorted(entries)


def resolver(path, exe, table):
    entries, addresses = table
    mappings = []
    for line in path.read_text().splitlines():
        fields = line.split(maxsplit=5)
        lo, hi = (int(s, 16) for s in fields[0].split("-"))
        mappings.append((lo, hi, int(fields[2], 16),
                         fields[5] if len(fields) == 6 else "[anonymous]"))
    # Both PIE executables have an initial PT_LOAD at virtual address and file offset zero.
    base = next(lo for lo, _, offset, file in mappings if offset == 0 and file == exe)

    @lru_cache(maxsize=None)
    def resolve(ip):
        for lo, hi, _, file in mappings:
            if lo <= ip < hi:
                if file == exe:
                    relative = ip - base
                    index = bisect_right(addresses, relative) - 1
                    if index >= 0:
                        addr = addresses[index]
                        size, name = entries[addr]
                        if addr <= relative < addr + size:
                            return name
                    return "[ELF unknown]"
                return f"[{Path(file).name.strip('[]')}]"
        return "[unmapped]"

    return resolve


def case(row, table):
    directory = D / row["directory"]
    resolve = resolver(directory / "maps", row["exe"], table)
    data = (directory / "samples.bin").read_bytes()
    markers, stacks = [], Counter()
    truncated = 0
    for record in RECORD.iter_unpack(data):
        if record[0] == 2:
            markers.append(record[:4])
            continue
        assert record[0] == 1 and record[3] == 1 and 0 < record[4] <= 96
        truncated += record[4] == 96
        stack = (resolve(record[1]), *(resolve(ip - 1) for ip in record[7:5 + record[4]]
                                       if ip != record[1]))
        stacks[stack] += 1
    assert [m[3] for m in markers] == [1, 2], markers
    return stacks, dict(truncated=truncated, samples=stacks.total(),
                        main_ns=markers[1][2] - markers[0][2],
                        process_ns=markers[1][1] - markers[0][1])


def inventory(rows, stacks):
    for arm in ("both", "off", "vanilla"):
        inclusive, exclusive, chains = Counter(), Counter(), Counter()
        selected = [r for r in rows if r["arm"] == arm and r["period_ns"]]
        for row in selected:
            for stack, n in stacks[row["directory"]].items():
                for name in set(stack):
                    inclusive[name] += n
                exclusive[stack[0]] += n
                chain = tuple(s for s in stack if s.startswith(("typst", "<typst", "krilla")))
                chains[chain[-8:]] += n
        print(f"\n{arm}: nominal {median(r['cpu_ns'] for r in rows if r['arm']==arm and not r['period_ns']) / 1e6:.2f} ms / sample count {exclusive.total()}")
        for counter, limit in ((inclusive, 105), (exclusive, 65), (chains, 30)):
            for name, n in counter.most_common(limit):
                print(n, name)
            print()


def tekt_phase(stack):
    def includes(*patterns):
        return any(pattern in frame for frame in stack for pattern in patterns)

    if includes("::eval_to_module_in_session"):
        return "evaluation"
    if includes("::compile_paged_in_session"):
        if includes("::compiler::layout::physical_memo::layout_with"):
            return "physical_memo"
        if includes("::shaper::traversal::shape_document_with_faces"):
            return "document_shaping"
        return "other_paged"
    if includes("::compile_to_pdf_bytes_impl"):
        return "pdf"
    return "outside_pipeline"


INCLUSIVE = {
    "stabilization_scope": "::context_stabilization::Session::stabilize",
    "root_traversal_runner": "::compiler::layout::convergence::run_planned",
    "context_substitution": "::pipeline::substitute_context_blocks",
    "context_realization": "::context_stabilization::Session::realize",
    "index_physical_steps": "::index_physical_steps",
    "running_body_preparation": "::prepare_running_body",
    "outline_candidate": "::prepare_outline_candidate",
    "system_font_scanning": "fontdb::Database::load_system_fonts",
    "font_catalog_build": "FontCatalogSession::catalog::",
    "pdf_backend": "::export::lab_multifont",
}


def tekt_phases(rows, stacks):
    result = []
    for row in rows:
        if not row["period_ns"] or row["arm"] == "vanilla":
            continue
        counts, inclusive = Counter(), Counter()
        for stack, n in stacks[row["directory"]].items():
            counts[tekt_phase(stack)] += n
            for key, pattern in INCLUSIVE.items():
                if any(pattern in frame for frame in stack):
                    inclusive[key] += n
        factor = row["main_ns"] / 1e6 / counts.total()
        phases = {phase: n * factor for phase, n in counts.items()}
        phases["paged_total"] = sum(phases.get(k, 0) for k in (
            "physical_memo", "document_shaping", "other_paged"))
        phases["unprofiled"] = (row["cpu_ns"] - row["main_ns"]) / 1e6
        result.append(dict(arm=row["arm"], period_ns=row["period_ns"],
                           block=row["block"], samples=row["samples"],
                           inclusive_ms={key: inclusive[key] * factor for key in INCLUSIVE},
                           truncated=row["truncated"], phase_ms=phases,
                           cpu_ms=row["cpu_ns"]/1e6,
                           sampling_coverage=row["samples"]*row["period_ns"]/row["main_ns"]))
    return result


def trace_case(row):
    directory = D / row["directory"]
    events = json.loads((directory / "timings.json").read_text())
    pairs = list(struct.iter_unpack("<QQ", (directory / "clock.bin").read_bytes()))
    clocks = dict(pairs)
    assert len({event["tid"] for event in events}) == 1
    shift = [round(event["ts"] * 1000) for event in events]
    offset, = (wall for wall, _ in pairs if all(wall + delta in clocks for delta in shift[:16]))
    cpu = [clocks[offset + delta] for delta in shift]
    assert cpu == sorted(cpu)
    totals, counts, pending, bounds = Counter(), Counter(), [], {}
    for event, t in zip(events, cpu):
        name = event["name"]
        if event["ph"] == "B":
            pending.append((name, t))
        else:
            previous, start = pending.pop()
            assert name == previous
            totals[name] += t - start
            counts[name] += 1
            bounds[name] = (start, t)
    assert not pending and counts["compile"] == counts["compile once"] == counts["eval"] == counts["pdf"] == 1
    assert counts["layout document"] == 3
    before = bounds["compile once"][0]
    after = bounds["compile once"][1]
    phases = {phase: duration / 1e6 for phase, duration in dict(
        before_compile_once=before,
        source=totals["create source"],
        evaluation=totals["eval"],
        paged_total=totals["compile"]-totals["eval"]-totals["create source"],
        layout_document=totals["layout document"],
        pdf=totals["pdf"],
        other_inside_compile_once=totals["compile once"]-totals["compile"]-totals["pdf"],
        after_compile_once=row["cpu_ns"]-after,
    ).items()}
    return dict(block=row["block"], group=row["group"], matched_events=len(events),
                cpu_ms=row["cpu_ns"]/1e6, phase_ms=phases,
                iterations_ms={str(n): totals[f"iter ({n})"] / 1e6 for n in (1, 2, 3)})


def packed(rows):
    if not rows:
        return {}
    phases = sorted({key for row in rows for key in row["phase_ms"]})
    return dict(n=len(rows), cpu_ms=median(r["cpu_ms"] for r in rows),
                samples=sum(r.get("samples", 0) for r in rows),
                truncated=sum(r.get("truncated", 0) for r in rows),
                phases_ms={phase: median(r["phase_ms"].get(phase, 0) for r in rows)
                           for phase in phases})


def effect(values):
    rng = Random(500)
    sampling = sorted(median(rng.choices(values, k=len(values))) for _ in range(10000))
    return dict(median=median(values), ci95=[sampling[250], sampling[9749]])


def read(name):
    return [json.loads(line) for line in (D / name).read_text().splitlines()]


def summarize(rows, stacks):
    tekt = tekt_phases(rows, stacks)
    nominal = {arm: {r["block"]:r["cpu_ns"]/1e6 for r in rows
                     if not r["period_ns"] and r["arm"] == arm}
               for arm in ("both", "off", "vanilla")}
    nominal_diff = {}
    for a,b in (("both","vanilla"),("off","vanilla"),("both","off")):
        nominal_diff[f"{a}-{b}"] = effect([nominal[a][i]-nominal[b][i] for i in sorted(nominal[a])])
    vanilla = [trace_case(row) for row in read("trace-cpu.jsonl")]
    font_file = D / "font-control.jsonl"
    font = read(font_file.name) if font_file.exists() else []
    vanilla.extend(trace_case(row) for row in font if row["group"].startswith("vanilla-cpu-trace"))
    out = dict(nominal_ms={arm: effect(list(values.values())) for arm, values in nominal.items()},
               paired_nominal_delta_ms=nominal_diff,
               tekt_profiles={arm: {str(frequency): packed(
                   [r for r in tekt if r["arm"] == arm and
                    (not frequency or r["period_ns"] == frequency)])
                   for frequency in (0, 677000, 937000)} for arm in ("both", "off")},
               vanilla_traces={group: packed([r for r in vanilla if r["group"] == group])
                               for group in sorted({r["group"] for r in vanilla})},
               tekt_rows=tekt, vanilla_rows=vanilla)
    out["tekt_inclusive_ms"] = {
        arm: {key: median(r["inclusive_ms"][key] for r in tekt if r["arm"] == arm)
              for key in INCLUSIVE}
        for arm in ("both", "off")}
    if font:
        font_nominal = {f"{group}/{arm}": {r["block"]: r["cpu_ns"]/1e6 for r in font
                                          if r["group"] == group and r["arm"] == arm}
                        for group,arm in (("font-default","both"),("font-default","vanilla"),
                                          ("font-ignored","vanilla"))}
        a, b = (font_nominal[f"font-{group}/vanilla"] for group in ("default", "ignored"))
        out["vanilla_font_delta_ms"] = effect([a[i]-b[i] for i in sorted(a)])
        out["font_control_nominal_ms"] = {key: effect(list(value.values()))
                                          for key, value in font_nominal.items()}
    (D / "summary.json").write_text(json.dumps(out, indent=2) + "\n")


def main():
    rows = [json.loads(line) for line in (D / "measurements.jsonl").read_text().splitlines()]
    tables = {arm: symbols(D / f"{arm}.symbols") for arm in ("candidate", "vanilla")}
    stacks = {}
    for row in rows:
        if row["period_ns"]:
            stacks[row["directory"]], stats = case(row, tables[Path(row["exe"]).name])
            row.update(stats)
    inventory(rows, stacks)
    summarize(rows, stacks)


if __name__ == "__main__":
    main()
