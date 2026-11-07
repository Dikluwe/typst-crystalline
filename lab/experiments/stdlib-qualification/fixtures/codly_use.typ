#import "@preview/codly:1.3.0": *
#show: codly-init.with()
#raw("let answer = 42;\nanswer", lang: "typ", block: true)

#let qualification_ok = true
#let qualification_inputs = sys.inputs

Probe #sys.inputs.at("probe", default: "none")
