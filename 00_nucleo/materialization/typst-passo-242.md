# Spec do passo P242 — M7+5 A.4 radius/clip infrastructure (M9d terceira sub-passo; promoção real graded ADR-0054 para `radius`+`clip` em Block+Boxed)

**Data**: 2026-05-14.
**Tipo**: feature geometry + L0/L1 + L3 exporter; **NÃO** é
walk-time refactor (distinto P240/P241). Refino estrutural de
`ShapeKind` + tipo container `Corners<T>` novo + scope-outs P156G/H
graded promovidos para real (`radius` + `clip`) em `Content::Block`
+ `Content::Boxed`; stdlib aceita os 2 named args; PDF exporter
desenha rounded-rect via Bezier 4 corners + emite `W n` per clip.
**Magnitude planeada**: M-L (~3-5h). Inferida pelo §8 dos
relatórios P240 e P241.
**Marco**: **terceira sub-passo materialização M9d / M7+ pós-P241**;
**primeiro sub-passo M7+ não-pipeline** (P240+P241 foram pipeline
walk-time; P242 é geometry isolada); **primeira promoção real
graded ADR-0054** de scope-outs P156G/H para semantic concreta
(ganho user-facing imediato; sem dependência pipeline);
quinta aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=4 → 5 cumulativo.

---

## §1 O que será feito

P242 materializa M7+5 per ADR-0081 IMPLEMENTADO parcial (2/5
pós-P241; M7+5 alvo aqui). Quatro adições estruturais
ortogonais à pipeline (não toca `apply_state_displays` /
`apply_counter_displays`):

1. **Novo tipo `Corners<T>`** em `01_core/src/entities/corners.rs`,
   paralelo absoluto a `Sides<T>` (P156C) — quatro fields
   `top_left`/`top_right`/`bottom_right`/`bottom_left`. Genérico
   `T`, derives padrão (Debug, Clone, Copy, PartialEq, Eq).

2. **Novo variant `ShapeKind::RoundedRect { radii: Corners<Length> }`**
   em `entities/geometry.rs`. Coexiste com `Rect` / `Ellipse` /
   `Line`. Quando todos os 4 cantos são zero, semantic é idêntica
   a `Rect` (degenera literal).

3. **Refino `Content::Block` + `Content::Boxed`** — adicionar
   fields `radius: Corners<Length>` e `clip: bool` (default
   zero/false). Os 2 attrs deixam de ser scope-out P156G/H —
   transitam para "implementados" (refino qualitativo per
   ADR-0054 graded "promoção real"). Stdlib accept named args
   `radius:` (Length uniforme ou dict por canto) e `clip:` (bool).

4. **PDF exporter (L3)** desenha rounded-rect via Bezier 4 corners
   quando `clip == true` E pelo menos um canto não-zero. Infraestrutura
   `FrameItem::Group.clip_mask: Option<ShapeKind>` (DEBT-30 fechado
   P79) reusada absoluta — apenas adicionar arm para `ShapeKind::RoundedRect`
   na função que gera o path PDF.

**Tests esperados**: 2175 → ~2191 verdes (+12 a +18 baseline;
range alinhado P240/P241 +12-14). Zero regressões esperadas
(refino aditivo a Block/Boxed; backwards compat literal —
`radius: Corners::default()` + `clip: false` é estado pré-P242
em todos os tests existentes).

**Audit C1 P242** deve refinar 2-3 hipóteses (paridade método
P240/P241): forma de `radius` em stdlib (Length uniforme vs
dict por canto), bezier kappa magic constant (0.5523), arms a
adicionar em pattern-match cascata Block/Boxed (~12 sítios por
relatório P156L).

---

## §2 Auditoria pré-P242 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=5 cumulativo

**Audit empírico obrigatório** antes de qualquer código tocado
(paralelo lição refinada `P236.div-1 → P238.div-1 → P239 audit
→ P240 audit → P241 audit` N=5 cumulativo). Aspectos críticos:

