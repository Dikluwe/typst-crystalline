"""Audit payload identities and compare frozen coverage with a private prototype."""
from pathlib import Path
import datetime, gzip, hashlib, json, os, subprocess, tempfile

D = Path(__file__).resolve().parent
R = D.parents[2]
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()

def prototype(source):
    source = source.replace('    let raw = face.raw_face().table(Tag::from_bytes(b"cmap"));',
                            '    let raw = face.raw_face().table(Tag::from_bytes(b"cmap"));\n    let mut seen = [None; 8];', 1)
    source = source.replace('                    native(bytes, format, &mut out)', '''                    let key = (offset, format);
                    if seen.contains(&Some(key)) {
                        return Some(());
                    }
                    native(bytes, format, &mut out)?;
                    if let Some(slot) = seen.iter_mut().find(|slot| slot.is_none()) {
                        *slot = Some(key);
                    }
                    Some(())''', 1)
    return source

if __name__ == '__main__':
    before = json.load(gzip.open(D/'before.json.gz', 'rt'))
    original = before['owners']['03_infra/src/fonts/coverage.rs']
    prefix = original[:original.index('#[cfg(test)]')]
    current = (R/'03_infra/src/fonts/coverage.rs').read_text()
    mode = os.environ.get('CMAP_PROBE_MODE', 'prototype')
    candidate = prototype(prefix) if mode == 'prototype' else current[:current.index('#[cfg(test)]')]
    roots = ['/usr/share/fonts', '/usr/local/share/fonts', '/home/dikluwe/.fonts',
             '/home/dikluwe/.local/share/fonts', '/usr/share/texmf/fonts',
             '/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts',
             str(R/'03_infra/fixtures/fonts')]
    paths = set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines())
    for root in roots:
        paths.update(str(p) for p in Path(root).rglob('*') if p.is_file() and p.suffix.lower() in ('.ttf','.otf','.ttc'))
    fonts = {str(Path(p).resolve()):sha(p) for p in paths if Path(p).is_file()}
    state = {'utc_start':utc(), 'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
             'working_tree':'uncommitted', 'git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),
             'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),
             'script_sha256':sha(__file__), 'mode':mode, 'fonts_before':fonts,
             'productive_sha256':sha(R/'03_infra/src/fonts/coverage.rs'), 'sources':{}, 'libraries':{}}
    wrapper = '\npub(super) fn ranges(face: &Face) -> Option<Vec<Range<u32>>> { collect(face) }\npub(super) fn legacy_coverage(face: &Face) -> Coverage { legacy(face) }\n'
    with tempfile.TemporaryDirectory(prefix='cmap-payload-') as temp:
        T=Path(temp)
        for label, source in [('old',prefix),('new',candidate)]:
            p=T/(label+'.rs');p.write_text(source+wrapper)
            state['sources'][label]={'text':p.read_text(),'sha256':sha(p)}
        main=r'''mod old; mod new;
use ttf_parser::{Face,Tag,cmap::Format};
use std::{collections::BTreeSet,time::Instant,hint::black_box};
fn main() {
 let mut db=fontdb::Database::new();db.load_system_fonts();
 let mut inputs=Vec::new();
 for face in db.faces() {
  if let fontdb::Source::File(path) | fontdb::Source::SharedFile(path,_) = &face.source { inputs.push((path.clone(),face.index,"system")); }
 }
 for (directory,kind) in [("/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts","embedded"),("03_infra/fixtures/fonts","fixture")] {
  for entry in std::fs::read_dir(directory).unwrap() {
   let path=entry.unwrap().path();if !path.is_file() {continue;}
   let bytes=std::fs::read(&path).unwrap();
   for index in 0..ttf_parser::fonts_in_collection(&bytes).unwrap_or(1) {
    if Face::parse(&bytes,index).is_ok() {inputs.push((path.clone(),index,kind));}
   }
  }
 }
 let owned:Vec<_>=inputs.iter().map(|(p,i,k)|(p.clone(),*i,*k,std::fs::read(p).unwrap())).collect();
 let faces:Vec<_>=owned.iter().map(|(_,i,_,b)|Face::parse(b,*i).unwrap()).collect();
 let mut rows=Vec::new();
 for ((path,index,kind,_),face) in owned.iter().zip(&faces) {
  assert_eq!(old::extract(face),new::extract(face),"coverage {:?} {}",path,index);
  assert_eq!(old::extract(face),old::legacy_coverage(face),"legacy {:?} {}",path,index);
  let mut offsets=BTreeSet::new();let mut unicode=0usize;let mut duplicates=0usize;let mut records=Vec::new();
  if let (Some(cmap),Some(raw))=(face.tables().cmap,face.raw_face().table(Tag::from_bytes(b"cmap"))) {
   for i in 0..cmap.subtables.len() {
    let Some(sub)=cmap.subtables.get(i) else {break;};if !sub.is_unicode(){continue;}
    unicode+=1;
    let format=match sub.format {Format::SegmentMappingToDeltaValues(_)=>4,Format::SegmentedCoverage(_)=>12,Format::ManyToOneRangeMappings(_)=>13,_=>0};
    let at=8+usize::from(i)*8;let offset=u32::from_be_bytes(raw[at..at+4].try_into().unwrap());
    if format!=0 && !offsets.insert((offset,format)) {duplicates+=1;}
    records.push(serde_json::json!({"record":i,"format":format,"offset":offset}));
   }
  }
  let old_len=old::ranges(face).map(|v|v.len());let new_len=new::ranges(face).map(|v|v.len());
  assert_eq!(old_len.is_some(),new_len.is_some());
  rows.push(serde_json::json!({"path":path,"index":index,"kind":kind,"unicode_records":unicode,"duplicate_native_payloads":duplicates,"unique_native_payloads":offsets.len(),"records":records,"old_ranges":old_len,"new_ranges":new_len,"coverage_equal":true}));
 }
 let mut samples=Vec::new();
 for round in 0..3 { for pair in 0..8 { for candidate in if (round+pair)%2==0 {[false,true]} else {[true,false]} {
  let start=Instant::now();let mut sink=0;
  for _ in 0..10 {for face in &faces {let coverage=if candidate {new::extract(black_box(face))} else {old::extract(black_box(face))};sink+=black_box(coverage).iter().take(1).count();}}
  black_box(sink);
  samples.push(serde_json::json!({"round":round,"pair":pair,"engine":if candidate {"after"} else {"before"},"milliseconds":start.elapsed().as_secs_f64()*1000.0/10.0}));
 }}}
 println!("{}",serde_json::json!({"faces":rows,"samples":samples}));
}'''
        (T/'main.rs').write_text(main);state['sources']['main']={'text':main,'sha256':sha(T/'main.rs')}
        cmd=['rustc','--edition=2021','-O',str(T/'main.rs'),'-L','dependency='+str(R/'target/debug/deps'),'-o',str(T/'probe')]
        for name in ['typst_core','fontdb','ttf_parser','serde_json']:
            lib=max((R/'target/debug/deps').glob('lib'+name+'-*.rlib'),key=lambda p:p.stat().st_mtime_ns)
            state['libraries'][str(lib)]=sha(lib);cmd+=['--extern',name+'='+str(lib)]
        state['build']=cmd;state['rustc']=subprocess.check_output(['rustc','--version','--verbose'],text=True)
        p=subprocess.run(cmd,cwd=R,capture_output=True,text=True);state['build_stderr']=p.stderr;assert p.returncode==0,p.stderr
        state['binary_sha256']=sha(T/'probe');state['command']=[str(T/'probe')];state['cpu']=min(os.sched_getaffinity(0))
        p=subprocess.run(state['command'],cwd=R,capture_output=True,text=True,timeout=180,preexec_fn=lambda:os.sched_setaffinity(0,{state['cpu']}))
        state['stderr']=p.stderr;assert p.returncode==0,p.stderr;state['result']=json.loads(p.stdout)
        for row in state['result']['faces']:
            p=Path(row['path']).resolve();assert sha(p)==fonts[str(p)]
        assert state['script_sha256']==sha(__file__)
        assert state['productive_sha256']==sha(R/'03_infra/src/fonts/coverage.rs')
        state['utc_end']=utc()
        with gzip.open(D/(mode+'-probe.json.gz'),'wt') as f:json.dump(state,f)
        rows=state['result']['faces']
        print('faces',len(rows),'duplicated native records',sum(r['duplicate_native_payloads'] for r in rows),
              'faces with duplicates',sum(r['duplicate_native_payloads']>0 for r in rows),
              'ranges',sum(r['old_ranges'] or 0 for r in rows),'->',sum(r['new_ranges'] or 0 for r in rows))
        import statistics
        for label in ['before','after']:print(label,statistics.median(s['milliseconds'] for s in state['result']['samples'] if s['engine']==label),'ms per corpus')
