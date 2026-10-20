#counter(figure.where(kind:table)).update(n=>panic("NESTED_IMPLICIT_DEMAND"))
#figure([OUTER], caption:[OUTER-CAP], supplement:figure([INNER], kind: table, caption: [INNER-CAP], supplement: [Inner]))