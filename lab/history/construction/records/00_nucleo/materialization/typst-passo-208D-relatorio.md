# Relatório do passo P208D

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-208D.md`.
**Tipo**: encerramento série + decisão de captura.
**Magnitude planeada**: S (~30min-1h) puro **ou** M+ (~3-5h) com
`Content::Context` — decidido em C1.
**Magnitude real**: S (Caminho 1 fixado).
**Marco**: M9c (encerramento série P208; M9c continua com P209+).

---

## §1 O que foi feito

Encerramento da série P208 (4 sub-passos materializados: A + B
+ C + D). Decisão sobre materialização de `Content::Context`
block (sub-mecanismo (ii) análogo a vanilla `ContextElem` +
show-rule, deferido em P208B C1) fixada em C1 como **Caminho 1
— documental puro**: deferido por zero consumers production +
custo M+ + P209 Q-decisões excluem `Selector::Before/After`.
Zero código tocado em P208D; só ADR-0076 + blueprint anotados.

---

## §2 Decisão de captura fixada (Caminho 1) — evidência empírica

**C2 = Caminho 1** — encerramento documental puro (~30min real).

Justificação literal (C1):

- **C1.1 — Zero consumers persistente**: re-grep
  `native_here`/`native_locate` em `01_core/`, `02_shell/`,
  `03_infra/`, `04_wiring/` (filtrando matches do scanner
  homónimo `s.locate(...)` em `lexer/scanner.rs:484+` e
  `ast/expr.rs:371`). Production matches: apenas
  declarações (`pub fn native_here`/`pub fn native_locate`),
  re-exports (`stdlib/mod.rs:36`) e scope registers
  (`eval/mod.rs:608/613`). **Zero callers** de `here()` ou
  `locate()` fora dos próprios definitions + tests.
- **C1.2 — Custo `Content::Context` block**: M+ confirmado.
  Componentes:
  1. Novo variant `Content::Context { body }` em
     `01_core/src/entities/content.rs`.
  2. Hash impl manual (regra cristalina P204B/P204D pattern).
  3. Walk arm em `from_tags` para emitir `Content::Context`
     locatable.
  4. Show-rule equivalente — cristalino tem `show.rs`
     pattern, mas show-rule deferred-eval não está consolidado.
  5. Re-entry de eval em layout-time com
     `EvalContext.current_location` set.
  6. Tests E2E (~5-8).

  Estimativa total: M+ (~3-5h), confirmado per spec C1.2.
- **C1.3 — Roadmap M9c**: P209 Q-decisões fixadas em P207A
  C10 (humano):
  - Q2=γ — `Selector::Where` adiado.
  - Q3=α — `Selector::Regex` + `Selector::Location` (sem
    `Before`/`After`).
  - `Selector::Label` + `And` + `Or` — escopo C4 P207A.

  `Selector::Before/After` (consumers naturais de
  `Content::Context` para query-relative-to-here) **não estão
  no roadmap M9c**. P210+ (Counter/State extras) e P211
  (Outline configurável) também não exigem `Content::Context`.

Critério satisfeito (spec §2 Caminho 1):
> "se C1.1 confirmar zero consumers persistente **E** C1.2
> estimar custo M+ **E** C1.3 mostrar P209+ não desbloqueia
> consumer imediato"

Todas as 3 condições verificadas. Caminho 2
(`Content::Context`) rejeitado por custo sem benefício
proporcional. Caminho 3 (parcial) sem precedente.

**Consumer real virá em passo futuro** (provavelmente P209
ou pós-M9c) quando alguma feature naturalmente exigir
`#context { ... here() ... }` block. Aí a materialização
de `Content::Context` justifica-se empíricamente.

---

## §3 Alterações em código

**Caminho 1 = zero código tocado.**

L0/L1 inalterados. Sem novo prompt L0; sem novos módulos
L1; sem novos tests; sem mudanças em `CountingIntrospector`
(regra empírica P207B §5 **não acionada** em toda a série
P208 — `here()`/`locate()` são stdlib funcs, não trait
methods).

Anotações documentais:

| Ficheiro | Edição |
|----------|--------|
| `00_nucleo/adr/typst-adr-0076-introspector-completion.md` | §Plano de materialização: série P208 transita "EM CURSO" → "✅ MATERIALIZADO 2026-05-12"; P208D anotado com Caminho 1; bloco "Agregado série P208" adicionado com sumário 4 sub-passos + métricas agregadas + pattern emergente "Caminho 1 anti-inflação 4ª aplicação". |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | §3.0quinquies Marca de actualização adicionada (paralelo a §3.0quater P207E): regista série P208 fechada + decisão Caminho 1 + limitações herdadas (`here()` Err sem `current_location`; `locate(<label>)` exige P209). |
| `00_nucleo/materialization/typst-passo-208D-relatorio.md` | Este ficheiro (novo). |

---

## §4 Decisões substantivas

- **Caminho 1 preferido a 2** (Caminho 1 fixado): zero
  consumers + custo M+ + roadmap excludente = pattern
  anti-inflação P205D/P207E aplicável. 4ª aplicação do
  pattern.
- **Sem split D em E** (P208D fica único encerramento):
  P208E reservado se necessário para captura adicional;
  não foi necessário.
- **`EvalContext.current_location` mantém-se como infra
  minimal**: P208B materializou; consumer real virá com
  futuro `Content::Context` block. Não é código morto —
  é interface preparada.