| Aspecto a auditar | Hipótese a confirmar | Implicação se falhar |
|-------------------|---------------------|----------------------|
| `Sides<T>` em `entities/sides.rs` | Forma `{ left, top, right, bottom }` (não CSS order); derives Debug/Clone/Copy/PartialEq/Eq; método `uniform(v)` para `T: Clone` | Pattern espelho directo para `Corners<T>` |
| `ShapeKind` actual em `entities/geometry.rs` | Variants `Rect | Ellipse | Line { dx, dy }` confirmados; derives Debug/Clone/PartialEq | Adicionar variant `RoundedRect` preserva derives |
| `Stroke` em geometry.rs | `{ paint: Color, thickness: f64 }` preservado | Sem alteração; rounded-rect usa mesmo Stroke |
| `Content::Block` fields pós-P156G | `{ body, width: Option<Length>, height: Option<Length>, inset: Sides<Length>, breakable: bool }` | Adicionar 2 fields novos: 5 → 7 fields; arms cascata afectados |
| `Content::Boxed` fields pós-P156H | `{ body, width, height, inset: Sides<Length>, baseline: Length }` | Adicionar 2 fields novos: 5 → 7 fields; arms cascata afectados |
| Scope-out P156G "9 attrs" + P156H "6 attrs" | Listas em §"Limitações conscientes" mencionam `radius` + `clip` como rejeitados em `native_block` / `native_box` com erro hard | **2 attrs deixam de ser rejeitados** — `radius` + `clip` transitam para aceites; rejeição actual continua para os outros 7/4 attrs |
| DEBT-30 status | ENCERRADO P79; `FrameItem::Group.clip_mask: Option<ShapeKind>` existe; exporter emite `W n` | Infraestrutura clip reusada absoluta; só adicionar arm RoundedRect ao path gen |
| Helper `extract_length` em `stdlib/layout.rs` | Reusado N=7+ (P156C-L); aceita Length uniforme | Pode ser reusado para `radius:` quando uniforme; dict por canto exige helper novo |
| Pattern-match cascata para refino field-add | P156L documentou "12 sítios pattern-match" para refactor de `Content::Pad` (sides) | Refino Block+Boxed afecta ~12-15 sítios per variant ≈ 24-30 sítios total; magnitude consistente M-L |
| Tests baseline pré-P242 | **2175 verdes** confirmado pós-P241 | Baseline para +12-18 |
| ADR-0079 Categoria A "render adiada graded P231" | Verificar texto exacto da pendência A.4 | P242 fecha A.4 — anotar promoção real |

**Sem `P242.div-N` formal antecipado** — ajustes triviais
paralelos aos quatro precedentes. Se audit revelar bloqueador
material (e.g. `FrameItem::Group` API exige mudança não-trivial),
criar `P242.div-1` formal e parar.

---

## §3 Decisões fixadas P242 — 9 decisões

### Decisão 1 — Tipo `Corners<T>` paralelo absoluto a `Sides<T>`

**Forma esperada** (`01_core/src/entities/corners.rs`):

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

**Ordem dos campos**: `top_left` → `top_right` → `bottom_right`
→ `bottom_left` (sentido horário começando top-left; paridade
vanilla `lab/typst-original/.../layout/corners.rs`).

L0 novo: `00_nucleo/prompts/entities/corners.md` (paridade
absoluta `sides.md` substituindo Left/Top/Right/Bottom por
TopLeft/TopRight/BottomRight/BottomLeft). **Sub-padrão #14
"Tipo entity em ficheiro próprio" N=5 → 6 cumulativo** (Sides
P156C → Parity P156E → Dir P156I → BibEntry P159A → CitationForm
P159C → **Corners P242**).

### Decisão 2 — `ShapeKind::RoundedRect { radii: Corners<Length> }`

```rust
pub enum ShapeKind {
    Rect,
    RoundedRect { radii: Corners<Length> },   // novo P242
    Ellipse,
    Line { dx: f64, dy: f64 },
}
```

**Degeneração**: quando todos os 4 radii são zero, layout +
exporter podem opcionalmente normalizar para `Rect` (decisão
local em audit C1 — preferível **não** normalizar; preservar
distinguibilidade estrutural).

L0 actualizado: `00_nucleo/prompts/entities/geometry.md` ganha
secção `RoundedRect`.

### Decisão 3 — Refino `Content::Block` + `Content::Boxed` simétrico

Fields novos em ambos os variants (paridade absoluta entre os
dois — coerência P156G/H consolidada):

```rust
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
    radius:    Corners<Length>,   // novo P242
    clip:      bool,              // novo P242
}

Boxed {
    body:     Box<Content>,
    width:    Option<Length>,
    height:   Option<Length>,
    inset:    Sides<Length>,
    baseline: Length,
    radius:   Corners<Length>,    // novo P242
    clip:     bool,               // novo P242
}
```

