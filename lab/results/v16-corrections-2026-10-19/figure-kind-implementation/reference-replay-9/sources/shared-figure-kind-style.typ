#let f = figure(table([A]))
#show figure: it => [#metadata((kind: repr(it.kind), kind-type: repr(type(it.kind)))) <probe>]
#block({set figure(kind:raw); f})
#block({set figure(kind:image); f})
