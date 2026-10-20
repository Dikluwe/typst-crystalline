#show block:none
#figure([OUTER],kind:image,supplement:figure([INNER],kind:table,caption:[INNER-CAP],supplement:[Inner])) <outer>
#block[@outer]
#context [#metadata((query(figure).len(), counter(figure.where(kind:table)).final(), counter(figure.where(kind:image)).final(), query(figure).map(it => it.location()).dedup().len())) <probe>]
