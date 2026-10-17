from pathlib import Path
import subprocess,json,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='02_shell/src/cli.rs'
def run(label,cmd,expected):
 print(label,flush=True)
 with (out/('shell-refinement-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==expected,(label,r.returncode)
run('fmt',['cargo','fmt','--all'],0);run('seal-red',['python3',str(out/'workers/seal_owned.py'),source],0);run('red',['cargo','test','--offline','--locked','-p','typst-shell','--lib','v16_walk_','--','--test-threads=1'],101);assert '1 passed; 2 failed;' in (out/'shell-refinement-red.log').read_text()
p=Path(source);s=p.read_text();needle='''        Content::Title(title) => {''';assert s.count(needle)==1;replacement='''        Content::OutlineEntry(entry) => {
            fields.push(("level".into(), SemanticValue::Int(entry.level as i64)));
            fields.push(("element".into(), content_to_semantic(&entry.element)));
            fields.push(("fill".into(), entry.fill.as_ref().map(content_to_semantic).unwrap_or(SemanticValue::Null)));
        }
''';s=s.replace(needle,replacement+needle,1);s=re.sub(r'^//! @updated .*$', '//! @updated 2026-10-17',s,count=1,flags=re.M);p.write_text(s)
r=json.load(open(out/'refinement.json'));r['sources'][source]=s;r['shell_refinement']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'RED':'1 passed; 2 failed;','scope':'OutlineEntry legacy projection only; authoritative Some untouched'};(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
run('fmt-green',['cargo','fmt','--all'],0);run('seal-green',['python3',str(out/'workers/seal_owned.py'),source],0);run('guard',['python3',str(out/'workers/guard.py')],0)
print('Shell projection corrected after functional RED',flush=True)
