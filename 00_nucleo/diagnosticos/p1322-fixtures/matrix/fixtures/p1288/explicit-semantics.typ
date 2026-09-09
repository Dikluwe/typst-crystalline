#set page(width: 230pt, height: 150pt, margin: 10pt)
#pdf.table-summary(
  summary: "The first column labels each row; the third header cell is data.",
  table(
    columns: 3,
    table.header(
      level: 1,
      pdf.header-cell(scope: "both")[Key],
      pdf.header-cell(scope: "column")[Value],
      pdf.data-cell[Status data],
    ),
    pdf.header-cell(level: 2, scope: "row")[R1], [A], [ok],
    pdf.header-cell(level: 2, scope: "row")[R2], [B], [warn],
  ),
)
