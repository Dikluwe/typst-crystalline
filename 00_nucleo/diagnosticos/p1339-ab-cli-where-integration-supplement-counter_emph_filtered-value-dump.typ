#let p1339-original-metadata = metadata
#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))
*Bold* _Emph_ Text
#context metadata((counter(emph).get(), counter(emph.where()).get(), counter(emph.where(body: [Emph])).get(), counter(emph.where(body: [Miss])).get(), counter(emph.where()).display()))
