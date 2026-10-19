#!/usr/bin/env python3
"""Preserve frozen inputs and exact executable after admitted measurement."""
import json,pathlib,shutil,sys,tarfile
P=pathlib.Path(__file__).resolve().parents[1];sys.path.insert(0,str(P));import measure as m
frozen=json.loads((P/'frozen-state.json').read_text());assert m.fingerprint()['sha256']==frozen['sha256']
assert not (P/'consumers-inputs.tar.gz').exists()
with tarfile.open(P/'consumers-inputs.tar.gz','w:gz') as archive:
 for name,digest in frozen['files'].items():
  path=pathlib.Path(name);path=path if path.is_absolute() else m.ROOT/path
  assert m.sha(path)==digest
  if path.is_relative_to(m.ROOT):arc='repository/'+str(path.relative_to(m.ROOT))
  elif path.is_relative_to(pathlib.Path('/tmp/tekt-font-vanilla-pin-9f2b6e871')):arc='vanilla-source/'+str(path.relative_to('/tmp/tekt-font-vanilla-pin-9f2b6e871'))
  else:continue # External font bytes identified by hashes, not redistributed.
  archive.add(path,arcname=arc,recursive=False)
build=json.loads((P/'build-provenance.json').read_text());(P/'bin').mkdir(exist_ok=True)
assert m.sha(build['executable'])==build['sha256'];shutil.copy2(build['executable'],P/'bin/font-vanilla-comparison')
print('Frozen repository and staged vanilla sources archived; executable copied.')
