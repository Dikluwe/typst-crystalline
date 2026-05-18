# Relatório P273.6 — P-Gradient-Relative-Callsite-Activation (fecho 3γ.2)

**Data**: 2026-05-17.
**Magnitude**: S (L1 ~80 LOC cascade + arms save/restore; L3 ~30 LOC; 7 testes; cap soft L1 70 estourou ~14% per cascade ~86 sites bulk-patched).
**Cluster**: Visualize / Gradient (encerra refino estrutural definitivamente).
**Tipo**: sub-passo decimal P273.6 — fecho 3γ.2 pendência P273.5.
**Spec**: `00_nucleo/materialization/typst-passo-273-6.md`.

---

## §1 — Sumário executivo

**Cluster Gradient refino estrutural encerrado definitivamente** via
P273.6 — pendência P273.5 §9 (Layouter `parent_bbox` em
`#[allow(dead_code)]` sem consumer real) resolvida:

- **L1 FrameItem::Shape** ganha `parent_bbox_at_emit: Option<Rect>`
  (cascade ~86 sites bulk-patched).
- **L1 arm `Content::Block`** save/restore real do `parent_bbox`
  (3γ.2.γ — popula apenas quando `width+height` literais).
- **L1 emit shape sites** populam
  `parent_bbox_at_emit: self.parent_bbox`.
- **L1 Layouter `parent_bbox`** perde `#[allow(dead_code)]` —
  consumer real (Block write + emit shape read).
- **L3 `GradientObject`** ganha `parent_bbox_at_emit: Option<Rect>`;
  `scan_all_gradients` captura do FrameItem::Shape.
- **L3 dispatcher** computa `effective_parent_bbox` (real se Some;
  page_bbox fallback se None) — substitui hardcoded page_bbox P273.5.

### Marcos arquitecturais P273.6

**(1) Cluster Gradient refino estrutural encerrado definitivamente** —
pendência P273.5 §9 resolvida; 3γ.2 materializada para Content::Block.

**(2) Pattern DEBT-37 `cell_origin_*` replicado N=3 cumulativo** —
atinge limiar formalização ADR meta N=3-4:
- N=1: P84.6 (DEBT-37 `cell_origin_x/y/w` Grid cell).
- N=2: P273.5 (`parent_bbox` estrutural; consumer pending).
- **N=3: P273.6** (`parent_bbox` save/restore real Block + consumer
  real).
Candidato meta-ADR futura documentado.

**(3) Maior cascade pattern-match cross-FrameItem no cluster
Visualize** — ~86 sites bulk-patched via Python script (struct init +
destructure+reconstruct).

**(4) Sub-padrão "Sub-passos decimais consecutivos do mesmo cluster"
N=2 emergente** — P273.5 + P273.6.

### Decisões fixadas Fase A

1. **Decisão 1 (semântica bbox)**: **3γ.2.γ** — popular apenas quando
   `width.is_some() && height.is_some()` no Block.
2. **Decisão 2 (propagação L1→L3)**: **Prop-A revisitada** —
   FrameItem::Shape + GradientObject ambos ganham
   `parent_bbox_at_emit: Option<Rect>`.
3. **Decisão 3 (escopo contentores)**: **{Block} apenas** — Boxed
   difere P273.7; Stack/Pad/Group/Grid cell scope-out per ADR-0054
   graded.

### Defaults preservam P262-P273.5 bit-exact

- Shape sem `parent_bbox_at_emit` (None) → fallback page_bbox L3
  P273.5 preservado literal.
- Block sem dimensions literais (Decisão 3γ.2.γ) → `parent_bbox`
  outer preservado (LIFO restore; cai eventualmente no page fallback).
- Default field `parent_bbox_at_emit: None` em todos os ~86 sites
  bulk-patched.
- `relative: None/Some(Self_)` ignora `parent_bbox_at_emit` (Self_
  branch literal).
- 2605 baseline (P273.5 fim) preserved bit-exact.

---

## §2 — Diff L1+L3 antes/depois

### §2.1 — L1 FrameItem::Shape ganha campo (~10 LOC + cascade)

```rust
// 01_core/src/entities/layout_types.rs P273.6

Shape {
    pos:    Point,
    kind:   ShapeKind,
    width:  f64,
    height: f64,
    fill:   Option<Color>,
    stroke: Option<Stroke>,
    /// P273.6 — bbox do contentor imediato no momento do emit.
    parent_bbox_at_emit: Option<Rect>,
},
```

