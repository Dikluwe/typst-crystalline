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
 assert!(!cfg!(debug_assertions),"performance mode must be release");
 let args:Vec<_>=std::env::args().collect();let input=PathBuf::from(&args[1]);let fonts=PathBuf::from(&args[2]);let output=PathBuf::from(&args[3]);let warm=args.iter().any(|a|a=="--warm");let stack=args.iter().any(|a|a=="--stack");
 let inner=SystemWorld::new(input.parent().unwrap(),input.file_name().unwrap()).unwrap().with_fonts_and_system(&[fonts]);let source=inner.source(inner.main()).unwrap();
 let before=Instant::now();if warm{let _=inner.book();}let preload_ms=before.elapsed().as_secs_f64()*1000.0;
 let w=Observed{inner,calls:AtomicUsize::new(0),book_ns:Mutex::new(vec![]),stack};let start=Instant::now();let (result,warnings,timings)=compile_to_pdf_bytes_with_timings(&w,&source,StreamMode::Verbose,PdfTags::Disabled);let pipeline_ms=start.elapsed().as_secs_f64()*1000.0;
 let data=result.expect("equivalent baseline fixture must compile");std::fs::write(output,data).unwrap();let times=w.book_ns.lock().unwrap();
 println!("{}",serde_json::json!({"input":input,"warm":warm,"stack_control":stack,"preload_ms":preload_ms,"pipeline_ms":pipeline_ms,"book_calls":w.calls.load(Ordering::Relaxed),"book_first_ms":times.first().copied().unwrap_or(0) as f64/1e6,"book_total_ms":times.iter().sum::<u128>() as f64/1e6,"book_entries":w.inner.book().len(),"timings":{"eval_ms":timings.eval_ms,"layout_ms":timings.layout_ms,"introspect_ms":timings.introspect_ms,"expand_context_ms":timings.expand_context_ms,"shape_ms":timings.shape_ms,"subset_ms":timings.subset_ms,"render_ms":timings.render_ms,"total_ms":timings.total_ms},"warnings":warnings.len(),"setup_parsing_and_output_write_excluded":true,"decorator_overhead_included":true}));
}
