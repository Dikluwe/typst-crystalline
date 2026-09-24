// P1491: measure() exposes causal baseline together with width and height.
#context {
  let rect-case = measure(rect(width: 10pt, height: 20pt))
  let text-case = measure([A])
  let box-case = measure(box(baseline: 3pt, rect(width: 10pt, height: 20pt)))
  let empty-case = measure([])
  let composite-case = measure([A#box(baseline: 3pt, rect(width: 10pt, height: 20pt))])

  assert.eq((rect-case.width, rect-case.height, rect-case.baseline), (10pt, 20pt, 20pt))
  assert.eq(text-case.baseline, text-case.height)
  assert(text-case.width > 0pt and text-case.height > 0pt)
  assert.eq((box-case.height, box-case.baseline), (20pt, 17pt))
  assert.eq((empty-case.width, empty-case.height, empty-case.baseline), (0pt, 0pt, 0pt))
  assert.eq((composite-case.height, composite-case.baseline), (20pt, 17pt))
  assert(composite-case.width > box-case.width)

  let shadowed = {
    let measure = x => x + 1
    measure(4)
  }
  assert.eq(shadowed, 5)
  [baseline: #(repr(rect-case.baseline))]
}
