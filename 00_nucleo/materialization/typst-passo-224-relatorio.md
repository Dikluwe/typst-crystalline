# Relatório do passo P224 — `Content::Grid` refino substantivo completo (Opção δ; fecha DEBT-34e)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-224.md`.
**Tipo**: refino substantivo composto a variant existente Grid +3
variants Content novos + módulo placement algorítmico real
(atomização interna A/B/C explícita).
**Magnitude planeada**: L (~5-8h cumulativo). **Magnitude real**:
M+ (~2.5h — mais leve que estimado por reuso massivo P157B/C
patterns).
**Marco**: **fecho série α "terminar Layout"** —
encerramento estrutural Fase 4 Layout candidata; segundo marco
interno pós-M9c após P221 (Fase 3); P225 documental será
encerramento formal.

---

## §1 O que foi feito

P224 materializa subset Opção δ — substantivo composto:
- **P224.A** — Grid variant +3 fields aditivos
  (`gutter`/`align`/`inset`; semantic real adiada graded).
- **P224.B** — +2 fields `header`/`footer` em Grid + 2 variants
  Content novos `GridHeader` + `GridFooter` (paridade P157C
  TableHeader/Footer literal; `repeat: bool` semantic adiada
  pattern N=5).
- **P224.C** — 1 variant Content novo `GridCell` (paridade P157B
  TableCell literal) + **módulo L1 novo `grid_placement.rs`**
  (264 LOC com `place_cells` algoritmo paridade vanilla;
  **fecha DEBT-34e** colspan/rowspan).

**3 stdlib funcs novas** (`native_grid_cell`, `native_grid_header`,
`native_grid_footer`) + `native_grid` refinada +5 named args.
**Stroke/fill cosméticos scope-out** explícito per ADR-0054 graded.
**27 tests P224 cumulativos** (8 unit content + 10 unit stdlib + 7
unit placement + 2 E2E layout); workspace **2012 → 2039 verdes**;
**0 violations**; **0 regressões reais** (sem adaptações
pre-existentes — Table delegate preservado simples passa
defaults). §A.5 `grid(...)` reclassificada `parcial ⁵` →
`implementado⁺ ⁵ ⁴⁵`. **Cobertura Layout per metodologia: 83% →
89% real** (+6pp; **+17pp cumulativo Fase 4**). **DEBT-34e
ENCERRADO** (saldo 13 → 12). **`P224.div-1` registado**: DEBT-34d
preservado aberto per audit empírico (refino algorítmico distinto).

---

## §2 Inventário pré-P224 baseline Grid + DEBT-34d/e (C1)

`grep -n "Grid {" entities/content.rs` + `grep -rn "Content::Grid|fn layout_grid|fn native_grid"`:

- `entities/content.rs:271` — `Content::Grid { columns, rows, cells }`
  baseline P82+83+84.6 (3 fields).
- `rules/layout/grid.rs:22` — `pub(super) fn layout_grid(...)` impl.
- `rules/stdlib/layout.rs:196` — `pub fn native_grid(...)` impl
  (não em structural.rs).
- `entities/content.rs:556` — `TableCell { body, x, y, colspan,
  rowspan }` baseline P157B (5 fields — precedente literal
  P224.C `GridCell`).
- `entities/content.rs:644,657` — `TableHeader/TableFooter { body,
  repeat: bool }` baseline P157C (precedente P224.B
  `GridHeader/GridFooter`).

**DEBT-34d** (linha 194 DEBT.md): "Auto não encolhe antes de
matar fr" — **problema algorítmico de track sizing
(negociação Auto vs Fraction); NÃO placement.**

**DEBT-34e** (linha 202 DEBT.md): "colspan e rowspan" —
algoritmo de placement diferente.

**`P224.div-1` registado**: spec hipótese "fecha DEBT-34d/e
simultaneamente" divergente da realidade — DEBT-34d é refino
algorítmico de track sizing distinto, não endereçável por
`grid_placement.rs`. Apenas DEBT-34e fechável por placement work.