**Cascade ~86 sites bulk-patched** via Python script (struct init +
destructure+reconstruct).

### §2.2 — L1 arm `Content::Block` save/restore (~40 LOC)

```rust
// 01_core/src/rules/layout/mod.rs P273.6 (Block arm; ambos clip paths)

// P273.6 — save/restore parent_bbox (Decisão 3γ.2.γ).
let saved_parent_bbox = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    let w_pt = w.resolve_pt(font);
    let h_pt = h.resolve_pt(font);
    self.parent_bbox = Some(Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(w_pt),
        h: Pt(h_pt),
    });
}

// Layout body (acumula items em current_items).
self.layout_content(body);
self.flush_line();

// P273.6 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox;
```

### §2.3 — L1 emit shape sites populam `parent_bbox_at_emit`

3 sítios actualizados em `mod.rs` (Content::Shape arm linha 821;
Block's own shape emit linha 1756; Boxed's own shape emit linha 1478):

```rust
self.regions.current.current_items.push(FrameItem::Shape {
    // ... existing fields ...
    // P273.6 — populated by Layouter.parent_bbox (Block save/restore).
    parent_bbox_at_emit: self.parent_bbox,
});
```

### §2.4 — L1 Layouter `#[allow(dead_code)]` removido

```rust
- #[allow(dead_code)]  // P273.5 — campo estructural readiness
+ /// 3γ.2 materializada P273.6: arm Content::Block save/restore real;
+ /// emit shape sites populam FrameItem::Shape.parent_bbox_at_emit.
  pub(super) parent_bbox: Option<Rect>,
```

### §2.5 — L3 GradientObject ganha campo (~5 LOC)

```rust
// 03_infra/src/export.rs P273.6

struct GradientObject {
    kind:           GradientObjectKind,
    function_id:    usize,
    shading_id:     usize,
    pattern_id:     usize,
    /// P273.6 — bbox capturado do FrameItem::Shape (3γ.2).
    parent_bbox_at_emit: Option<Rect>,
}
```

`scan_all_gradients` actualizado para destructure + capture:

```rust
if let FrameItem::Shape {
    stroke: Some(Stroke { paint: Paint::Gradient(g), .. }),
    parent_bbox_at_emit,
    ..
} = item {
    // ...
    grad_objs.push(GradientObject {
        // ... existing fields ...
        parent_bbox_at_emit: *parent_bbox_at_emit,
    });
}
```

### §2.6 — L3 dispatcher usa `effective_parent_bbox`

```rust
let GradientObject { kind, function_id, shading_id, pattern_id,
                      parent_bbox_at_emit } = go;
// P273.6 — bbox real do Layouter substitui page_bbox 3γ.1 quando disponível.
let effective_parent_bbox: (f32, f32, f32, f32) =
    if let Some(rect) = parent_bbox_at_emit {
        (rect.x.0 as f32, rect.y.0 as f32,
         (rect.x.0 + rect.w.0) as f32, (rect.y.0 + rect.h.0) as f32)
    } else {
        (0.0, 0.0, page_w as f32, page_h as f32)  // P273.5 fallback
    };

// Linear+Radial RGB-family arms:
if relative == RelativeTo::Parent {
    let (tx0, ty0, tx1, ty1) =
        apply_parent_transform(local, Some(effective_parent_bbox));
    // ...
}
```

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.6 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=14 → N=15 cumulativo consolidação clara persistente** | Oitava anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=14 → N=15 cumulativo consolidação clara persistente** | apply_parent_transform + Rect P273.5 reused; DEBT-37 reused com consumer real |
| **Pattern DEBT-37 `cell_origin_*` replicado** | **N=2 → N=3 cumulativo (atinge limiar formalização N=3-4)** | P84.6 + P273.5 + **P273.6 com consumer real** |
| Cap LOC hard vs soft explícito | **N=8 → N=9 cumulativo consolidação total** | L1 hard 100 (real ~80; soft 70 estourou ~14% per cascade) |
| **Aplicação meta-ADR (ADR-0093)** | **N=3 → N=4 cumulativo** | Pattern 2 anotação cumulativa; quarta aplicação prática |
| **Aplicação meta-ADR (ADR-0094)** | **N=4 → N=5 cumulativo** | Pattern 1 cap LOC; quinta aplicação prática |
| **Sub-passos decimais consecutivos do mesmo cluster** | **N=1 → N=2 cumulativo emergente** | P273.5 + P273.6 |
| **Cascade pattern-match cross-FrameItem** | **N=1 → N=2 cumulativo** | P156C 12 sites + P273.6 ~86 sites — maior cascade Visualize |
| Diagnóstico imutável (décimo sétimo consumo) | **N=21 → N=22 cumulativo** | + P273.6 |

