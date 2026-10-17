#set page(width: 400pt, height: 600pt, margin: 24pt)
#set text(font: "Libertinus Serif", size: 10pt)
#let c = { set rect(fill: red); table.cell[Local] }
#table(columns: 4, ..range(1600).map(_ => c))
