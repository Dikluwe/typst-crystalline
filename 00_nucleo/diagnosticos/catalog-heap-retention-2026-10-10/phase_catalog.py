"""Sealed loader copies, actual release dependencies, explicit heap lifecycle marks."""
from pathlib import Path
import datetime,gzip,hashlib,json,os,subprocess,tempfile
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before_cmap=json.load(gzip.open(R/'00_nucleo/diagnosticos/cmap-payload-reuse-2026-10-10/before.json.gz','rt'))
early=os.environ.get('CATALOG_PROTO_EARLY_DROP','0')=='1'
state={'driver_text':Path(__file__).read_text(),'early_drop_prototype':early,'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'driver_sha256':sha(__file__),'sources':{},'libraries':{},'samples':[]}
font_probe=json.load(gzip.open(R/'00_nucleo/diagnosticos/cmap-payload-reuse-2026-10-10/current-probe.json.gz','rt'))
state['fonts_before']={p:sha(p) for p in font_probe['fonts_before']}
cmd=['cc','-shared','-fPIC','-O2','-Wall','-Wextra','-x','c',str(D/'heap_probe.c.txt'),'-o',str(D/'libheap_probe.so')]
p=subprocess.run(cmd,capture_output=True,text=True);assert p.returncode==0,p.stderr
state['interposer_build']={'command':cmd,'stderr':p.stderr,'source_sha256':sha(D/'heap_probe.c.txt'),'binary_sha256':sha(D/'libheap_probe.so')}
with tempfile.TemporaryDirectory(prefix='catalog-phases-') as tmp:
 T=Path(tmp)
 for name,source in [('old',before_cmap['owners']['03_infra/src/fonts/coverage.rs']),('new',(R/'03_infra/src/fonts/coverage.rs').read_text())]:
  source=source[:source.index('#[cfg(test)]')]
  source=source.replace('Some(ranges) => Coverage::from_ranges(ranges),','Some(ranges) => { crate::range_stat(ranges.len(), ranges.capacity()); Coverage::from_ranges(ranges) },')
  (T/(name+'.rs')).write_text(source);state['sources'][name]={'text':source,'sha256':sha(T/(name+'.rs'))}
 sel=(R/'03_infra/src/fontdb/selective.rs').read_text();sel=sel[:sel.index('#[cfg(test)]')];(T/'selective.rs').write_text(sel);state['sources']['selective']={'text':sel,'sha256':sha(T/'selective.rs')}
 catalog=(R/'03_infra/src/fontdb.rs').read_text();catalog=catalog[:catalog.index('#[cfg(test)]')].replace('pub(crate) mod generation;','').replace('mod selective;', '#[path="'+str(T/'selective.rs')+'"] mod selective;')
 (T/'catalog.rs').write_text(catalog);state['sources']['catalog']={'text':catalog,'sha256':sha(T/'catalog.rs')}
 world=(R/'03_infra/src/world/fonts.rs').read_text();combine=world[world.index('fn combine_catalog('):world.index('\nimpl SystemWorld {',world.index('fn combine_catalog('))]
 if early:
  combine=combine.replace('    slots.extend(sets.math_code_slots);','    drop(system.1);\n    slots.extend(sets.math_code_slots);')
  combine=combine.replace('    let (project_slots, project_book)', '    drop(sets.math_code_book);\n    let (project_slots, project_book)')
  combine=combine.replace('    (slots, book)', '    drop(project_book);\n    (slots, book)')
 # Marks are at stage boundaries; candidate cleanup is explicitly recorded.
 combine=combine.replace('    let mut slots = sets.text_slots;', '    phase(b"combine_enter\\0",0);\n    let mut slots = sets.text_slots;')
 combine=combine.replace('    slots.extend(sets.math_code_slots);','    phase(b"system_cloned\\0",0);\n    slots.extend(sets.math_code_slots);')
 combine=combine.replace('    let (project_slots, project_book)', '    phase(b"math_cloned\\0",0);\n    let (project_slots, project_book)')
 combine=combine.replace('    slots.extend(project_slots);','    phase(b"project_book_created\\0",0);\n    slots.extend(project_slots);')
 combine=combine.replace('    (slots, book)','    phase(b"all_cloned_before_return\\0",0);\n    (slots, book)')
 main=r'''mod old;mod new;mod catalog;
mod embedded_fonts {pub use typst_infra::embedded_fonts::*;}
mod fonts {
 pub use typst_infra::fonts::{FontSlot,font_info_from_bytes,pair_slots_with_book};
 pub fn extract_coverage(face:&ttf_parser::Face)->typst_core::entities::font_book::Coverage {
  if crate::OLD.load(std::sync::atomic::Ordering::Relaxed) {crate::old::extract(face)} else {crate::new::extract(face)}
 }
}
use fonts::FontSlot;
use typst_core::entities::font_book::FontBook;
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
static OLD:AtomicBool=AtomicBool::new(false);
static RANGE_LEN:AtomicUsize=AtomicUsize::new(0);
static RANGE_CAP:AtomicUsize=AtomicUsize::new(0);
static MAX_RANGE_CAP:AtomicUsize=AtomicUsize::new(0);
fn range_stat(len:usize,cap:usize) {RANGE_LEN.fetch_add(len,Ordering::Relaxed);RANGE_CAP.fetch_add(cap,Ordering::Relaxed);MAX_RANGE_CAP.fetch_max(cap,Ordering::Relaxed);}
extern "C" {fn heap_phase(label:*const std::ffi::c_char,trim:i32);}
fn phase(label:&[u8],trim:i32) {unsafe {heap_phase(label.as_ptr().cast(),trim);}}
COMBINE
fn audit(slots:&Vec<FontSlot>,book:&FontBook)->serde_json::Value {
 use std::hash::{Hasher,Hash};let mut hash=std::collections::hash_map::DefaultHasher::new();book.hash(&mut hash);
 let family_len:usize=book.infos().iter().map(|i|i.family.len()).sum();
 let family_cap:usize=book.infos().iter().map(|i|i.family.capacity()).sum();
 let logical_coverage_words:usize=book.infos().iter().map(|i|{
  let mut previous=None;let mut intervals=0;for point in i.coverage.iter() {if previous.map(|p:u32|p.checked_add(1)!=Some(point)).unwrap_or(true){intervals+=1;}previous=Some(point);}2*intervals
 }).sum();
 serde_json::json!({"slots":slots.len(),"slot_capacity":slots.capacity(),"infos":book.len(),"hash":hash.finish(),"family_len":family_len,"family_capacity":family_cap,"logical_coverage_words":logical_coverage_words,"slot_size":std::mem::size_of::<FontSlot>(),"info_size":std::mem::size_of::<typst_core::entities::font_book::FontInfo>()})
}
fn main() {
 let args:Vec<_>=std::env::args().collect();OLD.store(args[1]=="old",Ordering::Relaxed);
 phase(b"start\0",0);
 let sets=embedded_fonts::load_embedded_fonts();phase(b"embedded_loaded\0",0);
 let mut db=fontdb::Database::new();db.load_system_fonts();phase(b"discovery_alive\0",0);
 let system=catalog::catalog_from_database(&db);phase(b"system_book_db_alive\0",0);
 let system_audit=audit(&system.0,&system.1);
 drop(db);phase(b"system_book_db_dropped\0",0);
 let project=typst_infra::fonts::discover_fonts(&[std::path::PathBuf::from(&args[2])]);phase(b"project_discovered\0",0);
 let (slots,book)=combine_catalog(sets,system,project);phase(b"catalog_alive_temporaries_dropped\0",0);
 let final_audit=audit(&slots,&book);phase(b"catalog_alive_audit_complete\0",0);
 if args[3]=="trim" {phase(b"catalog_alive_after_trim\0",1);}
 drop(slots);drop(book);phase(b"catalog_dropped\0",0);
 if args[3]=="trim" {phase(b"catalog_dropped_after_trim\0",1);}
 println!("{}",serde_json::json!({"system":system_audit,"final":final_audit,"collected_ranges":RANGE_LEN.load(Ordering::Relaxed),"range_capacity_sum":RANGE_CAP.load(Ordering::Relaxed),"max_range_capacity":MAX_RANGE_CAP.load(Ordering::Relaxed)}));
}'''.replace('COMBINE',combine)
 (T/'main.rs').write_text(main);state['sources']['main']={'text':main,'sha256':sha(T/'main.rs')}
 cmd=['rustc','--edition=2021','-O',str(T/'main.rs'),'-L','dependency='+str(R/'target/release/deps'),'-L','native='+str(D),'-l','dylib=heap_probe','-C','link-arg=-Wl,-rpath,'+str(D),'-o',str(T/'probe')]
 for name in ['typst_core','typst_infra','fontdb','ttf_parser','serde_json']:
  lib=max((R/'target/release/deps').glob('lib'+name+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns);state['libraries'][str(lib)]=sha(lib);cmd+=['--extern',name+'='+str(lib)]
 state['build']=cmd;state['rustc']=subprocess.check_output(['rustc','--version','--verbose'],text=True)
 p=subprocess.run(cmd,cwd=R,capture_output=True,text=True);state['build_stderr']=p.stderr;assert p.returncode==0,p.stderr
 state['binary_sha256']=sha(T/'probe');env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',LC_ALL='C',TZ='UTC')
 cpu=min(os.sched_getaffinity(0));state['cpu']=cpu
 for rep in range(5):
  for mode in (['old','new'] if rep%2==0 else ['new','old']):
   for trim in ['no-trim','trim']:
    output=T/'phases.jsonl';env['HEAP_PHASE_OUT']=str(output)
    cmd=[str(T/'probe'),mode,str(R/'00_nucleo/diagnosticos/performance-baseline/fonts'),trim]
    p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));assert p.returncode==0,p.stderr
    rows=[json.loads(line) for line in output.read_text().splitlines()]
    assert all(r['dropped']==0 for r in rows)
    state['samples'].append({'rep':rep,'mode':mode,'trim':trim,'utc':utc(),'command':cmd,'audit':json.loads(p.stdout),'phases':rows,'stderr':p.stderr})
 for p,h in state['fonts_before'].items():assert sha(p)==h
 assert sha(__file__)==state['driver_sha256']
 state['utc_end']=utc()
 with gzip.open(D/os.environ.get('CATALOG_PHASE_RECEIPT', 'phase-catalog-early-drop.json.gz' if early else 'phase-catalog.json.gz'),'wt') as f:json.dump(state,f)
 import statistics
 for mode in ['old','new']:
  print(mode)
  samples=[s for s in state['samples'] if s['mode']==mode and s['trim']=='no-trim']
  print('audit',samples[0]['audit'])
  for phase in [r['phase'] for r in samples[0]['phases']]:
   rows=[next(r for r in s['phases'] if r['phase']==phase) for s in samples]
   print(phase,{k:int(statistics.median(r[k] for r in rows)) for k in ['tracked_requested_live','tracked_usable_live','arena','uordblks','fordblks','hblkhd','rss','registry_rss']})
