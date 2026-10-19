"""Run only after the coordinator builds and freezes the recorded product binary."""
import argparse, datetime, hashlib, json, pathlib, subprocess

p = argparse.ArgumentParser()
p.add_argument('--product', default='lab/target/product/debug/typst')
p.add_argument('--product-sha256', required=True)
p.add_argument('--product-build-receipt', required=True)
p.add_argument('--output', required=True)
a = p.parse_args()
root = pathlib.Path.cwd()
out = pathlib.Path(a.output)
out.mkdir(parents=True, exist_ok=False)
def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(argv):
    started = now()
    try:
        r = subprocess.run(argv, capture_output=True, text=True, timeout=25)
        return dict(started_utc=started, finished_utc=now(), argv=argv,
                    exit=r.returncode, stdout=r.stdout, stderr=r.stderr)
    except subprocess.TimeoutExpired as e:
        return dict(started_utc=started, finished_utc=now(), argv=argv,
                    timeout=True, stdout=str(e.stdout), stderr=str(e.stderr))
def state():
    return {k: run(cmd)['stdout'] for k, cmd in {
        'head':['git','rev-parse','HEAD'],
        'diffstat':['git','diff','HEAD','--stat'],
        'status':['git','status','--short'],
    }.items()}
oracle = pathlib.Path('lab/typst-original/target/release/typst')
assert sha(oracle) == '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
assert sha(a.product) == a.product_sha256
receipt = pathlib.Path(a.product_build_receipt)
assert receipt.is_file(), 'Require coordinator build receipt before candidate measurement'
paths = ['01_core/src/compiler/eval/expression_dispatch.rs',
         '01_core/src/compiler/eval/markup_stream.rs',
         '01_core/src/compiler/eval/rules/realize.rs',
         '01_core/src/compiler/stdlib/structural/heading.rs']
report = dict(started_utc=now(), initial_state=state(),
    source_sha256={x:sha(x) for x in paths},
    binaries={'product':dict(path=a.product, sha256=sha(a.product),
        build_receipt=str(receipt), build_receipt_sha256=sha(receipt)),
        'oracle':dict(path=str(oracle), sha256=sha(oracle),
        commit='bd5ebec285505889e8ff1d4f68804ab147a699f3',
        tree='cac836c07e4dec2a89219370428292393a3216b5',
        build_receipt='lab/results/construction/diagnostics/vanilla-resync-2026-10-17/oracle-build.json.gz')}, results=[])
# Each diagnostic also requires the unconditional-return warning AND invalid-return
# error, so a bad source setup cannot masquerade as positive/negative acceptance.
cases = {
 'direct': ('state("k",0).update(1)', True),
 'content-block': ('[#state("k",0).update(1)]', True),
 'strong': ('strong(state("k",0).update(1))', True),
 'metadata-content': ('metadata(state("k",0).update(1))', True),
 'metadata-array': ('metadata((state("k",0).update(1),))', True),
 'metadata-dict': ('metadata((value:state("k",0).update(1)))', False),
 'ref': ('ref(<x>,supplement:state("k",0).update(1))', True),
 'ref-none': ('ref(<x>,supplement:none)', False),
 'raw-line': ('raw.line(1,1,"x",state("k",0).update(1))', True),
 'raw-line-context': ('raw.line(1,1,"x",context {panic("EXECUTED");state("k",0).update(1)})', False),
 'heading-constructor': ('heading(supplement:state("k",0).update(1))[A]', True),
 'heading-set': ('set heading(supplement:state("k",0).update(1)); heading[A]', False),
 'heading-numbering-inherited': ('set heading(supplement:state("k",0).update(1)); heading(numbering:"1")[A]', False),
 'heading-explicit-none': ('set heading(supplement:state("k",0).update(1)); heading(numbering:"1",supplement:none)[A]', False),
 'heading-explicit-update': ('set heading(supplement:none); heading(numbering:"1",supplement:state("k",0).update(1))[A]', True),
 'heading-callback': ('heading(supplement:it=>{panic("EXECUTED");state("k",0).update(1)})[A]', False),
 'page-body': ('page(state("k",0).update(1))', True),
 'context': ('context {panic("EXECUTED");state("k",0).update(1)}', False),
}
for field in ['header','footer','supplement','background','foreground']:
    cases['page-'+field]=(f'page({field}:state("k",0).update(1))[A]',False)
debts = {
 'figure-supplement': ('figure(supplement:state("k",0).update(1))[A]', True),
 'equation-supplement': ('math.equation(supplement:state("k",0).update(1))[A]', True),
}
for name, (expr, expected) in {**cases, **debts}.items():
    src = '#{ '+expr+'; return "x" }'
    f = out/(name+'.typ'); f.write_text(src)
    item = dict(name=name, scope='open-producer-dependency' if name in debts else 'local-acceptance',
                source=src, source_sha256=sha(f), expected_update_hint=expected, runs={})
    for who, binary in [('oracle',oracle),('product',pathlib.Path(a.product))]:
        r = run([str(binary.resolve()),'compile',str(f),str(out/(name+'-'+who+'.pdf'))])
        stderr = r['stderr']
        r['observables']=dict(invalid_return='cannot return outside' in stderr,
            discarded_warning='this return unconditionally discards the content before it' in stderr,
            update_hint='state/counter updates are content that must end up in the document to have an effect' in stderr,
            panic_error='error: panicked with' in stderr)
        item['runs'][who]=r
    report['results'].append(item)
f=out/'display-located.typ'
f.write_text('#heading[*RICH*] <rich>\n#context {let c=query(<rich>).first();metadata((repr([#c]),repr({set text(size:12pt);c}),repr($ #c $)))}')
item=dict(name='display-located',scope='real-query-producer',source=f.read_text(),source_sha256=sha(f),runs={})
for who,binary in [('oracle',oracle),('product',pathlib.Path(a.product))]:
    r=run([str(binary.resolve()),'query',str(f),'metadata','--field','value'])
    try:
        payload=json.loads(r['stdout'])[0]
        r['rich_body_routes']=[dict(route=route,has_heading='heading(' in value,
            has_strong='strong(' in value,has_text='RICH' in value)
            for route,value in zip(['markup','code-tail','math'],payload)]
    except (ValueError,IndexError,TypeError):
        r['rich_body_routes']=None
    item['runs'][who]=r
report['results'].append(item)
report.update(finished_utc=now(), final_state=state(),
    source_unchanged=all(sha(x)==report['source_sha256'][x] for x in paths),
    product_binary_unchanged=sha(a.product)==a.product_sha256,
    oracle_binary_unchanged=sha(oracle)==report['binaries']['oracle']['sha256'])
(out/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
failures=[]
for item in report['results']:
    if item['scope']=='local-acceptance':
        for who,r in item['runs'].items():
            v=r['observables']
            if not (v['invalid_return'] and v['discarded_warning'] and not v['panic_error']
                    and v['update_hint']==item['expected_update_hint']):
                failures.append([item['name'],who,v])
    elif item['scope']=='real-query-producer':
        for who,r in item['runs'].items():
            routes=r['rich_body_routes']
            if r.get('exit')!=0 or not routes or len(routes)!=3 or not all(
                    x['has_heading'] and x['has_strong'] and x['has_text'] for x in routes):
                failures.append([item['name'],who,routes])
print(json.dumps(dict(receipt=str(out/'receipt.json'),local_acceptance_failures=failures,
    source_unchanged=report['source_unchanged'],product_binary_unchanged=report['product_binary_unchanged']),indent=2))
raise SystemExit(bool(failures or not report['source_unchanged'] or not report['product_binary_unchanged']))
