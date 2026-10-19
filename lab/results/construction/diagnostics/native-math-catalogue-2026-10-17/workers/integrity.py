from pathlib import Path
import json,gzip,re,hashlib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));cases=json.load(open(out/'batch-schema.json'))['cases'];newbridges={c['callback'] for c in cases if c['new_bridge']};strip=lambda s:re.sub(r'^//! @prompt-hash .*\n','',s,flags=re.M);flat=lambda s:re.sub(r'\s+','',strip(s));records=[]
for source in scope['sources']:
 old=b['files'][source];new=Path(source).read_text()
 if source.endswith('/value_methods/element_where.rs'):
  marker='#[cfg(test)]\nmod native_math_catalogue_tests';assert new.count(marker)==1;prod=new[:new.index(marker)];anchor='    element!(native_title,';ap=strip(old[:old.index(anchor)]);zp=strip(prod[:prod.index(anchor)]);assert zp.startswith(ap);added=zp[len(ap):];found=re.findall(r'element!\(\s*(\w+),',added);assert set(found)=={c['callback'] for c in cases} and len(found)==26;assert old[old.index(anchor):].strip()==prod[prod.index(anchor):].strip();records.append(dict(source=source,existing_production_and_tests_preserved=True,new_metadata=len(found)));continue
 if source.endswith('/math/limits.rs'):
  for name in ['native_math_scripts','native_math_limits']:new=new.replace('pub(in crate::compiler) fn '+name,'pub(super) fn '+name)
  assert strip(old)==strip(new);records.append(dict(source=source,bodies_and_signatures_preserved=True,only_two_internal_visibility_changes=True));continue
 if source.endswith('/structural/math.rs'):
  pattern=r'pub\(in crate::compiler\) use \w+::[^;]+;';blocks=re.findall(pattern,new);assert blocks;names=set()
  for block in blocks:
   for part in block.split(' use ',1)[1].split('::',1)[1].removesuffix(';').strip().strip('{}').split(','):
    if part.strip():names.add(part.strip().split(' as ')[-1])
  assert names==newbridges
 else:
  module='math' if source.endswith('/structural/mod.rs') else 'structural';pattern=r'pub\(in crate::compiler\) use '+module+r'::\{[\s\S]*?\};';blocks=re.findall(pattern,new);assert len(blocks)==1;names={n.strip() for n in blocks[0].split('{',1)[1].rsplit('}',1)[0].split(',') if n.strip()};assert names==newbridges
 new=re.sub(pattern,'',new);assert flat(old)==flat(new),source;records.append(dict(source=source,only_direct_internal_reexports=True,bridge_count=len(names)))
allowed=set(scope['sources'])|set(scope['owners'].values());unrelated=[p for p,h in b['hashes'].items() if p not in allowed and hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h];assert not unrelated;result=dict(records=records,unrelated_changes=unrelated,public_api_changes=False);(out/'integrity-result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
