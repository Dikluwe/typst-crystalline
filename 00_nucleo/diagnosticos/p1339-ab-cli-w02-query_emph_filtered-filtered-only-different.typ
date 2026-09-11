#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[\"[Emph]\"],[\"[Emph]\"],[\"[Emph]\"],0]"); p1339-original-metadata(value) }
*Bold* _Emph_ Text
#context metadata((query(emph.where()).map(it => repr(it.body)), query(emph.where()).map(it => repr(it.body)), query(emph.where(body: [Emph])).map(it => repr(it.body)), query(emph.where(body: [Miss])).len()))
