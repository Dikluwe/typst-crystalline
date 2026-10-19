from pathlib import Path
import json,hashlib,datetime,subprocess
P=Path('00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28');load=lambda f:json.loads((P/f).read_text());checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
def physical(run):
 out=[]
 for page in run.get('pages',[]):
  groups={}
  for line in page['lines']:
   for word in line['words']:
    groups.setdefault(round(word['box']['yMin'],3),[]).append(word)
  out.append({'width':page['width'],'height':page['height'],'lines':[[{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for w in sorted(ws,key=lambda x:x['box']['xMin'])] for _,ws in sorted(groups.items())]})
 return out
EPS=0.0005
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
def hyphen(run):return any(w['text'].endswith(('-', '\u00ad')) for p in run.get('pages',[]) for l in p['lines'] for w in l['words'])
before={x['case']:x for x in load('before-pdf/resultados.json')};after=load('after-pdf/resultados.json');opens={
'native-nested':'Vanilla rejeita Par dentro de Par com warning (inline/collect.rs:247); Tekt ainda o renderiza. Não é aceito como paridade nem interpretado como defeito upstream.',
'native-justify':'auto escolhe optimized no vanilla; Tekt ainda usa seleção simples. Mesmas fronteiras exteriores, quebras internas distintas.',
'paragraph-pages':'page_count coincide mas a admissão de órfãs/viúvas difere; o segundo parágrafo começa na página anterior no Tekt.',
'show-par-repr':'Realização/show permanece divergente; não há nova decisão que aceite paridade do resultado ou do erro.'}
for row in after:
 k=row['case'];a=row['runs']['tekt'];b=row['runs']['vanilla'];check(k+': same source',row['source_sha256']==before[k]['source_sha256']);check(k+': oracle stable',b['exit_code']==before[k]['runs']['vanilla']['exit_code'] and equal(physical(b),physical(before[k]['runs']['vanilla'])))
 if k not in opens:
  check(k+': compiles',a['exit_code']==b['exit_code']==0);check(k+': physical words and boxes agree',equal(physical(a),physical(b)))
empty=load('empty-details-after/results.json')['rows']
for row in empty:
 check('empty '+row['name']+': compiles',row['tekt']['exit']==row['vanilla']['exit']==0)
 def geometry(run):return [{'height':float(p['height']),'words':[{'text':w['text'],'box':{k:float(v) for k,v in w.items() if k!='text'}} for w in p['words']]} for p in run.get('pages',[])]
 check('empty '+row['name']+': geometry agrees',equal(geometry(row['tekt']),geometry(row['vanilla'])))
precedence=load('precedence-pdf/resultados.json')
for row in precedence:
 k=row['case'];a=row['runs']['tekt'];b=row['runs']['vanilla'];expected=k in ['native-text-inline-override','mid-par-set-override'];check(k+': compiles',a['exit_code']==b['exit_code']==0);check(k+': physical words and boxes agree',equal(physical(a),physical(b)));check(k+': lexical hyphenation agrees',hyphen(a)==hyphen(b)==expected)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();binary=sha('target/release/typst')
for folder in ['after-pdf','precedence-pdf','regression-policy','regression-lexical']:
 check(folder+': exact release identity',load(folder+'/proveniencia.json')['binary_sha256']['tekt']==binary)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[x for x in checks if not x['passed']],'binary_sha256':binary,'bilateral_inputs':len(after)+len(empty)+len(precedence),'closed_main_cases':len(after)-len(opens),'open_cases':opens,'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for a,b in load('scope.json').items() for f in [a,b]}}
(P/'acceptance.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'inputs':result['bilateral_inputs'],'closed_main_cases':result['closed_main_cases']},ensure_ascii=False));raise SystemExit(bool(result['failed']))
