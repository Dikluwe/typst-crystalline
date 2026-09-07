#!/usr/bin/env python3
"""Bounded independent Content data-availability audit; no candidate or implementation."""
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location("p1307_frozen",HERE/"p1307-r4-oracle.py")
base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)

def main():
    assert base.sha(HERE/"p1307-r4-oracle.py")=="ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68"
    assert not (HERE/"p1307-r4-content-observability.json").exists(),"preserve first attempt before revision"
    settings={"default":"", "supplement-Alpha":"#set heading(supplement: [Alpha])\n", "supplement-Beta":"#set heading(supplement: [Beta])\n", "numbering-1":'#set heading(numbering: "1")\n', "numbering-I":'#set heading(numbering: "I")\n'}
    projections={"type":"repr(type(found.first()))", "repr":"repr(found.first())", "fields":"repr(found.first().fields())", "json":"json.encode(found.first())"}
    cases=[]
    for name,setting in settings.items():
        for projection,expr in projections.items():
            document=setting+base.transport(expr)
            cases.append({"id":name+"."+projection,"setting":name,"projection":projection,"expression":expr,"document":document,"route":"compile","source_sha256":base.source_hash(document),"expected_shape":"value","classes":["Content","LocatedContent",name,projection]})
    folder=base.fixtures(cases)
    rows=base.measure(cases,["default"])
    lookup={(r["case"],r["side"]):r for r in rows}
    pairs=[]
    for left,right,classification in (("supplement-Alpha","supplement-Beta","Existing set-rule acceptance/handling debt and downstream projection must be distinguished; no inference that every loss occurs in the serializer."),("numbering-1","numbering-I","Different accepted numbering patterns require distinct queried Content data; identical baseline fields alone does not identify the exact internal loss without source audit.")):
        for projection in projections:
            obs={side:{"left":lookup[left+"."+projection,side]["observable"],"right":lookup[right+"."+projection,side]["observable"],"same_literal":lookup[left+"."+projection,side]["observable"]==lookup[right+"."+projection,side]["observable"]} for side in ("vanilla","baseline")}
            pairs.append({"left":left,"right":right,"projection":projection,"observations":obs,"classification_boundary":classification})
    unknowns=[{"case":r["case"],"side":r["side"],"reason":r["observable"].get("reason")} for r in rows if r["observable"]["kind"]=="Unknown"]
    failed_construction=[{"case":r["case"],"side":r["side"],"observable":r["observable"]} for r in rows if r["case"].endswith(".type") and r["observable"].get("value")!="content"]
    obj={"schema":"p1307-r4-content-data-availability-focal-v1","regime":"executado sem atestacao de isolamento tecnico","candidate_read":False,"provenance":base.provenance(),"script_sha256":base.sha(__file__),"runner_sha256":base.sha(HERE/"p1307-r4-oracle.py"),"baseline_sha256":base.BASELINE_PIN,"binaries":base.binaries(),"scope":"Default-profile focal only; complete public type/repr/fields/json values. Baseline missing json.encode is an observed API absence, not evidence of missing Content data. Source-level causality remains a separate audit.","fixture_directory":folder,"cases":cases,"rows":rows,"pairs":pairs,"unknowns":unknowns,"failed_content_construction":failed_construction,"finished":base.now(),"limitations":["No full matrix or general serialization certificate.","Probe headings have labels; direct fields are observed explicitly, not inferred from encoder failure.","Supplement and numbering are independent settings; known set-rule debt must not be attributed entirely to introspection or repaired by fixture literals."]}
    base.save("p1307-r4-content-observability.json",obj)
    print(json.dumps({"runs":len(rows),"unknowns":unknowns,"failed_content_construction":failed_construction,"observations":[{"case":r["case"],"side":r["side"],"observable":r["observable"]} for r in rows]},ensure_ascii=False))

if __name__=="__main__":main()
