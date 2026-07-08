# Estado das disparidades com o vanilla — depois de P593

**Data:** 2026-07-05  
**Actualizado:** 2026-07-07 (P602)

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
| Escrita vertical CJK (de cima para baixo) | P531, confirmado como scope-out específico em P576 (`ttb`/`btt`) |
| Quebra de linha para CJK/Thai (scripts sem espaços) | P531. Nunca testado directamente — a sequência RTL confirmou o algoritmo para árabe/hebraico, não para CJK/Thai. |
| PDF Tagged / PDF-UA (acessibilidade) | P531 |
| Compressão por object streams / cross-reference streams | P531 |
| Pacotes (`#import "@preview/..."`) | P531 |
| `subject` em `#set document(...)` | P536 |

## Scope-out deliberado, com razão escrita

| Item | Razão |
|------|-------|
| Stream de metadados XMP | Só `/Info` é emitido; cobre o caso comum (P536). |
| Zoom explícito nos destinos de bookmarks | Cristalino usa `null`; funciona nos leitores testados (P535). |
| Bookmarks só a partir de headings | Não suporta labels manuais (P535). |
| `y_offset` no emit PDF (diacríticos) | Scope-out histórico (P486). |
| `smcp` via OpenType real | Scope-out histórico; usa scaling em vez do mecanismo real (P486). |
| `x_advance` exacto vs `hmtx` | Scope-out histórico (P485). |
| Descritor PDF `CIDFontType2` vs `CIDFontType0C` para fontes CFF | Mecânico, não afecta renderização nos leitores testados (P523). |

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
| Nota de rodapé maior do que o espaço restante numa coluna | P595 — detecta overflow, emite aviso, e nunca descarta em silêncio |
| `/Producer` no `/Info` | P600 — medição mostrou que o vanilla 0.15.0 não usa `/Producer`; usa `/Creator`. O cristalino já preenche `/Creator (typst-crystalline)`. |
| `/Count` de bookmarks aberto/fechado | P602 — vanilla 0.15.0 usa `/Count -N` (negativo) para entradas com filhos (fechadas por defeito) e `/Count N` positivo na raiz `/Outlines`; implementado no cristalino. |
| Fusão de blocos `BT...ET` | Não corrigido — continua scope-out (ver acima, faltou listar antes; confirmar se ainda se aplica) |

## Ainda por confirmar (não é "corrigido", nem "scope-out" — é incerto)

| Item | Porquê fica incerto |
|------|----------------------|
| Benchmark completo (`macro-10x`) | Excede o tempo limite do script sempre que é tentado (P546, P563, P565, P593). Nunca foi medido de forma completa desde P548. Substituído por medições isoladas em cada passo, o que não cobre o mesmo terreno. |
| Cobertura de `text_width`/`line_content_right` fora dos ficheiros já revistos em P593 | P593 confirmou consolidação em `cursor.rs`, `helpers.rs`, `layout_bidi.rs`, `shaper.rs`. Não confirmou se `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs` (tocados em P579/P580) continuam a usar as suas próprias contas antigas, ou se já chamam as funções únicas. |
| Escrita vertical e a mesma classe de bug de largura letra→palavra→linha | Nunca construída; se for, precisa de reaproveitar a cascata de P593, não repetir os quatro erros já encontrados para RTL. |
| Documentos com `tracking` + texto árabe | P593 identificou que esta combinação nunca foi testada; a inconsistência de tracking entre `word_width`/`estimate_width` e as versões shaped foi corrigida na consolidação, mas sem teste específico desta combinação. |

---

## Resumo

Ausente por implementar: 11 itens, a maioria funcionalidades grandes (exportação, acessibilidade, pacotes) fora do que esta conversa tratou.

Scope-out com razão: 10 itens, todos pequenos, já decididos.

Corrigido: a lista mais longa — a maior parte do trabalho recente (P544 a P593) fechou disparidades reais, uma a seguir à outra.

Incerto: 4 itens, o mais importante sendo o benchmark completo, que nunca correu até ao fim desde P548.
