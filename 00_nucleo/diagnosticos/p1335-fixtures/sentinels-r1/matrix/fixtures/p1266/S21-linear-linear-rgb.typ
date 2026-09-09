#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)
#let g = (gradient.linear((red, 0%), (rgb(46, 204, 64), 11.1111111111111%), (blue.transparentize(100%), 44.4444444444444%), (yellow.transparentize(100%), 100%), space: color.linear-rgb, angle: -75deg)).sharp(4)
#let body = rect(width: 120pt, height: 0pt, fill: g)
#let transformed = body
#align(center + horizon, transformed)
