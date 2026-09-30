//! Post-window diagnostic World decorator: public APIs, no production changes.
use std::{path::PathBuf,sync::{Arc,Mutex,atomic::{AtomicUsize,Ordering}},time::Instant};
use typst_core::{contracts::{world::{World,SysInputs},plugin_host::PluginHost},entities::{font_book::FontBook,file_id::FileId,source::Source,world_types::{Bytes,Datetime,FileResult,Font,Library},duration::Duration,path::RootedPath,package_spec::PackageSpec}};
use typst_infra::{world::SystemWorld,pipeline::compile_to_pdf_bytes_with_timings,export::{StreamMode,PdfTags}};
struct Observed{inner:SystemWorld,calls:AtomicUsize,book_ns:Mutex<Vec<u128>>,stack:bool}
impl World for Observed {
 fn library(&self)->&Library{self.inner.library()} fn main(&self)->FileId{self.inner.main()}
 fn source(&self,id:FileId)->FileResult<Source>{self.inner.source(id)}fn file(&self,id:FileId)->FileResult<Bytes>{self.inner.file(id)}
 fn book(&self)->&FontBook{let n=self.calls.fetch_add(1,Ordering::Relaxed);if self.stack&&n==0{eprintln!("FIRST BOOK DEMAND:\n{}",std::backtrace::Backtrace::force_capture());}let t=Instant::now();let b=self.inner.book();self.book_ns.lock().unwrap().push(t.elapsed().as_nanos());b}
 fn font(&self,i:usize)->Option<Font>{self.inner.font(i)}fn today(&self,x:Option<Duration>)->Option<Datetime>{self.inner.today(x)}
 fn resolve_path(&self,id:FileId,p:&str)->Result<RootedPath,String>{self.inner.resolve_path(id,p)}
 fn read_path(&self,p:&RootedPath)->Result<Arc<Vec<u8>>,String>{self.inner.read_path(p)}fn include_path(&self,p:&RootedPath)->Result<Source,String>{self.inner.include_path(p)}
 fn read_bytes(&self,id:FileId,p:&str)->Result<Arc<Vec<u8>>,String>{self.inner.read_bytes(id,p)}fn resolve_package(&self,p:&PackageSpec)->Result<Source,String>{self.inner.resolve_package(p)}
 fn inputs(&self)->SysInputs{self.inner.inputs()}fn plugin_host(&self)->Option<Arc<dyn PluginHost>>{self.inner.plugin_host()}fn candidates_for_char(&self,c:char)->Vec<usize>{self.inner.candidates_for_char(c)}
}
fn main(){
 assert!(!cfg!(debug_assertions),"release required");
 let args:Vec<_>=std::env::args().collect();let input=PathBuf::from(&args[1]);let fonts=PathBuf::from(&args[2]);let output=PathBuf::from(&args[3]);let repeats:usize=args[4].parse().unwrap();let stack=args.iter().any(|a|a=="--stack");let preload=args.iter().any(|a|a=="--preload");
 let setup=Instant::now();let inner=SystemWorld::new(input.parent().unwrap(),input.file_name().unwrap()).unwrap().with_fonts_and_system(&[fonts]);let source=inner.source(inner.main()).unwrap();let setup_ms=setup.elapsed().as_secs_f64()*1000.0;
 let t=Instant::now();if preload {std::hint::black_box(inner.book());}let preload_ms=t.elapsed().as_secs_f64()*1000.0;
 let w=Observed{inner,calls:AtomicUsize::new(0),book_ns:Mutex::new(vec![]),stack};let mut rows=vec![];
 for iteration in 0..repeats {
  w.calls.store(0,Ordering::Relaxed);w.book_ns.lock().unwrap().clear();
  let start=Instant::now();let (result,warnings,timings)=compile_to_pdf_bytes_with_timings(&w,&source,StreamMode::Verbose,PdfTags::Disabled);let pipeline_ms=start.elapsed().as_secs_f64()*1000.0;
  let data=result.expect("fixture compiles");let times=w.book_ns.lock().unwrap();
  rows.push(serde_json::json!({"iteration":iteration,"pipeline_ms":pipeline_ms,"book_calls":w.calls.load(Ordering::Relaxed),"book_ms":times.iter().map(|t|*t as f64/1e6).collect::<Vec<_>>(),"eval_ms":timings.eval_ms,"layout_ms":timings.layout_ms,"render_ms":timings.render_ms,"warnings":warnings.len(),"pdf_bytes":data.len()}));
  if iteration==repeats-1 {std::fs::write(&output,&data).unwrap();}
 }
 println!("{}",serde_json::json!({"input":input,"setup_ms":setup_ms,"preload_ms":preload_ms,"preload":preload,"stack_control":stack,"rows":rows}));
}
