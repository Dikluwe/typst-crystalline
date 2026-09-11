#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[1],[1],[1],[0],\"1\"]"); p1339-original-metadata(value) }
*Bold* _Emph_ Text
#context metadata((counter(emph.where()).get(), counter(emph.where()).get(), counter(emph.where(body: [Emph])).get(), counter(emph.where(body: [Miss])).get(), counter(emph.where()).display()))
