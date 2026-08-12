// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/vec/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/vec/
// Citação: "A column vector."
$ vec(a, b, c) dot vec(1, 2, 3)
    = a + 2b + 3c $

// Fonte: https://typst.app/docs/reference/math/vec/#parameters-delim
// Citação: "Can be a single character specifying the left delimiter, in which case the right delimiter is inferred. Otherwise, can be an array containing a left and a right delimiter."
#set math.vec(delim: "[")
$ vec(1, 2) $

// Fonte: https://typst.app/docs/reference/math/vec/#parameters-align
// Citação: "The horizontal alignment that each element should have."
#set math.vec(align: right)
$ vec(-1, 1, -1) $

// Fonte: https://typst.app/docs/reference/math/vec/#parameters-gap
// Citação: "The gap between elements."
#set math.vec(gap: 1em)
$ vec(1, 2) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Content in the vector's elements can be aligned with the `align` parameter, or the `&` symbol."
$ vec(1&, &10) $

// Caso derivado da prosa, sem exemplo na doc (delimiter como array esquerdo/direito):
// Citação: "Otherwise, can be an array containing a left and a right delimiter."
#set math.vec(delim: ("[", "]"))
$ vec(1, 2) $
