//! Post-window diagnostic World decorator: public APIs, no production changes.
use std::{path::PathBuf,sync::{Arc,Mutex,atomic::{AtomicUsize,Ordering}},time::Instant};
use typst_core::{contracts::{world::{World,SysInputs},plugin_host::PluginHost},entities::{font_book::FontBook,file_id::FileId,source::Source,world_types::{Bytes,Datetime,FileResult,Font,Library},duration::Duration,path::RootedPath,package_spec::PackageSpec}};
use typst_infra::{world::SystemWorld,pipeline::compile_to_pdf_bytes_with_timings,export::{StreamMode,PdfTags}};
struct Observed{inner:SystemWorld,calls:AtomicUsize,book_ns:Mutex<Vec<u128>>,stack:bool,font_calls:Mutex<std::collections::BTreeMap<usize,(usize,u128)>>,candidate_calls:Mutex<Vec<(char,u128)>>}
impl World for Observed {
 fn library(&self)->&Library{self.inner.library()} fn main(&self)->FileId{self.inner.main()}
 fn source(&self,id:FileId)->FileResult<Source>{self.inner.source(id)}fn file(&self,id:FileId)->FileResult<Bytes>{self.inner.file(id)}
 fn book(&self)->&FontBook{let n=self.calls.fetch_add(1,Ordering::Relaxed);if self.stack&&n==0{eprintln!("FIRST BOOK DEMAND:\n{}",std::backtrace::Backtrace::force_capture());}let t=Instant::now();let b=self.inner.book();self.book_ns.lock().unwrap().push(t.elapsed().as_nanos());b}
 fn font(&self,i:usize)->Option<Font>{let t=Instant::now();let f=self.inner.font(i);let ns=t.elapsed().as_nanos();let mut calls=self.font_calls.lock().unwrap();let e=calls.entry(i).or_default();e.0+=1;e.1+=ns;f}fn today(&self,x:Option<Duration>)->Option<Datetime>{self.inner.today(x)}
 fn resolve_path(&self,id:FileId,p:&str)->Result<RootedPath,String>{self.inner.resolve_path(id,p)}
 fn read_path(&self,p:&RootedPath)->Result<Arc<Vec<u8>>,String>{self.inner.read_path(p)}fn include_path(&self,p:&RootedPath)->Result<Source,String>{self.inner.include_path(p)}
 fn read_bytes(&self,id:FileId,p:&str)->Result<Arc<Vec<u8>>,String>{self.inner.read_bytes(id,p)}fn resolve_package(&self,p:&PackageSpec)->Result<Source,String>{self.inner.resolve_package(p)}
 fn inputs(&self)->SysInputs{self.inner.inputs()}fn plugin_host(&self)->Option<Arc<dyn PluginHost>>{self.inner.plugin_host()}fn candidates_for_char(&self,c:char)->Vec<usize>{let t=Instant::now();let v=self.inner.candidates_for_char(c);self.candidate_calls.lock().unwrap().push((c,t.elapsed().as_nanos()));v}
}

mod catalog;
mod compression;
fn main() {
 assert!(!cfg!(debug_assertions));
 let args:Vec<_>=std::env::args().collect();
 if args[1]=="compression" {compression::run();return;}
 if args[1]=="catalog" {catalog::run();return;}
 let input=PathBuf::from(&args[1]);let output=PathBuf::from(&args[2]);let warm=args.iter().any(|a|a=="--warm");
 let total=Instant::now();let t=Instant::now();
 let inner=SystemWorld::new(input.parent().unwrap(),input.file_name().unwrap()).unwrap().with_fonts_and_system(&[]).with_plugin_host(Arc::new(typst_infra::plugin_host::WasmiPluginHost::new()));
 let setup_ms=t.elapsed().as_secs_f64()*1000.0;let t=Instant::now();let source=inner.source(inner.main()).unwrap();let source_ms=t.elapsed().as_secs_f64()*1000.0;
 let t=Instant::now();if warm{let _=inner.book();}let preload_ms=t.elapsed().as_secs_f64()*1000.0;
 let w=Observed{inner,calls:AtomicUsize::new(0),book_ns:Mutex::new(vec![]),stack:args.iter().any(|a|a=="--stack"),font_calls:Mutex::new(Default::default()),candidate_calls:Mutex::new(vec![])};
 let t=Instant::now();let (result,warnings,timings)=compile_to_pdf_bytes_with_timings(&w,&source,StreamMode::Verbose,PdfTags::Disabled);let pipeline_ms=t.elapsed().as_secs_f64()*1000.0;
 let data=result.expect("probe compilation");let t=Instant::now();std::fs::write(output,&data).unwrap();let write_ms=t.elapsed().as_secs_f64()*1000.0;
 let total_ms=total.elapsed().as_secs_f64()*1000.0;let times=w.book_ns.lock().unwrap();let fc=w.font_calls.lock().unwrap();let cc=w.candidate_calls.lock().unwrap();
 println!("{}",serde_json::json!({"warm":warm,"setup_ms":setup_ms,"source_ms":source_ms,"preload_ms":preload_ms,"pipeline_ms":pipeline_ms,"write_ms":write_ms,"total_ms":total_ms,"pdf_bytes":data.len(),"book_calls":w.calls.load(Ordering::Relaxed),"book_first_ms":times.first().copied().unwrap_or(0) as f64/1e6,"book_total_ms":times.iter().sum::<u128>() as f64/1e6,"font_calls":fc.iter().map(|(i,(n,t))|serde_json::json!({"index":i,"calls":n,"ms":*t as f64/1e6})).collect::<Vec<_>>(),"candidate_calls":cc.iter().map(|(c,t)|serde_json::json!({"char":c.to_string(),"ms":*t as f64/1e6})).collect::<Vec<_>>(),"timings":{"eval_ms":timings.eval_ms,"layout_ms":timings.layout_ms,"introspect_ms":timings.introspect_ms,"expand_context_ms":timings.expand_context_ms,"shape_ms":timings.shape_ms,"subset_ms":timings.subset_ms,"render_ms":timings.render_ms,"total_ms":timings.total_ms},"warnings":warnings.len(),"startup_and_teardown_excluded":true,"decorator_overhead_included":true}));
}
