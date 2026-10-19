from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-direcao-alinhamento');bins={'baseline':T/'baseline-typst','candidate':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}};env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1');norm=lambda s:re.sub(r'[\s\u202a-\u202e]','',s)
for n,s in json.loads((H/'fixtures.json').read_text()).items():
 fp=T/(n+'.typ');fp.write_text(s);case={'source':s,'engines':{}}
 for k,b in bins.items():
  pdf=T/(n+'-'+k+'.pdf');cmd=[str(b),'compile',str(fp),str(pdf),'--pdf-tagged=false','--pretty=false'];run=subprocess.run(cmd,env=env,capture_output=True,text=True);assert run.returncode==0,run.stderr
  xml=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);text=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);words=[{'text':e.text,**e.attrib} for e in E.fromstring(xml).iter() if e.tag.endswith('}word')];case['engines'][k]={'command':cmd,'stderr':run.stderr,'bbox':xml,'text':text,'words':words}
 a,b,v=[case['engines'][k] for k in bins];case['text_match']=norm(b['text'])==norm(v['text']);deltas=[];missing=[]
 for w in b['words']:
  matched=[z for z in v['words'] if norm(z['text'] or '')==norm(w['text'] or '') and abs(float(w['yMin'])-float(z['yMin']))<.05]
  if not matched:missing.append(w);continue
  z=min(matched,key=lambda z:abs(float(w['xMin'])-float(z['xMin'])));deltas.append(max(abs(float(w[p])-float(z[p])) for p in ['xMin','xMax','yMin','yMax']))
 case['missing_words']=missing;case['max_bbox_delta_pt']=max(deltas,default=0);case['bbox_matches']=not missing and case['max_bbox_delta_pt']<.05;d['cases'][n]=case;print(n,'text',case['text_match'],'bbox',case['bbox_matches'],'max',round(case['max_bbox_delta_pt'],4),'missing',len(missing),flush=True)
gzip.open(H/'verification.json.gz','wt').write(json.dumps(d,ensure_ascii=False))

required=['rtl_inline_only','rtl_lines','rtl_pars','rtl_scope','ltr_inline_only','ltr_lines','rtl_explicit_par','rtl_terminal','ltr_terminal']
assert all(d['cases'][n]['text_match'] and d['cases'][n]['bbox_matches'] for n in required)
# Whole scripts/explicit align/block certification remains a reported boundary.