**Refactor preserva backwards compat**: defaults `Corners::default()`
(todos os cantos zero) + `clip: false` reproduzem comportamento
pré-P242 literal. Construtores Rust `Content::block(...)` /
`Content::boxed(...)` ganham 2 args extras (breaking change
interno cristalino-only; tests pré-existentes adaptados em
massa). **Alternativa considerada e rejeitada**: builders
fluentes `with_radius` / `with_clip` (consistência com
construtores actuais que tomam todos os fields posicionais é
preferível neste passo; pattern fluente não-utilizado em
variants Content).

### Decisão 4 — `radius:` named arg aceita Length uniforme OU dict por canto

**Opção α (recomendada)**: aceitar 2 formas:
- `Length` único → `Corners::uniform(v)` (paridade
  `inset` em P156G/H que aceitou Length uniforme).
- `Dict` com keys `top-left` / `top-right` / `bottom-right` /
  `bottom-left` / `top` (atalho para `top-left + top-right`) /
  `bottom` (idem) / `left` (idem) / `right` (idem) / `rest`
  (fallback).

Precedência em audit: **específico > eixo > rest** (paridade
P156C `extract_sides_lengths` precedence per ADR-0064 Caso C).

**Opção β**: aceitar apenas Length uniforme; dict por canto
diferido a P242+1 futuro.

**Decisão fixada P242**: **Opção α** — paridade vanilla mais
completa; ganho user-facing maior em ~30 LOC extra. Helper novo
`extract_corners_lengths` em `stdlib/layout.rs` paralelo
absoluto `extract_sides_lengths` (P156L). **Sub-padrão "reuso
template helpers extract_*" promovido formalmente** N=3 → 4
cumulativo (Sides extract N=1 P156L → reuso N=2 P156-? → 
**Corners extract P242**).

### Decisão 5 — `clip:` named arg aceita Bool simples

`clip: false` (default) preserva pré-P242. `clip: true` activa
clip-mask no PDF Group via `FrameItem::Group.clip_mask = Some(shape)`
onde `shape` é `Rect` (radii zero) ou `RoundedRect { radii }`
(qualquer canto não-zero).

**Validação `native_block` / `native_box`**: `clip` deve ser
Bool; outros tipos rejeitados.

### Decisão 6 — Layout vs Export — onde desenhar a rounded-rect

**Cristalino preserva separação L1 / L3** (ADR-0029). Layouter
(L1) emite `FrameItem::Group` com `clip_mask: Some(RoundedRect
{ radii })` quando `clip == true && radii != Corners::default()`.
Quando `clip == false`, fields `radius` armazenados mas **não
geram clip-mask** (radius isolado sem clip é cosmético adiado
per ADR-0054 graded — vanilla também trata radius+clip
acoplados em casos comuns; radius sem clip é refino futuro
NÃO reservado).

PDF exporter (L3) desenha o path Bezier 4 corners via
`build_rounded_rect_path` novo (helper local em `03_infra/src/export.rs`)
e emite operadores PDF `m`/`c`/`l`/`W n` no espaço local do
Group, após a matriz `cm` actual (paridade absoluta DEBT-30
implementação P79 mas para shape rounded em vez de rect/ellipse).

**Bezier kappa**: `0.5523` (constante magic — aproxima quarto
de círculo via 2 control points). Audit C1 deve confirmar este
valor — vanilla typst usa o mesmo (`lab/typst-original/.../export/pdf/...`).

### Decisão 7 — Opção γ L0 partial tocado (terceira excepção ADR-0080)

L0 a tocar (estimado 4-5 ficheiros):
- `entities/corners.md` (**ficheiro novo**).
- `entities/geometry.md` (+ secção RoundedRect).
- `entities/content.md` (+ campos radius/clip em Block + Boxed
  + actualizar listas scope-out P156G/H removendo `radius`+`clip`).
