"""Bind real frozen P1309 artifacts to the coordinator audit gate."""
import base64
import csv
from collections import defaultdict
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
import tomllib
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
ROOT=D.parents[1]
spec=importlib.util.spec_from_file_location("rec",D/"p1309-record-r2.py");record=importlib.util.module_from_spec(spec);spec.loader.exec_module(record)
def read(n):return json.loads((D/n).read_text())
def rows(n):
    with (D/n).open()as f:return list(csv.DictReader(f,delimiter="\t"))
def pin(n):return {"path":"00_nucleo/diagnosticos/"+n,"sha256":record.sha(D/n)}
def raw(o):
    return [o["exit_code"],hashlib.sha256(base64.b64decode(o["stdout_base64"])).hexdigest(),hashlib.sha256(base64.b64decode(o["stderr_base64"])).hexdigest()]
def normalize(o,side,binary):
    o=dict(o)
    o["exit_code"]=o.get("exit_code",o.get("returncode"))
    for channel in("stdout","stderr"):
        data=o[channel].encode()
        o.setdefault(channel+"_base64",base64.b64encode(data).decode())
        o.setdefault(channel+"_sha256",hashlib.sha256(data).hexdigest())
    o.update(side=side,binary_path=binary["path"],binary_sha256=binary["sha256"])
    o.setdefault("complete",o["exit_code"]in(0,1)and o.get("observable",{}).get("kind")!="unknown")
    o.setdefault("reason_code",None if o["complete"]else"EXECUTION_UNKNOWN")
    return o

