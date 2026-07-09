# Estado das disparidades com o vanilla — depois de P593

**Data:** 2026-07-05  
**Actualizado:** 2026-07-09 (P621)

---

## Nunca implementado

| Item | Onde foi confirmado |
|------|---------------------|
| Exportação HTML | P526 |
| Exportação SVG | P526 |
| Exportação PNG/raster | P526 |
| IDE / LSP | P526 |
| Fontes de cor para emoji (COLR/CPAL/CBDT/CBLC/sbix) | P531 |
| Fontes Type1/PostScript | P531 |
| Escrita vertical CJK (de cima para baixo) | **Não é disparidade** — o vanilla 0.15.0 também rejeita `dir: ttb/btt` em `#set text(...)` (P616). O layout vertical de texto continua não implementado em ambos; é funcionalidade nova, não bug. |
| Quebra de linha para CJK/Thai (scripts sem espaços) | P531. Nunca testado directamente — a sequência RTL confirmou o algoritmo para árabe/hebraico, não para CJK/Thai. |
| PDF Tagged / PDF-UA (acessibilidade) | P531 |
| Compressão por object streams / cross-reference streams | P531 |
| Pacotes (`#import "@preview/..."`) | P531 |
| `subject` em `#set document(...)` | P536 |

## Scope-out deliberado, com razão escrita

| Item | Razão |
|------|-------|
| Zoom explícito nos destinos de bookmarks | Cristalino usa `null`; funciona nos leitores testados (P535). |
| `y_offset` no emit PDF (diacríticos) | Scope-out histórico (P486). |
| `smcp` via OpenType real | Scope-out histórico; usa scaling em vez do mecanismo real (P486). |
| `x_advance` exacto vs `hmtx` | Scope-out histórico (P485). |
| Descritor PDF `CIDFontType2` vs `CIDFontType0C` para fontes CFF | Mecânico, não afecta renderização nos leitores testados (P523). |
| Fusão de blocos `BT...ET` | P608 — confirmado que ainda diverge: cristalino gera 11 blocos num documento de fallback onde o vanilla gera 5. É uma optimização de export (o texto renderiza correctamente); correcção exigiria fundir runs adjacentes com fontes diferentes num único bloco `BT...ET`, o que está fora do scope actual. |

## Corrigido ao longo desta conversa (já não é disparidade)

