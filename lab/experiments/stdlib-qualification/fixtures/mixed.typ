#import "@preview/cetz:0.5.2" as cetz
#import "@preview/oxifmt:1.0.0" as oxifmt
#import "@preview/tablex:0.0.9" as tablex
#import "@preview/codly:1.3.0" as codly
#import "@preview/fletcher:0.5.8" as fletcher
#import "@preview/showybox:2.0.4" as showybox
#import "@preview/physica:0.9.3" as physica

#let qualification_ok = true
#let qualification_inputs = sys.inputs
Mixed packages
Probe #sys.inputs.at("probe", default: "none")
