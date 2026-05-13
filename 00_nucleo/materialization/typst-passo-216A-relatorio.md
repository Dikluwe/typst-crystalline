# Relatório do passo P216A — Region type + Layouter refactor

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-216A.md`.
**Tipo**: refactor estrutural cross-modular sem mudança
observable.
**Magnitude planeada**: M+ (~3-5h). **Magnitude real**: M
(~2h — sed mecânico + 1 ajuste sincronização SetPage).
**Marco**: nenhum (quarto passo pós-M9c; primeira
materialização real DEBT-56).

---

## §1 O que foi feito

P216A introduziu tipo `Region` em L1
(`01_core/src/entities/region.rs`) agregando 5 fields
escalares (`cursor_x`, `cursor_y`, `line_start_x`,
`current_items`, `current_line`) + 2 dimensões (`width`,
`height`) previamente dispersos no `Layouter`. ~167
call-sites refactored mecânicamente em 6 ficheiros.
Caminho B1 fixado (PageConfig preservado; region.width/height
é cópia derivada). Opção α fixada (sem helpers; acesso
directo `self.region.X`). Tests preservados: 1939 → **1943
verdes** (+4 sentinelas P216A); 0 violations. Critério
rígido cumprido.

---

## §2 Confirmação inventário call-sites (C1)

Inventário empírico pré-refactor (P215 estimou 135 em
mod.rs):

| Ficheiro | Cursor/items/line | line_start_x | page_config.* | Total |
|----------|--------------------|---------------|----------------|-------|
| `mod.rs` | 89 | 17 | 30 (page_config inteiro) | ~136 |
| `cursor.rs` | — | — | — | 22 (combinado) |
| `equation.rs` | — | — | — | 8 |
| `grid.rs` | — | — | — | 13 |
| `placement.rs` | — | — | — | 12 |
| `tests.rs` | — | — | — | 2 |
| **Total all files** | | | | **~167** |

P215 estimativa (135 em mod.rs) próxima do empírico real
(136). Spread total > P215 (167 vs 135) por inclusão de
ficheiros adjacentes não contemplados na estimativa
original. **Sem `P216A.div-N` necessário** — desvio dentro
do esperado para refactor cross-modular.

---

## §3 Region tipo criado

`01_core/src/entities/region.rs` (~144 LOC):

**Struct**:
```rust
pub struct Region {
    pub cursor_x:      Pt,        // newtype f64
    pub cursor_y:      Pt,
    pub line_start_x:  Pt,
    pub current_items: Vec<FrameItem>,
    pub current_line:  Vec<FrameItem>,
    pub width:         f64,        // paridade PageConfig
    pub height:        f64,
}
```

**Métodos**: `new(width, height)`, `reset()`, `has_pending()`.

**Sentinelas (4 tests P216A)**:
- `p216a_region_new_inicia_cursor_zero` — cursor zerado.
- `p216a_region_reset_preserva_dimensoes` — reset preserva
  dimensions + line_start_x.
- `p216a_region_has_pending_false_apos_new` — empty buffers.
- `p216a_region_clone_funciona` — Clone derivado funciona.

**L0**: `00_nucleo/prompts/entities/region.md` (~184 linhas)
paralelo a `layouter_runtime_state.md`. Hash propagado:
`60b07b65` (L0) ↔ `2d938d3d` (L1) via `crystalline-lint
--fix-hashes`.

**Re-export**: `pub mod region;` em
`01_core/src/entities/mod.rs:51`.

---

## §4 Layouter refactor

**Struct antes** (5 fields escalares + 2 derivados de
PageConfig):
```rust
pub(super) current_items: Vec<FrameItem>,
pub(super) cursor_x:      Pt,
pub(super) cursor_y:      Pt,
pub(super) line_start_x:  Pt,
pub(super) current_line:  Vec<FrameItem>,
// width/height via page_config.width / page_config.height
```

**Struct depois** (1 field agregado):
```rust
/// P216A: agregação de state geométrico (5 fields + 2 dims).
pub(super) region: crate::entities::region::Region,
```

**`Layouter::new` ajuste**:
```rust
region: {
    let mut r = Region::new(cfg.width, cfg.height);
    r.cursor_x = Pt(cfg.margin);
    r.cursor_y = Pt(cfg.margin) + ascender;
    r.line_start_x = Pt(cfg.margin);
    r
},
```

**Substituição mecânica** (sed em 6 ficheiros):
- `self.cursor_x` → `self.region.cursor_x`
- `self.cursor_y` → `self.region.cursor_y`
- `self.line_start_x` → `self.region.line_start_x`
- `self.current_items` → `self.region.current_items`
- `self.current_line` → `self.region.current_line`
- `self.page_config.width` → `self.region.width`
- `self.page_config.height` → `self.region.height`

Total real: **~167 substituições** distribuídas entre 6
ficheiros (mod.rs + cursor.rs + equation.rs + grid.rs +
placement.rs + tests.rs).

---

## §5 Decisões substantivas

- **Caminho B1 fixado** (vs B2): `PageConfig.width/height`
  preservados em `PageConfig`; `region.width/height` é cópia
  derivada em `Layouter::new`. Redundância controlada;
  minimiza blast radius. **1 ajuste manual obrigatório**:
  sincronização adicionada em `Content::SetPage` arm
  (`mod.rs:761-763`) para actualizar `region.width/height`
  quando `page_config` muda — descoberto via tests
  integration que falharam após sed mecânico.
- **Opção α fixada** (vs β/γ): sem helpers
  `cursor_x()`/`set_cursor_x()`. Acesso directo
  `self.region.X` em todos os call-sites. Anti-inflação
  aplicada 10ª vez cumulativa pós-P205D.
- **Pt newtype preservado**: spec original assumiu `f64`
  para cursor fields; auditoria revelou `Pt` newtype
  (`pub struct Pt(pub f64)`). Region adoptou `Pt` para
  cursor/line_start_x; manteve `f64` para width/height
  (paridade `PageConfig`).
- **Borrow checker**: 0 quebras detectadas. Refactor
  mecânico mais limpo do que esperado — Rust aceita
  `self.region.cursor_x = self.region.cursor_y + ...`
  porque `region` é único field acedido via auto-deref
  field-projection (não disjoint borrow conflict).
- **Tests file**: tests.rs ficou fora do `for f in ...`
  inicial; 2 referências `l.cursor_y.val()` corrigidas
  manualmente para `l.region.cursor_y.val()` antes de
  re-test. Único ponto de manual fix além do SetPage sync.

**Sem `P216A.div-N`** — Caminho B1 + Opção α + 2 ajustes
manuais documentados como conduta empírica esperada do
refactor.

---

## §6 Resultados verificação

**Tests workspace**:
- Pré-P216A: **1939 verdes**.
- Pós-P216A: **1943 verdes** (1939 baseline + 4 sentinelas
  P216A region).
- 0 failures; 0 ignored relevantes.
- Distribuição:
  - typst-core: 1654 (era 1650; +4 region tests)
  - typst-shell: 242
  - typst-infra: 24
  - typst-wiring: 2
  - integration: 21

**Linter**:
- `crystalline-lint .` antes do `--fix-hashes`: 1 V5 drift
  warning (esperado — region.rs criado com hash
  placeholder `00000000`).
- Após `crystalline-lint --fix-hashes .`: hash sync
  region.rs → `2d938d3d`; L0 region.md → `60b07b65`.
- `crystalline-lint .` final: **0 violations**.

**Critério rígido P216A cumprido**: 0 mudança observable;
1939 tests pre-existentes preservados verdes.

---

## §7 ADR-0078 anotação

`00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
§"Plano de materialização" anotado com bloco
**`### P216A materializado 2026-05-12`**:

