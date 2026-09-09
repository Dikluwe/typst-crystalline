#set page(width: 180pt, height: 120pt, margin: 10pt)
#curve(
  fill: rgb("#ff4136"),
  fill-rule: "even-odd",
  stroke: 2pt + rgb("#0074d9"),
  curve.move((0pt, 30pt)),
  curve.line((30pt, 0pt)),
  curve.cubic((45pt, 0pt), (55pt, 30pt), (70pt, 30pt)),
  curve.quad((85pt, 60pt), (100pt, 30pt)),
  curve.close(),
  curve.move((20pt, 25pt)),
  curve.line((30pt, 15pt)),
  curve.line((40pt, 25pt)),
  curve.close(),
)
