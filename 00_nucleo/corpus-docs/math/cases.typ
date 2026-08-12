// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/cases/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/cases/
// Citação: "A case distinction."
$ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Content across different branches can be aligned with the `&` symbol."
$ f(x) := cases(
  1 & "if" x < 0,
  2 & "if" x >= 0,
) $

// Fonte: https://typst.app/docs/reference/math/cases/#parameters-delim
// Citação: "The delimiter to use."
#set math.cases(delim: "[")
$ x = cases(1, 2) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Otherwise, can be an array containing a left and a right delimiter."
#set math.cases(delim: ("(", "]"))
$ x = cases(1, 2) $

// Fonte: https://typst.app/docs/reference/math/cases/#parameters-reverse
// Citação: "Whether the direction of cases should be reversed."
#set math.cases(reverse: true)
$ cases(1, 2) = x $

// Fonte: https://typst.app/docs/reference/math/cases/#parameters-gap
// Citação: "The gap between branches."
#set math.cases(gap: 1em)
$ x = cases(1, 2) $