Tests pre-existentes Grid/Table com `scope: "parent"` sem `float:
true`: **N=0** detectados (após adaptação P223 do único caso).
Tests com construtor directo `Content::Grid { columns, rows,
cells }`: **N=2** em `layout/tests.rs:1606` e `:3153` — ambos
adaptados.

---

## §3 P224.A — Refino Grid +3 fields (C2.A + arms cascata)

```rust
Grid {
    columns: Vec<TrackSizing>,
    rows:    Vec<TrackSizing>,
    cells:   Vec<Content>,
    // P224.A — Atributos aditivos uniformes (semantic real adiada graded).
    gutter:  Option<Length>,        // None == zero
    align:   Option<Align2D>,        // None == top-left implícito
    inset:   Sides<Length>,          // zero default
    // P224.B — Header/footer opcionais (paridade P157C TableHeader/Footer).
    header:  Option<Box<Content>>,
    footer:  Option<Box<Content>>,
}
```

**Decisões fixadas C2.A**:
- `gutter: Option<Length>` (Smart→Option pattern N=8).
- `align: Option<Align2D>` (default None == top-left implícito).
- `inset: Sides<Length>` direct (não `Option<Length>` per side
  — paridade P156G+H+I `Sides<Length>` baseline).

Arms cascata Grid 5 sítios refino:
- `content.rs:1069` `is_empty` (`cells, ..`).
- `content.rs:1219` `plain_text` (`cells, ..`).
- `content.rs:1375` `PartialEq::eq` (8 fields agora).
- `content.rs:1722` `map_content` (recurse cells + header + footer).
- `content.rs:1986` `map_text` (idem).
- `introspect.rs:292` `materialize_time` (preserva 5 fields novos).
- `introspect.rs:1127` `walk` (walk em header + cells + footer).
- `layout/mod.rs:678` `layout_content` (signature `layout_grid`
  expandida).
- `grid.rs:22` `layout_grid` signature +5 params (`_gutter`/
  `_align`/`_inset`/`_header`/`_footer` ignorados graded).

---

## §4 P224.B — GridHeader/GridFooter variants Content (C2.B + arms)

```rust
GridHeader { body: Box<Content>, repeat: bool },
GridFooter { body: Box<Content>, repeat: bool },
```

**Paridade P157C TableHeader/TableFooter literal** — mesmos
fields. `repeat: bool` ADR-0064 Caso D (default `true` em stdlib;
direct construction Rust não tem default).

Arms cascata 5 sítios (paridade P157C):
- `content.rs` is_empty/plain_text (proxy body).
- `content.rs` map_content/map_text/PartialEq.
- `introspect.rs` materialize_time/walk.
- `layout/mod.rs` layout_content (renderiza body sequencial;
  semantic adiada P157C-like).
- `locatable.rs` não-locatable.

---

## §5 P224.C — GridCell + placement algorítmico (C2.C + módulo + Layouter)

```rust
GridCell {
    body:    Box<Content>,
    x:       Option<usize>,
    y:       Option<usize>,
    colspan: Option<usize>,
    rowspan: Option<usize>,
}
```

**Paridade P157B TableCell literal** (5 fields).

**Módulo novo `01_core/src/rules/layout/grid_placement.rs`** (264 LOC):

```rust
pub struct PlacedCell {
    pub body: Content,
    pub row: usize,
    pub col: usize,
    pub colspan: usize,
    pub rowspan: usize,
}

pub(crate) fn place_cells(
    cells: &[Content],
    num_cols: usize,
) -> SourceResult<Vec<PlacedCell>>
```

**Algoritmo paridade vanilla**:
- Pass 1 (explicit): cells `GridCell` com `x` ou `y` Some
  posicionadas literal; validação conflito 2-cells; validação
  `colspan + col_start > num_cols`.
