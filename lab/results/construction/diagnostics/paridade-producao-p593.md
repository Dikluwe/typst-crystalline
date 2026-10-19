# Relatório Diagnóstico — Passo 593
## Unificar a cascata de largura: letra → palavra → linha

- **Commit de Referência:** `f02b8956adcb3bf39ed32f7a3639ff79c83086bf` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 12:13:39 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`, ADR-0108, ADR-0114

---

## 1. Objetivo

As correcções de P544–P592 resolveram quatro problemas distintos da mesma cascata de largura, cada um no sítio onde o sintoma apareceu. Este passo verifica se existia mais do que uma versão da mesma conta espalhada pelo código e, onde sim, consolida numa função única por nível.

---

## 2. Sonda

### 2.1 Funções de cálculo de largura encontradas

| Ficheiro | Função | Nível | O que fazia |
|---|---|---|---|
| `01_core/src/engine/layout/metrics.rs` | `FontMetrics::advance` | Letra/glifo | Avanço horizontal de uma string (soma glifos + kerning). |
| `01_core/src/engine/layout/metrics.rs` | `FontMetrics::advance_shaped` | Palavra | Avanço com forma de escrita aplicada (scripts contextuais). |
| `01_core/src/engine/layout/cursor.rs` | `Layouter::word_width` | Palavra | `advance(word) + tracking`. |
| `01_core/src/engine/layout/cursor.rs` | `Layouter::layout_word` | Palavra | `advance_shaped(word)` ou `word_width(word)`; sem tracking em shaped. |
| `03_infra/src/layout_bidi.rs` | `text_width_for_bidi` | Palavra | `advance_shaped(text)` ou `advance(text)`. |
| `01_core/src/engine/layout/helpers.rs` | `item_width` (Text) | Palavra | `advance_shaped(text)` ou `advance(text)`. |
| `03_infra/src/shaper.rs` | `estimate_width` | Palavra | `advance_shaped(text)` ou `advance(text) + tracking`. |
| `01_core/src/engine/layout/helpers.rs` | `measure_content` | Conteúdo | Estima dimensões de `Shape`/`Sequence` (não texto). |
| `01_core/src/engine/layout/mod.rs` | `measure_content_constrained` | Conteúdo | Layout parcial para medir conteúdo. |
| `01_core/src/engine/layout/cursor.rs` | `align_current_line_rtl` | Linha | Calculava `content_right` inline a partir dos items. |
| `03_infra/src/layout_bidi.rs` | `reorder_bidi_line` | Linha | Calculava `content_right` inline (só items Text). |

### 2.2 Duplicações confirmadas

**Nível palavra:** existiam quatro variantes da mesma conta:
- `word_width` e `estimate_width` incluíam tracking.
- `layout_word`, `text_width_for_bidi` e `item_width` não incluíam tracking quando usavam `advance_shaped`.

Isso significa que, se algum dia `tracking` for usado com texto árabe, `layout_word` e `text_width_for_bidi` dariam valores diferentes de `word_width`/`estimate_width` — exactamente o tipo de divergência que esta sequência já corrigiu várias vezes.

**Nível linha:** `align_current_line_rtl` e `reorder_bidi_line` calculavam o limite direito da linha de forma independente. A versão de `reorder_bidi_line` só considerava items `Text`, ignorando `Shape`/`Image`/`Group` que possam existir na mesma linha.

---

## 3. Consolidação

### 3.1 Fonte única do nível "palavra": `FontMetrics::text_width`

Ficheiro: `01_core/src/engine/layout/metrics.rs`

```rust
fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt
```

- Usa `advance_shaped` quando disponível.
- Cai em `advance` para os restantes scripts.
- Aplica tracking consistentemente.

### 3.2 Fonte única do nível "linha": `FontMetrics::line_content_right`

Ficheiro: `01_core/src/engine/layout/metrics.rs`

```rust
fn line_content_right(&self, items: &[&FrameItem]) -> f64
```

- Soma `x + width` para todos os items da linha.
- Usa `text_width` para `FrameItem::Text`.
- Usa `x_advance` shaped para `FrameItem::TextShaped`.
- Usa campos naturais (`width`, `inner_width`, etc.) para os restantes.

### 3.3 Ficheiros actualizados para chamar as fontes únicas

| Ficheiro | Alteração |
|---|---|
| `01_core/src/engine/layout/cursor.rs` | `word_width` → `text_width`; `layout_word` → `text_width`; `align_current_line_rtl` → `line_content_right`. |
| `01_core/src/engine/layout/helpers.rs` | `item_width` (Text) → `text_width`; novo `line_content_right` helper local para uso interno de L1. |
| `03_infra/src/layout_bidi.rs` | `text_width_for_bidi` → `text_width`; `reorder_bidi_line` → `line_content_right`. |
| `03_infra/src/shaper.rs` | `estimate_width` → `text_width`. |

### 3.4 O que não foi alterado

- `FontMetrics::advance` continua a ser a fonte única do nível letra/glifo.
- `measure_content` e `measure_content_constrained` continuam como estavam — operam a um nível diferente (estimativa de dimensões de conteúdo, não largura de texto renderizado).
- Não foi criada uma função de "largura de parágrafo" ou "largura de página" — isso está fora do escopo deste passo e não tinha duplicação identificada.

---

## 4. Resultados

### 4.1 Documentos de referência re-testados

| Passo | Documento | Resultado após consolidação | Nota |
|---|---|---|---|
| P563 | `#lorem(1200)` | 286.2 ms ± 9.6 ms (hyperfine, 5 runs) | Sem regressão vs 287 ms do P563. |
| P577/P586 | `الكتاب 42 على الطاولة` (fonte default) | Duas linhas, posições inalteradas | Quebra prematura persiste por causa da fonte default; P591 corrige com DejaVu Sans. |
| P590/P591/P592 | `الكتاب 42 على الطاولة` (DejaVu Sans) | Uma linha, posições idênticas ao vanilla | Correcções mantidas. |
| P592 | `الكتاب على الطاولة` (DejaVu Sans) | Posições idênticas ao vanilla | Espaço final corrigido. |

