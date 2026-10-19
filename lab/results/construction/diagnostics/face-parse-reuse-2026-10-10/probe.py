from pathlib import Path
import gzip,json,hashlib,subprocess,datetime,tempfile,os,statistics
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def transform(s):
 old='pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {\n    let face = ttf_parser::Face::parse(data, index).ok()?;'
 new='''pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;
    font_info_from_face(&face)
}

pub(crate) fn font_info_with_coverage(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;
    let mut info = font_info_from_face(&face)?;
    info.coverage = extract_coverage(&face);
    Some(info)
}

fn font_info_from_face(face: &ttf_parser::Face) -> Option<FontInfo> {'''
 assert old in s;return s.replace(old,new,1)
if __name__=='__main__':
 before=json.load(gzip.open(D/'before.json.gz','rt'));mode=os.environ.get('FACE_PROBE_MODE','prototype'); counted=os.environ.get('FACE_PROBE_COUNT')=='1'
 base=before['owners']['03_infra/src/fonts.rs'];candidate=transform(base) if mode=='prototype' else (R/'03_infra/src/fonts.rs').read_text()
 def fragment(s):return 'use typst_core::entities::font_book::{Coverage,FontInfo,FontWeight,FontFlags,FontStyle,FontVariant,FontStretch};\n#[path="'+str(R/'03_infra/src/fonts/coverage.rs')+'"] mod coverage;\n'+s[s.index('pub fn font_info_from_bytes'):s.index('pub fn pair_slots_with_book')]
 old=fragment(base)+'''pub fn acquire(b:&[u8],i:u32)->Option<FontInfo>{let mut info=font_info_from_bytes(b,i)?;let face=ttf_parser::Face::parse(b,i).ok()?;info.coverage=extract_coverage(&face);Some(info)}'''
 new=fragment(candidate)+'''pub fn acquire(b:&[u8],i:u32)->Option<FontInfo>{font_info_with_coverage(b,i)}'''
 prior=json.load(gzip.open(R/'00_nucleo/diagnosticos/cmap-payload-reuse-2026-10-10/current-probe.json.gz','rt'));inputs=[(r['path'],r['index']) for r in prior['result']['faces']]
 state={'utc_start':utc(),'head':before['head'],'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'script_text':Path(__file__).read_text(),'script_sha256':sha(__file__),'mode':mode,'fonts':{p:sha(p) for p,i in inputs},'sources':{},'libraries':{}}
 with tempfile.TemporaryDirectory(prefix='face-parse-') as temp:
  T=Path(temp);(T/'inputs.json').write_text(json.dumps(inputs))
  main='''mod old;mod new;use std::{hint::black_box,time::Instant};fn main(){let inputs:Vec<(String,u32)>=serde_json::from_str(include_str!("inputs.json")).unwrap();let inputs:Vec<_>=inputs.into_iter().map(|(p,i)|{let b=std::fs::read(&p).unwrap();(p,i,b)}).collect();for(p,i,b)in &inputs{assert_eq!(old::acquire(b,*i),new::acquire(b,*i),"{} {}",p,i);assert_eq!(old::font_info_from_bytes(b,*i),new::font_info_from_bytes(b,*i));assert_eq!(old::acquire(b,u32::MAX),new::acquire(b,u32::MAX));}for b in [b"".as_slice(),b"not a font".as_slice()]{assert_eq!(old::acquire(b,0),new::acquire(b,0));}let mut samples=Vec::new();for round in 0..3{for pair in 0..8{for after in if(round+pair)%2==0{[false,true]}else{[true,false]}{let start=Instant::now();for _ in 0..10{for(_,i,b)in &inputs{black_box(if after{new::acquire(black_box(b),*i)}else{old::acquire(black_box(b),*i)});}}samples.push(serde_json::json!({"round":round,"pair":pair,"engine":if after{"after"}else{"before"},"ms":start.elapsed().as_secs_f64()*100.0}));}}}println!("{}",serde_json::json!({"faces":inputs.len(),"equal":true,"samples":samples}));}'''
  if counted:
   counter='\nstatic PARSES:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0); pub fn count()->usize{PARSES.swap(0,std::sync::atomic::Ordering::Relaxed)} fn parse(b:&[u8],i:u32)->Result<ttf_parser::Face,ttf_parser::FaceParsingError>{PARSES.fetch_add(1,std::sync::atomic::Ordering::Relaxed);ttf_parser::Face::parse(b,i)}'
   old=old.replace('ttf_parser::Face::parse','parse')+counter;new=new.replace('ttf_parser::Face::parse','parse')+counter
   main=main.replace('let mut samples=Vec::new();','old::count();new::count();let(_,i,b)=inputs.iter().find(|(_,i,b)|old::acquire(b,*i).is_some()).unwrap();old::count();black_box(old::acquire(b,*i));black_box(new::acquire(b,*i));println!("{}",serde_json::json!({"faces":inputs.len(),"equal":true,"samples":[],"parse_counts":{"before":old::count(),"after":new::count()}}));return;let mut samples=Vec::new();')
  for n,s in [('old.rs',old),('new.rs',new),('main.rs',main)]:
   (T/n).write_text(s);state['sources'][n]={'text':s,'sha256':sha(T/n)}
  state['inputs']=inputs;cmd=['rustc','--edition=2021','-O',str(T/'main.rs'),'-L','dependency='+str(R/'target/release/deps'),'-o',str(T/'probe')]
  for n in ['typst_core','ttf_parser','serde_json']:
   p=max((R/'target/release/deps').glob('lib'+n+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns);cmd+=['--extern',n+'='+str(p)];state['libraries'][str(p)]=sha(p)
  state['build']=cmd;state['rustc']=subprocess.check_output(['rustc','--version','--verbose'],text=True);p=subprocess.run(cmd,capture_output=True,text=True,cwd=R);state['build_stderr']=p.stderr;assert p.returncode==0,p.stderr
  state['binary_sha256']=sha(T/'probe');state['cpu']=min(os.sched_getaffinity(0));state['command']=[str(T/'probe')];p=subprocess.run(state['command'],cwd=R,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{state['cpu']}));assert p.returncode==0,p.stderr;state['result']=json.loads(p.stdout)
  assert all(sha(p)==h for p,h in state['fonts'].items());state['utc_end']=utc()
 with gzip.open(D/(mode+('-counted' if counted else '')+'-probe.json.gz'),'wt')as f:json.dump(state,f)
 print('faces',state['result']['faces'],'equal',state['result']['equal'])
 for e in (['before','after'] if state['result']['samples'] else []):print(e,statistics.median(s['ms'] for s in state['result']['samples']if s['engine']==e))

 if counted: print('parse counts',state['result']['parse_counts'])
