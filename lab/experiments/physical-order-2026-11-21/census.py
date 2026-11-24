"""Count entry points in a separate watch run for the published phase graph."""
import json
import subprocess

from observe import ARMS, D, EXES, OUT, STATES, WORK, describe, document_key, dump, env

names={
 'tekt':{
    'wait':'::watch::wait_for_change_since',
    'paged':'::compile_paged_in_session',
    'realize_producer':'::Session::realize_with_provisional_locations',
    'physical_candidate':'::Session::physical_candidate',
    'physical_step':'>::physical_step',
    'physical_index':'>::index_physical_steps',
    'prepare_body':'::Session::prepare_running_body',
    'build_body':'::introspect::page_running::build_body',
    'provisional_introspect':'::introspect::entrypoints::introspect_with_introspector',
    'planned_index':'::introspect::page_running::introspect_with_page_running',
    'planned_layout':'::compiler::layout::convergence::run_planned',
    'physical_memo_call':'::physical_memo::layout_with',
    'line_commit':'::flush_line_with_outer_singleton',
 },
 'vanilla':{
    'wait':'typst_kit::watcher::Watcher::wait',
    'layout_document':'typst_layout::pages::layout_document',
    'layout_body':'typst_layout::pages::layout_document_common',
    'realization':'typst_realize::realize',
    'pdf':'typst_pdf::pdf',
 },
}

def main():
    expected=json.loads((OUT/'witness.json').read_text())
    counts={}
    for arm in ARMS:
        case=WORK/'census'/arm
        (case/'edits').mkdir(parents=True, exist_ok=True)
        addresses={name:[] for name in names[arm]}
        for row in (OUT/(arm+'.symbols')).read_text().splitlines():
            fields=row.split(maxsplit=3)
            if len(fields)!=4:
                continue
            symbol=fields[3].split('.llvm.',1)[0]
            for name,pattern in names[arm].items():
                if symbol.endswith(pattern):
                    addresses[name].append(int(fields[0],16))
        assert len(addresses['wait'])==1, addresses
        (case/'addresses.json').write_text(json.dumps(addresses,indent=2))
        for index,(_,data,leaf) in enumerate(STATES):
            assert leaf is None
            (case/('main.typ' if index==0 else f'edits/{index}.typ')).write_bytes(data)
        with (OUT/f'census-{arm}.log').open('wb') as log:
            subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'census.gdb'),
               '--args',str(EXES[arm]),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                cwd=case,env=env(case),stdout=log,stderr=subprocess.STDOUT,timeout=110,check=True)
        for index,(name,_,_) in enumerate(STATES):
            pdf=case/f'publication-{index}.pdf'
            if document_key(arm,pdf)!=expected[arm][name]['key']:
                assert describe(pdf)==expected[arm][name]['description'], (arm,name)
        counts[arm]=dict(available={name:bool(addr) for name,addr in addresses.items()},
                        counts=json.loads((case/'counts.json').read_text()))
        dump('census.json',counts)

if __name__=='__main__':
    main()
