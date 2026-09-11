#strong[] #emph[] #strong[#strong[]] #emph[#emph[]]
#context metadata((strong: query(strong.where()).map(it => (body: repr(it.body), fields: repr(it.fields()))), emph: query(emph.where()).map(it => (body: repr(it.body), fields: repr(it.fields()))), ordered: query(selector(strong.where()).or(emph.where())).map(it => (func: repr(it.func()), body: repr(it.body)))))
