#import "@preview/oxifmt:1.0.0": strfmt
#strfmt("Value {} / {:#06x}", 42, 255)

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
