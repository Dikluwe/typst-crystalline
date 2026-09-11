"""Vanilla-only runtime exploration. Does not produce a contract or verdict."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BINARY = '/usr/local/bin/typst'
EXPECTED = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

CASES = {
    'field_order': '''#let a = counter(heading.where(level: 1, outlined: true))
#let b = counter(heading.where(outlined: true, level: 1))
#set heading(numbering: "1")
= A
#a.update(n => n + 10)
= B
#context metadata((a == b, a.get(), b.get()))''',
    'int_float_keys': '''#let a = counter(heading.where(level: 1))
#let b = counter(heading.where(level: 1.0))
#set heading(numbering: "1")
= A
#a.update(n => n + 10)
= B
#context metadata((a == b, a.get(), b.get(), query(heading.where(level: 1.0)).len()))''',
    'nan_keys': '''#let a = counter(heading.where(level: float.nan))
#let b = counter(heading.where(level: float.nan))
#set heading(numbering: "1")
= A
#a.update(7)
#context metadata((a == a, a == b, a.get(), b.get(), query(heading.where(level: float.nan)).len()))''',
    'interleaved_callbacks': '''#let a = counter(heading.where(level: 1))
#let b = counter(heading.where(level: 2))
#set heading(numbering: "1")
= A
#a.update(n => n + 10)
== B
#b.update((..n) => (20, 30))
= C
#a.update(n => n * 2)
#context metadata((a.get(), b.get(), counter(heading).get()))''',
    'callback_get_other_outside_context': '''#let a = counter(heading.where())
#let b = counter("other")
#b.update(7)
#a.update(n => n + b.get().first())
#context metadata(a.get())''',
    'callback_get_other_created_in_context': '''#let a = counter(heading.where())
#let b = counter("other")
#b.update(7)
#context a.update(n => n + b.get().first())
#context metadata(a.get())''',
    'unqueried_callback': '''#counter(heading.where()).update(n => panic("unused-callback"))
#context metadata(("survived", counter(page).final()))''',
    'get_prefix_before_bad_callback': '''#let c = counter(heading.where())
#set heading(numbering: "1")
= A
#context metadata(c.get())
#c.update(n => panic("after-prefix"))''',
    'counter_captured_outside_context': '''#let c = counter(heading.where())
#let read() = c.get()
#set heading(numbering: "1")
= A
#c.update(n => n + 10)
= B
#context metadata(read())''',
    'context_generated_update_assert12': '''#let c = counter(heading.where())
#set heading(numbering: "1")
= A
#context c.update(n => n + 10)
= B
#context { assert(c.get() == (12,)); metadata(("passed", c.get(), counter(page).final())) }''',
    'stable_one_page_plain': '''#let c = counter(heading.where())
#set page(height: 100cm)
#set heading(numbering: "1")
= A
#c.update(n => n + 10)
= B
#context metadata((c.get(), c.final(), counter(page).final()))''',
    'stable_one_page_context': '''#let c = counter(heading.where())
#set page(height: 100cm)
#set heading(numbering: "1")
= A
#context c.update(n => n + 10)
= B
#context { assert(c.get() == (12,)); metadata((c.get(), c.final(), counter(page).final())) }''',
}

INPUTS = [
    '00_nucleo/diagnosticos/p1339-where-counter-phase-probe.py',
    '00_nucleo/diagnosticos/p1339-where-counter-phase-probe-runs.json',
    '00_nucleo/diagnosticos/p1339-where-counter-phase-file-runs.json',
    'lab/typst-original/crates/typst-library/src/introspection/counter.rs',
]

def main():
    assert sha(BINARY) == EXPECTED
    result = dict(
        start=now(), head=git('rev-parse', 'HEAD').strip(),
        working_tree='uncommitted', diff_stat=git('diff', 'HEAD', '--stat'),
        status=git('status', '--short'), runner_sha256=sha(__file__),
        baseline=dict(path=BINARY, sha256=EXPECTED, upstream='a51e02804'),
        capabilities=dict(
            role='vanilla runtime measurer /root/p1339_counter_runtime_probe',
            readable=['vanilla sources', *INPUTS, 'skill instructions', 'git status and diff stat'],
            forbidden=['crystalline source', 'crystalline L0', 'candidate patch', 'materialization/context contents'],
            writable=['p1339-where-counter-runtime-probe.py', 'p1339-where-counter-runtime-probe.md', 'p1339-where-counter-runtime-probe-runs.json', '/tmp'],
            inheritance='repository rules and bounded role request; no candidate implementation',
            isolation='declared read discipline only; technically shared filesystem',
            regime='exploratory measurement; no contract, seal or implementation verdict'),
        inputs={p: sha(ROOT / p) for p in INPUTS}, cases=CASES, runs=[])
    for case, source in CASES.items():
        # Repeat only the stability controls; no adaptive revisions in this corpus.
        for repeat in range(2 if case.startswith('stable_') else 1):
            command = [BINARY, 'query', '-', 'metadata', '--field', 'value', '--format', 'json']
            start = now()
            proc = subprocess.run(command, cwd=ROOT, input=source, text=True,
                                  capture_output=True, timeout=30)
            result['runs'].append(dict(case=case, repeat=repeat, command=command,
                cwd=str(ROOT), source=source, source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                start=start, end=now(), exit_code=proc.returncode,
                stdout=proc.stdout, stderr=proc.stderr))
    result.update(end=now(), status_after=git('status', '--short'),
                  diff_stat_after=git('diff', 'HEAD', '--stat'))
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
