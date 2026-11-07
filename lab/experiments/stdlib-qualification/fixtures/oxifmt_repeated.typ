#import "@preview/oxifmt:1.0.0" as p0
#import "@preview/oxifmt:1.0.0" as p1
#import "@preview/oxifmt:1.0.0" as p2
#import "@preview/oxifmt:1.0.0" as p3
#import "@preview/oxifmt:1.0.0" as p4
#import "@preview/oxifmt:1.0.0" as p5
#import "@preview/oxifmt:1.0.0" as p6
#import "@preview/oxifmt:1.0.0" as p7

#let qualification_ok = true
#let qualification_inputs = sys.inputs
#p0.strfmt("Repeat {}", 8)
Probe #sys.inputs.at("probe", default: "none")
