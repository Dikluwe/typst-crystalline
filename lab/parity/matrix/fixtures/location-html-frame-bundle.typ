// A real mixed bundle keeps paged stores separate from local HTML positions.
#document("paged.pdf")[
  #context {
    assert.eq(target(), "paged")
    assert.eq(here().page(), 1)
    [Paged control.]
  }
]

#document("island.html")[
  HTML control.

  #html.frame(box(width: 100pt, height: 50pt, {
    place(top + left, dx: 30pt, dy: 20pt, context {
      assert.eq(here().page(), none)
      assert.eq(here().position(), (x: 30pt, y: 20pt))
    })
  }))
]
