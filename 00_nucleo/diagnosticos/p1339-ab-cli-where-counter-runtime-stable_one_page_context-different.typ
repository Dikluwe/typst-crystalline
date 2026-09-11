#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[12],[12],[1]]"); p1339-original-metadata(value) }
#let c = counter(heading.where())
#set page(height: 100cm)
#set heading(numbering: "1")
= A
#context c.update(n => n + 10)
= B
#context { assert(c.get() == (12,)); metadata((c.get(), c.final(), counter(page).final())) }
