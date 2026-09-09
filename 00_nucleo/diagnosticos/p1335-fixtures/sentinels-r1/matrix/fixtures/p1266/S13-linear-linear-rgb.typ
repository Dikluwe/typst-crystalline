#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)
#let g = (gradient.linear((black, 0%), (white, 100%), space: color.linear-rgb, angle: 0deg)).sharp(2)
#let body = rect(width: 120pt, height: 120pt, fill: g)
#let transformed = body
#align(center + horizon, transformed)
