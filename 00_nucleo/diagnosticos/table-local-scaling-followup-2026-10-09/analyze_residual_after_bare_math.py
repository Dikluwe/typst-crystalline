"""Attribute sampled allocation stacks after the bare-math shortcut, read-only."""
import datetime
import gzip
import hashlib
import json
import re
import subprocess
from collections import Counter
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
IN = D / 'stacks-after-bare-math.json.gz'
PHASE = D / 'phases-after-bare-math.json.gz'
COUNTS = D / 'layout-call-counts-after-bare-math.json.gz'
OUT = D / 'residual-attribution-after-bare-math.json.gz'

def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):
    with gzip.open(path, 'rt') as f: return json.load(f)
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()

raw = read(IN)
phases = read(PHASE)
counts = read(COUNTS)
symbols = raw['symbols']
needles = {
    'table_defaults': 'table_cells::with_defaults',
    'table_realize': 'table_cells::realize_cell',
    'table_walk': 'table_cells::walk_node',
    'table_map': 'TableElem$u20$as',
    'cell_map': 'TableCellElem$u20$as',
    'style_scope': 'style_scope::layout_styled',
    'style_collapse': 'StyleChain::collapse',
    'content_styled': 'content::construction::_$LT$impl$u20$typst_core..entities..content..Content$GT$::styled',
    'source_span': 'content::source::_$LT$impl$u20$typst_core..entities..content..Content$GT$::with_source_span',
    'bare_math_prepare': 'bare_math::prepare',
    'font_select': 'FontBook::select_pattern',
}

def frames(line):
    return [symbols.get(pc, '') for pc in line.split()[2:]]

def own_leaf(fs):
    for frame in fs:
        if frame.startswith(('typst_core::', 'typst_infra::', 'typst_wiring::')) or 'typst_core..' in frame:
            return frame
    return None

def classify(row):
    inclusive = Counter(); leaves = Counter(); domains = Counter(); sampled_bytes = Counter()
    for line in row['stacks']:
        fs = frames(line)
        for label, needle in needles.items():
            if any(needle in f for f in fs): inclusive[label] += 1
        leaf = own_leaf(fs)
        for label, needle in needles.items():
            if leaf and needle in leaf: leaves[label] += 1
        if any('table_cells::' in f for f in fs): domain = 'table_realization'
        elif any('style_scope::layout_styled' in f for f in fs): domain = 'layout_styled_without_table_realization'
        elif any('compiler::layout::' in f for f in fs): domain = 'other_layout'
        elif any('compiler::eval::' in f for f in fs): domain = 'other_eval'
        else: domain = 'other'
        domains[domain] += 1
        sampled_bytes[domain] += int(line.split()[1])
    return {'fixture': row['fixture'], 'rep': row['rep'], 'header': row['header'],
            'inclusive_samples': dict(inclusive), 'owner_leaf_samples': dict(leaves),
            'disjoint_domains': dict(domains), 'sampled_requested_bytes': dict(sampled_bytes)}

result = {
    'utc': utc(), 'head': subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
    'working_tree': 'uncommitted',
    'git_status': subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),
    'git_diff_stat': subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),
    'source_sha256': sha(Path(__file__)),
    'inputs': {p.name: sha(p) for p in (IN, PHASE, COUNTS)},
    'binary_sha256': raw['binary_sha256'],
    'scope': 'One of 64 glibc allocation requests; inclusive stacks overlap, owner leaf categories are disjoint where symbolized; requested bytes are not live heap; debugger counts are calls, not CPU time.',
    'needles': needles,
    'cases': [classify(row) for row in raw['samples']],
    'phase_summary_ms': {k:{s:phases['summary'][k][s] for s in ('eval_ms','expand_context_ms','layout_ms','shape_ms','render_ms','total_ms')} for k in ('local-set-1600','no-fill-1600','alias-1600','inline-set-1600')},
    'layout_counts': {row['case']: row['counts'] for row in counts['samples']},
}
assert raw['binary_sha256'] == phases['binary_sha256'] == counts['binary_sha256']
assert all(result['cases'][i]['inclusive_samples'] == result['cases'][i+1]['inclusive_samples'] for i in (0,2,4))
with gzip.open(OUT,'wt') as f: json.dump(result,f,ensure_ascii=False,indent=2)
for row in result['cases'][::2]:
    print(row['fixture'],row['header'])
    print('domains',row['disjoint_domains'])
    print('inclusive',row['inclusive_samples'])
    print('leaf',row['owner_leaf_samples'])
print('phases',result['phase_summary_ms'])
print('layout_counts',result['layout_counts'])
