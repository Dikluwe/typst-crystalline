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
}
