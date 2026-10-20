#figure([OUTER],kind:image,supplement:figure([INNER],kind:table,caption:[INNER-CAP],supplement:[Inner])) <outer>
#figure([SECOND],kind:image,supplement:[SECOND-SUPPLEMENT])<outer>
@outer
#context [#metadata((query(figure).len(), counter(figure.where(kind:table)).final(), counter(figure.where(kind:image)).final(), query(figure).map(it => it.location()).dedup().len())) <probe>]
