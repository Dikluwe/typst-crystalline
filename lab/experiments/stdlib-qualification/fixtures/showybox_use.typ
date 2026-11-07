#import "@preview/showybox:2.0.4": showybox
#showybox[Qualified box]

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
