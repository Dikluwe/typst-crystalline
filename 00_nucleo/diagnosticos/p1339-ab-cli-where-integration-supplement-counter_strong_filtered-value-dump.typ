#let p1339-original-metadata = metadata
#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))
*Bold* _Emph_ Text
#context metadata((counter(strong).get(), counter(strong.where()).get(), counter(strong.where(body: [Bold])).get(), counter(strong.where(body: [Miss])).get(), counter(strong.where()).display()))
