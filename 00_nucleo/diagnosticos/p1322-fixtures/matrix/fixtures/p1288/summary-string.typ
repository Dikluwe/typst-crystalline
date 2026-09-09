#set page(width: 180pt, height: 120pt, margin: 10pt)
#pdf.table-summary(
  summary: "First column is a key; second column is its value.",
  table(
    columns: 2,
    [Key], [Value],
    [A], [1],
  ),
)
