from common import *
import gzip,signal
entries=[('typst_core::entities::source::Source::new','SOURCE_NEW'),('typst_core::compiler::eval::modules::eval_module_import','IMPORT_EVAL'),('typst_core::compiler::eval::eval_with_full_error_target_and_features','ROOT_EVAL'),('typst_infra::pipeline::context_stabilization::Session::execute','CONTEXT_EXECUTE'),('typst_core::compiler::introspect::entrypoints::introspect_with_introspector','INTROSPECT'),('typst_infra::pipeline::context_stabilization::Session::content','SESSION_CONTENT'),('typst_infra::fontdb::generation::prepare','FONT_PREPARE')]
state={'utc_start':now(),'before_sha256':sha(HERE/'before.json.gz'),'binary_sha256':sha(BINS['tekt']),'script_sha256':sha(__file__),'common_sha256':sha(HERE/'common.py'),'cases':[]}
with tempfile.TemporaryDirectory(prefix='watch-count-current-')as temp:
 base=Path(temp)
 for name in ['import','context300']:
  folder=base/name;folder.mkdir();main=folder/'main.typ';lib=folder/'lib.typ';pdf=folder/'main.pdf';log=folder/'gdb.log';script=folder/'count.gdb';body=(HERE/'fixtures'/(name+'.typ')).read_text();initial_rows=300 if name=='context300'else 8;word='Alpha';rows=initial_rows
  source=lambda:body.replace('Alpha',word).replace(f'range({initial_rows})',f'range({rows})')
  main.write_text(source());lib.write_bytes((HERE/'fixtures/lib.typ').read_bytes())
  commands='set pagination off\nset confirm off\n'
  for fn,marker in entries:commands+=f'break {fn}\ncommands\nsilent\nprintf "{marker}\\n"\ncontinue\nend\n'
  commands+='run\n';script.write_text(commands);cmd=['gdb','--batch','-x',str(script),'--args',str(BINS['tekt']),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false'];records=[]
  with log.open('w')as output:
   p=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=output,stderr=output,start_new_session=True)
   try:
    assert wait_pdf(p,pdf,None,30),'missing initial PDF under GDB'
    for stage in ['initial','metadata_touch','same_bytes','small','structural']:
     time.sleep(.2)
     if stage!='initial':
      old=generation(pdf)
      if stage=='metadata_touch':
       st=main.stat();os.utime(main,ns=(st.st_atime_ns,st.st_mtime_ns+1_000_000_000))
      else:
       if stage=='small':word='Bravo'
       if stage=='structural':rows=320 if name=='context300'else 12
       tmp=folder/'main.new';tmp.write_text(source());os.replace(tmp,main)
      assert wait_pdf(p,pdf,old,30),stage;time.sleep(.2)
     content=log.read_text();records.append({'stage':stage,'utc':now(),'source':main.read_text(),'pdf_sha256':sha(pdf),**{m:content.count(m+'\n')for _,m in entries}})
     print(name,records[-1],flush=True)
   finally:
    (HERE/f'count-{name}-latest.log.txt').write_text(log.read_text())
    if p.poll() is None:os.killpg(p.pid,signal.SIGTERM)
    try:p.wait(timeout=5)
    except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
  state['cases'].append({'name':name,'command':cmd,'gdb_script':commands,'log':log.read_text(),'records':records})
state['utc_end']=now();assert state['binary_sha256']==sha(BINS['tekt'])
with gzip.open(HERE/'counts.json.gz','wt')as f:json.dump(state,f)
