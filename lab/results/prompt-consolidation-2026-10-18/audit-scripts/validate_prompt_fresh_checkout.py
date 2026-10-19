from pathlib import Path
import subprocess,json,datetime
root=Path('/repos/Antigravity/typst-crystalline'); checkout=root/'lab/scratch/staged-layout-check'; out=root/'lab/results/prompt-consolidation-2026-10-18'
prior=json.loads((root/'lab/results/prompt-layout-migration-2026-10-18/fresh-checkout.json').read_text())['tree']
tree=subprocess.check_output(['git','write-tree'],cwd=root,text=True).strip()
changes=subprocess.check_output(['git','diff','--name-only','-z',prior,tree],cwd=root).decode().split('\0'); changes=[x for x in changes if x]
deleted=subprocess.check_output(['git','diff','--name-only','--diff-filter=D','-z',prior,tree],cwd=root).decode().split('\0'); deleted=[x for x in deleted if x]
for rel in deleted:
 p=checkout/rel
 if p.is_file() or p.is_symlink():p.unlink()
added=[x for x in changes if x not in deleted]
# Each batch is an archive of the exact staged tree, never a working-file copy.
for offset in range(0,len(added),300):
 archive=subprocess.Popen(['git','archive','--format=tar',tree,'--',*added[offset:offset+300]],cwd=root,stdout=subprocess.PIPE)
 extract=subprocess.run(['tar','-xf','-','-C',str(checkout)],stdin=archive.stdout)
 archive.stdout.close()
 if archive.wait() or extract.returncode:raise RuntimeError('archive extraction failed')
records=[]
commands=[['python3','lab/tools/ci/check_mature_layout.py'],['python3','lab/tools/ci/check_reverse_lineage.py'],[str(root/'lab/target/linter/debug/crystalline-lint'),'.','--checks','v1,v5,v15,v26','--fail-on','warning'],['cargo','metadata','--offline','--locked','--no-deps','--format-version','1']]
for number,cmd in enumerate(commands):
 with (out/f'fresh-checkout-{number}.log').open('w') as log: result=subprocess.run(cmd,cwd=checkout,stdout=log,stderr=subprocess.STDOUT)
 records.append({'command':cmd,'exit':result.returncode,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()})
receipt={'tree':tree,'prior_archived_tree':prior,'method':'Existing isolated Git archive refreshed with changed tracked paths from exact staged tree; tracked deletions removed. No ignored working-tree files copied.','changed_files':len(changes),'deleted_files':len(deleted),'commands':records}
(out/'fresh-checkout.json').write_text(json.dumps(receipt,indent=2)+'\n'); print(json.dumps(receipt,indent=2)); raise SystemExit(any(x['exit'] for x in records))
