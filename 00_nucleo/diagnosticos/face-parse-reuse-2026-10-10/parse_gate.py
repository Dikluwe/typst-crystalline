from pathlib import Path
import gzip,json,hashlib,sys,datetime
D=Path(__file__).resolve().parent;mode=sys.argv[1];file=('prototype' if mode=='red' else 'current')+'-counted-probe.json.gz';data=json.load(gzip.open(D/file,'rt'));count=data['result']['parse_counts']['before' if mode=='red' else 'after']
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'mode':mode,'source_receipt':file,'source_receipt_sha256':hashlib.sha256((D/file).read_bytes()).hexdigest(),'head':data['head'],'working_tree':data['working_tree'],'git_status':data['git_status'],'git_diff_stat':data['git_diff_stat'],'parse_calls':count,'limit':1,'passed':count==1}
(D/('parse-gate-'+mode+'.json')).write_text(json.dumps(x,indent=2));print(mode,'parse calls',count,'passed',x['passed']);sys.exit(0 if x['passed'] else 1)
