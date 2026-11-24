"""Read direct CPU brackets and call counts without a patched product build."""
from contextlib import contextmanager
import json
from pathlib import Path
import re
from statistics import median
import struct
import subprocess


D = Path(__file__).resolve().parent
LABELS = ("observe", "finish", "normalization", "columns", "index", "recipes")
TABLE = struct.Struct(f"<{3 + len(LABELS) * 8}Q")
WORD = struct.Struct("<Q")
POINTS = {
    "observe": (0x220AA40, 0x2E106A0, "::map_content"),
    "finish": (0x220AC14, 0, "::finish_occurrences"),
    "normalization": (0x220AD8E, 0x2E10758, "::map_content"),
    "columns": (0x220ADD6, 0x2E12E90, "::wrap_page_columns"),
    "index": (0x220AE88, 0x2E11890, "::introspect_with_introspector"),
    "recipes": (0x220ABB5, 0, "::recipes::realize"),
}


def configure(bench):
    binary, out = bench.WORK / "candidate", bench.OUT
    listing = (out / "tekt-realization.asm").read_text()
    symbols = {}
    for line in (bench.WORK / "candidate.symbols").read_text().splitlines():
        fields = line.split(maxsplit=3)
        if len(fields) == 4 and fields[2] in ("t", "T"):
            symbols.setdefault(int(fields[0], 16), []).append(fields[3])
    relocations = subprocess.check_output(["readelf", "-rW", binary], text=True)
    sites = {}
    for label, (site, got, token) in POINTS.items():
        line, = re.findall(rf"^\s*{site:x}:.*\bcall\s+(.+)$", listing, flags=re.M)
        if got:
            assert f"# {got:x} " in line, line
            target, = re.findall(rf"^0*{got:x}\s+.*R_X86_64_RELATIVE\s+([0-9a-f]+)$",
                                  relocations, flags=re.M)
        else:
            target = line.split()[0]
        callee = int(target, 16)
        name, = [symbol for symbol in symbols[callee] if token in symbol]
        sites[label] = (site, callee, got, name)
    assert int((D / "normalization-pcs.txt").read_text(), 16) == sites["normalization"][0] + 6
    return sites


@contextmanager
def tracing(base, probe, table):
    original_sources, original_env = base.cycle.base.runner.setup_sources, base.environment

    def setup(case, *args):
        path = case / "realization.bin"
        if not path.exists():
            path.write_bytes(bytes(table.size))
        with path.open("r+b") as stream:
            publication, = WORD.unpack(stream.read(WORD.size))
            stream.seek(0)
            stream.write(WORD.pack(publication + 1))
        original_sources(case, *args)

    def env(tmp, arm, trace):
        values = original_env(tmp, arm, trace)
        values.update(LD_PRELOAD=str(probe), TEKT_REALIZATION_TRACE=str(tmp.parent / "realization.bin"))
        return values

    base.cycle.base.runner.setup_sources = setup
    base.environment = env
    try:
        yield
    finally:
        base.cycle.base.runner.setup_sources = original_sources
        base.environment = original_env


