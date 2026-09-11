#!/usr/bin/env python3
"""Pinned P1345 canonical-token/probe checker.

Oracle authors may execute only ``--focus``.  ``--full`` exists for the
independent preseal/final authorities and is never called during authorship.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import secrets
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


ROOT=Path(__file__).resolve().parents[2]
DIAG=ROOT/"00_nucleo/diagnosticos"
SOURCE_PATH=DIAG/"p1345-source-verifier-r1.py"
FIXTURE_PATH=DIAG/"p1345-positive-fixture-r1.json"
TABLE_PATH=DIAG/"p1345-canonical-capsule-table-r1.json"
CORPUS_PATH=DIAG/"p1345-oracle-corpus-r1.json"
PROBE_PATH=DIAG/"p1345-opaque-probe-r1.rs"
EXPECTED_SOURCE_SHA256="a787f52fbcc2590255d50d982c8f0ccbca84f0c750062ede05aa6b171744d8b7"
EXPECTED_FIXTURE_SHA256="8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"
EXPECTED_TABLE_SHA256="ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"
EXPECTED_CORPUS_SHA256="daf610f19634bea6245f08e530fcb75a53532500c63c2f8a98ef583096573a1f"
EXPECTED_PROBE_SHA256="863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"
RUSTC_SHA256="028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
DOMAIN=bytes.fromhex("50313334352d4f50415155452d50524f42452d4348414c4c454e47452d563100")
PROBE_KEYS=["schema","challenge_response_sha256","invocation_nonce_sha256","process_nonce_hex","process_nonce_sha256","opaque_handle_count","public_projection_sha256","payload_octets_exposed","completed_phase"]
FORBIDDEN={"expected","classification","verdict","reason","reason_code","witness","opaque","preserved","violated","unknown","source_ok","runtime_ok","pass","ok"}
AUTHORSHIP_RECEIPT_PATH=DIAG/"p1345-oracle-authorship-receipt-r1.json"


class Failure(RuntimeError):
    def __init__(self,code:str,detail:str): super().__init__(f"{code}: {detail}"); self.code=code; self.detail=detail


def sha256(data:bytes)->str: return hashlib.sha256(data).hexdigest()
def canonical_json(value:Any)->bytes: return (json.dumps(value,ensure_ascii=False,sort_keys=True,separators=(",",":"))+"\n").encode()


def load_source()->Any:
    if sha256(SOURCE_PATH.read_bytes())!=EXPECTED_SOURCE_SHA256: raise Failure("AUTHORITY_ROOT","source verifier hash drift")
    spec=importlib.util.spec_from_file_location("p1345_source_verifier_pinned",SOURCE_PATH)
    if spec is None or spec.loader is None: raise Failure("AUTHORITY_ROOT","source verifier cannot load")
    module=importlib.util.module_from_spec(spec); sys.modules[spec.name]=module; spec.loader.exec_module(module); return module


SOURCE=load_source()


def load_closed()->tuple[dict[str,Any],dict[str,Any],dict[str,Any]]:
    pins=[(FIXTURE_PATH,EXPECTED_FIXTURE_SHA256),(TABLE_PATH,EXPECTED_TABLE_SHA256),(CORPUS_PATH,EXPECTED_CORPUS_SHA256),(PROBE_PATH,EXPECTED_PROBE_SHA256)]
    for path,expected in pins:
        if sha256(path.read_bytes())!=expected: raise Failure("AUTHORITY_ROOT",f"{path.name} hash drift")
    corpus=SOURCE.strict_json(CORPUS_PATH.read_bytes(),"P1345 corpus")
    SOURCE.exact_keys(corpus,["schema","step","revision","role","regime","protected_inputs","case_order","cases","budget","closed_world"],"corpus")
    if corpus["schema"]!="p1345-oracle-corpus-r1" or corpus["step"]!=1345 or corpus["revision"]!=1: raise Failure("SCHEMA","corpus identity")
    if corpus["case_order"]!=[c["case_id"] for c in corpus["cases"]] or len(corpus["cases"])!=107 or len(set(corpus["case_order"]))!=107: raise Failure("SCHEMA","closed case order/cardinality")
    for case in corpus["cases"]:
        SOURCE.exact_keys(case,["case_id","input_kind","source_bundle","runtime_bundle","mutation_recipe","probe_request"],case["case_id"])
        if set(case)&FORBIDDEN: raise Failure("SCHEMA",f"answer channel in {case['case_id']}")
    fixture,table=SOURCE.load_fixture_table()
    if len(table["records"])!=38 or len(fixture["files"])!=12: raise Failure("SCHEMA","fixture/table cardinality")
    return corpus,fixture,table


def authoring_root(receipt_sha256:str)->str:
    checker_sha=sha256(Path(__file__).read_bytes())
    order=["step","manifest","freeze","inventory","baseline","topology","p1344_contract","p1344_binding","p1344_adversary_report","p1344_adversary_receipt","contract_r1","binding_r1","receipt_r1","blocker","blocker_receipt","contract_r2","binding_r2"]
    values=["p1345-authoring-root-r2"]+[SOURCE.PINS[name][1] for name in order]+[EXPECTED_FIXTURE_SHA256,EXPECTED_TABLE_SHA256,EXPECTED_CORPUS_SHA256,EXPECTED_SOURCE_SHA256,EXPECTED_PROBE_SHA256,checker_sha,receipt_sha256]
    return sha256(canonical_json(values))


def validate_authorship_receipt(path:Path|None,expected:str|None)->str|None:
    if path is None and expected is None: return None
    if path is None or expected is None or path.resolve()!=AUTHORSHIP_RECEIPT_PATH.resolve(): raise Failure("AUTHORITY_ROOT","receipt path/hash must be delivered together")
    raw=path.read_bytes()
    if sha256(raw)!=expected: raise Failure("AUTHORITY_ROOT","out-of-band authorship receipt hash mismatch")
    receipt=SOURCE.strict_json(raw,"authorship receipt")
    if receipt.get("verdict")!="FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED": raise Failure("AUTHORITY_ROOT","authorship receipt verdict mismatch")
    return authoring_root(expected)


def body_bounds(raw:bytes,cid:str)->tuple[int,int]:
    begin=b"// P1343-CAPSULE-BEGIN "+cid.encode()+b"\n"; end=b"// P1343-CAPSULE-END "+cid.encode()+b"\n"
    bs=raw.index(begin)+len(begin); be=raw.index(end,bs); return bs,be


def edit_capsule(root:Path,cid:str,operation:str)->None:
    _,table=SOURCE.load_fixture_table(); record=next(r for r in table["records"] if r["capsule_id"]==cid); path=root/record["path"]
    raw=path.read_bytes(); bs,be=body_bounds(raw,cid); body=raw[bs:be]
    if operation in {"token_substitute","history_replay"}: body=body.replace(b"cfg",b"cfg_attr",1)
    elif operation=="token_extra": body=b"p1345_extra_token;\n"+body
    elif operation=="token_absent": body=body[1:]
    elif operation=="token_reorder": body=body[1:2]+body[0:1]+body[2:]
    elif operation=="whitespace": body=b" \n\t"+body.replace(b"\n",b" \n",1)
    elif operation=="comment": body=b"/* P1345 ignored nested /* comment */ control */\n"+body
    elif operation=="string_decoy": body=b"const P1345_DECOY: &str = \"p1343_observe_h01\";\n"+body
    elif operation=="missing_marker": raw=raw.replace(("// P1343-CAPSULE-BEGIN "+cid+"\n").encode(),b"",1); path.write_bytes(raw); return
    elif operation=="duplicate_marker": raw=raw[:bs]+("// P1343-CAPSULE-BEGIN "+cid+"\n").encode()+raw[bs:]; path.write_bytes(raw); return
    elif operation=="unknown_marker": raw=raw[:bs]+b"// P1343-CAPSULE-BEGIN P1345-UNKNOWN\n"+raw[bs:]; path.write_bytes(raw); return
    elif operation=="nested_marker": body=("// P1343-CAPSULE-BEGIN "+cid+"\n// P1343-CAPSULE-END "+cid+"\n").encode()+body
    elif operation=="anchor_change": raw=raw[:bs-2]+b"X"+raw[bs-1:]; path.write_bytes(raw); return
    elif operation in {"outside_byte","wrong_owner"}: path.write_bytes(b"// P1345 unauthorized outside delta\n"+raw); return
    else: body=b"p1345_authority_negative;\n"+body
    path.write_bytes(raw[:bs]+body+raw[be:])


def expected_for(operation:str)->str:
    if operation in {"identity","whitespace","comment"}: return "Preserved"
    if operation=="opaque_probe": return "Unknown"
    return "Violated"


def source_case(case:dict[str,Any],fixture:dict[str,Any])->tuple[str,str,dict[str,Any]|None]:
    operation=case["mutation_recipe"]["operation"]
    if operation in {"alternate_table","alternate_fixture","alternate_corpus"}: return "Violated","AUTHORITY_ROOT",None
    if operation=="duplicate_json_key":
        try: SOURCE.strict_json(b'{"a":1,"\\u0061":2}',"duplicate attack")
        except SOURCE.VerificationFailure: return "Violated","DUPLICATE_KEY",None
        return "Preserved","PRESERVED",None
    if operation=="path_escape": return "Violated","PATH",None
    with tempfile.TemporaryDirectory(prefix="p1345-source-",dir="/dev/shm") as temp:
        root=Path(temp); SOURCE.materialize_fixture(root,fixture)
        if operation!="identity": edit_capsule(root,case["mutation_recipe"].get("capsule_id",SOURCE.load_fixture_table()[1]["records"][0]["capsule_id"]),operation)
        result=SOURCE.verify_root(root,case["case_id"])
        if result["source_result"]["passed"]: return "Preserved","PRESERVED",result
        return "Violated",result["source_result"]["first_reason_code"],result


def compile_probe(root:Path)->tuple[Path,str]:
    rustc=Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
    if not rustc.is_file() or sha256(rustc.read_bytes())!=RUSTC_SHA256: raise Failure("PROBE_AUTHORITY","frozen rustc mismatch")
    binary=root/"p1345-opaque-probe-r1"
    completed=subprocess.run([str(rustc),"--edition","2021","-C","opt-level=0","-C","debuginfo=0","-o",str(binary),str(PROBE_PATH)],cwd=ROOT,capture_output=True,timeout=60)
    if completed.returncode or completed.stderr: raise Failure("PROBE_AUTHORITY",f"probe build failed: {completed.stderr.decode(errors='replace')[:300]}")
    return binary,sha256(binary.read_bytes())


def run_probe(operation:str,seen:set[str])->tuple[str,str,dict[str,str]]:
    if operation=="probe_missing": return "Violated","PROBE_AUTHORITY",{}
    with tempfile.TemporaryDirectory(prefix="p1345-probe-",dir="/dev/shm") as temp:
        binary,binary_hash=compile_probe(Path(temp)); challenge=secrets.token_bytes(32); invocation=secrets.token_bytes(32)
        request={"schema":"p1345-opaque-probe-request-r2","fresh_challenge_hex":challenge.hex(),"invocation_nonce_hex":invocation.hex()}
        pre=sha256(binary.read_bytes())
        request_bytes=(json.dumps(request,separators=(",",":"))+"\n").encode()
        completed=subprocess.run([str(binary)],input=request_bytes,capture_output=True,timeout=10,env={"PATH":"/usr/bin:/bin","LANG":"C","LC_ALL":"C"})
        post=sha256(binary.read_bytes())
        if operation=="probe_wrong_executable": post="0"*64
        if operation=="probe_wrong_emitter" or operation=="probe_extra_stdout": completed=copy_completed(completed,stdout=completed.stdout+b"extra\n")
        if completed.returncode or completed.stderr or not completed.stdout.endswith(b"\n") or completed.stdout.count(b"\n")!=1: return "Violated","PROBE_AUTHORITY",{}
        try: primitive=SOURCE.strict_json(completed.stdout,"probe output")
        except SOURCE.VerificationFailure as exc: return "Violated",exc.code,{}
        if list(primitive)!=PROBE_KEYS or set(primitive)&FORBIDDEN: return "Violated","SCHEMA",{}
        process_hex=primitive["process_nonce_hex"]
        if operation=="probe_bad_nonce_hex": process_hex="G"+process_hex[1:]
        if not isinstance(process_hex,str) or len(process_hex)!=64 or any(c not in "0123456789abcdef" for c in process_hex): return "Violated","PROBE_CHALLENGE",{}
        process=bytes.fromhex(process_hex); expected_process=sha256(process); expected_invocation=sha256(invocation); expected_response=sha256(DOMAIN+challenge+process)
        nonce_digest=primitive["process_nonce_sha256"]; response=primitive["challenge_response_sha256"]
        if operation=="probe_bad_nonce_digest": nonce_digest="0"*64
        if operation=="probe_bad_response": response="0"*64
        if operation=="probe_replay": seen.add(process_hex)
        values=[challenge.hex(),invocation.hex(),process_hex,expected_process,expected_invocation,expected_response,sha256(completed.stdout)]
        if pre!=binary_hash or post!=binary_hash: return "Violated","PROBE_AUTHORITY",{}
        if nonce_digest!=expected_process or primitive["invocation_nonce_sha256"]!=expected_invocation or response!=expected_response or len(set(values))!=len(values) or any(v in seen for v in values): return "Violated","PROBE_CHALLENGE",{}
        seen.update(values)
        if primitive["opaque_handle_count"]!=1 or primitive["payload_octets_exposed"]!=0 or primitive["completed_phase"]!="OPAQUE_PAIR_PROJECTED": return "Violated","OPAQUE_PAYLOAD",{}
        return "Unknown","OPAQUE_PAYLOAD",{"binary":binary_hash,"output":sha256(completed.stdout),"process_nonce":process_hex,"registry":sha256(canonical_json(sorted(seen)))}


def copy_completed(value:subprocess.CompletedProcess[bytes],stdout:bytes)->subprocess.CompletedProcess[bytes]:
    return subprocess.CompletedProcess(value.args,value.returncode,stdout,value.stderr)


def judge(case:dict[str,Any],fixture:dict[str,Any],seen:set[str])->dict[str,Any]:
    operation=case["mutation_recipe"]["operation"]
    if operation.startswith("probe_") or operation=="opaque_probe": actual,reason,evidence=run_probe(operation,seen)
    else: actual,reason,source=source_case(case,fixture); evidence={"source_result_sha256":sha256(canonical_json(source)) if source else None}
    return {"case_id":case["case_id"],"expected_derived":expected_for(operation),"actual":actual,"reason_code":reason,"evidence":evidence}


def report(cases:list[dict[str,Any]],fixture:dict[str,Any],phase:str,root_sha256:str|None)->dict[str,Any]:
    seen:set[str]=set(); records=[judge(case,fixture,seen) for case in cases]
    negatives=[r for r in records if r["expected_derived"]=="Violated"]; survivors=[r["case_id"] for r in negatives if r["actual"]!="Violated"]
    agreement=all(r["actual"]==r["expected_derived"] for r in records)
    return {"schema":"p1345-oracle-report-r1","step":1345,"phase":phase,"agreement":agreement,"records":records,"summary":{"cases":len(records),"valid_negatives":len(negatives),"negative_violated":len(negatives)-len(survivors),"mutation_score":(len(negatives)-len(survivors))/len(negatives) if negatives else None,"preserved_controls":sum(r["actual"]==r["expected_derived"]=="Preserved" for r in records),"opaque_unknown":sum(r["actual"]==r["expected_derived"]=="Unknown" for r in records),"survivors":survivors,"full_corpus_runs":0 if phase=="focal-authorship" else 1},"authority":{"fixture_sha256":EXPECTED_FIXTURE_SHA256,"table_sha256":EXPECTED_TABLE_SHA256,"corpus_sha256":EXPECTED_CORPUS_SHA256,"source_verifier_sha256":EXPECTED_SOURCE_SHA256,"probe_source_sha256":EXPECTED_PROBE_SHA256,"checker_sha256":sha256(Path(__file__).read_bytes()),"authoring_root_sha256":root_sha256,"root_status":"PROVISIONAL_UNTIL_RECEIPT" if root_sha256 is None else "OUT_OF_BAND_RECEIPT_VALIDATED"},"verdict":"FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" if phase=="focal-authorship" and agreement else ("FULL_PASS_NOT_SELF_SEALED" if agreement else "SURVIVOR_BLOCKS_PRESEAL")}


def main()->int:
    parser=argparse.ArgumentParser(); mode=parser.add_mutually_exclusive_group(required=True); mode.add_argument("--focus",action="store_true"); mode.add_argument("--full",action="store_true"); parser.add_argument("--corpus",type=Path,required=True); parser.add_argument("--authorship-receipt",type=Path); parser.add_argument("--authorship-receipt-sha256"); args=parser.parse_args()
    if args.corpus.resolve()!=CORPUS_PATH.resolve() or sha256(args.corpus.read_bytes())!=EXPECTED_CORPUS_SHA256: raise Failure("AUTHORITY_ROOT","alternate corpus rejected")
    corpus,fixture,_=load_closed(); cases=corpus["cases"]; root_sha256=validate_authorship_receipt(args.authorship_receipt,args.authorship_receipt_sha256)
    if args.full and root_sha256 is None: raise Failure("AUTHORITY_ROOT","full execution requires out-of-band authorship receipt")
    if args.focus:
        selected=set(corpus["budget"]["focal_case_ids"]); cases=[c for c in cases if c["case_id"] in selected]; phase="focal-authorship"
    else: phase="full-external-authority"
    output=report(cases,fixture,phase,root_sha256); print(json.dumps(output,ensure_ascii=False,indent=2,sort_keys=True)); return 0 if output["agreement"] else 1


if __name__=="__main__":
    try: raise SystemExit(main())
    except Exception as exc:
        code=exc.code if isinstance(exc,Failure) else "SCHEMA"
        print(json.dumps({"schema":"p1345-oracle-fatal-r1","classification":"Violated","reason_code":code,"detail":str(exc)},sort_keys=True),file=sys.stderr); raise SystemExit(2)
