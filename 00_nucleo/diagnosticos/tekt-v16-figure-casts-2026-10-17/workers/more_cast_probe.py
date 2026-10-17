from pathlib import Path
import subprocess,json,datetime,hashlib,sys
out=Path(__file__).resolve().parents[1];values={'None':'none','Auto':'auto','Bool':'true','Int':'1','Float':'1.2','Str':'"A"','Array':'(1,2)','Dictionary':'(a:1)','Module':'std','Datetime':'datetime(year:2020,month:1,day:1)','Function':'assert','Content':'[A]','Length':'1pt','Ratio':'50%','Relative':'50%+1pt','Angle':'1deg','Color':'red','Stroke':'1pt+red','Fraction':'1fr','Alignment':'left','Gradient':'gradient.linear(red,blue)','Regex':'regex("a")','Tiling':'tiling(size:(2pt,2pt),[x])','Bytes':'bytes((1,2))','Decimal':'decimal("1")','Duration':'duration(seconds:1)','Version':'version(1,2,3)','Selector':'selector(heading)','Symbol':'sym.alpha','Arguments':'((..a)=>a)(1)','State':'state("k",0)','Counter':'counter("k")','Label':'<x>','Direction':'ltr','Path':'path("x")','Type':'int','SpotColorant':'color.spot("p",red)'};cases={}
for kind,value in values.items():cases['type-'+kind]='figure([A],kind:type('+value+'))'
for value in ['α','^','(',')','|','⌈','↔\ufe0e','ab','\u0302','[','⌊']:
 expr='symbol('+json.dumps(value,ensure_ascii=False)+')';cases['symbol-kind-'+value]='figure([A],kind:'+expr+')';cases['symbol-body-'+value]='figure('+expr+').body';cases['symbol-caption-'+value]='figure([A],caption:'+expr+').caption'
rows=[]
for name,query in cases.items():
 for kind,binary in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]:
  cmd=[str(Path(binary).resolve()),'eval',query,'--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(name=name,binary=kind,query=query,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,argv=cmd))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=json.load(open(out/'validation-result.json'))['source_hashes'],binaries={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in ['target/debug/typst','lab/typst-original/target/release/typst']},rows=rows);(out/('more-cast-probe.json' if len(sys.argv)==1 else 'more-cast-after.json')).write_text(json.dumps(record,indent=2)+'\n')
for r in rows:
 if r['binary']=='vanilla':print(r['name'],r['exit'],r['stderr'].splitlines()[:1] if r['exit'] else r['stdout'].strip()[:100])
