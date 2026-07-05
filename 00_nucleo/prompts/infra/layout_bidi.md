---
prompt: infra/layout_bidi
layer: L3
created: 2026-07-04
passo: P562
adr: ADR-0120, ADR-0109, ADR-0114
---

# Prompt L0 — Reordenação visual bidireccional de linhas (layout bidi)
Hash do Código: `PENDENTE — calcular após implementação e guarda pelo dono`

## Medições que fundamentam a decisão

1. `01_core/src/rules/layout/cursor.rs:134` — `layout_word` avança
   `self.regions.current.cursor_x += w;` sem qualquer noção de direcção.
2. `01_core/src/rules/layout/cursor.rs:149` — `layout_chunk` repete o
   mesmo padrão: avanço puramente LTR.
3. `01_core/src/rules/layout/text.rs:191` — `layout_word(part)` é
   invocado para cada segmento de `text.split(' ')`, mantendo a ordem
   dos tokens da esquerda para a direita.
4. `01_core/src/rules/layout/mod.rs:727` — o braço `Content::Text`
   delega em `text::layout(self, text)`; não existe arm de direcção nem
   campo de bidi no `Layouter`.
5. `03_infra/src/shaper.rs:121` — `bidi_runs(text.as_str())` existe em
   L3 e é usado **apenas** para decidir a direcção de shaping de cada
   sub-run; os `FrameItem::TextShaped` resultantes mantêm a posição base
   imposta pelo Layouter.
6. `03_infra/src/pipeline.rs:346-374` — a pipeline é `layout → shape →
   export`, portanto uma passagem pura sobre `PagedDocument` pode ser
   inserida entre layout e shape sem alterar a lógica de quebra de
   linha.

## Decisão arquitectural

**Opção escolhida:** passagem posterior em L3, entre `layout` e
`shape_document`, que reordena os `FrameItem::Text` dentro de cada linha
visual usando `unicode-bidi` (já dependência do workspace desde P484).

**Opções rejeitadas e porquê:**

- **Mudar o Layouter (L1)** para posicionar palavras com noção de
  direcção: exigiria trazer `unicode-bidi` para L1, violando a
  fronteira de I/O/fontes (bytes de fonte vivem em L3, ADR-0120) e
  aumentando a complexidade do hot path de layout/quebra de linha.
- **Reordenar após `shape_document`**: um `FrameItem::Text` pode ter
  sido dividido em múltiplos `FrameItem::TextShaped` (fallback por
  caractere/fonte). Reordenar antes do shaping mantém a unidade da
  palavra e evita ter de reconstruir textos fragmentados.

## Módulo

`03_infra/src/layout_bidi.rs`

## Propósito

Corrige a ordem visual das palavras em linhas com conteúdo RTL (árabe,
hebraico, etc.) sem alterar a quebra de linha nem o shaping interno de
cada palavra.

## API pública

```rust
pub fn reorder_bidi_document(doc: PagedDocument) -> PagedDocument
```

Chamada na pipeline logo após `layout_with_introspector_and_metrics` e
antes de `shape_document`:

```rust
let mut doc = layout_with_introspector_and_metrics(...);
doc.extracted_headings = extracted_headings;
// ...
let doc = crate::layout_bidi::reorder_bidi_document(doc);
let doc = crate::shaper::shape_document(world, doc);
```

## Comportamento

### 1. Agrupamento por linha visual

Para cada página, percorre os items em ordem e agrupa-os em linhas
visuais. Dois items pertencem à mesma linha quando partilham a mesma
**baseline y** (dentro de uma tolerância fixa de 0.01 pt para flutuante).

Itens fora de texto (`Shape`, `Image`, `Line`, etc.) mantêm-se na linha
mas não participam na reordenação bidireccional.

### 2. Detecção de necessidade de reordenação

Para cada linha, concatena o texto dos `FrameItem::Text` (em ordem LTR
imposta pelo Layouter) num buffer. Se o buffer for puramente LTR
(`BidiInfo::new` produzir um único run LTR), a linha não é alterada.

### 3. Reordenação com unicode-bidi

Se a linha contiver runs RTL:

- Usa `unicode_bidi::BidiInfo::new(buffer, None)` e
  `visual_runs(para, line)` para obter a ordem visual dos runs.
- Para cada run devolvido por `visual_runs`, identifica o subconjunto
  de `FrameItem::Text` (ou frações de `FrameItem::Text`) que cobre esse
  run.
- Substitui os items de texto da linha pelos mesmos items na ordem
  visual calculada, ajustando as coordenadas `x` para que fiquem
  contíguos sem sobreposição nem lacunas.

### 4. Preservação de propriedades

- A posição `y` (baseline) dos items não muda.
- Itens não-texto na linha mantêm a sua posição relativa: são
  considerados pontos de ancoragem neutros e não são deslocados
  horizontalmente.
- Altura da linha, leading e quebra de linha permanecem inalterados.

## Casos de teste mínimos

- `p562_reorder_arabic_line`: documento `الكتاب على الطاولة` — a
  primeira palavra fica à direita da linha, a última à esquerda.
- `p562_mixed_latin_arabic`: `الكتاب 42 على الطاولة` — o número 42
  mantém-se LTR no meio; só os trechos árabes invertem visualmente.
- `p562_latin_no_change`: texto latino puro — saída idêntica à entrada.
- `p562_empty_text_unchanged`: `FrameItem::Text` vazio — sem panic.
- `p562_line_with_shape_unchanged`: linha com `Shape` no meio — shape
  mantém posição; texto ao redor reordena-se correctamente.

## Scope-out

- Algoritmo completo UBA (Unicode Bidirectional Algorithm) com
  embeddings explícitos, overrides e isolates — a implementação cobre
  o caso de texto RTL/LTR natural via `unicode-bidi`, suficiente para
  documentos árabes/hebraicos simples.
- Mudança da direcção base da página (`dir: rtl`) — trata-se da
  ordenação visual dentro da linha, não do alinhamento de parágrafo.
- Texto vertical ou scripts top-down.

## Dependências

- `unicode-bidi` (já em `[workspace.dependencies]` desde P484).
- `typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt}`.
