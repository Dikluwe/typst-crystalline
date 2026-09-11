#let c = counter("p1339-context")
#c.update(12)
#context assert(c.get() == (12,))
OK
