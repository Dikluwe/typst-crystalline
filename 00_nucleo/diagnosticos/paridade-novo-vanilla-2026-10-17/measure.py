from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,re,sys,collections,xml.etree.ElementTree as ET
from html.parser import HTMLParser
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17';work=out/'outputs';work.mkdir(exist_ok=True)
load=lambda p:json.load(gzip.open(p));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before=load(out/'before.json.gz');bins=before['binaries'];records=[];fixtures={};specs=[]
assert all(sha(p)==h for p,h in before['source_hashes'].items())
for n,r in bins.items():assert sha(r['path'])==r['sha256']
empty=work/'empty.typ';empty.write_text('// parity audit context\n')
def remember(p):
 p=Path(p)
 if p.is_file() and str(p) not in fixtures:fixtures[str(p)]={'sha256':sha(p),'text':p.read_text()}
 return p
def execute(cmd):
 start=now()
 try:
  r=subprocess.run(cmd,capture_output=True,text=True,timeout=45);result={'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 except subprocess.TimeoutExpired as e:result={'exit':124,'stdout':(e.stdout or b'').decode() if isinstance(e.stdout,bytes) else e.stdout or '', 'stderr':'TIMEOUT'}
 result.update(command=cmd,start=start,end=now())
 result['errors']=re.findall(r'^error: (.*)$',result['stderr'],re.M)
 result['hints']=re.findall(r'(?:^hint: | = hint: )(.*)$',result['stderr'],re.M)
 result['anchors']=re.findall(r'┌─ .*?:(\d+):(\d+)',result['stderr'])
 try:result['value']=json.loads(result['stdout'])
 except ValueError:result['value_raw']=result['stdout']
 return result
class Html(HTMLParser):
 def __init__(self):super().__init__();self.inbody=False;self.text=[];self.rows=[];self.row=None;self.cell=None;self.raw=[]
 def handle_starttag(self,t,a):
  if t=='body':self.inbody=True
  if self.inbody:
   self.raw.append(['start',t,sorted(a)])
   if t=='tr':self.row=[];self.rows.append(self.row)
   if t in ['th','td'] and self.row is not None:self.cell={'tag':t,'attrs':sorted((k,v) for k,v in a if k in ['colspan','rowspan','scope','headers']),'text':''};self.row.append(self.cell)
 def handle_endtag(self,t):
  if self.inbody:self.raw.append(['end',t])
  if t in ['th','td']:self.cell=None
  if t=='tr':self.row=None
  if t=='body':self.inbody=False
 def handle_data(self,s):
  if self.inbody:
   self.text.append(s);self.raw.append(['text',s])
   if self.cell is not None:self.cell['text']+=s

def observe_file(path,result):
 if not path.is_file():return
 result['artifact_sha256']=sha(path)
 if path.suffix=='.pdf' and result['exit']==0:
  box=path.with_suffix('.bbox');r=execute(['pdftotext','-bbox',str(path),str(box)]);result['bbox_run']=r
  if r['exit']==0:
   tree=ET.parse(box);ns='{http://www.w3.org/1999/xhtml}';result['pages']=[{'width':float(p.get('width')),'height':float(p.get('height')),'words':[{'text':w.text,'bounds':[float(w.get(k)) for k in ['xMin','yMin','xMax','yMax']]} for w in p.iter(ns+'word')]} for p in tree.iter(ns+'page')]
 elif path.suffix=='.html':
  text=path.read_text();parser=Html();parser.feed(text);result.update(html=text,body_text=''.join(parser.text),table_rows=parser.rows,body_events=parser.raw)
 elif path.suffix=='.svg':
  text=path.read_text();result['svg']=text
  try:ET.fromstring(text);result['xml_valid']=True
  except ET.ParseError as e:result.update(xml_valid=False,xml_error=str(e))
 elif path.suffix=='.png':
  im=Image.open(path);result['image_size']=list(im.size)

def add_eval(group,label,expr,args=None,third=False):
 args=args or ['eval',expr,'--in',str(empty),'--format','json']
 specs.append({'group':group,'label':label,'mode':'eval','expression':expr,'args':args,'third':third})
def add_compile(group,label,source,fmt='pdf',features=None,third=False,tail=None):
 source=remember(source);args=['compile',str(source),'OUTPUT','--format',fmt]+(['--features',','.join(features)] if features else [])+(tail or [])
 specs.append({'group':group,'label':label,'mode':'compile','source':str(source),'source_sha256':sha(source),'args':args,'fmt':fmt,'third':third})

def flat_eval(folder,filename,group):
 p=root/'00_nucleo/diagnosticos'/folder/filename
 if not p.exists():return
 d=load(p)
 for c in d.get('cases',[]):
  if 'results' in c:r=c['results'].get('tekt');label=c.get('label',c.get('expression',''))
  elif c.get('binary')=='tekt':r=c;label=c.get('name','')
  else:continue
  if not r:continue
  cmd=r.get('argv',r.get('command',[]))
  if len(cmd)>1 and cmd[1]=='eval':
   if '--in' in cmd:remember(cmd[cmd.index('--in')+1])
   add_eval(group,label,c.get('expression',cmd[2]),args=cmd[1:])

flat_eval('tekt-v16-figure-casts-2026-10-17','after-measurement.json.gz','figure-relative')
flat_eval('text-styled-equality-2026-10-17','after-measurement.json.gz','styled-controls')
flat_eval('import-syntax-2026-10-17','after-probe.json.gz','import')
flat_eval('import-syntax-2026-10-17','after-context-probe.json.gz','import-context')
flat_eval('call-once-2026-10-17','after-probe.json.gz','call-controls')
flat_eval('call-once-2026-10-17','after-math-probe.json.gz','math-diagnostics')
flat_eval('tekt-v16-catalogos-2026-10-17','after-measurement.json.gz','catalogue-diagnostics')
for folder in ['import-syntax-2026-10-17','call-once-2026-10-17']:
 for p in (root/'00_nucleo/diagnosticos'/folder).rglob('*.typ'):remember(p)
# Re-execute the RG1 sources without installing any candidate contract.
for filename in ['measurement.json.gz','metadata.json.gz','supplement.json.gz','lines.json.gz','styles-phase.json.gz','svg.json.gz']:
 p=root/'00_nucleo/diagnosticos/table-row-groups-2026-10-17'/filename;d=load(p)
 for c in d.get('cases',[]):
  if c.get('binary')!='tekt':continue
  cmd=c.get('argv',[])
  if len(cmd)>3 and cmd[1]=='compile':
   source=remember(c['source']);fmt=cmd[cmd.index('--format')+1] if '--format' in cmd else Path(cmd[3]).suffix.lstrip('.')
   args=cmd[1:];args[2]='OUTPUT';specs.append({'group':'table-'+p.stem.split('.')[0],'label':c['name'],'mode':'compile','source':str(source),'source_sha256':sha(source),'args':args,'fmt':fmt,'third':False})
# Render debts and valid controls after TS1, from immutable author sources.
for filename,names in [('after-measurement.json.gz',{'named-abs','named-em','nested-affine','show-relative','show-two-relative','named-gradient','pos-gradient','gradient-relative-self'}),('after-extended.json.gz',{'scripts','scripts-abs','heading-replaced','capture-heading-affine','heading-affine-child','heading-level2','heading-level3'})]:
 d=load(root/'00_nucleo/diagnosticos/text-size-length-2026-10-17'/filename)
 for c in d['cases']:
  if c.get('binary')=='tekt' and c['name'] in names:add_compile('text-render',c['name'],c['source'],tail=['--font-path','/usr/share/fonts/truetype/dejavu'])
extra={
 'int-positive-limit':'9223372036854775807','int-overflow-positive':'9223372036854775808','int-negative-literal':'-9223372036854775808','int-min-control':'int.min','int-constructor-min':'int("-9223372036854775808")','int-negative-group':'-(9223372036854775808)','int-positive-explicit-float':'9223372036854775808.0',
 'relative-h':'h(50%+3pt).fields()','relative-box':'box(width:50%+3pt)[A].fields()','relative-block-control':'block(width:50%+3pt)[A].fields()',
 'numeric-h-extension':'h(12).fields()','numeric-box-extension':'box(width:12)[A].fields()',
 'figure-kind-function-vs-name':'figure(kind:table)[A].kind == table','figure-kind-string-control':'figure(kind:"table")[A].kind == "table"',
 'figure-caption-omitted':'figure[A].has("caption")','figure-caption-none':'figure(caption:none)[A].has("caption")',
 'html-off-root':'type(html)','html-off-std':'type(std.html)','html-off-std-alias':'{let s=std;type(s.html)}',
 'symbol-func':'[−].func() == text','bare-symbol-func':'[$alpha$].func()',
 'spanned-heading-equality':'(heading(supplement:[A])[B] == heading(supplement:[A])[B], heading(supplement:[A])[B] == heading(supplement:[C])[B])',
 'filter-return-type':'(1,2,3).filter(i=>i+1)',
 'with-missing-argument':'{let f(a,b,c)=none;let f=f.with(1);f(2)}',
 'with-callback-missing':'{let f(a,b,c)=none;let f=f.with(1);(1,2,3).position(f)}',
 'document-format-function':'document("child.html",format:format.html)[A]',
 'document-format-string':'document("child.html",format:"html")[A]',
 'document-format-auto':'document("child.html")[A]',
}
for name,expr in extra.items():
 third=name in ['spanned-heading-equality','filter-return-type','with-missing-argument','with-callback-missing']
 flags=['--features','bundle,html'] if name.startswith('document-') else []
 add_eval('targeted',name,expr,['eval',expr,'--in',str(empty),'--format','json']+flags,third)
for name in ['math.frac','math.equation','table.cell','figure','text','format.png','std.html.elem','std.pdf.header-cell']:
 add_eval('counter','counter-'+name,'counter('+name+')')
# Cross-file identity under equal feature profiles.
stdchild=work/'stdchild.typ';stdchild.write_text('#let exported=std\n');remember(stdchild)
for flag in [[],['--features','html']]:
 expr='{import "stdchild.typ" as m;std == m.exported}'
 add_eval('std-identity','std-cross-file-'+('html' if flag else 'default'),expr,['eval',expr,'--in',str(empty),'--format','json']+flag)
# Fresh upstream regression inputs; three products separate new oracle deltas.
new_sources={
 'html-raw-utf8':('#html.elem("style", "</abcd−")','html',['html']),
 'heading-supplement-trace':('#set heading(supplement: (h)=>if h.depth==2 {panic("oops")})\n= A\n== B','pdf',None),
 'show-definition-trace':('#let func(c)=panic(c.body.text)\n#show heading:func\n= Heading','pdf',None),
 'grid-inset-trace':('#let compute(x,y)="100"\n#set grid(inset:compute)\n#grid(columns:2,[A],[B],[C],[D])','pdf',None),
 'outline-indent-trace':('#let compute(n)=panic()\n#set outline(indent:compute)\n#outline()\n= Heading','pdf',None),
 'state-update-trace':('#let page_count(n)=panic()\n#state("page").update(page_count)\n#context state("page").final()','pdf',None),
 'cancel-angle-trace':('#let compute(angle)=panic(angle)\n$ cancel((j+e)/(f+e),angle:#compute) $','pdf',None),
 'outline-indent-return-type':('#outline(indent:n=>(a:"dict"))\n= Heading','pdf',None),
}
for name,(text,fmt,flags) in new_sources.items():
 p=work/(name+'.typ');p.write_text(text+'\n');add_compile('upstream-new',name,p,fmt,flags,True)
# PPI, tiling, multipage export and malformed-image validation are observed separately.
for name in ['overlap-alpha','subpixel','cell-parent-gradient','text-cell']:
 p=root/'00_nucleo/diagnosticos/tiling-nativo-2026-10-14/final-renders'/(name+'.typ')
 for fmt in ['pdf','svg','png']:add_compile('tiling',name+'-'+fmt,p,fmt,tail=['--ppi','144'] if fmt=='png' else None)
p=work/'tiling-order.typ';p.write_text((root/'00_nucleo/diagnosticos/tiling-overlap-2026-10-14/final-renders/overlap-order.typ').read_text())
for fmt in ['pdf','svg','png']:add_compile('tiling','order-'+fmt,p,fmt,tail=['--ppi','144'] if fmt=='png' else None)
for ppi in [72,144]:
 p=work/f'ppi-{ppi}.typ';p.write_text('#set page(width:20pt,height:10pt,margin:0pt)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)')
 add_compile('png','ppi-'+str(ppi),p,'png',tail=['--ppi',str(ppi)],third=True)
p=work/'multipage.typ';p.write_text('#set page(width:50pt,height:50pt,margin:2pt)\nA\n#pagebreak()\nB')
for fmt in ['svg','png']:add_compile('multipage',fmt,p,fmt)
for ppi in [None,72]:
 p=work/('bundle-ppi-'+str(ppi)+'.typ');setting='' if ppi is None else '#set format.png(ppi:72)\n'
 p.write_text('#document("tiny.png")[\n'+setting+'#set page(width:2pt,height:2pt,margin:0pt)\n#rect(width:2pt,height:2pt,fill:red,stroke:none)\n]')
 add_compile('png-bundle','ppi-'+str(ppi),p,'bundle',['bundle'],third=True)
# This sample tests the observable of a corrupted, unused image, not data type identity.
broken=work/'broken.png';broken.write_bytes(b'\x89PNG\r\n\x1a\nBAD')
for name,expr in [('image-decode','image("broken.png")'),('unused-image-decode','{let unused=image("broken.png");42}')]:add_eval('images',name,expr)
remember(empty)
(out/'fixtures.json').write_text(json.dumps(fixtures,ensure_ascii=False))
for index,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla']+(['vanilla_old'] if spec.get('third') else []):
  args=spec['args'][:]
  if 'OUTPUT' in args:
   folder=work/f'{index:03d}-{name}';folder.mkdir(exist_ok=True)
   template='result-{p}.'+spec['fmt'] if spec['group']=='multipage' else 'result.'+spec['fmt']
   dest=folder/template;args[args.index('OUTPUT')]=str(dest)
  result=execute([bins[name]['path'],*args])
  if 'dest' in locals() and spec['mode']=='compile':
   observe_file(dest,result)
   if spec.get('fmt')=='bundle' and dest.is_dir():
    result['bundle_pngs']={str(p.relative_to(dest)):list(Image.open(p).size) for p in dest.rglob('*.png')}
   if spec['group']=='multipage':
    result['files']=sorted(p.name for p in folder.iterdir() if p.suffix=='.'+spec['fmt'])
  results[name]=result
 records.append(dict(spec,results=results))
 if index%40==0:
  with gzip.open(out/'measurement.json.gz','wt') as f:json.dump({'start':before['utc'],'updated':now(),'cases':records},f,ensure_ascii=False)
  print(json.dumps({'completed':index+1,'total':len(specs),'group':spec['group']}),flush=True)
assert all(sha(p)==h for p,h in before['source_hashes'].items())
assert all(sha(r['path'])==r['sha256'] for r in bins.values())
with gzip.open(out/'measurement.json.gz','wt') as f:json.dump({'start':before['utc'],'end':now(),'cases':records,'sources_and_binaries_unchanged':True},f,ensure_ascii=False)
print('Measured pairs',len(records),collections.Counter(c['group'] for c in records),flush=True)
