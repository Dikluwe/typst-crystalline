#set page(width: 90pt, height: 34pt, margin: 0pt)
#let p = tiling(red, size: (7pt, 5pt), spacing: (2pt, 3pt), relative: "parent")
#box(width: 90pt, height: 34pt)[
  #place(dx: 3pt, dy: 4pt, rect(width: 30pt, height: 20pt, fill: p, stroke: none))
  #place(dx: 47pt, dy: 4pt, rect(width: 37pt, height: 20pt, fill: p, stroke: none))
]
