#let c = counter(heading.where(level: 1))
#let step(outer, n) = {
  let witness = (outer: outer,)
  n + witness.outer.a
}
#context [
  #c.update(step.with((a: 1,)))
  #c.get()
]
