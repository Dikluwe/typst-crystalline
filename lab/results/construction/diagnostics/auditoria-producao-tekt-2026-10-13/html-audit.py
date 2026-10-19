from pathlib import Path
import re,json,gzip,hashlib,subprocess,tempfile,datetime,os
R=Path.cwd();D=R/'00_nucleo/diagnosticos/auditoria-producao-tekt-2026-10-13';s=(R/'04_wiring/tests/cli.rs').read_text();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'state_sha256':sha(D/'before.json.gz'),'script_sha256':sha(__file__),'cases':{}}
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'};x['binaries']={k:sha(v) for k,v in bins.items()}
names=['p1171_1_html_br_void_e_whitespace_dom','p1173_1_html_lote_global_only_2_e_whitespace_block','p1174_1_html_lote_global_only_3','p1176_1_html_lote_residual_normal','p1177_1_html_familia_ruby','p1178_1_html_familia_documento']
cases={}
for name in names:
 section=s.split('fn '+name+'()')[1].split('\n#[test]')[0]
 lit=re.search(r'let input = temp_typ\(\s*"[^"\\]*",\s*("(?:[^"\\]|\\.)*")',section).group(1)
 cases[name]=(json.loads(lit),'html')
cases['bundle-cross-introspection']=('#document("one.html")[= One <one>\n#context query(heading).len()]\n#document("two.html")[= Two <two>\n= Three <three>\n#context query(heading).len()]','bundle')
cases['title-literal-control']=('#html.html[#html.head[#html.title[ABC]]#html.body[B]]','html')
cases['title-text-control']=('#html.html[#html.head[#html.title[#text("ABC")]]#html.body[B]]','html')
cases['bundle-heading-control']=('#document("one.html")[= One]','bundle')
cases['bundle-text-control']=('#document("one.html")[One]','bundle')
for name,(body,fmt) in cases.items():
 src=D/'fixtures'/(name+'.typ');src.write_text(body);rows={}
 for e,b in bins.items():
  with tempfile.TemporaryDirectory() as tmp:
   out=Path(tmp)/('result.html' if fmt=='html' else 'bundle')
   cmd=[str(b),'compile',str(src),str(out),'--format',fmt,'--features','html' if fmt=='html' else 'bundle,html','--pretty=false']
   p=subprocess.run(cmd,cwd=R,capture_output=True,text=True,timeout=45)
   outputs={str(f.relative_to(Path(tmp))):f.read_text() for f in Path(tmp).rglob('*.html')}
   rows[e]={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'code':p.returncode,'stderr':p.stderr,'stdout':p.stdout,'outputs':outputs}
 x['cases'][name]={'source':body,'source_sha256':sha(src),'engines':rows}
 print(name,json.dumps({e:{'code':r['code'],'outputs':r['outputs'],'stderr':r['stderr'][:180]} for e,r in rows.items()},ensure_ascii=False),flush=True)
with gzip.open(D/'html-probes.json.gz','wt') as f:json.dump(x,f,ensure_ascii=False)
