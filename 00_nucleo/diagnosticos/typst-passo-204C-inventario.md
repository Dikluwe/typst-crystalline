# Inventário interno P204C — Layouter migração para `Tracked<dyn Introspector>`

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204C.md`.
**Natureza**: diagnóstico interno (factos empíricos +
decisões + alterações aplicadas).

---

## §1 C1 — Inventário empírico

### 1.1 Localização do `Layouter`

- **Struct**: `01_core/src/rules/layout/mod.rs:69-154` (não
  em `layouter.rs`; está em `mod.rs` directamente).
- **Pre-P204C**: `pub struct Layouter<M: FontMetrics, S:
  ImageSizer = NullImageSizer>` — sem lifetime.
- **Field introspector** (linha 105):
  `pub(super) introspector: TagIntrospector` (por valor,
  com default `TagIntrospector::empty()` na construção).

### 1.2 Field count

22 fields confirmados (per snapshot 2026-05-05 §5;
P201 auditoria delta §1 #11).

### 1.3 Impl blocks (5 total)

| # | Ficheiro | Linha |
|---|----------|-------|
| 1 | `01_core/src/rules/layout/mod.rs` | 156 (main impl) |
| 2 | `01_core/src/rules/layout/cursor.rs` | 18 |
| 3 | `01_core/src/rules/layout/equation.rs` | 19 |
| 4 | `01_core/src/rules/layout/grid.rs` | 19 |
| 5 | `01_core/src/rules/layout/placement.rs` | 19 |

Todos com pattern `impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S>`.

### 1.4 Consumers de `self.introspector`

| Ficheiro | Linhas | Métodos consumidos |
|----------|--------|---------------------|
| `mod.rs` | 356, 361, 409, 411, 521, 680, 684 | `formatted_counter_at`, `formatted_counter`, `figure_number_at_index`, `bib_entry_for_key`, `bib_number_for_key` |
| `equation.rs` | 35, 111 | `formatted_counter_at`, `flat_counter_at` |

**~9 sites self.** consumers em produção.

### 1.5 Acesso externo via `layouter.introspector`

| Ficheiro | Linhas | Acesso |
|----------|--------|--------|
| `references.rs` | 51, 61 | `figure_number_for_label`, `resolved_label_for` |
| `outline.rs` | 35 | `headings_for_toc().to_vec()` |

**3 sites de acesso externo** (free fns que recebem
`&Layouter`). Todos read-only via método trait.

### 1.6 Mutações `l.introspector = ...`

| Ficheiro | Linha | Pattern |
|----------|-------|---------|
| `mod.rs` | 1485 | `l.introspector = introspector` (no-outline path) |
| `mod.rs` | 1519 | `l.introspector = introspector.clone()` (fixpoint loop) |
| `tests.rs` | 4292 | `layouter.introspector = intr_clone` (test) |

**3 mutações** post-construção que precisam ser
eliminadas (Tracked é borrow, não valor).

### 1.7 Layouter::new call sites (13 total, excluindo MathLayouter)

| Ficheiro | Linhas | Tipo |
|----------|--------|------|
| `mod.rs` | 1484, 1516 | produção (no-outline + fixpoint loop) |
| `tests.rs` | 51, 1300, 1358, 1443, 4173, 4262, 4315 | tests (7 sites) |
| `03_infra/src/layout.rs` | 25 | produção externa (`layout_with_font`) |
| outros | 3 sites | tests indirectos |

### 1.8 API pública externa

- `pub fn layout(content: &Content) -> PagedDocument` —
  `mod.rs:1441`. Sem lifetime exposed.
- `pub fn layout_with_introspector(content: &Content,
  introspector: TagIntrospector) -> PagedDocument` —
  `mod.rs:1465`. Aceita TagIntrospector por valor.

### 1.9 External callers de `pub fn layout`

4 sites em `03_infra/`:
- `pipeline.rs:28` (use).
- `export.rs:1559` (use).
- `integration_tests.rs:22` (use, em tests).
- `layout.rs:30` (fallback em `layout_with_font`).

### 1.10 Etiquetas

Todos os items A1.1-A1.10 **CONFIRMADO**. Sem
divergências relevantes face ao snapshot 2026-05-05.

---

## §2 C2 — Decisão sobre wrapper

### Dados

- 4 sites externos de `pub fn layout` em 03_infra.
- 1 site externo de `Layouter::new` directo (em
  03_infra/src/layout.rs).
- API pública sem lifetime exposed actualmente.

### Decisão fixada — **Wrapper SIM**

`pub fn layout(content)` e `pub fn layout_with_introspector(
content, introspector: TagIntrospector)` mantêm assinatura
pública sem `'a` exposto. Internamente:

