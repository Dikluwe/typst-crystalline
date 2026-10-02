import hashlib, json, os, pathlib, resource, statistics, subprocess, time
root=pathlib.Path('/repos/Antigravity/typst-crystalline')
out=pathlib.Path('/tmp/tekt-grid-measure'); out.mkdir(exist_ok=True)
engines={'tekt':root/'target/release/typst','vanilla':root/'lab/typst-original/target/release/typst'}
sha=lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
fx={}
for rows in (1,20,80):
    p=out/f'grid{rows}.typ'
    cells=',\n'.join(f'[C{r:02d}{c}]' for r in range(rows) for c in range(4))
    p.write_text('#set page(width: 500pt, height: 3500pt, margin: 30pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n#table(columns: 4, rows: '+str(rows)+',\n'+cells+'\n)\n')
    fx[rows]=p
cpu=min(os.sched_getaffinity(0))
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1')
def run(rows,engine,i,record=True):
    pdf=out/f'{rows}-{engine}-{i}.pdf'
    cmd=[str(engines[engine]),'compile',str(fx[rows]),str(pdf),'--root',str(out),'--pdf-tagged=false','--pretty=false']
    if engine=='vanilla': cmd+=['--jobs','1']
    t=time.perf_counter_ns()
    proc=subprocess.Popen(cmd,cwd=root,env=env,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}))
    _,status,ru=os.wait4(proc.pid,0)
    stderr=proc.stderr.read().decode(errors='replace')
    if status: raise RuntimeError(f'{cmd}: {status} {stderr}')
    elapsed=(time.perf_counter_ns()-t)/1e6
    if not record: return
    info=subprocess.check_output(['pdfinfo',str(pdf)],text=True)
    pages=next(x.split(':',1)[1].strip() for x in info.splitlines() if x.startswith('Pages:'))
    size=next(x.split(':',1)[1].strip() for x in info.splitlines() if x.startswith('Page size:'))
    raw=subprocess.check_output(['pdftotext','-raw',str(pdf),'-'])
    return {'rows':rows,'engine':engine,'index':i,'wall_ms':elapsed,'user_ms':ru.ru_utime*1000,'system_ms':ru.ru_stime*1000,'pages':pages,'page_size':size,'text_sha256':hashlib.sha256(raw).hexdigest(),'text_bytes':len(raw)}
rows_data=[]
for rows in fx:
    for warm in range(2):
        for engine in ('tekt','vanilla'): run(rows,engine,f'w{warm}',False)
    for i in range(12):
        order=('tekt','vanilla') if i%2==0 else ('vanilla','tekt')
        for engine in order: rows_data.append(run(rows,engine,i))
summary={}
for rows in fx:
    t=[x['wall_ms'] for x in rows_data if x['rows']==rows and x['engine']=='tekt']
    v=[x['wall_ms'] for x in rows_data if x['rows']==rows and x['engine']=='vanilla']
    summary[rows]={'tekt_median_ms':statistics.median(t),'vanilla_median_ms':statistics.median(v),'paired_delta_median_ms':statistics.median(a-b for a,b in zip(t,v)),'tekt_range_ms':[min(t),max(t)],'vanilla_range_ms':[min(v),max(v)],'output_equal':all(rows_data[j]['pages']==rows_data[j+1]['pages'] and rows_data[j]['page_size']==rows_data[j+1]['page_size'] and rows_data[j]['text_sha256']==rows_data[j+1]['text_sha256'] for j in range((list(fx).index(rows))*24, (list(fx).index(rows)+1)*24,2))}
result={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1'],cwd=root,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=root,text=True),'utc':time.strftime('%Y-%m-%d %H:%M:%S UTC',time.gmtime()),'cpu':cpu,'binary_sha256':{k:sha(v) for k,v in engines.items()},'fixture_sha256':{str(k):sha(v) for k,v in fx.items()},'summary':summary,'samples':rows_data}
(out/'results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(summary,indent=2))
