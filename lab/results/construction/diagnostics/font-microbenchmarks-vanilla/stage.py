#!/usr/bin/env python3
"""Stage byte-identical pinned vanilla sources outside nested workspace ancestry."""
import hashlib,json,pathlib,shutil
P=pathlib.Path(__file__).resolve().parent;ROOT=P.parents[2]
record=json.loads((ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment/vanilla-provenance.json').read_text())
assert record['pin']=='9f2b6e8715237cb086899a42873660fe744622e8'
D=pathlib.Path('/tmp/tekt-font-vanilla-pin-9f2b6e871');D.mkdir(exist_ok=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for name,digest in record['source_files'].items():
 source=ROOT/'lab/typst-original'/name;assert sha(source)==digest,name
 target=D/name;target.parent.mkdir(parents=True,exist_ok=True)
 if not target.exists() or sha(target)!=digest:shutil.copy2(source,target)
 assert sha(target)==digest
manifest=P/'Cargo.toml';s=manifest.read_text()
s=s.replace('../../../lab/typst-original/crates/typst-library',str(D/'crates/typst-library')).replace('../../../lab/typst-original/crates/typst-kit',str(D/'crates/typst-kit'))
manifest.write_text(s)
(P/'staged-provenance.json').write_text(json.dumps({'pin':record['pin'],'tree':record['tree'],'staged_path':str(D),'source_files':record['source_files'],'upstream_source_differences_already_recorded':record['source_differences'],'staged_bytes_equal_ratified_lab_bytes':True,'reason':'Cargo resolved workspace inheritance through outer repository for direct nested path dependencies; staged copy preserves all original manifest/code bytes.'},indent=2)+'\n')
print('Pinned source bytes verified and staged; lab and root manifests unchanged.')