- `infra/export.md` (+ secção rounded-rect path gen).
- Possivelmente `entities/sides.md` (+ cross-reference "ver
  também Corners<T>").

**Terceira excepção justificada ADR-0080 EM VIGOR pós-P229** —
N=2 (P241) → 3 (P242) cumulativo. Padrão "L0 tocado para
features runtime novas + walk integration" foi sub-categoria
P240/P241 — P242 é **sub-categoria diferente**: "L0 tocado
para geometry/exporter infrastructure". Anotação ADR-0080
§"Excepções" diferencia as duas sub-categorias.

### Decisão 8 — Promoção real graded ADR-0054 para `radius`+`clip`

Os attrs `radius` e `clip` foram declarados scope-out em P156G
(Block) + P156H (Boxed) com rejeição hard em `native_block` /
`native_box`. P242 é a **primeira promoção real graded** de
scope-outs ADR-0054 declarados — distinta de:
- Refino qualitativo P156L (Pad `sides` Length → `Option<Length>`).
- Refactor cosmético P158C (Figure.kind `String → Option<String>`).
- Sub-categoria nova: **"promoção real de scope-out P156G/H
  → semantic concreta + render PDF real"**. Sub-padrão emergente
  P242 N=1 inaugurado.

Anotação ADR-0079 Categoria A.4 transita "graded P231" →
"materializado parcial P242" (radius+clip ✓; outset/fill/stroke
permanecem scope-out N=4 restantes).

### Decisão 9 — Sem fechamento Fase 5 graded

P242 completa M7+5 (3/5 M7+ sub-passos). Cobertura Layout per
metodologia transita 89% → **~91-92%** (refino qualitativo +
quantitativo — primeira variant qualitativo+quantitativo P242).
Fase 5 graded transita ~80-92% (12/13-15) → **~85-92%** (13/13-15).

**Sem promoção formal Fase 5 → IMPLEMENTADO** — depende de
decisão humana sobre M7+3 + M7+4 (restantes 2 sub-passos
pendentes) ou scope-out formal.

---

## §4 `Corners<T>` + `ShapeKind::RoundedRect` (C2+C3)

Detalhes em §3 Decisões 1 e 2 acima. Tests dedicados:

**Unit `entities/corners.rs`** (4 tests):
- `p242_corners_new_preserva_4_cantos`.
- `p242_corners_uniform_clona_valor`.
- `p242_corners_default_zero_em_todos_cantos`.
- `p242_corners_clone_eq_partial_eq_funcionam`.

**Unit `entities/geometry.rs`** (2 tests):
- `p242_shapekind_rounded_rect_radii_zero_eq_rect_distinguivel`
  (degeneração estrutural preserva PartialEq distinto — não
  normaliza).
- `p242_shapekind_rounded_rect_radii_uniforme_pt_5`.

---

## §5 Refino `Content::Block` + `Content::Boxed` (C4)

**Arms cascata afectados** (estimado 12-15 sítios por variant,
~24-30 total; paridade método P156L `Pad` refactor):
- `Content::PartialEq` arm Block + Boxed: comparar fields novos.
- `Content::plain_text` arm: proxy body inalterado.
- `Content::is_empty` arm: proxy body inalterado.
- `Content::map_content` / `map_text` arms: recurse body;
  preservar radius/clip via Copy primitivos.
- `Content::materialize_time` arms: recurse body.
- Walk arm em `rules/introspect.rs`: recurse body (radius/clip
  não passam por tags — são layout-time apenas).
- `layout/mod.rs::layout_content` arm Block: lógica nova ao
  fim do arm — se `clip == true && radius != Corners::default()`,
  emit `FrameItem::Group { clip_mask: Some(ShapeKind::RoundedRect
  { radii: radius }) }` envolvendo o output existente.
- `layout/mod.rs::layout_content` arm Boxed: idem (inline-aware).
- `measure_content_constrained` arms: dimensões inalteradas
  (radius/clip não afectam bounding box per ADR-0054 graded —
  vanilla também não considera radius/clip no measure;
  approximation aceite).
- `native_block` em `stdlib/layout.rs`: remover rejeição hard
  de `radius:` e `clip:`; adicionar processamento via novo
  `extract_corners_lengths` + Bool check.
- `native_box` idem.

**Construtores Rust afectados** (breaking change cristalino-only):
- `Content::block(body, width, height, inset, breakable)` →
  `Content::block(body, width, height, inset, breakable,
  radius, clip)`.
- `Content::boxed(body, width, height, inset, baseline)` →
  `Content::boxed(..., radius, clip)`.

**Tests pré-existentes adaptados** (estimado ~20-25 sítios,
paridade P156L que adaptou 7 tests). Adaptação trivial via
default `Corners::default()` + `false`.

**Counts pós-P242**:
- Content variants: 62 preservado (sem variant novo; apenas
  refino field-add em 2 existentes).
- ElementPayload variants: preservado.
- ShapeKind variants: 3 → **4** (+RoundedRect).
- Stdlib funcs: 64 preservado (refino existentes block/box;
  sem novas).
- Helper `extract_corners_lengths`: **+1** em
  `stdlib/layout.rs`.

---

## §6 PDF exporter rounded-rect (C5)

`03_infra/src/export.rs` ganha helper local:

```rust
/// Constrói path PDF para rounded-rect via Bezier 4 corners.
/// kappa = 0.5523 (constante magic — aproxima quarto de círculo).
fn build_rounded_rect_path(
    x: f64, y: f64, w: f64, h: f64,
    radii: &Corners<f64>,
) -> String {
    const K: f64 = 0.5523;
    // 8 segmentos: 4 cantos (Bezier) + 4 lados (Line).
    // Sequência horária começando top-left após inset do radius.
    // Output PDF operators: m + c×4 + l×4 + h.
    // [...detalhes em audit C1...]
}
```

Arm em geração de clip path:
```rust
match shape {
    ShapeKind::Rect          => { /* existente DEBT-30 */ }
    ShapeKind::RoundedRect { radii } => {
        let path = build_rounded_rect_path(0.0, 0.0, w, h, &radii.to_pt());
        out.push_str(&path);
        out.push_str("W n\n");
    }
    ShapeKind::Ellipse       => { /* existente */ }
    ShapeKind::Line { .. }   => { /* existente */ }
}
```

**Coordenadas**: inversão Y mantida (cristalino y=0 topo;
PDF y=0 baixo) — `y_pdf = page_height - y_cristalino` aplicada
externamente per convenção exporter (não dentro de
`build_rounded_rect_path`).

L0 actualizado: `infra/export.md` ganha secção "Rounded-rect
clip path" referenciando DEBT-30 fechado P79 + P242 extensão.

---

## §7 Critério aceitação P242 (C8+C9+C10)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | verde |
| `cargo test --workspace` | **~2191 verdes** (range 2187-2193; +12-18 vs 2175 baseline P241) |
| `crystalline-lint .` | 0 violations |
| `crystalline-lint --fix-hashes` | 4-5 L0 hashes + ~10-15 ficheiros L1 actualizados (estimado superior a P240/P241 por refactor field-add cross-variant) |
| Adaptações pre-existentes | **N=~20-25** (paridade P156L que adaptou 7 tests; P242 afecta ~20-25 sítios por Block+Boxed dual) |
| Content variants | 62 preservado (refino field-add) |
| ShapeKind variants | 3 → **4** (+RoundedRect) |
| Block fields | 5 → **7** (+radius +clip) |
| Boxed fields | 5 → **7** (+radius +clip) |
| Tipos entity novos | **Corners<T>** (paralelo Sides<T>) |
| Helpers stdlib novos | `extract_corners_lengths` (paralelo `extract_sides_lengths` P156L) |
| ADR-0081 status | IMPLEMENTADO parcial 2/5 → 3/5 |
| ADR-0079 Categoria A.4 | "graded P231" → "materializado parcial P242" anotada |
| ADR-0080 §"Excepção P242" | anotada N=3 cumulativo (sub-categoria geometry/exporter) |
| ADR-0061 §"Aplicações cumulativas" | actualizada com P242 (10ª aplicação consecutiva pós-P156L) |
| L0 partial tocado | 4-5 ficheiros |
| Regressões reais | **0** |

**Tests P242** (estimativa ~16-18 unit + cenários canónicos):

**Unit corners** (4 tests em `entities/corners.rs`) — ver §4.

**Unit geometry** (2 tests em `entities/geometry.rs`) — ver §4.

**Unit content** (3-4 tests em `entities/content.rs`):
- `p242_block_radius_clip_partial_eq_distingue_cantos`.
- `p242_boxed_radius_clip_partial_eq_distingue_cantos`.
- `p242_block_default_radius_zero_clip_false_preserva_pre_p242`.
- `p242_boxed_default_radius_zero_clip_false_preserva_pre_p242`.

**Unit stdlib** (4-5 tests em `stdlib/layout.rs`):
- `p242_native_block_aceita_radius_length_uniforme`.
- `p242_native_block_aceita_radius_dict_por_canto`.
- `p242_native_block_aceita_clip_bool`.
- `p242_native_block_clip_nao_bool_rejeita`.
- `p242_native_box_paridade_block_radius_clip`.

**Unit/E2E layout** (3-4 tests em `rules/layout/tests.rs`):
- `p242_block_clip_true_radius_zero_emit_group_rect_clip_mask`.
- `p242_block_clip_true_radius_non_zero_emit_group_rounded_rect_clip_mask`.
- `p242_boxed_clip_paridade_block`.
- `p242_radius_sem_clip_armazenado_mas_sem_clip_mask` (per
  Decisão 6 — radius sem clip não gera clip-mask).

**E2E exporter** (1-2 tests em `03_infra/src/export.rs::tests`):
- `p242_export_pdf_rounded_rect_path_contem_bezier_operadores`
  (verifica output PDF contém `m`/`c`/`l`/`W n` no espaço Group).

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2175 verdes pré-P242 →
   ~2191 verdes pós-P242 (+12-18; 0 regressões esperadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P242 NÃO toca trait Introspector nem methods — refino
   geometry isolada; invariants preservados literais.
3. **Backward compat**: tests pré-P242 que usam `Content::block(...)`
   / `Content::boxed(...)` adaptados via default
   `Corners::default()` + `false` (estado idêntico pré-P242).

**Promoções ADR esperadas**:
- ADR-0081 IMPLEMENTADO parcial **2/5 → 3/5** (M7+5 ✓; M7+3 +
  M7+4 pendentes). Distribuição preservada literal.
- ADR-0079 Categoria A.4 "graded P231" → "materializado parcial
  P242" anotada (radius+clip ✓; outset/fill/stroke restantes
  N=4 permanecem scope-out).
- ADR-0080 §"Excepções" entrada P242 anotada N=3 cumulativo
  sub-categoria "geometry/exporter infrastructure".
- ADR-0061 §"Aplicações cumulativas" actualizada — **10ª
  aplicação consecutiva** Layout pós-P156L (após pausa série
  Model P157-P159 + série M7+ P240-P241). Slope cumulativo:
  refino qualitativo+quantitativo (Block+Boxed 5 fields → 7
  cada; ShapeKind 3 variants → 4).
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁶¹** adicionada (~200 linhas
estimadas) documentando: M7+5 promoção real graded
materializado; lição N=5 cumulativo C1 audit validada;
sub-padrão "promoção real de scope-out ADR-0054" N=1
inaugurado; sub-padrão #14 "Tipo entity em ficheiro próprio"
N=5 → 6; terceira excepção justificada ADR-0080 sub-categoria
nova; 9 decisões fixadas.

---

## §8 Próximo sub-passo pós-P242

P242 completa M7+5 (M9d terceira sub-passo). Restantes 2
sub-passos M9d/M7+ pendentes (magnitude cumulativa restante
~13-20h).

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+3 multi-region completion cell-level** | `Regions { current, backlog, last }` completo; DEBT-56b candidato | **L+ (~8-12h)** | **alta** (desbloqueia C.2 + A.4 breakable per-cell; maior desbloqueio cumulativo restante) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1; magnitude isolada) |
| ADR meta admin XS | Promoção formal patterns N=2-3 acumulados pós-P242: "L0 tocado features runtime + walk" N=2; "refino aditivo paralelo callers fixpoint" N=2; "promoção real scope-out ADR-0054 graded" N=1 (não atinge limiar) | XS por pattern | baixa-média |
| Refino A.4 — `outset` + `fill` + `stroke` em Block+Boxed | Completar 3 dos 4 scope-outs P156G/H restantes pós-P242; magnitude S-M cada | S-M por attr | baixa-média (cumulativo eventual A.4 → "implementado completo"; sem urgência) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~85-92% (12 + M7+1 + M7+2 + M7+5 = 13/13-15) | XS | baixa |

