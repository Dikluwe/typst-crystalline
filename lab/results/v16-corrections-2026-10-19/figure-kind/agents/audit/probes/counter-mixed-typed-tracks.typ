#counter(figure.where(kind: table)).update(7)
#counter(figure.where(kind: "table")).update(20)
#figure(table([A]),caption:[A]) <element>
#figure([B],kind:"table",supplement:[Custom],caption:[B]) <name>
#context [#metadata((element:counter(figure.where(kind: table)).at(<element>),name:counter(figure.where(kind: "table")).at(<name>))) <probe>]
