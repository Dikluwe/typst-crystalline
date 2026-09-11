#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[]"); p1339-original-metadata(value) }
#set heading(numbering: "1.1")
= First
== Child
= Last
*Bold* _Emph_ Text
#context metadata(query(heading.where(level: "bad")).map(it => (it.level, repr(it.body))))
