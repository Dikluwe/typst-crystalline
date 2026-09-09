// Boundary fixture: vanilla exposes offset/angle; current public crystalline model does not.
#let body = rect(width: 4pt, height: 3pt, fill: red)
#let p = tiling(body, size: (8pt, 8pt), offset: (25%, 50%), angle: 30deg)
#rect(width: 32pt, height: 24pt, fill: p)
