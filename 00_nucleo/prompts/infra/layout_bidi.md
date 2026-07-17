---
prompt: infra/layout_bidi
layer: L3
created: 2026-07-04
updated: 2026-07-04
passo: P562, P564, P569
adr: ADR-0120, ADR-0109, ADR-0114, ADR-0108
---

# Prompt L0 — Reordenação visual bidireccional de linhas (layout bidi)
Hash do Código: 9e9de308

## Medições que fundamentam a decisão

1. `01_core/src/engine/layout/cursor.rs:134` — `layout_word` avança
   `self.regions.current.cursor_x += w;` sem qualquer noção de direcção.
2. `01_core/src/engine/layout/cursor.rs:149` — `layout_chunk` repete o
   mesmo padrão: avanço puramente LTR.
3. `01_core/src/engine/layout/text.rs:191` — `layout_word(part)` é
   invocado para cada segmento de `text.split(' ')`, mantendo a ordem
   dos tokens da esquerda para a direita.
4. `01_core/src/engine/layout/mod.rs:727` — o braço `Content::Text`
   delega em `text::layout(self, text)`; não existe arm de direcção nem
   campo de bidi no `Layouter`.
5. `03_infra/src/shaper.rs:508` — `bidi.visual_runs(para, line)` separa
   caracteres neutros (espaços) e sufixos LTR (pontuação, números) em
   runs próprios, o que faz com que extratores sequenciais como
   `pdftotext` percam ou desloquem o espaço entre palavras árabes.
6. `03_infra/src/pipeline.rs:346-374` — a pipeline é `layout → shape →
   export`, portanto uma passagem pura sobre `PagedDocument` pode ser
   inserida entre layout e shape sem alterar a lógica de quebra de
   linha.
7. `00_nucleo/diagnosticos/paridade-producao-p563.md` — a inversão
   simples de P562 preserva posições x do layout LTR. Em texto misto
   (`الكتاب 42 على الطاولة`), `الطاولة` (mais larga) é colocada na
   posição x de `الكتاب` (mais estreita), estoura a linha e salta para
   a linha seguinte. A correção exige recalcular as posições x a partir
   das larguras reais das palavras.
8. Sonda P569 (documento `معلومات قيمة.` com `lang: "ar"`) —
   `pdftotext` devolve `معلوماتقيمة.` (palavras coladas, sem espaço);
   `mutool draw -F txt` devolve `معلومات قيمة.` (visualmente correcto).
   A causa é a interacção entre (a) items de espaço neutro soltos pela
   separação em runs bidi e (b) sufixos LTR (o ponto final) colados ao
   run RTL seguinte na ordem do stream PDF.

## Decisão arquitectural

**Opção escolhida:** passagem posterior em L3, entre `layout` e
`shape_document`, que reordena os `FrameItem::Text` dentro de cada linha
visual usando `unicode-bidi` (já dependência do workspace desde P484),
recalcula as coordenadas x com base em `FontMetrics`, faz **reflow** de
blocos RTL adjacentes quando necessário e, a partir de P569, aplica um
pós-processamento de limpeza da ordem visual para preservar espaços
entre palavras árabes e isolar sufixos LTR.

**Opções rejeitadas e porquê:**

- **Mudar o Layouter (L1)** para posicionar palavras com noção de
  direcção: exigiria trazer `unicode-bidi` para L1, violando a
  fronteira de I/O/fontes (bytes de fonte vivem em L3, ADR-0120) e
  aumentando a complexidade do hot path de layout/quebra de linha.
- **Reordenar após `shape_document`**: um `FrameItem::Text` pode ter
  sido dividido em múltiplos `FrameItem::TextShaped` (fallback por
  caractere/fonte). Reordenar antes do shaping mantém a unidade da
  palavra e evita ter de reconstruir textos fragmentados.
- **Preservar posições x do layout LTR após inversão (P562)**: reprovado
  em P563 porque gera quebra de linha incorrecta em texto misto.
- **Reflow completo em cascata**: rejeitado por complexidade XL neste
  passo. A implementação limita-se a fundir parágrafos RTL — sequências
  de linhas consecutivas do mesmo parágrafo, incluindo linhas LTR
  intermédias como números — quando o texto total cabe numa única linha.
- **Corrigir a extracção ajustando `visual_runs` no shaper (P569)**:
  experimentado e revertido; alterar a ordem dos glifos no nível do
  shaping quebra a renderização visual. A solução correcta é no nível
  de `FrameItem::Text`, antes do shaping, onde ainda temos o texto
  plano e podemos recalcular posições e larguras.

## Módulo

`03_infra/src/layout_bidi.rs`

