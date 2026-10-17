from pathlib import Path
import subprocess,gzip,json,hashlib,datetime,sys
root=Path.cwd();out=root/'00_nucleo/diagnosticos/raw-presenca-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
paths=['02_shell/src/cli.rs','00_nucleo/prompts/shell/cli.md','01_core/src/compiler/stdlib/loading/encoding.rs','00_nucleo/prompts/compiler/stdlib/loading/encoding.md','01_core/src/entities/elements/raw.rs','00_nucleo/prompts/entities/elements/raw.md','01_core/src/compiler/stdlib/structural/markup.rs','00_nucleo/prompts/compiler/stdlib/structural/markup.md']
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in paths},rows=[])
cases={
'empty':'[]','empty-code':'{set text()}', 'styled-empty':'{set text(fill:red)}',
'raw-default':'raw("x")','raw-false':'raw("x",block:false)','raw-true':'raw("x",block:true)','raw-lang':'raw("x",lang:"typc")','raw-lang-none':'raw("x",lang:none)','raw-all':'raw("x",block:false,lang:"typc")',
'raw-markup':'[`x`]','raw-block-markup':'[```typc\nx\n```]','display-bool':'{set text() if false;true}','display-auto':'{set text() if false;auto}',
'raw-fields-default':'raw("x").fields()','raw-fields-false':'raw("x",block:false).fields()','raw-fields-none':'raw("x",lang:none).fields()',
'source-text':'[x]','source-sequence':'[x #strong[y]]','source-styled':'{set text(fill:red);[x]}','nested-empty':'(a:[],b:{set text(fill:red)},c:([],))',
'raw-none-block':'raw("x",block:none)','raw-no-text':'raw()',
}
cases.update({'raw-eq-block':'raw("x")==raw("x",block:false)','raw-eq-lang':'raw("x")==raw("x",lang:none)','raw-has-none':'raw("x",lang:none).has("lang")','raw-at-none':'raw("x",lang:none).at("lang")','raw-at-omitted':'raw("x").at("lang")','raw-at-unknown':'raw("x").at("missing")','raw-alias':'{let f=raw;let a=f("x",block:false,lang:none);a.fields()}','raw-none-both':'raw("x",block:false,lang:none)'})
for name,expr in cases.items():
 modes=['cli-json','encode-json']
 if name in ['empty','styled-empty','raw-all','source-styled','raw-none-both']:modes+=['cli-yaml','encode-yaml']
 for mode in modes:
  expression=('json.encode(('+expr+'),pretty:false)' if mode=='encode-json' else 'yaml.encode(('+expr+'))' if mode=='encode-yaml' else expr)
  fmt='raw' if mode.startswith('encode') else 'yaml' if mode=='cli-yaml' else 'json'
  for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
   cmd=[str(exe),'eval',expression,'--format',fmt];start=now();v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=name,mode=mode,start=start,end=now(),expression=expression,command=cmd,binary=label,binary_sha256=sha(exe),exit=v.returncode,stdout=v.stdout,stderr=v.stderr);r['rows'].append(row)
   print(name,mode,label,v.returncode,repr(v.stdout[:220]),v.stderr.splitlines()[:1],flush=True)
r['end']=now();r['sources_unchanged']=all(sha(p)==v['sha256'] for p,v in r['files'].items());assert r['sources_unchanged']
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
