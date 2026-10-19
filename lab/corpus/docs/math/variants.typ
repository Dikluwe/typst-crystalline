// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/variants/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/variants/#functions-sans
// Citação: "Sans-serif font style in math."
$ sans(A B C) $

// Fonte: https://typst.app/docs/reference/math/variants/#functions-frak
// Citação: "Fraktur font style in math."
$ frak(P) $

// Fonte: https://typst.app/docs/reference/math/variants/#functions-mono
// Citação: "Monospace font style in math."
$ mono(x + y = z) $

// Fonte: https://typst.app/docs/reference/math/variants/#functions-bb
// Citação: "Blackboard bold (double-struck) font style in math. For uppercase
// latin letters, blackboard bold is additionally available through symbols of
// the form `NN` and `RR`."
$ bb(b) $
$ bb(N) = NN $
$ f: NN -> RR $

// Fonte: https://typst.app/docs/reference/math/variants/#functions-cal
// Citação: "Calligraphic (chancery) font style in math."
Let $cal(P)$ be the set of ...

// Fonte: https://typst.app/docs/reference/math/variants/#functions-scr
// Citação: "Script (roundhand) font style in math."
$scr(L)$ is not the set of linear
maps $cal(L)$.

// Fonte: https://typst.app/docs/reference/math/variants/#functions-scr
// Citação (exemplo "Recreation using stylistic set 1"): "The other way is
// using font features. For example, the roundhand style might be available in
// a font through the stylistic set 1 (ss01) feature."
#let scr(it) = text(
  stylistic-set: 1,
  $cal(it)$,
)

We establish $cal(P) != scr(P)$.

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Serif (roman) font style in math. This is already the default."
$ serif(A B C) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "These functions are distinct from the `text` function because math
// fonts contain multiple variants of each letter."
// Nota: o vanilla 0.15.1 aceita `text[A]` directamente em math; o cristalino
// rejeita ("unknown variable: text", hint: usar `#text`). ACHADO já registado
// em styles.typ; mantém-se `#text` para compilar nos dois motores.
$ serif(A) != #text[A] $
