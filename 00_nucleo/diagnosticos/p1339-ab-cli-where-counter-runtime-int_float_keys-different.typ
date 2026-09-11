#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[true,[12],[12],2]"); p1339-original-metadata(value) }
#let a = counter(heading.where(level: 1))
#let b = counter(heading.where(level: 1.0))
#set heading(numbering: "1")
= A
#a.update(n => n + 10)
= B
#context metadata((a == b, a.get(), b.get(), query(heading.where(level: 1.0)).len()))
