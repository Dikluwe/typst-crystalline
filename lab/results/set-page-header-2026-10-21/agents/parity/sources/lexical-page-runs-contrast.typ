#page(width:220pt,height:240pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>],footer:[#figure([FOOTER],caption:[FC]) <footer>])[#figure([BODYONE],caption:[B1]) <body>]
#page(width:220pt,height:240pt,margin:60pt,header:none,footer:none)[#figure([BODYTWO],caption:[B2]) <body>
#context [#metadata((
  count: query(figure).len(),
  unique: query(figure).map(it => it.location()).dedup().len(),
  pages: query(figure).map(it => it.location().page()),
  states: query(figure).map(it => counter(figure).at(it.location())),
  final: counter(figure).final(),
  header: query(<header>).map(it => (it.location().page(),counter(figure).at(it.location()))),
  footer: query(<footer>).map(it => (it.location().page(),counter(figure).at(it.location()))),
  body: query(<body>).map(it => (it.location().page(),counter(figure).at(it.location()))),
)) <probe>]
]
