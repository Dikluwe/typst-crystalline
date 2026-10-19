# Prompt L0 — layout_types
Hash do Código: 58bcffb3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

## Identidade do hífen discricionário — contrato aprovado ADR-0127

### Medição anterior à decisão

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
registra UTC, working tree não commitada sobre
`17f13b1df79b87ea065ee62e78164b18e477da3d`, lista exata de alterações,
hashes das fontes e dos binários. SHA-256 do Tekt medido:
`1ca83fd69c72643e53d54dcbdad9488e0738cb2d37d71057a17e61ffa3b03717`.
Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
`before/resultados.json` conserva comandos, PDFs, texto UTF-8 e caixas.
Oito entradas en/pt, simple/optimized e justify false/true conservam palavras
e caixas após normalização exclusivamente visual SHY→'-' (tolerância 0.015pt),
mas as trinta ocorrências discricionárias são U+002D no bbox Tekt e U+00AD no
vanilla. A extração comum de pdftotext remove vários hífens de fim de linha;
não prova ausência da ocorrência no PDF. Comparar codepoints sem normalização
textual e conservar ambas as extrações.

`compiler/layout/paragraph_hyphen.rs:208-214` materializa '-' comum;
`compiler/layout/break_selection.rs:51-58` concatena '-' ao prefixo local.
`word_layout.rs:137-144` emite Text sem causa distinta e
`shaper/traversal.rs:72-75` recebe somente essa string.
`entities/layout_types.rs:398-415` separa texto/glyphs apenas em TextShaped,
produzido em L3: L1 não possui a face para fabricar esse estado (P484).
`export/fonts.rs:146-154` coleta texto de clusters; os builders registram
uma identidade ToUnicode por GID. Um mesmo glifo '-' pode servir texto
literal e discricionário: alterar seu mapeamento global perde essa distinção.
Vanilla `inline/line.rs:173-179` pede hyphen soft na quebra selecionada;
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:583-635`
usa o glifo de '-' e conserva SHY no texto. Não requer sua mecânica ou bytes.
Controles de compostos e SHY autoral em before/ têm divergências adicionais;
não certificam seleção UAX14 ou SHY não quebrado. Não inferir intenção histórica.
A hipótese de perda causal seria refutada por transporte já existente antes
do shaping; os contratos e os dois emissores lidos não o conservam.

### Contrato público proposto

Adicionar à enum fechada FrameItem um envelope de identidade textual:

```rust
TextReplacement {
    text: EcoString,
    items: Vec<FrameItem>,
}
```

`text` é o texto extraível da ocorrência inteira; `items` são sua representação
visual já posicionada. O envelope não tem posição, transformação, fonte,
largura ou altura própria. Bounds, métricas, tradução, clipping e reconciliação
operam nos filhos exatamente uma vez. Recursos/shaping consultam os filhos
visuais; text não cria glifo, avanço ou nova medição.

A primeira materialização usa somente a causa de hifenização já selecionada.
No caminho global, o filho desenha '-' e text contém U+00AD. No incremental,
pode envolver o prefixo inteiro já medido/desenhado, com text igual ao prefixo
lexical seguido de U+00AD, preservando kerning. O seletor fornece essa distinção
por dados privados explícitos; não inferir causa de sufixo '-', posição ou GID.
Hífen literal segue como Text normal U+002D. Não introduzir candidatos novos,
reconhecimento de SHY autoral ou alterações de penalidades por este envelope.

The initial hyphenation materialization uses Text/TextShaped children in
order. A mathematical Large operator may use already-positioned Glyph parts
as visual children of one TextReplacement occurrence. Neither use nests a
TextReplacement, and the envelope itself has no independent geometry.
Visitors de recursos descem nos filhos, sem novo método público de trait.
Extractors textuais usam text em vez de duplicar a extração dos filhos;
plain_text conserva sua junção aproximada e não vira oráculo lexical.
Shaping e fallback preservam o envelope ao processar os filhos. Não fundir
seus filhos com texto externo nem apagar o envelope em merge/reflow.
Group/Link/Semantic e as camadas da página conservam a ocorrência. SVG visual
percorre os filhos; HTML semântico continua regido por seu pipeline próprio.

PDF substitui texto por ocorrência, independente de tagging e Compact/Verbose.
ActualText codifica text em UTF-16BE com BOM e equilibra BDC/EMC em top/local.
Filhos conservam GIDs, avanços e ToUnicode visual; o hífen literal que compartilha
GID mantém U+002D. Type1/fontes embutidas recebem a mesma identidade. ActualText
não fabrica MCID, structure tree, Alt de fórmula ou prova de acessibilidade.
O handler de substituição pertence a unidade L3 própria com owner L0 exclusivo;
dispatch estático/exaustivo permanece. Não usar Semantic.alt como substituição.

### Gate aprovado e aceitação

O dono confirmou “confirmo” nesta tarefa após revisão da proposta
SHA-256 b744f80e801eceddefadc76e79cf343ba19174d65ad84f53a2c4b9ccaebc1870.
Gate ADR-0127 satisfeito; autorização e UTC estão em authorization.json do
lote. Atualizar owners antes de código e resselo restrito. Os invariantes
normativos compartilhados pertencem ao Núcleo pinado abaixo (ADR-0129).

RED→GREEN deve provar transporte global/incremental, extração U+00AD nas
ocorrências selecionadas e U+002D nos hífens literais coexistentes, inclusive
com mesma fonte/GID. Conferir simple/optimized, justify, overhang, estilos,
fontes, grupos/links, Type1, Compact/Verbose e tags ligados/desligados.
Medir geometria separadamente contra baseline; preservar quebras/avanços e
aparência. Exercitar visitors, tradução e fallback sem perda/duplicação de
identidade. Revalidar regressões, build e lint. SHY autoral e compostos ficam
controles abertos até medição/decisão próprias; não declarar paridade completa.


## Separadores textuais — contrato aprovado ADR-0127 (2026-09-28)

### Medição anterior à decisão

As sondas de `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/`
usaram o working tree sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`,
binário reconstruído e vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`A B C` e `#text("A B C")` extraem `ABC` no PDF Tekt e `A B C` no vanilla;
`A#("B")C` extrai `ABC` nos dois. Os comandos, timestamps, fontes, hashes
binários e PDFs estão na proveniência dessa medição.

`compiler/eval/mod.rs:1473` conserva Space/SpaceFromNewline. O adapter
`compiler/layout/space.rs:9` e os separadores internos de
`compiler/layout/text.rs:229` avançam o cursor sem transportar um item que
identifique a origem lexical do separador. `export/stream/text.rs` agrupa
fragmentos shaped e compensa distâncias; a nova unidade proprietária desse
agrupamento é `00_nucleo/prompts/infra/export/stream/text/runs.md`. Uma distância não identifica
por si só um espaço. `layout_types.rs:903,930` insere separadores no helper
plain_text, de modo que esse helper não prova retenção da origem.

A diferença de glyph buffers/operadores é mecânica; separação de palavras,
texto extraível e geometria do documento são os observáveis investigados.
A divergência foi reproduzida com pdftotext, não generalizada para todos os
leitores. Rastreios que revelem outra proveniência causal preservada ou
controles que invalidem a separação lexical refutam esta classificação.

### Contrato público aprovado

Adicionar a `FrameItem` uma variante de transporte textual explícito:

```rust
TextSeparator {
    pos: Point,
    advance: Pt,
    style: TextStyle,
    origin: TextSeparatorOrigin,
}
```

Adicionar a enum pública de dados:

```rust
pub enum TextSeparatorOrigin {
    Markup,
    Newline,
    Literal,
}
```

Cada carrier identifica um espaço U+0020 efetivamente retido pelo layout.
Markup representa Space normalizado; Newline conserva a origem de
SpaceFromNewline; Literal corresponde a cada espaço literal já percorrido
pelo adapter Text. `pos` é a posição causal de início do avanço na baseline;
`advance` é a largura física efetivamente atribuída ao separador, em pontos;
`style` conserva a identidade de fonte/estilo usada naquele ponto. Esses
campos são dados, sem método de render nem import de compiler.

O carrier é neutro para tinta e métricas tipográficas: não cria linha,
baseline, ascent/descent, altura ou fonte de imagem/forma. Sua largura não é
somada uma segunda vez ao cursor. O layout mantém a supressão de espaços
iniciais e as decisões de wrapping/justificação existentes. Text e Space
produzem a mesma classe de dado por suas unidades proprietárias, conservando
as diferenças de origem e das regras locais de avanço/flush.

