// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/op/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/op/#example
// Citação: "A text operator in an equation."
$ tan x = (sin x)/(cos x) $
$ op("custom",
     limits: #true)_(n->oo) n $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Typst predefines the operators `arccos`, `arcsin`, `arctan`, `arg`, `cos`, `cosh`, `cot`, `coth`, `csc`, `csch`, `ctg`, `deg`, `det`, `dim`, `exp`, `gcd`, `lcm`, `hom`, `id`, `im`, `inf`, `ker`, `lg`, `lim`, `liminf`, `limsup`, `ln`, `log`, `max`, `min`, `mod`, `Pr`, `sec`, `sech`, `sin`, `sinc`, `sinh`, `sup`, `tan`, `tanh`, `tg` and `tr`."
$ sin x + cos y + tan z $
$ lim_(n->oo) 1/n = 0 $
$ gcd(a, b) quad det(A) quad ker(f) $

// Fonte: https://typst.app/docs/reference/math/op/#parameters-text
// Caso derivado da prosa, sem exemplo na doc:
// Citação: "The operator's text."
$ op("argmin")_(x in RR) f(x) $

// Fonte: https://typst.app/docs/reference/math/op/#parameters-limits
// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Whether the operator should show attachments as limits in display mode." Default: `false`
$ op("sup", limits: #false)_(x in S) x $
