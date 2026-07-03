// P538i/P540 — #for com destructuring de tuplo via enumerate().
#let items = ("A", "B", "C")
#for (i, x) in items.enumerate() [
  #str(i+1). #x
]
