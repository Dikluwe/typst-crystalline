#!/usr/bin/env python3
"""P1256: localiza worst-t/canal/intervalo nos SVGs congelados de P1237."""

from __future__ import annotations

import argparse, csv, hashlib, importlib.util, math, xml.etree.ElementTree as ET
from pathlib import Path


def read(path: Path):
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write(path: Path, rows):
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader(); writer.writerows(rows)


def sha(path: Path): return hashlib.sha256(path.read_bytes()).hexdigest()
def local(tag: str): return tag.rsplit("}", 1)[-1]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--svg-root", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args(); args.out.mkdir(parents=True, exist_ok=True)
    fixtures = read(args.fixtures)
    spec = importlib.util.spec_from_file_location("p1256_oracle", args.oracle_script)
    if spec is None or spec.loader is None: raise ValueError("oracle unavailable")
    oracle = importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
    definitions = {row["id"]: row for row in oracle.FIXES}
    rows = []
    for fixture in fixtures:
        fid = fixture["id"]; definition = definitions[fid]
        source = args.fixture_root / f"{fid}.typ"; svg = args.svg_root / f"{fid}.svg"
        if sha(source) != fixture["fixture_sha256"]: raise ValueError("fixture drift: " + fid)
        tag = fixture["kind"] + "Gradient"
        servers = [e for e in ET.parse(svg).getroot().iter() if local(e.tag) == tag and any(local(c.tag) == "stop" for c in e)]
        if len(servers) != 1: raise ValueError(f"{fid}: expected one server")
        stops = []
        for child in servers[0]:
            if local(child.tag) != "stop": continue
            color = oracle.rgba(child.get("stop-color")); opacity = float(child.get("stop-opacity", "1"))
            stops.append((oracle.parse_offset(child.get("offset")), color[:3] + (color[3] * opacity,)))
        mesh = oracle.mesh_for(definition, 4096)
        exact, query_hash = oracle.query(oracle.expr(definition), tuple(mesh))
        color_errors=[]; alpha_errors=[]
        for t, expected in zip(mesh, exact):
            actual=oracle.approx(stops,t)
            color_errors.append(oracle.dist(oracle.prem(expected),oracle.prem(actual)))
            alpha_errors.append(abs(expected[3]-actual[3]))
        for metric, errors in (("color_max",color_errors),("alpha_max",alpha_errors)):
            index=max(range(len(errors)),key=errors.__getitem__); t=mesh[index]
            left=max(i for i,s in enumerate(stops) if s[0] <= t)
            right=min(left+1,len(stops)-1)
            width=stops[right][0]-stops[left][0]
            declared={"two":[0,1],"base":[0,.37,1],"coincident":[0,0,1],
                "alpha-first":[0,.37,1],"alpha-mid":[0,.37,1],"alpha-last":[0,.37,1],
                "hue-seam":[0,.37,1]}[definition["stops"]]
            segments=[(a,b) for a,b in zip(declared,declared[1:]) if b>a and a <= t <= b]
            segment_left,segment_right=segments[-1]
            subdivisions=0 if width <= 0 else round((segment_right-segment_left)/width)
            depth="exact-or-coincident" if subdivisions <= 0 else str(round(math.log2(subdivisions)))
            rows.append({"pair":f'{fixture["kind"]}/{fixture["space"]}',"fixture_id":fid,"metric":metric,
                "worst_t":format(t,".17g"),"error":format(errors[index],".17g"),
                "left_offset":format(stops[left][0],".17g"),"right_offset":format(stops[right][0],".17g"),
                "interval_width":format(width,".17g"),"declared_segment":f"{segment_left:.17g}:{segment_right:.17g}",
                "inferred_subdivisions":subdivisions,"inferred_dyadic_depth":depth,"cap_64_hit":str(subdivisions >= 64).lower(),
                "emitted_stops":len(stops),"query_sha256":query_hash,"svg_sha256":sha(svg)})
    write(args.out / "localization.tsv", rows)
    summary=[{"field":"fixtures","value":len(fixtures)},
             {"field":"observations","value":len(rows)},
             {"field":"scope","value":"frozen SVG corpus; emitted intervals, not private runtime trace"},
             {"field":"mutation_score","value":"NOT_MEASURED"},
             {"field":"attestation","value":"EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO"}]
    write(args.out / "summary.tsv", summary)


if __name__ == "__main__": main()
