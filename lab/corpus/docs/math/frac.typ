// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/frac/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/frac/
// Citação: "A mathematical fraction."
$ 1/2 < (x+1)/2 $
$ ((x+1)) / 2 = frac(a, b) $

// Caso derivado da prosa, sem exemplo na doc:
// Fonte: https://typst.app/docs/reference/math/frac/#syntax
// Citação: "Multiple atoms can be grouped into a single expression using round grouping parentheses. Such parentheses are removed from the output, but you can nest multiple to force them."
$ (a + b) / c = ((a + b)) / c $

// Caso derivado da prosa, sem exemplo na doc:
// Fonte: https://typst.app/docs/reference/math/frac/#parameters-num
// Citação: "num: The fraction's numerator."
// Citação: "denom: The fraction's denominator."
$ frac(1, 2) $

// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "style: How the fraction should be laid out."
$ frac(x, y, style: "vertical") $
$ frac(x, y, style: "skewed") $
$ frac(x, y, style: "horizontal") $

// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "Setting the default"
#set math.frac(style: "skewed")
$ a / b $

// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "Handling of grouping parentheses"

// Grouping parentheses are removed.
#set math.frac(style: "vertical")
$ (a + b) / b $

// Grouping parentheses are removed.
#set math.frac(style: "skewed")
$ (a + b) / b $

// Grouping parentheses are retained.
#set math.frac(style: "horizontal")
$ (a + b) / b $

// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "Different styles in inline vs block equations"

// This changes the style for inline equations only.
#show math.equation.where(block: false): set math.frac(style: "horizontal")

This $(x-y)/z = 3$ is inline math, and this is block math:
$ (x-y)/z = 3 $

// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "Use LaTeX-like convention"

// Change the default style.
#set math.frac(style: "horizontal")
// Define a shorthand with the original style.
#let frac = math.frac.with(style: "vertical")

$ p/q = frac(p, q) $

// The shadowed definition can still be accessed.
#assert.eq($p/q$, $std.math.frac(p, q)$)

// Caso derivado da prosa, sem exemplo na doc:
// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "\"skewed\": Numerator and denominator separated by a slash."
$ frac(1, 2, style: "skewed") $

// Caso derivado da prosa, sem exemplo na doc:
// Fonte: https://typst.app/docs/reference/math/frac/#parameters-style
// Citação: "\"horizontal\": Numerator and denominator placed inline and parentheses are not absorbed."
$ frac(1, 2, style: "horizontal") $
