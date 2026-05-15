# Relatório do passo P246 — Cell layout migration: `cell_available_h` + `cell_origin_w` → `regions.cell: Option<Region>` (refactor consumer Layouter; activa A.4 breakable per-cell arquiteturalmente)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-246.md`.
**Tipo**: refactor consumer Layouter — migração field-by-field
para API entity-side via wrapper `Regions.cell: Option<Region>`.
Decisão 7 P243 diferida materializada.
**Magnitude planeada**: M (~2-4h). **Magnitude real**: **S+
(~1h)** — audit C1 revelou escopo reduzido (~12 sítios; trivial
migração).
**Marco**: continuação materialização pós-M9d completo P245;
**primeira aplicação Layouter refactor sem migração funcional**
(refactor estrutural não-feature); **fechamento estrutural
Decisão 7 P243 diferida**; activa **A.4 breakable per-cell**
arquiteturalmente; nona aplicação cumulativa pattern "spec C1
audit obrigatório bloqueante pós-P236.div-1" N=8 → 9 cumulativo.

---

## §1 O que foi feito

P246 migra 2 dos 4 fields cell-related do Layouter
(`cell_available_h` + `cell_origin_w`) para serem consumidos via
`self.regions.cell: Option<Region>` (entity-side). `cell_origin_x`
+ `cell_origin_y` preservados como Layouter fields legacy
(Region sem `origin: Point`).

**Trabalho real**:
1. **`Regions.cell: Option<Region>` field novo** em
   `01_core/src/entities/region.rs` + 3 métodos (`effective`,
   `enter_cell`, `exit_cell`).
2. **Remoção 2 fields Layouter**: `cell_available_h` +
   `cell_origin_w` (declarações + init).
3. **Preservação 2 fields legacy**: `cell_origin_x` +
   `cell_origin_y` (Decisão 6).
4. **Refactor save/restore em `grid.rs:361-382`** — 2 chamadas
   API substitui 4 atribuições directas.
5. **Refactor reads em `placement.rs`** — 3 sites (54, 98,
   137-145).
6. **6 tests novos** em `entities/region.rs` (cell None/Some,
   effective, enter_cell, exit_cell aninhamento, clone).
7. **L0 `region.md` extensão** documentando field novo + 3
   métodos.
8. **ADRs anotadas**: 0079 Categoria A.4 desbloqueada P246;
   0080 sub-categoria nova "Layouter consumer migration via
   API wrapper" N=1; 0061 §"Refino futuro" breakable per-cell
   desbloqueado.

**2203 → 2209 verdes** (+6 P246 entity-side; 0 regressões;
**0 adaptações** em tests pré-existentes).
**Sem `P246.div-N`** — audit converge com Decisão 1 Opção B.

---

## §2 Auditoria pré-P246 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=8 → 9 cumulativo

