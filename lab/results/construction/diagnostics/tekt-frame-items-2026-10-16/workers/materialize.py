from pathlib import Path
import json,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tekt-frame-items-2026-10-16');scope=json.loads((out/'scope.json').read_text());rows=json.loads((out/'classification.json').read_text())
assert json.loads((out/'baseline-result.json').read_text())['exit']==0
assert json.loads((out/'probe-baseline-result.json').read_text())['exit']==0
before={p:Path(p).read_text() for p in scope['sources']};edits=[]
for r in rows:
 p=r['source'];data=before[p].encode()
 if r['action']=='nominalize':
  a,b=r['pattern_start_byte'],r['pattern_end_byte'];old=r['pattern_source'];new=r['new_pattern'];kind='pattern'
 else:
  a,b=r['arm_start_byte'],r['arm_end_byte'];old=r['arm_source'];new='';kind='dead-arm'
 assert data[a:b].decode()==old,(r['id'],data[a:b].decode(),old)
 edits.append(dict(path=p,start=a,end=b,old=old,new=new,name=r['id'],kind=kind))
p='01_core/src/entities/layout_types.rs';old='''/// Centraliza a travessia de `Group` e `Link` e define um ponto único de fallback
/// com `debug_assert!` para capturar variantes novas em tempo de debug/teste sem
/// custo em release.''';new='''/// Centraliza a travessia recursiva e cobre todas as variantes de `FrameItem`.
/// Uma variante nova exige decidir seu callback e sua travessia em compilação.''';data=before[p].encode();a=data.index(old.encode());edits.append(dict(path=p,start=a,end=a+len(old.encode()),old=old,new=new,name='visitor-doc',kind='doc'))
replay={}
for p,t in before.items():
 data=t.encode()
 for e in sorted((e for e in edits if e['path']==p),key=lambda e:e['start'],reverse=True):
  assert data[e['start']:e['end']].decode()==e['old'];data=data[:e['start']]+e['new'].encode()+data[e['end']:]
 replay[p]=data.decode()
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before=before,edits=edits,unformatted=replay)
(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
for p,t in replay.items():Path(p).write_text(t)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
print('69 padrões, um braço morto e sua supressão removidos; documentação do visitante atualizada.')
