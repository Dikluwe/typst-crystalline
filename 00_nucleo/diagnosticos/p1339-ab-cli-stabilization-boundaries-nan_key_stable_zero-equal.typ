#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) == "[0]"); p1339-original-metadata(value) }
#let c = counter(heading.where(level: float.nan))
#context c.update(12)
#context { assert(c.get() == (0,)); metadata(c.get()) }
