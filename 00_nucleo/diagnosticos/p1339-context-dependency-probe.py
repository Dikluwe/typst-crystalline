"""Bilateral compile assertions; exploratory, not a sealed test suite.

Prints the complete receipt. Temporary fixture creation uses apply_patch.
"""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
BINS = {
    'vanilla': ('/usr/local/bin/typst', '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'),
    'crystalline': ('/tmp/p1338-target.vlNAmp/release/typst', 'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'),
}
CASES = {
    'plain_set_control': '#let c = counter("p1339-context")\n#c.update(12)\n#context assert(c.get() == (12,))\nOK',
    'context_set_forward': '#let c = counter("p1339-context")\n#context c.update(12)\n#context assert(c.get() == (12,))\nOK',
    'context_set_final_before': '#let c = counter("p1339-context")\n#context assert(c.final() == (12,))\n#context c.update(12)\nOK',
    'context_filtered_set': '#let c = counter(heading.where(level: 1))\n#set heading(numbering: "1")\n= A\n#context c.update(11)\n= B\n#context assert(c.get() == (12,))',
    'context_filtered_func': '#let c = counter(heading.where(level: 1))\n#set heading(numbering: "1")\n= A\n#context c.update(n => n + 10)\n= B\n#context assert(c.get() == (12,))',
    'stable_error_control': '#let c = counter("p1339-context")\n#context c.update(12)\n#context { assert(c.get() == (12,)); panic("stable-error") }',
}

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def main():
    directory = Path(tempfile.mkdtemp(prefix='p1339-context-dependency.', dir='/tmp'))
    patch = '*** Begin Patch\n'
    for case, source in CASES.items():
        patch += f'*** Add File: {directory / (case + ".typ")}\n'
        patch += ''.join('+' + line + '\n' for line in source.splitlines())
    patch += '*** End Patch\n'
    applied = subprocess.run(['apply_patch', patch], cwd=ROOT, text=True, capture_output=True, check=True)
    result = dict(start=now(), head=git('rev-parse', 'HEAD').strip(),
                  diff_stat=git('diff', 'HEAD', '--stat'), status=git('status', '--short'),
                  runner_sha256=sha(__file__), fixtures=str(directory),
                  fixture_creation=dict(tool='apply_patch', stdout=applied.stdout, stderr=applied.stderr),
                  vanilla_upstream='a51e02804', cases=CASES, runs=[],
                  classification='exploratory compile assertions, no contract/seal/GREEN claim',
                  author='/root; inspected crystalline source, not an independent oracle')
    for name, (binary, expected) in BINS.items():
        assert sha(binary) == expected
        for case, source in CASES.items():
            argv = [binary, 'compile', str(directory / (case + '.typ')), str(directory / (name + '-' + case + '.pdf'))]
            start = now()
            proc = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True, timeout=45)
            result['runs'].append(dict(binary=name, binary_sha256=expected, case=case,
                                      source_sha256=hashlib.sha256((source + '\n').encode()).hexdigest(),
                                      argv=argv, cwd=str(ROOT), start=start, end=now(),
                                      exit=proc.returncode, stdout=proc.stdout, stderr=proc.stderr))
    result.update(end=now(), diff_stat_after=git('diff', 'HEAD', '--stat'), status_after=git('status', '--short'))
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
