#!/usr/bin/env python3
"""P1238: public polar-gradient sampling for Oklch, Hsl and Hsv."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import re
import subprocess
from pathlib import Path

POSITIONS = (0.0, 0.000001, 0.25, 0.5, 0.75, 0.999999, 1.0)
SPACES = ("oklch", "hsl", "hsv")
VARIANTS = ("linear", "radial")


def stops(space: str, scenario: str) -> tuple[str, str]:
    seam = ("rgb(230,0,140)", "rgb(255,80,120)") if space == "oklch" else (
        "rgb(255,0,43)", "rgb(255,43,0)")
    if scenario == "seam-forward": return seam
    if scenario == "seam-reverse": return seam[1], seam[0]
    if scenario == "no-wrap": return "rgb(255,128,0)", "rgb(0,255,128)"
    if scenario == "low-chroma": return "rgb(128,128,128)", "rgb(255,43,0)"
    if scenario == "alpha-first": return seam[0] + ".transparentize(60%)", seam[1]
    if scenario == "alpha-last": return seam[0], seam[1] + ".transparentize(60%)"
    raise ValueError(scenario)


def expression(variant: str, space: str, scenario: str) -> str:
    first, last = stops(space, scenario)
    args = f"({first},0%),({last},100%),space:color.{space}"
    if variant == "linear": args += ",angle:0deg"
    else: args += ",center:(50%,50%),radius:50%,focal-center:(50%,50%),focal-radius:0%"
    return f"gradient.{variant}({args})"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_sha(path: Path) -> str:
    return sha(path.read_bytes())


def evaluate(binary: Path, expr: str, at: float) -> dict[str, object]:
    query = f"repr(({expr}).sample({at * 100:.12f}%).components(alpha:true))"
    argv = [str(binary), "eval", "--format", "json", query]
    cp = subprocess.run(argv, text=True, capture_output=True)
    raw = cp.stdout.encode()
    result: dict[str, object] = {"argv": json.dumps(argv, separators=(",", ":")),
        "exit": cp.returncode, "raw": "", "sha256": sha(raw),
        "stderr_sha256": sha(cp.stderr.encode()), "status": "Unknown", "values": []}
    if cp.returncode != 0: return result
    try:
        text = json.loads(cp.stdout)
        tokens = [token.strip() for token in text.strip("()").split(",")]
        values = []
        for token in tokens:
            match = re.fullmatch(r"([-+0-9.eE]+)(%|deg)?", token)
            if not match: return result
            value = float(match.group(1))
            if match.group(2) == "%": value /= 100.0
            values.append(value)
        if len(values) != 4: return result
        result.update(raw=text, values=values, status="Available")
    except Exception:
        pass
    return result


def deltas(space: str, vanilla: list[float], crystalline: list[float]) -> tuple[list[float], float, float]:
    hue_index = 2 if space == "oklch" else 0
    component = []
    for index, (left, right) in enumerate(zip(vanilla, crystalline)):
        if index == hue_index:
            component.append(abs((right - left + 180.0) % 360.0 - 180.0))
        else:
            component.append(abs(right - left))
    alpha = component[3]
    non_hue = max(value for index, value in enumerate(component) if index not in (hue_index, 3))
    return component, non_hue, alpha


def write(path: Path, fields: list[str], rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fields, delimiter="\t", lineterminator="\n")
        writer.writeheader(); writer.writerows(rows)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--crystalline", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args(); args.out.mkdir(parents=True, exist_ok=True)
    scenarios = ("seam-forward", "seam-reverse", "no-wrap", "low-chroma", "alpha-first", "alpha-last")
    observations=[]; metrics=[]; commands=[]
    variants = tuple(reversed(VARIANTS)) if args.reverse else VARIANTS
    spaces = tuple(reversed(SPACES)) if args.reverse else SPACES
    scenario_order = tuple(reversed(scenarios)) if args.reverse else scenarios
    positions = tuple(reversed(POSITIONS)) if args.reverse else POSITIONS
    binary_hashes = {"vanilla": file_sha(args.vanilla), "crystalline": file_sha(args.crystalline)}
    for variant in variants:
        for space in spaces:
            for scenario in scenario_order:
                expr = expression(variant, space, scenario)
                for at in positions:
                    pair = {}
                    for system, binary in (("vanilla", args.vanilla), ("crystalline", args.crystalline)):
                        item = evaluate(binary, expr, at); pair[system] = item
                        observations.append({"pair":f"{variant.title()}/{space.title()}","scenario":scenario,
                            "t":format(at,".12f"),"system":system,"public_tuple":item["raw"],
                            "status":item["status"],"output_sha256":item["sha256"]})
                        commands.append({"pair":f"{variant.title()}/{space.title()}","scenario":scenario,
                            "t":format(at,".12f"),"system":system,"argv":item["argv"],
                            "binary_sha256":binary_hashes[system],"exit_status":item["exit"],
                            "stdout_sha256":item["sha256"],"stderr_sha256":item["stderr_sha256"]})
                    status = "Unknown"; hue_delta=non_hue=alpha=""
                    if all(pair[key]["status"] == "Available" for key in pair):
                        ds, non_hue_value, alpha_value = deltas(space, pair["vanilla"]["values"], pair["crystalline"]["values"])
                        hue_index = 2 if space == "oklch" else 0
                        hue_delta=ds[hue_index]; non_hue=non_hue_value; alpha=alpha_value
                        status = "Preserved" if hue_delta <= 0.01 and non_hue <= 0.0001 and alpha <= 0.0001 else "Violated"
                    metrics.append({"pair":f"{variant.title()}/{space.title()}","scenario":scenario,
                        "t":format(at,".12f"),"hue_delta_deg":hue_delta,"non_hue_max":non_hue,
                        "alpha_delta":alpha,"status":status})
    observations.sort(key=lambda r:(r["pair"],r["scenario"],r["t"],r["system"]))
    metrics.sort(key=lambda r:(r["pair"],r["scenario"],r["t"]))
    commands.sort(key=lambda r:(r["pair"],r["scenario"],r["t"],r["system"]))
    write(args.out/"observations.tsv",list(observations[0]),observations)
    write(args.out/"metrics.tsv",list(metrics[0]),metrics)
    write(args.out/"commands.tsv",list(commands[0]),commands)
    pairs=[]
    for variant in VARIANTS:
        for space in SPACES:
            name=f"{variant.title()}/{space.title()}"; rows=[row for row in metrics if row["pair"]==name]
            pairs.append({"pair":name,"preserved":sum(r["status"]=="Preserved" for r in rows),
                "violated":sum(r["status"]=="Violated" for r in rows),"unknown":sum(r["status"]=="Unknown" for r in rows),
                "total":len(rows),"classification":"Preserved" if all(r["status"]=="Preserved" for r in rows) else "Unknown" if any(r["status"]=="Unknown" for r in rows) else "Violated"})
    write(args.out/"pairs.tsv",list(pairs[0]),pairs)
    summary={"fixtures":len(VARIANTS)*len(SPACES)*len(scenarios),"positions":len(POSITIONS),
        "observations":len(observations),"pairs":pairs,
        "mutation_score":None,"attack_regime":"not executed; no mutation claim",
        "binary_sha256":binary_hashes,
        "isolation":"EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO"}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")


if __name__ == "__main__": main()
