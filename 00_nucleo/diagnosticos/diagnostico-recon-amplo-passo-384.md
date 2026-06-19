# Diagnóstico — recon amplo do projeto (Passo 384)

**Tipo:** read-only (zero `.rs` alterado; lint verde; suíte não tocada). **Data:** 2026-06-19.
**HEAD:** `d47b57e1b` (pós-P383). **Caveat de stack:** `RUST_MIN_STACK=33554432`.

## Critérios fixados (antes das tabelas, para reprodutibilidade)
- **LOC de produção** = total do `.rs` menos o bloco de testes inline (`#[cfg(test)] mod tests` no
  fundo). Heurística do "1.º `#[cfg(test)]`" é enganosa (ex.: `introspect.rs:54` é a probe de teste);
  usei o **último** `^#[cfg(test)]` como início do bloco de testes.
- **Limiar de "monólito"** (fixado pela distribuição real): **> 800 LOC de produção** (tier-1);
  **600-800** é tier-2 (vigiar). A distribuição tem um degrau natural ~800: acima dele há ~9
  ficheiros; entre 600-800 mais ~5; o resto cai abaixo.
- **Atomização** = ADR-0109 (lógica legível sozinha; `match` exaustivo + estático + imports ficam;
  NÃO zerar `content→elements`, NÃO `dyn`). Exclui o que P376-383 já fechou.

---

## Eixo A — Cobertura por domínio (refresh do Inventário 148)

O documento-mestre (`typst-cobertura-vanilla-vs-cristalino.md`) **já estava mantido até ao P299** —
não parado no P148. O agregado vigente (Tabela A/B do doc):

| Domínio | Cobertura (impl+impl⁺) | Última medição |
|---|---:|---|
| Markup syntactic | 78% | P299 |
| `#let`/`#set`/`#show`/import | 62% | P299 |
| Text features | 65% | P292 |
| Math | 92% | P299 |
| Layout | 89% | P221/P223 |
| **Model (structural)** | **50%** | P159G |
| **Visualize** | **54%** | — |
| Foundations stdlib | 67% | — |
| **Introspection** | **83%** | (M3-M9 pós-P160) |
| **User-facing agregado** | **~69%** | P299 |
| Arquitetural agregado | ~82% (Content variants 95%) | P270.3 |

**Delta P299→P384 = 0 features.** O arco P300-383 foi de-baking/morfologia (F-5a/b) + atomização
content-preserving — nenhuma feature nova. **A premissa do plano P384 ("Introspection 17%") está
obsoleta:** o valor real é **83%**. Os mais baixos (alvos naturais de cobertura): **Model 50%**,
**Visualize 54%**.

---

## Eixo B — Monólitos por camada (LOC de produção; descreve, não propõe)

| Camada | Ficheiro | LOC prod | `match` grande? | Candidato atomização? | Notas |
|---|---|---:|:--:|:--:|---|
| L1 | `rules/stdlib/layout.rs` | 1451 | parcial (dispatch de funções) | **sim** | stdlib layout; **sem L0** (DEBT-57) |
| L1 | `rules/stdlib/structural.rs` | 1289 | parcial | **sim** | stdlib structural; sem L0 (DEBT-57) |
| L1 | `entities/content.rs` | ~2314 | **sim** (6 métodos exaustivos) | parcial | hub: enum (76 variantes) + métodos **já delegação-magra** (P375); o gordo é o enum (dados) |
| L1 | `rules/introspect.rs` | ~1168 | **sim** (walk, materialize_time) | **não** (máquina, P383) | walk = recursão/tags; por-elemento já extraído |
| L1 | `rules/stdlib/foundations.rs` | 989 | parcial | **sim** | sem L0 (DEBT-57) |
| L1 | `entities/syntax_node.rs` | ~970 | — | vigiar | nó de sintaxe (dados+API) |
| L1 | `rules/eval/mod.rs` | ~916 | sim (eval de AST) | vigiar | avaliador; máquina de eval |
| L1 | `rules/math/layout/mod.rs` | ~875 | sim (`layout_node`) | **não** (subsistema já atomizado, P382) | MathLayouter |
| L1 | `rules/stdlib/calc.rs` | 809 | — | **sim** | sem L0 (DEBT-57) |
| L1 | `entities/ast/expr.rs` | ~808 | sim | vigiar | AST (dados) |
| L1 | `entities/gradient.rs` | ~799 | — | vigiar | gradientes (dados+cálculo) |
| L1 | `entities/introspector.rs` | ~700 | sim | vigiar | TagIntrospector (máquina de query) |
| L1 | `entities/layout_types.rs` | ~668 | — | vigiar (tier-2) | tipos de layout (dados) |
| L3 | `export/stream.rs` | 685 | — | vigiar (tier-2) | PDF stream |
| L3 | `export/builder.rs` | 671 | — | vigiar (tier-2) | PDF builder |
| L3 | `pipeline.rs` | 606 | — | vigiar (tier-2) | pipeline I/O |
| L2 | `cli.rs` | 324 | — | não | abaixo do limiar |
| L4 | `main.rs` | 144 | — | não | composição mínima |

