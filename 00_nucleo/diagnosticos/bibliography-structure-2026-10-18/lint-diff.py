from pathlib import Path
import gzip,json,collections,datetime,sys
p=Path(__file__).resolve().parent
input_file=sys.argv[1] if len(sys.argv)>1 else 'complete-architecture.log'
before=json.loads(gzip.decompress((p/'lint-before.json.gz').read_bytes()))['sarif'];after=json.loads((p/input_file).read_text())
def rows(d):return [r for run in d['runs'] for r in run.get('results',[])]
def key(r):return(r['ruleId'],r.get('level'),r['locations'][0]['physicalLocation']['artifactLocation']['uri'],r['message']['text'])
b=collections.Counter(key(r) for r in rows(before));a=collections.Counter(key(r) for r in rows(after));new=a-b;removed=b-a
result={'input_file':input_file,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'Compare rule/level/consumer/message with multiplicity; line positions ignored because edits move lines. Baseline from completed A2 boundaries/types; exact source freeze preserved in lint-before.json.gz.','before':dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rows(before))),'after':dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rows(after))),'added':[{'rule':k[0],'level':k[1],'file':k[2],'message':k[3],'count':v} for k,v in sorted(new.items())],'removed':[{'rule':k[0],'level':k[1],'file':k[2],'message':k[3],'count':v} for k,v in sorted(removed.items())]}
(p/'lint-diff.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2,ensure_ascii=False).encode()))
print(json.dumps(result,indent=2,ensure_ascii=False))
assert not [x for x in result['added'] if x['level'] in ['error','warning']]
