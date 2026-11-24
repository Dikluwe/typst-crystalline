"""Resolve sampled stacks and attribute mutually exclusive pipeline phases."""
from bisect import bisect_right
from collections import Counter
from functools import lru_cache
import json
from pathlib import Path
from random import Random
from statistics import median
import struct
import sys
import tarfile

RECORD = struct.Struct("<101Q")
ARMS = ("candidate", "hybrid", "vanilla")


def symbols(path):
    entries = {}
    for line in path.read_text().splitlines():
        fields = line.split(maxsplit=3)
        if len(fields) == 4 and fields[2] in ("t", "T", "w", "W"):
            address, size = (int(s, 16) for s in fields[:2])
            entries[address] = (size, fields[3])
    return entries, sorted(entries)


def resolver(maps, exe, table, arm):
    entries, addresses = table
    mappings = []
    for line in maps.splitlines():
        fields = line.split(maxsplit=5)
        lo, hi = (int(s, 16) for s in fields[0].split("-"))
        mappings.append((lo, hi, int(fields[2], 16),
                         fields[5] if len(fields) == 6 else "[anonymous]"))
    base = next(lo for lo, _, offset, file in mappings if offset == 0 and file == exe)

    @lru_cache(maxsize=None)
    def resolve(ip):
        for lo, hi, _, file in mappings:
            if lo <= ip < hi:
                if file == exe:
                    relative = ip - base
                    if arm == "candidate" and 0x29676c2 <= relative < 0x2967ed9:
                        return "[candidate:translate_operator_cids]"
                    if arm == "vanilla" and 0x1dc5350 <= relative < 0x1dc57c5:
                        return "[vanilla:deferred_page_stream]"
                    index = bisect_right(addresses, relative) - 1
                    if index >= 0:
                        addr = addresses[index]
                        size, name = entries[addr]
                        if relative < addr + size:
                            return name
                    return "[ELF unknown]"
                return f"[{Path(file).name.strip('[]')}]"
        return "[unmapped]"
    return resolve


def stacks(archive, row, tables):
    case = row["directory"]
    def read(name):
        with archive.extractfile(f"{case}/{name}") as f:
            return f.read()

    resolve = resolver(read("maps").decode(), row["exe"], tables[row["arm"]], row["arm"])
    markers, counts = [], Counter()
    truncated = 0
    threads = set()
    for record in RECORD.iter_unpack(read("samples.bin")):
        if record[0] == 2:
            markers.append(record[:4])
            continue
        assert record[0] == 1 and record[3] == 1 and 0 < record[4] <= 96
        threads.add(record[2])
        truncated += record[4] == 96
        stack = (resolve(record[1]), *(resolve(ip - 1) for ip in record[7:5 + record[4]]
                                       if ip != record[1]))
        counts[stack] += 1
    assert [m[3] for m in markers] == [1, 2] and len(threads) == 1
    return counts, dict(truncated=truncated, samples=counts.total(),
                        main_ns=markers[1][2]-markers[0][2],
                        process_ns=markers[1][1]-markers[0][1])


def category(arm, stack):
    def contains(*parts):
        return any(part in frame for part in parts for frame in stack)
    if arm == "vanilla":
        if not contains("typst_pdf::", "krilla::"):
            return "outside_pdf"
        if contains("typst_pdf::tags::", "krilla::interchange::tagging::"):
            return "tags"
        if contains("krilla::serialize::SerializeContext::register_font",
                    "krilla::text::FontContainer::"):
            return "fonts"
        if contains("krilla::serialize", "krilla::text::cid", "krilla::text::Font"):
            if contains("serialize_fonts", "krilla::text::cid", "subsetter::", "skrifa::",
                        "krilla::text::font::"):
                return "fonts"
        if contains("deflate_encode", "miniz_oxide::deflate", "zlib::", "flate2::"):
            if contains("[vanilla:deferred_page_stream]", "krilla::page::"):
                return "compression"
            return "other_compression"
        if contains("typst_pdf::text::", "krilla::surface::", "krilla::content::"):
            return "emit"
        return "pdf_other"
    if contains("::compile_paged_in_session", "::eval_to_module_in_session"):
        return "outside_pdf"
    if not contains("::compile_to_pdf_bytes_impl"):
        return "outside_pdf"
    if contains("[candidate:translate_operator_cids]"):
        return "cid"
    if contains("::lab_pdf_reuse::same_items", "::lab_pdf_reuse::same_bidi"):
        return "page_compare"
    if contains("::serialize_page_stream"):
        return "compression" if contains("::compress_zlib","miniz_oxide::deflate") else "stream_other"
    if contains("::emit_tag_structure", "::with_pdf_tags"):
        return "tags"
    if contains("::export::stream::page::build_page_stream"):
        return "emit"
    if contains("::export::fonts::", "::font_binding::partition", "::font_binding::bind_document",
                "::export::subset::", "subsetter::", "::collect_fonts_from_doc", "::resolve_fonts",
                "::bind_owned_pdf_fonts", "::build_font_stream", "::per_font_used_glyphs"):
        return "fonts"
    if contains("::compress_zlib", "miniz_oxide::deflate"):
        return "other_compression"
    return "pdf_other"


