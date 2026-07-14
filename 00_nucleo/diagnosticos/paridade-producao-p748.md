# Registo — P748: correção da margem por defeito para formas

**Data:** 2026-07-14
**Commit de base:** `27bb2f3172b6deae711a702440113e2349add19e`
**Commit da correção:** `72ad29a0a`
**Passo:** P748

## O que aconteceu

O P748 reportou que a margem por defeito do cristalino parecia ser ~81,9 pt em vez dos ~70,87 pt esperados para A4 (fórmula `min(w,h) * 2.5/21`). A sonda mostrou que a fórmula em `01_core/src/entities/layout_types.rs:463` está correcta; o problema era o posicionamento vertical de formas (`rect`, `circle`, `line`, etc.) no fluxo principal.

## Causa

No layout principal, `Layouter` inicializa `cursor_y = margin + ascender` (`01_core/src/rules/layout/mod.rs:512`). Esse cursor representa a **baseline** da linha de texto. Quando `shape.rs` emitia uma forma, colocava o topo da forma em `cursor_y`, o que deslocava as formas para baixo pelo valor do ascender (~8,8 pt). O resultado observável era uma margem "aparente" de ~81,9 pt para formas.

## Solução

`01_core/src/rules/layout/shape.rs` passa a subtrair o ascender ao posicionar o topo da forma no fluxo principal. Para evitar duplicar esse ajuste em sub-layouts (células de grid, `place`, medições), onde `sub_frame.rs` já inicializa o cursor a `ascender` e o `grid.rs` depois translada os items por `-ascender`, foi introduzida uma flag `is_sub_frame` no `Layouter`:

- `01_core/src/rules/layout/mod.rs` — novo campo `is_sub_frame: bool`.
- `01_core/src/rules/layout/sub_frame.rs` — salva/restaura e activa a flag durante `layout_sub_frame`.
- `01_core/src/rules/layout/shape.rs` — subtrai ascender só quando `!layouter.is_sub_frame`.

## Medições

### Documento `line` + `circle`

Ficheiro: `/tmp/p748/line_circle.typ`

```typst
#line(dx: 100pt)
#circle(radius: 30pt, fill: rgb(255, 200, 0))
```

Extraído com `mutool draw -F trace`:

| Elemento | Cristalino Y (topo da página) | Vanilla Y (topo da página) |
|----------|-------------------------------|----------------------------|
| Linha    | 70.867 pt                     | 70.866 pt                  |
| Círculo  | 70.867 pt (topo)              | 84.066 pt (centro)         |

A linha passou a coincidir com a margem vanilla. O círculo no cristalino alinha o topo com a margem; no vanilla o círculo desce o equivalente ao ascender da linha — diferença de morfologia já conhecida, fora do scope deste passo.

### Documento apenas com texto "X"

Ficheiro: `/tmp/p748/text_x.typ`

| Elemento | Cristalino baseline Y | Vanilla baseline Y |
|----------|----------------------|---------------------|
| X        | 81.9 pt                | 78.10 pt            |

A diferença de ~3,8 pt mantém-se e é atribuída a métricas de fonte diferentes (CrystallineFont vs LibertinusSerif); o texto não foi afectado pela alteração.

## Validação

- `cargo test --workspace` — 631 passed; 0 failed; 5 ignored (2026-07-14, working tree).
- `crystalline-lint .` — `✓ No violations found`.
- Snapshots p307b afectados (04-shapes, 05-gradient-linear, 06-gradient-conic, 07-multi-feature) foram regenerados com `UPDATE_P307B_SNAPSHOTS=1`; o diff reflecte apenas o deslocamento vertical esperado das formas.

## Decisão

A margem por defeito para formas no fluxo principal está corrigida. O ajuste é condicional para não quebrar sub-layouts. Os snapshots p307b foram actualizados como consequência legítima da mudança de posicionamento.
