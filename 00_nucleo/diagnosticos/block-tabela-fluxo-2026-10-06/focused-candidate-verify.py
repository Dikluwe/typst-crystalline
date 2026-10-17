from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E
R=Path.cwd();H=R/'00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06';T=Path('/tmp/tekt-block-table')
fixtures={k:v for k,v in json.loads((H/'fixtures.json').read_text()).items() if k in json.loads((R/'00_nucleo/diagnosticos/espaco-terminal-2026-10-06/summary.json').read_text())['focused']['remaining']}
bins={'before':T/'typst-before','after':T/'typst-candidate','vanilla':R/'lab/typst-original/target/release/typst'}
git=lambda *a:subprocess.check_output(['git',*a],text=True)
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1')
# pdftotext can add embeddings to mark visual direction: retain raw extraction
# and report both exact and formatting-insensitive comparisons explicitly.
norm=lambda s:re.sub(r'[\s\u202a-\u202e]','',s or '')

def grouped(words):
 result=[]
 for page in sorted(set(w['page_index'] for w in words)):
  lines=[]
  for w in [w for w in words if w['page_index']==page]:
   line=next((line for line in lines if abs(float(line[0]['yMin'])-float(w['yMin']))<.05),None)
   if line is None:lines.append([w])
   else:line.append(w)
  for line in lines:
   line.sort(key=lambda w:float(w['xMin']));groups=[]
   for word in line:
    w={**word,'text':norm(word['text'])}
    if groups and abs(float(w['xMin'])-float(groups[-1]['xMax']))<.05:
     g=groups[-1];g['text']+=w['text'];g['xMax']=w['xMax'];g['yMin']=str(min(float(g['yMin']),float(w['yMin'])));g['yMax']=str(max(float(g['yMax']),float(w['yMax'])))
    else:groups.append(w)
   result.extend(groups)
 return result
for name,source in fixtures.items():
 fp=T/(name+'.typ');fp.write_text(source);case={'source':source,'engines':{}}
 for k,binary in bins.items():
  pdf=T/(name+'-'+k+'.pdf');cmd=[str(binary),'compile',str(fp),str(pdf),'--pdf-tagged=false','--pretty=false'];
  if name.startswith('variable-') or name=='static-axis':cmd += ['--font-path',str(R/'03_infra/fixtures/fonts')]
  run=subprocess.run(cmd,env=env,capture_output=True,text=True);assert run.returncode==0,(name,run.stderr)
  xml=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);raw=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True)
  words=[{'text':e.text,**e.attrib,'page_index':i} for i,page in enumerate(e for e in E.fromstring(xml).iter() if e.tag.endswith('}page')) for e in page.iter() if e.tag.endswith('}word')];case['engines'][k]={'command':cmd,'stderr':run.stderr,'text':raw,'words':words,'bbox':xml}
 v=case['engines']['vanilla']
 for k in ['before','after']:
  c=case['engines'][k];remaining=v['words'].copy();deltas=[];missing=[]
  for word in c['words']:
   choices=[z for z in remaining if z['page_index']==word['page_index'] and norm(z['text'])==norm(word['text']) and abs(float(z['yMin'])-float(word['yMin']))<.05]
   if not choices:missing.append(word);continue
   z=min(choices,key=lambda z:abs(float(z['xMin'])-float(word['xMin'])));remaining.remove(z)
   deltas.append(max(abs(float(word[p])-float(z[p])) for p in ['xMin','xMax','yMin','yMax']))
  case[k+'_comparison']={'exact_text_match':c['text']==v['text'],'formatting_insensitive_text_match':norm(c['text'])==norm(v['text']),'max_bbox_delta_pt':max(deltas,default=0),'missing':missing,'unmatched_vanilla':remaining,'bbox_match':not missing and not remaining and max(deltas,default=0)<.05}
 for k in ['before','after']:
  remaining=grouped(v['words']);missing=[];deltas=[]
  for word in grouped(case['engines'][k]['words']):
   choices=[z for z in remaining if z['page_index']==word['page_index'] and z['text']==word['text'] and abs(float(z['yMin'])-float(word['yMin']))<.05]
   if not choices:missing.append(word);continue
   z=min(choices,key=lambda z:abs(float(z['xMin'])-float(word['xMin'])));remaining.remove(z);deltas.append(max(abs(float(word[p])-float(z[p])) for p in ['xMin','xMax','yMin','yMax']))
  case[k+'_comparison']['grouped_bbox_match']=not missing and not remaining and max(deltas,default=0)<.05
  case[k+'_comparison']['grouped_max_bbox_delta_pt']=max(deltas,default=0)
  case[k+'_comparison']['grouped_missing']=missing
  case[k+'_comparison']['grouped_unmatched_vanilla']=remaining
 d['cases'][name]=case
 a=case['after_comparison'];print(name,'text',a['formatting_insensitive_text_match'],'bbox',a['bbox_match'],'grouped',a['grouped_bbox_match'],'max',round(a['max_bbox_delta_pt'],4),'missing',len(a['missing']),flush=True)
with gzip.open(H/'focused-candidate-verification.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
(H/'focused-candidate-verify.py').write_text(Path(__file__).read_text())
