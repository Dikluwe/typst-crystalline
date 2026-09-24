// P1492: HTML sem ilha paginada não possui página nem coordenadas físicas.
#context {
  let loc = here()
  assert.eq(loc.page(), none)
  assert.eq(loc.position(), none)
  [outside: #(repr(loc.page())) / #(repr(loc.position()))]
}
