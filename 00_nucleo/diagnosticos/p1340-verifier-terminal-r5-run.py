"""Independent R5 input preservation/discrimination and three vanilla runs only."""
import base64, datetime, hashlib, importlib.util, json, os, re, subprocess, tempfile, time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline'); D=ROOT/'00_nucleo/diagnosticos'
PINS={
'p1340-terminal-source-authority-r5.json':'32269929fe08b2c887226e1a987a51b3d83e63aa40c2736086272dec668a96f7',
'p1340-terminal-contract-r5.json':'293dca9cdac3f96cf2110fb37f4d77ab4773cd1478dcf8089fa07f6a722091d0',
'p1340-terminal-contract-r5.py':'83093348fe7eb4c7aec9127682239702224e654d37784d7a8bbd4829e48b6de1',
'p1340-contract-terminal-harness-r5.rs':'cff1314502b1b113f86cd335c423a65b6772eedb3cdabe45bc21a4eff676a6a6',
'p1340-contract-terminal-harness-r2.rs':'85c3f151cbf1e88c241acea0b68df1f6c468ba0a1a956e5a3f6af91cd0a19e7d',
'p1340-contract-binding-r2.md':'4d9426025cbc986b79793658e99a20203216089eb9951d9ece759cb962027f8c'}
BIN=Path('/usr/local/bin/typst'); BH='7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def pincheck():
    assert all(sha(D/n)==h for n,h in PINS.items());assert sha(BIN)==BH
