//! Diagnostic only: no production changes. Discovery-based validation candidate.
use std::{path::{Path,PathBuf},os::unix::fs::MetadataExt,hint::black_box,time::Instant};
#[derive(Clone,Debug,PartialEq,Eq)]
struct Stamp {path:PathBuf,index:u32,dev:u64,ino:u64,len:u64,mode:u32,mtime:(i64,i64),ctime:(i64,i64)}
fn stamp(path:&Path,index:u32)->Option<Stamp>{let m=std::fs::metadata(path).ok()?;if !m.is_file(){return None}Some(Stamp{path:path.into(),index,dev:m.dev(),ino:m.ino(),len:m.len(),mode:m.mode(),mtime:(m.mtime(),m.mtime_nsec()),ctime:(m.ctime(),m.ctime_nsec())})}
fn scan(host:bool,dir:&Path,project:&[PathBuf])->Option<Vec<Stamp>>{
 let mut db=fontdb::Database::new();if host{db.load_system_fonts()}else{db.load_fonts_dir(dir)}
 let mut result=Vec::new();
 for f in db.faces(){match &f.source{fontdb::Source::File(p)=>result.push(stamp(p,f.index)?),fontdb::Source::SharedFile(..)=>return None,fontdb::Source::Binary(_)=>{}}}
 for slot in typst_infra::fonts::discover_fonts(project){result.push(stamp(&slot.path,slot.index)?)}
 for old in &result { if stamp(&old.path,old.index).as_ref()!=Some(old){return None} }
 Some(result)
}
fn control(){let root=std::env::args().nth(2).unwrap();let root=PathBuf::from(root);std::fs::create_dir_all(&root).unwrap();let sys=root.join("system");let project=root.join("project");std::fs::create_dir_all(&sys).unwrap();std::fs::create_dir_all(&project).unwrap();let inputs=vec![project.clone()];let font=std::fs::read("03_infra/fixtures/fonts/NotoSans-Regular.ttf").unwrap();let a=sys.join("a.ttf");std::fs::write(&a,&font).unwrap();let mut events=Vec::new();let mut prev=scan(false,&sys,&inputs).unwrap();
 macro_rules! check{($name:expr,$action:expr)=>{{($action)();let next=scan(false,&sys,&inputs).unwrap();assert_ne!(prev,next,"{}",$name);events.push(serde_json::json!({"case":$name,"invalidated":true,"before_count":prev.len(),"after_count":next.len()}));prev=next;}}}
 check!("add-system",||{std::fs::write(sys.join("b.ttf"),&font).unwrap();});
 check!("replace-same-size-and-restore-mtime",||{use std::fs::FileTimes;let original=std::fs::metadata(&a).unwrap().modified().unwrap();std::thread::sleep(std::time::Duration::from_millis(2));let mut bytes=font.clone();let last=bytes.len()-1;bytes[last]^=1;std::fs::write(&a,bytes).unwrap();std::fs::File::options().write(true).open(&a).unwrap().set_times(FileTimes::new().set_modified(original)).unwrap();});
 check!("atomic-replace-system",||{let temp=sys.join("replacement");std::fs::write(&temp,&font).unwrap();std::fs::rename(temp,&a).unwrap();});
 check!("delete-system",||{std::fs::remove_file(sys.join("b.ttf")).unwrap();});
 check!("add-project",||{std::fs::write(project.join("a.ttf"),&font).unwrap();});
 check!("nested-add-project",||{std::fs::create_dir_all(project.join("nested")).unwrap();std::fs::write(project.join("nested/z.ttf"),&font).unwrap();});
 check!("delete-project",||{std::fs::remove_file(project.join("a.ttf")).unwrap();});
 check!("invalid-to-valid-system",||{std::fs::write(sys.join("negative.ttf"),b"invalid").unwrap();let unchanged=scan(false,&sys,&inputs).unwrap();assert_eq!(unchanged,prev);std::fs::write(sys.join("negative.ttf"),&font).unwrap();});
 check!("symlink-insert",||{let target=root.join("target-a.ttf");std::fs::write(&target,&font).unwrap();std::os::unix::fs::symlink(target,sys.join("link.ttf")).unwrap();});
 check!("symlink-retarget",||{let target=root.join("target-b.ttf");std::fs::write(&target,&font).unwrap();std::fs::remove_file(sys.join("link.ttf")).unwrap();std::os::unix::fs::symlink(target,sys.join("link.ttf")).unwrap();});
 assert!(stamp(&root.join("missing.ttf"),0).is_none());events.push(serde_json::json!({"case":"missing-stamp","declined":true}));
 let old_config=std::env::var_os("FONTCONFIG_FILE");let config=root.join("fonts.conf");
 std::fs::write(&config,format!("<fontconfig><dir>{}</dir></fontconfig>",sys.display())).unwrap();std::env::set_var("FONTCONFIG_FILE",&config);
 let config_a=scan(true,&sys,&[]).unwrap();
 std::fs::write(&config,format!("<fontconfig><dir>{}</dir></fontconfig>",project.display())).unwrap();
 let config_b=scan(true,&sys,&[]).unwrap();assert_ne!(config_a,config_b);events.push(serde_json::json!({"case":"fontconfig-dir-change","invalidated":true,"before_count":config_a.len(),"after_count":config_b.len()}));
 match old_config{Some(x)=>std::env::set_var("FONTCONFIG_FILE",x),None=>std::env::remove_var("FONTCONFIG_FILE")};
 let unchanged=scan(false,&sys,&inputs).unwrap();assert_eq!(prev,unchanged);events.push(serde_json::json!({"case":"unchanged","reuse_eligible":true}));
 println!("{}",serde_json::json!({"events":events,"font_bytes":font.len(),"note":"Validity receipt only. Not catalog retention or language acceptance."}));
}
fn main(){assert!(!cfg!(debug_assertions));if std::env::args().nth(1).as_deref()==Some("control"){control();return}
 let project=Vec::new();let expected=scan(true,Path::new("."),&project).unwrap();let full=typst_infra::fontdb::load_system_fonts();let full_count=full.0.len();assert_eq!(expected.len(),full_count);for (s,f) in expected.iter().zip(&full.0){assert_eq!((&s.path,s.index),(&f.path,f.index));}drop(full);
 let mut rows=Vec::new();for round in 0..3{for pair in 0..12{let(mut validation,mut acquisition)=(0u128,0u128);for step in 0..2{let validate=(round+pair+step)%2==0;let now=Instant::now();if validate{let v=scan(true,Path::new("."),&project).unwrap();assert_eq!(v,expected);black_box(v);validation=now.elapsed().as_nanos()}else{black_box(typst_infra::fontdb::load_system_fonts());acquisition=now.elapsed().as_nanos()}}rows.push(serde_json::json!({"round":round,"pair":pair,"validation_ns":validation,"acquisition_ns":acquisition}));}}
 println!("{}",serde_json::json!({"samples":rows,"faces":full_count,"unique_paths":expected.iter().map(|s|s.path.clone()).collect::<std::collections::HashSet<_>>().len(),"ordered_paths_indices_equal":true,"metadata_stamp_fields":["path","face index","dev","ino","len","mode","mtime-sec-nsec","ctime-sec-nsec"],"excludes_project_fonts":true}));
}
