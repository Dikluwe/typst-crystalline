# Prompt L0 — `entities/region`
Hash do Código: 3f89e228

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/region.rs`
**Criado em**: 2026-05-12 (P216A — DEBT-56 sub-fase a parte 1)
**ADRs relevantes**: ADR-0078 (PROPOSTO; column flow algorithm),
ADR-0029 (pureza física L1), ADR-0036 (atomização progressiva).

---

## Contexto

ADR-0078 PROPOSTO (P215) fixou caminho `Region/Regions`
abstraction para column flow. P215.div-1 decompôs sub-fase
(a) em P216A+P216B (135 call-sites > 100 limiar).

`Region` é introduzido em P216A como **agregação de state
geométrico** previamente disperso em fields escalares no
`Layouter`:

- `cursor_x` / `cursor_y` / `line_start_x`.
- `current_items: Vec<FrameItem>` / `current_line: Vec<FrameItem>`.
- `width` / `height` (cópia derivada de `PageConfig.width/height`
  per Caminho B1; redundância controlada).

Single-region por design em P216A. `Regions` (Vec<Region>)
introduzido em P216B; consumer multi-column em P219.

Pattern arquitectural "Layouter-state agregado em struct
dedicada" — N=2 (precedente `LayouterRuntimeState` P190C).

Vanilla equivalente: `typst-layout/src/regions.rs::Region`.
Cristalino simplifica per ADR-0078 §"Análise paridade vanilla":
- Sem `expand` axes (auto-expand controlado por flush helpers).
- Sem `full` flag (cristalino infere via `cursor_y` vs `height`).
- Owned `Vec<FrameItem>` (vs vanilla borrow/lifetime).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read+write durante layout (Layouter detém ownership single).
- `Clone` derivado (preserva semântica de cópia explícita
  para futuras iterações multi-region).
- `Debug` derivado (diagnóstico).
- Sem `PartialEq` / `Eq` / `Hash` em P216A (sem consumer
  identificado; pode ser adicionado se P216B/P219 precisarem).
- Tipo interno: `Vec<FrameItem>` (paridade `Layouter`
  pre-P216A; sem `EcoVec` em hot path de mutação).
- Pureza física per ADR-0029: `Vec` permitido em struct
  de domínio; sem I/O.

---

## Interface pública

```rust
use crate::entities::layout_types::FrameItem;

#[derive(Debug, Clone)]
pub struct Region {
    pub cursor_x:      f64,
    pub cursor_y:      f64,
    pub line_start_x:  f64,
    pub current_items: Vec<FrameItem>,
    pub current_line:  Vec<FrameItem>,
    pub width:         f64,
    pub height:        f64,
}

impl Region {
    pub fn new(width: f64, height: f64) -> Self;
    pub fn reset(&mut self);
    pub fn has_pending(&self) -> bool;
}

// P216B (DEBT-56 sub-fase a parte 2)
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: Region,
    // backlog: Vec<Region> — DIFERIDO a P219 (anti-inflação 11ª)
    // last:    Option<Region> — DIFERIDO a P219
}

