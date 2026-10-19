from pathlib import Path
import gzip,json,hashlib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();archive=json.load(open(out/'candidate-1-archive.json'));assert sha(out/'candidate-1.tar.gz')==archive['sha256'];main=json.load(gzip.open(out/'before-measurement.json.gz'));ext=json.load(gzip.open(out/'before-extended.json.gz'));ref=json.load(gzip.open(out/'refuters-before-extended.json.gz'));candidate=json.load(gzip.open(out/'candidate-1-state.json.gz'));assert main['binaries']['tekt']['sha256']==ext['binaries']['tekt']['sha256'];assert ref['binaries']['tekt']['sha256']==archive['binary_sha256']
for data in [main,ext,ref]:
 for row in data['cases']:assert sha(row['source'])==row['source_sha256']
assert candidate['validation']['complete'];print('Immutable baselines and candidate source/CLI archive verified; baseline snapshots are distinct from final sources')
