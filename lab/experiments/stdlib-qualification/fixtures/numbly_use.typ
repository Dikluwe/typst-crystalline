#import "@preview/numbly:0.1.0": numbly
#set heading(numbering: numbly("Part {1:A}.", "{1:A}.{2}."))
= Alpha
== Beta

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
