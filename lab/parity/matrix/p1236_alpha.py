#!/usr/bin/env python3
"""P1236: probe público de Luma/alpha, adjudicado pelo L0 P1252 vigente."""

from __future__ import annotations

import argparse, csv, hashlib, json, math, re, subprocess
from pathlib import Path

POSITIONS = (
    ("endpoint_first", 0.0), ("epsilon_after_first", 0.000001),
    ("epsilon_before_mid", 0.369999), ("endpoint_mid", 0.37),
    ("epsilon_after_mid", 0.370001), ("epsilon_before_last", 0.999999),
    ("endpoint_last", 1.0),
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path: Path, fields: list[str], rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fields, delimiter="\t", lineterminator="\n")
        writer.writeheader(); writer.writerows(rows)


def gradient(text: str) -> str:
    match = re.search(r"gradient\.(?:linear|radial)\(", text)
    if not match: raise ValueError("gradient prefix absent")
    depth = 0
    for index in range(match.start(), len(text)):
        if text[index] == "(": depth += 1
        elif text[index] == ")":
            depth -= 1
            if depth == 0: return text[match.start():index + 1]
        if depth < 0: raise ValueError("negative depth")
    raise ValueError("unbalanced gradient")


def evaluate(binary: Path, expression: str, at: float) -> dict[str, object]:
    query = f"repr(({expression}).sample({at * 100:.12f}%).components(alpha:true))"
    cp = subprocess.run([str(binary), "eval", "--format", "json", query],
                        text=True, capture_output=True)
    raw = cp.stdout.encode()
    result: dict[str, object] = {"argv": json.dumps([str(binary), "eval", "--format", "json", query], separators=(",", ":")),
                                "exit": cp.returncode, "output_sha256": hashlib.sha256(raw).hexdigest(),
                                "tuple": "", "luma": "", "alpha": "", "status": "Unknown"}
    if cp.returncode: return result
    try:
        value = json.loads(cp.stdout)
        match = re.fullmatch(r"\(\s*([-+0-9.]+)%\s*,\s*([-+0-9.]+)%\s*\)", value)
        if not match: return result
        result.update(tuple=value, luma=float(match.group(1)) / 100,
                      alpha=float(match.group(2)) / 100, status="Available")
    except Exception: pass
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--crystalline", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args(); args.out.mkdir(parents=True, exist_ok=True)
    fixtures = sorted(args.fixtures.glob("candidate-*-luma-alpha-*.typ"))
    if len(fixtures) != 6: raise ValueError(f"expected 6 Luma alpha fixtures, got {len(fixtures)}")

    observations=[]; metrics=[]; boundaries=[]; commands=[]
    binary_hashes = {"vanilla": sha(args.vanilla), "crystalline": sha(args.crystalline)}
    for fixture in fixtures:
        expression = gradient(fixture.read_text())
        for slot, at in POSITIONS:
            pair={}
            for system, binary in (("vanilla", args.vanilla), ("crystalline", args.crystalline)):
                result=evaluate(binary, expression, at); pair[system]=result
                observations.append({"fixture_id": fixture.stem, "probe_slot": slot,
                    "t": format(at, ".12f"), "system": system, "public_tuple": result["tuple"],
                    "luma": result["luma"], "alpha": result["alpha"], "status": result["status"],
                    "output_sha256": result["output_sha256"]})
                commands.append({"fixture_id": fixture.stem, "probe_slot": slot, "system": system,
                    "argv": result["argv"], "declared_reads": f"{fixture};{binary}",
                    "fixture_sha256": sha(fixture), "binary_sha256": binary_hashes[system],
                    "exit_status": result["exit"], "stdout_sha256": result["output_sha256"]})
            if any(pair[s]["status"] != "Available" for s in pair):
                metrics.append({"fixture_id":fixture.stem,"probe_slot":slot,"luma_delta":"",
                    "alpha_delta":"","premultiplied_rgb_delta":"","raw_status":"Unknown",
                    "normative_status":"Unknown","witness":"public tuple unavailable"})
                boundaries.append({"fixture_id":fixture.stem,"probe_slot":slot,
                    "earliest_boundary":"B03","status":"Unknown","owner":"Unknown"}); continue
            ld=float(pair["crystalline"]["luma"])-float(pair["vanilla"]["luma"])
            ad=float(pair["crystalline"]["alpha"])-float(pair["vanilla"]["alpha"])
            pv=float(pair["vanilla"]["luma"])*float(pair["vanilla"]["alpha"])
            pc=float(pair["crystalline"]["luma"])*float(pair["crystalline"]["alpha"])
            pd=math.sqrt(3*(pc-pv)**2)
            raw="Preserved" if ld == 0 and ad == 0 else "Violated"
            # P1252 permits only the alpha-loss divergence; luminance remains independently open.
            if ad != 0 and ld != 0:
                normative="Known-Upstream-Bug+Violated-Luma"
            elif ad != 0:
                normative="Known-Upstream-Bug"
            elif ld != 0:
                normative="Violated-Luma"
            else:
                normative=raw
            metrics.append({"fixture_id":fixture.stem,"probe_slot":slot,"luma_delta":format(ld,".17g"),
                "alpha_delta":format(ad,".17g"),"premultiplied_rgb_delta":format(pd,".17g"),
                "raw_status":raw,"normative_status":normative,
                "witness":"P1252 preserves source alpha; luminance delta is never forgiven"})
            boundaries.append({"fixture_id":fixture.stem,"probe_slot":slot,"earliest_boundary":"B03",
                "status":normative,
                "owner":"Unresolved-Causal-Locus" if "Violated-Luma" in normative else
                        ("Current-L0-P1252" if normative == "Known-Upstream-Bug" else "None")})

    write(args.out/"observations.tsv",list(observations[0]),observations)
    write(args.out/"metrics.tsv",list(metrics[0]),metrics)
    write(args.out/"boundaries.tsv",list(boundaries[0]),boundaries)
    write(args.out/"commands.tsv",list(commands[0]),commands)
    groups={}
    for row in metrics: groups.setdefault(row["normative_status"],[]).append(row["fixture_id"]+"#"+row["probe_slot"])
    clusters=[{"cluster":key,"member_count":len(value),"members":";".join(value)} for key,value in sorted(groups.items())]
    write(args.out/"clusters.tsv",list(clusters[0]),clusters)
    summary={"fixtures":len(fixtures),"positions":len(POSITIONS),"observations":len(observations),
             "pairs":len(metrics),"known_upstream_bug":sum(r["normative_status"]=="Known-Upstream-Bug" for r in metrics),
             "alpha_divergent":sum(float(r["alpha_delta"] or 0) != 0 for r in metrics),
             "violated_luma":sum("Violated-Luma" in r["normative_status"] for r in metrics),
             "preserved":sum(r["normative_status"]=="Preserved" for r in metrics),
             "unknown":sum(r["normative_status"]=="Unknown" for r in metrics),
             "alpha_max":max(abs(float(r["alpha_delta"])) for r in metrics if r["alpha_delta"]!=""),
             "luma_max":max(abs(float(r["luma_delta"])) for r in metrics if r["luma_delta"]!=""),
             "mutation_score":None,
             "attack_regime":"not_executed; no mutation claim",
             "isolation":"EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO"}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")

if __name__ == "__main__": main()
