// A nested frame stays in the coordinate system of the outer HTML island.
#html.frame(box(width: 100pt, height: 50pt, {
  place(top + left, dx: 10pt, dy: 5pt,
    html.frame(box(width: 50pt, height: 30pt, {
      place(top + left, dx: 7pt, dy: 3pt, context {
        assert.eq(here().page(), none)
        assert.eq(here().position(), (x: 17pt, y: 8pt))
      })
    }))
  )
}))
