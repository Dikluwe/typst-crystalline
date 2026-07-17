# Diagnóstico P763h — Auditoria de primitivas de desenho (inline vs block-level)

**Data da medição:** 2026-07-15T20:52:00-03:00  
**Commit base:** `f3eec288e91e29ce94264e4f6adf8c3090349676`  
**Working tree:** alterações não relacionadas (deleções pendentes em `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md`); nenhuma alteração de código deste passo.  
**Passo:** P763h  
**Objectivo:** Verificar se a divergência `line`/`circle` reportada em P763g é um padrão sistémico entre todas as primitivas de desenho, e registar a decisão antes de qualquer correcção.

---

## Parte A — Classificação no vanilla (código-fonte)

Leitura directa de `lab/typst-original/crates/typst-layout/src/engine.rs:765-803`:

```rust
const LINE_RULE: ShowFn<LineElem> = |elem, _, _| {
    Ok(BlockElem::single_layouter(elem.clone(), crate::shapes::layout_line).pack())
};
const RECT_RULE: ShowFn<RectElem> = |elem, _, styles| {
    Ok(BlockElem::single_layouter(elem.clone(), crate::shapes::layout_rect)
        .with_width(elem.width.get(styles))
        .with_height(elem.height.get(styles))
        .pack())
};
// ... SQUARE_RULE, ELLIPSE_RULE, CIRCLE_RULE, POLYGON_RULE, CURVE_RULE
```

Todas as primitivas de desenho são realizadas para `BlockElem::single_layouter`. Não há `InlineElem`, `InlineItem` nem comportamento de linha nos seus `ShowFn`. A busca por `Behaviour::Inline`/`is_inline` em `typst-library` e `typst-layout` não devolveu nenhuma marcação específica para estas primitivas.

| Primitiva | Inline ou block no vanilla (fonte) |
|---|---|
| `line` | **Block** (`BlockElem::single_layouter`, `rules.rs:765`) |
| `curve` | **Block** (`BlockElem::single_layouter`, `rules.rs:801`) |
| `path` | Não existe como elemento standalone no vanilla; caminhos são construídos internamente por `curve`/`polygon`. |
| `polygon` | **Block** (`BlockElem::single_layouter`, `rules.rs:797`) |
| `rect` | **Block** (`BlockElem::single_layouter`, `rules.rs:769`) |
| `square` | **Block** (`BlockElem::single_layouter`, `rules.rs:776`) |
| `circle` | **Block** (`BlockElem::single_layouter`, `rules.rs:790`) |
| `ellipse` | **Block** (`BlockElem::single_layouter`, `rules.rs:783`) |

**Conclusão da Parte A:** contrariamente à hipótese de P763g, o Typst vanilla trata **todas** as primitivas de desenho como elementos de **bloco**. O avanço vertical observado entre `line` e `circle` em P763g não é "leading entre elementos inline", mas sim o espaçamento normal de blocos (`BlockElem` com `above`/`below` implícitos) no fluxo do documento.

---

## Parte B — Estado actual no cristalino

```bash
grep -n "enum ShapeKind\|ShapeElem\|layout_shape" 01_core/src/entities/*.rs 01_core/src/engine/layout/*.rs
```

Todas as primitivas mapeiam para `Content::Shape(Arc<ShapeElem>)` (`01_core/src/entities/content.rs:410`) e o layout está centralizado em `01_core/src/engine/layout/shape.rs`. A função `layout`:

1. Chama `layouter.ensure_initial_baseline()`;
2. Resolve `width`/`height` (ou `dx`/`dy` para `Line`);
3. Chama `layouter.flush_line()`;
4. Posiciona a forma em `cursor_x`/`cursor_y` e avança `cursor_y += resolved_h`.

| Primitiva | Mapeamento cristalino (actual) |
|---|---|
| `line` | `ShapeKind::Line { dx, dy }` |
| `curve` | `ShapeKind::Path(...)` (via `01_core/src/engine/layout/curve.rs`) |
| `path` | `ShapeKind::Path(...)` (construído por `curve`/`polygon`) |
| `polygon` | `ShapeKind::Path(...)` |
| `rect` | `ShapeKind::Rect` |
| `square` | `ShapeKind::Rect` (helper morfológico) |
| `circle` | `ShapeKind::Ellipse` (`width == height`) |
| `ellipse` | `ShapeKind::Ellipse` |

**Conclusão da Parte B:** no cristalino, todas as primitivas passam pelo mesmo `ShapeElem`/`ShapeKind`. O layout tenta ser block-like (avanço vertical de `cursor_y`), mas opera **dentro do fluxo contínuo de texto** gerado por `Content::Sequence`, sem quebrar parágrafo. Isso explica a divergência em documentos mistos (ver Parte C).

---

## Parte C — Medições reais