| Item | Passo de fecho |
|------|-----------------|
| Shaping RTL (forma das letras) | P484, P521 |
| Fonte embutida no PDF para RTL (leitores a abrir o ficheiro) | P560 |
| Ordem visual das palavras numa linha RTL | P562, P564, P567 |
| Espaço entre palavras em RTL | P569, P592 |
| Quebra de linha prematura em RTL (causa: `font_size_pt` estático) | P579, P580, P582 |
| Quebra de linha prematura em RTL (causa: espaço inicial de parágrafo) | P587, P588 |
| Quebra de linha prematura em RTL (causa: largura sem forma de escrita aplicada) | P590, P591 |
| Alinhamento de parágrafo `dir: rtl` | P576, revisto e corrigido em P592 |
| Numeração de página (padrões simples e compostos) | P532, P541 |
| Citações bibliográficas inline | P533 |
| Formatação CSL da bibliografia | P547, P548 |
| Colunas: numeração de notas de rodapé, posicionamento | P552 |
| Colunas: paginação geométrica | P553 |
| Fonte por defeito (Helvetica → FreeSerif → Liberation Serif) | P554, P558 |
| Acentos trocados na extracção de texto | P558 |
| `#for` a descartar conteúdo | P538f |
| `#for` com destructuring de tuplos | P540 |
| `#{expr}` em markup | P545 |
| Caracteres de escape e abreviaturas tipográficas | P581, confirmado com teste em P584/P585 |
| Fallback de fonte por carácter (multi-script) | P534, P543 |
| Subsetting de fontes de fallback em `.ttc` | P609 — `FontSlot::get()` agora extrai a face individual de uma TrueType Collection antes de expor os bytes; o subsetter (`oxifont_subset`) recebe uma fonte simples em vez da coleção completa. Documento de teste de P608 passou de 15,7 MB para ~196 KB. |
| `tracking` + texto árabe / devanágari | P621 — tracking aplicado nos `x_advance` dos glifos durante o shaping (não no export PDF). O espaçamento é inserido entre clusters de caracteres distintos, preservando conjuntos em devanágari. Limitação visual separada: quebras de parágrafo em RTL ainda não são respeitadas (ver secção "Divergência conhecida" abaixo). |
| Nota de rodapé maior do que o espaço restante numa coluna | P595 — detecta overflow, emite aviso, e nunca descarta em silêncio |
| `/Producer` no `/Info` | P600 — medição mostrou que o vanilla 0.15.0 não usa `/Producer`; usa `/Creator`. O cristalino já preenche `/Creator (typst-crystalline)`. |
| `/Count` de bookmarks aberto/fechado | P602 — vanilla 0.15.0 usa `/Count -N` (negativo) para entradas com filhos (fechadas por defeito) e `/Count N` positivo na raiz `/Outlines`; implementado no cristalino. |
| Stream de metadados XMP | P611 — o vanilla 0.15.0 emite sempre um pacote XMP (`<?xpacket ... ?>`) mesmo sem metadados de utilizador. O cristalino passa a emitir o equivalente: `/Type /Metadata /Subtype /XML` referenciado no catálogo, com `CreatorTool`, datas, `NPages`, `format`, IDs e `PDFVersion`. |
| Bookmarks só a partir de headings | P604 — medição directa ao vanilla 0.15.0 mostra que só headings geram bookmarks; labels manuais (`#label(...)`) não entram em `/Outlines`, e `#outline(target: ...)` é para TOC/listas no documento, não para bookmarks PDF. O comportamento do cristalino coincide com o vanilla. |
| Parâmetro `bookmarked`/`outlined` de `heading()` | P606 — o vanilla 0.15.0 distingue `outlined` (índice do documento) de `bookmarked` (bookmarks PDF), com `bookmarked: auto` a seguir `outlined` por defeito. O cristalino agora implementa a mesma separação: `HeadingElem` tem `outlined: bool` e `bookmarked: Option<bool>`, e o walk emite `HeadingForToc` e `HeadingForBookmarks` independentemente. |

## Confirmado (medição completa)

| Item | Passo de confirmação | Resultado |
|------|----------------------|-----------|
| Benchmark completo (`macro-10x`) | P618 — correr `tools/perf/benchmark-p507.py` até ao fim (timeout 900s). Tempo real: ~4m41s. `macro-10x`: vanilla 5303 ms, cristalino 35958 ms, rácio **6.78×** (melhoria face a 11.98× em P548 e 28.68× em P546). Micro: mediana **1.70×**, média 3.07× (dois outliers de documentos muito pequenos: `test-stroke-sides` 26.27×, `test-image-fit` 25.70×). | Confirmado — o ajuste de P594 continua válido; o benchmark termina dentro do tempo. |

## Ainda por confirmar (não é "corrigido", nem "scope-out" — é incerto)

| Item | Porquê fica incerto |
|------|----------------------|
| Cobertura de `text_width`/`line_content_right` fora dos ficheiros já revistos em P593 | P593 confirmou consolidação em `cursor.rs`, `helpers.rs`, `layout_bidi.rs`, `shaper.rs`. Não confirmou se `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs` (tocados em P579/P580) continuam a usar as suas próprias contas antigas, ou se já chamam as funções únicas. |
| Escrita vertical e a mesma classe de bug de largura letra→palavra→linha | Nunca construída; se for, precisa de reaproveitar a cascata de P593, não repetir os quatro erros já encontrados para RTL. |

## Divergência conhecida (não corrigida nesta conversa)

| Item | Onde foi confirmado |
|------|---------------------|
| Quebras de parágrafo em RTL | P621 — um documento com duas linhas de árabe separadas por linha em branco é renderizado numa única linha visual no cristalino, independentemente de `tracking`. O vanilla 0.15.0 mantém as duas linhas. O problema é anterior a P621 e afecta a comparação visual de tracking. |

---

## Resumo

Ausente por implementar: 11 itens, a maioria funcionalidades grandes (exportação, acessibilidade, pacotes) fora do que esta conversa tratou.

Scope-out com razão: 10 itens, todos pequenos, já decididos.

Corrigido: a lista mais longa — a maior parte do trabalho recente (P544 a P593) fechou disparidades reais, uma a seguir à outra.

Incerto: 3 itens. O benchmark completo foi confirmado em P618; já não é incerto.
