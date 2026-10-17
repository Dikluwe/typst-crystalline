use typst_core::contracts::world::World;
use typst_core::entities::{file_id::FileId,scope::Scope,value::Value};
use typst_core::entities::compiler_features::{Feature,Features};
#[derive(Default)]
struct NullWorld { library:typst_core::entities::world_types::Library,book:typst_core::entities::font_book::FontBook }
impl World for NullWorld {
 fn library(&self)->&typst_core::entities::world_types::Library {&self.library}
 fn book(&self)->&typst_core::entities::font_book::FontBook {&self.book}
 fn main(&self)->FileId {FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())}
 fn source(&self,_:FileId)->typst_core::entities::world_types::FileResult<typst_core::entities::source::Source> {Err(typst_core::entities::world_types::FileError::NotFound)}
 fn file(&self,_:FileId)->typst_core::entities::world_types::FileResult<typst_core::entities::world_types::Bytes> {Err(typst_core::entities::world_types::FileError::NotFound)}
 fn font(&self,_:usize)->Option<typst_core::entities::world_types::Font> {None}
 fn today(&self,_:Option<typst_core::entities::duration::Duration>)->Option<typst_core::entities::world_types::Datetime> {None}
}
fn walk(profile:u8,path:&str,scope:&Scope,stack:&mut Vec<usize>) {
 let id=scope as *const Scope as usize;
 if stack.contains(&id) {eprintln!("CYCLE\t{profile}\t{path}");return}
 if stack.len()>12 {panic!("inventory depth exceeded at {path}")}
 stack.push(id);
 for (name,binding) in scope.iter() {
  let path=format!("{path}.{name}");
  match binding.value() {
   Value::Func(f)=>{println!("{profile}\t{path}\tfunction\t{}",f.name().unwrap_or(""));if let Some(ns)=f.namespace(){walk(profile,&path,ns,stack)}}
   Value::Module(m)=>walk(profile,&path,m.scope(),stack),
   Value::Type(t)=>println!("{profile}\t{path}\ttype\t{}",t.name()),
   _=>{}
  }
 }
 stack.pop();
}
fn main(){
 for profile in 0..8 {
  let mut features=Features::empty();
  if profile&1!=0 {features.enable(Feature::Html)}
  if profile&2!=0 {features.enable(Feature::A11yExtras)}
  if profile&4!=0 {features.enable(Feature::Bundle)}
  let (result,_)=typst_core::compiler::eval::eval_expression_with_features(&NullWorld::default(),"std",features);
  let Value::Module(module)=result.expect("std evaluation failed") else {panic!("std must be a module")};
  walk(profile,"std",module.scope(),&mut vec![]);
 }
}
