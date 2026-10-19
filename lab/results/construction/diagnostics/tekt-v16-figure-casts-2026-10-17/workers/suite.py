from pathlib import Path
import subprocess,os,json,gzip,datetime,hashlib,tempfile,sys,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17/workspace';out.mkdir(exist_ok=False)
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(cmd):
 r=subprocess.run(cmd,capture_output=True,text=True);return dict(command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
with tempfile.TemporaryDirectory(prefix='tekt-infra-ci-',dir='/dev/shm',ignore_cleanup_errors=True) as temp:
 env=os.environ.copy();env.update(TMPDIR=temp,TYPST_VANILLA=str(root/'lab/typst-original/target/release/typst'),CARGO_BUILD_JOBS='1');env['PATH']='/tmp/tekt-linter-none-target/debug:'+env['PATH']
 assert sha(env['TYPST_VANILLA'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
 cmd=['cargo','test','--offline','--locked','--workspace','--no-fail-fast','--','--test-threads=1']
 record=dict(start=now(),head=run(['git','rev-parse','HEAD']),status=run(['git','status','--porcelain=v1']),diffstat=run(['git','diff','HEAD','--stat']),rustc=run(['rustc','-Vv']),environment={n:env[n] for n in ['TMPDIR','TYPST_VANILLA','CARGO_BUILD_JOBS','PATH','TEKT_FIGURE_CAST_MEMORY_LIMIT']},path_prefix='/tmp/tekt-linter-none-target/debug',command=cmd,files=files)
 with gzip.open(out/'workspace-pre.json.gz','wt') as f:json.dump(record,f)
 with (out/'workspace.log').open('w') as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
 summaries=[dict(status=m[0],passed=int(m[1]),failed=int(m[2]),ignored=int(m[3])) for m in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (out/'workspace.log').read_text(),re.M)]
 result=dict(start=record['start'],end=now(),command=cmd,exit=r.returncode,environment=record['environment'],sources_unchanged=all(sha(n)==v for n,v in files.items()),summaries=summaries,passed=sum(x['passed'] for x in summaries),failed=sum(x['failed'] for x in summaries),ignored=sum(x['ignored'] for x in summaries))
 (out/'workspace-result.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2),flush=True);assert result['sources_unchanged'];sys.exit(r.returncode)
