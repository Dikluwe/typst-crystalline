from pathlib import Path
import json,gzip,hashlib,subprocess,collections,datetime
p=Path(__file__).resolve().parent;tool=Path('/tmp/tekt-linter-none-target/debug/crystalline-lint');sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest();tool_sha=sha(tool)
z=subprocess.run([str(tool),'.','--format','sarif'],text=True,capture_output=True);d=json.loads(z.stdout);before=json.loads(gzip.decompress((p/'lint-before.json.gz').read_bytes()))['sarif']
def rows(d):return [r for run in d['runs'] for r in run.get('results',[])]
def key(r):return r['ruleId'],r.get('level'),r['locations'][0]['physicalLocation']['artifactLocation']['uri'],r['message']['text']
a=collections.Counter(key(r) for r in rows(d));b=collections.Counter(key(r) for r in rows(before));new=[(k,v) for k,v in (a-b).items() if k[1] in ['error','warning']];counts=dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rows(d)))
assert sha(tool)==tool_sha
(p/'preflight-architecture.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'tool':str(tool),'tool_sha256':tool_sha,'tool_metadata':{k:d['runs'][0]['tool']['driver'].get(k) for k in ['name','version','semanticVersion']},'counts':counts,'new_errors_warnings':new,'sarif':d},indent=2).encode()))
print(counts,'new errors/warnings',len(new));assert counts.get('V16:error')==106 and not new
