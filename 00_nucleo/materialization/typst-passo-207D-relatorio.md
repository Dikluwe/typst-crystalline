# Relatório do passo P207D

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-207D.md`.
**Tipo**: implementação cross-modular (sub-store novo + 4 trait
methods).
**Magnitude planeada**: M-L (~5-6h). **Magnitude real**: M.
**Marco**: M9c (Bloco II page-aware + Bloco VIII infraestrutura
parcial per ADR-0076).

---

## §1 O que foi feito

Materialização dos 4 trait methods page-aware (`pages`, `page`,
`page_numbering`, `page_supplement`) + criação do sub-store
`PageStore` (paralelo a `SealedPositions` per P205B/C). Trait
passa de 22 para 26 métodos. Pre-injecção: todos os 4 retornam
`None` graciosamente (pattern P204D §C6a preservado). Pós-`inject_pages`:
`pages` e `page` resolvem real; `page_numbering` e `page_supplement`
resolvem real quando `PageStore::from_runtime` for usado com
Vecs populados (P207D materializa o construtor; captura
runtime no walk de layout permanece deferred a Bloco VIII
futuro). C1-C5 cumpridas; sem `P207D.div-N`. Tests: 1899 verdes
(1885 baseline + 14 novos); `crystalline-lint`: 0 violations.

---

## §2 Opção de infraestrutura fixada

**C2 = Opção 2** — `PageStore` sub-store dedicado paralelo a
`SealedPositions`.

Justificação empírica (C1):

- **C1.1 — Zero consumers**: grep production em
  `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` por
  `pages`/`page`/`page_numbering`/`page_supplement` →
  **zero** consumers reais. Confirma P207A A11 / P205D D3.
  Critério spec: "decisão estrutural antecipada para
  P208/P211. Forma menos invasiva é preferida".
- **C1.2 — Dados page-level existentes**: `LayouterRuntimeState`
  já tem `label_pages: HashMap<Label, usize>`, `positions:
  HashMap<Location, Position>` (P204D, sealed via P205B).
  Cristalino `Page` struct tem apenas `width`, `height`,
  `items` — **sem `numbering` ou `supplement`** (divergência
  vs vanilla).
- **C1.3 — Vanilla pre-computa em constructor**:
  `PagedIntrospector::new(pages: &[Page])` constrói
  `page_numberings: Vec<Option<Numbering>>` e
  `page_supplements: Vec<Content>` em
  `lab/typst-original/crates/typst-layout/src/introspect.rs:38-58`.
  Cristalino segue o pattern: sealing pós-finish em sub-store
  dedicado (não na PagedIntrospector global — P205A.div-1).

Critério spec satisfeito: "Se vanilla pre-computa em
`PagedIntrospector::new()`, cristalino deveria seal análogo —
**Opção 2**". Opção 1 (estender `LayouterRuntimeState`)
rejeitada — quebraria pattern P205B/C de sealing-por-sub-store.
Opção 3 (estender `SealedPositions`) rejeitada por risco 1 do
spec (mistura concerns point-level vs page-meta).

**Sem split D+E**: spec §6 autoriza split se infra para
numbering/supplement é "significativamente distinta" de page/pages.
Não foi o caso — uma única `PageStore` cobre os 4 métodos com
custo proporcional. P207E reservado para encerramento série
documental.

---

## §3 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L0 (novo) | `00_nucleo/prompts/entities/page_store.md` | Novo prompt L0 (≈190L) com Contexto + Restrições + Interface + Integração + Tests + Não-objectivos + Cross-references. Hash do Código: `47a8d343`. |
| L0 | `00_nucleo/prompts/entities/introspector.md` | +4 entradas no trait (`pages`, `page`, `page_numbering`, `page_supplement`) + 1 field novo `pub page_store: PageStore` + linha Histórico 2026-05-12. Hash: `cb327d65 → e447b139`. |
| L0 | `00_nucleo/prompts/infra/measurements.md` | "22 métodos" → "26 métodos" em 3 pontos. |
| L1 (novo) | `01_core/src/entities/page_store.rs` | Novo módulo (≈190L) com struct `PageStore { total_pages, numberings, supplements }` + 6 métodos (`empty`, `from_total_pages`, `from_runtime`, `total_pages`, `numbering_for_page`, `supplement_for_page`, `is_empty`) + 5 tests. `@prompt-hash 47a8d343`. |
| L1 | `01_core/src/entities/mod.rs` | +`pub mod page_store;`. |
| L1 | `01_core/src/entities/introspector.rs` | +4 trait methods (pages/page/page_numbering/page_supplement) + 4 impls em `TagIntrospector` + 1 field `pub page_store: PageStore` + 1 método `inject_pages` paralelo a `inject_positions` (P205C). +`use ecow::EcoString;`. +9 tests P207D. `@prompt-hash 12aab176 → bfe24f58`. |
| L3 | `03_infra/Cargo.toml` | `ecow` promovido de `[dev-dependencies]` para `[dependencies]` (production: `CountingIntrospector::page_numbering` delega `Option<&EcoString>`). |
| L3 | `03_infra/src/measurements.rs` | `INTROSPECTOR_METHODS: [&str; 22] → [&str; 26]` (4 entries page-aware); `CALL_COUNTERS: [...; 22] → [...; 26]`; impl `Introspector for CountingIntrospector` ganha 4 métodos com `record_call(22..=25)`; sentinel actualizado (22 → 26); +`use std::num::NonZeroUsize; use ecow::EcoString;`. Hash: `cbccd899 → 0520956b`. |

Hashes L0+L1 propagados via `crystalline-lint --fix-hashes .`;
0 drifts remanescentes.

---

## §4 Decisões substantivas

- **`Numbering` como `EcoString`** (divergência vanilla):
  vanilla tem enum `Numbering` (pattern + Func variants);
  cristalino L1 não tem este tipo. Decisão: usar `EcoString`
  directo (string pattern como "1", "I", "α") per ADR-0024
  (`EcoString` é o tipo cristalino para string-like values).
  Documentado em `page_store.md` §Contexto + doc comment do
  trait method.
- **`pages(loc)` ignora `loc`**: paridade literal com vanilla
  (`PagedIntrospector::pages` retorna `Some(self.pages)`
  ignorando location). Cristalino segue.
- **`page(loc)` delega a `SealedPositions`** (não a `PageStore`):
  `positions.position_of(loc)?.page`. Decisão minimiza
  coupling — `PageStore` só tem dados page-level (total +
  numbering + supplement); position-to-page é responsabilidade
  de `SealedPositions` (P205B/C).
- **`page_numbering`/`page_supplement` bypass do trait method
  `page`**: impl interna usa `self.positions.position_of(loc)?.page`
  directamente em vez de chamar `self.page(loc)?`. Evita
  recursão tracked (CLAUDE.md convenção comemo).
- **Sub-store novo vs extensão**: novo `PageStore` em vez de
  estender `SealedPositions`. Per spec §5 risco 1 + ADR-0026
  separation of concerns: position é point-level (page+point);
  numbering+supplement é page-meta. Mantém structs focadas.
- **Captura no walk de layout deferred**: P207D materializa
  `PageStore::from_runtime` mas Layouter não captura numbering/
  supplement no walk (cristalino `Page` não os carrega).
  P207E ou Bloco VIII futuro adiciona captura. Pre-captura,
  `page_numbering`/`page_supplement` retornam `None` mesmo
  pós-injecção minimal.

---

## §5 Métricas

| Métrica | Antes (P207C) | Depois (P207D) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 22 | 26 | +4 |
| `TagIntrospector` sub-stores | 9 | 10 | +1 |
| Sub-stores L1 (módulos entities/) | 23 | 24 | +1 |
| `CALL_COUNTERS` slots | 22 | 26 | +4 |
| Tests workspace | 1885 | 1899 | +14 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts novos | — | 1 | +1 |
| L0 prompts modificados | — | 2 | +2 |
| L1 ficheiros novos | — | 1 | +1 |
| L1 ficheiros modificados | — | 3 | +3 |
| Production call-sites quebrados | — | 0 | 0 |
| Cargo.toml dependências promovidas | — | 1 | +1 |

---

## §6 Divergências

Nenhuma `P207D.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4 → C5.

**Confirmações empíricas registadas**:
- Regra empírica P207B §5 propagada novamente — 4 trait methods
  novos exigiram 4 entries + 4 slots + 4 impls + 1 sentinel
  update em `CountingIntrospector`. Mecânico mas obrigatório.
- Surpresa P207D: `ecow` precisou ser promovido a `[dependencies]`
  de `03_infra` (era dev-dep). Justifica-se: `CountingIntrospector`
  é production code que delega `Option<&EcoString>`. Documentado
  em Cargo.toml.

---

## §7 Próximo sub-passo

**P207E** (per ADR-0076 §Plano de materialização): encerramento
série P207 — anotações ADR + opcionalmente captura no walk de
layout para popular `PageStore::numberings`/`supplements` (Bloco
VIII completion). Magnitude S documental (~30min-1h se sem
captura; M se inclui captura).

**P208 série** (`here()` + `locate()`): segue P207E. Bloqueada
por escopo reduzido `P207A.div-1` confirmado.

ADR-0076 §Plano de materialização anotado: P207D marcado
`✅ MATERIALIZADO 2026-05-12 (Opção 2 fixada em C2)`. Estado
M9c: 4 sub-passos completos (A diagnóstico + B + C + D infra
page-aware).
