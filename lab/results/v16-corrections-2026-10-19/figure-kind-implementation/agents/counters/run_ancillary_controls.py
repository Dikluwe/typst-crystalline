#!/usr/bin/env python3
"""Frozen real-language Block membership and Ref origin evidence; no source writes."""
import argparse, hashlib, json, re, subprocess
from pathlib import Path
from datetime import datetime, timezone

def utc(): return datetime.now(timezone.utc).isoformat()
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def mismatch(inputs): return {p: {'expected':h,'actual':sha(p) if Path(p).is_file() else None} for p,h in inputs.items() if not Path(p).is_file() or sha(p)!=h}
def execute(argv):
    start=utc();p=subprocess.run(argv,capture_output=True,text=True,timeout=60)
    return {'argv':argv,'utc_start':start,'utc_end':utc(),'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
def diagnostic(stderr):
    at=re.search(r'\.typ:(\d+):(\d+)',stderr)
    return {'errors':re.findall(r'^error: (.*)$',stderr,re.M),'hints':re.findall(r'^\s*= hint: (.*)$',stderr,re.M),'location':list(map(int,at.groups())) if at else None,'warnings':re.findall(r'^warning: (.*)$',stderr,re.M)}
p=argparse.ArgumentParser();p.add_argument('--build-receipt',required=True);p.add_argument('--binary',default='lab/target/product/debug/typst');p.add_argument('--output',required=True);a=p.parse_args();base=Path(__file__).parent
build=json.loads(Path(a.build_receipt).read_text());assert build['exit']==0 and not build['changed_inputs'];assert sha(a.binary)==build['product_sha256'];assert not mismatch(build['inputs'])
controls=[]
for folder,receiptfile in [('ordinary-block-hidden-heading','receipt.json'),('ordinary-block-labelled','receipt.json'),('reference-layout-origin','receipt.json')]:
    receipt=base/folder/receiptfile;r=json.loads(receipt.read_text());cases=r.get('cases',[r])
    for c in cases:
        runs=c['runs'];oracle=next(run for run in runs if run.get('flavor',run.get('name'))=='oracle')
        name=c.get('name','identity-labelled-block');source=base/folder/(name+'.typ')
        if folder=='ordinary-block-labelled':source=base/folder/'identity-labelled-block.typ'
        controls.append({'name':folder+'-'+name,'source':str(source),'source_sha256':c.get('source_sha256',r.get('source_sha256')),'oracle_receipt':str(receipt),'oracle_receipt_sha256':sha(receipt),'expected':oracle,'known_open':folder=='ordinary-block-labelled'})
corpus={c[k]:c[k+'_sha256'] for c in controls for k in ['source','oracle_receipt']};assert not mismatch(corpus)
out=Path(a.output);out.mkdir(parents=True,exist_ok=False)
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'state':'uncommitted working tree; full successful build inputs/binary and fixed oracle corpus verified before/after; evidence-only','diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'build_receipt':a.build_receipt,'build_receipt_sha256':sha(a.build_receipt),'build_input_hashes':build['inputs'],'binary':a.binary,'binary_sha256':sha(a.binary),'runner_sha256':sha(__file__),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','source_and_oracle_receipt_hashes':corpus,'cases':[]}
for c in controls:
    source=Path(c['source']).resolve();pdf=(out/(c['name']+'.pdf')).resolve();run=execute([a.binary,'compile',str(source),str(pdf)]);expected=c['expected'];result={**c,'compile':run,'checks':{'exit':run['returncode']==expected['returncode']}}
    if expected['returncode']:
        result['actual_diagnostic']=diagnostic(run['stderr']);result['expected_diagnostic']=diagnostic(expected['stderr']);result['checks']['diagnostic']=result['actual_diagnostic']==result['expected_diagnostic']
    else:
        text=execute(['pdftotext','-layout',str(pdf),'-']) if run['returncode']==0 else None;normalized=' '.join(text['stdout'].split()) if text else None;result['normalized_text']=normalized;result['text_extraction']=text
        oracle_pdf=Path(expected['argv'][-1]);oracle_text=execute(['pdftotext','-layout',str(oracle_pdf),'-']);expected_text=' '.join(oracle_text['stdout'].split());result['oracle_pdf_sha256']=sha(oracle_pdf);result['oracle_text_extraction']=oracle_text;result['expected_normalized_text']=expected_text
        result['checks']['render']=oracle_text['returncode']==0 and normalized==expected_text
        query=execute([a.binary,'query',str(source),'<probe>','--field','value']);result['query']=query;result['checks']['language_query']=query['returncode']==expected['query']['returncode'] and query['stdout']==expected['query']['stdout'];result['checks']['compile_diagnostics']=run['stderr']==expected['stderr'] and run['stdout']==expected['stdout']
        if c['known_open']:
            values=json.loads(query['stdout']);result['partial_checks']={'actual_child_count_once':values[0][0]==1,'Block_label_membership':values[0][1]==1};result['known_open_reason']='Ordinary nonlocatable Block-label query membership remains absent; child traversal correction must not be mistaken for full label parity.'
    result['passed']=all(result['checks'].values());r['cases'].append(result)
r.update(utc_end=utc(),changed_build_inputs_after=mismatch(build['inputs']),changed_corpus_after=mismatch(corpus),binary_unchanged=sha(a.binary)==build['product_sha256']);r['passed']=all(c['passed'] for c in r['cases']) and not r['changed_build_inputs_after'] and not r['changed_corpus_after'] and r['binary_unchanged'];(out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({'passed':r['passed'],'cases':[{k:c.get(k) for k in ['name','checks','partial_checks','passed','actual_diagnostic']} for c in r['cases']],'receipt':str(out/'receipt.json')}));raise SystemExit(0 if r['passed'] else 1)