**Audit empírico** (lição refinada P245 N=8 → P246 N=9
cumulativo: "mapear empíricamente distribuição de usos por
sub-módulo antes de fixar arquitectura de migração"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| 4 fields cell_* declarados | mod.rs:151-160 + init 271-274 | ✓ Confirmado | OK |
| Save/restore points | hipotetizou cross-submodule | **Único em `grid.rs:361-382`** (8 sítios: 4 save + 4 write) | Escopo reduzido |
| Read points | hipotetizou Place + resolve_alignment + outros | **Apenas `placement.rs`** (4 read sites em linhas 54, 98, 137-145, 182) | Escopo reduzido |
| `regions.push`/`pop` API pré-existente | Possível | **NÃO existe** | Opção A desnecessária |
| `regions.cell` pré-existente | Possível | **NÃO existe** | Opção B viável directa |
| Tests baseline pré-P246 | 2203 verdes | ✓ Confirmado | Baseline para +0-6 |

**Conclusão audit C1**: trabalho real ~12 sítios; Decisão 1
**Opção B confirmada** (snapshot `cell: Option<Region>` em
`Regions`). Magnitude real S+ face M hipotetizado.

**Sem `P246.div-N`** — audit converge com spec; paridade lição
N=9 cumulativo precedente.

---

## §3 `Regions.cell` + métodos (C2)

`01_core/src/entities/region.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,      // P243
    pub last:    Option<Region>,   // P243
    pub cell:    Option<Region>,   // P246
}

impl Regions {
    pub fn single(width: f64, height: f64) -> Self {
        Self {
            current: Region::new(width, height),
            backlog: Vec::new(),
            last:    None,
            cell:    None,
        }
    }

    /// Region efectiva: cell se activa, senão current.
    pub fn effective(&self) -> &Region {
        self.cell.as_ref().unwrap_or(&self.current)
    }

    /// Entra célula; retorna saved (suporta aninhamento).
    pub fn enter_cell(&mut self, cell: Region) -> Option<Region> {
        std::mem::replace(&mut self.cell, Some(cell))
    }

    /// Sai célula restaurando saved.
    pub fn exit_cell(&mut self, saved: Option<Region>) {
        self.cell = saved;
    }
}
```

**Suporta aninhamento Grid-in-Grid** via save/restore stack
LIFO.

**Sub-padrão #14 "Tipo entity em ficheiro próprio"** N=6
preservado (Regions já existe desde P216B; P246 estende
existente — não criar novo ficheiro).

---

## §4 Migração Layouter fields (C3)

`01_core/src/rules/layout/mod.rs`:

```diff
- pub(super) cell_available_h: Option<f64>,  // P246 removido (→ regions.cell.height)
  pub(super) cell_origin_x: Option<f64>,     // P246 preservado legacy
  pub(super) cell_origin_y: Option<f64>,     // P246 preservado legacy
- pub(super) cell_origin_w: Option<f64>,     // P246 removido (→ regions.cell.width)
```

**Justificação `cell_origin_x/y` preservados**: `Region` actual
sem `origin: Point` (geometria abstracta width+height); cell
origin absoluto em pt na página exige fields paralelos. Refactor
futuro com `Region.origin` permitirá eliminar (DEBT opcional
não-aberto per política P158).

### `grid.rs` save/restore

```diff
- let saved_cell_h  = self.cell_available_h;
- let saved_cell_ox = self.cell_origin_x;
- let saved_cell_oy = self.cell_origin_y;
- let saved_cell_ow = self.cell_origin_w;
- self.cell_available_h = Some(body_h);
- self.cell_origin_x    = Some(body_x);
- self.cell_origin_y    = Some(body_y);
- self.cell_origin_w    = Some(body_w);
+ let saved_cell_ox = self.cell_origin_x;
+ let saved_cell_oy = self.cell_origin_y;
+ let saved_cell_region = self.regions.enter_cell(
+     Region::new(body_w, body_h),
+ );
+ self.cell_origin_x = Some(body_x);
+ self.cell_origin_y = Some(body_y);

  // ... layout cell body ...

- self.cell_available_h = saved_cell_h;
- self.cell_origin_x    = saved_cell_ox;
- self.cell_origin_y    = saved_cell_oy;
- self.cell_origin_w    = saved_cell_ow;
+ self.regions.exit_cell(saved_cell_region);
+ self.cell_origin_x = saved_cell_ox;
+ self.cell_origin_y = saved_cell_oy;
```

**Redução**: 8 atribuições → 2 chamadas API + 2 atribuições
legacy preservadas. Redução de 50% de superfície + risco de bug
"esquecer restaurar" eliminado.

### `placement.rs` reads

```diff
- let (remaining_h, effective_v) = if let Some(cell_h) = self.cell_available_h {
+ let (remaining_h, effective_v) = if let Some(cell) = self.regions.cell.as_ref() {
-     (cell_h, alignment.v)
+     (cell.height, alignment.v)
  } else if self.is_height_unconstrained {
```

```diff
- match (self.cell_available_h.is_some(), effective_v) {
+ match (self.regions.cell.is_some(), effective_v) {
```

```diff
  PlaceScope::Column => match (
      self.cell_origin_x,
      self.cell_origin_y,
-     self.cell_origin_w,
-     self.cell_available_h,
+     self.regions.cell.as_ref(),
  ) {
-     (Some(cx), Some(cy), Some(cw), Some(ch)) => (cx, cy, cw, ch),
+     (Some(cx), Some(cy), Some(cell)) => (cx, cy, cell.width, cell.height),
```

---

## §5 Decisões substantivas (8 decisões fixadas incl. Decisão 0 lição N=8 → 9 cumulativo)

| # | Decisão | Resolução |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=9 cumulativo; refino procedural "mapear empíricamente distribuição de usos por sub-módulo antes de fixar arquitectura" |
| 1 | Arquitectura migração | **Opção B** confirmada pós-audit (snapshot `cell: Option<Region>`) — escopo reduzido (~12 sítios) viabiliza minimal refactor |
| 2 | Arm Grid save/restore refactor | `enter_cell`/`exit_cell` API substitui 4 atribuições directas |
| 3 | Reads `placement.rs` migrated | 3 sites refactor |
| 4 | Activação A.4 breakable per-cell | **DIFERIDA** (passo futuro não-reservado per política P158) |
| 5 | DEBT-34c + DEBT-37 sentinelas | preservadas ENCERRADO |
| 6 | `Region` struct intocada | preservação P216A literal; `cell_origin_x/y` Layouter fields legacy |
| 7 | Anti-inflação 38ª aplicação cumulativa | ✓ |
| 8 | Sub-padrão "Layouter consumer migration via API wrapper" N=1 inaugurado | ✓ |

---

## §6 Resultados verificação + tests + pré-condições obrigatórias

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2203-2206 verdes (range +0-3) | **2209 verdes** (1920+242+24+2+21) ✓ (+6 entity-side; acima range) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | 0 ou 1 hash propagado | ✓ (`entities/region.md` hash propagado; paridade Opção minimal) |
| Adaptações pre-existentes | N=0-5 | **N=0** ✓ |
| Content variants | 62 preservado | ✓ |
| ShapeKind variants | 5 preservado | ✓ |
| Layouter fields | -2 (cell_available_h + cell_origin_w) | ✓ |
| Regions fields | 3 → 4 (+cell) | ✓ |
| Regions methods | +3 (effective, enter_cell, exit_cell) | ✓ |
| Stdlib funcs | 64 preservado | ✓ |
| §A.5 distribuição | preservada literal (refactor não-feature) | ✓ |
| Cobertura Layout per metodologia | ~93-94% preservado | ✓ |
| Cobertura user-facing total | ~75-76% preservado | ✓ |
| ADR-0079 Categoria A.4 | anotação "arquiteturalmente desbloqueado P246" | ✓ |
| ADR-0080 sub-categorias | "Layouter consumer migration via API wrapper" N=1 inaugurada | ✓ |
| ADR-0061 §"Refino futuro" | anotação P246 | ✓ |
| DEBT-34c | ENCERRADO preservado | ✓ |
| DEBT-37 | ENCERRADO preservado | ✓ |
| L0 hashes propagados | 1 (region.md hash) | ✓ |
| Regressões reais | 0 | **0** |

**Tests P246** (6 unit em `entities/region.rs`):
- `p246_regions_single_cell_none_inicial`.
- `p246_regions_effective_sem_cell_retorna_current`.
- `p246_regions_effective_com_cell_retorna_cell`.
- `p246_regions_enter_exit_cell_top_level`.
- `p246_regions_enter_exit_cell_aninhado`.
- `p246_regions_clone_preserva_cell`.

**3 pré-condições obrigatórias verificadas**:
1. **Tests baseline preservados**: 2203 → 2209 verdes (+6
   entity-side; 0 regressões; 0 adaptações).
2. **Comemo memoization invariants ADR-0073/0074 preservados**
   — P246 toca Layouter consumer apenas.
3. **Backward compat E2E**: tests P83 + P84.6 + P156G/H +
   P157A/B passam inalterados (semantic preservada via wrapper).

**Promoções ADR**:
- ADR-0079 Categoria A.4 anotada "breakable per-cell
  arquiteturalmente desbloqueado P246".
- ADR-0080 sub-categoria nova "Layouter consumer migration via
  API wrapper" N=1 inaugurada.
- ADR-0061 §"Refino futuro" anotada A.4 breakable per-cell.
- **Sem novas ADRs criadas**.
- Distribuição ADRs preservada literal: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**.

---

## §7 Patterns emergentes inaugurados/consolidados P246 (2)

- **"Layouter consumer migration via API wrapper" N=1
  inaugurado P246** — sub-padrão novo (migração field-by-field
  Layouter privado → API entity-side; reduz acoplamento).
  Candidato a formalização N=3-4 futuro.
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=8
  → **9 cumulativo** (P237+P238 reescrito+P240+P241+P242+P243+
  P244+P245+P246). Lição refinada N=9: "mapear empíricamente
  distribuição de usos por sub-módulo antes de fixar arquitectura
  de migração".

**Anti-inflação 38ª aplicação cumulativa** pós-P205D — Opção α
extensão Regions minimal (1 field + 3 métodos) + Opção α API
substitui atribuições directas + Opção α preservação fields
legacy `cell_origin_x/y` (transição não-completa consciente) +
Opção β L0 minimal (region.md hash propagado) + Opção α
anotação cumulativa minimal ADRs + Opção α sub-padrão N=1
inaugurado.

---

## §8 Próximo sub-passo pós-P246

P246 fecha cell layout migration. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 breakable per-cell activação real** | Materializar semantic `Block.breakable` + `Boxed.height` + `TableCell` overflow dentro célula | **M (~2-4h)** | **alta** (primeira oportunidade pós-P246 desbloqueio) |
| Refino A.4 outset/fill/stroke Block+Boxed | 3 de 4 scope-outs restantes pós-P242 | S-M por attr | média |
| ADR-0079 → IMPLEMENTADO graded | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| ADR meta admin XS | Formalizar "passo administrativo XS" N=6 (P244 limiar) ou sub-padrões N=2 P244/P245/P246 | XS | média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| DEBT novo `Region origin` opcional | Refactor Region com `origin: Point` permitindo eliminar `cell_origin_x/y` | M | baixa — débito latente |

**Recomendação subjectiva pós-P246**: **A.4 breakable per-cell
activação real**. Materializa o desbloqueio arquitectural que
P246 instala; magnitude M paridade P246. Sequente natural.

**Decisão humana fica em aberto literal** pós-P246.

**Estado pós-P246**:
- Tests workspace: 2203 → **2209 verdes** (+6 P246 entity-side).
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado.
- **Layouter fields: -2** (cell_available_h + cell_origin_w
  removidos); cell_origin_x/y preservados legacy.
- **Regions fields: 3 → 4** (+cell).
- **Regions methods: +3** (effective, enter_cell, exit_cell).
- Stdlib funcs: 64 preservado.
- §A.5 distribuição: preservada literal.
- Cobertura Layout per metodologia: ~93-94% preservado.
- Cobertura user-facing total: ~75-76% preservado.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM
  VIGOR 29; IMPLEMENTADO 23; total **68 preservado**. ADR-0079
  Categoria A.4 desbloqueada arquiteturalmente P246. ADR-0080
  sub-categoria nova N=1 inaugurada. ADR-0061 §"Refino futuro"
  anotada.
- **Saldo DEBTs: 11 preservado** (DEBT novo opcional não-aberto
  per política P158; DEBT-34c + DEBT-37 sentinelas preservadas).
- **38 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P246** (2):
  - "Layouter consumer migration via API wrapper" N=1
    inaugurado P246.
  - "Spec C1 audit obrigatório bloqueante" N=8 → **9 cumulativo**.
- **Categoria A Fase 5 Layout**: 5/5 + parcial A.4 P242 +
  **arquiteturalmente desbloqueada P246**.
- **Categoria B Fase 5 Layout**: 3/3 preservado.
- **Categoria C.1 Fase 5 Layout**: cumprida P245.
- **Categoria C.2 Fase 5 Layout**: pendente; cell layout migration
  P246 é pré-requisito desbloqueador (não cumpre C.2 mas reduz
  risco arquitectural).
- **Categoria D Fase 5 Layout**: 3/? preservado.
- **Fase 5 Layout candidata**: 15/13-15 → **15/13-15** (P246
  refactor não-feature; não-incrementa contador feature).
- **Marco interno**: cell layout migration completa; 4 fields
  Layouter reduzidos a 2 + API Regions; A.4 breakable per-cell
  arquiteturalmente desbloqueado; padrão "Layouter consumer
  migration via API wrapper" inaugurado N=1; lição C1 audit
  N=9 cumulativo refinada.