- **Marca blueprint §3.0quinquies**: 5ª marca cirúrgica do
  pattern P204H/P205E/P206E/P207E. Documenta encerramento
  série P208 sem reescrita ampla.
- **ADR-0076 mantém PROPOSTO**: transição PROPOSTO →
  ACEITE fica para P212 (encerramento M9c inteiro). Não
  fechar a ADR aqui é deliberado — séries P209+ continuam.

---

## §5 Métricas

| Métrica | Antes (P208C) | Depois (P208D) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `EvalContext` fields/methods | 5/3 | 5/3 | 0 |
| Stdlib funcs registadas | ~52 | ~52 | 0 |
| Tests workspace | 1907 | 1907 | 0 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| L0 prompts modificados (em P208D) | — | 0 | — |
| L1 ficheiros modificados (em P208D) | — | 0 | — |
| Documentação modificada (em P208D) | — | 3 | +3 |

**Agregado série P208** (P208A diagnóstico + B + C + D):

| Métrica | Pré-P208 | Pós-P208D | Δ série |
|---------|----------|-----------|---------|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `EvalContext` fields | 4 | 5 | +1 (`current_location`) |
| `EvalContext` methods | 2 | 3 | +1 (`with_current_location`) |
| Stdlib funcs novas | — | 2 (`here`, `locate`) | +2 |
| Tests workspace | 1899 | 1907 | +8 |
| L0 prompts novos | — | 0 | 0 |
| L1 ficheiros modificados | — | 3 (3× em B+C; D zero) | +3 distintos |
| ADRs anotadas | — | 1 (`ADR-0076` série) | +1 |
| Blueprint marcas | — | 1 (§3.0quinquies) | +1 |
| Patterns emergentes confirmados | — | 1 (Caminho 1 anti-inflação 4ª aplicação) | +1 |

---

## §6 Encerramento série P208 — sumário literal

Série P208 fechou em 4 sub-passos. Pattern emergente do
projecto (P204A-H, P205A-E, P206A-E, P207A-E) replicado:
diagnóstico-primeiro (A reduzido) → materialização
incremental (B, C) → encerramento documental (D).

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P208A | Diagnóstico-primeiro reduzido | S-M (real ~45min) | Auditoria A1-A6 + decisões C1-C5 + plano P208B-D. 1 ficheiro `00_nucleo/diagnosticos/typst-passo-208A-relatorio.md`. Caminho B fixado: especializado cristalino sem `Tracked<Context>`. |
| P208B | Infra + `here()` materialização | S (real ~1h) | Field `EvalContext.current_location` + setter; `native_here()` em `foundations.rs`; scope register; 4 tests. Sub-mecanismo (i) minimal sem walk advance fixado em C1. |
| P208C | `locate()` materialização trivial | S (real ~40min) | `native_locate()` reusando pattern `native_query` literal; scope register; 4 tests. Limitação herdada: `Selector::Kind` only (P209 desbloqueia `Label`). |
| P208D | Encerramento série + decisão captura | S (real ~30min) | Caminho 1 fixado; ADR-0076 anotado; blueprint §3.0quinquies; relatório este. Zero código tocado. |

**Custo agregado real**: ~3h (estimado ~5-7h per ADR-0076;
~3-4h reestimado per P208A C4). Magnitude **S** efectiva
para série inteira — abaixo do estimado por reuso pattern
P175/P179 + Caminho 1 puro em D.

**Pattern formalizado P208**:

1. **Reuso pattern `native_X` uniforme**: stdlib funcs novas
   seguem `(ctx: &mut EvalContext, args: &Args, world, file,
   figure_numbering) -> SourceResult<Value>`. `here()` e
   `locate()` confirmam estabilidade do pattern P165+.
2. **Stdlib funcs sem trait extension**: `here()`/`locate()`
   são stdlib, não trait `Introspector` methods.
   `CountingIntrospector` L3 wrapper **não toca** — regra
   empírica P207B §5 inactiva por design.
3. **Caminho 1 anti-inflação 4ª aplicação**: P205D
   (`SealedLabelPages`), P207E (page-meta capture), P208B
   C1 (sub-mecanismo i), P208D (`Content::Context`). Pattern
   robusto: materialização honesta quando custo > benefício
   imediato; emerge naturalmente quando consumer real
   aparece.

---

## §7 Próximo sub-passo

**P209 série** — `Selector` minimal extension per
`P207A.div-1` + Q-decisões (humano):
- Q2=γ — `Selector::Where` adiado.
- Q3=α — `Selector::Regex` + `Selector::Location` (sem
  `Before`/`After`).
- C4 P207A — `Selector::Label` + `And` + `Or`.

Plano P209 (per ADR-0076 §Plano de materialização):

- **P209A** — diagnóstico-primeiro Selector enum design.
- **P209B** — `Selector::Label` materialização (S ~1h).
- **P209C** — `Selector::And` + `Or` impls (M ~2-3h).
- **P209D** — encerramento série.

Pré-condição para P209A: série P208 fechada (cumprido em
P208D). Magnitude estimada série P209: S-M (~4-5h).

ADR-0076 mantém `PROPOSTO` até P212 (encerramento M9c
inteiro com auditoria 7 condições + transição PROPOSTO →
ACEITE).

Estado M9c: 2 séries fechadas (P207 + P208). Restam P209+.
