// P1490: preserve the public morphology of automatic and explicit stroke slots.
#let automatic = stroke()
#let thickness = stroke(thickness: 1pt)
#let paint = stroke(paint: black)
#let both = stroke(paint: black, thickness: 1pt)

#assert.eq(repr(automatic), "stroke()")
#assert.eq(repr(thickness), "1pt")
#assert.eq(repr(paint), "luma(0%)")
#assert.eq(repr(both), "1pt + luma(0%)")
#assert.eq(
  repr((automatic, thickness, paint, both)),
  "(stroke(), 1pt, luma(0%), 1pt + luma(0%))",
)

// The constructor currently accepts named slots, not positional shorthand.
#assert.eq(eval(repr(automatic)), automatic)
#assert.eq(stroke(thickness: eval(repr(thickness))), thickness)
#assert.eq(stroke(paint: eval(repr(paint))), paint)
#assert.eq(
  stroke(
    paint: eval(repr(paint)),
    thickness: eval(repr(thickness)),
  ),
  both,
)

#repr(automatic)
