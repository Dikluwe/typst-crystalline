// HTML has no physical pages or coordinates outside a frame.
#context {
  assert.eq(here().page(), none)
  assert.eq(here().position(), none)
  [outside: #(repr(here().page())) / #(repr(here().position()))]
}

// Inside an HTML frame, position has x/y coordinates but no page key.
#html.frame(box(width: 100pt, height: 50pt, {
  place(top + left, dx: 30pt, dy: 20pt, context {
    assert.eq(here().page(), none)
    assert.eq(here().position(), (x: 30pt, y: 20pt))
  })
}))
