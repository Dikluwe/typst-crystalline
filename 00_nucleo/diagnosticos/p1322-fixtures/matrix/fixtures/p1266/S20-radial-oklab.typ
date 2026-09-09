#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)
#let g = gradient.radial((red.transparentize(100%), 0%), (rgb(46, 204, 64), 33.3333333333333%), (blue, 66.6666666666667%), (yellow, 100%), space: color.oklab, center: (50%, 50%), radius: 50%, focal-center: (50%, 50%), focal-radius: 0%)
#let body = rect(width: 0pt, height: 120pt, stroke: 8pt + g)
#let transformed = body
#align(center + horizon, transformed)