def stats(values):
    rng = Random(50046)
    draws = sorted(median(rng.choices(values, k=len(values))) for _ in range(10000))
    return dict(median=median(values), ci95=[draws[250], draws[9749]])


def main(root):
    data = [json.loads(line) for line in (root / "measurements.jsonl").read_text().splitlines()]
    assert len(data) == 96
    tables = {arm:symbols(root / f"{arm}.symbols") for arm in ARMS}
    vanilla_symbols = (root / "vanilla-symbols-verified").exists()
    if vanilla_symbols:
        tables["vanilla"] = symbols(root / "vanilla-symbols-twin.txt")
    by_sample, inventory, chains = [], {arm:Counter() for arm in ARMS}, {arm:Counter() for arm in ARMS}
    with tarfile.open(root / "profiles.tar.gz", "r:gz") as archive:
        for row in data:
            if not row["period_ns"]:
                continue
            arm = row["arm"]
            counts, detail = stacks(archive, row, tables)
            phases = Counter()
            for stack, count in counts.items():
                phase = category(arm, stack) if arm != "vanilla" or vanilla_symbols else None
                if phase is not None:
                    phases[phase] += count
                for symbol in set(stack):
                    inventory[arm][symbol] += count
                typed = tuple(name for name in stack if name.startswith(("typst", "<typst", "krilla")))
                chains[arm][typed[-9:]] += count
            factor = detail["main_ns"] / 1e6 / counts.total()
            estimates = {k:count*factor for k,count in phases.items()}
            if estimates:
                estimates["pdf_total"] = sum(v for k,v in estimates.items() if k != "outside_pdf")
            by_sample.append(dict(row, **detail, phase_ms=estimates,
                                  unsampled_ns=row["cpu_ns"]-detail["main_ns"]))
    out = dict(samples=by_sample, arms={}, nominal_paired_cpu_ms={}, vanilla_symbol_verified=vanilla_symbols)
    nominal_by_block = {arm:{r["block"]:r["cpu_ns"]/1e6 for r in data if r["arm"]==arm and not r["period_ns"]}
                        for arm in ARMS}
    for arm in ("hybrid", "vanilla"):
        out["nominal_paired_cpu_ms"][arm] = stats([
            value - nominal_by_block[arm][block] for block,value in nominal_by_block["candidate"].items()])
    for arm in ARMS:
        nominal = [r["cpu_ns"]/1e6 for r in data if r["arm"]==arm and not r["period_ns"]]
        all_rows = [r for r in by_sample if r["arm"]==arm]
        out["arms"][arm] = {"nominal_cpu_ms":stats(nominal), "profiles":{}}
        for period in (0, 577000, 937000):
            selected = [r for r in all_rows if not period or r["period_ns"]==period]
            total = sum(r["main_ns"] for r in selected)
            phases = sorted({phase for row in selected for phase in row["phase_ms"]})
            out["arms"][arm]["profiles"][str(period)] = dict(
                n=len(selected), samples=sum(row["samples"] for row in selected),
                truncated=sum(row["truncated"] for row in selected),
                sampled_cpu_ms=median(row["cpu_ns"] for row in selected)/1e6,
                unsampled_ms=median(row["unsampled_ns"] for row in selected)/1e6,
                coverage=sum(row["samples"] * row["period_ns"] for row in selected) / total,
                phases_ms={phase:median(row["phase_ms"].get(phase,0) for row in selected)
                           for phase in phases})
    with (root / "stacks.txt").open("w") as f:
        for arm in ARMS:
            print(arm, file=f)
            for name,count in inventory[arm].most_common(125):
                print(count, name, file=f)
            print("CALL CHAINS", file=f)
            for stack,count in chains[arm].most_common(35):
                print(count, stack, file=f)
    (root / "phases.json").write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps(out["arms"], indent=2))


if __name__ == "__main__":
    main(Path(sys.argv[1]))
