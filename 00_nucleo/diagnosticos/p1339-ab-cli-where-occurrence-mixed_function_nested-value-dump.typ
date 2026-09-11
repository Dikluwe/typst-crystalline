#let p1339-original-metadata = metadata
#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))
#strong[#emph[inner]] #emph[#strong[inner]]
#context metadata((strong: query(strong).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    emph: query(emph).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    ordered: query(selector(strong).or(emph)).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    strong_empty: query(strong.where()).len(),
    emph_empty: query(emph.where()).len(),
    strong_default: query(strong.where(delta: 300)).map(it => repr(it.body)),
    strong_zero: query(strong.where(delta: 0)).map(it => repr(it.body)),
    strong_custom: query(strong.where(delta: 100)).map(it => repr(it.body)),
    strong_body: query(strong.where(body: [inner])).map(it => repr(it.body)),
    emph_body: query(emph.where(body: [inner])).map(it => repr(it.body))))
