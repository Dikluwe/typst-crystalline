#!/usr/bin/env python3
"""Focal vanilla-only occurrence measurement; emits receipt to stdout."""
import datetime
import hashlib
import json
import pathlib
import shlex
import subprocess

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
BIN = '/usr/local/bin/typst'
EXPECTED = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def run(argv, stdin=None):
    start = utc()
    proc = subprocess.run(argv, input=stdin, text=True, capture_output=True,
                          cwd=ROOT, timeout=45)
    return dict(argv=argv, shell_command=shlex.join(argv), stdin=stdin,
                utc_start=start, utc_end=utc(), exit_code=proc.returncode,
                stdout=proc.stdout, stderr=proc.stderr)

def state():
    return {name: run(argv) for name, argv in (
        ('head', ['git', 'rev-parse', 'HEAD']),
        ('diff_stat', ['git', 'diff', 'HEAD', '--stat']),
        ('status', ['git', 'status', '--short']),
    )}

project = '''it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))'''
observe = '''(strong: query(strong).map(PROJECT),
    emph: query(emph).map(PROJECT),
    ordered: query(selector(strong).or(emph)).map(PROJECT),
    strong_empty: query(strong.where()).len(),
    emph_empty: query(emph.where()).len(),
    strong_default: query(strong.where(delta: 300)).map(it => repr(it.body)),
    strong_zero: query(strong.where(delta: 0)).map(it => repr(it.body)),
    strong_custom: query(strong.where(delta: 100)).map(it => repr(it.body)),
    strong_body: query(strong.where(body: [inner])).map(it => repr(it.body)),
    emph_body: query(emph.where(body: [inner])).map(it => repr(it.body)))'''.replace('PROJECT', project)
fixtures = [
    ('call_vs_markup', '#strong[call-S] *syntax-S* #emph[call-E] _syntax-E_'),
    ('same_function_nested', '#strong[outer #strong[inner] tail] #emph[outer #emph[inner] tail]'),
    ('mixed_function_nested', '#strong[#emph[inner]] #emph[#strong[inner]]'),
    ('markup_nested', '*outer *inner* tail* _outer _inner_ tail_'),
    ('text_style_controls', '#text(weight: "bold")[weight-only] #text(style: "italic")[style-only] #text(weight: "bold")[#strong[semantic]]'),
    ('set_strong_delta', '#set strong(delta: 100)\n*style-default* #strong[call-default] #strong(delta: 0)[explicit-zero] #strong(delta: 300)[explicit-default]'),
    ('explicit_delta', '#strong(delta: 100)[custom] #strong(delta: 0)[zero] *implicit*'),
    ('label_boundaries', '#strong[outer #strong[inner]<inner-s>]<outer-s> #emph[inner]<inner-e>'),
    ('reused_content', '#let s = strong[inner]\n#let e = emph[inner]\n#s #s #e #e'),
    ('body_morphology', '#strong[inner] #strong[_inner_] #strong[*inner*] #emph[inner] #emph[*inner*]'),
]
extra = '''(label_direct: query(<inner-s>).map(PROJECT),
    strong_labeled: query(strong.where(label: <inner-s>)).map(PROJECT),
    wrong_label: query(strong.where(label: <inner-e>)).len())'''.replace('PROJECT', project)
cases = []
for name, body in fixtures:
    expression = '(main: ' + observe + ', labels: ' + extra + ')' if name == 'label_boundaries' else observe
    source = body + '\n#context metadata(' + expression + ')\n'
    cases.append(dict(id=name, source=source,
                      argv=[BIN, 'query', '-', 'metadata', '--field', 'value']))
cases.append(dict(id='constructor_projection', source=None,
    argv=[BIN, 'eval', '''{ let s = strong[inner]; let e = emph[inner];
        (strong: (func: repr(s.func()), fields: repr(s.fields()),
          delta: s.at("delta", default: "absent")),
        emph: (func: repr(e.func()), fields: repr(e.fields()))) }''', '--format', 'json']))
inputs = [
    pathlib.Path(__file__).relative_to(ROOT).as_posix(),
    '00_nucleo/diagnosticos/p1339-where-integration-probe.py',
    '00_nucleo/diagnosticos/p1339-where-integration-probe.md',
    '00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md',
    '00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md',
    '00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md',
]
receipt = dict(regime='executado sem atestacao de isolamento',
    executor='/root/p1339_strong_emph_occurrence_probe',
    baseline_upstream='a51e02804', binary=BIN, binary_sha256=sha(BIN),
    utc_start=utc(), inputs={p: sha(ROOT / p) for p in inputs}, before=state(), cases=cases)
assert receipt['binary_sha256'] == EXPECTED
receipt['runs'] = [dict(id=c['id'], **run(c['argv'], c['source'])) for c in cases]
receipt['after'] = state()
receipt['utc_end'] = utc()
print(json.dumps(receipt, ensure_ascii=False, indent=2))
