from pathlib import Path
import gzip,json,hashlib,tempfile,os,re,tarfile,shutil,datetime
p=Path(__file__).resolve().parent;r=Path.cwd();load=lambda f:json.loads(gzip.decompress((p/f).read_bytes()));sha=lambda b:hashlib.sha256(b).hexdigest();frozen=load('grid-freeze.json.gz');base=load('before-sources.json.gz')['files']
recovered={f:Path(f).read_text() for f in frozen['sources']};prescription=json.loads((p/'prescription.json').read_text())
with tempfile.TemporaryDirectory(prefix='tekt-grid-source-recovery-') as td:
 t=Path(td)
 for rel in prescription['owners']:
  name='00_nucleo/prompts/'+rel
  if name in base:
   f=t/name;f.parent.mkdir(parents=True,exist_ok=True);f.write_text(base[name])
 (t/'00_nucleo/prompts/_nuclei/layout').mkdir(parents=True,exist_ok=True)
 os.chdir(t)
 script=(p/'prescribe.py').read_text().replace('p=Path(__file__).resolve().parent','p=Path.cwd()')
 exec(compile(script,str(p/'prescribe.py'),'exec'),{'__file__':str(p/'prescribe.py')})
 os.chdir(r)
 for rel in prescription['owners']:
  name='00_nucleo/prompts/'+rel;raw=(t/name).read_text();current=Path(name).read_text();meta=re.search(r'^Hash do Código: .*$',current,re.M).group();raw=re.sub(r'^Hash do Código: .*$',meta,raw,flags=re.M)
  lines=re.findall(r'^- 00_nucleo/prompts/_nuclei/[^\n]+ sha256:[a-f0-9]{64}$',raw,re.M);it=iter(sorted(lines));raw=re.sub(r'^- 00_nucleo/prompts/_nuclei/[^\n]+ sha256:[a-f0-9]{64}$',lambda _:next(it),raw,flags=re.M);recovered[name]=raw
 recovered['00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml']=(t/'00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml').read_text()
f='01_core/src/compiler/layout/bibliography.rs';raw=recovered[f];index=raw.index('    #[test]\n    fn a2_structure_long_entry_starts_in_remaining_region');prefix=raw[:index].rstrip()
choices=[prefix+'\n}\n',prefix+'\n\n}\n'];recovered[f]=next(s for s in choices if sha(s.encode())==frozen['sources'][f])
assert all(sha(s.encode())==frozen['sources'][f] for f,s in recovered.items())
(p/'grid-sources.json.gz').write_bytes(gzip.compress(json.dumps({'reconstructed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'Reconstructed exact bytes from pre-stage L0s and original prescription, preserving sealed code metadata; every file verified against measurement grid-freeze hashes. No productive source was changed.','measurement_freeze':frozen,'files':recovered}).encode()))
with tarfile.open(p/'grid-artifacts.tar.gz','w:gz') as tar:tar.add(p/'outputs',arcname='outputs')
shutil.copy2(r/'target/debug/typst',p/'bin/typst-grid');assert sha((p/'bin/typst-grid').read_bytes())=='7c01fa4a9d183b371af3efcc0ea3952b4ca386969436c7fa6a37aa968c703434'
print('Grid exact sources recovered',len(recovered),'and PDF/binary archived')
