from collections import Counter
import importlib.util
import json
from pathlib import Path
from statistics import mean
import tarfile
import sys

D=Path(__file__).resolve().parent
DIAG=D.parents[1]/'results/construction/diagnostics'
OUT=DIAG/D.name
spec=importlib.util.spec_from_file_location('sample_decoder',D.parent/'references-500-visible-edit-phases-2026-11-21/analyze.py')
decoder=importlib.util.module_from_spec(spec)
spec.loader.exec_module(decoder)

def category(arm, stack):
    def has(*patterns):
        return any(p in f for p in patterns for f in stack)
    if arm=='vanilla':
        if has('typst_eval::eval'): return 'eval'
        if has('typst_pdf::pdf'): return 'pdf'
        if has('typst_layout::layout_document','typst_layout::layout::','typst_layout::flow::','typst_layout::inline::','typst_layout::introspect::'): return 'physical'
        if has('typst_realize::realize'): return 'realization'
        if has('typst::compile', 'typst::engine', 'typst::compile_impl'): return 'compile_other'
        return 'other'
    if has('::eval_to_module_in_session'): return 'eval'
    if has('::compile_paged_in_session'):
        if has('::shaper::traversal'): return 'shaping'
        if has('::layout_bidi::','::bidi_context::Registry'): return 'bidi'
        if has('::shaper::reconciliation::'): return 'shaping_reconciliation'
        if has('::introspect::entrypoints::introspect_with_introspector'):
            if has('::Session::realize'): return 'topology_introspection'
            return 'source_introspection'
        if has('::physical_memo::document_bounded'): return 'physical_budget_audit'
        if has('::coverage_complete'): return 'trace_coverage'
        if has('::index_physical_steps', '::introspect_with_page_running'): return 'plan_index'
        if has('::prepare_running_body','::prepare_with_root_chain'): return 'source_plan'
        if has('::Journal::validates','::Journal::page_inputs_match'): return 'physical_read_validation'
        if has('::lab_prepare_three_splice','::lab_finish_three'): return 'suffix_composition'
        if has('::convergence::run_planned','::layout_planned_root'): return 'physical'
        if has('::prepare_outline_candidate'): return 'outline'
        if has('::pipeline::substitute_context_blocks','::realize_content','::realize_document',
               '::realize_root_contexts','::compiler::realize::','::Session::realize'): return 'realization'
        return 'paged_other'
    if has('::compile_to_pdf_bytes_impl'): return 'pdf'
    return 'other'

combined='--combined' in sys.argv
label='post-profile' if combined else 'profile'
rows=[json.loads(r) for r in (OUT/f'{label}-rows.jsonl').read_text().splitlines()]
symbols_native=DIAG/'references-500-visible-page-operator-cost-2026-11-21'
assert (symbols_native/'vanilla-symbols-verified').exists()
tables={'control':decoder.symbols(OUT/'control.symbols'),
        'vanilla':decoder.symbols(symbols_native/'vanilla-symbols-twin.txt')}
arms=('candidate','control','vanilla') if combined else ('control','vanilla')
if combined:
    tables['candidate']=decoder.symbols(OUT/'candidate.symbols')
inventories={a:Counter() for a in arms}
exclusive={a:Counter() for a in arms}
chains={a:Counter() for a in arms}
computed=[]
with tarfile.open(OUT/f'{label}s.tar.gz') as archive:
    for row in rows:
        arm=row['arm']
        counts,detail=decoder.stacks(archive,row,tables)
        phase=Counter()
        for stack,n in counts.items():
            c=category(arm,stack)
            phase[c]+=n
            for f in set(stack):
                inventories[arm][(c,f)]+=n
            typed=tuple(f for f in stack if f.startswith(('typst','<typst','krilla')))
            chains[arm][(c,typed[:12])]+=n
            exclusive[arm][(c,stack[0])]+=n
        factor=detail['main_ns']/1e6/counts.total()
        computed.append(row|detail|dict(phase_ms={k:n*factor for k,n in phase.items()}))
summary={}
for arm in arms:
    a=[r for r in computed if r['arm']==arm]
    ks=sorted({k for r in a for k in r['phase_ms']})
    summary[arm]={'sessions':len(a),'ticks':sum(r['samples'] for r in a),
         'truncated':sum(r['truncated'] for r in a),
         'main_thread_coverage':sum(r['main_ns'] for r in a)/sum(r['cpu_ns'] for r in a),
         'by_period':{
             str(period):{k:mean(r['phase_ms'].get(k,0) for r in a if not period or period==r['period_ns']) for k in ks}
             for period in (0,347000,937000)
         }}
with (OUT/('post-stack-ranks.txt' if combined else 'stack-ranks.txt')).open('w') as f:
    for arm in arms:
        for title,c in [('leaf',exclusive[arm]),('inclusive',inventories[arm]),('chain',chains[arm])]:
            print(f'==== {arm} {title}',file=f)
            for k,n in c.most_common(80):
                print(n,k,file=f)
(OUT/f'{label}.json').write_text(json.dumps({'summary':summary,'rows':computed},indent=2)+'\n')
print(json.dumps(summary,indent=2))
