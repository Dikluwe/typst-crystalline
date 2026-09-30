from pathlib import Path
import json,datetime
base=Path('00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28')
old=Path('00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28')
new=base/'after'
results=[]
for filename in ['resultados.json','resultados-controles.json','resultados-isolamento.json']:
    before={row['case']:row for row in json.loads((old/filename).read_text())}
    after={row['case']:row for row in json.loads((new/filename).read_text())}
    assert before.keys()==after.keys()
    for case,a in before.items():
        b=after[case];changes=[]
        if a['source_sha256']!=b['source_sha256']:changes.append('source')
        for lane in ['tekt','vanilla']:
            for field in ['exit_code','pages','text','page_count','line_count']:
                if a['runs'][lane].get(field)!=b['runs'][lane].get(field):changes.append(lane+':'+field)
            def normalize(value,root):return value.replace(str(Path.cwd()/root),'<probe-root>').replace(str(root),'<probe-root>')
            if normalize(a['runs'][lane]['stderr'],old)!=normalize(b['runs'][lane]['stderr'],new):changes.append(lane+':stderr')
        results.append({'case':case,'same':not changes,'changes':changes})
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'comparison':'Same sources, exit codes, exact extracted text and full page/line/word bounding boxes in both lanes. Stderr normalizes only the probe directory; binary PDF bytes are not the parity criterion.','cases':results,'total':len(results),'unchanged':sum(r['same'] for r in results)}
(base/'behavior-comparison.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
print('unchanged',receipt['unchanged'],'of',receipt['total'])
print('differences',[r for r in results if not r['same']])
assert receipt['unchanged']==receipt['total']
