#let c = counter(heading.where())
#context panic("unrelated-error")
#context c.update(12)
#context { assert(c.get() == (12,)); metadata(c.get()) }
