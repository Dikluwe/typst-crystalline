from pathlib import Path
import gzip,json,re,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert json.loads((out/'baseline-result.json').read_text())['exit']==0
baseline=json.loads((out/'baseline-details.json').read_text());assert all(sha(p)==h for p,h in baseline['source_hashes'].items())
scope=json.loads((out/'scope.json').read_text());texts={p:Path(p).read_text() for p in scope['sources']};edits=[]
def edit(p,old,new):
 assert texts[p].count(old)==1,(p,old,texts[p].count(old));texts[p]=texts[p].replace(old,new,1);edits.append(dict(path=p,old=old,new=new))
for path,method,owner in [('01_core/src/compiler/lexer/mod.rs','next','00_nucleo/prompts/compiler/lexer/mod.md'),('01_core/src/entities/locator.rs','next','00_nucleo/prompts/entities/locator.md'),('01_core/src/entities/decimal.rs','from_str','00_nucleo/prompts/entities/decimal.md'),('01_core/src/entities/version.rs','from_str','00_nucleo/prompts/entities/version.md')]:
 reason=scope['contracts'][owner][1];anchor='    pub fn '+method+'('
 edit(path,anchor,'    #[expect(clippy::should_implement_trait, reason="'+reason+'")]\n'+anchor)
edit('01_core/src/compiler/lexer/mod.rs','/// An iterator over a source code string which returns tokens.','/// Stateful tokenizer with an explicit End token; the parser controls its mode.')
p='01_core/src/entities/duration.rs';edit(p,'let mut rem = self.nanos.abs();','let mut rem = self.nanos.unsigned_abs();')
for expr in ['Self::DAY.nanos','Self::HOUR.nanos','Self::MINUTE.nanos','NANOS_PER_SECOND']:
 for op in [' / ',' %= ',' % ']:
  old=op+expr+';'
  if old in texts[p]:edit(p,old,op+expr+' as u128;')
# Only the formatting slice uses these divisors with rem; no accessor casts change.
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before_implementation={p:Path(p).read_text() for p in texts},edits=edits,unformatted=texts),indent=2)+'\n')
for p,t in texts.items():Path(p).write_text(t)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*texts],check=True)
print(json.dumps(dict(sources=list(texts),edits=len(edits))),flush=True)
