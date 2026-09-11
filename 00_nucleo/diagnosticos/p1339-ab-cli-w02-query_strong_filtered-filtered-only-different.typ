#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[[\"[Bold]\"],[\"[Bold]\"],[\"[Bold]\"],0]"); p1339-original-metadata(value) }
*Bold* _Emph_ Text
#context metadata((query(strong.where()).map(it => repr(it.body)), query(strong.where()).map(it => repr(it.body)), query(strong.where(body: [Bold])).map(it => repr(it.body)), query(strong.where(body: [Miss])).len()))