## Propósito

Corrige a ordem visual das palavras em linhas com conteúdo RTL (árabe,
hebraico, etc.) sem alterar a quebra de linha nem o shaping interno de
cada palavra, recalcula as posições x para que as palavras reordenadas
fiquem contíguas sem sobreposição nem lacunas e, em P569, garante que
espaços entre palavras RTL não desapareçam na extracção sequencial de
texto (ex.: `pdftotext`).

## API pública

```rust
use typst_core::engine::layout::FontMetrics;

pub fn reorder_bidi_document(
    doc: PagedDocument,
    metrics: &dyn FontMetrics,
) -> PagedDocument
```

Chamada na pipeline logo após `layout_with_introspector_and_metrics` e
antes de `shape_document`:

```rust
let mut doc = layout_with_introspector_and_metrics(...);
doc.extracted_headings = extracted_headings;
// ...
let metrics = crate::font_metrics::FallbackFontMetrics::new(world);
let doc = crate::layout_bidi::reorder_bidi_document(doc, &metrics);
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

### 3. Reordenação visual

Se a linha contiver runs RTL:

- Usa `unicode_bidi::BidiInfo::new(buffer, None)` para determinar a
  direcção base do parágrafo.
- Quando a direcção base é RTL, inverte a ordem visual dos
  `FrameItem::Text` da linha: a primeira palavra lógica passa a ocupar a
  posição mais à direita e a última a posição mais à esquerda.

### 4. Recálculo das posições x

Após a inversão, as posições x são recalculadas a partir das **larguras
reais** devolvidas por `FontMetrics::advance(text, size, style)`:

- Determina o intervalo horizontal ocupado pelos items de texto da
  linha: `x_min` (posição x do primeiro item na ordem original) e
  `x_max` (posição x do último item original, sem incluir a sua
  largura).
- Calcula o espaçamento médio entre items originais:
  `gap = (x_max - x_min - sum(widths[0..n-1])) / (n - 1)` quando
  `n > 1`; `gap = 0.0` quando `n <= 1`.
- Posiciona os items na ordem visual invertida a partir de `x_min`,
  avançando `x += width_i + gap` para cada item.

Itens de texto vazios contribuem com largura zero.

### 5. Ordenação do vector por x crescente (P569)

Após reposicionar visualmente, os items de cada linha são reordenados no
vector `page.items` para a ordem visual esquerda→direita (x crescente).
Extratores sequenciais como `pdftotext` seguem a ordem dos operadores no
stream PDF; sem esta ordenação, caracteres RTL podem ficar na ordem
lógica e espaços entre palavras podem desaparecer na extracção.

### 6. Coalescência de espaços soltos (P569)

Items de texto cujo conteúdo seja apenas espaços (ou whitespace) são
**coalescidos no item de texto anterior na ordem visual** (menor x),
transformando-se num *trailing space* desse item. Isto posiciona o
espaço entre palavras adjacentes na ordem do stream, evitando que o
espaço neutro seja capturado por sufixos LTR ou descartado pelo
shaper/extractor.

A função não altera o comprimento do vector; items totalmente vazios
ficam com texto `EcoString` vazio e são ignorados pelo export.

### 7. Separação de sufixos LTR (P569)

Após a coalescência, cada `FrameItem::Text` que termine num sufixo com
direcção forte LTR (pontuação, dígitos e outros caracteres `L`/`EN`/`AN`
da classificação bidi) é dividido em dois items:

- O **sufixo** é removido do texto original e colocado num novo
  `FrameItem::Text` independente, com o mesmo `TextStyle`, posicionado
  imediatamente à esquerda do item original (`x = x_original -
  largura_do_sufixo`, mesmo `y`).
- O **texto base** (RTL) permanece na posição original.

O sufixo é identificado a partir do final da string: percorre os
caracteres de trás para a frente enquanto forem espaços ou tiverem
classificação bidi forte LTR (`L`, `EN`, `AN`). O primeiro caractere que
não satisfizer essa condição define o limite. Espaços iniciais do sufixo
são removidos e não produzem item separado.

Esta separação evita que o ponto final (ou outro sufixo LTR) fique
colado ao run árabe seguinte no stream PDF, o que era a causa raiz do
espaço perdido em `pdftotext`.

### 8. Reflow de parágrafos RTL (P565/P567)

Após a reordenação individual de cada linha, a passagem identifica
**parágrafos RTL** — sequências de linhas consecutivas na mesma página
que fazem parte do mesmo parágrafo (baseline y próximo, até 1.5× a
altura da linha) e cujo conjunto de items seja predominantemente RTL.

Para cada parágrafo:

- Coleta todos os `FrameItem::Text` do parágrafo, na ordem lógica em
  que o Layouter os emitiu.
- Calcula a largura total: soma das larguras reais dos items.
- Calcula a largura útil disponível: `page_width - 2 * x_min`, onde
  `x_min` é a menor posição x dos items do parágrafo (aproxima a margem
  esquerda).
- Se a largura total couber na largura útil, funde o parágrafo numa
  única linha. As palavras são reordenadas visualmente (RTL) e
  reposicionadas a partir de `x_min`, distribuindo o espaço restante
  uniformemente como `gap` entre os items.
- Se a largura total exceder a largura útil, o parágrafo não é fundido.
- Linhas LTR intermédias (ex.: números dentro de texto árabe) são
  incluídas na run, desde que a run total seja predominantemente RTL.

O reflow aplica os passos 5, 6 e 7 à linha resultante.

### 9. Preservação de propriedades

- A posição `y` (baseline) dos items não muda, excepto quando um bloco
  é fundido: nesse caso, os items das linhas subsequentes movem-se para
  a `y` da primeira linha do bloco, e as linhas fundidas desaparecem.
- Itens não-texto dentro de um bloco são considerados ancoragens
  fixas: o reflow não move items não-texto nem funde blocos que os
  contenham entre as linhas afectadas.
- Altura da linha, leading e paginação permanecem inalterados; o
  reflow só afecta a distribuição horizontal de palavras dentro de um
  parágrafo RTL.

## Casos de teste mínimos

- `p565_latin_no_change`: documento LTR puro — saída idêntica à entrada,
  incluindo posições x e quebras de linha.
- `p565_reorder_arabic_line`: documento `الكتاب على الطاولة` — textos
  invertidos e posições x recalculadas com base nas larguras reais; a
  primeira palavra visual (`الطاولة`) fica em `x_min` e a última
  (`الكتاب`) em `x_max - largura`.
- `p565_mixed_latin_arabic`: `الكتاب 42 على الطاولة` a 20 pt — o número
  42 mantém-se LTR no meio; os trechos árabes invertem visualmente e as
  posições x de todos os items são recalculadas.
- `p565_mixed_40pt_reflow`: `الكتاب 42 على الطاولة` a 40 pt — o
  Layouter LTR coloca `الطاولة` numa segunda linha; o reflow funde o
  parágrafo numa única linha quando o texto total cabe na largura útil,
  mesmo com `42` (LTR) entre palavras árabes.
- `p565_empty_text_unchanged`: `FrameItem::Text` vazio — sem panic.
- `p565_line_with_shape_unchanged`: linha com `Shape` no meio — shape
  mantém posição; texto ao redor reordena-se correctamente; reflow não
  funde linhas com shapes.
- `p569_arabic_words_not_glued`: documento `معلومات قيمة.` com
  `lang: "ar"` — após a passagem, `pdftotext` deve extrair as duas
  palavras separadas por um espaço; a ordem visual pode permanecer
  invertida (`قيمة. معلومات`/`قيمة .معلومات` conforme extractor), mas
  o espaço entre palavras não pode desaparecer.
- `p569_ltr_suffix_split`: item `FrameItem::Text` contendo
  `قيمة.` numa linha RTL — deve ser dividido em dois items independentes
  (`قيمة` e `.`), com o ponto posicionado imediatamente à esquerda do
  texto árabe.
- `p569_trailing_space_coalesced`: linha RTL com items `معلومات`,
  `<espaço>`, `قيمة` — o espaço torna-se trailing space de `قيمة`
  (a palavra que o precede visualmente, mais à esquerda), de modo que o
  stream esquerda→direita leia `قيمة معلومات`.

## Scope-out

- Algoritmo completo UBA (Unicode Bidirectional Algorithm) com
  embeddings explícitos, overrides e isolates — a implementação cobre
  o caso de texto RTL/LTR natural via `unicode-bidi`, suficiente para
  documentos árabes/hebraicos simples.
- Garantia de ordem lógica na extracção de texto por `pdftotext`. O
  objectivo de P569 é **preservar os espaços entre palavras** na
  extracção sequencial; a inversão visual das palavras (ordem
  direita→esquerda no texto extraído) é um comportamento conhecido e
  aceite para este passo.
- Texto vertical ou scripts top-down (ver secção seguinte).

## Alinhamento de parágrafo com `text.dir` — Passo 576

A direcção base do parágrafo passa a ser controlada pela propriedade
`dir` do `#set text(...)`. Quando `text.dir == Dir::RTL`, o parágrafo
inicia-se na margem direita e flui para a esquerda; quando `Dir::LTR`,
comporta-se como o default actual (margem esquerda).

