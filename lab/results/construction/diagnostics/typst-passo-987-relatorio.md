# Relatório — Passo 987: número de equação a ~206pt do conteúdo em página auto-width

**Estado do código das medições**: HEAD `e490544a6` (P986) + alterações deste
passo (2 agentes). Commit final no fim.
**Gate ADR-0127**: não aplicável — correcção de paridade interna do mecanismo
de diferimento P896 (sem contrato público novo). Fluxo contínuo: L0 editado
primeiro + resselo.

## Fase A — causa confirmada

Repro mínimo (`temp/p987/min.typ`, página auto + 2 equações numeradas):
- Vanilla: conteúdo `E=mc²` acaba x=90.36, `(1)` em x=96.48 — gap = **0.5em**
  (NUMBER_GUTTER). Idem `(2)` junto a `a+b`.
- Cristalino: `(1)` e `(2)` ambos em x=56.75 — **sobrepostos ao conteúdo**,
  porque (a) o fixup usava `page_width − margin − number_width` (a margem
  direita da largura computada da página INTEIRA) e (b) a página auto-width
  encolhia ao conteúdo (a reserva do número não entrava na largura). No doc
  de 30 secções: número em x=496.47, a ~206pt do conteúdo.

Leitura do vanilla (`typst-layout/src/math/mod.rs:209-330`,
`add_equation_number`/`resize_equation`): para largura de região infinita,
linha da equação numerada `W = eq_width + 2 × full_number_width`,
`full_number_width = number_width + NUMBER_GUTTER` (0.5em, `mod.rs:217`);
conteúdo centrado na linha, número no fim da linha; linha centrada na região.
Invariante (independente da recentragem): **`number_x = content_end_x +
gutter`**. Largura finita: número ao fim da região — igual ao cristalino
actual (guardado por teste de não-regressão).

## Fase B — protocolo de dois agentes (como o passo manda)

L0 primeiro (orquestrador): `equation.md` §P987 (revoga o scope-out P813 "não
reserva a calha do número"), `engine/layout.md` §P987 (3 mudanças ao
mecanismo P896).

**Agente A** (testes, `p987_tests` em `engine/layout/tests.rs`): RED
confirmado com a assinatura exacta do bug — T1 `num_x=23.20` sobreposto ao
"𝑎" (esperado 35.30 = content_end+gutter); T2 `page.width=26.60` sem reserva
(esperado 77.20); T3 (página fixa, margem direita) já verde como guarda.

**Agente B** (implementação, só `mod.rs`/`equation.rs`):
1. `pending_equation_numbering` passa a tuplo de 6 campos (+`eq_width`,
   +`applied_offset`).
2. `compute_page_width` inclui `applied_offset + eq_width + 2 ×
   (number_width + gutter)` por equação numerada pendente.
3. Fixup de numeração: `number_x = margin + (usable − eq_width)/2 + eq_width
   + gutter` (fim do conteúdo centrado + calha).

**Revisão do orquestrador**: 3/3 testes p987 verdes; **5784 testes, 0
falhas** (P895/P896 intactos); `crystalline-lint .` 0 violations (só V7 órfão
pré-existente). Doc de várias equações numeradas (o canónico, secção 20+):
cada número acompanha a sua equação — ver Fase C.

## Fase C — Revalidação

Doc canónico (`temp/p987/depois.pdf`): posições x dos números vs vanilla
(`p975-vanilla.pdf`): (1) 296.17 vs 295.05, (2) 294.29 vs 292.86, (3) 299.78
vs 297.26, (4) 304.91 vs 302.39, (5) 329.78 vs 327.26 — todos dentro de
1–2.5pt (ruído de largura de conteúdo acumulado, não do mecanismo; antes:
todos em 496.47). Confirmação visual a 300dpi: (27)/(28) colados ao fim das
suas equações com a calha certa (`temp/p987/eqnum-1.png`).

Benchmark canónico (`benchmark-p987-canonical.py`, antes = release P986):
ratios 0.996–1.017, **média 1.007 — sem regressão**.
