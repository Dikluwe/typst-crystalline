#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

#let Res = "Res"
#let Var = "Var"
#let Cov = "Cov"
#let hbar = "ℏ"
#let bra(x) = [⟨#x\|]
#let ket(x) = [\|#x⟩]
#let expval(x) = [⟨#x⟩]

== 12. Probabilidade e Estatistica

$ P(A | B) = (P(B | A) P(A)) / P(B) $
$ E[X] = sum_(i) x_i P(X = x_i) $
$ Var(X) = E[X^2] - (E[X])^2 $
$ sigma = sqrt(Var(X)) $
$ X ~ N(mu, sigma^2) $
$ Phi(x) = 1 / sqrt(2 pi) integral_(-∞)^x e^(-t^2 / 2) dif t $
$ hat(mu) = 1 / n sum_(i=1)^n X_i $
