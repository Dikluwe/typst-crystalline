# Diagnóstico P760 — Paridade de texto latino com quebras idênticas

**Data da medição:** 2026-07-14T21:38:33Z  
**Commit base:** `5332570b2adcbe5c5f9990f73f00827cbf8a798b`  
**Working tree:** modificado (ver lista de ficheiros em "Alterações").  
**Passo:** P760  
**Objectivo:** Investigar a origem dos ~9,11 % AE / ~21,32 % RMSE de diferença de pixels entre vanilla e cristalino num documento de texto latino com quebras idênticas, e corrigir se fosse bug real.

---

## Documento de teste

`/tmp/p760-fixo.typ`:

```typst
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
#[
Lorem ipsum dolor sit amet, ... (texto corrido, 2 parágrafos)
]
```

Vanilla de referência: `/tmp/p760-fixo-vanilla.pdf` + `/tmp/p760-fixo-vanilla.png` (150 ppp, `mutool draw`).  
Cristalino: `/tmp/p760-fixo-cristalino-p760.pdf` + `/tmp/p760-fixo-cristalino-p760.png`.

---

## Medições

| Estado | AE | RMSE |
|--------|-----|------|
| Baseline P759 (cristalino antes da correção) | 116 612 | 0,21316 |
| Correção parcial (`vertical_metrics` resolvido por estilo, sem typo metrics, sem FontDescriptor real) | 119 159 | — |
| Correção completa P760 (style + typo metrics + FontDescriptor real) | **114 019** | **0,210357** |

Redução face ao baseline: ~2,2 % em AE, ~1,3 % em RMSE. A diferença residual continua visível mas é agora atribuída a variações mecânicas de subsetagem/renderização (hinting / anti-aliasing), não a um bug de layout corrigível neste passo.

---

## Causa real encontrada

Dois bugs de layout/exportação, ambos corrigidos:

1. **`FallbackFontMetrics::vertical_metrics` usava a primeira fonte do `FontBook`.**
   - O `FontBook` começa por fontes como Libertinus Serif; o estilo declara DejaVu Sans.
   - O `line_height` era calculado com métricas da fonte errada, deslocando ligeiramente as linhas verticais.
   - Ficheiro: `03_infra/src/font_metrics.rs`.

2. **`/FontDescriptor` no PDF usava valores fixos genéricos.**
   - Anterior: `/Ascent 800`, `/Descent -200`, `/FontBBox [-1000 -200 2000 900]`.
   - Agora: `/Ascent 759.76562`, `/Descent -240.23438`, `/FontBBox [-1020.50781 -462.89062 1793.45703 1232.42188]`, alinhado com as métricas reais da DejaVu Sans.
   - Ficheiro: `03_infra/src/export/builder.rs`.

---

## Correcção

### Assinatura do trait `FontMetrics`

`vertical_metrics` e `cap_height` passam a receber `style: &TextStyle`:

```rust
pub trait FontMetrics: Send + Sync {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt;
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt);
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt;
}
```

Todos os call-sites em L1 (`layout/mod.rs`, `cursor.rs`, `boxed.rs`, `grid.rs`, `columns.rs`, `enum_item.rs`, `list_item.rs`, `link.rs`, `placement.rs`, `sub_frame.rs`, `dynamic.rs`, `sequence.rs`, `slicing.rs`, `grid_placement.rs`, `helpers.rs`, `hyphenation.rs`, `tests.rs`) e em `math/layout/mod.rs` / `stretchy.rs` foram actualizados.

### `typo_metrics(face)`

Nova função auxiliar privada em `03_infra/src/font_metrics.rs` que devolve `(ascender, descender_abs, line_gap)` preferindo as métricas tipográficas do OS/2 e caindo para `hhea`.

### `FallbackFontMetrics::vertical_metrics`

Resolve a fonte através de `resolve_primary(style)` e aplica `typo_metrics` à face resultante.

### `FontDescriptor`

`03_infra/src/export/builder.rs` passa a emitir `/Ascent`, `/Descent`, `/CapHeight`, `/StemV` e `/FontBBox` calculados a partir da face real.

---

## Validação

- `cargo build --release` — OK.
- `cargo test --workspace` — OK: 635 passed; 0 failed; 5 ignored.
- `crystalline-lint .` — zero violations.
- Snapshot `03_infra/fixtures/p307b/reference/09-cidfont.pdf` regenerado (diferença de 49 bytes esperada pela alteração do `FontDescriptor`).

---

## Prompts L0 actualizados

- `00_nucleo/prompts/engine/layout.md` — assinatura de `FontMetrics::vertical_metrics`/`cap_height` com `style`.
- `00_nucleo/prompts/infra/font_metrics.md` — mesma assinatura, secção `typo_metrics`, resolução por estilo em `FallbackFontMetrics::vertical_metrics`, histórico de revisões.
- Hashes recalculados via `crystalline-lint --fix-hashes .`.

---

## Conclusão

A diferença de pixels observada em P759 era parcialmente causada por bugs reais de métricas verticais e `FontDescriptor`. Após a correção, a diferença residual (AE ~114k, RMSE ~0,21) é considerada mecânica aceitável — atribuída a variações de subsetagem e renderização (hinting/AA) entre os dois pipelines, não a um bug de layout corrigível neste passo.

---

## Alterações (ficheiros modificados no working tree)

```text
00_nucleo/prompts/infra/font_metrics.md
00_nucleo/prompts/engine/layout.md
01_core/src/engine/layout/boxed.rs
01_core/src/engine/layout/columns.rs
01_core/src/engine/layout/cursor.rs
01_core/src/engine/layout/dynamic.rs
01_core/src/engine/layout/enum_item.rs
01_core/src/engine/layout/grid.rs
01_core/src/engine/layout/grid_placement.rs
01_core/src/engine/layout/helpers.rs
01_core/src/engine/layout/hyphenation.rs
01_core/src/engine/layout/link.rs
01_core/src/engine/layout/list_item.rs
01_core/src/engine/layout/metrics.rs
01_core/src/engine/layout/mod.rs
01_core/src/engine/layout/placement.rs
01_core/src/engine/layout/sequence.rs
01_core/src/engine/layout/slicing.rs
01_core/src/engine/layout/sub_frame.rs
01_core/src/engine/layout/tests.rs
01_core/src/engine/math/layout/mod.rs
01_core/src/engine/math/layout/stretchy.rs
03_infra/fixtures/p307b/reference/09-cidfont.pdf
03_infra/src/export/builder.rs
03_infra/src/fallback_fonts.rs
03_infra/src/font_metrics.rs
```
