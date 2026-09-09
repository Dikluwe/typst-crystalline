#set page(width: 120pt, height: 90pt, margin: 10pt)
#curve(
  fill: rgb("#ff4136"),
  stroke: stroke(paint: rgb("#0074d9"), thickness: 2pt, cap: "round", join: "bevel"),
  curve.move((0pt, 30pt)),
  curve.cubic((20pt, 0pt), (40pt, 0pt), (60pt, 30pt)),
  curve.close(mode: "straight"),
)
