// Boundary fixture: no external bytes are supplied; construction/render is not a positive oracle.
#let body = image("intentionally-absent-p1243.png")
#let p = tiling(body, size: (8pt, 8pt), relative: "self")
#rect(width: 32pt, height: 24pt, fill: p)
