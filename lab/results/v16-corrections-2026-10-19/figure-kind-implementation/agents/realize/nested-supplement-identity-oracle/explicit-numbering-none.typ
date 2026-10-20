#show figure:it=>it
#figure([OUTER],numbering:none,caption:[OUTER-CAP],supplement:figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION")))