impl Regions {
    pub fn single(width: f64, height: f64) -> Self;
    pub fn reset_current(&mut self);
}
```

---

## Semântica

- `new(width, height)`: cursor zerado; buffers vazios;
  `line_start_x = 0.0` (pode ser ajustado externamente para
  margem esquerda).
- `reset()`: cursor reseta para `(line_start_x, 0.0)`;
  buffers limpos; dimensões preservadas. Usado entre páginas
  numa Region single (P216A).
- `has_pending()`: true se algum buffer tem items.

### Regions (P216B)

- `single(width, height)`: cria `Regions` com 1 region única
  via `Region::new(width, height)`. Single-region preservado;
  fields `backlog`/`last` (paridade vanilla literal) deferidos
  a P219 quando consumer multi-column emergir
  (`Content::Columns` arm Layouter).
- `reset_current()`: delega a `current.reset()`. Conveniência
  semântica para Layouter chamar `self.regions.reset_current()`
  em vez de `self.regions.current.reset()` (paridade simétrica
  com `Region::reset`).

**Anti-inflação 11ª aplicação cumulativa pós-P205D**: forma
minimal `{ current: Region }` rejeita estrutura rica vanilla
até consumer real emergir. Critério de reabertura: P219
materialização `Content::Columns` consumer no Layouter.

---

## Invariantes

- `cursor_x >= line_start_x` durante layout normal (cursor
  avança para a direita; reseta na left edge).
- `cursor_y >= 0.0` durante layout normal (cursor avança para
  baixo).
- `current_items` e `current_line` preservam ordem de
  inserção (paridade comportamento Layouter pre-P216A).

---

## Tests obrigatórios (P216A C2 sentinelas)

- `p216a_region_new_inicia_cursor_zero`.
- `p216a_region_reset_preserva_dimensoes`.
- `p216a_region_has_pending_false_apos_new`.
- `p216a_region_clone_funciona`.

---

## Consumers

- **P216A**: `Layouter::region` field único (substitui 5
  fields escalares + 2 derivados de `PageConfig`).
- **P216B planeado**: `Regions { current: Region, backlog:
  Vec<Region>, last: Option<Region> }` wrapper.
- **P219 planeado**: consumer multi-column iterando
  `Regions`.

---

## Sobre paridade

Vanilla `typst-layout/src/regions.rs`:

```rust
pub struct Region {
    size: Size,
    expand: Axes<bool>,
    full: Abs,
}
```

Cristalino simplifica per ADR-0078 §"Análise paridade":
- `size: Size` → `width: f64` + `height: f64` (cristalino
  usa `f64` puro vs `Abs` newtype).
- `expand: Axes<bool>` → adiar (cristalino auto-expand
  controlado por flush helpers).
- `full: Abs` → adiar (cristalino infere via cursor_y).

Adicionalmente cristalino agrega state que vanilla mantém
fora da `Region` (cursor + items + line) para reduzir
fragmentação no `Layouter` cristalino single-pass.

---

## Não-objectivos

- `Regions` wrapper — diferido P216B.
- Multi-column consumer — diferido P219.
- `expand` axes — adiar (sem consumer cristalino).
- `full` flag — adiar (cristalino infere).
- Mover `width`/`height` de `PageConfig` exclusivamente
  para `Region` — Caminho B2 rejeitado em P216A C4
  (Caminho B1 preserva PageConfig por minimizar blast
  radius).

---

## Cross-references

- ADR-0078 PROPOSTO §"Decisão" — tipo `Region` simplificado
  vs vanilla.
- ADR-0029 (pureza física L1) — `Vec` permitido em struct
  de domínio.
- ADR-0036 (atomização progressiva) — P216A primeira
  atomização do refactor multi-region.
- P190C `LayouterRuntimeState` — precedente arquitectural
  "Layouter-state agregado em struct dedicada".
- P215 C1 — inventário 135 call-sites empíricos.
- P215.div-1 — decomposição sub-fase (a) em P216A+P216B.
- Vanilla `typst-layout/src/regions.rs:Region` — referência
  arquitectural.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-12 | P216A criação — DEBT-56 sub-fase (a) parte 1: tipo `Region` introduzido para agregar state geométrico Layouter (5 fields escalares + 2 dimensões). Paridade vanilla simplificada per ADR-0078 PROPOSTO. Pattern "Layouter-state agregado" N=2 (precedente P190C). | `region.rs`, `region.md`, `mod.rs` re-export |
| 2026-05-12 | P216B adição — DEBT-56 sub-fase (a) parte 2: struct `Regions` adicionada cohabitando com `Region` no mesmo módulo. Forma minimal `{ current: Region }` por **anti-inflação 11ª aplicação cumulativa pós-P205D** — fields `backlog: Vec<Region>` + `last: Option<Region>` (paridade vanilla literal) diferidos a P219 quando `Content::Columns` consumer emergir. Refactor Layouter `region: Region` → `regions: Regions` (~158 call-sites `self.region.X` → `self.regions.current.X`). Sub-fase (a) DEBT-56 fechada estruturalmente; sub-fase (b) consumer multi-column é P219+. | `region.rs`, `region.md`, `layout/{mod,cursor,equation,grid,placement}.rs` |

## Extensão `Regions` `backlog` + `last` — Passo 243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)

P216B introduziu `Regions { current: Region }` minimal por
anti-inflação 11ª aplicação cumulativa pós-P205D. **P243 estende**
para `Regions { current, backlog, last }` paridade vanilla
literal:

```rust
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,      // P243 — fase (b) populated
    pub last:    Option<Region>,   // P243 — fase (b) populated
}
```

**Fase (a) P243**: `backlog` vazio + `last: None` em produção
(single-region preservado literal P216A/P216B observable).
**Fase (b) DEBT-56**: populated quando `Content::Columns`
materializar (passo subsequente fora P243 — fase (a) é
infrastructure-only).

**Novo método `Regions::advance`**:
```rust
pub fn advance(&mut self) -> Option<Region>;
```

Comportamento:
- **`backlog` não-vazio (fase (b))**: move `current` → `last`;
  consome primeira região do `backlog` como novo `current`;
  retorna `Some(prev_current)` (caller pode commit para Page).
- **`backlog` vazio (fase (a))**: retorna `None`; caller cria
  nova região externa (e.g. `new_page` no Layouter). Preserva
  semantic P216A/B literal.

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=1 →
**2 cumulativo** (P242 radius/clip + **P243 multi-region
scope-outs**). Atinge limiar formalização N=2 candidato a ADR
meta passo administrativo XS futuro.

**Audit C1 P243 finding material**: spec hipotetizou refactor
profundo cross-module L+ (5-7 fields migrar, ~30-50 sítios).
Reality: refactor field-agregation já feito em P216A/B; P243
apenas estende `Regions` com `backlog` + `last` + promove
scope-outs via `regions.current.width` save/restore. Magnitude
real M (~2-3h) face L+ hipotetizado. **Sem `P243.div-N`** —
paridade lição N=6 cumulativo precedente P237/P240/P241/P242.

**Sub-padrão #14 "Tipo entity em ficheiro próprio"** preservado
N=6 (Regions já existe em `region.rs` desde P216B — P243 estende
o existente em vez de criar novo ficheiro).

## Extensão `Regions.cell` + métodos `effective`/`enter_cell`/`exit_cell` — Passo 246 (cell layout migration; activa A.4 breakable per-cell arquiteturalmente)

P216A introduziu `Region` (geometria abstracta width+height);
P216B adicionou `Regions { current }`; P243 estendeu para
`Regions { current, backlog, last }` + método `advance`.
**P246 estende novamente** para suportar consultas de contexto
células sem acoplar Layouter:

```rust
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,      // P243
    pub last:    Option<Region>,   // P243
    pub cell:    Option<Region>,   // P246 — cell region transient
}
```

**Métodos novos**:

```rust
/// Region efectiva: cell se activa, senão current (page).
pub fn effective(&self) -> &Region;

