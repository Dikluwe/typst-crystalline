from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,os
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items())
recipe=json.load(open(out.parent/'tekt-v16-math-layout-2026-10-17/audit-recipe.json'));cmd=recipe['command'];cmd[-3:]=[str(out/'workers/audit.rs'),'-o','/tmp/tekt-whitespace-audit'];assert all(sha(v['path'])==v['sha256'] for v in recipe['externs'].values());(out/'audit-recipe.json').write_text(json.dumps(recipe,indent=2)+'\n');subprocess.run(cmd,check=True)
latent=[]
for source in b['sources']:
 matches=json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',source,'matches'],text=True))
 for m in matches:
  for a in m['arms']:
   if source.endswith('html/whitespace.rs') and m['scrutinee']=='visual_content(content)' and a['pattern']=='content':latent.append(dict(source=source,line=a['line'],reason='Projeção de identidade após Styled: complemento nominal sem captura @.',ids=['classified-content-forwarding']))
(out/'latent.json').write_text(json.dumps(latent,indent=2)+'\n');assert len(latent)==1
with (out/'contexts.json').open('w') as f:subprocess.run(['/tmp/tekt-whitespace-audit','.',str(out/'input.sarif.json'),str(out/'latent.json')],stdout=f,check=True)
body='\n'.join(line for line in Path(b['sources'][0]).read_text().splitlines() if not line.startswith('//!'))
probe=r'''
 fn run(parts: &[Content])->Vec<Content> {let mut out=parts.to_vec();normalize_markup_spaces(&mut out);out}
 pub fn measure(){
  use crate::entities::{span::Span,file_id::FileId};
  let span=Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),1..2);
  fn view(c:&Content)->String {match c.source_body(){Content::Empty=>"_".into(),Content::Space=>"S".into(),Content::SpaceFromNewline=>"N".into(),Content::Linebreak(_)=>"L".into(),Content::Parbreak=>"P".into(),Content::Text(t)=>format!("T:{t}"),other=>format!("{other:?}")}}
  for name in ["break-right","break-left","empty-run","empty-run-reverse","explicit-run","paragraph","text"] {for source in [false,true]{
   let mut middle=match name {"break-right"|"break-left"=>Content::linebreak(),"paragraph"=>Content::Parbreak,"text"=>Content::text("x"),_=>Content::Empty};if source{middle=middle.with_source_span(span);}
   let input=match name {"break-right"=>vec![Content::text("a"),Content::Space,middle,Content::text("b")],"break-left"=>vec![Content::text("a"),middle,Content::SpaceFromNewline,Content::text("b")],"empty-run-reverse"=>vec![Content::text("a"),Content::SpaceFromNewline,middle,Content::Space,Content::text("b")],"explicit-run"=>vec![Content::text("a"),Content::Space,middle,Content::Space,Content::text("b")],_=>vec![Content::text("a"),Content::Space,middle,Content::SpaceFromNewline,Content::text("b")]};
   let output=run(&input);let lazy=normalize_markup_spaces_if_changed(&input);println!("{name} source={source} {:?} lazy_changed={}",output.iter().map(view).collect::<Vec<_>>(),lazy.is_some());
  }}
 }
'''
src=out/'workers/source_probe.rs';src.write_text('pub use typst_core::entities;\nmod measured {\n'+body+'\n'+probe+'\n}\nfn main(){measured::measure();}\n');lib=root/'target/debug/libtypst_core.rlib';icu=list((root/'target/debug/deps').glob('libicu_properties-*.rlib'));assert len(icu)==1
cmd=['rustc','--edition=2021',str(src),'-L','dependency='+str(root/'target/debug/deps'),'--extern','typst_core='+str(lib),'--extern','icu_properties='+str(icu[0]),'-o','/tmp/tekt-whitespace-before'];record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=b['hashes'],command=cmd,probe_source_sha256=sha(src),owner_source_sha256=sha(b['sources'][0]),library_sha256=sha(lib),icu_sha256=sha(icu[0]),copied_owner_body=body)
r=subprocess.run(cmd,capture_output=True,text=True);record.update(build_exit=r.returncode,build_stderr=r.stderr);assert r.returncode==0,r.stderr
r=subprocess.run(['/tmp/tekt-whitespace-before'],capture_output=True,text=True);record.update(exit=r.returncode,stdout=r.stdout,stderr=r.stderr,binary_sha256=sha('/tmp/tekt-whitespace-before'),sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()));(out/'source-before.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and record['sources_unchanged'];print(r.stdout,flush=True)
lines=r.stdout.splitlines();pairs=[dict(case=a.split()[0],plain=a,source=z,same=a.replace('source=false','source=*')==z.replace('source=true','source=*')) for a,z in zip(lines[::2],lines[1::2])];assert len(pairs)==7;(out/'source-before-analysis.json').write_text(json.dumps(pairs,indent=2)+'\n');print({'different':[p['case'] for p in pairs if not p['same']]})
