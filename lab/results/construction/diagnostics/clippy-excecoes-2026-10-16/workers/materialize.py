from pathlib import Path
import gzip,json,hashlib,re,subprocess,collections,datetime
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));red=json.loads((out/'red-details.json').read_text());assert len(red['commands'])==3 and all(c['exit']==101 for c in red['commands'])
changes={};records=[]
for p in before['plans']:
 source=Path(p['source']);data=source.read_bytes();assert hashlib.sha256(data).hexdigest()==before['files'][str(source)]['sha256'],source
 if 'span' in p:
  offset=data.rfind(b'\n',0,p['span']['byte_start'])+1
  assert re.search(rb'\bfn '+re.escape(p['function'].encode())+rb'\b',data[offset:p['span']['byte_end']])
 else:
  matches=list(re.finditer(rb'(?m)^([ \t]*)(?:pub(?:\([^)]*\))? )?fn '+re.escape(p['function'].encode())+rb'\b',data));assert len(matches)==1,p['id'];offset=matches[0].start()
 indent=re.match(rb'[ \t]*',data[offset:]).group();attribute=f'#[expect({p["rule"]}, reason = {json.dumps(p["reason"],ensure_ascii=False)})]'
 addition=indent+attribute.encode()+b'\n';changes.setdefault(str(source),[]).append((offset,addition));records.append(dict(id=p['id'],source=str(source),function=p['function'],rule=p['rule'],reason=p['reason'],inserted_at_byte=offset))
for source,edits in changes.items():
 p=Path(source);data=p.read_bytes();assert len(set(a for a,b in edits))==len(edits),source
 for offset,addition in sorted(edits,reverse=True):data=data[:offset]+addition+data[offset:]
 p.write_bytes(data)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*changes],check=True)
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),attributes=records),ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(attributes=len(records),sources=len(changes))))
