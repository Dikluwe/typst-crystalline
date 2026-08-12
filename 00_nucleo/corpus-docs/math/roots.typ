// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/roots/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/roots/#example
// Citação: "Square and non-square roots."
$ sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1 $
$ root(3, x) $

// Fonte: https://typst.app/docs/reference/math/roots/#functions-root
// Citação: "A general root."
$ root(3, x) $

// Fonte: https://typst.app/docs/reference/math/roots/#functions-sqrt
// Citação: "A square root."
$ sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1 $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "index — none or content — Which root of the radicand to take."
$ root(4, 16) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "radicand — content — RequiredPositional — The expression to take the root of."
$ root(2, a + b) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "index — none or content — Default: `none`"
$ root(#none, x) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "radicand — content — RequiredPositional — The expression to take the square root of."
$ sqrt(x^2 + y^2) $
