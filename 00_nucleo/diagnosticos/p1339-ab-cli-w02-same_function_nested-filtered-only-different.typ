#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "{\"strong\":[{\"func\":\"strong\",\"body\":\"sequence([outer], [ ], strong(body: [inner]), [ ], [tail])\",\"fields\":\"(\\n  delta: 300,\\n  body: sequence([outer], [ ], strong(body: [inner]), [ ], [tail]),\\n)\",\"label\":\"none\",\"delta\":300},{\"func\":\"strong\",\"body\":\"[inner]\",\"fields\":\"(delta: 300, body: [inner])\",\"label\":\"none\",\"delta\":300}],\"emph\":[{\"func\":\"emph\",\"body\":\"sequence([outer], [ ], emph(body: [inner]), [ ], [tail])\",\"fields\":\"(\\n  body: sequence([outer], [ ], emph(body: [inner]), [ ], [tail]),\\n)\",\"label\":\"none\",\"delta\":\"absent\"},{\"func\":\"emph\",\"body\":\"[inner]\",\"fields\":\"(body: [inner])\",\"label\":\"none\",\"delta\":\"absent\"}],\"ordered\":[{\"func\":\"strong\",\"body\":\"sequence([outer], [ ], strong(body: [inner]), [ ], [tail])\",\"fields\":\"(\\n  delta: 300,\\n  body: sequence([outer], [ ], strong(body: [inner]), [ ], [tail]),\\n)\",\"label\":\"none\",\"delta\":300},{\"func\":\"strong\",\"body\":\"[inner]\",\"fields\":\"(delta: 300, body: [inner])\",\"label\":\"none\",\"delta\":300},{\"func\":\"emph\",\"body\":\"sequence([outer], [ ], emph(body: [inner]), [ ], [tail])\",\"fields\":\"(\\n  body: sequence([outer], [ ], emph(body: [inner]), [ ], [tail]),\\n)\",\"label\":\"none\",\"delta\":\"absent\"},{\"func\":\"emph\",\"body\":\"[inner]\",\"fields\":\"(body: [inner])\",\"label\":\"none\",\"delta\":\"absent\"}],\"strong_empty\":2,\"emph_empty\":2,\"strong_default\":[\"sequence([outer], [ ], strong(body: [inner]), [ ], [tail])\",\"[inner]\"],\"strong_zero\":[],\"strong_custom\":[],\"strong_body\":[\"[inner]\"],\"emph_body\":[\"[inner]\"]}"); p1339-original-metadata(value) }
#strong[outer #strong[inner] tail] #emph[outer #emph[inner] tail]
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