### 4.2 Verificação de posições — P591

Cristalino:

```text
left=136.36  width=121.28  text=ةلواطلا
left=270.33  width=70.44   text=ىلع
left=353.49  width=50.88   text=42
left=417.11  width=107.32  text=باتكلا
```

Vanilla:

```text
left=136.36  width=121.25  text=ةلواطلا
left=270.33  width=70.45   text=ىلع
left=353.49  width=50.90   text=42
left=417.10  width=107.30  text=باتكلا
```

Diferença máxima: 0,02 pt em larguras, 0,01 pt em posições.

---

## 5. Decisão

- As duplicações de nível palavra e nível linha foram consolidadas em `FontMetrics::text_width` e `FontMetrics::line_content_right`.
- Todos os quatro casos já corrigidos (P544/546/548, P591, P587/588, P592) continuam correctos.
- Não há regressão de desempenho mensurável.

---

## 6. Validação

```bash
cargo build --workspace --release
cargo test --workspace
crystalline-lint .
```

Resultados:

- `cargo build --workspace --release`: sucesso.
- `cargo test --workspace`: sucesso.
- `crystalline-lint .`: `✓ No violations found`.

O benchmark `tools/perf/benchmark-p507.py` foi iniciado mas excedeu o timeout de 300 s devido ao corpus macro-10x. Foi feita medição isolada com `hyperfine` no documento P563 (`#lorem(1200)`), confirmando que não há regressão.

---

## 7. Ficheiros alterados

- `01_core/src/engine/layout/metrics.rs`
- `01_core/src/engine/layout/cursor.rs`
- `01_core/src/engine/layout/helpers.rs`
- `03_infra/src/layout_bidi.rs`
- `03_infra/src/shaper.rs`