Translate/rebase, Group/Link/Semantic, rollback de suffixes, direção e shaping
transportam a ocorrência em ordem causal. Acrescentar carriers é append-only;
não mudar payloads do prefixo já certificado por um checkpoint de comprimento.
A identidade do separador não é recuperada por procura de conteúdo igual nem
por classificação de qualquer gap positivo entre glifos.

O exportador preserva a ocorrência textual como espaço extraível usando a
fonte preparada e respeitando pos/advance; resolve/subseta/mapeia o recurso
necessário pelo owner de recursos. A neutralidade tipográfica pertence ao
layout; emissão e agrupamento pertencem às unidades L3. Visitores exaustivos
tratam a variante explicitamente. A implementação segue a topologia estática
existente; o carrier não escolhe algoritmos nem executa callbacks.

### Gate aprovado e aceitação

O dono confirmou “Autorizar o contrato proposto” em 2026-09-28, nesta tarefa.
O gate ADR-0127 para a variante pública e a enum de origem está satisfeito.
A aprovação e o L0 submetido foram registrados na proveniência da materialização.

Após confirmação, cada consumer afetado recebe seu L0 próprio atualizado
antes de código. Invariantes normativas compartilhadas, quando pinadas por
mais de um owner, pertencem a um Núcleo Tekt; este prompt não assume ownership
dos adapters, tradutores, shaper ou exportadores. Nenhum código aponta para
Núcleo diretamente. A atomização pode acrescentar unidades dentro das camadas
atuais, cada qual com consumer exclusivo.

O gate funcional da materialização é RED→GREEN na extração PDF de `A B C`,
`A  B   C` e `#text("A B C")`, mantendo `A#("B")C == ABC`. Exercitar espaços
em estilos/fontes distintos, newline, limites de linha/região, justificação,
tracking/kerning, RTL, callbacks/suffix rollback e grupos/links. Comparar a
geometria do conteúdo visível com a base medida, os diagnósticos e a retenção
do texto; identidade de bytes, número de itens ou passos internos não fecha
paridade. Testes de helper com separadores inventados não substituem o
percurso fonte→PDF. Revalidar build e crystalline-lint após materialização.

## P1339 — causa existente das conversões de Angle

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`layout_types.rs:1277-1290` já define Angle em radianos e os métodos
to_rad/to_deg. A fonte vanilla ratificada `layout/angle.rs:142-151`
documenta conversões Angle → float, não os constructors Rust homônimos.
A medição histórica conserva expressões, saídas, UTC e proveniência; os
owners relevantes são os declarados neste L0 e em seus consumers atuais.

### Decisão

Angle conserva a unidade interna em radianos; to_rad devolve essa medida
como float, to_deg devolve a equivalente em graus. Estas são as causas
existentes reutilizadas por angle.rad/angle.deg. Os constructors Rust
Angle::rad/deg não são essas operações públicas de conversão.

Preservar finitos, zeros com sinal e infinitos; não alterar representação,
assinaturas, operações aritméticas ou construção de Angle. Não criar novo
módulo para copiar estas fórmulas. Divergência pública de precisão nas
entradas medidas refuta a suficiência e exige correção de causa, não
arredondamento arbitrário no repr. Angle NaN permanece sob a resolução
condicional já autorizada na decisão registrada:
não modificar aritmética para fabricar receiver bilateral.

## P1286 — attachments e artifact semântico propostos (GATE ADR-0127)

### Medição anterior à decisão

`PagedDocument` não transporta anexos. `FrameItem::Semantic` já transporta
Formula e builder/stream constroem tagging real; reutilizar Formula para
artifact emitiria `/Formula` e MCID, semanticamente errado.

### Contrato público proposto

- `PagedDocument` ganha `pub attachments: Vec<Arc<PdfAttachElem>>`, em ordem
  documental. O layout coleta markers invisíveis; nenhum `FrameItem` visual é
  criado. `plain_text` ignora o vetor.
- `SemanticKind` ganha `Artifact(ArtifactKind)`. O envelope preserva filhos
  visuais, usa `alt: None` e não participa na árvore estrutural/MCIDs.
  Visitors continuam descendo nos filhos.

Não há nova fase: eval produz markers, layout coleta/embrulha, export consome
o `PagedDocument`. São alterações de enum/campo público e ficam bloqueadas até
confirmação ADR-0127. `PdfTags::Enabled` continua default; Disabled preserva
o visual sem tags.

## P1140.20.2 — canvas e camadas

Conforme `entities/page_canvas.md`, PageConfig ganha bleed folded, PageFill,
background e foreground declarativos. Page ganha bleed físico, fill e vetores
separados background/body/foreground. Width/height seguem trimados. Plain
text/visitors semânticos percorrem só body; export visual percorre três layers.

## P1140.20.1 — geometria lógica

Tipos normativos pertencem a `entities/page_geometry.md`. `PageConfig` ganha
`binding: PageBinding`; margem preserva especificação folded e resolve
snapshot físico por página. Flip aplica exatamente uma vez no delta.

O default deriva de `Paper::A4.width_pt()/height_pt()`: remover `595.28`,
`841.89` e fallbacks em pontos. Margem mantém `2.5 / 21`; dois eixos infinitos
usam A4 normativo. `Page` guarda margens físicas, não `paper`. Propriedades
P1140.20.2–.4 continuam ausentes.

## P1140.15 — margens de página por lado

Medição: numa página de 180 pt com `left: 12pt`, `right: 38pt`, `top: 10pt`
e `bottom: 10pt`, o conteúdo RTL cristalino termina em 168 pt (`180 − 12`),
enquanto o vanilla termina em 142 pt (`180 − 38`). `PageConfig::margin: f64`
perde a distinção antes do layout.

Adicionar `PageMarginSpec`, com campos públicos
`left/right/top/bottom: Option<f64>`, e `PageMargins`, com os mesmos quatro
campos públicos resolvidos como `f64`. `None` num lado da especificação
significa `auto` para esse lado; `PageMargins` é o snapshot físico resolvido.

`PageConfig.margin` passa de `f64` para `PageMargins`. `margin_is_auto: bool` é
removido: auto existe por lado na especificação aplicada. O default resolve os
quatro lados com `auto_margin()`; margem uniforme produz quatro valores iguais.
Nenhum alias escalar escolhe um lado silenciosamente.

## Módulo
`01_core/src/entities/layout_types.rs`

## Propósito
Tipos de dados de layout: coordenadas, frames, documento paginado.

## P1227 — transporte de paint de formas até aos exporters

### Medição anterior à decisão

`01_core/src/entities/layout_types.rs:435` declara
`FrameItem::Shape.fill: Option<Color>`, enquanto
`01_core/src/compiler/layout/shape.rs:129` recebe `Paint` da forma mas chama
`to_color()`. Assim, Gradient e Tiling perdem tipo, stops, geometria e repetição
antes de L3. O stroke já transporta `Stroke.paint: Paint` sem esta perda.

### Mudança de contrato aprovada

Alterar `FrameItem::Shape.fill` de `Option<Color>` para `Option<Paint>`. Este
campo é o contrato público L1→L3. O dono confirmou o gate ADR-0127 em
2026-08-26. Todos os constructors, visitors e transforms preservam o valor sem
o reduzir a cor; exporters sem suporte nativo podem aplicar o fallback próprio
já especificado, mas recebem o `Paint` íntegro.

Conic continua transportado, mas o SVG não pode alegar equivalência nativa sem
orçamento de erro validado. Espaços de cor sem conversão morfológica modelada
também não podem ser promovidos silenciosamente a equivalência.

## P1140.5-A — grupo semântico de fórmula

### Medição antes da decisão

O vanilla default gerou PDF `Tagged: yes`, `/StructElem /S /Formula` e
`/Alt(accessible equation)`; `--no-pdf-tags` removeu a structure tree. O PDF
cristalino medido gerou `Tagged: no`. `FrameItem` atual possui apenas desenho,
Group geométrico e Link (`layout_types.rs:303-442`), sem unidade semântica.

### Decisão

Adicionar variante fechada `FrameItem::Semantic` com `SemanticKind::Formula`,
`SemanticPlacement::{Inline, Block}`, `alt: Option<EcoString>` e `items` filhos.
Ela não possui desenho próprio: bounds/plain text/visitors/shaping/export visual
descem nos filhos. String vazia permanece `Some("")`; ausência e `none` são
`None`. A variante é contrato L1→L3 e força revisão exaustiva dos consumidores.

P1140.5 prova transporte até a fronteira de exportação. A geração de MCIDs,
ParentTree e StructTreeRoot pertence ao P1140.6; não fingir tagging por
`/ActualText` ou por um comentário no content stream.
Puramente declarativos — sem I/O, sem métricas de fonte.

