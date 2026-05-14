# Relatório do passo P242 — M7+5 A.4 radius/clip infrastructure (M9d terceira sub-passo; primeira sub-passo M7+ não-pipeline; primeira promoção real graded scope-out ADR-0054 P156G/H → semantic concreta; terceira excepção justificada ADR-0080 EM VIGOR sub-categoria nova)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-242.md`.
**Tipo**: feature geometry + L0/L1 + L3 exporter; **NÃO** é
walk-time refactor (distinto P240/P241). Refino estrutural de
`ShapeKind` + tipo container `Corners<T>` novo + refino tipo
`Content::Block.radius` + `Content::Boxed.radius`
`Option<Length>` → `Corners<Length>` per-corner + materialização
clip semantic via `FrameItem::Group` + PDF exporter Bezier 4
corners.
**Magnitude planeada**: M-L (~3-5h). **Magnitude real**: M+
(~2.5h audit + implementação + tests + L0 + ADRs + relatorio).
**Marco**: terceira sub-passo materialização M9d / M7+;
**primeira sub-passo M7+ não-pipeline** (P240/P241 walk-time
refactor vs P242 geometry isolada); **primeira promoção real
graded ADR-0054** de scope-out P156G/H → semantic concreta;
**terceira excepção justificada ADR-0080 EM VIGOR pós-P229**
sub-categoria nova "geometry/exporter"; quinta aplicação cumulativa
pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=4 → 5 cumulativo.

---

## §1 O que foi feito

P242 materializa M7+5 per ADR-0081 IMPLEMENTADO parcial (2/5
pós-P241 → 3/5 pós-P242). Quatro adições estruturais
ortogonais à pipeline (não toca `apply_state_displays` /
`apply_counter_displays`):

1. **Novo tipo `Corners<T>`** em `01_core/src/entities/corners.rs`
   paralelo absoluto `Sides<T>` P156C.
2. **Novo variant `ShapeKind::RoundedRect { radii: Corners<Length> }`**
   em `entities/geometry.rs`.
3. **Refino tipo `Content::Block.radius` + `Content::Boxed.radius`**
   `Option<Length>` → `Corners<Length>` per-corner. Audit C1 P242
   refinou hipótese spec (assumira "5 fields → 7" mas Block/Boxed
   já tinham 8 fields P231; ajuste real = refine type).
4. **PDF exporter (L3)** desenha rounded-rect via Bezier 4 corners
   path via `emit_rounded_rect_ops` helper novo (5 sítios
   cross-arm reuso).

**2175 → 2190 verdes** (+15 novos; 0 regressões; 7 adaptações
triviais P231). Audit C1 P242 refinou naming/hipóteses sem
`P242.div-N` formal (paridade lição N=5 cumulativo).

---

## §2 Auditoria pré-P242 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=5 cumulativo

**Audit empírico** (paralelo lição refinada `P236.div-1 →
P238.div-1 → P239 audit → P240 audit → P241 audit` N=5 cumulativo):

| Aspecto auditado | Hipótese Spec | Realidade Empírica | Implicação |
|------------------|---------------|--------------------|------------|
| `Sides<T>` shape | `{ left, top, right, bottom }` + uniform + Default | ✓ Confirmado | Pattern espelho directo Corners |
| `Content::Block` fields baseline | 5 fields (body/width/height/inset/breakable) | **8 fields já existem** P231 (+outset/radius/clip "semantic adiada") | **Material divergence**: ajuste é refine type, não add fields |
| `Content::Boxed` fields baseline | 5 fields paralelo | **8 fields já existem** P231 paralelo | Idem Block |
| `radius` actual | scope-out P156G/H | `radius: Option<Length>` P231 graded | P242 refina `Option<Length>` → `Corners<Length>` |
| `clip` actual | scope-out P156G/H | `clip: bool` P231 + "semantic adiada" | P242 mantém type bool; materializa semantic via clip_mask emit |
| Stdlib `radius:` actual | rejected | **Aceita Length uniforme** P231 | P242 estende para Length OR Dict |
| Stdlib `clip:` actual | rejected | **Aceita Bool** P231 | P242 preserva; materializa semantic |
| `Length::ZERO` | constante | ✓ disponível em `layout_types.rs:582` | Default `Corners::uniform(Length::ZERO)` |
| `Value::Array(Vec<Value>)` | ✓ existe | OK | (não-usado P242; mas P241 já valida) |
| DEBT-30 + Group.clip_mask | ENCERRADO P79 + `Option<ShapeKind>` field | ✓ Confirmado | Reuso absoluto pattern + add arm RoundedRect |
| Vanilla kappa | 0.5523 | **0.552_284_749_831** já em Ellipse | Reuso valor literal paridade ADR-0033 |
| Tests baseline pré-P242 | 2175 verdes | ✓ Confirmado | Baseline para +12-18 |

