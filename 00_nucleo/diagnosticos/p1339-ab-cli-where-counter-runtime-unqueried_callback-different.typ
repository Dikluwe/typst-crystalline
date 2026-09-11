#let p1339-original-metadata = metadata
#let metadata(value) = { assert(json.encode(value, pretty: false) != "[\"survived\",[1]]"); p1339-original-metadata(value) }
#counter(heading.where()).update(n => panic("unused-callback"))
#context metadata(("survived", counter(page).final()))
