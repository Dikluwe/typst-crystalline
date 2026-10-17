from pathlib import Path
import subprocess, os, json, datetime, gzip, hashlib
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
before=json.load(gzip.open(out/'before.json.gz'))
root=Path(before['source'])
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
env=dict(os.environ,CARGO_TARGET_DIR='/tmp/tekt-linter-none-target',CARGO_BUILD_JOBS='1')
record={'start':now(),'commands':[]}
binary='/tmp/tekt-linter-none-target/debug/crystalline-lint'
commands=[
('fmt',['rustfmt','--edition','2021','--config','skip_children=true','03_infra/rs_parser.rs']),
('seal',[before['old_binary']['binary'],'.','--checks','v1,v5,v15,v26','--fix-hashes']),
('regression',['cargo','test','--offline','--locked','--lib','v16_none_','--','--test-threads=1']),
('suite',['cargo','test','--offline','--locked','--','--test-threads=1']),
('build',['cargo','build','--offline','--locked']),
('clippy',['cargo','clippy','--offline','--locked','--all-targets','--','-D','warnings']),
('doc',['cargo','doc','--offline','--locked','--no-deps']),
('fmt-check',['cargo','fmt','--check']),
('lineage',[binary,'.','--checks','v1,v5,v15,v26','--fail-on','warning']),
('self-lint',[binary,'.','--format','sarif'])]
for label,cmd in commands:
    item={'label':label,'command':cmd,'start':now()}
    print(json.dumps(item),flush=True)
    settings=dict(env)
    if label=='doc':settings['RUSTDOCFLAGS']='-D warnings'
    with (out/('green-'+label+'.log')).open('w') as log:
        result=subprocess.run(cmd,cwd=root,env=settings,stdout=log,stderr=subprocess.STDOUT)
    item.update(exit=result.returncode,end=now())
    record['commands'].append(item)
    (out/'green-result.json').write_text(json.dumps(record,indent=2)+'\n')
    print(json.dumps(item),flush=True)
    assert result.returncode==0,item
record.update(end=now(),binary={'path':binary,'sha256':sha(binary)},
              candidate_hashes={str(p.relative_to(root)):sha(p) for p in root.rglob('*') if p.is_file()})
(out/'green-result.json').write_text(json.dumps(record,indent=2)+'\n')
