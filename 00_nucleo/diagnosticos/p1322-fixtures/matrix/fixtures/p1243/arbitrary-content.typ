// Boundary fixture: vanilla accepts arbitrary Content; current TilingBody does not represent it.
#let body = rect(width: 4pt, height: 3pt, fill: red)
#let p = tiling(body, size: (8pt, 8pt), relative: "self")
#rect(width: 32pt, height: 24pt, fill: p)
