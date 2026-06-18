# Relatório P364 — F-5a de-bake: heading + equation numbering (fonte única)

> **Desfecho.** O de-bake **heading + equation** está **feito**: o campo assado
> `numbering_active` foi **removido** de `HeadingElem`/`EquationElem`; o gate de numeração
> vive **só na chain** (`custom("X.numbering")`, transportado por `Content::Styled`). Os
> produtores de produção (markup/eval) criam o elemento **simples** — a fatia-1 carrega o
> gate; os consumidores (layout + introspect, sobre o introspect-chain do P363) **leem o
> gate da chain**. **Content-preserving**: a rede de caracterização (+11) e a suíte inteira
> passam (**2738**, 0 falhas). **`figure` fatiado para o P365** (decisão do dono na Trava —
> arrasta a assinatura partilhada das natives). lint **0/0**.

**HEAD**: pós-P360 (8916829c2) — o bloco **P362 + P363 segue não-commitado** na árvore; o
P364 está **em cima** dele (a decisão de commit é do dono). **Branch**: Tekt.
**Justificativa = princípio (fonte única / atomização)**, decidido pelo dono (P362), não
demanda da lente. **Escopo aprovado na Trava**: heading + equation; figura → P365.

---

## Fase A — a varredura (medida; `file:line`)

Os 3 produtores de produção de numbering passam **todos** pelo transporte `Content::Styled`
da fatia-1 (estão na cauda que ela embrulha):

| Produtor | `file:line` | Classe |
|---|---|---|
| heading (eval) | `eval/markup.rs:96` | **[produção, transporte]** |
| equation (eval) | `eval/mod.rs:564` | **[produção, transporte]** |
| figure (eval) | `eval/closures.rs:79-83` | **[produção, transporte]** (P365) |
| heading/equation (fixtures) | `introspect.rs` 6 + `layout/tests.rs` 31 ≈ **37** | **[fixture, direto]** |

**→ zero produtores de produção a rotear** (a chain já carrega o custom; P363 provou o gate
disponível sob `Styled`). A dupla existia **só** porque o eval **também** bakava o campo.

**Gate ≠ número (confirmado, P335/DEBT-60):** `layout/mod.rs:714` lê `h.numbering_active` (o
**gate**) e **só então** `formatted_counter_at` (o **número**). O contador incondicional
(`apply_hierarchical_at`, P335) não é gateado. O de-bake troca **só a leitura do gate**.

---

## Estágio L0 (Trava aprovada pelo dono)

`f_fronteira_e1.md` **§3a.9** (nova secção) — a fatia F-5a: produção [transporte] (zero
roteamento); consumidores leem `chain.custom("X.numbering")`; gate ≠ número; fixtures
roteados ao transporte; figura → P365. Addendum pós-execução: **a forma de aninhamento**
(produção põe o transporte **fora** do `Labelled`). Hash sincronizado (9 backings;
`--fix-hashes`) **antes** do código. Lint 0/0. Trava aprovada ("Aprovado").

---

## Estágios 1–2 — o de-bake (`file:line`)

**Campo + construtores removidos (fonte única):**
- `entities/elements/heading.rs`: campo `numbering_active` + `new_numbered` **removidos**;
  `new` simplificado; clones `map_content`/`map_text` limpos.
- `entities/elements/equation.rs`: idem; `to_payload` põe `numbering_active: false`
  (placeholder — o gate é sourced da chain no walk).
- `entities/content.rs`: `heading_numbered`/`equation_numbered` **reescritos** para produzir
  a **forma de transporte** (`Content::Styled` + `push_custom("X.numbering", Bool(true))`) —
  a forma canônica de produção, usada pelos fixtures (campo removido → o construtor não pode
  mais setá-lo). `morph_canon`: arms dedicados de heading/equation **removidos** — o arm
  `Styled` semanticamente-vazio (transparente) já os subsume → **mecanismo único** (o custom
  é render, não morfologia, P345 N1).

**Produção cria elemento simples (a fatia-1 carrega o gate):**
- `eval/markup.rs:90-100` → `Content::heading(level, body)` (branch de baka removido).
- `eval/mod.rs:558-568` → `Content::equation(body, block)` (idem).

**Consumidores leem a chain:**
- `layout/mod.rs:714` (heading) → `matches!(self.chain.custom("heading.numbering"), Some(Bool(true)))`.
- `layout/mod.rs:816` (equation) → idem `equation.numbering` (mantém `block && numbering`).
- `introspect.rs` walk Heading (~862) → lê `chain.custom("heading.numbering")` (sobre o
  introspect-chain P363); passa a `compute_heading_auto_toc`.
