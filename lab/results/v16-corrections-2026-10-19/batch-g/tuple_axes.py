"""Isolate actual Content tuple evolution classes without executing mutants."""
from pathlib import Path
import argparse,gzip,hashlib,json,os,re,shutil,subprocess,datetime
ROOT=Path.cwd();P=Path(__file__).resolve().parent
ap=argparse.ArgumentParser();ap.add_argument('--base',required=True);ap.add_argument('--name',required=True);a=ap.parse_args()
base=P/a.base;receipt=json.loads((base/'receipt.json').read_text());phase=receipt['phase']
state=json.loads(gzip.decompress((base/'state.json.gz').read_bytes()))
original=json.loads(gzip.decompress((base/'original-sources.json.gz').read_bytes()))
mutants=json.loads(gzip.decompress((base/'enum-mutations.json.gz').read_bytes()))
bridged=json.loads(gzip.decompress((base/'bridged-core-sources.json.gz').read_bytes()))
out=P/('tuple-axes-'+a.name);stage=ROOT/('lab/scratch/tekt-batch-g-axes-'+a.name);assert not out.exists() and not stage.exists();out.mkdir()
for top in ('01_core','02_shell','03_infra','04_wiring','lab/benches','lab/corpus','lab/parity/corpus','lab/typst-original/tests'):
    shutil.copytree(ROOT/top,stage/top)
for name in ('Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml'):
    dest=stage/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/name,dest)
for top in ('01_core','02_shell','03_infra','04_wiring'):
    for path in (stage/top).rglob('*.rs'):
        if str(path.relative_to(stage)) not in original:path.unlink()
for name,text in original.items():(stage/name).write_text(text)
def gz(path,data):
    with gzip.open(path,'wt') as f:json.dump(data,f)
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
decl=original['01_core/src/entities/content.rs'];start=decl.index('pub enum Content {');end=decl.index('\n}',start)
variants=re.findall(r'^    (\w+)(\(|\s*\{|,)',decl[start:end],re.M);assert len(variants)==120
old=' | '.join('Content::'+name+('(..)' if shape=='(' else ' { .. }' if '{' in shape else '') for name,shape in variants)
sentinel='Content::TektV16EvolutionProbe'
masks={'new-left-old-right':f'(_, {sentinel}) => false,','old-left-new-right':f'({sentinel}, _) => false,','new-new':f'({sentinel}, {old}) | ({old}, {sentinel}) => false,'}
targets={t['id']:t for t in receipt['targets']}
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',CARGO_TARGET_DIR=str(ROOT/'lab/target/product'))
records=[]
for case,package in [('V16-074','typst-core'),('V16-098','typst-infra')]:
    for name,mask in masks.items():
        for path,text in original.items():(stage/path).write_text(text)
        for path,text in mutants.items():(stage/path).write_text(text)
        if package=='typst-infra':
            for path,text in bridged.items():(stage/path).write_text(text)
        t=targets[case];spec=t[phase];path=stage/t['file'];text=path.read_text();pos=text.index(spec['anchor'],text.index(spec['after']) if spec.get('after') else 0);insert=pos+len(spec['anchor'])
        path.write_text(text[:insert]+'\n'+mask+'\n'+text[insert:])
        target={'id':case,'file':t['file'],'line':text[:pos].count('\n')+1,'byte_start':len(text[:pos].encode()),'byte_end':len(text[:insert].encode()),'anchor':spec}
        argv=['cargo','check','--offline','--locked','-p',package,'--lib','--message-format=json'];label=case+'-'+name;start=utc();print(start,'START',label,flush=True)
        gz(out/(label+'-source.json.gz'),{'target':target,'mask':mask,'selected_source':path.read_text(),'source_sha256':sha(path),'base_receipt_sha256':sha(base/'receipt.json'),'copied_core_bridge_used':package=='typst-infra'})
        with (out/(label+'.log')).open('w') as log:run=subprocess.run(argv,cwd=stage,env=env,stdout=log,stderr=subprocess.STDOUT)
        errors=[]
        for line in (out/(label+'.log')).read_text().splitlines():
            try:item=json.loads(line)
            except ValueError:continue
            if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
        witnesses=[e for e in errors if (e.get('code') or {}).get('code')=='E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(stage)+'/')==target['file'] and s['line_start']==target['line'] and target['byte_start']<=s['byte_start']<target['byte_end'] for s in e['spans'])]
        other=[e for e in errors if (e.get('code') or {}).get('code')!='E0004']
        rec={'name':label,'phase':phase,'utc_start':start,'utc_end':utc(),'argv':argv,'cwd':str(stage),'exit':run.returncode,'target':target,'selected_witnesses':witnesses,'other_errors':other,'error_count':len(errors),'mask':mask};records.append(rec)
        (out/'commands.json').write_text(json.dumps(records,indent=2)+'\n');print(utc(),'END',label,'selected',len(witnesses),'other',len(other),flush=True)
        assert run.returncode==101 and not other
        assert bool(witnesses)==(phase=='green')
(out/'receipt.json').write_text(json.dumps({'utc_end':utc(),'phase':phase,'baseline_head':state['head'],'base_receipt':str(base/'receipt.json'),'base_receipt_sha256':sha(base/'receipt.json'),'original_sources_sha256':sha(base/'original-sources.json.gz'),'domain_variants':variants,'commands':records,'note':'Each mask covers the other two future tuple classes; only selected residual axis must still demand an explicit decision. Actual copied core dependency is bridged only for infrastructure checks; no runtime execution or productive mutation.'},indent=2)+'\n')
print('PASS six isolated actual tuple-class probes',phase,flush=True)
