# Relatório de Paridade — P751

**Passo:** P751  
**Data:** 2026-07-14  
**Hash do commit com a correção:** `f6200771e`  

## Resumo

Corrigido o bug em que `Layouter::new` fixava a baseline inicial (`cursor_y`) usando o estilo por defeito (11 pt), antes de processar qualquer `#set text(size: ...)` no início do documento. A primeira linha de texto passa a ser posicionada com o `cap_height` do estilo realmente activo no momento em que o primeiro conteúdo real é emitido.

## Causa exacta

`01_core/src/engine/layout/mod.rs:519` (pré-P751):

```rust
let initial_style = TextStyle::from(&StyleChain::default_chain());
let initial_cap_height = metrics.cap_height(initial_style.size);
rs.current.cursor_y = Pt(cfg.margin) + initial_cap_height;
```

O `cursor_y` era calculado uma única vez na construção do `Layouter`, com o tamanho por defeito. Um `#set text(size: 8pt)` no início do documento actualizava a `StyleChain`, mas o offset de baseline já estava fixo.

## Correção

- `Layouter::new` inicializa `cursor_y = margin` e activa uma flag `initial_baseline_pending`.
- `ensure_initial_baseline()` adiciona `cap_height(style.size)` na primeira emissão de conteúdo real (texto, equação, forma, imagem, marcadores de lista/enumeração).
- Sub-layouts isolados (`layout_sub_frame`) salvam/restauram a flag, para não alterar o estado do layout pai.

Ficheiros alterados:

- `01_core/src/engine/layout/mod.rs`
- `01_core/src/engine/layout/cursor.rs`
- `01_core/src/engine/layout/list_item.rs`
- `01_core/src/engine/layout/enum_item.rs`
- `01_core/src/engine/layout/equation.rs`
- `01_core/src/engine/layout/shape.rs`
- `01_core/src/engine/layout/image.rs`
- `01_core/src/engine/layout/sub_frame.rs`

## Medição — caso de reprodução

Ficheiro `/tmp/p751-tamanho-inicial.typ`:

```typst
#set text(size: 8pt)
X
```

Baseline do "X" (coordenada PDF `y`, espaço da página com Y para cima):

| Versão | Baseline `y` | Nota |
|--------|--------------|------|
| Vanilla Typst 0.15.0 | 765.75964 | referência |
| Cristalino antes de P751 | 763.510 | ~2.25 pt abaixo (usava 11 pt) |
| Cristalino após P751 | 765.559 | ~0.20 pt de diferença do vanilla |

A diferença residual (~0.20 pt) está dentro da margem já observada em P750 para métricas de fonte reais vs. estimadas.

## Não-regressão

### Caso sem `#set` inicial

```typst
X
```

- Vanilla: 763.78564
- Cristalino após P751: 763.510
- Diferença: ~0.275 pt (dentro da margem residual).

### `#set page(...)` a meio do documento

Documento com texto antes e depois de `#set page(width: 400pt, height: 300pt)`:

| Página | Vanilla | Cristalino após P751 |
|--------|---------|----------------------|
| 1 | 763.78564 | 763.510 |
| 2 | 257.04773 | 256.773 |

Diferenças ~0.27 pt em ambas as páginas.

### Grids e formas

Fixtures `p307b` 04-shapes e documentos de teste com grids compilam sem crash e sem diferenças qualitativas. Snapshots binários `02-markup-heading` e `07-multi-feature` foram regenerados porque a baseline inicial de headings passa a reflectir o tamanho real do heading (comportamento correcto, alinhado com o vanilla).

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
- Comparação de imagem (`mutool draw -r 150` + ImageMagick `compare -metric AE`):
  - Antes de P751: ~3298 pixels diferentes do vanilla.
  - Após P751: ~3312 pixels diferentes do vanilla.
- Estado inalterado: a diferença visual é a mesma que já existia antes de P751.

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

## Conclusão

- Sonda completa; causa localizada em `Layouter::new`.
- Correcção implementada e validada.
- Casos de não-regressão testados.
- `cargo test --workspace` limpo; `crystalline-lint .` limpo.
- `cetz` sem regressão adicional.
- Snapshots `02-markup-heading` e `07-multi-feature` actualizados intencionalmente.