### P1140.12 — barreira semântica de quebra explícita

Adicionar `SemanticKind::ExplicitLinebreakBoundary`. O envelope é visualmente
transparente, sem alt e contém apenas um filho Text vazio posicionado na
baseline da linha fechada. Sua única semântica é impedir que pós-processadores
fundam novamente linhas separadas por `linebreak`; não representa fórmula,
não produz texto e não desenha.

A variante é contrato público L1→L3. Posição y e espaço restante não
distinguem wrapping automático de quebra explícita depois que o Frame perdeu
a causa do flush. Não substituir o marcador por limiar empírico.

### P1140.13 — fronteira semântica de parágrafo

Medição prévia: `Content::Parbreak` altera apenas a geometria vertical em
`compiler/layout/mod.rs`; depois de fechado o `PagedDocument`, L3 não consegue
distinguir essa causa sem comparar distâncias entre baselines. A aproximação
vigente (`1.5 × altura`) não é semântica e pode confundir parágrafos com
wrapping automático.

Adicionar `SemanticKind::ParbreakBoundary`. O envelope é visualmente
transparente, tem `SemanticPlacement::Block`, contém um filho `Text` vazio na
baseline da linha que termina e preserva até L3 a causa estrutural da quebra.
Ele não representa texto, glifo, espaço, altura ou conteúdo acessível.

`ExplicitLinebreakBoundary` e `ParbreakBoundary` são variantes distintas: a
primeira fecha uma linha dentro do mesmo parágrafo; a segunda fecha o próprio
parágrafo. Consumers que só precisam impedir reflow podem tratá-las como a
mesma classe de barreira, sem apagar a distinção semântica do contrato.

## Divergência do original
- **Abs** original: `Abs(Scalar)` com unidades raw internas e conversões pt/mm/cm.
  Cristalino usa `Pt(f64)` — mais simples, suficiente para o Passo 19.
- **Frame** original: `items: Arc<LazyHash<Vec<(Point, FrameItem)>>>` — posição na tupla.
  Cristalino embute `pos: Point` em `FrameItem::Text` — mais simples.
- **PagedDocument** original: `EcoVec<Page>` + `DocumentInfo` + `Arc<PagedIntrospector>`.
  Cristalino usa `Vec<Frame>` — stub até Passo 20+.

## Tipos

### `Pt`
Newtype f64 para pontos tipográficos. `Pt + Pt` OK; `Pt + f64` NÃO implementado.

### `Point`, `Size`
Coordenada 2D e tamanho 2D em `Pt`.

### `FrameItem`
Variantes:
- `Text { pos, text, style }`
- `Line { start, end, thickness, color, cap }`
- `Glyph { pos, glyph_id, x_advance, size }`
- `Image { pos, data, width, height, intrinsic_width, intrinsic_height }`
- `Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit }` (Passo 76)
- `Group { pos, matrix, clip_mask, inner_width, inner_height, items }` (Passo 78)
- `Link { target, items, pos, size }` (P422/P424/P463)
- `LinkTarget::Url(EcoString) | LinkTarget::Destination(Label)` (P463)

**`Line.color: Option<Color>`** (Passo 285) — `Some(c)` emite `r g b RG`
antes do stroke (`S`) no PDF; `None` preserva default preto bit-exact
(backward-compat para frac/sqrt overline/linhas geométricas pré-P285).
Activa o `stroke` parseado em `Content::Underline`/`Strike`/`Overline`
(P284 §5.4 — pendência registada e agora resolvida). Regra de herança
no consumer Layouter: `color = stroke.or(self.style.fill)` (utilizador
explícito > herança do texto corrente > default preto).

### P1431 — `FrameItem::Line.cap` para augment de matriz

Medição anterior à decisão: `FrameItem::Line` transporta apenas posição,
espessura e cor no consumer produtivo (`01_core/src/entities/layout_types.rs:
395-402`), embora `Stroke` já possua `LineCap` (`entities/geometry.rs:10-15,
75-86`). A fonte vanilla pinada conserva o `FixedStroke` completo nas linhas
de augment e fixa `LineCap::Square` no stroke automático
(`typst-layout/src/math/table.rs:36-53`).

Adicionar `cap: LineCap` a `FrameItem::Line`. O campo é parte do contrato
L1→L3 e deve sobreviver a translação, slicing, grupos e reconstruções sem ser
recalculado. Todos os produtores legados de `Line` usam `LineCap::Butt`,
preservando o comportamento anterior; somente consumidores com intenção
explícita diferente escolhem outro valor. P1431 usa `Square` no augment auto e
propaga o cap do stroke customizado.

O escopo semântico deste refino inclui somente raster e PDF pelos L0s
proprietários P1431. SVG e demais targets não ganham promessa de cap neste
passo; adaptações necessárias para exaustividade compilável devem ignorar o
novo campo sem alterar a saída.

`Image`: representa uma imagem a renderizar na página.
- `pos`: canto superior esquerdo em coordenadas de página (pt).
- `data: Arc<Vec<u8>>`: bytes raw da imagem — zero-copy via Arc.
- `width`, `height`: dimensões físicas no documento (pt) — tamanho de layout.
- `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis — obrigatórias
  para o dicionário XObject PDF (/Width, /Height intrínsecos ≠ tamanho de layout).
- `orientation`: valor EXIF `Orientation` (1-8) lido do cabeçalho. O exportador
  PDF usa este valor para compor a matriz `cm` de transformação (P776).

`Link` (P422/P424/P463): hiperligação. O `body` é renderizado normalmente e os
itens resultantes ficam em `items`. O destino é preservado como metadado
(`LinkTarget`) para o exportador PDF:
- `LinkTarget::Url` → annotation `/Subtype /Link` com `/A << /S /URI >>`;
- `LinkTarget::Destination(Label)` → annotation `/Subtype /Link` com
  `/A << /S /GoTo /D /name >>` (navegação interna para `/Dests`).
`pos` e `size` definem a bounding box da área clicável. Cor/sublinhado
são scope-out (aguardam `FrameItem::Decoration`).

`plain_text()` ignora `Image`, `Line`, `Glyph`, `Shape`, `Group` e desce
recursivamente em `Link` — retorna apenas texto.

### `Frame`
Canvas de uma página. `plain_text()` para verificação em testes.

### `PagedDocument`
Resultado de `layout()`. `plain_text()` concatena páginas com `"\n"`.
Campos de labels (P460):
- `extracted_label_pages: HashMap<Label, usize>` — mapa label → página (1-based),
  gerado por `layout_labelled` durante a passagem.
- `extracted_label_positions: HashMap<Label, Point>` — mapa label → posição (x, y)
  no momento da inserção, usado pelo exportador PDF para `/Dests`.
**P488** — campos de página para LoF/LoT (fixpoint carry-forward):
- `extracted_figure_page_numbers: Vec<usize>` — páginas de figuras contadas, em ordem de documento.
- `extracted_table_page_numbers: Vec<usize>` — páginas de tabelas contadas, em ordem de documento.

**P535** — headings para bookmarks PDF:
- `extracted_headings: Vec<(Label, Option<String>, Content, usize)>` — cópia
  de `Introspector::headings_for_toc()` no pipeline, usada pelo exportador para
  construir a árvore `/Outlines`. Cada tuplo é `(auto-label, número, body, level)`.

**P536** — metadados do documento (`/Info`):
- `document_info: DocumentInfo` — metadados definidos por `#set document(...)`.
  Copiado do `Module` pelo pipeline antes da exportação PDF.

Todos inicializados vazios em `new()` e populados por `Layouter::finish()` ou
pelo pipeline pós-layout — sem alterar a assinatura de `layout()`.

## `PageConfig`

Configuração da página activa durante o layout. Campos principais:
- `width`, `height`: dimensões em pt.
- `margin`: margem uniforme em pt.
- `numbering`: padrão de numeração automática (P532).
- `columns`: número de colunas activas (P537b).

### Margem automática por omissão

A margem por omissão **não é um valor fixo**. Segue o vanilla 0.15.0:

```text
margin = min(width, height) * (2.5 / 21)
```

Equivalentemente, `≈ 11.90476 %` da menor dimensão da página. Para uma
página A4 (`595.28 pt × 841.89 pt`), isto produz `70.87 pt` (`≈ 2.5 cm`).
Para uma página pequena (`height: 200 pt`, `width: 595.28 pt`), produz
`≈ 23.81 pt`.

