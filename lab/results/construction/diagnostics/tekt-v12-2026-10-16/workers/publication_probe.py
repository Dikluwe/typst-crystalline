from pathlib import Path
import json,hashlib,datetime,tempfile,subprocess,socket,http.client,time,os,sys
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v12-2026-10-16';v=json.loads((out/'validation-details.json').read_text());hashes=v['source_hashes'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(p)==h for p,h in hashes.items());assert json.loads((out/'validation-result.json').read_text())['exit']==0
binary=root/'target/debug/typst';record=dict(start=now(),head=v['head'],validation_details_sha256=sha(out/'validation-details.json'),binary=str(binary),binary_sha256=sha(binary),source_hashes=hashes,observations=[]);child=None;error=None
with tempfile.TemporaryDirectory(prefix='tekt-v12-watch-html-',dir='/dev/shm') as temp:
 folder=Path(temp);source=folder/'main.typ';output=folder/'preview.html';source.write_text('V12HTMLINITIALMARKER')
 with socket.socket() as listener:listener.bind(('127.0.0.1',0));port=listener.getsockname()[1]
 cmd=[str(binary),'watch',str(source),str(output),'--format','html','--features','html','--port',str(port)];record.update(command=cmd,temporary_directory=str(folder),port=port)
 stdout=out/'publication-html-stdout.log';stderr=out/'publication-html-stderr.log'
 def alive():
  if child.poll() is not None:raise AssertionError('watch exited: '+str(child.returncode)+' '+stderr.read_text(errors="replace"))
 def get():
  conn=http.client.HTTPConnection('127.0.0.1',port,timeout=0.5)
  try:
   conn.request('GET','/');response=conn.getresponse();return response.status,response.getheader('Content-Type'),response.read()
  except (OSError,http.client.HTTPException):return None
  finally:conn.close()
 def wait(phase,condition):
  deadline=time.monotonic()+25
  while time.monotonic()<deadline:
   alive();value=condition()
   if value:return value
   time.sleep(0.05)
  raise AssertionError('timeout '+phase+' '+stderr.read_text(errors="replace"))
 def observed(marker):
  response=get()
  if response is None or response[0]!=200 or marker not in response[2] or not output.exists():return None
  disk=output.read_bytes()
  if marker not in disk:return None
  assert response[1]=='text/html; charset=utf-8',response[1]
  assert b'EventSource' in response[2] and b'EventSource' not in disk
  return response,disk
 def save(phase,value):
  response,disk=value
  (out/('publication-'+phase+'-http.html')).write_bytes(response[2]);(out/('publication-'+phase+'-disk.html')).write_bytes(disk)
  return dict(http_body_sha256=hashlib.sha256(response[2]).hexdigest(),disk_sha256=hashlib.sha256(disk).hexdigest())
 def edit(text):
  staging=folder/'next.typ';staging.write_text(text);staging.replace(source)
 try:
  with stdout.open('wb') as so,stderr.open('wb') as se:
   child=subprocess.Popen(cmd,stdout=so,stderr=se,env=dict(os.environ,NO_COLOR='1'))
   first=wait('initial publication',lambda:observed(b'V12HTMLINITIALMARKER'));save('initial',first);record['observations'].append(dict(phase='initial',utc=now(),status=first[0][0],content_type=first[0][1],disk_sha256=hashlib.sha256(first[1]).hexdigest(),served_html_has_reload_script=True,disk_has_no_reload_script=True))
   edit('V12HTMLSECONDMARKER');second=wait('updated publication',lambda:observed(b'V12HTMLSECONDMARKER'));assert b'V12HTMLINITIALMARKER' not in second[0][2];save('update',second);record['observations'].append(dict(phase='update',utc=now(),status=second[0][0],disk_sha256=hashlib.sha256(second[1]).hexdigest()))
   offset=stderr.stat().st_size;edit('#unknownv12publicationfixture');wait('error diagnostic',lambda:all(word in stderr.read_bytes()[offset:] for word in [b'error:',b'unknownv12publicationfixture']));alive();failed=get();assert failed==second[0] and output.read_bytes()==second[1],(failed,second[0]);save('failed',(failed,output.read_bytes()));record['observations'].append(dict(phase='failed compile',utc=now(),error_observed=True,last_http_snapshot_preserved=True,last_disk_artifact_preserved=True,watch_alive=True))
   edit('V12HTMLRECOVEREDMARKER');recovered=wait('recovery',lambda:observed(b'V12HTMLRECOVEREDMARKER'));assert b'V12HTMLSECONDMARKER' not in recovered[0][2];save('recovery',recovered);record['observations'].append(dict(phase='recovery',utc=now(),status=recovered[0][0],disk_sha256=hashlib.sha256(recovered[1]).hexdigest(),watch_alive=True))
 except BaseException as ex:error=repr(ex)
 finally:
  if child is not None:
   child.kill();child.wait(timeout=10);record['cleanup_returncode']=child.returncode
record.update(end=now(),passed=error is None,error=error,sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/'publication-probe-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['start','end','passed','error','sources_unchanged']}),flush=True);assert record['sources_unchanged'] and record['passed']
