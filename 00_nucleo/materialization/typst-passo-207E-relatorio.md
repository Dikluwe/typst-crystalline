# Relatório do passo P207E

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-207E.md`.
**Tipo**: encerramento série + decisão de captura.
**Magnitude planeada**: S (~30min-1h) documental puro **ou** M
(~2-3h) com captura — decidido em C1.
**Magnitude real**: S (Caminho 1 fixado).
**Marco**: M9c (encerramento série P207; M9c continua com P208+).

---

## §1 O que foi feito

Encerramento da série P207 (4 sub-passos materializados: B + C
+ D + E). Decisão de captura runtime de `page_numbering`/
`page_supplement` fixada em C1+C2 como **Caminho 1 — documental
puro**: captura deferida por zero consumers production + custo
M (refactor cross-modular). Zero código tocado em P207E; só
ADR-0076 + blueprint anotados.

---

## §2 Decisão de captura fixada (Caminho 1) — evidência empírica

**C2 = Caminho 1** — encerramento documental puro (~30min real).

Justificação literal (C1):

- **C1.1 — Ponto de captura**: `Layouter::layout_page` ou
  análogo. Inspecção empírica revela `01_core/src/entities/layout_types.rs:338`
  `PageConfig { width, height, margin }` — **sem campos
  `numbering`/`supplement`**. `Page` (closed snapshot,
  `layout_types.rs:359`): também só `width`/`height`/`items`.
  Cristalino não tem infraestrutura para esses metadata.
- **C1.2 — Custo empírico**: captura exige refactor
  cross-modular:
  1. Adicionar `numbering: Option<EcoString>` e
     `supplement: Option<Content>` a `PageConfig`.
  2. Adicionar handling em `stdlib::set_page(numbering,
     supplement)` no eval.
  3. Adicionar walk capture em `Layouter::layout_page` ou
     `finish_page`.
  4. Adicionar storage Vec em `LayouterRuntimeState`.
  5. Update `Layouter::finish` para popular
     `PageStore::from_runtime(total, numberings,
     supplements)` em vez de `from_total_pages` minimal.

  Magnitude: **M (~2-3h)** — não trivial (~10-20 LOC).
- **C1.3 — Consumers reais**: re-grep `page_numbering`/
  `page_supplement` em production confirmou **zero
  consumers** caller — todas as 17 ocorrências são "o próprio
  método" (trait def + TagIntrospector impl + tests +
  CountingIntrospector L3 wrapper). Nenhuma chamada externa.

Critério satisfeito (spec §2 C1 → C2):
> "Caminho 1 — encerramento documental puro (~30 min): se C1.3
> confirmar zero consumers reais **E** C1.2 estimar captura ≥
> M ou exigir refactor walk."

Ambas as condições verificadas. Caminho 2 rejeitado (custo M
sem consumer; spec §5 risco 1 explicitamente desaconselha).
Caminho 3 (parcial) rejeitado pela mesma razão (numberings
isolado teria mesmo custo M para PageConfig+walk).

**Consumer real virá com P208 (`here()`/`locate()`)**: spec
§2 nota explícita "emerge naturalmente quando P208 here()
desbloquear consumer real". Captura materializa-se então.

---

## §3 Alterações em código

**Caminho 1 = zero código tocado.**

L0/L1 inalterados. Sem novo prompt L0; sem novos módulos L1;
sem novos tests; sem mudanças em `CountingIntrospector` (regra
empírica P207B §5 não acionada porque zero métodos novos).

Anotações documentais:

| Ficheiro | Edição |
|----------|--------|
| `00_nucleo/adr/typst-adr-0076-introspector-completion.md` | §Plano de materialização: série P207 transita "EM CURSO" → "✅ MATERIALIZADO 2026-05-12"; P207E anotado com Caminho 1; bloco "Agregado série P207" adicionado com sumário 5 sub-passos + métricas agregadas. |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | §3.0quater Marca de actualização adicionada (paralelo a §3.0/§3.0bis/§3.0ter): regista início M9c + série P207 fechada + mudanças factuais (trait 20→26 métodos, 1873→1899 tests, surpresas, patterns emergentes). |
| `00_nucleo/materialization/typst-passo-207E-relatorio.md` | Este ficheiro (novo). |

---

## §4 Decisões substantivas

- **Caminho 1 preferido a 2** (Caminho 1 fixado): zero
  consumers + custo M = "captura sem consumer é
  over-engineering" (spec §5 risco 1). Pattern anti-inflação
  formalizado no blueprint §3.0quater.
- **Sem split E em F**: P207E fica como único sub-passo de
  encerramento. Não há "P207F captura" porque captura
  emerge naturalmente em P208.
- **`PageStore::from_runtime` materializado mas não usado**:
  P207D criou o construtor antecipando P208. Não é código
  morto — é interface preparada que se torna activa quando
  consumer emergir. Documentado em `page_store.md` §"Não-
  objectivos".
- **Marca blueprint paralela a P204H/P205E/P206E**: §3.0quater
  é a 4ª marca cirúrgica do mesmo pattern. Documenta início
  M9c + fecho série P207 sem reescrita ampla do blueprint.
- **ADR-0076 mantém PROPOSTO**: transição PROPOSTO → ACEITE
  fica para P211B (encerramento M9c inteiro). Não fechar a
  ADR aqui é deliberado — séries P208-P211 ainda decorrem.

---

## §5 Métricas

| Métrica | Antes (P207D) | Depois (P207E) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| Sub-stores L1 | 24 | 24 | 0 |
| `CALL_COUNTERS` slots | 26 | 26 | 0 |
| Tests workspace | 1899 | 1899 | 0 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| L0 prompts modificados (em P207E) | — | 0 | — |
| L1 ficheiros modificados (em P207E) | — | 0 | — |
| Documentação modificada (em P207E) | — | 3 | +3 |

**Agregado série P207** (P207A diagnóstico + B + C + D + E):

| Métrica | Pré-P207 | Pós-P207E | Δ série |
|---------|----------|-----------|---------|
| Trait `Introspector` métodos | 20 | 26 | +6 |
| Sub-stores L1 (entities/) | 23 | 24 | +1 (`PageStore`) |
| `CALL_COUNTERS` slots | 20 | 26 | +6 |
| Tests workspace | 1873 | 1899 | +26 |
| L0 prompts novos | — | 1 (`page_store.md`) | +1 |
| L0 prompts modificados | — | 3 (`introspector.md`, `label_registry.md`, `measurements.md`) | +3 |
| L1 ficheiros novos | — | 1 (`page_store.rs`) | +1 |
| L1 ficheiros modificados | — | 3 (`introspector.rs`, `label_registry.rs`, `measurements.rs`) | +3 |
| ADRs anotadas | — | 1 (`ADR-0076`) | +1 |
| Cargo.toml promoções | — | 1 (`ecow` dev→dep em `03_infra`) | +1 |
| Surpresas empíricas registadas | — | 2 (regra P207B §5; ecow promotion) | +2 |
| `div-N` registadas | — | 1 (`P207A.div-1` aprovada pelo humano) | +1 |

---

## §6 Encerramento série P207 — sumário literal

Série P207 fechou em 5 sub-passos. Pattern empírico do projecto
(P204A-H, P205A-E, P206A-E) replicado: diagnóstico-primeiro
(A) → materialização incremental (B, C, D) → encerramento
documental (E).

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P207A | Diagnóstico-primeiro | M (real ~50min) | Auditoria empírica 62 itens, ADR-0076 PROPOSTO, `P207A.div-1` registada, 4 outputs (auditoria + diagnóstico + relatório + ADR). |
| P207B | Trait extension baixo-custo | S (real <2h) | `Introspector::query_labelled()` materializado; `LabelRegistry::iter()` introduzido; trait 20→21. |
| P207C | Sub-store refactor + trait extension | M (real ~2h) | `LabelRegistry` multi-label (`HashMap<Label, Vec<Location>>`); `lookup_all` + `count` adicionados; trait `label_count`; trait 21→22. |
| P207D | Sub-store novo + 4 trait extensions | M-L (real ~4h) | `PageStore` sub-store novo (paralelo a `SealedPositions`); 4 trait methods page-aware; trait 22→26; `ecow` dep promotion. |
| P207E | Encerramento série + decisão captura | S (real ~40min) | Caminho 1 fixado; ADR-0076 anotado; blueprint §3.0quater; relatório este. |

**Custo agregado real**: ~9h (estimado 12-15h). Magnitude
agregada **L** confirmada empíricamente.

**Padrões emergentes formalizados** (consumíveis para passos
futuros):

1. **Regra empírica P207B §5**: cada novo trait method do
   `Introspector` propaga obrigatóriamente a
   `CountingIntrospector` em `03_infra/src/measurements.rs`
   (4 pontos: `INTROSPECTOR_METHODS`, `CALL_COUNTERS`,
   impl method, sentinel). Mecânico mas obrigatório.
2. **Caminho 1 anti-inflação**: encerramento documental
   puro é honesto quando custo de materialização excede
   benefício. P207E formaliza: spec autoriza "emerge
   naturalmente quando consumer aparecer".
3. **Sub-store sealed pattern (P205B/C reuse)**: `PageStore`
   reusa literal o pattern (P205B/C) — `empty` + `from_runtime`
   + `inject_*` em `TagIntrospector`. Reutilizável para
   futuros sub-stores sealed M9c+.

---

## §7 Próximo sub-passo

**P208 série** — `here()` + `locate()` materialização per
`P207A.div-1` aprovado:

- **P208A** — diagnóstico-primeiro (`Tracked<Context>` análogo
  cristalino).
- **P208B** — `here()` materialização (M ~3h).
- **P208C** — `locate(selector)` materialização (S-M ~2h).
- **P208D** — encerramento série.

Pré-condição para P208A: série P207 fechada (cumprido em
P207E). Magnitude estimada série P208: M (~5-7h).

ADR-0076 mantém `PROPOSTO` até P211B (encerramento M9c
inteiro com auditoria 7 condições + transição PROPOSTO →
ACEITE).
