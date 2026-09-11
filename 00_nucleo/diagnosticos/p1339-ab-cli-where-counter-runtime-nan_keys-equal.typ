#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) == "[false,false,[0],[0],0]"); p1339-original-metadata(value) }
#let a = counter(heading.where(level: float.nan))
#let b = counter(heading.where(level: float.nan))
#set heading(numbering: "1")
= A
#a.update(7)
#context metadata((a == a, a == b, a.get(), b.get(), query(heading.where(level: float.nan)).len()))
