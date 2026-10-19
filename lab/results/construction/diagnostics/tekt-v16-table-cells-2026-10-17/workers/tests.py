from pathlib import Path
import datetime,gzip,json,hashlib,subprocess
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));assert json.load(open(out/'l0-before-code.json'))['productive_sources_unchanged'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items() if p not in b['owners']);world=r'''
use crate::contracts::world::World;
use crate::entities::{file_id::FileId,font_book::FontBook,source::Source,world_types::{Library,Bytes,Datetime,FileResult,FileError,Font}};
struct TestWorld {library:Library,book:FontBook,source:Source}
impl TestWorld {fn new(text:&str)->Self{Self{library:Library::new(),book:FontBook::new(),source:Source::new(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),text.into())}}}
impl World for TestWorld {
 fn library(&self)->&Library{&self.library} fn book(&self)->&FontBook{&self.book} fn main(&self)->FileId{self.source.id()}
 fn source(&self,_:FileId)->FileResult<Source>{Ok(self.source.clone())} fn file(&self,_:FileId)->FileResult<Bytes>{Err(FileError::NotFound)}
 fn font(&self,_:usize)->Option<Font>{None} fn today(&self,_:Option<crate::entities::duration::Duration>)->Option<Datetime>{None}
}
'''
mods={}
mods[b['sources'][0]]=r'''
use super::*;
use crate::entities::elements::table_cell::{TableCellElem,TableCellKind,TableHeaderScope,TableCellStandaloneResolved,TableCellStage};
use std::sync::Arc;
fn span(at:usize)->Span {Span::from_range(TestWorld::new("").main(),at..at+1)}
fn sourced(c:Content)->Content{Content::Source(Arc::new(crate::entities::content::SourceContent{body:c,span:span(1)}))}
fn unwrap_value(v:Value)->Content{let Value::Content(c)=v else{panic!("content")};c}
fn raw(kind:TableCellKind)->Content{
 let world=TestWorld::new("");let mut args=Args::positional(vec![Value::Content(Content::text("H"))]);args.span=span(3);
 for (name,n) in [("x",0),("colspan",2),("rowspan",3)]{args.named.insert(name.into(),Value::Int(n));}
 let mut c=unwrap_value(native_table_cell(&mut EvalContext::new(),&args,&world,world.main()).unwrap());let Content::TableCell(cell)=&mut c else{panic!("cell")};Arc::make_mut(cell).kind=kind;c
}
fn header(c:Content)->Content{let world=TestWorld::new("");let mut args=Args::positional(vec![Value::Content(c)]);args.span=span(9);args.named.insert("level".into(),Value::Int(2));unwrap_value(native_table_header(&mut EvalContext::new(),&args,&world,world.main()).unwrap())}
fn header_cell(h:&Content)->&TableCellElem{let Content::TableHeader(h)=h.source_body()else{panic!("header")};let Content::TableCell(c)=h.body.source_body()else{panic!("cell")};c}
fn footer(c:Vec<Content>)->Content{let world=TestWorld::new("");let mut args=Args::positional(c.into_iter().map(Value::Content).collect());args.span=span(9);unwrap_value(native_table_footer(&mut EvalContext::new(),&args,&world,world.main()).unwrap())}
fn footer_body(c:&Content)->&Content{let Content::TableFooter(f)=c.source_body()else{panic!("footer")};&f.body}
fn assert_header(kind:TableCellKind,expected:TableCellKind){let h=header(sourced(raw(kind)));let c=header_cell(&h);assert_eq!((c.x,c.colspan,c.rowspan),(Some(0),Some(2),Some(3)));assert_eq!(c.kind,expected);assert_eq!(c.source_span,span(3));assert!(c.presence.colspan&&c.presence.rowspan);let Content::TableHeader(h)=h.source_body()else{panic!("header")};assert_eq!(h.body.source_span(),span(1));assert!(!matches!(c.body.source_body(),Content::TableCell(_)));}
#[test] fn v16_table_header_source_auto(){assert_header(TableCellKind::Auto,TableCellKind::Header{level:2,scope:TableHeaderScope::Column});}
#[test] fn v16_table_header_source_data(){assert_header(TableCellKind::Data,TableCellKind::Data);}
#[test] fn v16_table_header_source_explicit_header(){let k=TableCellKind::Header{level:7,scope:TableHeaderScope::Row};assert_header(k,k);}
#[test] fn v16_table_header_raw_uses_value_origin(){let h=header(sourced(Content::text("raw")));let c=header_cell(&h);assert_eq!(c.source_span,span(1));assert!(c.stage().is_none());assert_eq!(c.body.plain_text(),"raw");}
#[test] fn v16_table_header_preserves_prepared_occurrence(){let mut c=raw(TableCellKind::Data);let Content::TableCell(e)=&mut c else{panic!("cell")};let e=Arc::make_mut(e);e.standalone=Some(Arc::new(TableCellStandaloneResolved{visual:Content::text("shown"),stage:TableCellStage::Complete,guards:vec![41],properties:None}));let h=header(sourced(c));let c=header_cell(&h);assert_eq!(c.stage(),Some(TableCellStage::Complete));assert_eq!(c.guards(),&[41]);assert_eq!(c.kind,TableCellKind::Data);}
#[test] fn v16_table_footer_one_sequence_argument_is_one_raw_cell(){let arg=Content::Sequence(vec![Content::text("a"),Content::text("b")].into());let f=footer(vec![sourced(arg)]);let Content::TableCell(c)=footer_body(&f).source_body()else{panic!("one raw cell per argument")};assert_eq!(c.body.plain_text(),"ab");assert!(c.stage().is_none());assert!(c.resolved.is_none());assert_eq!(c.source_span,span(1));assert_eq!(footer_body(&f).source_span(),span(1));}
#[test] fn v16_table_footer_explicit_cell_keeps_origin_and_properties(){let f=footer(vec![sourced(raw(TableCellKind::Data))]);assert_eq!(footer_body(&f).source_span(),span(1));let Content::TableCell(c)=footer_body(&f).source_body()else{panic!("cell")};assert_eq!((c.colspan,c.rowspan),(Some(2),Some(3)));assert_eq!(c.kind,TableCellKind::Data);assert_eq!(c.source_span,span(3));}
#[test] fn v16_table_footer_two_arguments_are_two_raw_cells(){let f=footer(vec![Content::text("a"),Content::text("b")]);let Content::Sequence(items)=footer_body(&f).source_body()else{panic!("two cells")};assert_eq!(items.len(),2);for c in items.iter(){let Content::TableCell(e)=c.source_body()else{panic!("raw cell")};assert!(e.stage().is_none());}}
fn headers(a:u32,b:u32)->SourceResult<Value>{let h=|level|Content::table_header(sourced(raw(TableCellKind::Header{level,scope:TableHeaderScope::Column})),true);let world=TestWorld::new("");native_table(&mut EvalContext::new(),&Args::positional(vec![Value::Content(h(a)),Value::Content(h(b))]),&world,world.main())}
#[test] fn v16_table_header_source_levels_remain_distinct(){assert!(headers(2,3).is_ok());}
#[test] fn v16_table_header_source_duplicate_level_still_errors(){let e=headers(2,2).unwrap_err();assert!(e[0].message.contains("mais do que um header"));}
'''
mods[b['sources'][1]]=r'''
use super::*;
use crate::compiler::{layout::FixedMetrics,realize::realize_table_insertions};
use crate::entities::{content::ShowRecipeContent,span::Span,style::Styles,style_chain::StyleChain,show::{ShowRule,Transformation},world_types::{Route,Sink}};
use comemo::Track;
fn recipe(on:bool,floor:u64)->Value {let c=Content::ShowRecipe(Arc::new(ShowRecipeContent{body:Content::Empty,textual:false,context_id_floor:floor,rules:Arc::from([]),styles:StyleChain::empty(),guards:vec![],file:FileId::from_raw(std::num::NonZeroU16::new(2).unwrap())}));Value::Content(if on{c.with_source_span(Span::from_range(TestWorld::new("").main(),1..2))}else{c})}
fn capture(mode:&str,local:Option<Value>,bad_defaults:bool)->(u64,bool){
 let world=TestWorld::new("");let mut styles=StyleChain::default_chain().push_custom("eval.context.recipe",recipe(false,31));let mut content=Content::table_cell(Content::text("x"),None,None,None,None);
 if mode=="styled"{let mut delta=Styles::default();if let Some(v)=local{delta=delta.push_custom("eval.table.local.recipe",v);}content=Content::styled(Box::new(content),delta);}else if let Some(v)=local{styles=styles.push_custom("eval.table.local.recipe",v);}
 if bad_defaults{styles=styles.push_custom("cell.colspan",Value::Str("bad".into()));}
 let route=Route::root().with_id(world.main());let metrics=FixedMetrics;let mut sink=Sink::new();let mut rules:Arc<[ShowRule]>=Arc::from([ShowRule{id:71,selector:Selector::Text("not-x".into()),transform:Transformation::Str("unused".into())}]);let mut guards=vec![99];let mut ctx=EvalContext::new();let error;
 {let mut tracked=sink.track_mut();let mut engine=Engine{world:&world,font_metrics:&metrics,route:route.track(),styles:&mut styles,show_rules:&mut rules,active_guards:&mut guards,current_file:world.main(),sink:&mut tracked};error=realize_table_insertions(content,&mut ctx,&mut engine).is_err();assert_eq!(engine.current_file,world.main());assert_eq!(engine.active_guards.as_slice(),&[99]);assert_eq!(engine.show_rules.len(),1);assert_eq!(engine.show_rules[0].id,71);}
 (ctx.next_context_id,error)
}
#[test] fn v16_table_snapshot_source_at_root(){assert_eq!(capture("root",Some(recipe(true,77)),false),(77,false));}
#[test] fn v16_table_snapshot_source_in_styled(){assert_eq!(capture("styled",Some(recipe(true,77)),false),(77,false));}
#[test] fn v16_table_snapshot_error_restores_context(){assert_eq!(capture("root",Some(recipe(true,77)),true),(77,true));}
#[test] fn v16_table_snapshot_local_precedes_general(){assert_eq!(capture("root",Some(recipe(true,77)),false).0,77);assert_eq!(capture("root",None,false).0,31);}
#[test] fn v16_table_snapshot_invalid_local_value_shadows_general(){assert_eq!(capture("root",Some(Value::Int(0)),false).0,0);}
#[test] fn v16_table_snapshot_nonrecipe_content_shadows_general(){assert_eq!(capture("root",Some(Value::Content(Content::text("not-a-recipe"))),false).0,0);}
'''
mods[b['sources'][2]]=r'''
use crate::compiler::{eval::{EvalContext,EvalTarget},stdlib::native_table,realize::realize_document};
use crate::entities::{args::Args,content::Content,span::Span,value::Value,element_registry::ElementRegistry,introspector::TagIntrospector,compiler_features::Features};
fn span()->Span{Span::from_range(TestWorld::new("").main(),1..2)}
fn table(raw:Vec<Content>)->Content{let w=TestWorld::new("");let mut args=Args::positional(raw.into_iter().map(Value::Content).collect());args.named.insert("columns".into(),Value::Int(2));let Value::Content(t)=native_table(&mut EvalContext::new(),&args,&w,w.main()).unwrap()else{panic!("table")};t}
fn find_table(c:&Content)->Option<&crate::entities::elements::table::TableElem>{match c{Content::Source(s)=>find_table(&s.body),Content::Located(e)=>find_table(&e.body),Content::Realized(e)=>find_table(&e.visual),Content::Table(t)=>Some(t),Content::Sequence(v)=>v.iter().find_map(find_table),Content::Empty=>None,other=>panic!("unsupported observation {other:?}")}}
fn realize(t:Content)->crate::entities::source_result::SourceResult<crate::compiler::realize::RealizedDocument>{let w=TestWorld::new("logical table");realize_document(&t,&w,&w.source,&ElementRegistry::new(),&TagIntrospector::empty(),false,EvalTarget::Paged,Features::empty()).0}
fn group(header:bool){for on in [false,true]{let v=Content::Sequence(vec![Content::text("a"),Content::text("b")].into());let v=if on{v.with_source_span(span())}else{v};let g=if header{Content::table_header(v,true)}else{Content::table_footer(v,true)};let doc=realize(table(vec![g])).unwrap();let t=find_table(&doc.visual).unwrap();let r=t.resolved.as_ref().unwrap();let texts=r.cells.iter().map(|c|c.plain_text()).collect::<Vec<_>>();assert_eq!(texts,["a","b"]);assert_eq!(r.num_rows,1);}}
#[test] fn v16_table_header_group_source_sequence(){group(true);}
#[test] fn v16_table_footer_group_source_sequence(){group(false);}
#[test] fn v16_table_conflict_uses_first_cell_origin(){let h=Content::table_header(Content::text("H"),true);let c=Content::table_cell(Content::text("body"),None,Some(0),None,None).with_source_span(span());let e=realize(table(vec![h,c])).err().expect("header conflict");assert_eq!(e[0].span,span());assert!(e[0].message.contains("header"));}
'''
mods[b['sources'][3]]=r'''
fn run(text:&str)->crate::entities::source_result::SourceResult<crate::entities::module::Module>{let w=TestWorld::new(text);crate::compiler::eval::tests::eval_for_test(&w,&w.source)}
#[test] fn v16_table_show_none_literal_is_empty_content(){let m=run("#show \"gone\": none\ngoneZ").unwrap();let text=m.content().unwrap().plain_text();assert!(!text.contains("gone"));assert!(text.contains('Z'));}
#[test] fn v16_table_show_none_element_is_empty_content(){let m=run("#show strong: none\n*gone* Z").unwrap();let text=m.content().unwrap().plain_text();assert!(!text.contains("gone"));assert!(text.contains('Z'));}
#[test] fn v16_table_show_none_does_not_admit_other_scalars(){for value in ["1","auto"]{assert!(run(&format!("#show strong: {value}\n*gone*")).is_err());}}
'''
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),additions={},expected_files={})
for source,code in mods.items():
 assert Path(source).read_text()==b['files'][source];addition='\n\n#[cfg(test)]\nmod v16_table_tests {\n'+world+'\n'+code+'\n}\n';record['additions'][source]=addition;Path(source).write_text(b['files'][source]+addition);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',source],check=True);record['expected_files'][source]=Path(source).read_text()
record['new_tests']=sum(v.count('#[test]') for v in record['additions'].values());assert record['new_tests']==22;(out/'tests-before-code.json').write_text(json.dumps(record,indent=2)+'\n');print('22 tests written before implementation',flush=True)
