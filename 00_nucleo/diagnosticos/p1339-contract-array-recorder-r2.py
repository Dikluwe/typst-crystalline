#!/usr/bin/env python3
"""Independent prospective Array-only reference/baseline recorder; no product reads."""
import argparse, base64, datetime, hashlib, json, os, pathlib, subprocess
ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
D = ROOT / '00_nucleo/diagnosticos'
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def lossless(value):
    if value is None: return ""
    if isinstance(value, str): return value
    try: return value.decode("utf-8")
    except UnicodeDecodeError: return {"encoding":"base64", "data":base64.b64encode(value).decode("ascii")}
def run(argv):
    start = now()
    try:
        p = subprocess.run(argv, cwd=ROOT, input='', text=True, capture_output=True, timeout=30, env={**os.environ, 'PYTHONDONTWRITEBYTECODE':'1', 'NO_COLOR':'1'})
    except subprocess.TimeoutExpired as e:
        return dict(argv=argv, stdin='', stdout=lossless(e.stdout), stderr=lossless(e.stderr), exit=None, timeout_seconds=30, utc_start=start, utc_end=now())
    return dict(argv=argv, stdin='', stdout=p.stdout, stderr=p.stderr, exit=p.returncode, utc_start=start, utc_end=now())
def save(name, value):
    path = D / name
    assert name.startswith('p1339-contract-array-') and not path.exists(), path
    payload = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+'+line+'\n' for line in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
    print(json.dumps({'path':str(path), 'sha256':sha(path)}))
def main():
    parser=argparse.ArgumentParser(); parser.add_argument('stage', choices=['focal','expanded']); args=parser.parse_args()
    inputs=D/'p1339-contract-array-inputs-r2.json'; supplement=D/'p1339-contract-array-supplement-r2.json'
    assert sha(inputs)=='8a0c77118c80d220b58dd8070fe0da2399353c4ae0f46610b245224b389bb3a0'
    assert sha(supplement)=='3f6050dbd7b3373d7972f58e1343860697d2af7c672845cc06033cd5383767fc'
    integrity_before={str(p):sha(p) for p in [inputs,supplement,pathlib.Path(__file__)]}
    data=json.loads(inputs.read_text())
    bins={'vanilla':('/usr/local/bin/typst','7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'), 'baseline':('/tmp/p1338-target.vlNAmp/release/typst','f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1')}
    for path,pin in bins.values(): assert sha(path)==pin
    profiles={'default':[], 'html':['--features','html'], 'a11y':['--features','a11y-extras'], 'html+a11y':['--features','html,a11y-extras']}
    if args.stage=='focal': profiles={'default':[]}
    record={'schema':'p1339-array-independent-raw-v1','stage':args.stage,'utc_start':now(),'author':'/root/p1312_tests','authority':'combined contract/oracle, not implementer/adversary/verifier; inherited historical context, no technical isolation attestation','inputs_sha256':sha(inputs),'recorder_sha256':sha(__file__),'bins':bins,'integrity_before':integrity_before,'effective_relevant_environment':{k:v for k,v in {**os.environ,'PYTHONDONTWRITEBYTECODE':'1','NO_COLOR':'1'}.items() if k in ['PATH','LANG','LC_ALL','LC_CTYPE','TZ','HOME','NO_COLOR','PYTHONDONTWRITEBYTECODE','SOURCE_DATE_EPOCH'] or k.startswith(('TYPST_','FONTCONFIG_','XDG_'))},'environment_overrides':{'PYTHONDONTWRITEBYTECODE':'1','NO_COLOR':'1'},'git_before':{x:run(cmd) for x,cmd in {'head':['git','rev-parse','HEAD'],'status':['git','status','--short'],'diffstat':['git','diff','HEAD','--stat']}.items()},'runs':[]}
    for label,(binary,pin) in bins.items():
      for order in (['normal'] if args.stage=='focal' else ['normal','repeat','reverse']):
       cases=list(reversed(data['cases'])) if order=='reverse' else data['cases']
       for profile,flags in profiles.items():
        for case in cases:
         r=run([binary,'eval',*flags,case['source']]); r.update(id=case['id'], source=case['source'], source_sha256=hashlib.sha256(case['source'].encode()).hexdigest(), binary=label, profile=profile, order=order, policy=case['policy']); record['runs'].append(r)
    record['integrity_after']={p:sha(p) for p in integrity_before}; assert record['integrity_after']==integrity_before
    record['binary_integrity_after']={label:sha(path) for label,(path,pin) in bins.items()}; assert all(record['binary_integrity_after'][label]==pin for label,(path,pin) in bins.items())
    record['utc_end']=now(); record['git_after']={x:run(cmd) for x,cmd in {'head':['git','rev-parse','HEAD'],'status':['git','status','--short'],'diffstat':['git','diff','HEAD','--stat']}.items()}
    save('p1339-contract-array-'+args.stage+'-raw-r1.json',record)
    print(json.dumps([{'id':r['id'],'binary':r['binary'],'profile':r['profile'],'exit':r['exit'],'stdout':r['stdout'][:100],'primary':r['stderr'].splitlines()[0] if r['stderr'] else ''} for r in record['runs'] if r['profile']=='default' and r['order']=='normal'],ensure_ascii=False))
if __name__=='__main__': main()
