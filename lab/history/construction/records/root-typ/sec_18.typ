#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

#let Res = "Res"
#let Var = "Var"
#let Cov = "Cov"
#let hbar = "ℏ"
#let bra(x) = [⟨#x\|]
#let ket(x) = [\|#x⟩]
#let expval(x) = [⟨#x⟩]

== 18. Expressoes Aninhadas

$ 1 / (1 + 1 / (1 + 1 / (1 + x))) $
$ sqrt(a + sqrt(b + sqrt(c))) $
$ e^(x^2 + y^2) / (x^2 + y^2) $
$ (partial^2 f) / (partial x partial y) $
$ underbrace(overbrace(a + b, "top"), "bottom") $
