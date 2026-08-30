#!/usr/bin/env python3
"""P1259: compare midpoint and quarter-probe localized refinement."""

from __future__ import annotations

import argparse, csv, hashlib, importlib.util, json, math
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
    parser.add_argument("--fixtures",type=Path,required=True)
    parser.add_argument("--budgets",type=Path,required=True)
    parser.add_argument("--oracle-script",type=Path,required=True)
    parser.add_argument("--out",type=Path,required=True)
    args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    fixtures=read(args.fixtures);budgets={r["id"]:r for r in read(args.budgets)}
    spec=importlib.util.spec_from_file_location("p1259_oracle",args.oracle_script)
    if spec is None or spec.loader is None: raise ValueError("oracle unavailable")
    oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
    definitions={r["id"]:r for r in oracle.FIXES};results=[]
    declared_map={"two":[0,1],"base":[0,.37,1],"coincident":[0,0,1],
        "alpha-first":[0,.37,1],"alpha-mid":[0,.37,1],"alpha-last":[0,.37,1],
        "hue-seam":[0,.37,1]}

    for fixture in fixtures:
        fid=fixture["id"];definition=definitions[fid];mesh=oracle.mesh_for(definition,4096)
        exact,_=oracle.query(oracle.expr(definition),tuple(mesh));colors=dict(zip(mesh,exact))
        def at(t):
            if t not in colors:
                value,_=oracle.query(oracle.expr(definition),(t,));colors[t]=value[0]
            return colors[t]
        def err(x,y): return oracle.dist(oracle.prem(x),oracle.prem(y))
        def mix(x,y,u): return tuple(a*(1-u)+b*u for a,b in zip(x,y))
        bounds=sorted(set(declared_map[definition["stops"]]));budget=budgets[fid]
        limits=(max(.001,float(budget["V_color_max"]))+.000001,
            float(budget["V_color_p95"])+.000001,float(budget["V_alpha_max"])+.000001,
            float(budget["V_alpha_p95"])+.000001)
        for policy,probes in (("midpoint",(.5,)),("quarters",(.25,.5,.75))):
            positions=set(bounds);decisions=0
            def refine(a,b,depth):
                nonlocal decisions
                decisions+=1;left=at(a);right=at(b)
                worst=max(err(at(a+(b-a)*u),mix(left,right,u)) for u in probes)
                if worst>.001 and depth<6:
                    middle=(a+b)/2;positions.add(middle)
                    refine(a,middle,depth+1);refine(middle,b,depth+1)
            for a,b in zip(bounds,bounds[1:]):
                if b>a: refine(a,b,0)
            sampled=sorted((t,at(t)) for t in positions)
            ce=[];ae=[]
            for t,e in zip(mesh,exact):
                a=oracle.approx(sampled,t);ce.append(err(e,a));ae.append(abs(e[3]-a[3]))
            percentile=lambda values: sorted(values)[math.ceil(.95*len(values))-1]
            metrics=(max(ce),percentile(ce),max(ae),percentile(ae))
            results.append({"pair":f'{fixture["kind"]}/{fixture["space"]}',"fixture_id":fid,
                "policy":policy,"emitted_stops":len(sampled),"decisions":decisions,
                "color_max":format(metrics[0],".17g"),"color_p95":format(metrics[1],".17g"),
                "alpha_max":format(metrics[2],".17g"),"alpha_p95":format(metrics[3],".17g"),
                "status":"Preserved" if all(x<=y for x,y in zip(metrics,limits)) else "Violated"})
    write(args.out/"results.tsv",results)
    summary={"fixtures":len(fixtures),"threshold":.001,"max_depth":6,
        "midpoint_pass":sum(r["policy"]=="midpoint" and r["status"]=="Preserved" for r in results),
        "quarters_pass":sum(r["policy"]=="quarters" and r["status"]=="Preserved" for r in results),
        "scope":"frozen vanilla public samples; experimental, not productive implementation",
        "unknown_policy":"no promotion","mutation_score":None,
        "inputs":{"fixtures":sha(args.fixtures),"budgets":sha(args.budgets),"oracle":sha(args.oracle_script)}}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")


if __name__=="__main__":main()
