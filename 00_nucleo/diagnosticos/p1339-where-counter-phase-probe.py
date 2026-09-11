"""Exploratory phase evidence, not a sealed or independent oracle."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

cases = {
    'filtered_func_between': '#set heading(numbering: "1")\n= A\n#counter(heading.where()).update(n => n + 10)\n= B\n#context metadata((counter(heading.where()).get(), counter(heading.where()).final(), counter(heading).get()))',
    'filtered_func_then_set': '#set heading(numbering: "1")\n#counter(heading.where()).update(n => n + 10)\n#counter(heading.where()).update(4)\n= A\n#context metadata(counter(heading.where()).get())',
    'filtered_first_context_assert': '#set heading(numbering: "1")\n= A\n#counter(heading.where()).update(n => n + 10)\n= B\n#context { assert(counter(heading.where()).get() == (12,)); metadata("passed") }',
    'bare_first_context_assert': '#set heading(numbering: "1")\n= A\n#counter(heading).update(n => n + 10)\n= B\n#context { assert(counter(heading).get() == (12,)); metadata("passed") }',
    'context_generated_filtered_update': '#set heading(numbering: "1")\n= A\n#context counter(heading.where()).update(n => n + 10)\n= B\n#context metadata(counter(heading.where()).get())',
    'filtered_error_priority': '#counter(heading.where()).update(n => panic("callback-first"))\n#context panic("context-first")',
}
bins = {
    'vanilla': ('/usr/local/bin/typst', '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'),
    'crystalline': ('/tmp/p1338-target.vlNAmp/release/typst', 'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'),
}
result = dict(start=now(), head=git('rev-parse', 'HEAD').strip(),
              diff_stat=git('diff', 'HEAD', '--stat'), status=git('status', '--short'),
              role='L0 author exploratory measurement; not independent oracle',
              regime='executed without isolation attestation',
              baseline='vanilla upstream a51e02804; crystalline certified P1338',
              runner_sha256=sha(__file__), cases=cases, runs=[])
for name, (binary, expected) in bins.items():
    assert sha(binary) == expected, binary
    for case, source in cases.items():
        cmd = [binary, 'query', '-', 'metadata', '--field', 'value', '--format', 'json']
        begin = now()
        proc = subprocess.run(cmd, cwd=ROOT, input=source, text=True,
                              capture_output=True, timeout=30)
        result['runs'].append(dict(binary=name, path=binary, sha256=expected,
                                   case=case, command=cmd, cwd=str(ROOT),
                                   source=source, start=begin, end=now(),
                                   exit_code=proc.returncode, stdout=proc.stdout,
                                   stderr=proc.stderr))
result['end'] = now()
result['status_after'] = git('status', '--short')
print(json.dumps(result, ensure_ascii=False, indent=2))
