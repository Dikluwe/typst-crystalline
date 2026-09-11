#let c = counter(heading.where())
#context c.update(if c.final().first() == 0 { 1 } else { 0 })
#context metadata(c.final())
