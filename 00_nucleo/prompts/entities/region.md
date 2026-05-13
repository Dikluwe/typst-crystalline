# Prompt L0 — `entities/region`
Hash do Código: 18f0080f

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
