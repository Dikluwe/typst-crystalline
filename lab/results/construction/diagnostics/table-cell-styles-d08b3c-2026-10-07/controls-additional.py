from pathlib import Path
script=(Path(__file__).parent/'measure.py').read_text()
prefix=script[:script.index('cases={}')]
runner=script[script.index('for name,(source,args) in cases.items():'):]
cases={}
def query(name,source):cases[name]=(source,['eval','--in','main.typ','--format','json','query(metadata).map(it=>it.value)'])
def compile(name,source):cases[name]=(source,['compile','main.typ','out.pdf'])
query('fully-known-equality','#show table.cell: it=>{metadata(it==table.cell(x:auto,y:auto,colspan:1,rowspan:1,inset:auto,align:auto,fill:none,stroke:none,breakable:auto)[a]);it.body}\n#table.cell(fill:none,stroke:none)[a]')
query('prepared-alias-equality','#show table.cell: it=>{let c=it;metadata(c==it);c}\n#table.cell[a]')
query('two-rules-effects','#show table.cell: it=>{metadata([EARLY]);it.body}\n#show table.cell: it=>{metadata([LATE]);it}\n#table.cell[a]')
query('two-rules-table-effects','#show table.cell: it=>{metadata([EARLY]);it.body}\n#show table.cell: it=>{metadata([LATE]);it}\n#table(columns:1,[a])')
query('alias-duplicated-effects','#show table.cell: it=>{metadata([EARLY]);it.body}\n#show table.cell: it=>[ #it #it ]\n#table.cell[a]')
compile('local-rule-render','#show table.cell: it=>[OUT #it.body]\n#{show table.cell: it=>[LOCAL #it.body];table.cell[a]}\n#table.cell[b]')
compile('alias-selector-auto','#let c=table.cell\n#show selector(c.where(x:auto)): it=>[AUTO #it.body]\n#c[a]')
query('fresh-error','#show table.cell: it=>table.cell(it.body)\n#table.cell[a]')
exec(prefix+runner,globals())