**Audit C1 refinou hipótese spec material**: spec assumiu
operação "add 2 fields"; reality é "refine field type". Spec
intent preserved (move to per-corner Corners<Length>); ajuste
HOW não WHERE. **Sem `P242.div-N`** — paridade lição N=5
cumulativo precedente.

---

## §3 `Corners<T>` + `ShapeKind::RoundedRect` (C2+C3)

`01_core/src/entities/corners.rs` (ficheiro novo):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners<T> {
    pub top_left:     T,
    pub top_right:    T,
    pub bottom_right: T,
    pub bottom_left:  T,
}

impl<T> Corners<T> {
    pub fn new(top_left: T, top_right: T, bottom_right: T, bottom_left: T) -> Self;
}

impl<T: Clone> Corners<T> {
    pub fn uniform(value: T) -> Self;
}

impl<T: Default> Default for Corners<T>;
```

**Ordem horária** começando top-left (paridade vanilla).
**Sub-padrão #14 "Tipo entity em ficheiro próprio" N=5 → 6
cumulativo** (Sides → Parity → Dir → BibEntry → CitationForm →
**Corners**).

`01_core/src/entities/geometry.rs`:

```rust
pub enum ShapeKind {
    Rect,
    RoundedRect {
        radii: crate::entities::corners::Corners<crate::entities::layout_types::Length>,
    },   // P242
    Ellipse,
    Line { dx: f64, dy: f64 },
    Path(Vec<PathItem>),
}
```

**Degeneração estrutural preservada**: radii zero ≠ Rect
(PartialEq distinto; não normaliza). **ShapeKind variants**:
4 → **5** (+RoundedRect).

L0 prompts tocados: `corners.md` NOVO + `geometry.md` (secção
RoundedRect).

---

## §4 Refino `Content::Block.radius` + `Content::Boxed.radius` (C4)

```rust
// Block (paridade exata Boxed):
- radius: Option<Length>,          // P231 graded
+ radius: Corners<Length>,         // P242 per-corner
```

**Default migrado**:
```rust
// Construtores Content::block / Content::boxed:
- radius: None,
+ radius: Corners::uniform(Length::ZERO),
```

Audit C1 P242 refinou hipótese spec "5 fields → 7" (real:
already 8 P231; ajuste = refine type). Backwards compat preservado
via uniform construction.

Arms cascata afectados (compiler-driven): `PartialEq` arm Block
+ Boxed; map_content; map_text; pattern-match cascata Block/Boxed
~17 sítios cumulativos.

**Tests adaptados** (7 sítios P231 que usavam `radius: Some(len)`
→ `radius: Corners::uniform(len)` ou `Corners::uniform(Length::ZERO)`):
- `p231_block_variant_aceita_outset_radius_clip` (asserts
  per-corner).
- `p231_boxed_variant_aceita_outset_radius_clip`.
- `p231_block_partial_eq_inclui_3_fields`.
- `p231_block_map_content_preserva_3_fields`.
- `p231_block_outset_radius_clip_layout_preservado` (P242 ajustou
  recursive text collection — body items agora inside Group).
- `p231_boxed_cosmeticos_paridade_block`.
- Tests stdlib `p231_native_block_radius_5pt_aceita` /
  `p231_native_box_paridade_block` / `p231_native_block_3_fields_simultaneos`
  / `p231_native_block_outset_radius_clip_defaults` (asserts
  per-corner ou uniform).

**Counts pós-P242**: Content variants 62 preservado (refino
field-add não-add); Block fields 8 preservado; Boxed fields 8
preservado.

---

## §5 Stdlib radius `extract_corners_length_value` + walk integration (C5+C6)

`01_core/src/rules/stdlib/layout.rs` ganha helper novo:

```rust
fn extract_corners_length_value(value: &Value, fn_name: &str)
    -> SourceResult<Corners<Length>>
