"""P1309 coordinator-owned audit gate, independently attacked by role D.

audit(bundle) accepts a materialized bundle. A focal scope checks the same
subsystem used by full audit, without asserting that other gates passed.
Anchors are pinned predecessor/inventory/build/source records supplied by the
assembler; final verifier independently authenticates these anchors.
"""
import base64
import hashlib
import json
from pathlib import Path

ANCHOR_PATH=Path(__file__).resolve().parent/"p1309-audit-anchors.json"

PROFILES={"default":[],"html":["html"],"a11y":["a11y-extras"],"html+a11y":["html","a11y-extras"]}
MATCH={"MATCH_VALUE","MATCH_DIAGNOSTIC"}
EXCLUDED={"INTENTIONAL_PRODUCT_EXTENSION","EXPECTED_FEATURE_DISABLED","UNRESOLVED","CERTIFICATION_DEBT"}

def digest(raw):return hashlib.sha256(raw).hexdigest()

def decode(observation):
    try:
        out=base64.b64decode(observation["stdout_base64"],validate=True)
        err=base64.b64decode(observation["stderr_base64"],validate=True)
        consistent=(digest(out)==observation["stdout_sha256"] and digest(err)==observation["stderr_sha256"]
          and out.decode("utf-8",errors="replace")==observation["stdout"]
          and err.decode("utf-8",errors="replace")==observation["stderr"])
        valid=observation["complete"] and observation["reason_code"] is None and observation["exit_code"] in (0,1)
        if observation["exit_code"]==0:
            json.loads(out.decode("utf-8"),parse_constant=lambda x:(_ for _ in ()).throw(ValueError(x)))
        elif not err.strip():valid=False
        return out,err,consistent,valid
    except (ValueError,KeyError,TypeError,UnicodeError):return b"",b"",False,False

def runtime(v,c):
    vo,ve,vc,vv=decode(v);co,ce,cc,cv=decode(c)
    if not all((vc,vv,cc,cv)):return "EXECUTION_UNKNOWN"
    if v["exit_code"]==0 and c["exit_code"]==0:
        if vo!=co:return "DIFFERENT_VALUE"
        return "MATCH_VALUE" if ve==ce else "DIFFERENT_DIAGNOSTIC"
    if v["exit_code"]==0:return "VANILLA_ONLY"
    if c["exit_code"]==0:return "CRYSTALLINE_ONLY"
    return "MATCH_DIAGNOSTIC" if (vo,ve,v["exit_code"])==(co,ce,c["exit_code"]) else "DIFFERENT_DIAGNOSTIC"