- Tipo `Region` introduzido + L0 + sentinelas.
- Layouter refactored + Caminho B1 sync SetPage.
- Inventário real ~167 call-sites em 6 ficheiros.
- Opção α anti-inflação 10ª aplicação cumulativa.
- Tests 1943 verdes; 0 violations.

ADR-0078 **mantém status PROPOSTO**. Transição
IMPLEMENTADO ocorrerá em P221 quando 6 condições do
§"Plano de materialização" forem satisfeitas.

---

## §8 Próximo sub-passo

**P216B** — sub-fase (a) parte 2: introduzir `Regions`
(Vec<Region>) wrapper + `Layouter::with_regions` helper.
Magnitude estimada M (~2-3h per P215.div-1; ~30-40
call-sites refactor adicional).

Estrutura proposta (per ADR-0078 §"Decisão"):
```rust
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,
    pub last:    Option<Region>,
}
```

Pré-condição cumprida: P216A fechado; `Region` tipo
estabelecido como referência arquitectural.

**Estado actual**:
- Marco M9c: ✅ ACEITE preservado.
- ADR-0078 PROPOSTO (column flow); ADR-0076/0077 ACEITES.
- DEBT-56 sub-fase (a) parte 1 ✅ (parte 2 pendente).
- Layout 78% preservado (sem mudança observable).
- Tests **1943 verdes**; **0 violations** preservados.
- **10ª aplicação cumulativa anti-inflação** pós-P205D
  (Opção α).
- **Pattern arquitectural "Layouter-state agregado em
  struct dedicada" N=2** (P190C `LayouterRuntimeState` +
  P216A `Region`).

Trajectória aberta para P216B (continuação Caminho 1
humano fixado pré-P216A); decisão humana entre prosseguir
P216B imediatamente ou pivot para Bloco C opcional (P222
`measure` stdlib).
