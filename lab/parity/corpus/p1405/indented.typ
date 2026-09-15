#set heading(numbering: "1.")
#outline()

= Alpha
== Beta

#context {
  let entries = query(outline.entry)
  let first = entries.at(0)
  repr(outline.entry.indented(first, [P], [I]))
}
