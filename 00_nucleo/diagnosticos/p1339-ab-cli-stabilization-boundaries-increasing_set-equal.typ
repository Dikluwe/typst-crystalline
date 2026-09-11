#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) == "[4]"); p1339-original-metadata(value) }
#let c = counter(heading.where())
#context c.update(c.final().first() + 1)
#context metadata(c.final())
