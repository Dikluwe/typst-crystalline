// P500 — page com header, footer, numbering e context
#set page(
  header: [_Cabeçalho_],
  footer: context [Página #counter(page).display()],
  numbering: "1",
)
#lorem(50)