def main():
    cat=read("p1309-probe-catalog.json")
    baseline=read("p1309-baseline-r2.json");build=read("p1309-build-r2.json")
    matrices={p:read("p1309-matrix-"+p+".json")["results"]for p in("normal","repeat","reverse")}
    bins={"vanilla":baseline["vanilla"],"crystalline":build["candidate"]}
    ledger=rows("p1309-owner-ledger.tsv");transitions=rows("p1309-transition-ledger.tsv")
    selection=read("p1309-selection.json");sources=read("p1309-classification-sources.json")
    supplement=read("p1309-sentinels.json")
    sentinels=[]
    for row in supplement["rows"]:
        sentinels.append({"id":row["id"]+"|"+row["profile"],"profile":row["profile"],"bilateral_class":row["bilateral_class"],
          **{s:normalize(row[s],s,bins[s])for s in("vanilla","crystalline")}})
    arrays=[{"length":r["length"],"values":r["values"],"sentinel_id":r["id"]+"|"+r["profile"]}for r in supplement["array_integrity"]if r["side"]=="crystalline"]
    ownership=defaultdict(list)
    for p in baseline["product_inventory"]:
        if p.endswith(".rs")and p.startswith(("01_core/","02_shell/","03_infra/","04_wiring/")):
            text=(ROOT/p).read_text()
            match=re.search(r"^\s*//[/!]\s*@prompt\s+(\S+)",text,re.M)
            if match:ownership[match[1]].append(p)
    nuclei={};pins=[]
    for p in baseline["product_inventory"]:
        if p.startswith("00_nucleo/prompts/_nuclei/")and p.endswith(".toml"):
            data=tomllib.loads((ROOT/p).read_text())
            nuclei[p]={"sha256":None,"raw_sha256":record.sha(ROOT/p),"dependencies":[],"id":data["id"]}
    for owner in ownership:
        text=(ROOT/owner).read_text()
        for path,sha in re.findall(r"(00_nucleo/prompts/_nuclei/[^\s`]+\.toml)\s+sha256:([0-9a-f]{64})",text):
            pins.append({"owner":owner,"path":path,"sha256":sha})
            assert path in nuclei
            if nuclei[path]["sha256"]is not None:assert nuclei[path]["sha256"]==sha
            nuclei[path]["sha256"]=sha
    # Existing nuclei have no dependencies; reject an unhandled future format
    # rather than silently claiming to have audited a graph we did not parse.
    for p in nuclei:
        data=tomllib.loads((ROOT/p).read_text())
        assert set(data)<={"tekt","kind","id","title","claims"},("Unhandled nucleus format",p,set(data))
    normative={}
    for p,evidence in sources["extension_normative_evidence"].items():
        normative[p]={"INTENTIONAL_PRODUCT_EXTENSION":p not in("calc.deg","calc.rad","calc.log10"),"evidence":evidence}
    for row in ledger:
        if row["current_language_class"]=="EXPECTED_FEATURE_DISABLED":
            normative.setdefault(row["path"],{})["EXPECTED_FEATURE_DISABLED"]=bool(row.get("normative_evidence"))
    by_cohort=defaultdict(list)
    for row in ledger:
        if row.get("cohort_id"):by_cohort[row["cohort_id"]].append(row)
    facts={}
    for c in selection["cohorts"]:
        semantic=set(c.get("semantic_classes",[]))or{r["current_language_class"]for r in by_cohort[c["id"]]}
        facts[c["id"]]={"semantic_classes":sorted(semantic),"canonical_route_demonstrated":c.get("canonical_route_demonstrated",c["priority"]==2),
          "carriers_existing":c["carriers_existing"],"owners":c["owners"],"paths":c["paths"],"regression_surface_rank":c["regression_surface"]["rank"]}
    inventory_paths=set()
    # The inventory author's sealed catalog records exact structural origins;
    # derive union from the four raw R2 inventory files, not the probe list.
    for side in("vanilla","crystalline"):
        for profile in("default","html"):
            inv=read(f"p1309-inventory-{side}-{profile}-r2.json")
            entries=inv["entries"]
            if isinstance(entries,dict):inventory_paths.update(entries)
            else:
                for e in entries:inventory_paths.add(e.get("display_path",e.get("path")))
    assert None not in inventory_paths
    encoder_witnesses={}
    for route in("cbor.encode","json.encode","toml.encode","yaml.encode"):
        prefix="p1307."+route.split(".")[0]+"."
        encoder_witnesses[route]=[r["id"]for r in sentinels if r["id"].startswith(prefix)]
    historical=read("p1304-feature-matrix.json")["results"]
    anchors={"schema":"p1309-audit-anchors-v1","manifest":pin("p1309-manifest-r2.json"),"head":record.HEAD,
      "product_inventory":baseline["product_inventory"],"historical_probes":read("p1304-probe-catalog.json")["probes"],
      "inventory_paths":sorted(inventory_paths),"explicit_principal_controls":["csv.encode","xml.encode","read.encode"],
      "binaries":bins,"fresh_binary_path":record.TARGET+"/release/typst","forbidden_candidate_hashes":["09725fff8b4c472ee6b50a9ca6105d90268b2a0da8a22f1d8abd0f45e20c4c50"],
      "principal_probes":cat["probes"],"principal_probe_ids":[p["id"]for p in cat["probes"]],
      "raw_channels":{"|".join((p,r["id"],r["profile"],s)):raw(r[s])for p,rs in matrices.items()for r in rs for s in("vanilla","crystalline")},
      "historical_ledger_paths":[r["path"]for r in rows("p1304-owner-ledger.tsv")],"normative_facts":normative,
      "runtime_by_probe":{p["id"]:{r["profile"]:r["runtime_class"]for r in matrices["normal"]if r["id"]==p["id"]}for p in cat["probes"]},
      "historical_classes":{r["id"]+"|"+r["profile"]:r["runtime_class"]for r in historical},
      "required_sentinel_ids":[r["id"]for r in sentinels],"encoder_behaviour_witnesses":encoder_witnesses,
      "sentinel_raw_channels":{r["id"]+"|"+s:raw(r[s])for r in sentinels for s in("vanilla","crystalline")},
      "array_lengths":[39,40,41,42,81,256],"cohort_facts":facts,"ownership":dict(ownership),"nuclei":nuclei,"nucleus_pins":pins,
      "source_artifacts":[pin(n)for n in("p1309-probe-catalog.json","p1309-inventory-author-receipt-r2.json","p1309-owner-ledger.tsv","p1309-transition-ledger.tsv","p1309-selection.json","p1309-classification-sources.json","p1309-sentinels.json","p1309-stability.json","p1309-lint-r2.json")]}
    bundle={"schema":"p1309-audit-bundle-v1","scope":"all","anchors":anchors,"catalog":cat,"principal_probe_count":len(cat["probes"]),
      "supplement_ids":[r["id"]for r in sentinels],"matrices":matrices,"build":build,"ledger":ledger,"transitions":transitions,"normal_rows":matrices["normal"],
      "functional_failure_count":sum(r["runtime_class"]not in("MATCH_VALUE","MATCH_DIAGNOSTIC")for r in matrices["normal"]),
      "sentinels":sentinels,"array_integrity":arrays,"closed_encoder_paths":["json.encode","toml.encode","yaml.encode"],
      "selection":selection,"ownership":dict(ownership),"nuclei":nuclei,"nucleus_pins":pins,"head":record.HEAD,"product_inventory":record.inventory(),"allowlist_incidents_after_restart":[]}
    record.save("audit-anchors",anchors)
    record.save("audit-bundle",bundle)

if __name__=="__main__":main()
