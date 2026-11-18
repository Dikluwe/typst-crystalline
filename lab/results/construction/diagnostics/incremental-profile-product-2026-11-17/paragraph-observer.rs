use std::{path::PathBuf,sync::Arc};
use typst_core::contracts::world::World;
use typst_infra::{world::{SystemWorld,SourceSession,FontCatalogSession},pipeline::{compile_pdf_in_session,PdfSessionOptions,PdfCompileSession}};
fn main(){
 let d=PathBuf::from(std::env::args().nth(1).unwrap());let root=d.join("census-shared");std::fs::create_dir_all(&root).unwrap();
 for name in ["references_500","outline_100","mixed_report","math_supported","unicode_repo"]{
  let base=std::fs::read_to_string(d.join("inputs").join(format!("{name}.typ"))).unwrap();
  let sources=Arc::new(SourceSession::new());let fonts=Arc::new(FontCatalogSession::new());let session=PdfCompileSession::new();
  for i in 0..4{
   let text=if i==0{base.clone()}else{format!("{base}\n// paragraph census {i}\n")};std::fs::write(root.join("main.typ"),&text).unwrap();
   let world=SystemWorld::new_with_source_session(&root,"main.typ",sources.clone()).unwrap().with_font_catalog_session(fonts.clone(),&[]);let source=world.source(world.main()).unwrap();let before=session.paragraphs.stats();
   let (result,warnings,_)=compile_pdf_in_session(&world,&source,PdfSessionOptions{full_error:false,document_id:Some([3;16]),stream_mode:typst_infra::export::StreamMode::Verbose,pdf_tagged_override:Some(true),features:Default::default(),pretty_override:None,pages_override:None,standard_override:None},&session);
   let bytes=result.unwrap();let dir=d.join("paragraph-census").join(format!("{name}-{i}"));std::fs::create_dir_all(&dir).unwrap();std::fs::write(dir.join("out.pdf"),bytes).unwrap();std::fs::write(dir.join("main.typ"),text).unwrap();let after=session.paragraphs.stats();
   println!("{name}\t{i}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",before.0,before.1,before.2,after.0,after.1,after.2,warnings.len());
   drop(warnings);drop(source);drop(world);comemo::evict(10);
  }
 }
}
