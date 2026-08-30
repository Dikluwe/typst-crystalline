#!/usr/bin/env python3
"""P1237: recomputa quatro pares usando somente V_* e SVGs congelados."""

from __future__ import annotations

import argparse, csv, hashlib, importlib.util, json, xml.etree.ElementTree as ET
from pathlib import Path

PAIRS=(("linear","oklab"),("radial","oklab"),("linear","linear-rgb"),("radial","linear-rgb"))

def rows(path: Path):
    with path.open(newline="") as handle: return list(csv.DictReader(handle,delimiter="\t"))
def write(path: Path, data):
    with path.open("w",newline="") as handle:
        writer=csv.DictWriter(handle,list(data[0]),delimiter="\t",lineterminator="\n");writer.writeheader();writer.writerows(data)
def sha(path: Path): return hashlib.sha256(path.read_bytes()).hexdigest()
def local(tag): return tag.rsplit("}",1)[-1]

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--fixtures",type=Path,required=True)
    parser.add_argument("--fixture-root",type=Path,required=True)
    parser.add_argument("--svg-root",type=Path,required=True)
    parser.add_argument("--budgets",type=Path,required=True)
    parser.add_argument("--graph",type=Path,required=True)
    parser.add_argument("--oracle-script",type=Path,required=True)
    parser.add_argument("--out",type=Path,required=True)
    args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    fixture_rows=rows(args.fixtures); budget_rows=rows(args.budgets); graph=rows(args.graph)
    if len(fixture_rows)!=24 or {(r["kind"],r["space"]) for r in fixture_rows}!=set(PAIRS):
        raise ValueError("P1237 universe must be exactly 24 fixtures and four pairs")
    fixture_ids=[r["id"] for r in fixture_rows]
    if len(set(fixture_ids)) != 24: raise ValueError("duplicate fixture id")
    budget={r["id"]:r for r in budget_rows}
    if not set(fixture_ids) <= set(budget): raise ValueError("budget coverage incomplete")
    graph_ids=[r["fixture_id"] for r in graph]
    if len(graph_ids)!=24 or len(set(graph_ids))!=24 or set(graph_ids)!=set(fixture_ids):
        raise ValueError("graph universe must match the 24 fixtures exactly")
    spec=importlib.util.spec_from_file_location("p1237_oracle",args.oracle_script)
    if spec is None or spec.loader is None: raise ValueError("oracle unavailable")
    oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
    def stops(svg,kind):
        tag=kind+"Gradient";servers=[e for e in ET.parse(svg).getroot().iter() if local(e.tag)==tag and any(local(c.tag)=="stop" for c in e)]
        if len(servers)!=1: return []
        out=[]
        for child in servers[0]:
            if local(child.tag)!="stop": continue
            color=oracle.rgba(child.get("stop-color"));opacity=float(child.get("stop-opacity","1"))
            out.append((oracle.parse_offset(child.get("offset")),color[:3]+(color[3]*opacity,)))
        return out
    oracle.server_stops=stops;definitions={r["id"]:r for r in oracle.FIXES}
    numeric=[];input_rows=[]
    for fixture in fixture_rows:
        fid=fixture["id"];source=args.fixture_root/f"{fid}.typ";svg=args.svg_root/f"{fid}.svg"
        if not source.is_file() or sha(source)!=fixture["fixture_sha256"]: raise ValueError("fixture drift: "+fid)
        if not svg.is_file(): raise ValueError("missing frozen SVG: "+fid)
        values=oracle.numeric(definitions[fid],svg,4096);b=budget[fid]
        limits={"color_max":max(0.001,float(b["V_color_max"]))+0.000001,
                "color_p95":float(b["V_color_p95"])+0.000001,
                "alpha_max":float(b["V_alpha_max"])+0.000001,
                "alpha_p95":float(b["V_alpha_p95"])+0.000001}
        passed=all(float(values[key])<=limit for key,limit in limits.items())
        numeric.append({"pair":f'{fixture["kind"]}/{fixture["space"]}',"fixture_id":fid,"points":values["points"],
            "color_max":values["color_max"],"color_p95":values["color_p95"],"alpha_max":values["alpha_max"],"alpha_p95":values["alpha_p95"],
            "V_color_max_limit":limits["color_max"],"V_color_p95_limit":limits["color_p95"],
            "V_alpha_max_limit":limits["alpha_max"],"V_alpha_p95_limit":limits["alpha_p95"],
            "status":"Preserved" if passed else "Violated"})
        input_rows.append({"fixture_id":fid,"fixture_sha256":sha(source),"svg_sha256":sha(svg),
                           "decision_budget_fields":"V_color_max;V_color_p95;V_alpha_max;V_alpha_p95"})
    pair_rows=[]
    for kind,space in PAIRS:
        key=f"{kind}/{space}";g=[r for r in graph if r["pair"]==key];n=[r for r in numeric if r["pair"]==key]
        if len(g)!=6 or len(n)!=6: raise ValueError("each pair must contain exactly six rows")
        gp=sum(r["status"]=="Preserved" for r in g);np=sum(r["status"]=="Preserved" for r in n)
        witness=next((r["fixture_id"] for r in g if r["status"]!="Preserved"),"") or next((r["fixture_id"] for r in n if r["status"]!="Preserved"),"")
        preserved=gp==6 and np==6
        pair_rows.append({"pair":key,"graph_pass":gp,"graph_total":len(g),"numeric_pass":np,"numeric_total":len(n),
            "classification":"Preserved" if preserved else ("Unknown-native-approximation" if kind=="linear" else "Unknown-fallback"),
            "earliest_witness":witness})
    manifest=[{"input":name,"path":str(path),"sha256":sha(path)} for name,path in (
        ("fixtures_manifest",args.fixtures),("vanilla_budget",args.budgets),("graph",args.graph),
        ("oracle_script",args.oracle_script))]
    write(args.out/"numeric.tsv",numeric);write(args.out/"pairs.tsv",pair_rows);write(args.out/"inputs.tsv",input_rows);write(args.out/"manifest.tsv",manifest)
    summary={"fixtures":len(fixture_rows),"pairs":len(pair_rows),"graph_pass":sum(r["graph_pass"] for r in pair_rows),
             "numeric_pass":sum(r["numeric_pass"] for r in pair_rows),"promotions":sum(r["classification"]=="Preserved" for r in pair_rows),
             "decision_budget_fields":["V_color_max","V_color_p95","V_alpha_max","V_alpha_p95"],
             "mutation_score":None,"attack_regime":"not executed; no mutation claim",
             "scope":"frozen SVG corpus; not current productive binary",
             "isolation":"EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO"}
    (args.out/"summary.json").write_text(json.dumps(summary,sort_keys=True,indent=2)+"\n")

if __name__=="__main__": main()