1. Bind `introspector` num let.
2. `let intr_dyn: &dyn Introspector = &introspector`.
3. `use comemo::Track; let intr_tracked = intr_dyn.track();`
4. `Layouter::new(metrics, sizer, font_size, intr_tracked)`.

`Layouter::new` API pública **muda** (4º parâmetro
`Tracked` agora obrigatório). Site externo
`03_infra/src/layout.rs:25` adapta-se com construção
local de empty TagIntrospector + tracked.

### Justificação

- 4 callers externos de `pub fn layout` mantêm-se sem
  alteração — minimiza ondas.
- 1 caller externo de `Layouter::new` (em mesma
  workspace L3) é trivial de adaptar.
- API com lifetime exposto seria prestos viral pelo
  workspace.

---

## §3 C3-C7 — Alterações literais aplicadas

### 3.1 `01_core/src/rules/layout/mod.rs`

#### Struct (linha 69)

```text
- pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer> {
+ pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
```

#### Field (linha 105)

```text
- pub(super) introspector: crate::entities::introspector::TagIntrospector,
+ pub(super) introspector: comemo::Tracked<'a, dyn crate::entities::introspector::Introspector + 'a>,
```

#### `Layouter::new` (linha 156)

```text
- impl<M: FontMetrics, S: ImageSizer> Layouter<M, S> {
+ impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {

  pub fn new(
      metrics: M, sizer: S, font_size: f64,
+     introspector: comemo::Tracked<'a, dyn crate::entities::introspector::Introspector + 'a>,
  ) -> Self {
      ...
      Self {
          ...
-         introspector: TagIntrospector::empty(),
+         introspector,
          ...
      }
  }
}
```

#### `layout_with_introspector` (linha 1465+)

Pattern de construção Tracked uma vez antes do
no-outline path / fixpoint loop:

```text
+ use comemo::Track;
+ let intr_dyn: &dyn Introspector = &introspector;
+ let intr_tracked = intr_dyn.track();

  if !has_outline {
-     let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);
-     l.introspector = introspector;
+     let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);
      ...
  }

  for _ in 0..MAX_ITERATIONS {
-     let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);
-     l.introspector = introspector.clone();
+     let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);
      // Tracked é Copy; reusado entre iterações.
      ...
  }
```

### 3.2 Outros impl blocks (cursor.rs, equation.rs, grid.rs, placement.rs)

```text
- impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S> {
+ impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
```

4 ficheiros. Edição mecânica.

### 3.3 `03_infra/src/layout.rs`

```text
+ use comemo::Track;
+ use typst_core::entities::introspector::{Introspector, TagIntrospector};
+ let intr = TagIntrospector::empty();
+ let intr_dyn: &dyn Introspector = &intr;
+ let intr_tracked = intr_dyn.track();
- let mut l = Layouter::new(metrics, ImageSizeImageSizer, font_size);
+ let mut l = Layouter::new(metrics, ImageSizeImageSizer, font_size, intr_tracked);
```

### 3.4 Tests (7 sites em `01_core/src/rules/layout/tests.rs`)

Cada site adicionado boilerplate 5 linhas:

