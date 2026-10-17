from pathlib import Path
import gzip,json,re,hashlib,datetime,subprocess,difflib
out=Path('00_nucleo/diagnosticos/clippy-retencao-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert json.loads((out/'baseline-result.json').read_text())['exit']==0
record=json.loads((out/'baseline-details.json').read_text());assert all(sha(p)==h for p,h in record['source_hashes'].items())
texts={p:Path(p).read_text() for p in before['files'] if p.endswith('.rs')};edits=[]
def edit(p,old,new):
 assert texts[p].count(old)==1,(p,old,texts[p].count(old));texts[p]=texts[p].replace(old,new,1);edits.append(dict(path=p,old=old,new=new))
p='03_infra/src/pipeline/context_stabilization.rs'
edit(p,'    final_styles: StyleChain,','    // Retains style-owned values until this execution record is dropped.\n    _retained_styles: StyleChain,')
edit(p,'            final_styles: styles,','            _retained_styles: styles,')
edit(p,'    parent: Option<usize>,\n    children: Vec<usize>,','    children: Vec<usize>,')
edit(p,'            chain,\n            parent,\n            children: Vec::new(),','            chain,\n            children: Vec::new(),')
edit(p,'    retired: Vec<Box<Execution>>,','    retired: Vec<Execution>,')
for name in ['previous','old','discovery']:edit(p,f'self.retired.push({name});',f'self.retired.push(*{name});')
p='03_infra/src/export/subset.rs';a=texts[p].index('/// Wrapper compatível com a assinatura do Prompt L0 original.');b=texts[p].index('/// Aplica o mapa de remapeamento',a);edit(p,texts[p][a:b],'')
edit(p,'let subset = subset_font(&data, &used).expect("subset deve funcionar");','let subset = subset_font_with_mapping(&data, &BTreeMap::new(), &used)\n            .expect("subset deve funcionar").data;')
edit(p,'let subset = subset_font(&data, &used).expect("subset vazio deve funcionar");','let subset = subset_font_with_mapping(&data, &BTreeMap::new(), &used)\n            .expect("subset vazio deve funcionar").data;')
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before_implementation={p:Path(p).read_text() for p in texts},edits=edits,unformatted=texts),indent=2)+'\n')
for p,t in texts.items():Path(p).write_text(t)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*texts],check=True)
print(json.dumps(dict(sources=list(texts),changes=len(edits))),flush=True)
