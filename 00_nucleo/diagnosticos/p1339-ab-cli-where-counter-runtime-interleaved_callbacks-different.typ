#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[24],[20,30],[2]]"); p1339-original-metadata(value) }
#let a = counter(heading.where(level: 1))
#let b = counter(heading.where(level: 2))
#set heading(numbering: "1")
= A
#a.update(n => n + 10)
== B
#b.update((..n) => (20, 30))
= C
#a.update(n => n * 2)
#context metadata((a.get(), b.get(), counter(heading).get()))
