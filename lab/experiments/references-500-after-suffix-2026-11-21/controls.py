from pathlib import Path
import json
import re
import subprocess
import sys

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, STATES, EXES, cycle, run
sys.path.insert(0,str(D.parent/'references-500-visible-stable-cids-2026-11-21'))
from pdf_check import structure, text_layout

describe=cycle.cycle.original.compare.describe
settings={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
pattern=re.compile(r'^lab-topology-index attempt=(\d+) roots=(\d+) builds=(\d+) hits=(\d+) physical=(\d+) invalidations=(\d+) checked=(\d+)$',re.M)
fields=('attempt','roots','builds','hits','physical','invalidations','checked')

def attempts(log):
    return [dict(zip(fields,map(int,row))) for row in pattern.findall(log)]

previous=json.loads((OUT/'topology-control.json').read_text())['counts']
green=json.loads((OUT/'topology-candidate.json').read_text())['counts']
assert previous['3:introspect_for_realize']==20
assert green['3:introspect_for_realize']==1,(green,previous)
assert green['3:line_commit']==previous['3:line_commit']==57
verified=attempts((OUT/'topology-candidate-verify.log').read_text().split('compiled successfully')[-2])
assert any(r['builds']==1 and r['hits']==19 for r in verified),verified
assert all(r['checked']==r['hits'] for r in verified),verified
(OUT/'green.json').write_text(json.dumps(dict(control=previous,candidate=green,verified=verified),indent=2)+'\n')

def check_pdf(a,b):
    assert describe(a)==describe(b),(a,b)
    assert structure(a)==structure(b),(a,b)
    assert text_layout(a)==text_layout(b),(a,b)

def watch_controls():
    results={}
    visible=STATES[-1][1]
    for block,(label,data) in enumerate((
        ('visible',visible),
        ('height',visible.replace(b'height:600pt',b'height:440pt',1)),
        ('reference',visible.replace(b'See @s0.',b'See @s13.',1)),
        ('suffix',visible.replace(b'See @s19.',b'See @s13.',1)),
        ('prefix-wrap',visible.replace(b'Visible paragraph',
            b'Many additional words make this descriptive visible paragraph take several extra lines',1)),
    )):
        directory=f'directed/watch-{label}'
        documents={}
        for index,arm in enumerate(('control','candidate')):
            EXES['candidate']=WORK/arm
            extras=settings|({'TEKT_LAB_TOPOLOGY_INDEX_TRACE':'verify'} if arm=='candidate' else {})
            row=run(block*2+index,'candidate',0,overrides=extras,
                    states=[*STATES[:3],(label,data,None)],check=label=='visible',
                    directory=directory)
            documents[arm]=WORK/row['directory']/'out.pdf'
        check_pdf(documents['control'],documents['candidate'])
        actual=describe(documents['candidate'])
        results[label]={'equal':True,'pages':actual['page_count'],'links':sum(map(len,actual['links']))}
    return results

def compile_case(data, directory, arm):
    case=WORK/directory/arm
    case.mkdir(parents=True)
    (case/'main.typ').write_bytes(data)
    env=cycle.environment(case,'candidate',False)
    if arm=='candidate':
        env['TEKT_LAB_TOPOLOGY_INDEX_TRACE']='verify'
    env.update(settings)
    process=subprocess.run(['taskset','-c','1',str(WORK/arm),'compile','main.typ','out.pdf',
                   '--root',str(case),'--pdf-tagged=true'],
                   cwd=case,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=120)
    log=process.stdout.decode()
    (case/'compile.log').write_text(log)
    return case/'out.pdf',process.returncode,log

figure=b'#figure([ART], kind: image, caption: [CAPTION], supplement: it => [CALLBACK-SUPPLEMENT]) <fig>\n'
topology=b"""#set heading(numbering: "1.")
= Alpha <a>

@a

#context {
  let n = counter(heading.where(level: 1)).get()
  [#metadata("changed-topology") <changed> #context [Nested result]]
}

= Beta <b>

@b
"""
compiled={}
for label,data in (
    ('topology-change',topology),
    ('function-supplement',figure+b'@fig'),
    ('supplement-error',figure.replace(b'[CALLBACK-SUPPLEMENT]',b'panic("REF_INNER")')+b'@fig'),
):
    a,ac,al=compile_case(data,f'directed/{label}','control')
    b,bc,bl=compile_case(data,f'directed/{label}','candidate')
    assert ac==bc,(label,al,bl)
    batch=attempts(bl)
    assert all(r['checked']==r['hits'] for r in batch),batch
    if label=='supplement-error':
        assert ac!=0 and 'REF_INNER' in al and 'REF_INNER' in bl,(label,al,bl)
        language=lambda log:[s for s in log.splitlines() if s.startswith(('error:', 'warning:', 'hint:'))]
        assert language(al)==language(bl),(al,bl)
    else:
        assert ac==0,(label,al,bl)
        check_pdf(a,b)
    if label=='topology-change':
        assert any(r['invalidations'] and r['builds']>1 for r in batch),batch
    compiled[label]={'code':bc,'attempts':batch,'equal':True}
combined=dict(compile=compiled,watch=watch_controls())
(OUT/'directed.json').write_text(json.dumps(combined,indent=2)+'\n')
print(combined)
