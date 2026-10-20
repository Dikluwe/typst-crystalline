#show figure:it=>it
#figure([OUTER],numbering:none,caption:[OUTER-CAP],supplement:it=>figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION")))