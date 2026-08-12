// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/equation/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/equation/
// Citação: "A mathematical equation. Can be displayed inline with text or as a separate block. An equation becomes block-level through the presence of whitespace after the opening dollar sign and whitespace before the closing dollar sign."
#set text(font: "New Computer Modern")

Let $a$, $b$, and $c$ be the side
lengths of a right-angled triangle.
Then, we know that:
$ a^2 + b^2 = c^2 $

Prove by induction:
$ sum_(k=1)^n k = (n(n+1)) / 2 $

// Fonte: https://typst.app/docs/reference/math/equation/#parameters-numbering
// Citação: "How to number block-level equations. Accepts a numbering pattern or function taking a single number."
#set math.equation(numbering: "(1)")

We define:
$ phi.alt := (1 + sqrt(5)) / 2 $ <ratio>

With @ratio, we get:
$ F_n = floor(1 / sqrt(5) phi.alt^n) $

// Fonte: https://typst.app/docs/reference/math/equation/#parameters-number-align
// Citação: "The alignment of the equation numbering."
#set math.equation(numbering: "(1)", number-align: bottom)

We can calculate:
$ E &= sqrt(m_0^2 + p^2) \
    &approx 125 "GeV" $

// Fonte: https://typst.app/docs/reference/math/equation/#parameters-supplement
// Citação: "For references to equations, this is added before the referenced number."
#set math.equation(numbering: "(1)", supplement: [Eq.])

We define:
$ phi.alt := (1 + sqrt(5)) / 2 $ <ratio2>

With @ratio2, we get:
$ F_n = floor(1 / sqrt(5) phi.alt^n) $

// Fonte: https://typst.app/docs/reference/math/equation/#parameters-alt
// Citação: "An alternative description of the mathematical equation."
#math.equation(
  alt: "integral from 1 to infinity of a x squared plus b with respect to x",
  block: true,
  $ integral_1^oo a x^2 + b dif x $,
)

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "By default, block-level equations will not break across pages. This can be changed through `show math.equation: set block(breakable: true)`."
#show math.equation: set block(breakable: true)
$ a + b = c $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Whether the equation is displayed as a separate block."
#math.equation(block: true, $ x + y = z $)
