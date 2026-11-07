#import "@preview/cetz:0.5.2" as cetz
#cetz.canvas({
 import cetz.draw: *
 line((0, 0), (2, 1))
 circle((1, 0), radius: 0.25)
})

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
