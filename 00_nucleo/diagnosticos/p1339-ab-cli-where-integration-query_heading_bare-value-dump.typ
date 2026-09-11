#let p1339-original-metadata = metadata
#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))
#set heading(numbering: "1.1")
= First
== Child
= Last
*Bold* _Emph_ Text
#context metadata(query(heading).map(it => (it.level, repr(it.body))))
