"""Validate acquisition and exact projection ledger on system, embedded and fixture faces."""
from pathlib import Path
import subprocess,gzip,json,datetime,hashlib,tempfile,os
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before=json.load(gzip.open(D/'before.json.gz','rt'))
productive_digest=sha(R/'03_infra/src/fontdb/selective.rs')
font_paths=set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines())
for root in ['/usr/share/fonts','/usr/local/share/fonts','/home/dikluwe/.fonts','/home/dikluwe/.local/share/fonts','/usr/share/texmf/fonts','/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts',str(R/'03_infra/fixtures/fonts')]:
 for f in Path(root).rglob('*'):
  if f.is_file() and f.suffix.lower() in ('.ttf','.otf','.ttc'):font_paths.add(str(f))
font_before={str(Path(p).resolve()):sha(p) for p in font_paths if Path(p).is_file()}
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'script_sha256':sha(__file__),'compiler':subprocess.check_output(['rustc','--version','--verbose'],text=True),'sources':{},'libraries':{},'scope':'Validation only. Two copied productive private project bodies plus diagnostic wrappers; no timing claims from this standalone probe.'}
with tempfile.TemporaryDirectory(prefix='selective-corpus-') as tmp:
 T=Path(tmp)
 for label,source in [('old',before['owners']['03_infra/src/fontdb/selective.rs']),('new',(R/'03_infra/src/fontdb/selective.rs').read_text())]:
  prefix=source[:source.index('#[cfg(test)]')]
  text=prefix+"""
struct Trace<'a> { cursor: std::io::Cursor<&'a [u8]>, reads: usize, seeks: usize }
impl Read for Trace<'_> { fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> { self.reads += 1; self.cursor.read(b) } }
impl Seek for Trace<'_> { fn seek(&mut self, p: std::io::SeekFrom) -> std::io::Result<u64> { self.seeks += 1; self.cursor.seek(p) } }
pub(super) fn projected(bytes: &[u8], index: u32) -> Option<(Vec<u8>, u64, usize, usize)> {
 let mut trace=Trace { cursor: std::io::Cursor::new(bytes), reads: 0, seeks: 0 };
 let (b,n)=project(&mut trace, bytes.len() as u64, index)?; Some((b,n,trace.reads,trace.seeks))
}
"""
  (T/(label+'.rs')).write_text(text);state['sources'][label]={'text':text,'sha256':sha(T/(label+'.rs'))}
 cov=(R/'03_infra/src/fonts/coverage.rs').read_text();cov=cov[:cov.index('#[cfg(test)]')];(T/'coverage.rs').write_text(cov)
 main='''mod old; mod new;
mod fonts {
 pub use typst_infra::fonts::font_info_from_bytes;
 #[path = "COVERAGE_PATH"] mod coverage;
 pub(crate) fn extract_coverage(face: &ttf_parser::Face) -> typst_core::entities::font_book::Coverage { coverage::extract(face) }
}
fn main() {
 let mut db=fontdb::Database::new();db.load_system_fonts();
 let mut inputs=Vec::new();
 for face in db.faces() {
  if let fontdb::Source::File(path) | fontdb::Source::SharedFile(path,_) = &face.source { inputs.push((path.clone(),face.index,"system")); }
 }
 for (directory,kind) in [("/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts","embedded"),("03_infra/fixtures/fonts","fixture")] {
  for entry in std::fs::read_dir(directory).unwrap() {
   let path=entry.unwrap().path();if !path.is_file() {continue;}
   let bytes=std::fs::read(&path).unwrap();
   for index in 0..ttf_parser::fonts_in_collection(&bytes).unwrap_or(1) {
    if ttf_parser::Face::parse(&bytes,index).is_ok() {inputs.push((path.clone(),index,kind));}
   }
  }
 }
 let mut rows=Vec::new();
 for (path,index,kind) in inputs {
  let bytes=std::fs::read(&path).unwrap();
  let a=old::projected(&bytes,index);let b=new::projected(&bytes,index);
  assert_eq!(a.is_some(),b.is_some(),"projection acceptance {:?}",path);
  if let (Some(a),Some(b)) = (&a,&b) {
   assert_eq!(a.0,b.0,"projection bytes {:?} face {}",path,index);
   assert!(b.1<=a.1 && b.2<=a.2 && b.3<=a.3,"read ledger {:?}",path);
  }
  let ledger=serde_json::json!({"before":a.as_ref().map(|v|(v.0.len(),v.1,v.2,v.3)),"after":b.as_ref().map(|v|(v.0.len(),v.1,v.2,v.3))});
  let a=old::acquire(&path,index);let b=new::acquire(&path,index);
  assert_eq!(a,b,"metadata/coverage {:?} face {}",path,index);
  let legacy=fonts::font_info_from_bytes(&bytes,index).and_then(|mut info| {
   let face=ttf_parser::Face::parse(&bytes,index).ok()?;info.coverage=fonts::extract_coverage(&face);Some(info)
  });
  assert_eq!(b.clone().or(legacy.clone()),legacy,"fallback/full parser {:?} face {}",path,index);
  rows.push(serde_json::json!({"path":path,"index":index,"kind":kind,"accepted":b.is_some(),"ledger":ledger,"metadata_and_coverage_equal":true,"projected_bytes_equal_and_requests_nonincreasing":true}));
  if rows.len()%200==0 {eprintln!("validated {} faces",rows.len());}
 }
 println!("{}",serde_json::json!({"all_equal":true,"faces":rows}));
}
'''.replace('COVERAGE_PATH',str(T/'coverage.rs'))
 (T/'main.rs').write_text(main);state['sources']['main']={'text':main,'sha256':sha(T/'main.rs')};state['sources']['coverage']={'text':cov,'sha256':sha(T/'coverage.rs')}
 cmd=['rustc','--edition=2021','-O',str(T/'main.rs'),'-L','dependency='+str(R/'target/debug/deps'),'-o',str(T/'probe')]
 for name in ['typst_core','typst_infra','fontdb','ttf_parser','serde_json']:
  lib=max((R/'target/debug/deps').glob('lib'+name+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns);state['libraries'][str(lib)]={'sha256':sha(lib)};cmd+=['--extern',name+'='+str(lib)]
 state['build']=cmd;p=subprocess.run(cmd,cwd=R,capture_output=True,text=True);state['build_stderr']=p.stderr;assert p.returncode==0,p.stderr
 state['binary_sha256']=sha(T/'probe');cmd=[str(T/'probe')];state['run']=cmd
 p=subprocess.run(cmd,cwd=R,capture_output=True,text=True,timeout=180);state['returncode']=p.returncode;state['stderr']=p.stderr;assert p.returncode==0,p.stderr
 state['result']=json.loads(p.stdout)
 paths={x['path'] for x in state['result']['faces']};state['fonts_sha256']={path:sha(path) for path in paths}
 assert all(str(Path(path).resolve()) in font_before and font_before[str(Path(path).resolve())]==digest for path,digest in state['fonts_sha256'].items())
 state['font_hashes_before']=font_before
 state['productive_source_sha256']=productive_digest
 assert sha(R/'03_infra/src/fontdb/selective.rs')==productive_digest
 assert sha(__file__)==state['script_sha256']
 state['utc_end']=utc()
 with gzip.open(D/'corpus-validation.json.gz','wt') as f:json.dump(state,f)
 print('all equal',len(state['result']['faces']),'faces;',sum(x['accepted'] for x in state['result']['faces']),'accepted')