```

**Aceita 2 formas**:
- **`Value::Length(L)`** (ou coerções via `extract_length`) →
  `Corners::uniform(L)`.
- **`Value::Dict(d)`** com keys: `top-left` / `top-right` /
  `bottom-right` / `bottom-left` / `top` / `bottom` / `left` /
  `right` / `rest`.

**Precedência**: canto específico > eixo > rest (paridade
`extract_sides_lengths` per ADR-0064 Caso C).

**Validação**: negativos rejeitados; chaves canto inválidas
rejeitadas.

**Sub-padrão "Reuso template helpers extract_*"** N=3 → **4
cumulativo** (`extract_corners_length_value` via template
`extract_sides_lengths` P156L).

stdlib `block(radius:)` + `box(radius:)` refactor:
```rust
let radius = match args.named.get("radius") {
    Some(val) => extract_corners_length_value(val, "block")?,
    None => Corners::uniform(Length::ZERO),
};
```

Layouter Block arm (`01_core/src/rules/layout/mod.rs`):
```rust
if *clip {
    // Snapshot-and-extract: layout body normalmente; extrair items;
    // re-emit como Group com clip_mask.
    let pos_block   = Point { x: line_start_x, y: cursor_y };
    let items_before = current_items.len();
    let y_before     = cursor_y;
    self.layout_content(body);
    self.flush_line();
    let body_items = current_items.drain(items_before..).collect();
    let inner_h = (cursor_y - y_before).0;
    let clip_shape = if radius_is_zero {
        ShapeKind::Rect
    } else {
        ShapeKind::RoundedRect { radii: *radius }
    };
    current_items.push(FrameItem::Group {
        pos: pos_block, matrix: TransformMatrix::identity(),
        clip_mask: Some(clip_shape),
        inner_width: available_w, inner_height: inner_h,
        items: body_items,
    });
} else {
    // Caminho original inline preservado.
    self.layout_content(body);
    self.flush_line();
}
```

**Layouter permanece puro** — sem Engine+ctx em signature;
paridade arquitectural P240/P241 preservada.

---

## §6 PDF exporter rounded-rect Bezier 4 corners (C7)

`03_infra/src/export.rs` ganha helper novo:

```rust
fn emit_rounded_rect_ops(
    ops: &mut String,
    x: f64, y: f64, w: f64, h: f64,
    radii: &Corners<Length>,
)
```

**Algoritmo Bezier 4 corners**:
1. `m` posição inicial top-edge (`x + tl, y_top`).
2. `l` line para `x_right - tr, y_top`.
3. `c` cubic top-right corner (skip se `tr == 0`).
4. `l` line para `x_right, y_bottom + br`.
5. `c` cubic bottom-right corner (skip se `br == 0`).
6. `l` line para `x_left + bl, y_bottom`.
7. `c` cubic bottom-left corner (skip se `bl == 0`).
8. `l` line para `x_left, y_top - tl`.
9. `c` cubic top-left corner (skip se `tl == 0`).
10. `h` close path.

**Kappa**: `0.552_284_749_831` (paridade `ShapeKind::Ellipse`
mesmo ficheiro). **Clamp radii** a `min(w,h)/2.0` (evita overflow
geométrico).

**Reusado em 5 sítios cross-arm**:
- `draw_item_global` Shape (page-relative; pos.x.val(), pdf_y).
- `emit_shape_path_local` (local space; 0, -height).
- `draw_item_local` Shape (após cm; pos.x.0, local_y).
- 2× `draw_item` duplicados em paths Shape (similar arms).

L0 prompts tocados: `export.md` (secção rounded-rect clip path
Bezier).

---

## §7 Decisões substantivas (9 decisões fixadas incl. Decisão 0 lição N=5 cumulativo) + terceira excepção justificada ADR-0080 EM VIGOR

**9 decisões fixadas P242** (Decisão 0 = lição N=5 cumulativo
P237 + P238 reescrito + P240 + P241 + P242):

| # | Decisão | Opção fixada |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=5 aplicada; sem `P242.div-N` (audit refinou hipóteses fields) |
| 1 | `Corners<T>` paralelo absoluto `Sides<T>` | ✓ |
| 2 | `ShapeKind::RoundedRect { radii: Corners<Length> }` | ✓ (degeneração estrutural preservada) |
| 3 | Refino tipo radius (não add) | `Option<Length>` → `Corners<Length>` per-corner |
| 4 | `radius:` aceita Length OR Dict | **Opção α** com precedência específico > eixo > rest |
| 5 | `clip:` Bool com semantic materializada | Layouter emite Group com clip_mask |
| 6 | radius sem clip preserva graded | radius armazenado mas sem clip_mask emit |
| 7 | L0 partial tocado | **Terceira excepção justificada ADR-0080 sub-categoria nova "geometry/exporter"** |
| 8 | Promoção real graded scope-out | Sub-padrão emergente N=1 inaugurado |
| 9 | Sem fechamento Fase 5 graded | M7+3/M7+4 pendentes |

**Terceira excepção justificada ADR-0080 EM VIGOR pós-P229**:

**Sub-categoria diferente** de P240/P241 (walk-time runtime): P242
é "L0 tocado para geometry/exporter infrastructure". Justificada
pelo mesmo critério estructural (4+ entidades novas + cross-camada
L1/L3) mas semanticamente distinta.

L0 partial tocado (4 ficheiros):
- `00_nucleo/prompts/entities/corners.md` — **ficheiro novo**.
- `00_nucleo/prompts/entities/geometry.md` — secção
  `ShapeKind::RoundedRect`.
- `00_nucleo/prompts/entities/content.md` — refino `Block.radius`
  + `Boxed.radius` + materialização clip semantic.
- `00_nucleo/prompts/infra/export.md` — secção rounded-rect clip
  path Bezier 4 corners.

**ADR-0080 §"Excepção P242"** anotada formalmente cristalizando
N=3 cumulativo com 2 sub-categorias formalizadas:
- walk-time runtime (N=2 P240+P241).
- geometry/exporter (N=1 P242).

**Pattern emergente "promoção real scope-out ADR-0054 graded"
N=1 inaugurado P242** — sub-padrão novo distinto de refinos
qualitativos (P156L) ou cosméticos (P158C). Scope-out P156G/H
P231 "semantic adiada" → semantic concreta + render PDF real.

**Anti-inflação 34ª aplicação cumulativa** pós-P205D — Opção α
Corners paralelo + Opção α RoundedRect novo + Opção α refino
tipo (não add) + Opção α Length OR Dict + Opção β snapshot-extract
(não refactor pipeline) + Opção γ L0 partial terceira excepção
sub-categoria nova + Opção α sub-padrão promoção real scope-out
+ ADR-0081 IMPLEMENTADO parcial 3/5 (não completo).

---

## §8 Resultados verificação + tests E2E + pré-condições obrigatórias (C8+C10)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2191 verdes (range 2187-2193) | **2190 verdes** (1901+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | 4-5 L0 hashes + ~10-15 ficheiros L1 actualizados | ✓ (3 L0 + ~10 ficheiros L1 hashes propagados) |
| Adaptações pre-existentes | N=~20-25 | **N=7** (menor que spec; pattern paralelo absoluto + audit C1 refinado reduziu adaptação cumulativa) ✓ |
| Content variants | 62 preservado | ✓ |
| ShapeKind variants | 4 → 5 | ✓ |
| Block fields | 8 preservado (radius refinado type) | ✓ |
| Boxed fields | 8 preservado (radius refinado type) | ✓ |
| Tipos entity novos | +1 `Corners<T>` | ✓ |
| Helpers stdlib novos | +1 `extract_corners_length_value` | ✓ |
| ADR-0081 status | IMPLEMENTADO parcial 2/5 → 3/5 | ✓ (M7+5 ✓; M7+3+M7+4 pendentes) |
| ADR-0079 Categoria A.4 | scope-out P231 → materializado parcial P242 | ✓ |
| ADR-0080 §"Excepção P242" | anotada N=3 sub-categoria geometry/exporter | ✓ |
| L0 partial tocado | 4 ficheiros | ✓ |
| Regressões reais | 0 | **0** |

**Tests P242** (15 unit + cenários canónicos paridade P240/P241):

**Unit corners** (4 tests em `entities/corners.rs`):
- `p242_corners_new_preserva_4_cantos`.
- `p242_corners_uniform_clona_valor`.
- `p242_corners_default_zero_em_todos_cantos`.
- `p242_corners_clone_eq_partial_eq_funcionam`.

**Unit geometry** (2 tests em `entities/geometry.rs`):
- `p242_shapekind_rounded_rect_radii_zero_eq_rect_distinguivel`
  (degeneração estrutural preserva PartialEq distinto).
- `p242_shapekind_rounded_rect_radii_uniforme_pt_5`.

**Unit stdlib** (6 tests em `rules/stdlib/mod.rs`):
- `p242_native_block_radius_length_uniforme`.
- `p242_native_block_radius_dict_por_canto`.
- `p242_native_block_radius_dict_precedencia_eixo_rest`.
- `p242_native_block_radius_negativo_rejeita`.
- `p242_native_block_radius_chave_canto_invalida_rejeita`.
- `p242_native_box_radius_paridade_block`.

**Unit/E2E layout** (3 tests em `rules/layout/tests.rs`):
- `p242_block_clip_true_radius_non_zero_emit_group_rounded_rect_clip_mask`.
- `p242_block_clip_true_radius_zero_emit_group_rect_clip_mask`.
- `p242_block_clip_false_radius_non_zero_sem_clip_mask`
  (radius sem clip preserva graded per Decisão 6).

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2175 verdes pré-P242 → 2190
   verdes pós-P242 (+15 novos; 0 regressões reais; 7 adaptações
   triviais tests P231).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P242 NÃO toca trait Introspector nem methods (refino geometry
   isolada cross-camada L1/L3).
3. **Backward compat**: stdlib `block(radius: 5pt)` continua a
   funcionar via `Corners::uniform`; tests P231 adaptados;
   eval-time wrappers P236/P237 + walk-time runtime P240/P241
   intactos.

**Promoções ADR**:
- **ADR-0081 IMPLEMENTADO parcial 2/5 → 3/5** (M7+5 ✓; M7+3+M7+4
  pendentes). Distribuição preservada literal — sem novos ADRs
  criados; sem PROPOSTO ↔ IMPLEMENTADO. PROPOSTO 12; EM VIGOR 29;
  IMPLEMENTADO 22; total **68 preservado**.
- ADR-0079 Categoria A.4 scope-out P231 → **materializado parcial
  P242** anotada (radius+clip ✓; outset+fill+stroke restantes N=3
  scope-out).
- ADR-0080 §"Excepção P242" anotada N=3 cumulativo sub-categoria
  nova geometry/exporter.
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁶¹** adicionada (~250 linhas)
documentando: M7+5 promoção real graded materializado; lição N=5
cumulativo C1 audit validada; 4 patterns emergentes; sub-padrão
"promoção real scope-out ADR-0054 graded" N=1 inaugurado;
sub-padrão #14 "Tipo entity em ficheiro próprio" N=5 → 6;
terceira excepção justificada ADR-0080 sub-categoria nova; 9
decisões fixadas; Categoria A.4 transita scope-out → materializado
parcial.

---

## §9 Próximo sub-passo pós-P242

P242 completa M7+5 (M9d terceira sub-passo; primeira sub-passo
M7+ não-pipeline). Restantes 2 sub-passos M9d/M7+ pendentes
(magnitude cumulativa restante ~13-20h).

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+3 multi-region completion cell-level** | `Regions { current, backlog, last }`; DEBT-56b candidato | **L+ (~8-12h)** | **alta** (maior desbloqueio cumulativo restante — C.2 + A.4 breakable per-cell) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1; isolada) |
| Refino A.4 — `outset` + `fill` + `stroke` Block+Boxed | Completar 3 dos 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| ADR meta admin XS | Promoção formal patterns N=2-4 acumulados pós-P242 (L0 tocado sub-categorias N=3; refino paralelo callers fixpoint N=2; tipo entity #14 N=6; reuso template extract_* N=4; promoção real scope-out N=1) | XS por pattern | média (consolidação meta cumulativa) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~85-92% (13/13-15 sub-passos) | XS | baixa |

**Recomendação subjectiva pós-P242**: **M7+3 multi-region
completion**. Maior desbloqueio cumulativo restante (C.2 + A.4
breakable per-cell + DEBT-56b candidato fechar); magnitude L+
(~8-12h) — sub-passo grande mas é o último grande caminho M7+
pendente. Alternativa: M7+4 Place float (magnitude L isolada;
desbloqueia C.1; sem dependências M7+3).

**Decisão humana fica em aberto literal** pós-P242.

**Estado pós-P242**:
- Tests workspace: 2175 → **2190 verdes** (+15 P242).
- Content variants: 62 preservado.
- **ShapeKind variants: 4 → 5** (+RoundedRect).
- Block fields: 8 preservado (radius refinado type).
- Boxed fields: 8 preservado (radius refinado type).
- **Tipos entity novos: +1 Corners<T>**.
- Stdlib funcs: 64 preservado.
- **Helpers stdlib novos: +1 `extract_corners_length_value`**.
- §A.5 distribuição: preservada (sem mudança categoria Layout/Model).
- Cobertura Layout per metodologia: 89% → **~91-92%** (refino
  qualitativo+quantitativo — **primeira aplicação Layout
  pós-P156L** pós série Model + série M7+ walk-time).
- Cobertura user-facing total: ~72% → **~73-74%** (A.4 radius+clip
  real bonus cumulativo).
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  2/5 → **3/5** internamente. ADR-0079 Categoria A.4
  "materializado parcial P242" anotada. ADR-0080 §"Excepção P242"
  anotada N=3 sub-categoria geometry/exporter.
- **Saldo DEBTs: 11 preservado**.
- **34 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes inaugurados/consolidados P242** (4):
  - "Promoção real scope-out ADR-0054 graded" N=1 inaugurado P242.
  - "Tipo entity em ficheiro próprio" (sub-padrão #14) N=5 →
    **6 cumulativo** (Corners adiciona-se).
  - "Reuso template helpers extract_*" N=3 → **4 cumulativo**.
  - "Spec C1 audit obrigatório bloqueante" N=4 → **5 cumulativo**.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado (D.1+D.2+D.3 pós-P241; P242 é Categoria A).
- **Categoria A.4 Fase 5 Layout**: scope-out P231 → **materializado
  parcial P242** (radius+clip ✓; outset+fill+stroke N=3 restantes
  scope-out).
- **Fase 5 Layout candidata: 12/13-15 → 13/13-15 sub-passos
  materializados** (~85-92% cumulativo).
- **M9d / M7+ progresso**: **3/5 sub-passos materializados** (M7+1
  ✓; M7+2 ✓; **M7+5 ✓**; M7+3 + M7+4 pendentes; cumulativa restante
  ~13-20h).
- **Marco interno**: terceira sub-passo materialização M9d
  validada; **primeira sub-passo M7+ não-pipeline** (P242 geometry
  vs P240/P241 walk-time); primeira aplicação real do sub-padrão
  "promoção real scope-out ADR-0054 graded"; audit C1 P242 refinou
  hipótese spec fields sem div-N — paridade lição N=5 cumulativo.
  **Distinção qualitativa P242 vs P240/P241**: refino
  qualitativo+quantitativo (Layout +2 pp per metodologia) vs
  refino apenas qualitativo (Introspection P240/P241).
