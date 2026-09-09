"""Review partial principal ledger and execute frozen R09/R10/R12 on copies."""
import collections
import copy
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(name):
    s=importlib.util.spec_from_file_location(name,D/(name+'.py'));m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
s=load('p1335-review-semantic');a=load('p1335-review-attacks');c=s.core

def run():
    r=c.preflight();cat=c.read('p1335-probe-catalog.json');normal=c.read('p1335-matrix-normal.json')['results'];previous=c.read('p1322-matrix-normal.json')['results']
    ledger=s.table('p1335-classification-principal-owner-ledger.tsv');trans=s.table('p1335-classification-principal-transitions.tsv');summary=c.read('p1335-classification-principal-summary.json')
    c.merge_review(r,s.verify_principal(ledger,cat,normal));c.merge_review(r,s.verify_transitions(trans,normal,previous))
    counts=dict(collections.Counter(x['runtime_class'] for x in normal));closed=sum(x['runtime_class'] in c.MATCH for x in normal)
    p=summary['principal']
    r.require(p['probes']==len(cat['probes']) and p['cells']==len(normal),'PRINCIPAL_DENOMINATOR',p)
    r.require(p['raw_counts']==counts and p['raw_closed_cells']==closed and p['raw_ratio']==closed/len(normal),'PRINCIPAL_COUNTS',counts)
    r.require(p['unknown']==counts.get('EXECUTION_UNKNOWN',0),'PRINCIPAL_UNKNOWN',p['unknown'])
    r.require(summary['adjusted'] is None and summary['scope']=='NORMAL_ONLY_NO_CLOSURE_NO_SELECTION','PARTIAL_SCOPE',summary['scope'])
    attacks=[]
    entry=next(x for x in ledger if x['current_language_class']=='CLOSED_MEASURED_LOOKUP_REPR_ONLY' and x['path']=='json')
    catalog={**cat,'probes':[x for x in cat['probes'] if x['id']==entry['probe_id']]};rows=[x for x in normal if x['id']==entry['probe_id']]
    def judge(data):return s.verify_principal(data,catalog,rows)
    mutant=[copy.deepcopy(entry)];mutant[0]['current_language_class']='CLOSED_MEASURED_FUNCTIONAL_SENTINEL'
    attacks.append(a.attack('R09',[entry],mutant,judge,dict(promote='lookup/type/repr only to functional closure')))
    row=copy.deepcopy(next(x for x in rows if x['profile']=='default'));prior=next(x for x in previous if x['id']==row['id'] and x['profile']==row['profile'])
    row['crystalline']['stdout']='"R10 explicit simulation on data copy"\n'
    import base64
    raw=row['crystalline']['stdout'].encode();row['crystalline']['stdout_base64']=base64.b64encode(raw).decode();row['crystalline']['stdout_sha256']=c.sha(raw)
    row['runtime_class']=c.classify(row['vanilla'],row['crystalline'])
    tr=copy.deepcopy(next(x for x in trans if x['probe_id']==row['id'] and x['profile']==row['profile']))
    tr.update(current_runtime_class=row['runtime_class'],transition='REGRESSION_CANDIDATE',fixture_comparable='true',complete_literal_equal='false')
    def tjudge(data):return s.verify_transitions(data,[row],[prior])
    attacks.append(a.attack('R10',[tr],[],tjudge,dict(scenario='simulated changed current transcript, real historical MATCH',omit='required regression candidate transition',current_copy_sha256=a.fingerprint(row))))
    entry=next(x for x in ledger if x['path']=='calc.deg')
    catalog={**cat,'probes':[x for x in cat['probes'] if x['id']==entry['probe_id']]};rows=[x for x in normal if x['id']==entry['probe_id']]
    mutant=[copy.deepcopy(entry)];mutant[0].update(current_language_class='DOCUMENTED_PRODUCT_EXTENSION',normative_evidence='')
    attacks.append(a.attack('R12',[entry],mutant,judge,dict(promote='pending extension to documented extension, erase L0 foundation')))
    return dict(at=c.utc(),role='D',manifest_sha256=c.digest(D/'p1335-manifest.json'),checker_sha256=c.digest(__file__),semantic_checker_sha256=c.digest(D/'p1335-review-semantic.py'),inputs=c.INPUTS,counts=counts,scope='NORMAL_ONLY ledger, denominator and literal transitions; selection/adjustment pending',attacks=attacks,valid=sum(x['valid'] for x in attacks),rejected=sum(x['valid'] and x['rejected'] for x in attacks),**r.result())

if __name__=='__main__':print(json.dumps(run(),ensure_ascii=False,indent=2))
