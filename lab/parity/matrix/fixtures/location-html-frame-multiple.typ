// Two independent Locations in the same intrinsic HTML island.
#html.frame(box(width: 100pt, height: 50pt, {
  place(top + left, dx: 10pt, dy: 5pt, context {
    let loc = here()
    assert.eq(here(), loc)
    assert.eq(loc.page(), none)
    assert.eq(loc.position(), (x: 10pt, y: 5pt))
  })
  place(top + left, dx: 40pt, dy: 25pt, context {
    let loc = here()
    assert.eq(here(), loc)
    assert.eq(loc.page(), none)
    assert.eq(loc.position(), (x: 40pt, y: 25pt))
  })
}))