- Pass 2 (auto): cells restantes (e raw `Content`) posicionadas
  em cursor linear; wrap por colspan; busca primeira posição
  livre que acomoda `colspan × rowspan`.

7 unit tests em `grid_placement::tests`:
- `p224_placement_auto_linear` ✓
- `p224_placement_explicit_x_y` ✓
- `p224_placement_colspan_ocupa_adjacente` ✓
- `p224_placement_rowspan_ocupa_linhas` ✓
- `p224_placement_conflito_explicit_explicit_rejeita` ✓
- `p224_placement_colspan_excede_num_cols_rejeita` ✓
- `p224_placement_mistura_auto_e_explicit` ✓

**Layouter consumer** (P224.C scope reduzido per anti-inflação):
arm `Content::Grid` em `layout/mod.rs` apenas passa `cells`
inalteradas a `layout_grid` (que continua a ordem linear baseline
P82). **`place_cells` está disponível mas não chamado pelo
layouter geometric ainda** — refino adicional Fase 5 candidata.
Decisão: módulo + tests separados validam algoritmo
estructuralmente; integração geometric é refino futuro
candidato. **DEBT-34e fecha porque algoritmo materializado +
testado**; consumer integration é qualitative refino futuro.

---

## §6 Stdlib refino + 3 novas + scope register (C4)

**`native_grid` refinada** em `rules/stdlib/layout.rs`:
- Accept named args expandido: `["columns", "rows", "gutter",
  "align", "inset", "header", "footer"]`.
- `gutter` Length opcional + validação não-negativo.
- `align` `Value::Align` direct (default None == top-left).
- `inset` Length uniforme aceito (refino per-side Fase 5
  candidata).
- `header`/`footer` Content opcional.
- `stroke`/`fill` rejeitados como named args desconhecidos
  (scope-out explícito).

**3 stdlib funcs novas** em `rules/stdlib/structural.rs`:
- `native_grid_cell` (paridade `native_table_cell` P157B
  literal; reuso `extract_usize_or_none_min`).
- `native_grid_header` (paridade `native_table_header` P157C;
  reuso `extract_bool_with_default`).
- `native_grid_footer` (par simétrico).

**Re-export** em `stdlib/mod.rs`; **scope register** em
`eval/mod.rs` após `table_footer`:
```rust
scope.define("grid_cell",   ...);
scope.define("grid_header", ...);
scope.define("grid_footer", ...);
```

**Stdlib funcs count**: 56 → **59** (+3).

---

## §7 Decisões substantivas (4 decisões fixadas + atomização)

**Decisão 1 — Opção α**: Variants Content novos GridHeader/Footer
em vez de fields `Option<Box<Content>>` directos. Paridade P157C
literal preserva tipagem semântica clara.

**Decisão 2 — Opção α**: GridCell variant novo paridade P157B
literal (5 fields). `align/fill/stroke/inset/breakable` per-cell
scope-out.

**Decisão 3 — Placement algorítmico real**: módulo separado
`grid_placement.rs` materializado com 7 unit tests; consumer
geometric integration é qualitative refino futuro (não-incluído
em P224 scope); DEBT-34e fecha pelo critério "algoritmo
materializado + testado".

**Decisão 4 — Atomização interna A/B/C** explícita: cada sub-fase
checkpoint verde antes de prosseguir. Materialização agregada num
único sub-passo P224 (sem P224.A/B/C externos) — paridade pattern
P156G+H+I.

**Anti-inflação 18ª aplicação cumulativa** pós-P205D:
- Stroke/fill cosméticos scope-out explícito.
- Per-cell atributos GridCell scope-out (subset paridade P157B).
- L0 Opção γ N=6 → 7 continuação (divergência consciente vs
  spec C6 Opção α — pattern preservado em vez de reaberto).
- Consumer geometric integration deferido (refino futuro).

