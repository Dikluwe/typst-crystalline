#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

#let Res = "Res"
#let Var = "Var"
#let Cov = "Cov"
#let hbar = "ℏ"
#let bra(x) = [⟨#x\|]
#let ket(x) = [\|#x⟩]
#let expval(x) = [⟨#x⟩]

== 10. Decoradores e Acentos

$ hat(x) $
$ tilde(x) $
$ bar(x) $
$ vec(x) $
$ dot(x) $
$ dot(dot(x)) $
$ underbrace(a + b + c, "soma") $
$ overbrace(a + b + c, "soma") $
$ underbracket(a + b + c) $
$ overbracket(a + b + c) $
$ cancel(a + b) $
$ std.strike(a + b) $
