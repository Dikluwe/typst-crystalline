#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[12]"); p1339-original-metadata(value) }
#let c = counter(heading.where())
#context [#context c.update(12)]
#context { assert(c.get() == (12,)); metadata(c.get()) }
