from pathlib import Path
import json,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');scope=json.loads((out/'scope.json').read_text());base=json.loads((out/'baseline-details.json').read_text());assert [x['exit'] for x in base['commands'][:2]]==[0,0] and len(base['commands'])==3 and base['commands'][2]['exit']!=0
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before={p:Path(p).read_text() for p in scope['sources']},edits=[],unformatted={})
for p,t in json.loads((out/'tests-materialization.json').read_text())['after'].items():assert record['before'][p]==t

def edit(p,old,new,count=1):
 s=Path(p).read_text();assert s.count(old)==count,(p,s.count(old));s=s.replace(old,new);Path(p).write_text(s);record['edits'].append(dict(path=p,old=old,new=new,count=count))
p=scope['sources'][0]
edit(p,'''match args.items.len() {
                0 => self.call,
                1 if !self.has_spread => {
                    self.positional.first().copied().unwrap_or(args.span)
                }
                2.. if !self.has_spread => {
                    self.positional.get(1).copied().unwrap_or(args.span)
                }
                _ => args.span,
            }''','''match args.items.as_slice() {
                [] => self.call,
                [_] if !self.has_spread => {
                    self.positional.first().copied().unwrap_or(args.span)
                }
                [_, _, ..] if !self.has_spread => {
                    self.positional.get(1).copied().unwrap_or(args.span)
                }
                _ => args.span,
            }''')
edit(p,'''match args.items.len() {
                    0 => return self.call,
                    2.. => return self.positional(1, args.span),
                    _ => {}
                }''','''match args.items.as_slice() {
                    [] => return self.call,
                    [_, _, ..] => return self.positional(1, args.span),
                    [_] => {}
                }''',3)
probe=json.loads((out/'probe-details.json').read_text());edit(scope['sources'][1],probe['old_digit'],probe['new_digit'])
for p in scope['sources']:record['unformatted'][p]=Path(p).read_text()
(out/'materialization.json').write_text(json.dumps(record,indent=2)+'\n');subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True);print('Five replacement sites; six numeric/byte ranges removed, no other algorithm changes.')
