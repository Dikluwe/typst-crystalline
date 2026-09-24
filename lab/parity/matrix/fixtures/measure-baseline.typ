// measure() exposes the baseline in addition to width and height.
// A rectangle has a font-independent bottom-edge baseline.
#context {
  let plain = measure(rect(width: 10pt, height: 20pt))
  assert.eq(plain.baseline, 20pt)
  [baseline: #(repr(plain.baseline))]
}
