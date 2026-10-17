from pathlib import Path
import datetime,gzip,json
from rust_matches import block,arms,variants
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));file='01_core/src/compiler/stdlib/layout/casting.rs';source=Path(file).read_text();definition=variants(Path('01_core/src/entities/value.rs').read_text(),'Value');positive={'Length','Float','Int','Relative','Ratio'};rows=[]
for name in ['extract_length','extract_block_width']:
 old=arms(block(block(b['files'][file],'pub(crate) fn '+name+'(')[2],'match val')[2]);new=arms(block(block(source,'pub(crate) fn '+name+'(')[2],'match val')[2]);assert old[:-1]==new[:-1];assert old[-1][1]==new[-1][1]=='None';pattern=new[-1][0];expected={'Value::'+n+('(..)' if a else '') for n,a in definition if n not in positive};assert set(p.strip() for p in pattern.split('|'))==expected;rows.append(dict(name=name,positive_rhs_unchanged=5,explicit_negative_variants=len(expected)))
(out/'cast-integrity-result.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),results=rows),indent=2)+'\n');print(rows)
