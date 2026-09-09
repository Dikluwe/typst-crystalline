#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)
#let g = gradient.linear((red, 0%), (red, 0%), (rgb(46, 204, 64), 100%), space: color.linear-rgb, angle: 0.0001deg)
#let body = rect(width: 120pt, height: 120pt, fill: g)
#let transformed = body
#align(center + horizon, transformed)
