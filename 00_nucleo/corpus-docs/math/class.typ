// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/class/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/class/#example
// Citação: "This is useful to treat certain symbols as if they were of a different class, e.g. to make a symbol behave like a relation."
#let loves = math.class(
  "relation",
  sym.suit.heart,
)

$x loves y and y loves 5$

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Note that the latter can always be overridden using `limits` and `scripts`."
$ limits(class("large", union))_0^1 $
$ scripts(class("large", union))_0^1 $

// Casos derivados da tabela de parâmetros (variantes de `class`), sem exemplo na doc:
// Citação: "`"normal"`: The default class for non-special things."
$ class("normal", x) $

// Citação: "`"punctuation"`: Punctuation, e.g. a comma."
$ class("punctuation", x) $

// Citação: "`"opening"`: An opening delimiter, e.g. `(`."
$ class("opening", x) $

// Citação: "`"closing"`: A closing delimiter, e.g. `)`."
$ class("closing", x) $

// Citação: "`"fence"`: A delimiter that is the same on both sides, e.g. `|`."
$ class("fence", x) $

// Citação: "`"large"`: A large operator like `sum`. If the body is a single glyph, this class vertically centers it on the math axis (where the fraction line sits) and stretches it vertically when in `display` style."
$ class("large", x) $

// Citação: "`"relation"`: A relation like `=` or `prec`."
$ class("relation", x) $

// Citação: "`"unary"`: A unary operator like `not`."
$ class("unary", x) $

// Citação: "`"binary"`: A binary operator like `times`."
$ class("binary", x) $

// Citação: "`"vary"`: An operator that can be both unary or binary like `+`."
$ class("vary", x) $
