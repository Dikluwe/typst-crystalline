#show figure:it=>it
#figure([OUTER],caption:[OUTER-CAP],supplement:it=>figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION")))