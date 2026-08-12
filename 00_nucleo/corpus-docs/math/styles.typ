// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/styles/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/styles/#functions-upright
// Citação: "Upright (non-italic) font style in math."
$ upright(A) != A $

// Fonte: https://typst.app/docs/reference/math/styles/#functions-bold
// Citação: "Bold font style in math."
$ bold(A) := B^+ $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Italic font style in math. For roman letters and greek lowercase letters, this is already the default."
$ italic(x + y) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "These functions are distinct from the `text` function because math fonts contain multiple variants of each letter."
// Nota: o vanilla 0.15.1 aceita `text[A]` directamente em math; o cristalino rejeita
// ("unknown variable: text", hint: usar `#text`). ACHADO de divergência registado no passo.
$ upright(A) != #text[A] $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "The content to style." (parâmetro `body`, Required Positional, comum a upright, italic e bold)
$ upright(A B C) $
