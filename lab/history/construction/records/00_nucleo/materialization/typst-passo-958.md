# Passo 958 — `Gamma(z)`/`zeta(s)`/`Phi(x)`/`chi`/`Psi`/`omega` literais em vez do símbolo grego

**Precede este passo**: já catalogado desde P944 §8.3 item 2, reconfirmado por auditoria externa
independente (2026-08-04) com lista limpa: 7 ocorrências, todas só no cristalino — `Phi(𝑥)`,
`chi(𝑀)`, `Gamma(𝑧)`, `zeta(𝑠)`, `Psi(𝑥,`, `chi(𝐺)`, `omega(𝐺)`. O vanilla usa o glifo grego
(Φ, χ, Γ, ζ, Ψ, χ, ω); o cristalino escreve o nome em letras latinas.

**Pré-condição de árvore**: `git status`. Confirmar P957 presente.

---

## Fase A — confirmar a causa

1. Confirmar por que estes nomes específicos (`Gamma`, `zeta`, `Phi`, `chi`, `Psi`, `omega`) não
   resolvem para símbolo, quando outros nomes gregos já funcionam (per P899/902, boa parte do
   alfabeto grego já foi implementada) — ler a tabela de símbolos (`symbols.rs`) e confirmar se
   estes seis/sete nomes estão simplesmente ausentes da tabela, ou se há um conflito (por exemplo,
   `Gamma` colidindo com alguma outra entrada, ou sendo interpretado como identificador de função
   por já existir noutro contexto).
2. Confirmar se o padrão é "maiúscula inicial" (Gamma, Phi, Psi — letras gregas maiúsculas) tendo
   um problema sistemático diferente de minúsculas (zeta, chi, omega) — os sete casos misturam os
   dois grupos, então confirmar se a causa é uma só ou duas.
3. Confirmar contra o vanilla real (`lab/typst-original/`) a lista completa de nomes gregos
   maiúsculos e minúsculos esperados, para não corrigir só os sete já vistos e deixar outros
   ausentes sem notar.

## Fase B — Implementação (TDD directo, mapeamento de tabela — mesma classe de P895/902)

1. Teste com os sete casos reportados, mais qualquer outro nome grego que a Fase A confirmar como
   ausente.
2. Corrigir a tabela.
3. Suíte completa verde, discriminada por crate.
4. Confirmação visual/glifo (`mutool trace`) dos sete casos no documento de 30 secções.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

Recompilar o `.typ` de 30 secções, `compare.py` para confirmar que as secções afetadas (25, 26, 28
— onde estes nomes apareciam) melhoram. Benchmark completo, 7 cenários, `depois/antes`.

## Resultado esperado

- Os sete casos (e quaisquer outros da mesma família) resolvendo para o símbolo grego correto.
- Confirmação visual/glifo, não só teste unitário.
- Benchmark sem regressão.