> **Fonte de paridade (P1031)** — a fórmula é **citação literal**, e em duas camadas do
> vanilla ratificado (`e0e8ca4d`):
>
> - **Documentação da linguagem** — `crates/typst-library/src/layout/page.rs:127-131`, doc
>   comment do campo `margin`, publicado em
>   `typst.app/docs/reference/layout/page/#parameters-margin`:
>   *"The page's margins. - `{auto}`: The margins are set automatically to **2.5/21 times
>   the smaller dimension of the page**. This results in **2.5 cm margins for an A4 page**."*
>   Isto sustenta tanto a fórmula como o valor A4 de `≈ 2.5 cm` citados acima.
> - **Implementação** — `crates/typst-layout/src/pages/run.rs:121`:
>   `let default = Rel::<Length>::from((2.5 / 21.0) * min);`
>
> **Natureza**: literal. Note-se que a fórmula está **na documentação publicada**, não só no
> código — é regra de linguagem, não detalhe de implementação, o que reforça que
> `margin_is_auto` tem de reproduzir o recálculo e não fixar `70.87`.

`PageConfig` distingue duas situações através do campo `margin_is_auto`:

- `margin_is_auto: true` — margem calculada automaticamente. O default é
  `true`. Sempre que `SetPage` alterar `width` ou `height` sem fornecer um
  valor explícito de `margin`, a margem deve ser recalculada pela fórmula
  acima (`auto_margin()`).
- `margin_is_auto: false` — margem fixa definida pelo utilizador. Não é
  recalculada quando `width`/`height` mudam.

`PageConfig::default()` deve calcular a margem a partir de `width` e
`height`, não hard-codificar `70.87`. Quando `SetPage` recebe `margin`
explicitamente, `margin_is_auto` passa a `false`. Quando `margin` é
ausente, `margin_is_auto` não muda — o default é `true`, e uma margem
fixa pelo utilizador permanece fixa até novo `margin` explícito.

Sempre que `SetPage` altera `width` ou `height`, se `margin_is_auto` for
`true` a margem é recalculada pela fórmula; se for `false`, a margem
fixa é preservada.

## Critérios de verificação
- `Pt(10.0) + Pt(5.0) == Pt(15.0)`
- `Pt * f64` compila; `Pt + f64` não compila
- `Frame::plain_text()` junta texto dos FrameItem::Text com espaço
- `PagedDocument::plain_text()` concatena páginas com newline
- `PageConfig::default()` calcula margem como `min(width, height) * 2.5 / 21`

## P482 — `ShapedGlyph` e `FrameItem::TextShaped`

**P482** adiciona:

```rust
pub struct ShapedGlyph {
    pub glyph_id:  u16,
    pub x_advance: i32,
    pub x_offset:  i32,
    pub y_offset:  i32,
    pub cluster:   u32,
    pub char_code: char,
}
```

e variante:

```rust
FrameItem::TextShaped {
    pos:    Point,
    glyphs: Vec<ShapedGlyph>,
    style:  TextStyle,
    text:   EcoString,  // texto original (fallback + CMap)
}
```

`TextShaped` é produzido pelo shaper L3 (`03_infra/src/shaper.rs`) a
partir de `FrameItem::Text` com `style.font.is_some()`. `Text` é
preservado como fallback (Type1 / fonte não carregada).

`plain_text_items` trata `TextShaped` como `Text` — extrai `text` field.

Todos os match exaustivos de `FrameItem` em L1 devem incluir arm `TextShaped`.
Ver ADR-0120.

## P483 — FrameItem::Text deprecated

**Data:** 2026-06-28

`FrameItem::Text` marcado `#[deprecated(since = "P483")]`. Continua a existir
como fallback para fontes não carregadas ou Type1. Todos os sites de match
legítimos receberam `#[allow(deprecated)]` ou `#![allow(deprecated)]`.

`From<&StyleChain> for TextStyle` preenche agora `font` com pelo menos
`FontList("Helvetica")` quando nenhum `#set text(font:...)` está activo.
Garante cobertura ≥95% do shaping em produção.

Path primário em `export/stream.rs`: `TextShaped` antes de `Text`.

## P484 — FrameItem::Text como tipo pré-shaping (permanente)

**Data:** 2026-06-28

`FrameItem::Text` **não removido**. A remoção exigiria migrar os emit sites de
L1 (`cursor.rs`, `list_item.rs`, etc.) para emitir `FrameItem::TextShaped`
directamente — impossível sem bytes de fonte (ADR-0029). `FrameItem::Text` é
o tipo de **pré-shaping**: emitido por L1, convertido para `TextShaped` pelo
shaper L3. O `#[deprecated]` (P483) sinaliza que L3/export não deve usar
`Text` directamente; não indica remoção iminente.

## P485 — `units_per_em: u16` em `FrameItem::TextShaped`

**Data:** 2026-06-28

`FrameItem::TextShaped` recebe campo adicional `units_per_em: u16`:

```rust
TextShaped {
    pos:          Point,
    glyphs:       Vec<ShapedGlyph>,
    style:        TextStyle,
    text:         EcoString,
    units_per_em: u16,   // ← P485
}
```

Populado por `shaper.rs::try_shape` via `rb_face.units_per_em().max(1) as u16`.
Usado em `emit_shaped_pdf` para calcular números TJ: `-(x_advance / upm * 1000)`.
Sites de match que não precisam de `units_per_em` usam `..` (wildcard).

## P784 — `TextStyle.math: bool`

**Data:** 2026-07-17

Novo campo `pub math: bool` em `TextStyle` (default `false`, via `#[derive(Default)]`).
`true` sse este texto é conteúdo matemático. Definido **uma única vez**, em
`layout/equation.rs::layout_equation` (`TextStyle { math: true, ..self.style.clone() }`,
por cima do estilo herdado antes de chamar `MathLayouter::layout_equation`) — herdado
daí em diante por toda a árvore de layout math via `..style.clone()` (os sub-layouts
de math, `layout_text_node`/`attach.rs`/`frac.rs`/`root.rs`, nunca tocam `.font`
nem `.math`, só `.italic`/`.size`).

**Motivo**: `03_infra/src/shaper.rs` decide se engata a cadeia de fallback específica
de matemática (`fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH`) como primárias
adicionais. A condição original (P783, só `primary_has_math` — a fonte já resolvida
tem tabela MATH OpenType própria) nunca disparava no caso comum: a fonte de corpo
por omissão (`Libertinus Serif`) não tem tabela MATH. `style.math` dá ao shaper um
sinal directo e correcto de "isto é matemática", independente de qualquer propriedade
da fonte primária resolvida. Ver `infra/shaper.md` §P784 para a lógica de consumo.

Dois sites de construção não-spread de `TextStyle` (fora do `..style.clone()` normal)
precisaram de valor explícito: `entities/style_chain.rs::From<&StyleChain>` (`false`
— `StyleChain` não carrega contexto math) e `compiler/layout/text.rs` (herda de
`layouter.style.math`, merge de `#set text(...)` não é math-específico).

## P836 — campo `TextStyle::variations`

`TextStyle` ganha o campo
`pub variations: Option<crate::entities::font_variations::FontVariations>`
(default `None`), propagado do resolver `StyleChain::variations()` no
`From<&StyleChain> for TextStyle` e do merge de `layout/text.rs`
(canal custom `"text.variations"` + herdado do `layouter.style`).

Transporta as coordenadas de eixo explícitas de `#text(variations:)` /
`#set text(variations:)` até L3 (shaper, métricas, export), onde são
fundidas com os eixos derivados de `FontVariant` — ver
`infra/font_variant.md` (P836).

## P891 — campo `TextStyle::math_script: bool`

**Data:** 2026-07-24

Novo campo `pub math_script: bool` em `TextStyle` (default `false`, via
`#[derive(Default)]`, mesmo padrão de P784). `true` sse todo o conteúdo desta
chamada de layout está dentro de um script (sub/super-índice) de `MathAttach`
— definido **uma única vez**, em `attach.rs` ao construir `script_style`
(`TextStyle { math_script: true, size: style.size *
self.constants.script_percent_scale_down, ..style.clone() }`).

**Motivo**: `compiler/math/layout/spacing.rs::compute_gaps` precisa de saber se
a sequência inteira está em script size para suprimir `spacing_between`
(paridade vanilla `process.rs::spacing()`, condição "unless in script size" —
ver `math/layout/spacing.md` §P891). O cristalino processa uma sequência
inteira com um único `TextStyle` partilhado (ao contrário do `MathSize`
discreto por item do vanilla) — este campo é a adaptação mínima que carrega
esse sinal até `compute_gaps` sem introduzir um enum de tamanho discreto.

