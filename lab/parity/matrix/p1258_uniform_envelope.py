#!/usr/bin/env python3
"""P1258: upper-bound experiment using frozen vanilla public samples."""

from __future__ import annotations

import argparse, csv, hashlib, importlib.util, json
from pathlib import Path


def read(path: Path):
    with path.open(newline="") as handle: return list(csv.DictReader(handle, delimiter="\t"))


def write(path: Path, rows):
    with path.open("w", newline="") as handle:
        writer=csv.DictWriter(handle,list(rows[0]),delimiter="\t",lineterminator="\n")
        writer.writeheader(); writer.writerows(rows)


def sha(path: Path): return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--fixtures",type=Path,required=True)
    parser.add_argument("--budgets",type=Path,required=True)
    parser.add_argument("--oracle-script",type=Path,required=True)
    parser.add_argument("--out",type=Path,required=True)
    args=parser.parse_args(); args.out.mkdir(parents=True,exist_ok=True)
    fixtures=read(args.fixtures); budgets={r["id"]:r for r in read(args.budgets)}
    spec=importlib.util.spec_from_file_location("p1258_oracle",args.oracle_script)
    if spec is None or spec.loader is None: raise ValueError("oracle unavailable")
    oracle=importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
    definitions={r["id"]:r for r in oracle.FIXES}; results=[]
    for fixture in fixtures:
        fid=fixture["id"]; budget=budgets[fid]
        max_limit=max(0.001,float(budget["V_color_max"]))+0.000001
        p95_limit=float(budget["V_color_p95"])+0.000001
        curve=oracle.adaptive_curve(definitions[fid])
        for cap,stops,color_max,color_p95 in curve:
            results.append({"pair":f'{fixture["kind"]}/{fixture["space"]}',"fixture_id":fid,
                "uniform_cap":cap,"stops":stops,"color_max":format(color_max,".17g"),
                "color_p95":format(color_p95,".17g"),"V_color_max_limit":format(max_limit,".17g"),
                "V_color_p95_limit":format(p95_limit,".17g"),
                "status":"Preserved" if color_max<=max_limit and color_p95<=p95_limit else "Violated"})
    write(args.out/"curve.tsv",results)
    minima=[]
    for fixture in fixtures:
        rows=[r for r in results if r["fixture_id"]==fixture["id"]]
        first=next((r for r in rows if r["status"]=="Preserved"),None)
        minima.append({"pair":f'{fixture["kind"]}/{fixture["space"]}',"fixture_id":fixture["id"],
            "minimum_uniform_cap":first["uniform_cap"] if first else "none<=64",
            "status_at_64":rows[-1]["status"]})
    write(args.out/"minimum.tsv",minima)
    summary={"fixtures":len(fixtures),"experiment":"uniform powers-of-two from frozen vanilla public samples",
        "scope":"upper-bound experiment; not candidate implementation; alpha excluded",
        "pass_at_64":sum(r["status_at_64"]=="Preserved" for r in minima),
        "unknown_policy":"no promotion","mutation_score":None,
        "inputs":{"fixtures":sha(args.fixtures),"budgets":sha(args.budgets),"oracle":sha(args.oracle_script)}}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")


if __name__=="__main__": main()
