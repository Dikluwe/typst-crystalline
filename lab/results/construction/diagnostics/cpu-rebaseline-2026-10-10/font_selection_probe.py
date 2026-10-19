from common import *
import tempfile
x=state(__file__);source=r'''use typst_infra::world::SystemWorld;
use typst_core::{contracts::world::World,entities::{font_book::{FontVariant,FontWeight},font_list::FontList}};
use std::{collections::BTreeMap,path::PathBuf};
fn main(){let args:Vec<_>=std::env::args().collect();let world=SystemWorld::new(&args[1],"variants-1600.typ").unwrap().with_fonts_and_system(&[PathBuf::from(&args[2])]);let book=world.book();let list=FontList::single("Libertinus Serif".into());let name=&list.as_slice()[0].name;let mut counts=BTreeMap::new();let mut rows=Vec::new();let mut total=0usize;for w in 100..900{let variant=FontVariant{weight:FontWeight(w),..FontVariant::default()};let idx=book.select_pattern(name,&variant).unwrap();let font=world.font(idx).unwrap();total+=font.as_slice().len();if !counts.contains_key(&idx){let face=ttf_parser::Face::parse(font.as_slice(),0).unwrap();rows.push(serde_json::json!({"index":idx,"family":book.infos()[idx].family,"weight":book.infos()[idx].variant.weight.0,"bytes":font.as_slice().len(),"math_table":face.tables().math.is_some(),"math_variants":face.tables().math.and_then(|m|m.variants).is_some(),"variable":face.is_variable()}));}*counts.entry(idx).or_insert(0usize)+=1;}println!("{}",serde_json::json!({"requests":800,"selected":rows,"counts":counts,"copy_bytes_if_one_vec_per_request":total}));}'''
x['source']=source;x['libraries']={}
with tempfile.TemporaryDirectory(prefix='cpu-selected-fonts-')as temp:
 P=Path(temp);(P/'main.rs').write_text(source);cmd=['rustc','--edition=2021','-O',str(P/'main.rs'),'-L','dependency='+str(R/'target/release/deps'),'-o',str(P/'probe')]
 for name in ['typst_infra','typst_core','ttf_parser','serde_json']:
  lib=max((R/'target/release/deps').glob('lib'+name+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns);cmd+=['--extern',name+'='+str(lib)];x['libraries'][str(lib)]=sha(lib)
 p=run(cmd);x['build']=cmd;x['build_stderr']=p.stderr;x['binary_sha256']=sha(P/'probe');cmd=[str(P/'probe'),str(T),str(FONT)];p=run(cmd);x['command']=cmd;x['result']=json.loads(p.stdout)
save('font-selection-probe.json.gz',x);print(x['result'])
