#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) == "\"2\""); p1339-original-metadata(value) }
#set heading(numbering: "1.1")
= First
== Child
= Last
*Bold* _Emph_ Text
#show text: it => { metadata(it.text); it }
#context counter(heading.where(level: 1)).display()
