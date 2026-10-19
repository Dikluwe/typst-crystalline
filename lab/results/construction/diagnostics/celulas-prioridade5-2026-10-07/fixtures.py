from pathlib import Path
import json
D=Path(__file__).resolve().parent; F=D/'fixtures';F.mkdir(exist_ok=True)
prefix='#set page(width: 500pt, height: 3500pt, margin: 30pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'
cases={}
for kind in ['grid','table']:
 for name,expr in [('punctuation','"item-"+str(i)+".25"'),('alnum','"Cell"+str(i)'),('spaced','"item "+str(i)'),('unicode','"café-"+str(i)')]:
  cases[kind+'_'+name]=prefix+f'#{kind}(columns: (100pt,100pt,100pt,100pt), ..range(320).map(i => [#({expr})]))\n'
for kind in ['grid','table']:
 for label,token in [('literal_punctuation','item-{}.25'),('literal_alnum','Cell{}')]:
  cases[kind+'_'+label]=prefix+'#'+kind+'(columns: (100pt,100pt,100pt,100pt),\n'+',\n'.join('['+token.format(i)+']' for i in range(320))+'\n)'
cases['rtl_inset']=prefix+'#set text(dir: rtl, fill: red)\n#table(columns: (90pt,90pt), table.cell(inset: 3pt)[12.34], table.cell(inset: 5pt)[item-2])'
cases['multipage']=prefix.replace('3500pt','120pt')+'#table(columns: (100pt,100pt), ..range(40).map(i => [#("item-"+str(i))]))'
cases['styled']=prefix+'#show "item": strong\n#table(columns: (100pt,100pt), [item-2], [12.34], [*item-3*], [a/b])'
cases['placement']=prefix+'#table(columns: (100pt,100pt), table.cell(colspan: 2, align: right)[12.34], table.cell(x: 1, y: 1)[a/b], [item-2])'
cases['wrapped']=prefix+'#table(columns: (12pt,12pt), [long-token-1], [long-token-2])'
cases['fixed_rows']=prefix+'#table(columns: (100pt,100pt), rows: (20pt,30pt), [12.34], [item-2], [a/b], [abc_def])'
cases['effects']=prefix+'#show table.cell: it => { counter("hits").step(); it }\n#table(columns: (100pt,100pt), [12.34], [item-2], [a/b], [abc_def])\n#context counter("hits").display()'
for name,text in cases.items():(F/(name+'.typ')).write_text(text)
(D/'fixtures.json').write_text(json.dumps(cases,ensure_ascii=False,indent=2)+'\n')