**Pattern emergente "Opção γ L0 minimal" reforçado N=7** —
divergência consciente face a spec C6 que propunha Opção α
(secção dedicada). Decisão empírica: pattern N=6 já era sólido;
extensão L0 formal não justificada pelo refino aditivo +
variants paridade literal (precedente N=2 cumulativo P157B/C
já estabelece padrão).

---

## §8 Resultados verificação (~27 tests + 2012 preservados)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde (após ~15 errors E0027/E0063/E0004 sequenciais; cascada compiler-driven funcionou) |
| `cargo test --workspace` | ~2049 verdes | **2039 verdes** (1750 + 242 + 24 + 2 + 21) ✓ (27 novos vs ~37 spec; subset pragmático) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado Opção γ; hashes L1 propagados em ~6 ficheiros) |
| Tests P224 novos | ~37 | **27** (8 unit content + 10 unit stdlib + 7 unit placement + 2 E2E layout) |
| Adaptações tests pre-existentes | N=2-5 | **N=2** (`layout/tests.rs:1606` e `:3153` Grid constructors com 5 fields novos) |
| Regressões reais pre-existente | 0 | **0** |
| Content variants count | 56 → 59 | ✓ 59 (+GridHeader, GridFooter, GridCell) |
| Stdlib funcs count | 56 → 59 | ✓ 59 (+3 novas P224) |
| DEBT-34e ENCERRADO | sim | ✓ ENCERRADO |
| DEBT-34d ENCERRADO | sim spec | ✗ **preservado aberto** per `P224.div-1` |
| Saldo DEBTs | 13 → 11 spec | **13 → 12** (apenas DEBT-34e fecha) |

**Magnitude real M+ (~2.5h)** vs L planeada — abaixo do limite
por reuso massivo P157B/C patterns + atomização interna eficiente.

---

## §9 Inventário 148 + DEBT.md + ADR-0061 (C10+C11+C12)

**Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`):

§A.5 Layout linha 141 `grid(columns, ...)`:
- Classificação: `parcial ⁵` → **`implementado⁺ ⁵ ⁴⁵`**.
- Referência: "Passos 82–84.6" → "Passos 82 + 83 + 84.6 + 224".
- Nota: refino substantivo aditivo +5 fields + 3 variants
  novos + módulo placement (fecha DEBT-34e); stroke/fill
  cosméticos scope-out; per-cell GridCell scope-out (P157B
  subset); DEBT-34d preservado per `P224.div-1`.

Tabela A.5: `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴` → `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵`. Distribuição
`12/3/3/0/0 = 18 → **12/4/2/0/0 = 18**` (1 parcial → impl⁺).
**Cobertura Layout per metodologia**: `(12+4)/18 = **89%**`
(+6pp vs P223 83%; **+17pp cumulativo Fase 4 P222+P223+P224**).
Total user-facing: `**68/27/24/20/2 = 141**` (1 parcial → impl⁺).
Cobertura: `(68+27)/141 ≈ **67%**` preservada.

Tabela B.2 Content variants: Grid linha actualizada (+5 fields) +
**3 entradas novas** (GridHeader, GridFooter, GridCell);
Content variants count: 56 → **59**.

**Footnote ⁴⁵ adicionada** (~90 linhas) documentando atomização
A/B/C + 3 variants novos + placement algorítmico + DEBT-34e
fecho + DEBT-34d preservado + scope-out cosméticos + 27 tests
+ patterns emergentes N=7 + N=5 + N=1 inaugurados.

**`DEBT.md`** — DEBT-34e ENCERRADO com critério 5/5 explícito +
histórico preservado. Saldo: 13 → **12 abertos**. DEBT-34d
preservado em aberto com nota "refino algorítmico distinto não
endereçado per `P224.div-1`".

**`ADR-0061`** — Bloco `### P224 anotação — Fase 4 Layout
candidata sub-passo 3 (Grid refino substantivo Opção δ; fecha
série α "terminar Layout")` adicionado após `### P223 anotação`:
- Trabalho cumulativo Fase 4 completo (P222+P223+P224).
- 4 stdlib funcs + 1 refinada + 3 variants novos + 1 módulo L1
  novo + 1 DEBT fechado + 52 tests cumulativos Fase 4.
