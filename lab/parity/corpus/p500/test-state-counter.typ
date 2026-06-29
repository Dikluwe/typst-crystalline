// P500 — state, counter e context
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
