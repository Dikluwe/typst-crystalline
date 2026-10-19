from pathlib import Path
import json,subprocess,hashlib,datetime,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();sources={p:sha(p) for p in b['hashes']};record={'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':b['head'],'source_hashes':sources,'pin':'9f2b6e8715237cb086899a42873660fe744622e8','binaries':{},'cases':[]};assert sha('lab/typst-original/target/release/typst')=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
body='#table([\n= Heading\n])\n';fixtures={'plain':body,'show-it':'#show table: it => it\n'+body,'show-hide':'#show table: it => [Replacement]\n'+body,'show-cell-it':'#show table.cell: it => it\n'+body,'show-cell-hide':'#show table.cell: it => [Replacement]\n'+body,'alias-show-it':'#show table: it => it\n#let t = table([\n= Heading\n])\n#t\n'}
for name,s in fixtures.items():(out/'source-table-fixtures'/(name+'.typ')).write_text(s)
for binary in ['lab/typst-original/target/release/typst','target/debug/typst']:
 record['binaries'][binary]=sha(binary)
 for name in fixtures:
  fixture=out/'source-table-fixtures'/(name+'.typ');cmd=[str(Path(binary).resolve()),'query',str(fixture),'heading','--field','body'];r=subprocess.run(cmd,capture_output=True,text=True);row={'name':name,'fixture':str(fixture),'fixture_sha256':sha(fixture),'binary':binary,'command':cmd,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr};record['cases'].append(row);print(json.dumps({'case':name,'binary':binary,'exit':r.returncode,'stdout':r.stdout}),flush=True)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in sources.items()));assert record['sources_unchanged'];(out/'source-table-measurement.json').write_text(json.dumps(record,indent=2)+'\n')