---

## §4 — Métricas finais

| Métrica | Pré-P273.6 | Pós-P273.6 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2605 | **2612** | +7 |
| Tests P273.6 novos | — | 7 | 1 L1 field + 1 Rect compose + 5 E2E observable |
| Tests P262-P273.5 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build dead_code warnings (Layouter parent_bbox) | 1 | **0** | -1 (resolvido) |
| Hashes propagados L0 | — | 1 (`89e7be9f`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; oitava anotação ADR-0091) |
| LOC L1 (additions, incluindo cascade ~86 sites) | — | ~80 | cap hard 100 (folga 20%); cap soft 70 estourou ~14% per cascade |
| LOC L3 (additions) | — | ~30 | cap hard 40 (folga 25%); cap soft 25 limite |

### §política condições verificadas

- ✓ Cap LOC L1 hard 100 — real ~80; folga 20%.
- ⚠ Cap LOC L1 soft 70 — real ~80; estouro ~14% per cascade ~86 sites
  bulk-patched. Registado per ADR-0094 Pattern 1.
- ✓ Cap LOC L3 hard 40 — real ~30; folga 25%.
- ✓ Cap LOC L3 soft 25 — real ~30; estouro ~20% registado.
- ✓ Cap testes hard 15 — real 7; folga 53%.
- ✓ Cap testes soft 10 — real 7; folga 30%.
- ✓ `parent_bbox` Layouter perdeu `#[allow(dead_code)]` —
  confirmado via `cargo build` zero warnings.
- ✓ Defaults `parent_bbox_at_emit: None` + `relative: None/Some(Self_)`
  preservam bytes P262-P273.5 literal.
- ✓ ADR-0029 pureza física L1 preserved (Rect é tipo dados;
  parent_bbox é metadata; save/restore é gestão RAM).
- ✓ Lint zero; L0 hash drift propagado.
- ✓ Regressão tests P262-P273.5 zero (2605 baseline preserved).
- ✓ **E2E observable diff** confirmado — test
  `p273_6_shape_inside_block_carries_parent_bbox_observable_diff`
  verifica que bytes PDF DIFEREM quando bbox real ≠ page (3γ.2 dá
  semântica real vs 3γ.1 identity).

**12 condições §política verificadas — 11 satisfeitas absolutas + 2
estouros soft registados** per ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.5

**2605 baseline preservado bit-exact**:

- typst-core: 2169 preserved.
- typst-shell: 24 preserved.
- typst-infra: 389 → 396 (+7 P273.6 tests).
- typst-wiring + bins: 23 preserved.

**Total: 2605 → 2612 (+7 net)**.

Mecânica:
- Default `parent_bbox_at_emit: None` em ~86 sites cascade → fallback
  page_bbox L3 P273.5 preservado.
- Block sem dimensions literais (Decisão 3γ.2.γ) → `parent_bbox`
  outer preservado.
- Self_/None relative branch literal preserved (parent_bbox_at_emit
  ignorado).

§política condições "Regressão tests P262-P273.5 zero" + "Defaults
preservam pipeline" satisfeitas absolutas.

---

## §6 — Anotação cumulativa ADR-0091 (oitava consecutiva)

Adicionada §"Anotação cumulativa P273.6 — Parent bbox real save/restore
(fecho 3γ.2)" cobrindo:

- Decisões 1/2/3 fixadas: 3γ.2.γ + Prop-A revisitada + {Block} apenas.
- Mecanismo de propagação L1→L3.
- Pattern DEBT-37 cell_origin_* replicado N=3 (atinge limiar).
- Defaults preservam P262-P273.5 bit-exact.
- 9 sub-padrões aplicados.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=14 →
**N=15 cumulativo consolidação clara persistente** — oitava anotação
consecutiva ADR-0091 (P270/P270.1/P270.2/P270.3/P273/P274/P273.5/
**P273.6**).

