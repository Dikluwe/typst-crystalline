#import "@preview/physica:0.9.3": curl, grad
$ curl (grad f) $

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
