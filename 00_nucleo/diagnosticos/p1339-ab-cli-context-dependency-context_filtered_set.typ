#let c = counter(heading.where(level: 1))
#set heading(numbering: "1")
= A
#context c.update(11)
= B
#context assert(c.get() == (12,))
