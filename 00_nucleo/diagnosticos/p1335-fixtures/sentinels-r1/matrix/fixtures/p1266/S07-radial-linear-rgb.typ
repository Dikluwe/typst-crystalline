#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)
#let g = gradient.radial((red, 0%), (red, 0%), (rgb(46, 204, 64), 100%), space: color.linear-rgb, center: (50%, 50%), radius: 50%, focal-center: (50%, 50%), focal-radius: 0%)
#let body = rect(width: 120pt, height: 120pt, fill: g)
#let transformed = body
#align(center + horizon, transformed)
