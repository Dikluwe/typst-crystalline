#figure([#context figure([NESTED],kind:raw,supplement:[Raw])],kind:table)<alias>
#context { let prepared=query(<alias>).first(); [#figure([OUTER],kind:image,supplement:prepared)<outer>] }
@outer
@outer
#context [#metadata((query(figure).len(), counter(figure.where(kind:raw)).final(), counter(figure.where(kind:table)).final(), counter(figure.where(kind:image)).final(), query(figure).map(it => it.location()).dedup().len())) <probe>]
