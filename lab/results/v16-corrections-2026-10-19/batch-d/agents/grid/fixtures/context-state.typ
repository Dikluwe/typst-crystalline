#set page(width: 400pt, height: 300pt, margin: 24pt)
#set text(font: "Libertinus Serif", size: 10pt)
#let s = state("k",0)
#table(columns: (100pt,100pt), [#s.update(n=>n+1)A], [#context s.get()], [#s.update(n=>n+1)B], [#context s.get()])
#context [FINAL: #s.get()]