/// Entra célula com region dada; retorna saved (suporta
/// aninhamento Grid-in-Grid).
pub fn enter_cell(&mut self, cell: Region) -> Option<Region>;

/// Sai célula restaurando saved (None top-level; Some(outer)
/// aninhamento).
pub fn exit_cell(&mut self, saved: Option<Region>);
```

**Pattern uso no Layouter** (arm `Content::Grid` em `grid.rs`):

```rust
let saved_cell_region = self.regions.enter_cell(
    Region::new(body_w, body_h),
);
// ... layout cell body ...
self.regions.exit_cell(saved_cell_region);
```

Reader pattern (em `placement.rs`):

```rust
// Antes (4 fields):
if let Some(cell_h) = self.cell_available_h { ... }

// Depois (P246):
if let Some(cell) = self.regions.cell.as_ref() {
    let cell_h = cell.height;
    // ...
}
```

**Migração `cell_available_h` + `cell_origin_w`**: estes 2
fields do Layouter migrados para `Region.height` + `Region.width`
via `regions.cell`. **`cell_origin_x` + `cell_origin_y`
preservados** como Layouter fields legacy — `Region` actual sem
`origin: Point` (cell origin absoluto em pt na página exige
fields paralelos); refactor futuro com `Region.origin` permitirá
eliminar.

**Activa A.4 breakable per-cell arquiteturalmente** — pós-P246,
`Content::Block.breakable` + `Content::Boxed.height` + overflow
em TableCell podem consultar `regions.effective()` para decisão
real de quebra dentro da célula. Activação real (semantic
materialização) diferida a passo futuro não-reservado per
política P158.

**Sub-padrão emergente "Layouter consumer migration via API
wrapper" N=1 inaugurado P246** — migração field-by-field
Layouter privado → API entity-side. Reduz acoplamento entre
Layouter e contexto activo (cell/page/region). Candidato a
formalização N=3-4 futuro.

**Lição refinada audit C1 N=8 → 9 cumulativo** P246: "mapear
empíricamente distribuição de usos por sub-módulo antes de
fixar arquitectura de migração" (extensão da lição P245
"grep fields/arms já implementados antes de assumir trabalho
original"; extensão da lição P244 "grep variants `Content::*`
candidatas").

---

## Anotação cumulativa P251 — `pending_cell_tails` buffer + flush em new_page

**P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita ADR-0082
PROPOSTO N=2 segunda aplicação citante)** adiciona ao Layouter
buffer de cell tails que ultrapassaram limite vertical (row break
real cell-level γ-Items).

**Layouter field novo**:

```rust
pub(super) pending_cell_tails: Vec<DeferredCellTail>,

