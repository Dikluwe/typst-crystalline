use std::sync::Arc;
use typst_core::compiler::{eval::{EvalContext,EvalTarget},stdlib::{native_table,native_table_cell,native_table_header,native_table_footer},realize::realize_document};
use typst_core::contracts::world::World;
use typst_core::entities::{args::Args,content::{Content,SourceContent},elements::table_cell::{TableCellKind,TableHeaderScope},file_id::FileId,font_book::FontBook,source::Source,span::Span,value::Value,world_types::{Bytes,Datetime,FileError,FileResult,Font,Library},element_registry::ElementRegistry,introspector::TagIntrospector,compiler_features::Features};
#[derive(Default)] struct NullWorld {library:Library,book:FontBook}
impl World for NullWorld {
 fn library(&self)->&Library{&self.library} fn book(&self)->&FontBook{&self.book}
 fn main(&self)->FileId{FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())}
 fn source(&self,_:FileId)->FileResult<Source>{Err(FileError::NotFound)} fn file(&self,_:FileId)->FileResult<Bytes>{Err(FileError::NotFound)}
 fn font(&self,_:usize)->Option<Font>{None} fn today(&self,_:Option<typst_core::entities::duration::Duration>)->Option<Datetime>{None}
}
fn span(at:usize)->Span {Span::from_range(NullWorld::default().main(),at..at+1)}
fn source(c:Content,on:bool)->Content {if on {Content::Source(Arc::new(SourceContent{body:c,span:span(1)}))}else{c}}
// Observation only: public helpers never substitute any production algorithm.
fn body(c:&Content)->&Content {match c {Content::Source(e)=>body(&e.body),Content::Located(e)=>body(&e.body),Content::Realized(e)=>body(&e.visual),other=>other}}
fn content(v:Value)->Content {match v{Value::Content(c)=>c,other=>panic!("content {other:?}")}}
fn header(c:Content,level:i64)->Content {let world=NullWorld::default();let mut args=Args::positional(vec![Value::Content(c)]);args.span=span(9);args.named.insert("level".into(),Value::Int(level));content(native_table_header(&mut EvalContext::new(),&args,&world,world.main()).unwrap())}
fn explicit(kind:TableCellKind)->Content {let world=NullWorld::default();let mut args=Args::positional(vec![Value::Content(Content::text("H"))]);args.span=span(3);args.named.insert("colspan".into(),Value::Int(2));args.named.insert("rowspan".into(),Value::Int(3));args.named.insert("x".into(),Value::Int(0));let mut c=content(native_table_cell(&mut EvalContext::new(),&args,&world,world.main()).unwrap());let Content::TableCell(e)=&mut c else{panic!("cell")};Arc::make_mut(e).kind=kind;c}
fn table(items:Vec<Content>)->Result<Content,String>{let world=NullWorld::default();let mut args=Args::positional(items.into_iter().map(Value::Content).collect());args.named.insert("columns".into(),Value::Int(2));native_table(&mut EvalContext::new(),&args,&world,world.main()).map(content).map_err(|e|e.iter().map(|d|d.message.as_str()).collect::<Vec<_>>().join(";"))}
fn cell_from_header(c:&Content)->&typst_core::entities::elements::table_cell::TableCellElem{let Content::TableHeader(h)=body(c)else{panic!("header")};let Content::TableCell(c)=body(&h.body)else{panic!("cell")};c}
fn find_table(c:&Content)->Option<&typst_core::entities::elements::table::TableElem>{match body(c){Content::Table(t)=>Some(t),Content::Sequence(v)=>v.iter().find_map(find_table),_=>None}}
fn main(){
 let world=NullWorld::default();
 for kind in [TableCellKind::Auto,TableCellKind::Data,TableCellKind::Header{level:7,scope:TableHeaderScope::Column}] {for on in [false,true]{let h=header(source(explicit(kind),on),2);let c=cell_from_header(&h);println!("header kind={kind:?} source={on} x={:?} colspan={:?} rowspan={:?} result_kind={:?} kept_constructor_span={}",c.x,c.colspan,c.rowspan,c.kind,c.source_span==span(3));}}
 for on in [false,true]{
  let args=Args::positional(vec![Value::Content(source(Content::text("F"),on))]);let footer=content(native_table_footer(&mut EvalContext::new(),&args,&world,world.main()).unwrap());let Content::TableFooter(f)=body(&footer)else{panic!("footer")};println!("footer source={on} kept_value_source={}",matches!(&f.body,Content::Source(e) if e.span==span(1)));
  let h1=Content::table_header(source(explicit(TableCellKind::Header{level:2,scope:TableHeaderScope::Column}),on),true);let h2=Content::table_header(source(explicit(TableCellKind::Header{level:3,scope:TableHeaderScope::Column}),on),true);let result=table(vec![h1,h2]);println!("levels source={on} ok={} error={:?}",result.is_ok(),result.err());
  let seq=source(Content::Sequence(vec![Content::text("a"),Content::text("b")].into()),on);let footer=Content::table_footer(seq,true);let raw=table(vec![footer]).unwrap();let src=Source::new(world.main(),"logical-table-probe".into());let (doc,warnings)=realize_document(&raw,&world,&src,&ElementRegistry::new(),&TagIntrospector::empty(),false,EvalTarget::Paged,Features::empty());let doc=doc.unwrap();let t=find_table(&doc.visual).expect("realized table");let resolved=t.resolved.as_ref().unwrap();let cells:Vec<_>=resolved.cells.iter().map(|c|{let Content::TableCell(e)=body(c)else{panic!("resolved cell")};(c.get_field("x"),c.get_field("y"),e.body.plain_text())}).collect();println!("group source={on} warnings={} rows={} cells={cells:?}",warnings.len(),resolved.num_rows);
 }
}
