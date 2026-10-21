"""Probe real owner decisions after closed-domain evolution in copied workspaces.

Core witnesses are captured before any bridge. Compiler-suggested probe-only
arms then make that copied dependency build so infrastructure can be checked
against the actual enlarged enums. Selected infrastructure sources are never
bridged. No mutant or bridge enters the productive checkout or runs at runtime.
"""
from pathlib import Path
import argparse,gzip,hashlib,importlib.util,json,os,re,shutil,subprocess

ROOT=Path.cwd();P=Path(__file__).resolve().parent
parser=argparse.ArgumentParser();parser.add_argument('phase',choices=['baseline','green']);parser.add_argument('--name',required=True);parser.add_argument('--prepare-only',action='store_true');parser.add_argument('--prepared',action='store_true')
a=parser.parse_args()
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
out=P/('structural-'+a.name);stage=ROOT/('lab/scratch/tekt-batch-g-'+a.name)
def gz(path,data):
    with gzip.open(path,'wt') as f:json.dump(data,f)
if not a.prepared:
    assert not out.exists() and not stage.exists();out.mkdir()
    for top in ('01_core','02_shell','03_infra','04_wiring','lab/benches','lab/corpus','lab/parity/corpus','lab/typst-original/tests'):
        shutil.copytree(ROOT/top,stage/top)
    for name in ('Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml'):
        dest=stage/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/name,dest)
    baseline=json.loads(gzip.decompress((P/'baseline-inputs.json.gz').read_bytes()))
    if a.phase=='baseline':
        for top in ('01_core','02_shell','03_infra','04_wiring'):
            for path in (stage/top).rglob('*.rs'):
                if str(path.relative_to(stage)) not in baseline['files']:path.unlink()
        for name,text in baseline['files'].items():
            if not name.startswith('00_nucleo/'):
                dest=stage/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_text(text)
    original={str(path.relative_to(stage)):path.read_text() for top in ('01_core','02_shell','03_infra','04_wiring') for path in (stage/top).rglob('*.rs')}
    gz(out/'original-sources.json.gz',original)
    state={'utc':v.utc(),'phase':a.phase,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'actual_inputs':v.frozen_inputs(),'copied_inputs':{str(f.relative_to(stage)):v.digest(f) for f in stage.rglob('*') if f.is_file()},'worker_sha256':v.digest(Path(__file__))}
    gz(out/'state.json.gz',state)
    (out/'prepared.json').write_text(json.dumps({'utc':v.utc(),'phase':a.phase,'stage':str(stage),'source_count':len(original)},indent=2)+'\n')
    if a.prepare_only:
        print('Prepared immutable',a.phase,'workspace',stage,flush=True);raise SystemExit(0)
else:
    assert (out/'prepared.json').exists() and not (out/'receipt.json').exists()
    state=json.loads(gzip.decompress((out/'state.json.gz').read_bytes()))
    original=json.loads(gzip.decompress((out/'original-sources.json.gz').read_bytes()))
    assert all(v.digest(stage/n)==h for n,h in state['copied_inputs'].items())
targets=json.loads((P/'structural-targets.json').read_text())
for path,decl in [('01_core/src/entities/content.rs','pub enum Content {'),('01_core/src/entities/value.rs','pub enum Value {'),('01_core/src/entities/element_kind.rs','pub enum ElementKind {')]:
    dest=stage/path;text=dest.read_text();pos=text.index('\n}',text.index(decl));dest.write_text(text[:pos]+'\n    TektV16EvolutionProbe,'+text[pos:])
def locate(target):
    spec=target[a.phase];text=(stage/target['file']).read_text();pos=text.index(spec['anchor'],text.index(spec['after']) if spec.get('after') else 0)
    return dict(target,line=text[:pos].count('\n')+1,byte_start=len(text[:pos].encode()),byte_end=len(text[:pos+len(spec['anchor'])].encode()))
