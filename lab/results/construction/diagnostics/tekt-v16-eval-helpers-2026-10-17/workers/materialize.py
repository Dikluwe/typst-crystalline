from pathlib import Path
import json,gzip,subprocess,re,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));assert json.load(open(out/'baseline-result.json'))['structural_red'];assert json.load(open(out/'probe-baseline-result.json'))['selected']==0
p=Path(scope['sources'][0]);original=p.read_bytes();assert original.decode()==b['files'][str(p)];s=original
for r in sorted(scope['targets'],key=lambda r:r['pattern_start_byte'],reverse=True):
 a,z=r['pattern_start_byte'],r['pattern_end_byte'];assert s[a:z]==b'_';indent=original[original.rfind(b'\n',0,a)+1:a].decode();assert not indent.strip();replacement=('\n'+indent+'| ').join(scope['variants'][r['enum']][n] for n in r['complement']);s=s[:a]+replacement.encode()+s[z:]
s=re.sub(rb'^//! @updated [0-9-]+$',b'//! @updated 2026-10-17',s,count=1,flags=re.M);p.write_bytes(s)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',str(p)],check=True)
(out/'materialization.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'expected_after':p.read_text(),'unformatted_pattern_only':s.decode(),'source':str(p)},ensure_ascii=False,indent=2)+'\n');print('Eleven pattern replacements; bodies and assertions untouched',flush=True)
