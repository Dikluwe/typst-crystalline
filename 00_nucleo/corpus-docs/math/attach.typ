// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/attach/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/attach/
// Citação: "Subscript, superscripts, and limits."
$ sum_(i=0)^n a_i = 2^(1+i) $

// Fonte: https://typst.app/docs/reference/math/attach/#functions-attach
// Citação: "A base with optional attachments."
$ attach(
  Pi, t: alpha, b: beta,
  tl: 1, tr: 2+3, bl: 4+5, br: 6,
) $

// Fonte: https://typst.app/docs/reference/math/attach/#functions-scripts
// Citação: "Forces a base to display attachments as scripts."
$ scripts(sum)_1^2 != sum_1^2 $

// Fonte: https://typst.app/docs/reference/math/attach/#functions-limits
// Citação: "Forces a base to display attachments as limits."
$ limits(A)_1^2 != A_1^2 $

// Caso derivado da prosa, sem exemplo na doc (parâmetro `inline` de `limits`):
// Citação: "Whether to also force limits in inline equations."
$ limits(A, inline: #false)_1^2 $

// Caso derivado da prosa, sem exemplo na doc (sintaxe dedicada):
// Citação: "Use the underscore (`_`) to indicate a subscript i.e. bottom attachment and the hat (`^`) to indicate a superscript i.e. top attachment."
$ x_1^2 $
