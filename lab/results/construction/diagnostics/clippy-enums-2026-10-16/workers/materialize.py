from pathlib import Path
import gzip,json,re,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert json.loads((out/'baseline-result.json').read_text())['exit']==0
baseline=json.loads((out/'baseline-details.json').read_text());assert all(sha(p)==h for p,h in baseline['source_hashes'].items())
scope=json.loads((out/'scope.json').read_text());texts={p:Path(p).read_text() for p in scope['sources']};edits=[]
def edit(p,old,new):
 assert texts[p].count(old)==1,(p,old,texts[p].count(old));texts[p]=texts[p].replace(old,new,1);edits.append(dict(path=p,old=old,new=new))
p=scope['sources'][0]
old='''    OutlineEntryIndented {
        entry: crate::entities::content::Content,
        prefix: Value,
        inner: Value,
        gap: crate::entities::layout_types::Length,
    },'''
edit(p,old,'    OutlineEntryIndented(Box<OutlineEntryIndentedRead>),')
anchor='#[derive(Clone)]\npub(super) struct ContextRead {'
new='''#[derive(Clone)]
pub(crate) struct OutlineEntryIndentedRead {
    entry: crate::entities::content::Content,
    prefix: Value,
    inner: Value,
    gap: crate::entities::layout_types::Length,
}

impl ContextReadRequest {
    pub(crate) fn outline_entry_indented(
        entry: crate::entities::content::Content,
        prefix: Value,
        inner: Value,
        gap: crate::entities::layout_types::Length,
    ) -> Self {
        Self::OutlineEntryIndented(Box::new(OutlineEntryIndentedRead {
            entry, prefix, inner, gap,
        }))
    }
}

'''+anchor
edit(p,anchor,new)
edit(p,'            OutlineEntryIndented { entry, prefix, inner, gap } => {','''            OutlineEntryIndented(payload) => {
                let OutlineEntryIndentedRead { entry, prefix, inner, gap } = payload.as_ref();''')
edit(p,'ContextReadRequest::OutlineEntryIndented { .. } => continue,','ContextReadRequest::OutlineEntryIndented(_) => continue,')
p=scope['sources'][1]
old='''crate::compiler::eval::ContextReadRequest::OutlineEntryIndented {
            entry: Content::OutlineEntry(std::sync::Arc::new(entry)),
            prefix: args.items.get(1).cloned().unwrap_or(Value::None),
            inner: args.items.get(2).cloned().unwrap_or(Value::None),
            gap,
        }'''
new='''crate::compiler::eval::ContextReadRequest::outline_entry_indented(
            Content::OutlineEntry(std::sync::Arc::new(entry)),
            args.items.get(1).cloned().unwrap_or(Value::None),
            args.items.get(2).cloned().unwrap_or(Value::None),
            gap,
        )'''
edit(p,old,new)
for p,enum,reason in [(scope['sources'][2],'LayoutPassOutcome','E002: resultado único de layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento'),(scope['sources'][3],'MathLayoutPassOutcome','E003: resultado único de math layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento')]:
 anchor='pub enum '+enum+' {';edit(p,anchor,'#[expect(clippy::large_enum_variant, reason="'+reason+'")]\n'+anchor)
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before_implementation={p:Path(p).read_text() for p in texts},edits=edits,unformatted=texts),indent=2)+'\n')
for p,t in texts.items():Path(p).write_text(t)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*texts],check=True)
print(json.dumps(dict(sources=list(texts),edits=len(edits))),flush=True)
