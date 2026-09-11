#metadata(0)<opaque>
#let c = counter(heading.where())
#context { let _ = query(<opaque>); c.update(c.final().first() + 1) }