Dois sites de construção não-spread de `TextStyle` (fora do `..style.clone()`
normal) precisam de valor explícito, mesmos dois sites identificados em P784:
`entities/style_chain.rs::From<&StyleChain>` (`false` — `StyleChain` não
carrega contexto math/script) e `compiler/layout/text.rs` (herda de
`layouter.style.math_script`, merge de `#set text(...)` não é
script-específico).

## P915 — campo `TextStyle::cramped: bool`

**Data:** 2026-07-26

Novo campo `pub cramped: bool` em `TextStyle` (default `false`, via
`#[derive(Default)]`, mesmo padrão exacto de P891/`math_script`). `true` sse
o conteúdo desta chamada de layout está num contexto tipográfico "cramped"
(TeXbook/OpenType MATH — ver `scripts.rs:318-382` do vanilla, lido em
`typst-passo-915-relatorio.md` Fase A). Forçado a `true` em **4 pontos**
(mapeados por leitura do vanilla, não por inventário assumido):

1. **`attach.rs`** — estilo do subscrito (posições bottom-left/bottom-right,
   `bl`/`sub`), nunca o do superscrito (`tl`/`sup`, que herda `cramped` do
   estilo ambiente sem forçar). Ver `compiler/math/layout/attach.md` §P915.
2. **`frac.rs`** — estilo do denominador, nunca o do numerador (que herda).
   Ver `compiler/math/layout/frac.md` §P915.
3. **`root.rs`** — estilo do radicando e do índice (`root(n, x)`). Ver
   `compiler/math/layout/root.md` §P915.
4. **`accent.rs`** — estilo da base, incondicional (o cristalino só implementa
   accent "acima" — `MathAccentElem` não tem campo de posição — logo a
   condição do vanilla "só se accent.is_bottom()==false" é trivialmente
   sempre verdadeira aqui). Ver `compiler/math/layout/accent.md` §P915.

**Consumido** em `attach.rs::compute_script_shifts` — lê `cramped` do
`TextStyle` **ambiente** passado a `layout_attach` (não do `script_style`
interno), para decidir entre `superscript_shift_up`/`superscript_shift_up_
cramped` (`entities/math_constants.md` §P915) — só quando há superscrito
presente (`tl`/`sup`); nada mais na fórmula de `compute_script_shifts` muda.

**Achado de âmbito, não corrigido**: o vanilla também aplica cramped à base
de `overline()` (não `underline()`) — o cristalino não tem `overline()`/
`underline()` como construção matemática (esses nomes resolvem para
decoração de texto, `DecoKind::Overline`, via `stdlib/text.rs`, não para
`Content::MathUnderover`) — não há call site a tocar. Registado, não
inventado.

## P906 — `FrameItem::Glyph` ganha `style: TextStyle` + `base_char: char`

**Contexto**: mecanismo de esticamento horizontal de glifo (`compiler/layout.md`
§P906) — mas o achado abaixo é mais fundo, e partilhado com o eixo vertical
já existente (`layout_stretchy_delimiter`/`layout_assembly`).

**Achado, em cadeia, cada camada só visível depois de corrigir a anterior**:
`FrameItem::Glyph` (usado sempre que um glifo de variante/assembly
matemático não tem mapeamento Unicode de volta a char) só tinha
`{ pos, glyph_id, x_advance, size }` — nenhum campo de identidade de fonte
nem de carácter original. Consequência, confirmada por medição directa em
PDF real (`pdftotext -bbox`, `mutool`), não por inferência:

1. **Exportador não sabia que fonte usar** (`export/stream.md` §P906) —
   corrigido com `style` (usado para resolver via
   `FallbackFontMetrics::resolve_font_combo`).
2. **Selecção de QUAIS fontes embutir no PDF** (`infra/pipeline.md` §P906)
   é feita cedo, a partir de caracteres vistos em `Text`/`TextShaped` — cega
   a `Glyph` (que não carrega nenhum char). Corrigido com `base_char: char`
   (o carácter ORIGINAL pedido ao esticamento, ex. `⎵`/`⏟` — não o
   `glyph_id` resultante, que só é interpretável dentro da fonte de onde
   veio, informação perdida ao sair de L1, que trata `FontMetrics` como
   opaco).

**Não é redundante**: `style` sozinho (sem `base_char`) não chega para (2) —
`style.font` num `FrameItem::Glyph` nunca é reescrito para a fonte
efectivamente resolvida (ao contrário de `TextShaped.style.font`, reescrito
por `shaper.rs` durante o shaping) — só `base_char` permite re-resolver a
fonte candidata correcta no ponto de selecção. `base_char` sozinho (sem
`style`) não chega para (1) — a resolução de candidata depende de
`style.math`/`style.size`/variante, não só do char.

**Construção**: 4 sites em L1 (`layout_stretchy_delimiter`/
`layout_stretchy_glyph_horizontal` em `stretchy.rs`, `layout_assembly`/
`layout_assembly_horizontal` em `assembly.rs`) — todos já tinham `c: char`/
`style: &TextStyle` em scope, custo de fio zero. 6 sites de reconstrução
(`offset_item`/`translate_frame_item`-like em `equation.rs`, `cursor.rs`,
`slicing.rs`, `footnote_flush.rs`, `helpers.rs`, `math/layout/mod.rs`) só
propagam os campos inalterados.

## P945 — campo `TextStyle::math_size: MathSize` (nível discreto do vanilla)

**Data:** 2026-08-01

**Medição que motiva** (`typst-passo-945` Fase A): `$ mat(1,2,3;4,5,6;7,8,9) $`
em equação de **bloco** (`Display`) — o vanilla compõe as células a **11pt**
(denominator de `Display` = `Text`, factor 1.0 —
`lab/typst-original/crates/typst-library/src/math/style.rs:343-363`); o
cristalino compunha a `11pt × script_percent_scale_down = 7.7pt` (P923
implementou a descida de nível como ×0.7 incondicional — correcto só para
`Text→Script`, errado para `Display→Text`). Grelha ~30% mais curta → alvo do
delimitador curto → assembly com peças a menos (3 vs 4 glifos por lado,
`mutool trace`).

**Decisão**: novo enum e campo, mesmo padrão de P784/P891/P915:

```rust
pub enum MathSize { Display, Text, Script, ScriptScript }
// em TextStyle:
pub math_size: MathSize  // default: Text (via Default)
```

Semântica: o **nível MathSize discreto do vanilla** em vigor nesta chamada de
layout (equivalente a `EquationElem::size` na chain do vanilla). Os factores
de escala continuam a vir de `MathConstants`
(`script_percent_scale_down`/`script_script_percent_scale_down`, absolutos ao
tamanho base) — o campo só regista *em que nível* estamos, para que as
descidas de nível saibam o factor correcto:

| Transição (vanilla) | Factor sobre o tamanho corrente |
|---|---|
| `Display → Text` | ×1.0 |
| `Text → Script` | ×`script_percent_scale_down` |
| `Script → ScriptScript` | ×`sscript / script` (≈0.5/0.7) |
| `ScriptScript → ScriptScript` | ×1.0 |

**Ponto de entrada**: `compiler/layout/equation.rs::layout_equation` fixa
`math_size: Display` (bloco) / `Text` (inline) no `math_style` — paridade com
o vanilla (`EquationElem::size` = Display/Text conforme `block`,
`equation.rs:189-195`).

**Actualização nos pontos de descida** (sem mudar o factor de tamanho que
cada um já aplica hoje — só mantêm o campo honesto para consumidores
abaixo): `attach.rs` (scripts → desce um nível, Display/Text→Script,
Script/SScript→SScript), `frac.rs` (numerador/denominador → Display→Text,
Text→Script, …), `root.rs`, `underover.rs`, `matrix.rs`/`cases.rs` (estes
dois **também** corrigem o factor — ver `compiler/math/layout/matrix.md`
§P945).

## P968 — `faux_bold_stroke_pt`: limiar de intenção bold (weight ≥ 600), não "qualquer peso > 400"

**Data:** 2026-08-05

