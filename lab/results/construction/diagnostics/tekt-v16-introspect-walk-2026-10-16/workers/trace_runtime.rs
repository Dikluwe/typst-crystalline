use typst_core::{compiler::{eval::eval, stdlib::native_outline_entry, introspect::introspect_with_introspector}, contracts::world::World, entities::{file_id::FileId, source::Source, font_book::FontBook, world_types::{Library,Routines,Traced,Sink,Route,FileResult,Bytes,FileError,Font,Datetime}, element_registry::ElementRegistry, introspector::Introspector, selector::Selector, func::Func}};
use comemo::Track;
struct ProbeWorld {source:Source, library:Library, book:FontBook}
impl World for ProbeWorld {
 fn library(&self)->&Library {&self.library}
 fn book(&self)->&FontBook {&self.book}
 fn main(&self)->FileId {self.source.id()}
 fn source(&self,_:FileId)->FileResult<Source> {Ok(self.source.clone())}
 fn file(&self,_:FileId)->FileResult<Bytes> {Err(FileError::NotFound)}
 fn font(&self,_:usize)->Option<Font> {None}
 fn today(&self,_:Option<typst_core::entities::duration::Duration>)->Option<Datetime> {None}
}
fn main() {
 let path=std::env::args().nth(1).unwrap();
 let text=std::fs::read_to_string(path).unwrap();
 let source=Source::new(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),text);
 let world=ProbeWorld {source:source.clone(),library:Library::new(),book:FontBook::new()};
 let mut sink=Sink::new(); let route=Route::root(); let traced=Traced::default(); let routines=Routines::new();
 let module=eval(&routines,&world,traced.track(),sink.track_mut(),route.track(),&source,&ElementRegistry::new()).unwrap();
 let selector=Selector::NativeElement(Func::native("outline.entry",native_outline_entry));
 println!("CONTENT {:?}",module.content());
 if let Some(intr)=module.realization_introspector() { println!("CACHED {:?}",intr.query(&selector)); for (l,c) in &intr.elements {println!("CACHED ELEM {:?} {}",l,c.content().elem_name())} }
 if let Some(content)=module.content() { let intr=introspect_with_introspector(content); println!("FRESH {:?}",intr.query(&selector)); for (l,c) in &intr.elements {println!("FRESH ELEM {:?} {}",l,c.content().elem_name())} }
 if let Some(content)=module.content() {
  use typst_core::{compiler::eval::EvalContext, compiler::layout::FixedMetrics, entities::{engine::Engine, style_chain::StyleChain, show::ShowRule}};
  let mut ctx=EvalContext::new(); let mut styles=StyleChain::default_chain();
  let mut rules:std::sync::Arc<[ShowRule]>=std::sync::Arc::from([]); let mut guards=Vec::new();
  let mut runtime_sink=Sink::new(); let mut tracked=runtime_sink.track_mut();
  let mut engine=Engine {world:&world,font_metrics:&FixedMetrics,route:route.track(),styles:&mut styles,show_rules:&mut rules,active_guards:&mut guards,current_file:source.id(),sink:&mut tracked};
  let intr=typst_core::compiler::introspect::introspect_with_runtime(content,&mut engine,&mut ctx).unwrap();
  println!("RUNTIME {:?}",intr.query(&selector));
 }
 let (result,warnings)=typst_infra::pipeline::compile_query_context(&world,&source);
 println!("L3 WARNINGS {:?}",warnings);
 match result { Ok(intr)=> {println!("L3 {:?}",intr.query(&selector));for(l,c)in&intr.elements{println!("L3 ELEM {:?} {}",l,c.content().elem_name())}},Err(errors)=>println!("L3 ERRORS {:?}",errors) }

}
