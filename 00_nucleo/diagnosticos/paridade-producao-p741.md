# P741 — `polygon` com vértices `Ratio` (`50%`): sonda + scope-out reforçado

**Data:** 2026-07-13 (23:00 -03:00)
**Commit base:** `2df216742` ("P740: preenche hash do commit no relatório")
**Commit da implementação:** `17c82592831a8fe43675bb4d99b13aea50e079e2`
**Estado no momento das medições E2E:** working tree não commitado;
`git diff HEAD --stat`:

```
 00_nucleo/prompts/rules/stdlib/shapes.md | 20 +++++++++++++++++++-
 01_core/src/rules/stdlib/mod.rs          | 25 +++++++++++++++++++++++++
 01_core/src/rules/stdlib/shapes.rs       | 17 ++++++++++++++++-
 3 files changed, 60 insertions(+), 2 deletions(-)
```

Vanilla de referência: `lab/typst-original/target/release/typst`.
Render: `pdftoppm -png -r 150`; comparação: `python3 /tmp/pngdiff.py`.

---

## Sonda — dimensão de referência (confirmada com medição exacta)

`/tmp/p741-ratio.typ`:

```typst
#polygon((50%, 0pt), (0pt, 40pt), (25pt, 0pt))
#box(width: 200pt, height: 100pt, polygon((50%, 0pt), (0pt, 40pt), (25pt, 0pt)))
```

Render vanilla a 150 dpi (crop inspeccionado): o triângulo **dentro** do
box é muito menor que o de fora — `(50%, 0pt)` → ~100pt dentro do box
(50% de 200pt), contra ~50% da região de texto fora dele. **O ratio
resolve contra o contentor, não contra o próprio bounding box.**

Confirmação exacta para o eixo y (medição byte-a-byte):

```typst
#box(width: 200pt, height: 100pt, polygon((0pt, 50%), (0pt, 0pt), (50pt, 0pt)))
```

vs o mesmo com `(0pt, 50pt)`: **diff 0.0000%** (idêntico, 1045 px
não-brancos nos dois). O y relativo resolve contra a **altura** do
contentor.

## Sonda — mecanismo de resolução no cristalino

- **x**: `resolve_pt(rel, available_w)` existe (`layout/shape.rs`) e o
  `Boxed` clampa `regions.current.width` durante o body (P243) —
  parcialmente reaproveitável.
- **y**: **não existe referência de altura** — o `Boxed` faz layout do
  body via `layout_sub_frame_inline` com `unconstrained_height: true`
  (`layout/boxed.rs`); no ponto de emit do shape, a altura do box não
  está disponível.
- `ShapeKind::Path` carrega `Point` absoluto (`Pt`), construído em
  tempo de eval por `native_polygon`/`native_curve`.

## Custo medido da paridade completa

1. Novo tipo de ponto relativo em `geometry.rs` (`PathItem` carrega
   `Point` absoluto) + resolução em `layout/shape.rs` + alteração dos
   **3 braços** de `ShapeKind::Path` no exporter
   (`03_infra/src/export/stream.rs:492,587,756`) + os dois construtores
   (`polygon`, `curve`).
2. **Item estrutural**: estender o sub-frame inline para carregar
   altura de contentor (`SubLayoutRegion`/`layout_sub_frame_inline`,
   caminho partilhado por todo o conteúdo em box).

## Decisão — scope-out reforçado

Sem consumidor em `cetz` (usa `Length` absolutos via `transform-point`,
`canvas.typ:141-156`, registado em P734). O custo inclui uma lacuna
estrutural (altura de contentor inline inexistente), não só mecânica de
path — desproporcional face ao benefício. **Scope-out reforçado com
custo medido**, como o passo antecipava.

### Descoberta colateral corrigida neste passo

O caminho real do utilizador (`polygon((50%, 0pt), ...)`) chega como
`Value::Relative` (abs zero), **não** `Value::Ratio` (inalcançável por
sintaxe, scope-out P725) — o braço de scope-out do P734
(`Value::Ratio`) nunca disparava e a mensagem real era absurda:
"expected relative length, found relative length". Novo braço
`Value::Relative` em `vertex_component` com mensagem explícita:

```
error: polygon(): coordenada relativa (50%) não é resolvível em tempo
de eval — scope-out (o vanilla resolve contra o contentor no layout)
```

Teste unitário `p741_polygon_ratio_relativo_scope_out_mensagem`
(constrói `Value::Relative` directamente — o caminho do eval).

## Validação global

- `cargo test --workspace`: **4772 passed, 0 failed** (4771 em P740 + 1
  teste novo).
- `crystalline-lint .`: 0 violations (`--fix-hashes` aplicado).
- E2E: a mensagem de scope-out dispara no caso real; `cetz` inalterado
  (1535/1478 px, diff 0.1477%, B−A=+57 — idêntico a P736–P740).

## Achados

O item "polygon com vértices Ratio" de `achados-adiados-cetz.md`
permanece **aberto**, agora com o scope-out reforçado e o custo
registado (dimensão de referência confirmada + lacuna estrutural
identificada).
