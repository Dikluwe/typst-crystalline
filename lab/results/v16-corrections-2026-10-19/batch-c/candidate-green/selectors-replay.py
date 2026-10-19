"""Run only after the coordinator builds the fresh candidate executable."""
import datetime, hashlib, json, pathlib, subprocess, sys
# Explicit caller cwd prevents source attribution to another worktree.
ROOT=pathlib.Path.cwd()
OUT=ROOT/'lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors'
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def command(argv):
    start=utc();p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True)
    return {'utc_start':start,'utc_end':utc(),'argv':list(map(str,argv)),'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
def git(*args): return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
candidate=pathlib.Path(sys.argv[1]).resolve()
oracle=ROOT/'lab/typst-original/target/release/typst'
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
receipt=ROOT/'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'
assert 'bd5ebec285505889e8ff1d4f68804ab147a699f3' in receipt.read_text()
assert 'cac836c07e4dec2a89219370428292393a3216b5' in receipt.read_text()
files=[pathlib.Path(p) for p in git('ls-files','--cached','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','Cargo.toml','Cargo.lock','crystalline.toml','.cargo/config.toml').splitlines() if (ROOT/p).is_file()]
state={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_config_sha256':{str(p):sha(ROOT/p) for p in files},'binaries':{'oracle':{'path':str(oracle),'sha256':sha(oracle),'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe_receipt':str(receipt),'recipe_receipt_sha256':sha(receipt)},'candidate':{'path':str(candidate),'sha256':sha(candidate)}},'cases':[]}
def eval_case(name,expr,expected=None,error=None,features=False):
    row={'name':name,'observable':'language eval value' if error is None else 'language rejection category','expression':expr,'expected':expected,'error_category':error,'runs':{}}
    for role,exe in [('oracle',oracle),('candidate',candidate)]:
        argv=[str(exe),'eval']
        if features:argv+=['--features','bundle,html']
        run=command(argv+[expr]);row['runs'][role]=run
        if error:
            run['pass']=run['exit']!=0 and error in run['stderr']
        else:
            try:value=json.loads(run['stdout'])
            except json.JSONDecodeError:value=None
            run['parsed']=value;run['pass']=run['exit']==0 and value==expected
    row['pass']=all(run['pass'] for run in row['runs'].values());state['cases'].append(row)
for f in ['table','table.cell','heading','figure','page','math.equation','par']:
    for method in ['or','and']:
        eval_case(f'{method}-{f}',f'selector(heading).{method}({f}) == selector(heading).{method}(selector({f}))',True)
for f in ['table','heading','figure','math.equation','par']:
    for method in ['within','before','after']:
        eval_case(f'{method}-{f}',f'selector(heading).{method}({f}) == selector(heading).{method}(selector({f}))',True)
for f,msg in [('table.cell','cell is not locatable'),('page','page is not locatable')]:
    for method in ['within','before','after']:
        for operand in [f,f'selector({f})']:
            eval_case(f'{method}-{operand}-rejected',f'selector(heading).{method}({operand})',error=msg)
eval_case('literal-escaping','selector(heading).or("a.b") == selector(heading).or(regex("a\\\\.b"))',True)
eval_case('label-operand','selector(heading).or(<x>) == selector(heading).or(<x>)',True)
eval_case('page-alias','{let p=page; selector(p)==selector(page)}',True)
eval_case('page-bare-group','selector(page)==page.where()',False)
eval_case('receiver-operand-once','{let n=0; let s={n+=1;selector(heading)}.or({n+=10;table}); n}',11)
for exp,expected in [('selector(format.pdf)==selector(format.svg)',False),('selector(format.svg)==selector(format.png)',False),('{let p=format.pdf;selector(p)==selector(format.pdf)}',True),('selector(format.pdf)==format.pdf.where()',False),('selector(format.html)==selector(format.bundle)',False)]:eval_case('format-identity-'+exp,exp,expected,features=True)
# Actual compilation, with text extraction as the selected language observable.
for name,expected in [('table','MATCH'),('equation','MATCH'),('frac','MATCH'),('root','MATCH'),('op','MATCH'),('limits','MATCH'),('scripts','MATCH'),('binom-frac-negative',None),('par-preservation','x')]:
    source=OUT/(name+'.typ');row={'name':'show-'+name,'observable':'compiled text; not PDF bytes','source':str(source),'source_sha256':sha(source),'expected':expected,'runs':{}}
    for role,exe in [('oracle',oracle),('candidate',candidate)]:
        pdf=OUT/(name+'-replay-'+role+'.pdf');run=command([str(exe),'compile',str(source),str(pdf)])
        if run['exit']==0:
            extract=command(['pdftotext',str(pdf),'-']);run['extraction']=extract
            text=' '.join(extract['stdout'].split());run['observed_text']=text
            run['pass']=extract['exit']==0 and ((text==expected) if expected is not None else ('MATCH' not in text and '3' in text and '2' in text))
        else:run['pass']=False
        row['runs'][role]=run
    row['pass']=all(run['pass'] for run in row['runs'].values());state['cases'].append(row)
state['utc_end']=utc();state['changed_sources']=[str(p) for p in files if sha(ROOT/p)!=state['source_config_sha256'][str(p)]]
state['pass']=not state['changed_sources'] and all(row['pass'] for row in state['cases'])
(OUT/'candidate-replay.json').write_text(json.dumps(state,indent=2)+'\n')
print(json.dumps({'pass':state['pass'],'cases':len(state['cases']),'failures':[r['name'] for r in state['cases'] if not r['pass']],'changed_sources':state['changed_sources']}))
sys.exit(0 if state['pass'] else 1)
