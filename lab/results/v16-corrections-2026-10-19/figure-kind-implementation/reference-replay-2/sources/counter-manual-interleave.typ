#figure([A])
#counter(figure.where(kind:image)).update(7)
#figure([B],caption:[B]) <b>
#counter(figure.where(kind:image)).update(n=>n*2)
#figure([C],caption:[C]) <c>
#context [#metadata((b: counter(figure.where(kind:image)).at(<b>), c:counter(figure.where(kind:image)).at(<c>), final: counter(figure.where(kind:image)).final())) <probe>]
