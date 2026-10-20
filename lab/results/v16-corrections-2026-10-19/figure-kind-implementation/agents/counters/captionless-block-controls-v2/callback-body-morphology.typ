#show block: it => {
  assert.eq(it.body.children.len(), 2)
  assert.eq(it.body.children.at(0).func(), strong)
  assert.eq(it.body.children.at(1).func(), parbreak)
  [BLOCKBODYVALID]
}
#figure([*BODY*],kind:image)
