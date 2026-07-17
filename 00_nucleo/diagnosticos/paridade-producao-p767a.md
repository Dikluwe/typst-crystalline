# Relatório de paridade — P767a

**Passo:** 767a  
**Data:** 2026-07-15  
**Foco:** Implementar `Content::Shape` como bloco que quebra parágrafo, replicando `BlockElem::single_layouter` do Typst vanilla.  
**L0:** `00_nucleo/prompts/engine/layout/shape_block_behaviour.md`  
**Código alterado:**
- `01_core/src/engine/layout/shape.rs` — protocolo de bloco (flush, `above`/`below` de `1.2em`, colapso de margem) quando `!layouter.is_sub_frame`.
- `01_core/src/engine/layout/sequence.rs` — `Content::Shape` mantém a chain de colapso de blocos (`block_chain_active`/`prev_block_below_pending`).
- `03_infra/fixtures/p307b/reference/04-shapes.pdf` e `07-multi-feature.pdf` — snapshots actualizados (mudança de layout esperada).

**Estado de validação:** `cargo test --workspace` verde; `crystalline-lint .` com zero violações excepto `V7` (prompt órfão já conhecido, `package_version_resolution.md`).

---

## 1. Metodologia

Comparação rasterizada vanilla vs cristalino:

```bash
# vanilla
lab/typst-original/target/release/typst compile --font-path lab/typst-original/assets/fonts

# cristalino
target/release/typst --font-path lab/typst-original/assets/fonts

mutool draw -r 300 -o <base>.png <base>.pdf
compare -metric AE <base>-vanilla.png <base>-cristalino.png <base>-diff.png
```

Documentos de teste gerados em `temp_p763h/`.

---

## 2. Primitivas isoladas (baseline P763h)

| Primitiva | Documento | AE |
|-----------|-----------|-----|
| rect | `#rect(width: 1cm, height: 0.8cm, fill: red)` | 213 |
| square | `#square(width: 1cm, fill: red)` | 119 |
| ellipse | `#ellipse(width: 1cm, height: 0.6cm, fill: red)` | 223 |
| circle | `#circle(radius: 0.5cm, fill: red)` | 268 |
| line | `#line(end: (1cm, 0pt), stroke: red)` | 5 |
| polygon | `#polygon((0pt,0pt), (1cm,0pt), (0.5cm,1cm), fill: red)` | 285 |

**Resultado:** baseline mantido; nenhuma regressão nas formas isoladas.

---

## 3. Primitivas misturadas com texto

| Primitiva | Documento | AE |
|-----------|-----------|-----|
| rect | `A #rect(width: 1cm, height: 0.8cm, fill: red) B` | 6 979 |
| square | `A #square(width: 1cm, fill: red) B` | 7 005 |
| ellipse | `A #ellipse(width: 1cm, height: 0.6cm, fill: red) B` | 6 911 |
| circle | `A #circle(radius: 0.5cm, fill: red) B` | 7 076 |
| line | `A #line(end: (1cm, 0pt), stroke: red) B` | 1 915 |
| polygon | `A #polygon((0pt,0pt), (1cm,0pt), (0.5cm,1cm), fill: red) B` | 6 504 |

**Observação:** o valor de AE ainda é alto porque o cristalino ainda não implementa o espaçamento de parágrafo para texto puro (parágrafo sem formas). A estrutura visual, contudo, está correcta: a forma quebra o parágrafo, o texto anterior fica numa linha, o texto posterior começa abaixo da forma, e não há sobreposição. O espaçamento forma-forma foi verificado com `mutool trace` e bate com o vanilla (por exemplo, 13.2 pt entre dois `rect` consecutivos, equivalente a `1.2em`).

---

## 4. Checklist de sub-layouts (regra 5 do handoff)

| Cenário | Documento | AE | Nota |
|---------|-----------|-----|------|
| grid | `#grid(columns: 2, gutter: 5pt, [A], rect(...), [B], [C])` | 4 226 | Forma tratada como bloco dentro da célula; diverge do vanilla, que a mantém inline dentro da célula. |
| box | `A #box[#rect(...) B] C` | 19 684 | Forma quebra parágrafo dentro da `box`; no vanilla a forma comporta-se como inline-block dentro da caixa. |
| columns | `#columns(2)[A #rect(...) B]` | 6 972 | Estrutura visual semelhante ao vanilla; AE explicado sobretudo por diferenças de espaçamento de parágrafo de texto. |
| place | `A #place(top+right, rect(...)) B` | 996 | Sem regressão — caminho absoluto preservado. |

**Conclusão:** `place()` não regrediu, conforme exigido pelo L0. `grid` e `box` apresentam divergências porque o cristalino aplica o comportamento de bloco em qualquer contexto que não seja `is_sub_frame`, enquanto o vanilla trata formas como inline dentro de `box` e células de `grid`. Estes casos não estão no âmbito do L0 de P767a (que só garante `place()` inalterado) e ficam registados para passos futuros.

---

## 5. Caso cetz original

Documento:

```typ
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

| Cenário | AE |
|---------|-----|
| cetz-basic | 1 022 |

**Resultado:** sem regressão. O canvas do cetz usa posicionamento absoluto (`place`-based, corrigido em P763f) e não passa pelo fluxo de parágrafo normal.

---

## 6. Testes automatizados

```bash
cargo test --workspace
```

- **Resultado:** todas as suites passaram (4150 + 637 + 33 + 2 + 27 + 2 + ...). Nenhum teste de P745–P762 falhou.
- Snapshots de `p307b` actualizados conscientemente:
  - `03_infra/fixtures/p307b/reference/04-shapes.pdf`
  - `03_infra/fixtures/p307b/reference/07-multi-feature.pdf`

Os novos outputs são mais vanilla-like (formas empilhadas com espaço; o rect já não sobrepõe o texto "Subheading").

---

## 7. Linter

```bash
crystalline-lint --fix-hashes .
crystalline-lint .
```

- `shape.rs`: header `@prompt` actualizado para `00_nucleo/prompts/engine/layout/shape_block_behaviour.md`.
- Resultado final: zero violações excepto `V7` (prompt órfão pré-existente).

---

## 8. Decisões mantidas

- Não se replicou o aviso vanilla `block may not occur inside of a paragraph`, conforme L0.
- `Content::Curve` não foi alterado: `curve(...)` produz `Content::Shape` e está coberto; o enum separado `Content::Curve` fica para passo futuro se necessário.
- O comportamento de bloco só se aplica no fluxo principal (`!layouter.is_sub_frame`), preservando `place()` e outros sub-frames.
