//! Post-window diagnostic World decorator: public APIs, no production changes.
use std::{path::PathBuf,sync::{Arc,atomic::{AtomicUsize,Ordering}}};
use typst_core::{contracts::{world::{World,SysInputs},plugin_host::PluginHost},entities::{font_book::FontBook,file_id::FileId,source::Source,world_types::{Bytes,Datetime,FileResult,Font,Library},duration::Duration,path::RootedPath,package_spec::PackageSpec}};
use typst_infra::{world::SystemWorld,pipeline::compile_to_pdf_bytes_with_timings,export::{StreamMode,PdfTags}};
struct Observed{inner:SystemWorld,calls:AtomicUsize,stack:bool}
impl World for Observed {
 fn library(&self)->&Library{self.inner.library()} fn main(&self)->FileId{self.inner.main()}
 fn source(&self,id:FileId)->FileResult<Source>{self.inner.source(id)}fn file(&self,id:FileId)->FileResult<Bytes>{self.inner.file(id)}
 fn book(&self)->&FontBook{let n=self.calls.fetch_add(1,Ordering::Relaxed);if self.stack&&n==0{eprintln!("FIRST BOOK DEMAND:\n{}",std::backtrace::Backtrace::force_capture());}self.inner.book()}
 fn font(&self,i:usize)->Option<Font>{self.inner.font(i)}fn today(&self,x:Option<Duration>)->Option<Datetime>{self.inner.today(x)}
 fn resolve_path(&self,id:FileId,p:&str)->Result<RootedPath,String>{self.inner.resolve_path(id,p)}
 fn read_path(&self,p:&RootedPath)->Result<Arc<Vec<u8>>,String>{self.inner.read_path(p)}fn include_path(&self,p:&RootedPath)->Result<Source,String>{self.inner.include_path(p)}
 fn read_bytes(&self,id:FileId,p:&str)->Result<Arc<Vec<u8>>,String>{self.inner.read_bytes(id,p)}fn resolve_package(&self,p:&PackageSpec)->Result<Source,String>{self.inner.resolve_package(p)}
 fn inputs(&self)->SysInputs{self.inner.inputs()}fn plugin_host(&self)->Option<Arc<dyn PluginHost>>{self.inner.plugin_host()}fn candidates_for_char(&self,c:char)->Vec<usize>{self.inner.candidates_for_char(c)}
}
fn main(){
 let args:Vec<_>=std::env::args().collect();let input=PathBuf::from(&args[1]);let fonts=PathBuf::from(&args[2]);let output=PathBuf::from(&args[3]);
 let inner=SystemWorld::new(input.parent().unwrap(),input.file_name().unwrap()).unwrap().with_fonts_and_system(&[fonts]);let source=inner.source(inner.main()).unwrap();
 let w=Observed{inner,calls:AtomicUsize::new(0),stack:false};
 let (result,warnings,_timings)=compile_to_pdf_bytes_with_timings(&w,&source,StreamMode::Verbose,PdfTags::Disabled);
 let data=result.expect("fixture must compile");let calls=w.calls.load(Ordering::Relaxed);std::fs::write(output,data).unwrap();
 println!("{}",serde_json::json!({"input":input,"book_calls":calls,"warnings":warnings.len(),"functional_control_not_a_performance_run":true}));
}
