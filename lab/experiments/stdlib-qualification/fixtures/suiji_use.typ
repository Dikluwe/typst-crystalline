#import "@preview/suiji:0.3.0": gen-rng, integers
#let (rng, values) = integers(gen-rng(42), low: 0, high: 10, size: 3)
#values.map(str).join(",")

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
