#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

#let Res = "Res"
#let Var = "Var"
#let Cov = "Cov"
#let hbar = "ℏ"
#let bra(x) = [⟨#x\|]
#let ket(x) = [\|#x⟩]
#let expval(x) = [⟨#x⟩]

== 22. Delimitadores Escalaveis

$ lr((a/b)) $
$ lr([a/b]) $
$ lr({a/b}) $
$ lr(|a/b|) $
$ lr(⌊a/b⌋) $
$ lr(⌈a/b⌉) $
$ lr(chevron.l a/b chevron.r) $
$ lr(\]a/b\[) $
