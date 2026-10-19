#set page(width: 400pt, height: 600pt, margin: 24pt)
#for i in range(128) { place(top + left, dx: calc.rem(i, 8) * 30pt, dy: calc.floor(i / 8) * 14pt, rect(width: 24pt, height: 8pt, fill: blue)) }