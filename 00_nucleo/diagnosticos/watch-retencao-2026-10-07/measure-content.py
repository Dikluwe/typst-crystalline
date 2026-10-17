"""Semantic cell edits, unlike historical comment-only grid revisions."""
import importlib.util,json,statistics
from pathlib import Path
s=importlib.util.spec_from_file_location('run',Path(__file__).with_name('run.py'));r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
original=r.b.source
r.b.source=lambda name,rev:original(name,rev).replace('[C000]',f'[C000 revision {rev}]') if name=='grid320' else original(name,rev)
before=r.state('content-before');rows=[]
for round_ in range(3):
    for engine in (('tekt','vanilla') if round_%2==0 else ('vanilla','tekt')):
        print('run',round_,engine,flush=True);rows.append(r.run('grid320',engine,round_))
after=r.state('content-after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
for i in range(3):
    a=next(x for x in rows if x['round']==i and x['engine']=='tekt');b=next(x for x in rows if x['round']==i and x['engine']=='vanilla')
    assert [x['text'] for x in a['samples']]==[x['text'] for x in b['samples']]
    for x in a['samples']:assert f'revision {x["revision"]}' in x['text']
summary={e:{k:statistics.median(x[k] for row in rows if row['engine']==e for x in row['samples']) for k in ('latency_ms','cpu_ms','VmRSS','VmHWM')} for e in ('tekt','vanilla')}
r.save('content-results.json',{'summary':summary,'runs':rows,'source_sha256':r.b.sha(Path(__file__)),'cpu':r.b.CPU});print(json.dumps(summary),flush=True)
