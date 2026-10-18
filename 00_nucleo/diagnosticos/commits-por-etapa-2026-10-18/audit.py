from pathlib import Path
import subprocess,json,gzip,hashlib,base64,datetime
root=Path.cwd();out=Path(__file__).resolve().parent
load=lambda name:json.loads(gzip.decompress((out/name).read_bytes()))
def git(*a):return subprocess.check_output(['git',*a],text=True)
def productive(p):return p.startswith('00_nucleo/prompts/') or p.endswith('.rs') and p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/'))
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
def tree(commit):
 rows={}
 for line in git('ls-tree','-r','--format=%(objectmode) %(objectname) %(path)',commit).splitlines():
  mode,blob,path=line.split(' ',2)
  if productive(path):rows[path]=(mode,blob)
 return rows
plan=load('commit-plan.json.gz');history=load('commits.json.gz');assert len(history['commits'])==15
expected=tree(plan['base_head']);stages={s['key']:s for s in plan['stages']};checks=[]
assert git('rev-parse','HEAD').strip()==history['commits'][-1]['commit']
for receipt in history['commits']:
 stage=stages.get(receipt['key'])
 if stage:
  for path,text in stage['source_changes'].items():
   if text is None:expected.pop(path,None);continue
   blob=hashlib.sha1(b'blob '+str(len(text.encode())).encode()+b'\0'+text.encode()).hexdigest()
   mode=plan['modes'].get(path,'100755' if (root/path).stat().st_mode&0o111 else '100644');expected[path]=(mode,blob)
 assert tree(receipt['commit'])==expected,('committed checkpoint mismatch',receipt['key'])
 z=subprocess.run(['git','-c','gpg.ssh.allowedSignersFile='+str(out/'scratch/allowed-signers'),'verify-commit',receipt['commit']],text=True,capture_output=True);assert z.returncode==0
 for white in receipt.get('archive_whitespace',[]):
  f=white['path'];folder=root/Path(f).parts[0]/Path(f).parts[1]/Path(f).parts[2];manifest=json.loads((folder/'manifest.json').read_text())
  assert sha(folder/'manifest.json') in (folder/'manifest.sha256').read_text()
  assert any(x.get('path')==f and x.get('sha256')==white['sha256'] for x in manifest['files'])
 checks.append(dict(key=receipt['key'],commit=receipt['commit'],source_tree_matches_checkpoint=True,signature_verified=True,archival_whitespace=receipt.get('archive_whitespace',[])))
 print('Verified tree/signature:',receipt['key'],flush=True)
packages=load('evidence-packages.json.gz');pack_results=[]
for record in packages['packages']:
 path=root/record['path'];assert sha(path)==record['sha256']
 package=json.loads(gzip.decompress(path.read_bytes()));assert len(package['files'])==record['files']
 for entry in package['files']:
  data=base64.b64decode(entry['base64'],validate=True)
  assert len(data)==entry['bytes'] and hashlib.sha256(data).hexdigest()==entry['sha256']
  assert (root/entry['path']).read_bytes()==data,('original evidence changed',entry['path'])
 pack_results.append(dict(path=record['path'],files=record['files'],compressed_bytes=record['compressed_bytes'],sha256=record['sha256']))
 print('Verified evidence:',record['folder'],flush=True)
assert all(sha(root/f)==h for f,h in plan['final_hashes'].items())
assert git('diff','--cached','--name-only').strip()==''
assert git('diff','--name-only').splitlines()==['README.md']
assert all(f.startswith(str(out.relative_to(root))+'/') for f in git('ls-files','--others','--exclude-standard').splitlines())
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),base_head=plan['base_head'],tip_before_delivery=git('rev-parse','HEAD').strip(),commits=checks,source_files_unchanged=len(plan['final_hashes']),packages=pack_results,evidence_files=sum(x['files'] for x in pack_results),evidence_compressed_bytes=sum(x['compressed_bytes'] for x in pack_results),all_sources_match_original_final_working_tree=True,build_or_test_rerun=False,strict_lineage_checkpoints=12)
(out/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,ensure_ascii=False,indent=2).encode()))
print(json.dumps(dict(commits=len(checks),packages=len(pack_results),evidence_files=result['evidence_files'],source_files=result['source_files_unchanged'])))
