#figure([PREEXISTING],kind:table)<inner>
#context { let prepared=query(<inner>).first(); [#figure([OUTER],kind:image,supplement:prepared)<outer>] }
@outer
@outer
#context [#metadata((query(figure).len(), counter(figure.where(kind:table)).final(), counter(figure.where(kind:image)).final(), query(figure).map(it => it.location()).dedup().len())) <probe>]