targets=[locate(t) for t in targets]
gz(out/'enum-mutations.json.gz',{n:(stage/n).read_text() for n,t in original.items() if (stage/n).read_text()!=t})
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',CARGO_TARGET_DIR=str(ROOT/'lab/target/product'))
records=[]
def check(name,package,selected):
    argv=['cargo','check','--offline','--locked','-p',package,'--lib','--message-format=json']
    start=v.utc();print(start,'START',name,flush=True)
    with (out/(name+'.log')).open('w') as log:run=subprocess.run(argv,cwd=stage,env=env,stdout=log,stderr=subprocess.STDOUT)
    errors=[]
    for line in (out/(name+'.log')).read_text().splitlines():
        try:item=json.loads(line)
        except ValueError:continue
        if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
    for t in selected:
        t['witnesses']=[e for e in errors if (e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==t['file'] and s['line_start']==t['line'] and t['byte_start']<=s['byte_start']<t['byte_end'] for s in e['spans'])]
    rec={'name':name,'utc_start':start,'utc_end':v.utc(),'argv':argv,'cwd':str(stage),'exit':run.returncode,'targets':selected,'selected_e0004':sum(bool(t['witnesses']) for t in selected),'error_count':len(errors),'other_errors':[e for e in errors if (e.get('code') or {}).get('code')!='E0004']}
    records.append(rec);(out/'commands.json').write_text(json.dumps(records,indent=2)+'\n')
    print(v.utc(),'END',name,'exit',run.returncode,'selected',rec['selected_e0004'],'other',len(rec['other_errors']),flush=True)
    assert not rec['other_errors'],rec['other_errors']
    return run.returncode,errors
core=[t for t in targets if t['file'].startswith('01_core/')];infra=[t for t in targets if t['file'].startswith('03_infra/')]
exit,errors=check('core-witnesses','typst-core',core)
assert exit==101 and sum(bool(t['witnesses']) for t in core)==(0 if a.phase=='baseline' else len(core))
bridges=[]
for iteration in range(5):
    patches={}
    for error in errors:
        suggestion=next((child for child in error['children'] if child['level']=='help' and any(s.get('suggested_replacement') for s in child['spans'])),None)
        assert suggestion,error
        for span in suggestion['spans']:
            replacement=span.get('suggested_replacement')
            if replacement is None:continue
            name=span['file_name'].removeprefix(str(stage)+'/');assert name.startswith('01_core/'),name
            replacement=re.sub(r'(?<![\w:])entities::','crate::entities::',replacement)
            replacement=re.sub(r'(?<![\w:])(content|value|element_kind)::',r'crate::entities::\1::',replacement)
            patches.setdefault(name,[]).append((span['byte_start'],span['byte_end'],replacement))
    for name,changes in patches.items():
        path=stage/name;data=path.read_bytes();unique=sorted(set(changes),reverse=True)
        for start,end,replacement in unique:data=data[:start]+replacement.encode()+data[end:]
        path.write_bytes(data)
    bridges.append({'iteration':iteration,'compiler_suggestions':patches})
    gz(out/'core-probe-only-bridges.json.gz',bridges)
    exit,errors=check('core-bridge-'+str(iteration),'typst-core',[])
    if exit==0:break
    assert exit==101
assert exit==0,'Copied dependency bridge failed'
gz(out/'bridged-core-sources.json.gz',{n:(stage/n).read_text() for n in original if n.startswith('01_core/') and (stage/n).read_text()!=original[n]})
exit,errors=check('infra-witnesses','typst-infra',infra)
assert exit==101 and sum(bool(t['witnesses']) for t in infra)==(0 if a.phase=='baseline' else len(infra))
assert all((stage/n).read_text()==t for n,t in original.items() if n.startswith('03_infra/')),'Infrastructure target source changed inside probe'
after=v.frozen_inputs();changed=sorted(n for n in state['actual_inputs'].keys()|after.keys() if state['actual_inputs'].get(n)!=after.get(n))
receipt={'utc_end':v.utc(),'phase':a.phase,'head':state['head'],'selected_core':sum(bool(t['witnesses']) for t in core),'selected_infra':sum(bool(t['witnesses']) for t in infra),'targets':targets,'commands':records,'actual_inputs_changed_during_probe':changed,'baseline_is_immutable_recorded_snapshot':a.phase=='baseline','bridge_scope':'only copied core compiler suggestions after core witness recording; no copied infra target edits; no runtime execution'}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
if a.phase=='green':assert not changed
print('PASS real',a.phase,'owner evolution witnesses',receipt['selected_core'],receipt['selected_infra'],flush=True)
