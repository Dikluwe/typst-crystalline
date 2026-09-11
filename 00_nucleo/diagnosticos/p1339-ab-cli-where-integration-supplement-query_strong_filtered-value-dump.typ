#let p1339-original-metadata = metadata
#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))
*Bold* _Emph_ Text
#context metadata((query(strong).map(it => repr(it.body)), query(strong.where()).map(it => repr(it.body)), query(strong.where(body: [Bold])).map(it => repr(it.body)), query(strong.where(body: [Miss])).len()))
