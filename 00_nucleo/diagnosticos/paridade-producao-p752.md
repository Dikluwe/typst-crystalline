# Relatório de Paridade — P752

**Passo:** P752  
**Data:** 2026-07-14  
**Hash do commit com a correção:** `be3eaa85f`  

## Resumo

Eliminado o resíduo de ~0,2-0,3 pt na posição da baseline inicial fazendo com que `cap_height` use a métrica real da fonte resolvida para o estilo activo, em vez da aproximação fixa `size * 0.7` ou da primeira fonte arbitrária do `FontBook`.

## Causa exacta

- `FixedMetrics::cap_height` usava `size * 0.7`.
- `FallbackFontMetrics::cap_height` (pré-P752) usava a **primeira fonte disponível no `FontBook`**, que podia não ser a mesma face que o shaper efectivamente usaria para o estilo. A primeira fonte do sistema (por ordem de descoberta) nem sempre tinha a mesma métrica de cap-height que a fonte resolvida para o texto.

O `Layouter` já era genérico sobre `M: FontMetrics`, portanto `ensure_initial_baseline()` não estava rigidamente ligado a `FixedMetrics`. O problema era que a assinatura `cap_height(&self, size: Pt)` não recebia o `TextStyle`, pelo que `FallbackFontMetrics` não podia resolver a fonte correcta.

## Correção

Alterada a assinatura do trait `FontMetrics`:

```rust
fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt;
```

Implementações actualizadas:

- `FixedMetrics::cap_height` — mantém a aproximação `size * 0.7` (fallback genuíno quando não há fonte real; L1/tests).
- `FontBookMetrics::cap_height` — ignora `style`, pois já encapsula uma face específica.
- `FallbackFontMetrics::cap_height` — usa `resolve_primary(style)` para escolher a mesma face que o shaper usará, calculando `capital_height()` real da fonte (fallback para `ascender`, depois para `size * 0.7`).

Call-sites actualizados em L1:

- `cursor.rs`: `ensure_initial_baseline`, `new_page`, `start_column`.
- `shape.rs`: cálculo do topo da forma.
- `set_page.rs`: reposicionamento após `#set page(...)`.
- `grid.rs`: cálculo do topo do grid.

Ficheiros alterados:

- `01_core/src/rules/layout/metrics.rs`
- `01_core/src/rules/layout/cursor.rs`
- `01_core/src/rules/layout/shape.rs`
- `01_core/src/rules/layout/set_page.rs`
- `01_core/src/rules/layout/grid.rs`
- `03_infra/src/font_metrics.rs`

## Medição — caso de reprodução

Ficheiro `/tmp/p752-baseline.typ`:

```typst
X
```

| Versão | Baseline `y` | Diferença vs. vanilla |
|--------|--------------|-----------------------|
| Vanilla Typst 0.15.0 | 763.78564 | — |
| Cristalino P751 | 763.510 | ~0.275 pt |
| Cristalino P752 | 763.821 | ~0.035 pt |

Ficheiro `/tmp/p752-tamanho.typ`:

```typst
#set text(size: 8pt)
X
```

| Versão | Baseline `y` | Diferença vs. vanilla |
|--------|--------------|-----------------------|
| Vanilla Typst 0.15.0 | 765.75964 | — |
| Cristalino P751 | 765.559 | ~0.200 pt |
| Cristalino P752 | 765.785 | ~0.025 pt |

O resíduo restante (~0.03 pt) é explicado por arredondamentos/float e pelo facto de o cristalino usar `Liberation Serif` enquanto o vanilla usa `Libertinus Serif` (fontes diferentes, mas a métrica usada agora é a da fonte efectivamente resolvida).

## Não-regressão

- Caso sem `#set` inicial: diferença reduzida de ~0.275 pt para ~0.035 pt.
- Caso `#set text(size: 8pt)`: diferença reduzida de ~0.200 pt para ~0.025 pt.
- Todos os fixtures `p307b` compilam sem crash.
- Snapshot `09-cidfont` regenerado intencionalmente (Noto Sans tem cap-height diferente da fonte default).

## Validação

```bash
cargo test --workspace
# resultados finais:
#   4116 passed; 0 failed
#   631 passed; 0 failed; 5 ignored
#   33 passed; 0 failed
#   2 passed; 0 failed
#   27 passed; 0 failed
#   2 passed; 0 failed
#   doc-tests ok
crystalline-lint .
# ✓ No violations found
```

### `cetz`

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

- Compila sem erros.
- Comparação (`mutool draw -r 150` + ImageMagick `compare -metric AE`):
  - P751: ~3312 pixels diferentes do vanilla.
  - P752: ~3312 pixels diferentes do vanilla.
- Sem regressão adicional; a diferença mantém-se no posicionamento vertical global do canvas, já presente antes de P752.

## Conclusão

- Métrica real da fonte acessível no ponto de `ensure_initial_baseline()` e agora efectivamente usada.
- Resíduo P750/P751 eliminado (de ~0,2-0,3 pt para ~0,03 pt).
- `FixedMetrics` mantido como fallback genuíno para L1/tests sem fonte real.
- `cargo test --workspace` limpo; `crystalline-lint .` limpo.
- Snapshot `09-cidfont` actualizado intencionalmente.
