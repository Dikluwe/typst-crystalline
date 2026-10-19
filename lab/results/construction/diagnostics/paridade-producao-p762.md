# Diagnóstico P762 — Modelo de avanço vertical alinhado com o vanilla

**Data da medição:** 2026-07-15T09:52:00Z  
**Commit base:** `617a9489e68aa08431dc3738bd9dc00567209df6`  
**Working tree:** modificado (ver lista de ficheiros em "Alterações").  
**Passo:** P762  
**Objectivo:** Confirmar a fórmula vanilla de avanço de linha (`top-edge + |bottom-edge| + leading`) e implementá-la no cristalino, actualizando o L0 e os testes.

---

## Documento de teste

`/tmp/p762-fixo.typ`:

```typst
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
#lorem(50)
```

Vanilla de referência: `lab/typst-original/target/release/typst` (Typst 0.15.0).  
Cristalino: `target/release/typst`.

---

## Medições

### Coordenadas Y das linhas (extraídas com `pdftotext -bbox`)

| Linha | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|-------|---------------|-------------------|---------|
| 1 | 40,000028 | 39,999578 | −0,000450 |
| 2 | 55,507478 | 55,507578 | +0,000100 |
| 3 | 71,014878 | 71,014578 | −0,000300 |
| 4 | 86,522278 | 86,521578 | −0,000700 |
| 5 | 102,029678 | 102,029578 | −0,000100 |
| 6 | 117,537138 | 117,537578 | +0,000440 |
| 7 | 133,044578 | 133,044578 | 0,000000 |
| 8 | 148,551978 | 148,551578 | −0,000400 |

Erro máximo observado: **< 0,001 pt** por linha.

### Diferença global de pixels (150 ppp, ImageMagick)

| Estado | AE | RMSE |
|--------|-----|------|
| P762 (`/tmp/p762-fixo.typ`) | **0** | **0 (0)** |

O output visual é idêntico ao vanilla neste caso de teste.

### Validação de regressão com cetz

`/tmp/p762-cetz.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Compilação cristalina: **OK** (`/tmp/p762-cetz-cristalino.pdf`, 2387 bytes).

---

## Modelo implementado

A fórmula do Typst vanilla para avanço de baseline entre linhas é:

```text
line_advance = top_edge + |bottom_edge| + leading
```

Onde:
- `top_edge` default = `"cap-height"`.
- `bottom_edge` default = `"baseline"`.
- `leading` default = `0.65em`.

O modelo antigo do cristalino usava:

```text
line_height = ascender + |descender| + lineGap
```

Para DejaVu Sans 11 pt, isto dava um avanço de **13,203 pt**, enquanto o vanilla avança **15,507 pt** (`cap-height + 0.65em`). A diferença acumulava ~2,304 pt por linha (ver diagnóstico P761).

### Alterações aplicadas

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/layout_types.rs` | Adicionados `top_edge` e `bottom_edge` a `TextStyle`. |
| `01_core/src/entities/style_chain.rs` | Propagação de `top_edge` / `bottom_edge` via `StyleDelta` e `StyleChain`. |
| `01_core/src/engine/eval/rules.rs` | Eval de `#set text(top-edge: ...)` / `#set text(bottom-edge: ...)`. |
| `01_core/src/engine/layout/metrics.rs` | Novo método `FontMetrics::text_edges(size, style) -> (Pt, Pt)`; implementação em `FixedMetrics`. |
| `03_infra/src/font_metrics.rs` | Implementações de `text_edges` em `FontBookMetrics` e `FallbackFontMetrics` com base nas métricas reais da face. |
| `01_core/src/engine/layout/text.rs` | Propagação dos campos `top_edge` / `bottom_edge` do estilo para o layout. |
| `01_core/src/engine/layout/cursor.rs` | `flush_line()` avança `top + |bottom| + leading`; `ensure_initial_baseline()` e transições de página/coluna usam `text_edges` em vez de `cap_height`. |
| `01_core/src/engine/layout/sub_frame.rs` | Avanço interno de sub-frame usa o mesmo modelo. |
| `01_core/src/engine/layout/set_page.rs` | Inicialização de página/coluna usa `text_edges`. |
| `01_core/src/engine/layout/grid.rs` | Compensação de baseline inicial usa `top-edge` em vez de `cap_height`. |
| `01_core/src/engine/layout/list_item.rs` | Espaçamento entre itens soltos (`tight: false`) usa `text_edges + leading`. |
| `01_core/src/engine/layout/enum_item.rs` | Idem para listas ordenadas. |
| `01_core/src/engine/layout/tests.rs` | Actualizados testes de `tight` e de `leading = 0pt` para o novo modelo. |
| `00_nucleo/prompts/engine/layout.md` | Secção de avanço vertical actualizada; `FontMetrics::text_edges` documentado. |
| `03_infra/fixtures/p307b/reference/*.pdf` | Snapshots P307b regenerados (`02-markup-heading`, `03-text-styling`, `07-multi-feature`). |

---

## Validação

- `cargo build --release` — OK.
- `cargo test --workspace` — OK: 4121 passed; 0 failed.
- `crystalline-lint .` — zero violations.
- Caso P762-fixo alinhado com o vanilla a < 0,001 pt; comparação visual AE=0 / RMSE=0.
- `cetz` 0.5.2 compila sem erro.

---

## Conclusão

- O modelo de avanço vertical do cristalino foi alterado de `line_height = ascender + descender + lineGap` para `top_edge + |bottom_edge| + leading`, alinhando-se com o Typst vanilla.
- A diferença acumulada de ~2,304 pt por linha observada em P761 foi eliminada no caso de teste P762-fixo.
- Foram introduzidos os campos `top-edge` / `bottom-edge` em `TextStyle`, com eval e suporte em `FontMetrics`.
- O L0 `00_nucleo/prompts/engine/layout.md` foi actualizado e os hashes ajustados.

---

## Alterações (ficheiros modificados no working tree)

```text
00_nucleo/prompts/engine/layout.md
01_core/src/entities/layout_types.rs
01_core/src/entities/style_chain.rs
01_core/src/engine/eval/rules.rs
01_core/src/engine/layout/cursor.rs
01_core/src/engine/layout/enum_item.rs
01_core/src/engine/layout/grid.rs
01_core/src/engine/layout/list_item.rs
01_core/src/engine/layout/metrics.rs
01_core/src/engine/layout/set_page.rs
01_core/src/engine/layout/sub_frame.rs
01_core/src/engine/layout/tests.rs
01_core/src/engine/layout/text.rs
03_infra/fixtures/p307b/reference/02-markup-heading.pdf
03_infra/fixtures/p307b/reference/03-text-styling.pdf
03_infra/fixtures/p307b/reference/07-multi-feature.pdf
03_infra/src/font_metrics.rs
```

(Os restantes ficheiros `01_core/src/engine/layout/*.rs` com mudança de hash apenas reflectem o `crystalline-lint --fix-hashes` e não alterações semânticas.)