**Medição que motiva** (`typst-passo-968` Fase A; auditoria externa
2026-08-05 secção 8.3): no documento de 30 secções, **1541 de 2074 blocos
de texto (74.30%)** saem com `2 Tr` + `w` de 0.073pt/0.051pt; o vanilla usa
`0 Tr` em 100%. O documento tem **0% de negrito genuíno** (nenhum
`*...*`/`strong` — a única ocorrência de `*` no fonte é `x^*`, expoente).
Os dois `w` medidos batem exactamente com a fórmula de P139 aplicada a
**weight 450**: 50/300 × size × 0.04 = 0.073pt a 11pt e 0.051pt a 7.7pt
(script). A origem do 450: P944 fixa `weight: Some(450)` no `math_style` de
toda a equação (`compiler/layout/equation.rs`), replicando o show_set do
vanilla (`TextElem::weight = 450`,
`lab/typst-original/crates/typst-library/src/math/equation.rs:197`) —
**paridade de língua, correcta, fica**. A divergência é o gate de P139
disparar para ela: o vanilla **não tem** mecanismo de faux-bold em lado
nenhum (selecção de variante real via fontdb, sempre `0 Tr`).

**Decisão**: `faux_bold_stroke_pt` só produz stroke quando o weight
expressa **intenção de negrito** — limiar **weight ≥ 600** (fronteira
semibold/bold do OpenType). Abaixo de 600 o peso é um peso real da fonte
(450 Book do math, 500 medium, …), a satisfazer por selecção de variante,
nunca por contorno. A fórmula de P139 (`((w − 400)/300) × size × k`) fica
inalterada acima do limiar (700 → 0.44pt @ 11pt, como antes).

```rust
pub fn faux_bold_stroke_pt(&self, k: f64) -> f64 {
    let w = self.weight.unwrap_or(400);
    if w < 600 {
        return 0.0; // P968 — peso real da fonte não é intenção de negrito
    }
    ((w as f64 - 400.0) / 300.0) * self.size.val() * k
}
```

**O que não muda**: texto genuinamente negrito (weight 700) sem variante
bold real carregada continua a receber `2 Tr` + contorno — divergência de
mecânica consciente registada em P956 (a frente tipográfica de selecção de
variantes bold é pré-existente e fica como estava). Os pontos de emissão
(`stream.rs` Type1 e envelope verbose P956) não mudam — consomem o mesmo
helper.
## P1140.24 — running matter de página

`PageConfig` transporta `number_align`, `header`, `header_ascent`, `footer` e
`footer_descent` segundo `entities/page_running.md`. Os defaults são
center+bottom, Auto, 30%, Auto, 30%. Conteúdo marginal é composto em camada
visual separada do body.
## P1140.25 — snapshot de supplement

`PageConfig` guarda `PageSupplement`; `Page` guarda o `Content` resolvido por
página. O vetor validado mantém alinhamento 1:1 com pages e numberings. Ver
`entities/page_supplement.md`.

## P1157 — snapshots tipados de numbering

### Medição antes da decisão

O introspector vanilla preserva `Numbering`, e `loc.page-numbering()` devolveu
a função original. Logo `EcoString` no snapshot perde semântica.

### Decisão

`PageConfig.numbering` e `Page.numbering` passam a
`Option<entities::numbering::Numbering>`. São estado resolvido: `None` significa
desativado. `Page` também conserva o span de origem necessário para erro de
callback e o número lógico corrente validado; estes campos públicos exatos serão
materializados somente após o gate. Geometria e ordem de páginas não mudam.

## P1486 — número lógico da página para rótulos PDF (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

Na working tree não commitada sobre `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`
(proveniência, fixtures e extração: `lab/results/construction/diagnostics/typst-p1486-page-labels.md`),
o vanilla pinado rotula a seleção física 1+3 com `1, 3` sob padrão `"1"`
e com `10, 12` após `counter(page).update(10)`. O cristalino atual rotula
ambas como `1, 2`. Em `layout_types.rs:853-865`, `Page` preserva o
`numbering`, mas não o número lógico; em `pipeline.rs:1847-1856`, o
`PageStore` realizado possui os números lógicos, enquanto
`pipeline.rs:1558-1565` devolve apenas o documento. A inferência do Passo
1486 de que `Page.numbering` mais índice físico bastariam é refutada pelo
segundo probe. Rótulo visto pelo leitor é observável; a escolha de campo é
mecânica (ADR-0107). A inferência de que o store final é fonte suficiente
seria refutada por uma página cujo número lógico realizado difira do seu
snapshot após o último relayout.

### Contrato público proposto

`Page` ganha `pub logical_number: usize`, número lógico validado da própria
página, independente do índice físico e da seleção PDF. Construtores de
layout L1 iniciam-no pelo número físico quando ainda não há store realizado;
o pipeline L3 substitui esse valor pelo snapshot lógico do `PageStore` final,
em correspondência 1:1, antes de devolver o `PagedDocument` para exportação.
O campo não muda `plain_text`, canvas, conteúdo visual, contadores ou seleção.
Uma página construída diretamente para exportação declara explicitamente o
seu número lógico. O span de origem de P1157 permanece fora deste passo.

**Paragem obrigatória:** adicionar um campo público de entidade é mudança de
contrato pela ADR-0127. Nenhum código L1/L3 deve materializar este bloco
antes de confirmação específica do dono para P1486.
# P1226 — `FrameItem::Shape.fill_rule`

Medição pública: o frame cristalino perdia `curve(fill-rule: "even-odd")`
antes do export SVG. `fill_rule: FillRule` pertence ao variant público
`FrameItem::Shape`. Layout propaga o valor de `ShapeElem`; todos os emissores
existentes que não expõem regra usam `NonZero`. Exportadores consomem o campo,
sem derivá-lo da orientação dos segmentos.

## Vocabulário semântico de tabela

`SemanticKind` representa Table com summary, Row, HeaderCell com
level/scope/rowspan/colspan e DataCell com rowspan/colspan. O carrier também
representa THead e TBody e admite TR direta sob Table para linhas
explicitamente mistas.

Headers repetidos em páginas distintas preservam uma identidade lógica única,
com vínculos físicos por página destinados ao exporter. `level` é dado
relacional transitório e não atributo numérico de saída.

Todo carrier é visualmente transparente: bounds, plain text, visitors,
shaping, transforms e layers descem nos filhos. L1 não aloca object IDs ou
MCIDs e não escreve PDF.

## Proveniência `TextItem`

`TextStyle` contém:

```rust
pub math_text_item: bool
```

O default é `false`, inclusive nos construtores regulares. `true` é usado
somente quando `Content::Text` direto em contexto matemático corresponde à
morfologia TextItem. O estado válido é
`math=true && math_text_item=true`: `math` conserva família, fallback,
tamanho e shaping; o novo bit separa propriedades exclusivas da rota de glifo.

O campo não é configurável pelo utilizador, não entra em `StyleDelta`,
`StyleChain`, parsing ou eval. Ele é preservado em `FrameItem::Text`,
`TextShaped` e merges de `TextStyle`, sem criar campo em `FrameItem`.

MathIdent, MathText, números, GlyphFragment, formas extensíveis e
MathClassOverride promovido a glifo mantêm `math_text_item=false`.

## Efeito morfológico

Quando `math_text_item=true`:

- mantém `math=true` e todos os eixos não exclusivos de GlyphFragment;
- exclui a correção itálica de glifo singular;
- exclui a substituição OpenType `ssty` no shaping e no fallback métrico,
  mesmo em Script ou ScriptScript.

Quando `false`, IC e `.st/.sts` permanecem ativas nas rotas matemáticas que
as definem. Não criar outro campo, heurística nominal ou lista de caracteres.

## P1405 — label semântica de prefixo de outline

`SemanticKind` ganha `Label`, carrier fechado da função interna equivalente a
`PdfMarkerTag::Label` do vanila. É um envelope inline visualmente transparente:
preserva os items do prefixo e não cria texto, medida ou Location. Seu único
consumer produtivo é o pipeline PDF tagueado; não confundir com `LabelElem`
(destino nomeado) nem com `Artifact`, que remove conteúdo da structure tree.

## P1462 — opções de formato no documento paginado (MATERIALIZADO)

### Medição anterior à decisão

No baseline `7442d33b6bf48f136826d28cf71bd62f8710a432`,
`PagedDocument` transporta páginas, introspecção e `DocumentInfo`, mas nenhuma
opção de formato; por isso o exporter SVG só pode observar o booleano fornecido
externamente. O vanilla resolve as opções documentais antes do export.

### Contrato público

Acrescentar a `PagedDocument`:

```rust
pub format_options: DocumentFormatOptions
```

`PagedDocument::new` inicializa o campo com `Default`, preservando todos os
callers existentes e `pretty == false`. L3 substitui o default somente depois
de a extração pura do conteúdo final ter sucesso. O campo não contém flags
CLI, bytes serializados nem handles de exporter.

