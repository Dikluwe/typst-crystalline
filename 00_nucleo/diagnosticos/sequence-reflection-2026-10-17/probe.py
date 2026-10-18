from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
cases={
'empty-fields':'#repr([].fields())','empty-func':'#repr([].func())','empty-children':'#repr([].children)',
'empty-has':'#repr(( [].has("children"), [].has("body"), [].at("children"), [].at("missing",default:7)))',
'empty-unknown':'#repr([].missing)', 'empty-at-unknown':'#repr([].at("missing"))',
'sequence-fields':'#let c=[A]+[B]\n#repr(c.fields())','sequence-children':'#let c=[A]+[B]\n#repr(c.children)',
'sequence-single':'#let c=[]+[A]\n#repr((c.func(),c.fields(),c.children))',
'sequence-for':'#let c=for i in range(3) {[X]}\n#repr(c.children)',
'children-method':'#repr([].children())','single-text':'#repr([X].children)',
'opaque-styled':'#let c={set text(fill:red); [A]+[B]}\n#repr((c.func(),c.has("children")))',
'empty-call':'#let f=[].func()\n#f()', 'sequence-call':'#let f=([A]+[B]).func()\n#f()',
'empty-selector':'#repr(selector([].func()))','sequence-selector':'#repr(selector(([A]+[B]).func()))',
'empty-counter':'#repr(counter([].func()))',
'empty-json':'#json.encode([])','sequence-json':'#json.encode([A]+[B])',
}
rows=[]
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3'}
for label,src in cases.items():
 f=p/('outputs/'+label+'.typ');f.write_text(src+'\n');row={'label':label,'source':src,'source_sha256':sha(f),'results':{}}
 for name,b in [('tekt',r/'target/debug/typst'),('vanilla',r/'lab/typst-original/target/release/typst')]:
  meta['binaries'][name]={'path':str(b),'sha256':sha(b)}
  out=p/('outputs/'+label+'-'+name+'.pdf');cmd=[str(b),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=30);txt=subprocess.run(['pdftotext',str(out),'-'],capture_output=True,text=True).stdout if not z.returncode else '';row['results'][name]={'argv':cmd,'exit':z.returncode,'stderr':z.stderr,'text':' '.join(txt.split())}
 rows.append(row);print(label,[(k,v['exit'],v['text'] if not v['exit'] else v['stderr'].splitlines()[0]) for k,v in row['results'].items()],flush=True)
meta['cases']=rows
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))
