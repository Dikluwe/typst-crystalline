#import "@preview/fletcher:0.5.8": diagram, node, edge
#diagram(node((0,0), [A]), edge("-|>"), node((1,0), [B]))

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
