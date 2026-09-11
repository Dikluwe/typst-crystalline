#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[42],[1]]"); p1339-original-metadata(value) }
#set heading(numbering: "1")
= First
#counter(heading).update(42)
#context metadata((counter(heading).get(), counter(heading.where()).get()))