```rust
use comemo::Track;
use crate::entities::introspector::{Introspector, TagIntrospector};
let intr = TagIntrospector::empty();
let intr_dyn: &dyn Introspector = &intr;
let intr_tracked = intr_dyn.track();
let layouter = Layouter::new(FixedMetrics, NullImageSizer, ..., intr_tracked);
```

Test em linha 4292 (`layouter.introspector = intr_clone`):
- Eliminado o assignment.
- Tracked construído a partir do `intr` populado (não
  empty).
- Boilerplate inserido antes de Layouter::new.

---

## §4 C6+C9 — Migração de consumers

### 4.1 Sintaxe inalterada

Os ~9 self.introspector callers no Layouter (mod.rs +
equation.rs) **não precisaram de alteração**. Tracked
deref-coerces para acesso a métodos via trait — sintaxe
`self.introspector.<método>(...)` continua funcional.

### 4.2 Acesso externo (references.rs, outline.rs)

- `references.rs:51, 61`: `layouter.introspector.<método>(...)`
  funciona sem alteração.
- `outline.rs:35`: `layouter.introspector.headings_for_toc().to_vec()`
  funciona sem alteração.

Tracked oferece deref-like access a métodos do trait via
macro-generated impl.

### 4.3 Tabela consumers × ajuste

| Consumer | Site | Ajuste |
|----------|------|--------|
| Layouter mod.rs | 7 sites | nenhum |
| Layouter equation.rs | 2 sites | nenhum |
| Free fn references.rs | 2 sites | nenhum |
| Free fn outline.rs | 1 site | nenhum |
| Production assignments mod.rs | 2 sites | **eliminados** (substituídos por construtor) |
| Test assignment tests.rs | 1 site | **eliminado** + boilerplate adicionado |
| Test Layouter::new | 7 sites | **boilerplate adicionado** |
| External Layouter::new | 1 site (03_infra) | **boilerplate adicionado** |

---

## §5 C8+C9+C10+C11 — Verificações

### C8 — Compilação

```
cargo build --workspace
```

**Resultado**: verde, 2 warnings pré-existentes
(foundations.rs unreachable patterns; não relacionados
com P204C).

### C9 — Tests workspace

```
cargo test --workspace
Total tests: 1829
```

**1827 → 1829** (+2 P204C sentinels). **Sem regressões**.

### C10 — Sentinelas P204C (2 adicionadas)

#### `p204c_layouter_struct_aceita_tracked_introspector`

```rust
let _l: Layouter<'_, FixedMetrics, NullImageSizer> =
    Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
```

Sentinel de tipo: falha compilação se Layouter perder
lifetime ou se Layouter::new mudar para 3 args.

#### `p204c_pipeline_e2e_via_tracked`

Sentinel runtime: `let doc = layout(&Content::text("..."));
assert!(!doc.pages.is_empty());`. Confirma que pipeline
end-to-end funciona com Tracked.

3ª sentinel (decisão fixada não criar) seria
`p204c_introspector_field_e_tracked` — cobertura sobreposta
com #1.

### C11 — Linter

```
crystalline-lint .
```

**Resultado**: 0 violations.

---

## §6 Decisões tomadas durante a leitura

### 6.1 Wrapper SIM em vez de exposed lifetime

C2 fixou wrapper baseado em:
- 4 callers externos de `pub fn layout` (mais que
  zero — wrapper preferido).
- API pública existente sem lifetime.
- Manter convenção paridade vanilla onde possível,
  mas wrapper público é típico em projecto cristalino
  pré-M8.

### 6.2 Tracked construído UMA vez antes do fixpoint loop

A construção `let intr_tracked = intr_dyn.track()` é
feita ANTES do loop `for _ in 0..MAX_ITERATIONS`. Tracked
é Copy — reusado em todas as iterações. Sem perda de
performance.

### 6.3 Tests boilerplate inline

Considerada criação de helper `fn make_test_layouter()`,
mas:
- Helper não pode retornar Tracked porque lifetime do
  introspector terminaria.
- Tests precisam de inversion of control via closure —
  invasivo.

