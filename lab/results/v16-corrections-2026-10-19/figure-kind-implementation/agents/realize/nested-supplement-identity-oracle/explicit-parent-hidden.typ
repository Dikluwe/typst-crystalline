#show figure:it=>it
#show block:none
#block(figure([OUTER],caption:[OUTER-CAP],supplement:figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION"))))