**`stdlib/mod.rs` (8242 total) NÃO é monólito de produção** — são ~94 linhas de produção + ~8147 de
testes inline. **`lab/` confirmado isolado** (lint sem V10 QuarantineLeak).

**Leitura honesta:** a frente de atomização mais clara e maior é o **cluster `stdlib/`**
(`layout`/`structural`/`foundations`/`calc` + `shapes`/`transforms`/`gradients`/`assert`/`text`/
`math_style`/`figure_image`) — biggest pure-production logic, e coincide com **DEBT-57** (faltam L0s
para ~70 funções stdlib). `content.rs` é grande mas é **o hub** (enum-dados + delegação); os
`entities/*` grandes são sobretudo **dados/tipos** (atomização menos óbvia). `introspect`/`math` já
estão fechados (máquina/subsistema).

---

## Eixo C — DEBTs abertos (9 abertos + 1 triado; máx DEBT-61)

| DEBT | Título | Estado | Falta | Bloqueio |
|---|---|---|---|---|
| DEBT-2 | Closures eager vs lazy capture | parcial | semântica lazy + integração `comemo` | — |
| DEBT-9 | Tracking de paridade contínuo | baseline OK | expandir com novos SyntaxKind | — |
| DEBT-42 | `get_unchecked` no scanner | aberto/bloqueado | infra de benchmark (ADR-0032) | infra de bench |
| DEBT-43 | Linter whitelist crate vs type-level | aberto | whitelisting type-level no `crystalline.toml` | update do `crystalline-lint` |
| DEBT-50 | `#show strong` vs `#set text(bold:)` | latente | distinguir origem strong/emph | dispara com de-bake bold/italic |
| DEBT-55 | Bibliography + Cite (XL) | parcial | CSL styling + crate hayagriva | **ADR-0062** (não criada) |
| DEBT-57 | **L0 ausentes para ~70 funções stdlib** | aberto | L0 para 7 ficheiros stdlib | — |
| DEBT-58 | Primitivos AST fora do modelo D | triado (P329) | rotear variantes (lotes futuros) | gate de lotes |
| DEBT-59 | Flag de erro completo (CLI→L1) | aberto | parse `--full-error` + wiring L3 | — |
| DEBT-60 | Contador heading diverge + "Secção" | aberto | (a) aceitar divergência; (b) fix outline supplement | — (b é lote isolado) |

**DEBT-57 cruza com o Eixo B** (os monólitos stdlib são exatamente os ficheiros sem L0). **DEBT-55**
está bloqueado por **ADR-0062 (PROPOSTO)**.

---

## Eixo D — ADRs `PROPOSTO` (11; README desatualizado)

| ADR | Tópico | Falta para promover |
|---|---|---|
| 0005 | PackageSpec World | (não declarado) |
| 0006 | typst_timing | (não declarado) |
| 0008-0013 | inlining (perf) | materialização (deferida) |
| 0014 | unscanny inlinado | materialização |
| 0015 | ecow removido do parser | materialização |
| 0062 | crate `hayagriva` (bibliography/CSL) | passo de materialização real (desbloqueia DEBT-55) |
| 0066 | Introspection runtime (P160B minimal) | passo de materialização |

**Nota:** o `adr/README.md` está **desatualizado** (última manutenção ~P272; numera até **0094**),
mas o repo tem ADRs até **0109** (a própria ADR de atomização). A distribuição PROPOSTO/EM-VIGOR/
IMPLEMENTADO do README não reflete os ADRs 0095-0109. **Dívida documental:** refrescar o README dos
ADRs.

---

## Eixo E — Divergências vanilla (ADR-0033 + ADR-0054 graded)

- **Registadas:** ~**120 marcadores inline** em produção L1 (`ADR-0054 graded` / `divergência aceite`
  / `scope-out`), em ~33 ficheiros. A convenção **ADR-0054 (graded)** é o canal formal para
  divergências aceites — o código marca-as no ponto de divergência. Bem-documentadas.
- **Não-registadas:** uma auditoria exaustiva de divergências *sem* marcador é, ela própria, um item
  de backlog (não é factível em read-only amplo). **Dívida documental, não código.** O risco é
  baixo: a disciplina graded (ADR-0054) + a rede de caracterização (+11) + paridade vanilla
  (ADR-0033) capturam a maioria no ponto de materialização.

---

## Sanidade
- **lint:** `crystalline-lint .` = **0 violações** (cross-check do verde herdado do P383; sem V10).
- **árvore de código:** **0 `.rs` alterado** (recon read-only).
- **ADR/DEBT:** zero criada/fechado.

**Portões de decisão (§7, fora deste passo):** quais monólitos atomizar primeiro; decisão de crates.
Ver `backlog-priorizado-passo-384.md`.
