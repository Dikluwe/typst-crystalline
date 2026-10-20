#figure([OUTER], caption:[OUTER-CAP], supplement:it=>figure([INNER], kind: table, caption: [INNER-CAP], supplement: [Inner]))
#context [#metadata((query(figure).len(),counter(figure.where(kind: table)).final(),counter(figure.where(kind: image)).final())) <probe>]
