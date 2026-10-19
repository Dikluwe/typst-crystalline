#let h=heading[A]
#show heading:it=>par(it.body)
#show par:it=>{metadata("P");it}
#([#h]+[#h])