`DocumentFormatOptions` pertence exclusivamente a
`entities/document_format_options.rs`; este owner somente o transporta no
documento. `Page`, `PageConfig`, frames, geometria e plain text não mudam.
Trata-se de campo público. A mudança foi aprovada explicitamente e materializada
em 2026-09-19.

## P1466 — metadado de depreciação válido

O atributo de `FrameItem::Text` usa uma versão semver válida em `since`; a
mensagem e o status deprecated permanecem iguais. Esta correção é metadado
Rust para satisfazer análise estática e não remove a variante, não altera
fallback, layout, serialização ou API pública da linguagem.

## P1481 — carriers públicos de idioma e semântica PDF/UA (IMPLEMENTADO; confirmado em 2026-09-23)

### Medição anterior à decisão

Na fixture P1480 após P1480, veraPDF 1.30.2 `-f ua1` mede 102 regras
aprovadas e quatro falhadas; o vanilla pinado passa 106/0. O QDF vanilla
tem catálogo `/Lang (en)`, nós H1/P/Strong e spans com MCID; o
`PagedDocument` cristalino não transporta idioma documental, embora
`TextStyle.lang` transporte idioma local. Na fixture
`p1481-no-text-fr.typ` (SHA-256
`8e1b27cf49c91d2edbe132aa69f893bb554bca7141121fe87a544ed219455b38`),
o vanilla emite `/Lang (fr)` sem nenhum item de texto: inferir a língua
pelos glifos é refutado. `ImageElem` e `FrameItem::Image` perdem `alt`;
`StyleDelta.bold_from_strong/italic_from_emph` preserva a origem até
`TextStyle`, que hoje a descarta.

### Contrato público confirmado em 2026-09-23

- `PagedDocument` ganha `pub document_language: Option<Lang>`.
  `Some` é o primeiro idioma documental efetivo antes da emissão; `None`
  preserva ausência de regra e L3 aplica somente o default `en` medido no
  vanilla para PDF/UA. A escolha não depende de texto pintado nem de
  `dc:language` fixo. `PagedDocument::new` inicializa `None`.
- `TextStyle` ganha `pub semantic_strong: bool` e
  `pub semantic_emph: bool`, default `false`, derivados exclusivamente
  da proveniência `from_strong/from_emph` da chain. Não são sinônimos de
  `bold/italic` de render; mudar peso ou inclinação manualmente não cria
  Strong/Em.
- `FrameItem::Image` ganha `alt: Option<EcoString>`, com `None`
  distinto de `Some("")`; o valor atravessa layout/shaping sem afetar
  geometria ou bytes da imagem.

São campos públicos de L1 e alteram constructors/patterns existentes:
Gate ADR-0127 satisfeito pela confirmação humana em 2026-09-23. Sem novo modo,
trait, fase eval/layout ou dispatch dinâmico.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/entities/layout_types.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Own the approved TextSeparator data variant and TextSeparatorOrigin. Transport separators through the existing visitor text hook without adding a public trait method. Plain-text diagnostics retain the historical approximate join and skip carriers to avoid counting spaces twice; they are not lexical parity oracles.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

A restrição da primeira materialização às quebras automáticas é ampliada
somente para identidade de SHY autoral em runs já shaped. O texto original
é a causa da substituição por ocorrência. Filhos continuam TextShaped,
sem envelope aninhado, sem novo avanço ou candidato de quebra. A seleção
explícita de quebra SHY e o fallback sem fonte permanecem fora desta correção.

## Transporte de contexto bidi — contrato ratificado ADR-0127, 2026-10-05

### Medição anterior à decisão

Medição anterior à decisão: `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree integral, UTC, fontes e binários; treze entradas CLI com vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. rtl_auto_mixed põe ABC em x=124.020510 no Tekt contra152.949220 no vanilla; ltr_numeric põe árabe à esquerda e inverte 123/456, contra números à esquerda no vanilla. `layout_bidi.rs:73–82` usa primeiro caractere forte da linha e `:227–240` usa qualquer style.dir RTL como nível padrão; `shaper/bidi.rs:34` resolve cada string novamente com base automática. O contexto privado de paragraph.rs não chega a esses consumers. Vanilla `inline/mod.rs:189,232` resolve direção compartilhada; `inline/collect.rs:156–178` conserva embedding explícito por TextElem com direção diferente da base. `probe.json.gz` registra receita, rustc, hashes da biblioteca unicode-bidi 0.3.18 e sonda independente: dentro de ABC 123 456 789 مرحبا, linha 456 789 é um run LTR; a mesma string isolada com a mesma base RTL tem runs 789, espaço, 456. Isso refuta o transporte somente de Dir e a resolução por linha/palavra isolada. Não é prova de geometria de um candidato produtivo. Semântica/sintaxe/morfologia são obrigações; biblioteca, ranges, Arc e estratégia de cache são mecânica interna (ADR-0107).

### Contrato público ratificado

Adicionar exatamente este campo às variantes existentes `FrameItem::Text`, `FrameItem::TextSeparator` e `FrameItem::TextShaped`:

```rust
bidi: Option<crate::entities::bidi_context::BidiSpan>,
```

A entidade nova e sua validação pertencem exclusivamente a `00_nucleo/prompts/core/entities/bidi_context.md`. Não adicionar campo a TextStyle, variante a SemanticKind/FrameItem, método ao visitor ou trait FontMetrics. Preservar campos existentes, deprecação de Text e TextSeparatorOrigin. Produtores não certificados, matemática e frames manuais legados usam None; o default Typst/CLI e fases não mudam. O campo é contexto lógico opcional de uma ocorrência, sem tinta, largura, fonte, baseline, espaço ou conteúdo próprio. Debug/Rust patterns e struct literals de clientes precisam de migração: esta é mudança de contrato público, não compatibilidade Rust automática.

Todos os clones, translações, envelopes, visitors e diagnósticos conservam a opção sem extrair texto/controles dela. O shaper transporta o span quando substitui Text por TextShaped; splits fazem subspans validados. Não duplicar a lógica/validação da entidade neste módulo. Plain-text e PDF continuam extraindo o texto da folha e a origem existente do separador, nunca a entrada auxiliar de bidi. Nenhum código consumidor é autorizado por este bloco enquanto o pacote L0 não for confirmado.

Ratificação recebida: “confirmo”, manifesto 1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043, evidência em `lab/history/construction/records/00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`. Versões congeladas estão no receipt before.json.gz da implementação; alterações de Hash do Código não substituem a prova da aprovação.


## Face real de TextShaped — contrato ratificado ADR-0127

### Medição anterior à decisão

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/initial.json.gz`
conserva HEAD, working tree completa não commitada, UTC, fontes, binários e
owners antes desta proposta. `probe.rs`, `probe-build.txt` e `probe.txt`
reproduzem um catálogo controlado com DejaVu Sans regular e Oblique.
`03_infra/src/shaper/shaping.rs:254-255` carrega o slot efetivo para shaping;
`:358-362` conserva apenas sua família no estilo do sub-run.
`03_infra/src/pipeline/resources.rs:182-196,262-265` seleciona outra vez por
família/variante; `03_infra/src/export/stream/text.rs:22-43`,
`03_infra/src/export/render.rs:255-259` e
`03_infra/src/export/svg/glyphs.rs:299` usam a chave derivada do estilo.
A sonda emite os GIDs [5256,5260,5277,5288,5341] para مرحبا em ênfase,
iguais aos da face regular. O estilo ainda pede italic e seleciona Oblique;
a mesma entrada diretamente nessa face produz cinco GIDs zero e não cobre م.
`bilateral.json` conserva três controles fonte→PDF em cada compilador,
contra vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, sem inferir
correção visual apenas da extração. O raster previamente registrado em
`lab/results/construction/diagnostics/bidi-contexto-implementacao-2026-10-05/raster.json`
confirma tinta incompatível no Emph árabe.

O relatório `lab/results/construction/diagnostics/paridade-producao-p534.md:79-80`
descreve explicitamente transportar a família para embutir a face correta.
Isto prova a aproximação histórica documentada, não uma decisão consciente
de aceitar corrupção de GIDs. Identidade da face é mecânica de integridade
necessária à morfologia renderizada da língua; não exige mesmos bytes PDF,
mesma estrutura interna ou mesmas escolhas de fonte do vanilla em geral.
A hipótese seria refutada se a seleção de exportação já usasse a mesma face
que produziu os GIDs; a sonda controlada distingue as duas.

### Contrato público ratificado (materialização 2026-10-05)

Acrescentar **somente a TextShaped**, na enum FrameItem:

```rust
font: Option<crate::entities::world_types::Font>,
```

