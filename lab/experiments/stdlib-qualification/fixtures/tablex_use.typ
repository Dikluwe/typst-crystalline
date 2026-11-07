#import "@preview/tablex:0.0.9": tablex, cellx
#tablex(columns: 2, [A], [B], [C], [D])

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
