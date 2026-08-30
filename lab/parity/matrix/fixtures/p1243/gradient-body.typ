// Boundary fixture: current constructor L0 explicitly rejects Gradient.
#let body = gradient.linear(red, blue)
#let p = tiling(body, size: (8pt, 8pt), relative: "self")
#rect(width: 32pt, height: 24pt, fill: p)