### Decisão arquitectural

**Opção escolhida:** o Layouter (L1) lê `text.dir` da `StyleChain` e
ajusta a origem horizontal do parágrafo. A passagem `layout_bidi` em L3
continua a responsável pela reordenação visual das palavras dentro da
linha e pelo reflow de parágrafos RTL, mas assume que as linhas já
começam na margem correcta.

**Opções rejeitadas:**
- **Fazer o alinhamento só em L3 (passagem posterior)**: deslocar blocos
  inteiros após o layout LTR esconde o problema da quebra de linha e
  não reproduz o comportamento vanilla para texto misto.
- **Usar a detecção automática de direcção (`unicode-bidi`) para
  alinhar**: o vanilla só alinha à direita com `dir: rtl` explícito;
  texto árabe sem `dir:` mantém o alinhamento esquerdo.

### Medições que fundamentam a decisão

- `01_core/src/engine/eval/rules.rs:931` — `eval_set_rule` target `text`
  trata `bold`, `italic`, `size`, `fill`, `weight`, `tracking`, `lang`,
  `font`; `dir` cai no warn de propriedade não suportada.
- `01_core/src/engine/eval/mod.rs:1196-1203` — `left`, `center`, `right`,
  `start`, `end`, `top`, `horizon`, `bottom` são expostos como
  `Value::Align`; `ltr`/`rtl`/`ttb`/`btt` ainda não existem no escopo.