def audit(bundle):
    errors=[]
    def fail(code,detail):errors.append({"code":code,"detail":detail})
    scope=bundle.get("scope","all")
    anchors=json.loads(ANCHOR_PATH.read_text())
    if bundle["anchors"]!=anchors:
        fail("ANCHORS_CHANGED","Bundle anchors differ from the independently frozen input file")
    def enabled(name):return scope in ("all",name)
    if enabled("catalog"):
        catalog=bundle["catalog"];probes=catalog["probes"];lookup={p["id"]:p for p in probes}
        if len(lookup)!=len(probes):fail("CATALOG_DUPLICATE_ID","IDs are not unique")
        for p in anchors["historical_probes"]:
            if lookup.get(p["id"])!=p:fail("HISTORICAL_PROBE_CHANGED",p["id"])
        paths={p["path"]for p in probes}
        for path in anchors["inventory_paths"]:
            if path not in paths:fail("DISCOVERED_MEMBER_OMITTED",path)
        historical_ids={p["id"]for p in anchors["historical_probes"]}
        allowed=set(anchors["inventory_paths"])|set(anchors.get("explicit_principal_controls",[]))
        for p in probes:
            if p["id"]not in historical_ids and p["path"]not in allowed:fail("UNSUPPORTED_PRINCIPAL_PATH",p["path"])
        if catalog["profiles"]!=PROFILES:fail("PROFILE_SET_INVALID","catalog profiles")
        if bundle.get("principal_probe_count",len(probes))!=len(probes):fail("PRINCIPAL_DENOMINATOR_INFLATED","count differs from catalog")
        if set(bundle.get("supplement_ids",[]))&set(lookup):fail("SUPPLEMENT_IN_PRINCIPAL","overlapping universes")
    if enabled("matrix"):
        bins=anchors["binaries"];expected={(p["id"],profile)for p in bundle["catalog"]["probes"]for profile in PROFILES}
        probes={p["id"]:p for p in anchors["principal_probes"]}
        maps={}
        for phase in ("normal","repeat","reverse"):
            rows=bundle["matrices"][phase];keys=[(r["id"],r["profile"])for r in rows]
            if len(keys)!=len(set(keys))or set(keys)!=expected:fail("MATRIX_KEY_SET_INVALID",phase)
            maps[phase]={}
            for r in rows:
                key=(r["id"],r["profile"])
                if r["expression"]!=probes.get(r["id"],{}).get("expression"):fail("FIXTURE_IDENTITY",[phase,*key,"catalog"])
                for side in ("vanilla","crystalline"):
                    o=r[side];out,err,consistent,valid=decode(o)
                    if not consistent:fail("TRANSCRIPT_INTEGRITY",[phase,*key,side])
                    if not valid:fail("EXECUTION_UNKNOWN",[phase,*key,side])
                    anchor=anchors["raw_channels"].get("|".join((phase,*key,side)))
                    if anchor!=[o["exit_code"],digest(out),digest(err)]:fail("RAW_CHANNEL_EVIDENCE_CHANGED",[phase,*key,side])
                    if (o["side"]!=side or o["binary_sha256"]!=bins[side]["sha256"]or o["binary_path"]!=bins[side]["path"]or o["argv"][0]!=bins[side]["path"]):fail("BINARY_SIDE_IDENTITY",[phase,*key,side])
                    expected_features=PROFILES.get(r["profile"])
                    argv=o["argv"];actual_features=argv[argv.index("--features")+1].split(",")if"--features"in argv else []
                    if o["profile"]!=r["profile"]or o["features"]!=expected_features or actual_features!=expected_features:fail("PROFILE_ARGV_MISMATCH",[phase,*key,side])
                    if o["expression"]!=r["expression"]or digest(r["expression"].encode())!=o["source_sha256"]or len(argv)<3 or argv[2]!=r["expression"]:fail("FIXTURE_IDENTITY",[phase,*key,side])
                actual=runtime(r["vanilla"],r["crystalline"])
                if actual!=r["runtime_class"]:fail("RUNTIME_CLASS_MISMATCH",[phase,*key,actual,r["runtime_class"]])
                maps[phase][key]=(actual,tuple((r[s]["exit_code"],r[s]["stdout_base64"],r[s]["stderr_base64"],r[s]["binary_sha256"],tuple(r[s]["features"]))for s in ("vanilla","crystalline")))
        for phase in ("repeat","reverse"):
            if maps["normal"]!=maps[phase]:fail("ORDER_INSTABILITY",phase)
        build=bundle["build"]
        if build["candidate"]!=bins["crystalline"]or build["candidate"]["path"]!=anchors["fresh_binary_path"]or build["before"]["head"]!=anchors["head"]or build["before"]["diff"]or build["exit"]!=0:fail("FRESH_BUILD_PROVENANCE","fresh clean dedicated build required")
        if bins["crystalline"]["sha256"]in anchors["forbidden_candidate_hashes"]:fail("REUSED_P1308_BINARY",bins["crystalline"]["sha256"])
    if enabled("ledger"):
        lookup={r["path"]:r for r in bundle["ledger"]}
        for path in anchors["historical_ledger_paths"]:
            if path not in lookup:fail("HISTORICAL_LEDGER_PATH_LOST",path)
        for row in bundle["ledger"]:
            path=row["path"];cls=row["current_language_class"]
            if cls in ("INTENTIONAL_PRODUCT_EXTENSION","EXPECTED_FEATURE_DISABLED"):
                proof=anchors["normative_facts"].get(path,{})
                if not proof.get(cls)or not row.get("normative_evidence"):fail("NORMATIVE_AUTHORITY_MISSING",[path,cls])
            if cls=="CERTIFICATION_DEBT"and row.get("universe")!="certification":fail("CERTIFICATION_AS_LANGUAGE_FAILURE",path)
            if row.get("universe")=="principal"and row.get("probe_id")not in anchors["principal_probe_ids"]:fail("LEDGER_PROBE_IDENTITY",path)
            if row.get("universe")=="principal":
                expected_runtime=anchors["runtime_by_probe"].get(row["probe_id"])
                actual_runtime=row.get("runtime_class_by_profile")
                if isinstance(actual_runtime,str):actual_runtime=json.loads(actual_runtime)
                if actual_runtime!=expected_runtime:fail("LEDGER_RUNTIME_CHANGED",path)
            owner=row.get("owner_prompt")
            if owner:
                consumers=bundle["ownership"].get(owner,[])
                if len(consumers)!=1 or row.get("consumer_unique")not in (True,"true",consumers[0]):fail("PROMPT_OWNERSHIP_INVALID",owner)
        if bundle.get("functional_failure_count") is not None:
            measured=sum(r["runtime_class"]not in MATCH for r in bundle["normal_rows"])
            if bundle["functional_failure_count"]!=measured:fail("CERTIFICATION_AS_LANGUAGE_FAILURE","functional count differs from matrix")
        transition_keys={(t["probe_id"],t["profile"])for t in bundle["transitions"]}
        expected_transition_keys={tuple(k.split("|"))for k in anchors["historical_classes"]}
        if not expected_transition_keys<=transition_keys:fail("HISTORICAL_TRANSITION_OMITTED","Missing historical cells")
        for transition in bundle["transitions"]:
            key=(transition["probe_id"],transition["profile"])
            before=anchors["historical_classes"].get("|".join(key))
            if before is None:continue
            if transition["previous_runtime_class"]!=before:fail("HISTORICAL_TRANSITION_CHANGED",list(key))
            if transition["current_runtime_class"]!=anchors["runtime_by_probe"][key[0]][key[1]]:fail("TRANSITION_RUNTIME_CHANGED",list(key))
            if before in MATCH and transition["current_runtime_class"]not in MATCH and transition["transition"]!="NEW_REGRESSION":fail("REGRESSION_HIDDEN",list(key))
    if enabled("sentinels"):
        present={r["id"]:r for r in bundle["sentinels"]}
        for required in anchors["required_sentinel_ids"]:
            if required not in present:fail("SENTINEL_OBLIGATION_MISSING",required)
        for route,ids in anchors["encoder_behaviour_witnesses"].items():
            if route in bundle.get("closed_encoder_paths",[])and not set(ids)<=set(present):fail("ENCODER_CLOSED_BY_PRESENCE_ONLY",route)
        for identifier,row in present.items():
            for side in ("vanilla","crystalline"):
                expected=anchors["sentinel_raw_channels"].get(identifier+"|"+side)
                o=row[side];out,err,consistent,valid=decode(o)
                if not consistent or expected!=[o["exit_code"],digest(out),digest(err)]:fail("SENTINEL_RAW_EVIDENCE_CHANGED",[identifier,side])
                if not valid:fail("SENTINEL_EXECUTION_UNKNOWN",[identifier,side])
        for row in bundle.get("array_integrity",[]):
            n=row["length"]
            if row["values"]!=list(range(n)):fail("REPR_DATA_LOSS",n)
            actual=present.get(row["sentinel_id"])
            if actual:
                for side in ("vanilla","crystalline"):
                    try:value=json.loads(base64.b64decode(actual[side]["stdout_base64"]))
                    except(ValueError,KeyError):value=None
                    if not isinstance(value,list)or value[:2]!=[n,list(range(n))]:fail("REPR_DATA_LOSS",[n,side,"public transcript"])
            else:fail("ARRAY_INTEGRITY_WITNESS_MISSING",row["sentinel_id"])
        if set(anchors["array_lengths"])!={r["length"]for r in bundle.get("array_integrity",[])}:fail("ARRAY_INTEGRITY_WITNESS_MISSING","length coverage")
    if enabled("selection"):
        selection=bundle["selection"];eligible=[]
        for cohort in selection["cohorts"]:
            facts=anchors["cohort_facts"].get(cohort["id"],{})
            classes=set(facts.get("semantic_classes",[]))
            priority=(1 if"NEW_REGRESSION"in classes else 2 if"L0_CONTRADICTION"in classes and facts.get("canonical_route_demonstrated")else
              3 if"DIAGNOSTIC_DIVERGENCE"in classes else 4 if classes&{"WRONG_PUBLIC_VALUE","WRONG_PUBLIC_KIND_OR_IDENTITY","WRONG_PUBLIC_REPR"}else
              5 if"MISSING_LANGUAGE_MEMBER"in classes and facts.get("carriers_existing")else 6 if"MISSING_LANGUAGE_MEMBER"in classes else 7)
            if cohort["priority"]!=priority:fail("COHORT_PRIORITY_INVALID",cohort["id"])
            if set(cohort["owners"])!=set(facts.get("owners",[]))or set(cohort["paths"])!=set(facts.get("paths",[]))or cohort["regression_surface"]["rank"]!=facts.get("regression_surface_rank"):fail("COHORT_EVIDENCE_CHANGED",cohort["id"])
            calculated=[cohort["priority"],len(set(cohort["owners"])), -len(set(cohort["paths"])),cohort["regression_surface"]["rank"],cohort["id"]]
            if cohort["rank_key"]!=calculated:fail("COHORT_RANK_KEY_INVALID",cohort["id"])
            if cohort["eligible"]:
                if any(c in EXCLUDED for c in cohort.get("semantic_classes",[])):fail("INELIGIBLE_COHORT",cohort["id"])
                eligible.append((calculated,cohort["id"]))
        winner=min(eligible)[1]if eligible else None
        if selection.get("blockers"):winner=None
        if selection["selected_cohort"]!=winner:fail("DETERMINISTIC_SELECTION_VIOLATED",winner)
    if enabled("integrity"):
        if bundle["ownership"]!=anchors["ownership"]:fail("OWNERSHIP_EVIDENCE_CHANGED","owners differ from sealed headers")
        all_consumers=[]
        for owner,consumers in bundle["ownership"].items():
            if len(consumers)!=1:fail("PROMPT_OWNERSHIP_INVALID",owner)
            all_consumers.extend(consumers)
        if len(all_consumers)!=len(set(all_consumers)):fail("CONSUMER_MULTIPLE_OWNERS","one consumer attributed to multiple prompts")
        nuclei=bundle["nuclei"]
        if nuclei!=anchors["nuclei"]or bundle["nucleus_pins"]!=anchors["nucleus_pins"]:fail("NUCLEUS_EVIDENCE_CHANGED","source nuclei/pins differ from sealed evidence")
        for pin in bundle["nucleus_pins"]:
            if nuclei.get(pin["path"],{}).get("sha256")!=pin["sha256"]:fail("NUCLEUS_PIN_INVALID",pin["path"])
        visiting=set();visited=set()
        def walk(n):
            if n not in nuclei:fail("NUCLEUS_DEPENDENCY_MISSING",n);return
            if n in visiting:fail("NUCLEUS_DAG_INVALID",n);return
            if n in visited:return
            visiting.add(n)
            for child in nuclei.get(n,{}).get("dependencies",[]):walk(child)
            visiting.remove(n);visited.add(n)
        for n in nuclei:walk(n)
        reachable={p["path"]for p in bundle["nucleus_pins"]}
        for data in nuclei.values():reachable.update(data.get("dependencies",[]))
        for n in set(nuclei)-reachable:fail("NUCLEUS_ORPHAN",n)
        if bundle["head"]!=anchors["head"]or bundle["product_inventory"]!=anchors["product_inventory"]:fail("PRODUCT_OR_L0_CHANGED","baseline inventory mismatch")
        if bundle.get("allowlist_incidents_after_restart"):fail("ALLOWLIST_VIOLATION","successor run invalidated")
    return errors
