#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[2],[2],[2],[0,1],[0]]"); p1339-original-metadata(value) }
#set heading(numbering: "1.1")
= First
== Child
= Last
*Bold* _Emph_ Text
#context metadata((counter(heading).get(), counter(heading.where()).get(), counter(heading.where(level: 1)).get(), counter(heading.where(level: 2)).get(), counter(heading.where(level: 9)).get()))
