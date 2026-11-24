// comentario mais longo antes dos titulos
#set page(width:400pt,height:600pt)
#set heading(numbering: "1.")
#outline(title: none)
= Alpha <a>
See @b.
#pagebreak()
#pagebreak()
= Beta <b>
#context {
  let pos = query(<b>).first().location().position()
  /* cyan */ /* é */ [Observed page #pos.page.]
}