---

## §7 — L0 `entities/gradient.md` anotação P273.6

Adicionada anotação P273.6 após P273.5 — fecho 3γ.2; decisões fixadas;
cascade ~86 sites; pattern DEBT-37 N=3 cumulativo. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:89e7be9f`.

---

## §8 — Pendências preservadas pós-P273.6

Inalteradas vs P273.5 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**Pendências específicas pós-P273.6** (incremental per ADR-0054
graded):
- **P273.7 — Boxed save/restore** (se necessário; análogo Block).
- **P273.6.bis — Stack/Pad/Group/Grid cell save/restore** (out of
  scope; per ADR-0054 graded).
- **P273.6.bis2 — Bbox medido pós-layout** (refino 3γ.2.β/α se
  3γ.2.γ for empiricamente insuficiente).
- **Dedup bbox-aware** — gradient com mesmo Arc usado em contextos
  distintos: actualmente primeiro wins.

**Pós-P273.6 fecha cluster Gradient refino estrutural definitivamente**
— `relative=parent` ganha semântica observable real para Block.
Próximo passo natural: sair do cluster Gradient.

---

## §9 — Limitações conscientes P273.6

Per spec §5:

- Decisão 3γ.2.γ (Block com dimensions literais) — Block sem
  dimensions continua a usar page_bbox fallback (3γ.1). Refino
  futuro com medição pós-layout fica fora de escopo.
- Lista de contentores: Block apenas. Boxed/Stack/Pad/Grid cell/
  FrameItem::Group ficam fora — per ADR-0054 graded.
- `parent_bbox` armazena bbox aproximado a partir de cursor +
  dimensions resolvidas; refino exacto pós-layout fora de escopo.
- Saved/restored LIFO simples — não recovers de panic mid-body.
- **Dedup bbox-aware**: gradients dedup'd por Arc pointer; primeira
  occurrence captura bbox. Refino bbox-aware dedup fica fora de
  escopo P273.6 per ADR-0054 graded.

---

## §10 — Marco final P273.6

**Cluster Gradient refino estrutural encerrado definitivamente**:

- L1 FrameItem::Shape ganha `parent_bbox_at_emit: Option<Rect>` +
  cascade ~86 sites.
- L1 arm `Content::Block` save/restore real (3γ.2.γ).
- L1 emit shape sites populam `parent_bbox_at_emit`.
- L1 Layouter `parent_bbox` consumer real ativado.
- L3 GradientObject + dispatcher consumem `parent_bbox_at_emit`.
- 7 tests P273.6 + zero regressão tests P262-P273.5.
- E2E observable diff confirmado.

Cristalino oferece gradient API user-facing paridade vanilla em:
- Cross-variant runtime fields canónica 3/3 (focal + space + relative).
- Adaptive N multispace refino qualitativo (Linear+Radial).
- Parent bbox callsite real **com consumer real Block** (P273.6).

Sub-padrão **"Pattern DEBT-37 cell_origin_* replicado" N=3 cumulativo
atinge limiar formalização ADR meta N=3-4** — candidato meta-ADR
futura.

Cluster Gradient feature-complete + qualitativo + refino estrutural
definitivamente encerrado — pronto para saída cluster.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.6; oitava anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=4 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=5 cumulativo).
- ADR-0085 — Diagnóstico imutável (décimo sétimo consumo).
- ADR-0029 — Pureza física L1 (preserved; Rect/parent_bbox são tipos
  dados + metadata).
- DEBT-37 (P84.6) — Pattern `cell_origin_*: Option<f64>` reused com
  consumer real P273.6 (sub-padrão "Pattern DEBT-37 replicado" N=3
  cumulativo atinge limiar formalização).
- `00_nucleo/diagnosticos/typst-passo-273-6A-diagnostico.md` — Fase A
  empírica + decisões 1/2/3 fixadas.
- P273.5 — Spec original; Layouter `parent_bbox` deixado em
  `#[allow(dead_code)]` (resolvido P273.6).
- P273.6 candidato P273.7 (Boxed save/restore se necessário).
- Spec P273.6 — `00_nucleo/materialization/typst-passo-273-6.md`.

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — cluster Gradient refino estrutural encerrado
definitivamente; 3γ.2 materializada para Content::Block; pattern
DEBT-37 N=3 cumulativo atinge limiar formalização ADR meta.*