pub(super) struct DeferredCellTail {
    pub items:           Vec<FrameItem>,  // rebased pos.y
    pub origin_x:        f64,             // cell column-aligned
    pub width:           f64,             // body_w preservado
    pub fill:            Option<Color>,   // re-emit Z-order step 1
    pub stroke:          Option<Stroke>,  // re-emit Z-order step 3
    pub forwarded_count: u32,             // max 3 iter (loop mitigation)
}
```

**Paridade arquitectural P245** `DeferredFloat` + `floats_pending`
+ `flush_pending_floats` — subpadrão emergente "DeferredX buffer
+ flush em new_page" **N=1 → N=2 cumulativo P251**.

**Fluxo**:

1. `grid.rs` cell overflow em row Auto/Fraction → slice items
   `body_h` threshold; head emit na página actual; tail push ao
   buffer.
2. Próximo `new_page()` (causado por outro elemento ou pagebreak):
   - `flush_pending_floats()` emit floats P245.
   - Close current page; setup new page cursor.
   - **`flush_pending_cell_tails()`** emit tails no topo da nova
     página (Z-order paridade P248: fill atrás → items rebased →
     stroke à frente).

**Preservação P248 para rows Fixed**: cell overflow em
`TrackSizing::Fixed` mantém P248 clip implícito (paridade vanilla
"Fixed rows clip overflow"). Só Auto/Fraction usam P251 row
break.

**Limitações conscientes γ-Items (per ADR-0054 graded)**:

- Items atómicos (Group/Shape com bounds grandes) não dividem
  mid-item; vão completos para tail (paridade vanilla).
- Fill/stroke re-emit per fragment (visualmente "dois rectângulos
  separados"; paridade vanilla "split block draws two borders").
- Tail forwarding limit 3 iter (mitigação loop infinito;
  paridade vanilla heurística).
- Outras cells da row original **não continuam** na nova página
  (só cell que overflow continua; row-level imperfeito).

**Sub-padrão "Slice frame items at height"** N=1 inaugurado P251
— novo módulo `01_core/src/rules/layout/slicing.rs` com função
pura `slice_frame_items_at_height(items, threshold) -> (head,
tail)` + helper `rebase_item_y(item, delta)` exhaustive sobre
6 variants `FrameItem` (Text/Line/Glyph/Image/Shape/Group).

**Categoria C.2 Fase 5 Layout activada parcialmente P251**
(cell-level only; multi-region completo via column flow DEBT-56
continua diferido).

**Lição refinada audit C1 N=13 → 14 cumulativo P251**: "audit C1
deve confirmar localidade pos.y antes de fixar abordagem γ-Items
vs γ-Content para slicing" (audit §2.1 confirmou
`layout_sub_frame_with_width` retorna items com `pos.y` local
ao sub-frame; γ-Items magnitude L viável vs γ-Content L+).
