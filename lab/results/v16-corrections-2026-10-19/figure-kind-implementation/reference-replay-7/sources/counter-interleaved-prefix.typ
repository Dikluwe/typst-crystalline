#counter(figure.where(kind: table)).update(7)
#figure(table([A]),caption:[A]) <first>
#counter(figure.where(kind: table)).update(n => n + 10)
#figure(table([B]),caption:[B]) <second>
#context [#metadata((first:counter(figure.where(kind:table)).at(<first>),second:counter(figure.where(kind:table)).at(<second>))) <probe>]