**Recomendação subjectiva pós-P242**: **M7+3 multi-region
completion**. Maior desbloqueio cumulativo restante (C.2 +
A.4 breakable per-cell + DEBT-56b candidato fechar); magnitude
L+ (~8-12h) — sub-passo grande mas é o último grande caminho
M7+ pendente. Alternativa: M7+4 Place float (magnitude L
isolada; desbloqueia C.1; sem dependências M7+3).

**Decisão humana fica em aberto literal** pós-P242.

**Estado esperado pós-P242**:
- Tests workspace: 2175 → **~2191 verdes** (+16 P242 estimado).
- Content variants: 62 preservado.
- ShapeKind variants: 3 → **4**.
- Block fields: 5 → **7**.
- Boxed fields: 5 → **7**.
- Tipos entity novos: **+1 Corners<T>**.
- Stdlib funcs: 64 preservado.
- §A.5 distribuição: preservada (sem mudança categoria
  Layout/Model).
- Cobertura Layout per metodologia: 89% → **~91-92%** (refino
  qualitativo+quantitativo — promoção real graded primeiro
  exemplo).
- Cobertura user-facing total: ~71-72% → **~73-74%** (A.4
  radius+clip real + render PDF).
- **ADRs distribuição preservada**: PROPOSTO 12; EM VIGOR 29;
  IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  2/5 → **3/5** internamente. ADR-0079 Categoria A.4
  "materializado parcial P242" anotada. ADR-0080 §"Excepção
  P242" anotada N=3 sub-categoria geometry/exporter. ADR-0061
  §"Aplicações cumulativas" 10ª entrada Layout.
