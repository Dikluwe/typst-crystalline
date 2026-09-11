#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) == "{\"strong\":[{\"func\":\"strong\",\"body\":\"[semantic]\",\"fields\":\"(delta: 300, body: [semantic])\",\"label\":\"none\",\"delta\":300}],\"emph\":[],\"ordered\":[{\"func\":\"strong\",\"body\":\"[semantic]\",\"fields\":\"(delta: 300, body: [semantic])\",\"label\":\"none\",\"delta\":300}],\"strong_empty\":1,\"emph_empty\":0,\"strong_default\":[\"[semantic]\"],\"strong_zero\":[],\"strong_custom\":[],\"strong_body\":[],\"emph_body\":[]}"); p1339-original-metadata(value) }
#text(weight: "bold")[weight-only] #text(style: "italic")[style-only] #text(weight: "bold")[#strong[semantic]]
#context metadata((strong: query(strong.where()).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    emph: query(emph.where()).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    ordered: query(selector(strong.where()).or(emph.where())).map(it => (func: repr(it.func()), body: repr(it.body),
    fields: repr(it.fields()), label: repr(it.at("label", default: none)),
    delta: it.at("delta", default: "absent"))),
    strong_empty: query(strong.where()).len(),
    emph_empty: query(emph.where()).len(),
    strong_default: query(strong.where(delta: 300)).map(it => repr(it.body)),
    strong_zero: query(strong.where(delta: 0)).map(it => repr(it.body)),
    strong_custom: query(strong.where(delta: 100)).map(it => repr(it.body)),
    strong_body: query(strong.where(body: [inner])).map(it => repr(it.body)),
    emph_body: query(emph.where(body: [inner])).map(it => repr(it.body))))
