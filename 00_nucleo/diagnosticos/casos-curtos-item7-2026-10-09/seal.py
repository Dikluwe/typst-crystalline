"""Bundle ignored local fixtures and status snapshots into one durable receipt."""
import gzip,hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
status_names=('measurement-git-status.txt','measurement-git-diff-stat.txt',
              'qualification-git-status.txt','qualification-git-diff-stat.txt',
              'phases-git-status.txt','phases-git-diff-stat.txt',
              'raster-git-status.txt','raster-git-diff-stat.txt')
texts={name:(D/name).read_text() for name in status_names}
fixtures={p.name:{'text':p.read_text(),'sha256':sha(p)} for p in sorted((D/'fixtures').glob('*.typ'))}
assert len(fixtures)==9
measurement=json.load(gzip.open(D/'measurements.json.gz','rt'))
qualification=json.load(gzip.open(D/'qualification.json.gz','rt'))
phase=json.loads((D/'tekt-phases.json').read_text())
raster=json.loads((D/'raster-diff.json').read_text())
for label,state,prefix in [('measurement',measurement,'measurement'),('qualification',qualification,'qualification'),('phase',phase,'phases'),('raster',raster,'raster')]:
 assert state['git_status_sha256']==hashlib.sha256(texts[prefix+'-git-status.txt'].encode()).hexdigest(),label
 assert state['git_diff_stat_sha256']==hashlib.sha256(texts[prefix+'-git-diff-stat.txt'].encode()).hexdigest(),label
assert measurement['binaries']==qualification['binaries']==raster['binaries']
assert measurement['binaries']['tekt']==phase['binary_sha256']
assert len(measurement['samples'])==324 and len(phase['samples'])==27
scripts={p.name:sha(p) for p in sorted(D.glob('*.py'))}
receipt={'purpose':'item 7 scaled cases; ignored local source/status artifacts bundled for reproduction',
         'measurement_sha256':sha(D/'measurements.json.gz'),
         'qualification_sha256':sha(D/'qualification.json.gz'),
         'scripts_sha256':scripts,'statuses_and_diff_stats':texts,'fixtures':fixtures,
         'dry_run':json.loads((D/'dry-run.json').read_text()),
         'tekt_phases':phase,'raster_diff':raster}
with gzip.open(D/'evidence.json.gz','wt') as f:json.dump(receipt,f,ensure_ascii=False,indent=2)
print('sealed',sha(D/'evidence.json.gz'),len(fixtures),'fixtures',len(measurement['samples']),'samples')
