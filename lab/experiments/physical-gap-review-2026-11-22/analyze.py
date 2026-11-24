"""Attribute independent profile samples to mutually exclusive pipeline stages."""
from collections import Counter
import importlib.util
import json
from pathlib import Path
from statistics import mean
import tarfile

D = Path(__file__).resolve().parent
OUT = D.parents[1]/'results/construction/diagnostics'/D.name
source = D.parent/'references-500-visible-edit-phases-2026-11-21/analyze.py'
spec = importlib.util.spec_from_file_location('decoder', source)
decoder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(decoder)

def phase(arm, stack):
    def has(*patterns):
        return any(p in frame for p in patterns for frame in stack)

    if arm == 'vanilla':
        if has('typst_pdf::pdf', 'typst_pdf::export'):
            return 'pdf'
        if has('typst_realize::realize', 'typst_realize::visit'):
            return 'realization'
        if has('typst_eval::eval'):
            return 'eval'
        if has('typst_layout::introspect', 'DocumentIntrospector::new'):
            return 'index'
        if has('typst_layout::', 'typst_layout::layout_document'):
            return 'physical'
        if has('typst::compile'):
            return 'compile_other'
        return 'other'

    if has('::shaper::traversal::','::layout_bidi::','::shaper::reconciliation::',
           '::bidi_context::Registry'):
        return 'text_post'
    if has('::prepare_running_body', '::prepare_page_running_body', '::prepare_with_root_chain'):
        return 'source_plan'
    if has('::index_physical_steps', '::introspect_with_page_running',
           '::introspect_with_introspector', '::introspect_with_runtime_for_pipeline'):
        return 'index'
    if has('::prepare_outline_candidate', '::realize_outlines_for_pipeline'):
        return 'outline'
    if has('::substitute_context_blocks'):
        return 'substitution'
    if has('::compiler::layout::convergence::', '::layout_planned_root',
           '::layout_with_introspector_and_metrics', '::compiler::layout::physical_memo::'):
        return 'physical'
    if has('::realize_content','::realize_document','::realize_root_contexts',
           '::compiler::realize::','::Session::realize'):
        return 'realization'
    if has('::eval_to_module_in_session', 'typst_core::compiler::eval::'):
        return 'eval'
    if has('::compile_paged_in_session', '::context_stabilization::'):
        return 'compile_other'
    if has('::compile_to_pdf_bytes_impl','::export::'):
        return 'pdf'
    return 'other'

if __name__ == '__main__':
    rows=[json.loads(row) for row in (OUT/'profile.jsonl').read_text().splitlines()]
    tables={arm:decoder.symbols(OUT/(arm+'.symbols')) for arm in ('tekt','vanilla')}
    details={}
    breakdown={}
    ranks=[]
    with tarfile.open(OUT/'profiles.tar.gz') as raw:
        for arm,table in tables.items():
            arm_rows=[row for row in rows if row['arm']==arm]
            assert len(arm_rows)==20
            cohorts=[]
            hot=Counter()
            branches=Counter()
            raw_shares=Counter()
            for row in arm_rows:
                stacks,timing=decoder.stacks(raw,row,tables)
                counts=Counter()
                for stack,n in stacks.items():
                    label=phase(arm,stack)
                    counts[label]+=n
                    hot[label,stack[0]]+=n
                    visible=[f for f in stack if f.startswith(('typst','<typst','krilla'))]
                    branches[label,tuple(visible[:12])]+=n
                scale=timing['main_ns']/1e6/timing['samples']
                cohorts.append(row|timing|{'phase_ms':{label:n*scale for label,n in counts.items()}})
                raw_shares.update(counts)
            details[arm]=cohorts
            durations={}
            for period in (0,347000,937000):
                selected=[row for row in cohorts if not period or row['period_ns']==period]
                durations[str(period)]={key:mean(r['phase_ms'].get(key,0) for r in selected)
                    for key in sorted(raw_shares)}
            breakdown[arm]={'period_ms':durations,'samples':raw_shares.total(),
                'truncated':sum(r['truncated'] for r in cohorts),
                'profiled_main_to_process':sum(r['main_ns'] for r in cohorts)/sum(r['cpu_ns'] for r in cohorts)}
            for name,count in [('leaf',hot),('chain',branches)]:
                ranks.append('==== '+arm+' '+name)
                ranks.extend(f'{n} {key}' for key,n in count.most_common(90))
    (OUT/'profile-breakdown.json').write_text(json.dumps(dict(breakdown=breakdown,rows=details),indent=2)+'\n')
    (OUT/'stack-ranks.txt').write_text('\n'.join(ranks)+'\n')
    print(json.dumps(breakdown,indent=2))