- `introspect.rs` walk top (~786) → o payload da equation **tira `numbering_active` da
  chain** no momento da emissão (a consumição posterior `from_tags`/`populate_intr` não tem
  chain). Fonte única.

---

## Asserções de fixture que viraram (declaradas, S5b)

Nenhuma asserção de **comportamento** mudou. As mudanças de fixture:

1. `eval/tests.rs` — 3 probes de teste (`find_heading_numbered`, `collect_headings_numbered`,
   `find_equation_numbered`) passaram a **threadar o gate do `Styled` custom** (helper
   `styled_custom_bool`) em vez de ler um campo do elemento. **As asserções ficam idênticas**
   (`Some(true)`/`Some(false)`) e agora **verificam o transporte de produção**: `f2s1`/`f2s2`
   provam que a fatia-1 embrulha o heading/equação numerado (se não embrulhasse, o probe veria
   `false`). ✅
2. `introspect.rs:3119` (`consumer_layouter_equation_activa_via_introspector`) — o fixture
   construía `Labelled{ equation_numbered }` = `Labelled{ Styled{ Equation } }` (transporte
   **dentro** do label). **Medição:** produção põe o transporte **fora** (`Styled{ Labelled{
   Equation } }`), e o `compute_labelled` inspeciona o **tipo do alvo** — com o transporte
   dentro, o alvo é `Styled` e o arm-Equation não dispara. Fixture **roteado à forma de
   produção** (transporte fora do label). Declarado. ✅

(Considerei tornar o transporte transparente também ao `label_from_parent`; **revertido** —
a forma de produção não o exige e não fechava o `compute_labelled` de qualquer modo;
mínimo, ADR-0108.)

---

## Gates (todos verdes)

```
build: workspace limpo (warnings de import = pré-existentes, em ficheiros não tocados pelo
  de-bake; baseline tinha 5, P364 não adicionou nenhum).
suíte (RUST_MIN_STACK=33554432): typst-core 2738 (0 falhas); demais 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (oráculo = rede de caracterização +11 / vanilla):
  - heading + equation: output idêntico ao de antes (paridade), lendo o gate SÓ da chain.
  - fonte única: campo assado numbering_active REMOVIDO de heading/equation; só a chain
    carrega o gate.

INTACTOS (confirmado): o NÚMERO (formatted_counter_at) + o contador incondicional P335
  (apply_hierarchical_at) — não tocados; α/caso 2, caso 4, morph ==/morph_canon (arm Styled
  transparente subsume os arms removidos), flag P350c, Marco G — não tocados; FIGURA (campo
  numbering + assinatura partilhada das natives) — intacta, P365.

lente (instrumento): não re-corrida (tekt-cargo-dsm é externa, commit 98d8f9e, fora da
  árvore). Analiticamente: o de-bake troca leitura-de-campo por leitura-de-chain, ambas em
  `rules/`; nenhuma aresta content→elements adicionada (content.rs segue chamando
  HeadingElem::new/EquationElem::new). content→elements = 66 esperado inalterado.
perf (depois): 0.7063 s ± 0.0081 (n=19). Cross-session vs P363 (0.6809 s) — a deriva é
  ambiental (o rig avisa: comparar só na mesma sessão); o de-bake é behavior-neutral
  (chain.custom() vs campo, ambos O(1) sobre delta.custom minúsculo). Sem regressão.
L0 (critério 5): f_fronteira_e1.md §3a.9 + addendum, hashes sincronizados ANTES do código,
  Trava aprovada.
```

---

## Estado / próximo

**Tocados (P364):** `f_fronteira_e1.md` (§3a.9 + addendum, L0) + 9 backings (hash sync);
`heading.rs`, `equation.rs`, `content.rs`, `markup.rs`, `eval/mod.rs`, `layout/mod.rs`,
`introspect.rs`, `eval/tests.rs`. Árvore limpa fora de docs/lab.

**Bloco não-commitado:** P362 (auditoria) + P363 (introspect-chain) + P364 (este de-bake) —
todos na árvore. A decisão de commit (um lote ou separados) é do dono.

**Próximo (P365 — decisão do dono):** o de-bake da **figura** — o gate `figure.numbering`
(padrão `Option<String>`) + a **assinatura partilhada das natives** (`figure_numbering` em
~10 fns de `stdlib/structural.rs`) + `FigureElem.numbering`. O ponto 4 (`TextStyle`) é o
**F-5b**. **Termina aqui — não emendo o passo seguinte (Trava 5).**

## Fora de escopo (confirmado)

A **figura** (P365); o ponto 4 / `TextStyle` (F-5b); o contador/P335; F-6; o mapa aberto p/
`#set` de user-props; Marco G; DEBT-59 (flag CLI); qualquer toque no α / `morph_canon` / `==`
ou na flag P350c.
