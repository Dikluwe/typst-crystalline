// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/underover/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/underover/#functions-underline
// Citação: "A horizontal line under content."
$ underline(1 + 2 + ... + 5) $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-overline
// Citação: "A horizontal line over content."
$ overline(1 + 2 + ... + 5) $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-underbrace
// Citação: "A horizontal brace under content, with an optional annotation below."
$ underbrace(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-overbrace
// Citação: "A horizontal brace over content, with an optional annotation above."
$ overbrace(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-underbracket
// Citação: "A horizontal bracket under content, with an optional annotation below."
$ underbracket(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-overbracket
// Citação: "A horizontal bracket over content, with an optional annotation above."
$ overbracket(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-underparen
// Citação: "A horizontal parenthesis under content, with an optional annotation below."
$ underparen(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-overparen
// Citação: "A horizontal parenthesis over content, with an optional annotation above."
$ overparen(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-undershell
// Citação: "A horizontal tortoise shell bracket under content, with an optional annotation below."
$ undershell(0 + 1 + dots.c + n, n + 1 "numbers") $

// Fonte: https://typst.app/docs/reference/math/underover/#functions-overshell
// Citação: "A horizontal tortoise shell bracket over content, with an optional annotation above."
$ overshell(0 + 1 + dots.c + n, n + 1 "numbers") $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "A horizontal brace under content, with an optional annotation below."
// (parâmetro `annotation` tem Default: `none`; a doc não mostra chamada sem anotação)
$ underbrace(x + y + z) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "A horizontal brace over content, with an optional annotation above."
// (parâmetro `annotation` tem Default: `none`; a doc não mostra chamada sem anotação)
$ overbrace(x + y + z) $
