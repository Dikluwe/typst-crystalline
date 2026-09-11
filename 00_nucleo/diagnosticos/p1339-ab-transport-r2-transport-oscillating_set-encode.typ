#let p1339-original-metadata = metadata
#let metadata(value) = { let encoded = json.encode(value, pretty: false); p1339-original-metadata(value) }
#let c = counter(heading.where())
#context c.update(if c.final().first() == 0 { 1 } else { 0 })
#context metadata(c.final())