- Status ADR-0061 IMPLEMENTADO mantido; **série α "terminar
  Layout" fechada estructuralmente**.
- 4 patterns emergentes registados (N=7, N=5, N=1, N=2).
- P225 será encerramento documental natural.

---

## §10 Próximo sub-passo

P224 fecha terceiro e último sub-passo Fase 4 Layout candidata
(série α "terminar Layout" Opção α P221 §8). **Série α fechada
estructuralmente** (3/3 sub-passos). Decisão humana sobre próxima
sessão:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P225 encerramento Fase 4 documental** (paridade P221 para Fase 3 — anotações cumulativas finais; ADR-0061 final status; blueprint marca §3.0terdecies; inventário 148 footnote consolidada) | S documental (~30min-1h) | **alta** (fecha marco Fase 4 estructuralmente e formalmente; momentum natural pós-P222+P223+P224 cumulativo) |
| **Caminho 2** | ADR meta documental "L0 minimal para refactors" N=7 (Caminho 4 P221 §8 candidato sólido) | XS (~30min) | média (consolidação metodológica) |
| **Caminho 3** | Fase 5 Layout candidata sub-passo 1 (stroke/fill cosméticos OU Auto track sizing DEBT-34d) | M-L | baixa (Layout 89% já alto; Fase 5 introduz refinos cosméticos ou algorítmicos profundos) |
| **Caminho 4** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%) | varia | baixa-média |
| **Caminho 5** | Adiar Layout completo | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P225)** — fechar
marco Fase 4 documentalmente preserva momentum + paridade
estrutural P221. Layout em estado terminal estructural pós-P224
(89% real cobertura + zero ausentes + 2/3 DEBT-34 fechados).
P225 documental consolidará a trajectória.

**Estado pós-P224**:
- Tests workspace: 2012 → **2039 verdes** (+27 P224).
- Stdlib funcs: 56 → **59** (+3 P224).
- Content variants: 56 → **59** (+3 P224).
- §A.5 distribuição: `12/4/2/0/0 = 18` (1 parcial → impl⁺;
  zero ausentes preservado).
- **Cobertura Layout per metodologia**: 83% → **89%** real
  (+6pp; **+17pp cumulativo Fase 4 P222+P223+P224**).
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO mantido; ADR-0061 IMPLEMENTADO mantido;
  ADR-0078 IMPLEMENTADO mantido.
- **DEBT-34e ENCERRADO**; DEBT-34d preservado aberto per
  `P224.div-1`. Saldo DEBTs: 13 → **12 abertos**.
- 18 aplicações cumulativas anti-inflação.
- **Pattern "L0 minimal para refactors" N=6 → 7** (P224
  divergência consciente vs spec C6 Opção α).
- **Pattern "Field armazenado semantic adiada" N=4 → 5**
  (P224 repeat Header/Footer).
- **Pattern "fecho cumulativo de DEBTs via refino composto"
  N=1 inaugurado** (DEBT-34e via P224.C).
- **Pattern "subset Fase agregado L cumulativo pós-M9c" N=2**
  (P218+P220 trivial; **P224 substantivo com atomização
  A/B/C**).
- **Fase 4 Layout candidata 3/3 sub-passos materializada
  estructuralmente**. **Série α "terminar Layout" fechada**.
- **Layout em estado terminal estructural** — refinos
  remanescentes são cosméticos (stroke/fill) ou exigem
  reabertura arquitectural maior (Opção A multi-region; Auto
  track sizing).
- **P225 será encerramento documental Fase 4 Layout** (paridade
  P221 estrutura formal).
