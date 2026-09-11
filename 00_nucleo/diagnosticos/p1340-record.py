"""Record exact P1340 working-tree provenance and lossless command output."""
import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def state():
    def git(*args):
        return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
    modified = git('diff', 'HEAD', '--name-only').splitlines()
    new = git('ls-files', '--others', '--exclude-standard', '--',
              '01_core', '02_shell', '03_infra', '04_wiring', '00_nucleo/prompts').splitlines()
    return dict(utc=now(), head=git('rev-parse', 'HEAD').strip(),
                status=git('status', '--short'), diff_stat=git('diff', 'HEAD', '--stat'),
                modified={p: sha(ROOT/p) for p in modified},
                new_sources={p: sha(ROOT/p) for p in new})

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--freeze', action='store_true')
    parser.add_argument('receipt')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    assert Path(args.receipt).name == args.receipt and args.receipt.startswith('p1340-')
    target = D / args.receipt
    assert not target.exists(), 'Never overwrite evidence'
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    before = state()
    frozen = None
    if args.freeze:
        frozen = dict(diff_binary=subprocess.check_output(
            ['git', 'diff', 'HEAD', '--binary'], cwd=ROOT, text=True),
            new_sources_base64={p: base64.b64encode((ROOT/p).read_bytes()).decode()
                                for p in before['new_sources']})
    started = now()
    tick = time.monotonic()
    result = subprocess.run(command, cwd=ROOT, capture_output=True, timeout=3600)
    ended = now()
    after = state()
    record = dict(before=before, after=after, command=command, cwd=str(ROOT),
        start=started, end=ended, seconds=time.monotonic()-tick, exit=result.returncode,
        stdout=result.stdout.decode('utf-8', errors='replace'),
        stderr=result.stderr.decode('utf-8', errors='replace'),
        stdout_base64=base64.b64encode(result.stdout).decode(),
        stderr_base64=base64.b64encode(result.stderr).decode(),
        environment={k: v for k, v in os.environ.items() if k.startswith(('CARGO_', 'RUST'))},
        recorder_sha256=sha(__file__), frozen=frozen,
        source_unchanged=all(before[k] == after[k] for k in ['head','modified','new_sources']),
        independent_verdict=False)
    body = json.dumps(record, ensure_ascii=False, indent=2)
    patch = '*** Begin Patch\n*** Add File: '+str(target)+'\n'+''.join(
        '+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'], cwd=ROOT, input=patch, text=True, check=True,
                   capture_output=True)
    print(json.dumps(dict(receipt=str(target), sha256=sha(target), exit=result.returncode,
                          source_unchanged=record['source_unchanged'])))

if __name__ == '__main__':
    main()
