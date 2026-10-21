#let a = (context [A])
#let b = (context [B])
#metadata((equal: a == b, first: a.fields(), second: b.fields())) <probe>
#a #b
