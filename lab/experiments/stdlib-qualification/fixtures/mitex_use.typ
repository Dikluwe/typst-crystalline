#import "@preview/mitex:0.2.4": mi
#mi("x + 1")

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