def save(path,body):
    assert path.parent==D and path.name.startswith('p1340-verifier-terminal-r5-') and not path.exists()
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
def state():
    def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
    return dict(utc=utc(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat'))
pincheck();start=utc();tick=time.monotonic();before=state()
s=importlib.util.spec_from_file_location('r5_preservation',D/'p1340-terminal-contract-r5.py');checker=importlib.util.module_from_spec(s);s.loader.exec_module(checker)
harness=D/'p1340-contract-terminal-harness-r5.rs';base=harness.read_bytes();accepted=checker.check(harness)
contract=json.loads((D/'p1340-terminal-contract-r5.json').read_text())
literal_rows=re.findall(rb'Case::(\w+) => ("(?:\\.|[^"\\])*"),',base)
sources={name.decode():json.loads(literal.decode()) for name,literal in literal_rows};assert len(sources)==6
for row in contract['sources']:assert sources[row['case']]==row['successor_source']
class MutantInput:
    def __init__(self,name,data):self.name=name;self.data=data
    def read_bytes(self):return self.data
    def __str__(self):return 'in-memory-harness-mutation:'+self.name
mutations=[]
def mutate(name,old,new):
    assert base.count(old.encode())>=1,name
    b=base.replace(old.encode(),new.encode(),1);assert b!=base
    mutations.append((name,old,new,b))
mutate('skip-counter-read','let _ = c.final(); let _ = query(<opaque>);','let _ = query(<opaque>);')
mutate('skip-query-read','let _ = c.final(); let _ = query(<opaque>);','let _ = c.final();')
mutate('reorder-reads','let _ = c.final(); let _ = query(<opaque>);','let _ = query(<opaque>); let _ = c.final();')
mutate('replace-query-label','let _ = query(<opaque>);','let _ = query(<made>);')
mutate('replace-counter-read','let _ = c.final();','let _ = c.get();')
mutate('defer-counter-read','let _ = c.final();','let _ = () => c.final();')
mutate('change-update-value','c.update(c.final().first() + 1)','c.update(c.final().first() + 2)')
mutate('weaken-selected-body-assert','assert!(observed.selected_body_ok);','assert!(true);')
mutate('weaken-attempt-budget','observed.attempts.len() <= 5','observed.attempts.len() <= 6')
mutate('alter-T04-source','panic(\\"P1340_ORIGINAL\\")','panic(\\"CHANGED\\")')
mutate('alter-T05-source','Case::ProvenImpossibleTopologyTerminal => "#let c = counter(heading.where())\\n#context c.final()\\n"','Case::ProvenImpossibleTopologyTerminal => "#let c = counter(heading.where())\\n#context c.get()\\n"')
mutate('alter-T06-source','Case::ClosedIncreasing => "#let c = counter(heading.where())\\n#context c.update(c.final().first() + 1)','Case::ClosedIncreasing => "#let c = counter(heading.where())\\n#context c.update(c.final().first() + 2)')
mutate('change-terminal-message','contextual stability could not be verified','contextual stability changed')
mutate('change-sink-sentinel','P1340_RETAINED','P1340_CHANGED')
mutate('remove-real-opaque-carrier','opaque.then(|| Value::Func(Func::element(','false.then(|| Value::Func(Func::element(')
judgments=[]
for order,items in [('normal',mutations),('repeat',mutations),('reverse',list(reversed(mutations)))]:
    assert checker.check(harness)['oracle_sha256']==PINS[harness.name]
    for name,old,new,data in items:
        try:checker.check(MutantInput(name,data));rejected=False;reason=None
        except AssertionError as e:rejected=True;reason=str(e)
        judgments.append(dict(id=name,order=order,classification='Violated' if rejected else 'Preserved',reason=reason,old=old,new=new,mutated_harness_sha256=hashlib.sha256(data).hexdigest()))
        assert rejected,name
env={k:os.environ[k] for k in ('PATH','HOME','LANG','LC_ALL','LC_CTYPE','TZ','FONTCONFIG_FILE','FONTCONFIG_PATH','TYPST_FONT_PATHS') if k in os.environ};env.update(NO_COLOR='1',TERM='dumb')
outdir=Path(tempfile.mkdtemp(prefix='p1340-verifier-terminal-r5-'));rows=[]
for i,row in enumerate(contract['sources'],1):
    source=sources[row['case']];path=D/f'p1340-verifier-terminal-r5-T0{i}.typ';save(path,source);assert sha(path)==row['successor_sha256']
    argv=[str(BIN),'compile',str(path),str(outdir/f'T0{i}.pdf')];pincheck();at=utc();t=time.monotonic();timed_out=False
    try:p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,timeout=30);code=p.returncode;out=p.stdout;err=p.stderr
    except subprocess.TimeoutExpired as e:code=None;out=e.stdout or b'';err=e.stderr or b'';timed_out=True
    rows.append(dict(case=row['case'],source=source,source_path=str(path),source_sha256=sha(path),argv=argv,cwd=str(ROOT),environment=env,start=at,end=utc(),seconds=time.monotonic()-t,exit=code,timeout=timed_out,stdout=out.decode('utf-8',errors='replace'),stderr=err.decode('utf-8',errors='replace'),stdout_base64=base64.b64encode(out).decode(),stderr_base64=base64.b64encode(err).decode(),classification='Preserved_source_validity_only' if code==0 else ('Unknown' if code not in (0,1) else 'Violated_source_validity'),pdf_present=(outdir/f'T0{i}.pdf').exists()))
pincheck();after=state()
receipt=dict(schema='p1340-verifier-terminal-r5-input-discrimination',regime='executado sem atestação de isolamento',executor='/root/p1311_review',start=start,end=utc(),seconds=time.monotonic()-tick,before=before,after=after,pins=PINS,recorder_sha256=sha(__file__),binary=dict(path=str(BIN),sha256=BH),static_preservation=accepted,source_mutants=dict(valid=15,rejected=15,score=1.0,scope='Concrete harness/source mutations judged by full-byte preservation contract, not productive compiler mutations',orders=['normal','repeat','reverse'],runs=judgments),vanilla_rows=rows,product_processes=3,vanilla_focal_iteration='Independent confirmation of author-reported iteration1; author message/absent raw grants no measurement credit',author_incident='Original r5 oracle executor discarded for reading candidate-containing baseline before oracle output. Only replacement r5b harness is accepted. r5b reported successful focal but receipt absent at delegation; verifier reran exact three sources rather than crediting message.',scope='Input validity only. Vanilla without opaque injection cannot prove terminal outcome. R4 lifecycle remains NOT_SEALED.',verdict='PASS_INPUT_PRESERVATION_AND_VANILLA_VALIDITY' if all(r['exit']==0 for r in rows) else 'BLOCKED_SOURCE_VALIDITY')
path=D/'p1340-verifier-terminal-r5-discrimination.json';save(path,json.dumps(receipt,ensure_ascii=True,indent=2)+'\n')
print(json.dumps(dict(path=str(path),sha256=sha(path),verdict=receipt['verdict'],mutants=15,executions=3,rows=[{k:r[k] for k in ('case','exit','stderr')} for r in rows])))
