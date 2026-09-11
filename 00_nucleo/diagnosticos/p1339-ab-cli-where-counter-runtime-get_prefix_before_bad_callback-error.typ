#let c = counter(heading.where())
#set heading(numbering: "1")
= A
#context metadata(c.get())
#c.update(n => panic("after-prefix"))