- **Saldo DEBTs: 11 preservado**.
- **34 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P242** (3 novos/consolidados):
  - "Promoção real scope-out ADR-0054 graded" N=1 inaugurado
    P242.
  - "Tipo entity em ficheiro próprio" (sub-padrão #14) N=5 →
    **6 cumulativo** (Sides → Parity → Dir → BibEntry →
    CitationForm → **Corners**).
  - "Reuso template helpers extract_*" N=3 → **4 cumulativo**
    (extract_sides_lengths P156L → reuso → **extract_corners_lengths
    P242**).
  - "Spec C1 audit obrigatório bloqueante" N=4 → **5 cumulativo**.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado (D.1+D.2+D.3 pós-P241; P242 é Categoria A).
- **Categoria A.4 Fase 5 Layout**: scope-out P231 →
  **materializado parcial P242** (radius+clip ✓; outset+fill+stroke
  N=3 restantes scope-out).
- **Fase 5 Layout candidata: 12/13-15 → 13/13-15 sub-passos
  materializados** (~85-92% cumulativo).
- **M9d / M7+ progresso**: **3/5 sub-passos materializados**
  (M7+1 ✓; M7+2 ✓; **M7+5 ✓**; M7+3 + M7+4 pendentes;
  cumulativa restante ~13-20h).

---

## §9 Notas operacionais para o executor

1. **Audit C1 PRIMEIRO** — não tocar código antes de validar
   empíricamente os 11 aspectos da tabela §2. Lição N=5
   cumulativo.

2. **Pattern paralelo Sides<T>** — ler `entities/sides.md` e
   `entities/sides.rs`; replicar literal substituindo
   `left/top/right/bottom` por `top_left/top_right/bottom_right/bottom_left`.
   `Corners<T>` é Sides<T> ortogonal cantos vs lados.

3. **Bezier kappa = 0.5523** — confirmar valor exacto em vanilla
   typst `lab/typst-original/.../export/pdf/...` durante audit
   C1. Se vanilla usa outro valor (e.g. 0.55228...), usar valor
   vanilla literal por paridade ADR-0033.

4. **Refactor field-add Block + Boxed em massa** — usar pattern
   P156L como referência (`Pad` refactor sides). Adaptação dos
   ~20-25 tests pré-existentes é mecânica: adicionar
   `Corners::default()` + `false` ao final dos args.

5. **L0 partial tocado é terceira excepção justificada**
   ADR-0080 — anotar como **sub-categoria nova**
   "geometry/exporter infrastructure" (distinta de P240/P241
   "features runtime walk integration"). Permite N=3 cumulativo
   sem inflar a sub-categoria walk.

6. **Cobertura quantitativa primeira vez em M7+** — P240+P241
   foram refino qualitativo (Introspection); P242 é refino
   qualitativo+quantitativo (Layout +2 pontos percentuais
   per metodologia). Documentar a distinção em §"Aplicações
   cumulativas" ADR-0061 entrada 10ª.

7. **Tests devem incluir test do output PDF real** — verificar
   bytes do PDF contêm operadores Bezier `m`/`c`/`l`/`W n`
   quando `clip=true && radius non-zero`. Distinção do test
   `Rect` clip (apenas `m`/`l`/`W n` sem `c`).