- `01_core/src/engine/layout/mod.rs:499-501` — `cursor_x` e
  `line_start_x` inicializam-se em `margin`, fixando a origem LTR.
- `01_core/src/engine/layout/text.rs:54` — `text.lang` é lido da chain;
  `text.dir` ainda não é.
- Sonda P576: documento árabe sem `dir:` começa à esquerda no vanilla e
  no cristalino; com `dir: rtl` o vanilla começa à direita, o cristalino
  ainda não suporta a propriedade.

### Comportamento

1. O eval aceita `dir: Dir` em `#set text(...)` e transporta
   `text.dir` na `StyleChain` como `Value::Dir`.
2. `engine/layout/text.rs` lê `text.dir` e coloca-o no `TextStyle.dir`.
3. No início de cada parágrafo (ou bloco de texto contínuo), o Layouter
   consulta `TextStyle.dir`:
   - `Dir::LTR` (default): origem em `margin` (comportamento actual).
   - `Dir::RTL`: origem em `width - margin` e avanço do cursor para a
     esquerda; a quebra de linha ocorre quando `cursor_x < margin`.
4. A passagem `layout_bidi` preserva o comportamento de P562/P564/P569
   e ajusta as posições x das palavras dentro da linha já originada à
   direita.

### Casos de teste mínimos

- `p576_rtl_starts_right`: `#set text(dir: rtl, lang: "ar")` + texto
  árabe curto → linha começa na margem direita.
- `p576_ltr_starts_left`: `#set text(dir: ltr, lang: "ar")` + texto
  árabe curto → linha começa na margem esquerda (comportamento default).
- `p576_mixed_paragraphs`: documento com parágrafo árabe (`dir: rtl`) e
  parágrafo latino (`dir: ltr`) → cada um alinhado no seu próprio lado.
- `p576_no_regression_p569`: documentos de referência de P569 continuam
  com palavras separadas e com o novo alinhamento quando `dir: rtl`.

## Sugestões para passos futuros

A arquitectura de passagem posterior deste passo resolve apenas a
**reordenação visual dentro de linhas horizontais**. Outras direcções
de escrita são deixadas para passos dedicados:

- **Escrita vertical CJK (top → bottom, colunas da direita para a
  esquerda)**: requer alterar o Layouter para avançar `cursor_y` em
  vez de `cursor_x`, trocar a lógica de quebra de linha, e rotacionar
  glifos durante o shaping. Não é uma passagem posterior pura.
- **Escrita vertical bottom → top (ex.: mongol tradicional)**: similar
  ao CJK, mas com direcção de linha invertida e possíveis diferenças
  na rotação de glifos.

Estes casos devem ter os seus próprios Prompts L0 e não devem ser
acrescentados a este passo, sob pena de inflacionar o escopo para XL+
e atrasar a entrega do RTL horizontal.

## Dependências

- `unicode-bidi` (já em `[workspace.dependencies]` desde P484).
- `typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt}`.
- `typst_core::engine::layout::FontMetrics`.
- `crate::font_metrics::FallbackFontMetrics` para a chamada na pipeline.
