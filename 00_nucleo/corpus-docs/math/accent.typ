// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/accent/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/accent/#example
// Citação: "In math mode, common accents are also available as named symbols that can be directly called (like functions) to attach them to some content."
$grave(a) = accent(a, `)$ \
$arrow(a) = accent(a, arrow)$ \
$tilde(a) = accent(a, \u{0303})$

// Fonte: https://typst.app/docs/reference/math/accent/#parameters-base
// Citação: "The base to which the accent is applied. May consist of multiple letters."
$arrow(A B C)$

// Fonte: https://typst.app/docs/reference/math/accent/#parameters-size
// Citação: "The size of the accent, relative to the width of the base."
$dash(A, size: #150%)$

// Fonte: https://typst.app/docs/reference/math/accent/#parameters-size
// Citação: "Note that the resulting accent may not have the exact desired size. For example, an arrow may be either a pre-defined short glyph, or a long glyph assembled from building blocks (arrowhead + line) provided by the font."
#for i in range(6) {
  $ arrow(#box(
    width: 0.4em + 0.3em * i,
    fill: aqua,
    height: 0.4em,
  )) $
}

// Fonte: https://typst.app/docs/reference/math/accent/#parameters-dotless
// Citação: "Whether to remove the dot on top of lowercase i and j when adding a top accent."
$hat(dotless: #false, i)$

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Supported accents include:" (tabela de acentos: acute, hat, tilde, macron, dash, breve, dot, dot.double, dot.triple, dot.quad, circle, acute.double, caron, arrow.l, arrow.l.r, harpoon, harpoon.lt)
$ acute(a) hat(a) tilde(a) macron(a) dash(a) breve(a) dot(a) $
$ dot.double(a) dot.triple(a) dot.quad(a) circle(a) acute.double(a) caron(a) $
$ arrow.l(a) arrow.l.r(a) harpoon(a) harpoon.lt(a) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Whether to remove the dot on top of lowercase i and j when adding a top accent." (j também fica dotless por defeito; o exemplo da doc só usa i)
$ hat(j) tilde(i) $