**Decisão**: boilerplate inline em cada test. ~5 linhas
por site. Mecânico mas explícito.

### 6.4 Test linha 4292 — eliminação do assignment

Antes:
```rust
let intr = introspect_with_introspector(&content);
let intr_clone = intr.clone();
let mut layouter = Layouter::new(...);
layouter.introspector = intr_clone;  // assignment
```

Depois:
```rust
let intr = introspect_with_introspector(&content);
// (clone eliminado — não precisa)
let intr_dyn: &dyn Introspector = &intr;
let intr_tracked = intr_dyn.track();
let mut layouter = Layouter::new(..., intr_tracked);
```

Vantagem: clone eliminado; `intr` populado outlive
layouter no escopo da função; tracked refere directamente.

### 6.5 03_infra/src/layout.rs — empty TagIntrospector

`layout_with_font` é fonts-only path (sem TOC / queries
introspection). Empty TagIntrospector é suficiente.
Caso futuro precise de introspection populada, ajustar
nesse momento.

### 6.6 Sem alterações em references.rs, outline.rs,
equation.rs consumers

Consumers usam `layouter.introspector.<método>` —
deref-coerção para Tracked funciona transparentemente.
Sem trabalho aqui.

---

## §7 Métricas

| Métrica | Pré-P204C | Pós-P204C | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1827 | **1829** | +2 |
| Crystalline-lint violations | 0 | 0 | = |
| Layouter ganha `'a` | não | **sim** | breaking interno |
| `Tracked<dyn Introspector>` field | não | **sim** | new |
| LOC produção (mod.rs + 4 impls + 03_infra) | baseline | +~50 | +50 |
| LOC tests (boilerplate + sentinels) | baseline | +~80 | +80 |
| Consumers que precisaram alteração | — | 3 (assignments eliminados) + 7 (test boilerplate) + 1 (03_infra boilerplate) | — |
| Consumers que **não** precisaram | — | ~12 (deref-coerção transparente) | — |

---

## §8 Critério de fecho — C13

Per spec §3 C13:

- [x] C1 inventário completo.
- [x] C2 wrapper-decisão fixada (SIM).
- [x] C3+C4 Layouter ganha `'a` em struct e 5 impls.
- [x] C5 construção via `track_with` aplicada (em
  `layout_with_introspector` + 03_infra).
- [x] C6 ~10 consumers — migração trivial (deref).
- [x] C7 tests adaptados (7 boilerplate + 1 eliminação
  assignment).
- [x] C8 compilação verde.
- [x] C9 tests workspace verdes (1829).
- [x] C10 sentinelas (2 adicionadas).
- [x] C11 linter 0 violations.
- [x] Inventário registado (este ficheiro).
- [ ] Relatório escrito (próximo output).

**Sem `P204C.div-N`** — todos os items CONFIRMADO; sem
divergências.

---

## §9 Referências

### Modificados em P204C

- `01_core/src/rules/layout/mod.rs` (struct, field,
  Layouter::new, layout_with_introspector).
- `01_core/src/rules/layout/cursor.rs` (impl block).
- `01_core/src/rules/layout/equation.rs` (impl block).
- `01_core/src/rules/layout/grid.rs` (impl block).
- `01_core/src/rules/layout/placement.rs` (impl block).
- `01_core/src/rules/layout/tests.rs` (7 sites + 2
  sentinelas).
- `03_infra/src/layout.rs` (1 site externo).

### Inalterados (intencional)

- `01_core/src/entities/introspector.rs` (P204B
  trait + sentinels).
- `01_core/src/rules/layout/references.rs` (deref).
- `01_core/src/rules/layout/outline.rs` (deref).
- ADR-0073 (transita ACEITE em P204H).

### Auditoria fonte

- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`.
- `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`.
- `00_nucleo/diagnosticos/typst-passo-204B-inventario.md`.
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (PROPOSTO).
- `00_nucleo/materialization/typst-passo-204C.md` (spec).