Some transporta um clone da fonte efetivamente usada para produzir glyphs e
units_per_em. Font já é uma entidade L1 existente, com armazenamento Arc;
este campo não cria mmap, lê arquivos, interpreta OpenType nem referencia
World. A fonte deve estar carregada antes de entrar no carrier. Não guardar
slot de catálogo, caminho de arquivo, nome de família ou ponteiro de World
como identidade persistente. O carrier continua válido depois da sessão e
em outro World, mantendo os mesmos bytes de origem. O caminho vigente trata
face normalizada no índice zero; suporte novo a coleção/índice não zero é
scope-out. Não acrescentar unsafe ou estado global.

None preserva itens legados/manuais e sua resolução anterior. Toda emissão
produtiva bem-sucedida do shaper fornece Some, incluindo cache hits e fontes
variáveis. Nenhum item Text/TextSeparator adquire esse campo nesta mudança;
Glyph matemático mantém seu contrato próprio e fica fora desta correção.

A fonte carregada define o domínio de glyph_id e units_per_em; eixos
OpenType continuam derivados e explícitos segundo o contrato vigente de
TextStyle. O shaper e a preparação de recursos aplicam as mesmas coordenadas
fundidas a essa face, com explícitas vencendo por tag. Não mudar italic,
bold, weight, semantic_emph ou semantic_strong para forçar seleção de outra
face. Também não aplicar o número de GID a uma fonte apenas porque seu nome
ou sua variante coincide. Instanciação/subset pertencem a L3 e devem manter
uma associação explícita à face original e aos eixos efetivos; bytes do
subset/instância não substituem a identidade da origem.

Clone, tradução, reconciliação, fragmentação e wrappers geométricos/semânticos
preservam o campo. Junção de itens shaped exige fonte e coordenadas
compatíveis, além dos critérios anteriores; igual família não basta. Não
inferir a fonte por char_code, cluster, GID ou cobertura posterior.

Os owners L3 próprios conduzem uma migração interna de bindings de recursos
por face original + eixos efetivos e transportam essa associação a PDF,
SVG e PNG. Coleta, subset, CMap, recurso PDF e consumo de outlines devem usar
a mesma associação. Preservar os entrypoints públicos existentes, com
adaptação interna compatível para os recursos legados; se essa adaptação
exigir nova assinatura pública, redigir seu L0 e submetê-la a novo gate antes
de implementá-la. Não alterar FontKey público por inércia nesta aprovação.
Apenas o novo campo acima é o contrato público ratificável neste pacote.

### Aceitação e limites da materialização

- RED→GREEN: catálogo controlado regular/italic, árabe em ênfase e latim em
  ênfase; conferir transporte da fonte e desenho com face correta, conservando
  os marcadores semânticos. Executar novamente após cache hit.
- Duas faces de mesma família e variante mas cobertura/GIDs diferentes não
  podem compartilhar binding. Dois Worlds com slots diferentes não podem
  alterar a fonte já transportada. None segue o fallback legado.
- Fonte variável: pedidos derivados/explicitamente sobrepostos, dois eixos
  diferentes no mesmo documento, miss/hit e instanciação preservam as
  coordenadas do shaping. Não provar isto só com uma fonte estática.
- Exercitar envelopes e camadas de página; bilateral PDF com inspeção de
  tinta, texto e geometria; SVG e PNG devem desenhar com a face efetiva.
  Latin regular/italic, árabe regular, strong, math e recursos ausentes são
  controles de regressão. Não copiar defeitos do vanilla.
- Build, testes pertinentes, V15/V26 e resselo restrito; registrar warnings
  preexistentes. Medição de CLI só depois da correção, com estado/fontes/
  binários e amostras pareadas, sem prometer ganho de desempenho.

Este owner continua 1:1 com `01_core/src/entities/layout_types.rs`.
Os owners internos recebem seus próprios L0 atualizados antes de código;
o inventário de candidatos não constitui ownership compartilhado.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Metadado do carrier permanente de pré-shaping — 2026-10-13

Medição anterior à decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado; fontes, UTC, status, diffstat e hashes em
`lab/results/construction/diagnostics/pre-shaping-ci-2026-10-13/before.json.gz`.
O inventário Clippy anterior desta árvore, cfg-test-ci-2026-10-13,
contém 89 diagnósticos deprecated no core, todos da variante Text e seus
campos. layout_types.rs:393-407 mantém a marcação P483; text.rs:37 emite
Text em L1 e shaper/traversal.rs converte-o em L3. Este L0 §P484 e a
ADR-0120 §P484 declaram Text permanente como entrada pré-shaping.
Os testes shaper/tests.rs:235,286,309,465 exigem fallback Text sem fonte;
:426,448 exercitam shaping multilinha com fonte real.

Retificar apenas o metadado de obsolescência legado de P483: remover
#[deprecated] da variante Text e atualizar sua documentação para seu papel
permanente. Text não pode ser substituído por TextShaped nos emissores L1,
pois os bytes da fonte pertencem a L3. Nenhuma variante, campo, assinatura,
valor, algoritmo, default ou fase muda. Shaping permanece em L3 e os
exporters conservam TextShaped e o fallback Text conforme seus owners.
A referência de P484 ao atributo deprecated fica superada por esta correção;
seu contrato de pré-shaping permanece obrigatório. A nota de P483 é histórica,
não uma obrigação de restaurar um warning em usos arquiteturalmente válidos.

Remover também os allow(deprecated) locais deste consumer que só compensavam
essa marcação. Não introduzir allow global, não alterar os consumers nem
migrar dados artificialmente para glyphs. Aceitação: uma prova independente
constrói/lê Text sob deny(deprecated), falha antes e compila/executa depois;
os corpos e dados Rust permanecem iguais descontando comentários e os
metadados explicitamente retirados. Revalidar testes core, fallback/shaping
com e sem fontes, build, Clippy, ownership/núcleos e selos.


## Operações numéricas puras — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/layout_types.rs` nas linhas 1096, 1097, 1113, 1114 contém as chamadas sinalizadas como disallowed_methods pelo Clippy vigente. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` conserva fonte, owner, configuração, libm, compilador, UTC, status e diffstat. A sonda host-math compara os backends antes da escolha: 4968 casos, 69 diferenças de até 1 ULP em trigonometria/atan2, sem mudança de classe numérica ou sinal de zero; potências e hypot coincidem na amostra. É medição deste host, não prova universal nem nova paridade contra vanilla.

### Decisão e aceitação

TransformMatrix::rotate usa libm::cos/sin; skew usa libm::tan. Preservar matriz Y-down, sinais, posição dos coeficientes, concatenação, assinaturas, translação zero e política vigente para entradas não finitas. Manter o backend puro libm 0.2.11 já pinado; não criar uma nova representação de transformação.

Mudança interna em fluxo contínuo ADR-0127, sem contrato público novo, default ou fase. Preservar testes de geometria/linguagem existentes e comparar os exemplos SVG medidos antes/depois, declarando diferenças. A comparação geométrica não certifica todos os casos ou plataformas. Validar build, todos os targets, core, Clippy e linhagem. Não suprimir o lint nem substituir aceitação por identidade de bytes do compilador.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Adicionar somente o campo público paint_root: bool em FrameItem::Group, conforme o contrato ratificado. True estabelece raiz local com inner_width/inner_height; false herda. FrameVisitor mantém assinatura e travessia. Os campos de Shape ainda serão migrados no próximo sublote T1c; não reivindicar transporte fechado de FramePaint neste estado.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Trocar somente Shape.fill por Option<FramePaint> e Shape.stroke por Option<FrameStroke>, conforme API aprovada. Não alterar outros variants, PageFill, TextStyle, FrameVisitor ou plain_text. Células de paint não são filhos semânticos da página.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F051: 01_core/src/entities/layout_types.rs:1392 (visit_item). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F051 — `visit_item`**: Remover somente o último braço _ com debug_assert e seu #[allow(unreachable_patterns)]: as onze variantes já são cobertas pelos braços anteriores. Conservar a assinatura pública de FrameVisitor, todos os callbacks, seus defaults, ordem e recursão. Atualizar a descrição do trait para exigir decisão em compilação quando o enum cresce, em vez de prometer fallback apenas em debug. Não adicionar método de trait. O novo controle percorre as onze variantes, incluindo wrappers e filhos, e verifica a sequência de callbacks antes/depois.
A prescrição desta rodada supersede apenas a política de fallback hipotético de FrameVisitor; as restrições históricas de não alterar o trait nos pacotes de transporte continuam para métodos, assinaturas e travessia. FrameItem mantém exatamente as onze variantes e todos os campos.


Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
