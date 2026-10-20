#let inner=context figure([INNER],kind:table,caption:[INNER-CAP],supplement:[Inner])
#let outer=figure([OUTER],kind:image,caption:[OUTER-CAP],supplement:inner)
#outer
#outer
#context [#metadata((query(figure).len(),counter(figure.where(kind:table)).final(),counter(figure.where(kind:image)).final(),query(figure).map(it=>it.location()).dedup().len())) <probe>]