def run(bench):
    work, out, base = bench.WORK, bench.OUT, bench.base
    points = configure(bench)
    (out / "direct-points.json").write_text(json.dumps(points, indent=2) + "\n")
    (work / "points.inc").write_text("".join(
        f"{{{i}, {points[k][0]:#x}, {points[k][1]:#x}, {points[k][2]:#x}}},\n" for i, k in enumerate(LABELS)))
    probe = work / "direct.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared",
                    "-I", work, "-o", probe, D / "direct.c"], check=True)
    (out / "direct.sha256").write_text(subprocess.check_output(["sha256sum", probe], text=True))
    observations = []
    control_states = []
    for state, data, leaf in bench.STATES:
        text, count = re.subn(rb"See @s([0-9]+)\.", rb"See section \1.", data)
        assert count == 20, (state, count)
        control_states.append((state, text, leaf))
    with tracing(base, probe, TABLE):
        for i in range(24):
            case = work / "direct" / str(i)
            rows = base.watch(case, "candidate", bench.STATES)
            publication, calibration_count, calibration_ns, *totals = TABLE.unpack(
                (case / "realization.bin").read_bytes())
            assert publication == 4 and calibration_count == 256
            clock = calibration_ns / calibration_count
            phases = []
            for offset in range(0, len(totals), len(LABELS) * 2):
                pairs = totals[offset:offset + len(LABELS) * 2]
                phases.append({label: dict(calls=calls, raw_ns=ns, adjusted_ns=ns - calls * clock)
                               for label, calls, ns in zip(LABELS, pairs[::2], pairs[1::2])})
            assert all(p["calls"] == 1 for p in phases[-1].values()), (case, phases)
            observations.append(dict(clock_ns=clock, phases=phases, measurements=rows))
        control = work / "no-references-probe"
        control_rows = base.watch(control, "candidate", control_states, keep=True)
        direct_count = TABLE.unpack((control / "realization.bin").read_bytes())[3:]
    clean = base.watch(work / "no-references-uninstrumented", "candidate", control_states, keep=True)
    assert [r["pdf_hash"] for r in clean] == [r["pdf_hash"] for r in control_rows]
    for state, *_ in control_states:
        subprocess.run(["qpdf", "--check", control / f"{state}.pdf"], check=True, capture_output=True)
    result = dict(
        no_references_control=dict(patched=control_rows, uninstrumented=clean, counters=direct_count),
        summaries={label: dict(median_ms=median(v := [r["phases"][-1][label]["adjusted_ns"] / 1e6
                                                      for r in observations]),
                               min_ms=min(v), max_ms=max(v)) for label in LABELS},
        occurrences_median_ms=median(sum(r["phases"][-1][k]["adjusted_ns"] / 1e6
                                          for k in ("observe", "finish")) for r in observations),
        rows=observations,
    )
    (out / "direct.json").write_text(json.dumps(result, indent=2) + "\n")
    print("direct Tekt", result["summaries"], flush=True)


def run_vanilla(bench):
    work, out, base = bench.WORK, bench.OUT, bench.base
    labels = ("document_realize", "physical_index")
    table = struct.Struct("<28Q")
    probe = work / "vanilla-direct.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared",
                    "-o", probe, D / "vanilla-direct.c"], check=True)
    (out / "vanilla-direct.sha256").write_text(subprocess.check_output(["sha256sum", probe], text=True))
    observations = []
    with tracing(base, probe, table):
        for i in range(24):
            case = work / "vanilla-direct" / str(i)
            rows = base.watch(case, "vanilla", bench.STATES)
            publication, calibration_count, calibrated, _, *totals = table.unpack(
                (case / "realization.bin").read_bytes())
            assert publication == 4 and calibration_count == 256
            clock = calibrated / calibration_count
            phases = []
            for offset in range(0, len(totals), len(labels)*3):
                pairs = totals[offset:offset + len(labels)*3]
                phases.append({label: dict(calls=calls, adjusted_ns=ns - calls * clock,
                                           first_adjusted_ns=first - clock if calls else 0)
                               for label, calls, ns, first in zip(labels, pairs[::3], pairs[1::3], pairs[2::3])})
            assert all(p["calls"] > 0 for p in phases[-1].values()), (case, phases)
            observations.append(dict(clock_ns=clock, phases=phases, measurements=rows))
    summary = {label: {metric: median(r["phases"][-1][label][metric] for r in observations)
                       for metric in ("calls", "adjusted_ns", "first_adjusted_ns")} for label in labels}
    (out / "vanilla-direct.json").write_text(
        json.dumps(dict(summary=summary, rows=observations), indent=2) + "\n")
    print("direct vanilla", summary, flush=True)