Todos os testes usam `mutool draw -r 300` antes de `compare -metric AE`. Documentos gerados em `temp_p763h/` (artefactos temporários, não commitados).

### Isoladas (sem texto)

| Primitiva | Código | AE vanilla vs cristalino |
|---|---|---|
| `rect` | `#rect(width: 1cm, height: 0.8cm, fill: red)` | **213** |
| `square` | `#square(width: 1cm, fill: red)` | **119** |
| `ellipse` | `#ellipse(width: 1cm, height: 0.6cm, fill: red)` | **223** |
| `circle` | `#circle(radius: 0.5cm, fill: red)` | **268** |
| `line` | `#line(end: (1cm, 0pt), stroke: red)` | **5** |
| `polygon` | `#polygon((0pt,0pt), (1cm,0pt), (0.5cm,1cm), fill: red)` | **285** |
| `curve` | API incompatível entre vanilla e cristalino; não medido neste passo. | — |

### Misturadas com texto

| Primitiva | Código | AE vanilla vs cristalino |
|---|---|---|
| `rect` | `A #rect(...) B` | **6739** |
| `square` | `A #square(...) B` | **6752** |
| `ellipse` | `A #ellipse(...) B` | **6823** |
| `circle` | `A #circle(...) B` | **7070** |
| `line` | `A #line(...) B` | **1864** |
| `polygon` | `A #polygon(...) B` | **6563** |

### Verificação empírica do comportamento de bloco no vanilla

Documento `temp_p763h/par-test.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#par[A #rect(width: 1cm, height: 0.8cm, fill: red) B]
```

Vanilla emite:

```text
warning: block may not occur inside of a paragraph and was ignored
```

Isto confirma que o vanilla classifica `rect` (e, pelo mesmo mecanismo, as restantes primitivas) como **bloco**, não como conteúdo inline.

### Observação visual

- **Vanilla:** quando uma primitiva aparece no meio de texto a nível de documento, cada fragmento de texto e cada forma são empilhados como blocos independentes (linhas separadas com espaçamento de bloco).
- **Cristalino:** texto e formas continuam no mesmo fluxo de linha. Formas de altura zero (`line`) não avançam `cursor_y`, pelo que o texto seguinte fica na mesma baseline; formas de altura positiva empurram o texto para baixo, frequentemente sobrepondo-se. Este é um comportamento de **fluxo inline híbrido**, não de bloco.

---

## Decisão registada (regra 1)

| Opção | Viabilidade | Justificação |
|---|---|---|
| (1) Migrar para modelo inline real | **Rejeitada** | O vanilla **não** trata as primitivas como inline. Migrar para inline aumentaria a divergência em vez de reduzi-la. |
| (2) Replicar espaçamento dentro do modelo block | **Rejeitada** | A divergência não é apenas espaçamento; é o facto de o cristalino não quebrar parágrafo quando encontra uma forma no meio de texto. Ajustes locais de `cursor_y` não corrigem o modelo de fluxo. P763g já demonstrou que patches parciais pioram outros casos. |
| (3) Scope-out consciente | **Aceite** | A evidência mostra que o problema real é arquitetural: o cristalino precisa de fazer com que `Content::Shape` se comporte como um elemento de bloco que quebra o parágrafo corrente, analogamente a como o vanilla trata `BlockElem` dentro do realizador/flow. Essa mudança afecta `Content::Sequence`, o realizador e o layout de parágrafo, não apenas `shape.rs`. Não existe Prompt L0 para essa reformulação. |

**Decisão:** **scope-out** da correcção dentro de P763h. A divergência é registada como uma incompatibilidade de modelo (cristalino trata formas como conteúdo de fluxo contínuo; vanilla como blocos) e não como um bug localizável de cursor ou baseline.

---

## Validação

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

Não foram feitas alterações de código, pelo que o checklist de sub-layouts (grid, box, columns, place) não se aplica a uma correcção inexistente.

---

## Conclusão

- **Parte A:** todas as primitivas de desenho do vanilla (`line`, `curve`, `polygon`, `rect`, `square`, `circle`, `ellipse`) são **block-level** por construção (`ShowFn` → `BlockElem::single_layouter`).
- **Parte B:** o cristalino concentra todas estas primitivas em `ShapeElem`/`ShapeKind` e dispõe-as sequencialmente no fluxo de texto.
- **Parte C:** isoladamente, as formas cristalinas aproximam-se do vanilla (AE 5–285). Quando misturadas com texto, **todas** as primitivas testadas divergem fortemente (AE 1864–7070), confirmando um padrão sistémico.
- **Decisão:** scope-out. O padrão é amplo e a causa é o modelo de fluxo, não uma propriedade inline do vanilla. Corrigir isto exige um passo arquitetural dedicado com Prompt L0 próprio, fora do âmbito de P763h.
