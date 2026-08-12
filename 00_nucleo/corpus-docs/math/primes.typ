// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/primes/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/primes/
// Citação: "Grouped primes."
$ a'''_b = a^'''_b $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "This function has dedicated syntax: use apostrophes instead of primes. They will automatically attach to the previous element, moving superscripts to the next level."
$ a' $

// Caso derivado da prosa, sem exemplo na doc (parâmetro `count`):
// Citação: "The number of grouped primes."
// ACHADO: a forma função `primes(2)` é rejeitada pelo vanilla
// ("expected integer, found content") — o elemento só é usável
// através da sintaxe dedicada de apóstrofes.
$ a primes(2) $
