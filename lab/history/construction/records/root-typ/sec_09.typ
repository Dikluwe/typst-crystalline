#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

#let Res = "Res"
#let Var = "Var"
#let Cov = "Cov"
#let hbar = "ℏ"
#let bra(x) = [⟨#x\|]
#let ket(x) = [\|#x⟩]
#let expval(x) = [⟨#x⟩]

== 9. Funcoes por Partes e Casos

$ f(x) = cases(
  0 & "se" x < 0,
  x^2 & "se" 0 <= x < 1,
  1 & "se" x >= 1
) $

$ |x| = cases(
  x & "se" x >= 0,
  -x & "se" x < 0
) $
