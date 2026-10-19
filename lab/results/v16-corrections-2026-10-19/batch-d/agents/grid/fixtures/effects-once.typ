#set page(width: 400pt, height: 300pt, margin: 24pt)
#set text(font: "Libertinus Serif", size: 10pt)
#let s = state("hits",0)
#show table.cell: it => { s.update(n=>n+1); it }
#table(columns: (100pt,100pt), [12.34], [item-2], [a/b], [abc_def])
#context [HITS: #s.get()]