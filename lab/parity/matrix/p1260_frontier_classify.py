#!/usr/bin/env python3
"""P1260: classify P1255/P1256 worst points without changing production."""

from __future__ import annotations

import argparse, csv, hashlib, importlib.util, json
from pathlib import Path


def read(path: Path):
    with path.open(newline="") as handle: return list(csv.DictReader(handle,delimiter="\t"))


def write(path: Path, rows):
    with path.open("w",newline="") as handle:
        writer=csv.DictWriter(handle,list(rows[0]),delimiter="\t",lineterminator="\n")
        writer.writeheader();writer.writerows(rows)


def sha(path: Path): return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--numeric",type=Path,required=True)
    parser.add_argument("--localization",type=Path,required=True)
    parser.add_argument("--oracle-script",type=Path,required=True)
    parser.add_argument("--out",type=Path,required=True)
    args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    numeric={r["fixture_id"]:r for r in read(args.numeric)}
    locations=read(args.localization)
    spec=importlib.util.spec_from_file_location("p1260_oracle",args.oracle_script)
    if spec is None or spec.loader is None: raise ValueError("oracle unavailable")
    oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
    definitions={r["id"]:r for r in oracle.FIXES}
    declared_map={"two":[0,1],"base":[0,.37,1],"coincident":[0,0,1],
        "alpha-first":[0,.37,1],"alpha-mid":[0,.37,1],"alpha-last":[0,.37,1],
        "hue-seam":[0,.37,1]};rows=[]
    for location in locations:
        fid=location["fixture_id"]
        if numeric[fid]["status"]!="Violated": continue
        definition=definitions[fid];t=float(location["worst_t"]);metric=location["metric"]
        exact,_=oracle.query(oracle.expr(definition),(t,));rgba=exact[0]
        bounds=sorted(set(declared_map[definition["stops"]]));distance=min(abs(t-b) for b in bounds)
        nearest=min(bounds,key=lambda b:abs(t-b))
        if distance<=1e-6: position_class="declared-boundary"
        elif nearest in (0,1) and distance<=1/64: position_class="endpoint-near"
        else: position_class="segment-interior"
        saturated=any(channel<=0.0 or channel>=1.0 for channel in rgba[:3])
        limit_key={"color_max":"V_color_max_limit","alpha_max":"V_alpha_max_limit"}[metric]
        limit=float(numeric[fid][limit_key]);error=float(location["error"])
        classification=("no-alpha-divergence" if metric=="alpha_max" and error==0
            else "open" if error>limit else "within-envelope")
        rows.append({"pair":location["pair"],"fixture_id":fid,"role":definition["role"],"metric":metric,
            "worst_t":location["worst_t"],"nearest_declared":format(nearest,".17g"),
            "distance_to_declared":format(distance,".17g"),"position_class":position_class,
            "public_rgb_saturated":str(saturated).lower(),"public_alpha":format(rgba[3],".17g"),
            "error":location["error"],"limit":format(limit,".17g"),"excess":format(error-limit,".17g"),
            "classification":classification})
    write(args.out/"frontiers.tsv",rows)
    open_rows=[r for r in rows if r["classification"]=="open"]
    summary={"violated_fixtures":sum(r["status"]=="Violated" for r in numeric.values()),
        "frontiers":len(rows),"open_frontiers":len(open_rows),
        "position_counts":{key:sum(r["position_class"]==key for r in open_rows) for key in
            ("declared-boundary","endpoint-near","segment-interior")},
        "saturated_open":sum(r["public_rgb_saturated"]=="true" for r in open_rows),
        "scope":"public vanilla exact sample at frozen candidate worst-t; saturation is observable clipping, not proof of source out-of-gamut",
        "unknown_policy":"no owner or pair promoted","mutation_score":None,
        "inputs":{"numeric":sha(args.numeric),"localization":sha(args.localization),"oracle":sha(args.oracle_script)}}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")


if __name__=="__main__":main()
