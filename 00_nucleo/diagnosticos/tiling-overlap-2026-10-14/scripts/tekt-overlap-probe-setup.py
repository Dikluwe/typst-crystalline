from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14')
with gzip.open(out/'before.json.gz','rt') as f:old=json.load(f)
names=list(old['files'])+['00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml']
snapshot=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in names})
with gzip.open(out/'candidate-probe.json.gz','wt') as f:json.dump(snapshot,f)
s=Path('/tmp/tekt-native-valid-probe.py').read_text().replace('tiling-nativo-2026-10-14','tiling-overlap-2026-10-14').replace('candidate-final.json.gz','candidate-probe.json.gz')
index=s.index('record=dict(start=now()')
extra='''
quad = '#box(width:8pt,height:8pt)[' + ''.join('#place(dx:'+str(x)+'pt,dy:'+str(y)+'pt)[#rect(width:4pt,height:4pt,fill:rgb('+c+',50%),stroke:none)]' for x,y,c in [(0,0,'255,0,0'),(4,0,'0,0,255'),(0,4,'0,255,0'),(4,4,'255,255,0')]) + ']'
cases['overlap-order'] = '#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,8pt),spacing:(-4pt,-4pt))['+quad+'],stroke:none)'
cases['solid-alpha'] = '#rect(width:40pt,height:30pt,fill:rgb(255,0,0,50%),stroke:(paint:rgb(0,0,255,25%),thickness:4pt))'
cases['text-alpha'] = '#text(size:16pt,fill:rgb(255,0,0,50%))[AA]#text(size:16pt,fill:blue)[BB]'
cases['text-alpha-cell'] = '#rect(width:40pt,height:30pt,fill:tiling(size:(12pt,12pt))[#text(size:8pt,fill:rgb(0,0,255,50%))[A]],stroke:none)'
'''
s=s[:index]+extra+s[index:];Path('/tmp/tekt-overlap-probe.py').write_text(s)
