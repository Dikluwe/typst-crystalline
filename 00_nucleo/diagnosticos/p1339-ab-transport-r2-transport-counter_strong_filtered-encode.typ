#let p1339-original-metadata = metadata
#let metadata(value) = { let encoded = json.encode(value, pretty: false); p1339-original-metadata(value) }
*Bold* _Emph_ Text
#context metadata((counter(strong).get(), counter(strong.where()).get(), counter(strong.where(body: [Bold])).get(), counter(strong.where(body: [Miss])).get(), counter(strong.where()).display()))
