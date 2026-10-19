from pathlib import Path
import json,subprocess,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='01_core/src/compiler/eval/call_dispatch/content_methods.rs';entity='01_core/src/entities/elements/outline_entry.rs'
def run(label,cmd,expected):
 print(label,flush=True)
 with (out/('methods-refinement-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==expected,(label,r.returncode)
run('fmt',['cargo','fmt','--all'],0);run('seal-red',['python3',str(out/'workers/seal_owned.py'),source,entity],0);run('red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_walk_','--','--test-threads=1'],101)
assert '10 passed; 2 failed;' in (out/'methods-refinement-red.log').read_text()
p=Path(entity);s=p.read_text();needle='''        match self.element.source_body() {''';assert s.count(needle)==1;s=s.replace(needle,'''        let mut element = self.element.source_body();
        while let Content::Located(located) = element {
            element = located.body.source_body();
        }
        match element {''',1);p.write_text(s)
p=Path(source);s=p.read_text();old='''                content
            {''';assert s.count(old)==1;s=s.replace(old,'''                content.and_then(outline_entry_receiver)
            {''',1);at=s.index('\n#[cfg(test)]\nfn outline_entry_receiver');s=s[:at]+'\n';variants=json.load(open(out/'scope.json'))['variants'];patterns=[v for n,v in variants.items() if n not in ['Located','OutlineEntry']];group='        '+('\n        | ').join(patterns)+' => None,';s+='''
fn outline_entry_receiver(content: &crate::entities::content::Content) -> Option<&crate::entities::content::Content> {
    use crate::entities::content::Content;
    let content = content.source_body();
    match content {
        Content::Located(located) => outline_entry_receiver(&located.body),
        Content::OutlineEntry(_) => Some(content),
'''+group+'''
    }
}
''';s=re.sub(r'^//! @updated .*$', '//! @updated 2026-10-16',s,count=1,flags=re.M);p.write_text(s)
r=json.load(open(out/'refinement.json'));r['sources'].update({p:Path(p).read_text() for p in [source,entity]});r['methods_refinement']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'real_RED':'attempt-method-carriers-measurement-after.json','unit_RED':'10 passed; 2 failed;','helper_RED_scaffold':'cfg(test) copy of old raw-only recognition; removed in final source'};(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
run('fmt-green',['cargo','fmt','--all'],0);run('seal-green',['python3',str(out/'workers/seal_owned.py'),source,entity],0);run('guard',['python3',str(out/'workers/guard.py')],0)
print('Receiver and entity carrier refinement applied after RED',flush=True)
