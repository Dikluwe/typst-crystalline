#set page(width: 230pt, height: 150pt, margin: 10pt)
#table(
  columns: 3,
  table.header(
    pdf.header-cell(
      scope: "both",
      table.cell(colspan: 2)[Group],
    ),
    [Metric],
  ),
  pdf.header-cell(
    scope: "row",
    table.cell(rowspan: 2)[Region],
  ),
  [A], [1],
  [B], [2],
)
