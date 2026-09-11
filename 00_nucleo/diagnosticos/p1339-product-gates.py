"""Capture a product gate's complete channels and exact working-tree provenance."""
import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT=Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def state():
    def git(*args):
        return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
    return dict(at=now(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat'),
      modified_sha256={p:sha(ROOT/p) for p in git('diff','HEAD','--name-only').splitlines()},
      untracked_product_sha256={p:sha(ROOT/p) for p in git('ls-files','--others','--exclude-standard','--','01_core','02_shell','03_infra','04_wiring','tests').splitlines()})
def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--target-dir',type=Path,default=Path('/tmp/p1339-target.UD8gh7'),
                        help='Actual Cargo target directory; use a separate task-specific directory for instrumented tests')
    parser.add_argument('receipt',help='New diagnostic filename beginning p1339-')
    parser.add_argument('argv',nargs=argparse.REMAINDER)
    args=parser.parse_args()
    assert args.receipt.startswith('p1339-') and args.receipt.endswith('.json') and Path(args.receipt).name==args.receipt
    target=D/args.receipt
    assert not target.exists(), 'Do not overwrite evidence; name a successor'
    argv=args.argv[1:] if args.argv and args.argv[0]=='--' else args.argv
    assert argv and argv[0] in ['cargo','crystalline-lint','git'], 'Product gates only'
    before=state();at=now();tick=time.monotonic()
    assert args.target_dir.is_absolute(), 'Record an absolute Cargo target directory'
    env=dict(os.environ,CARGO_TARGET_DIR=str(args.target_dir))
    build_environment={key:env[key] for key in (
        'CARGO_TARGET_DIR','CARGO_ENCODED_RUSTFLAGS','RUSTFLAGS','RUSTDOCFLAGS',
        'RUSTC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','CARGO_BUILD_TARGET',
        'CARGO_BUILD_RUSTFLAGS','CARGO_INCREMENTAL','TYPST_COMMIT_SHA'
    ) if key in env}
    timed_out=False
    try:
        p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,timeout=3600)
        stdout,stderr,returncode=p.stdout,p.stderr,p.returncode
    except subprocess.TimeoutExpired as error:
        timed_out=True
        stdout,stderr,returncode=error.stdout or b'',error.stderr or b'',None
    after=state()
    result=dict(argv=argv,cwd=str(ROOT),start=at,end=now(),seconds=time.monotonic()-tick,
       environment_override=dict(CARGO_TARGET_DIR=env['CARGO_TARGET_DIR']),exit=returncode,timed_out=timed_out,timeout_seconds=3600,
       build_environment=build_environment,
       stdout=stdout.decode('utf-8',errors='replace'),stderr=stderr.decode('utf-8',errors='replace'),
       stdout_base64=base64.b64encode(stdout).decode(),stderr_base64=base64.b64encode(stderr).decode(),
       before=before,after=after,source_unchanged=before['head']==after['head'] and before['modified_sha256']==after['modified_sha256'] and before['untracked_product_sha256']==after['untracked_product_sha256'],
       source_identity_scope='HEAD, modified tracked files and untracked files in 01_core/02_shell/03_infra/04_wiring/tests; protected diagnostic fixtures/harness must additionally match the independent seal',
       recorder_sha256=sha(__file__),manifest_sha256=sha(D/'p1339-authority-manifest-r2.json'),
       interpretation='Raw product gate execution only; no independent verdict assigned')
    body=json.dumps(result,ensure_ascii=False,indent=2)
    patch='*** Begin Patch\n*** Add File: '+str(target)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],cwd=ROOT,input=patch,text=True,check=True)
    print(json.dumps(dict(receipt_sha256=sha(target),exit=returncode,timed_out=timed_out,source_unchanged=result['source_unchanged'])))
if __name__=='__main__':main()
