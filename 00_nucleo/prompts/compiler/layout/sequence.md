# Prompt L0 — layout de sequência
Hash do Código: 2ebc0f5a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/sequence.rs`

## Contrato e aceitação

Iterar Content::Sequence com lookahead, sticky e colapso de spacing, delegando
cada filho ao dispatcher sem perder ordem, boundary ou evento de paginação.

## P1374 — distribuição fracionária vertical proposta

### Medição anterior à decisão

O vanila coleta Blocks fracionários como itens de flow, desconta itens e gaps
fixos, soma os pesos e só na finalização atribui a cada ocorrência sua parcela
(`flow/collect.rs:244-264`; `flow/distribute.rs:589-681`). O Sequence
cristalino hoje despacha streaming e não possui carrier nem soma vertical de
frações.

### Decisão proposta para a primeira fatia

Em Sequence direta, região única e altura finita, executar preflight puro do
mesmo conjunto de siblings: conservar os gaps/itens fixos, excluir o corpo e
a altura própria dos Blocks fracionários, calcular espaço restante não
negativo e soma finita dos pesos. Cada ocorrência recebe
`remaining * weight / total_weight`; peso zero recebe zero e soma zero não
divide. A ordem dos fracionários não muda as parcelas. O valor é entregue ao
Block por estado interno, delimitado por save/restore e incluído em qualquer
checkpoint relevante; nunca é gravado na entidade.

O preflight não emite itens, Locations ou páginas e possui guarda contra
reentrada. Wrappers transparentes, colunas, multi-região e frações aninhadas
ficam para P1375, explicitamente `Unknown`. Nenhuma API pública do Layouter é
criada. A mudança do carrier público de Block exige gate ADR-0127.

## P1375 — transparência estrutural de estilo

### Medição anterior à decisão

Na sonda com `#text(fill: red)[#block(height: 1fr)[One]]`, seguida por um
`block(height: 2fr)` e texto fixo, o vanila ratificado distribui os dois pesos
numa página (`One=7,404`, `Two=82,724670`, `After=220,166`). O cristalino P1374
produz duas páginas (`One=7,404`, `Two=42,230`, `After=7,404` na página 2),
porque o preflight direto classifica `Content::Styled` como cauda fixa.

### Decisão interna

Para a distribuição P1374, um caminho composto apenas por `Content::Styled` e
por `Content::Sequence` com exatamente um filho substantivo é transparente.
`Space` e `Empty` não contam como filhos substantivos. Se esse caminho termina
num único `Block` de altura fracionária, o preflight usa o peso e os cosméticos
desse Block como faria para um sibling direto, e a parcela atribuída atravessa
o wrapper até ser consumida pelo Block. Uma Sequence com zero ou mais de um
filho substantivo não é achatada: evita associar uma parcela a conteúdo
ambíguo ou esconder distribuição aninhada.

O reconhecimento é iterativo e sem alocação proporcional ao conteúdo: apenas
desce referências existentes, sem consumir pilha por nível de wrapper. Não
amplia estado público, não muda eval nem a
fase do pipeline. Colunas, multi-região, Strong/Emph e distribuição fracionária
aninhada permanecem `Unknown` para passos posteriores.

## P1376 — região restante em colunas contínuas

### Medição anterior à decisão

Em `#columns(2)` contínuo no topo da página, o vanila posiciona os Blocks 1fr
e 2fr e a cauda em `7,404`, `82,724670` e `220,166`; o cristalino P1375 usa
altura intrínseca e produz `7,404`, `42,230` e `77,056`. A forma sintética
`#set page(columns: 2)` repete os mesmos valores. Com `Intro` antes do
contentor, o vanila desloca `One` para `27,842`, `Two` para `96,349990` e
mantém `After=220,166`, provando que a base fracionária é a região vertical
restante desde o cursor inicial da coluna.

### Decisão interna

O preflight fracionário é permitido durante `column_mode` somente quando
`page_columns.is_some()`, marcador do fluxo contínuo configurado por
`columns::layout_flow`. Nesse contexto, o espaço regional é
`max(0, page_bottom_limit - cursor_y)` no início da Sequence. Fora dele,
preserva-se a base P1374. O modo segmentado por `colbreak()`, onde
`page_columns` fica `None`, permanece fechado e `Unknown`.

A fórmula não cria carrier nem API pública. `grid`, `box` e `place` continuam
barrados pelos contextos de célula/subframe e não recebem `MATCH`; as sondas
P1376 registram essas lacunas conforme ADR-0124. O cálculo é constante, sem
recursão, buffers ou coleções adicionais.

## P1379 — região restante em segmentos explícitos de colunas

### Medição anterior à decisão

Depois do P1378, `#columns(2)[#block(height: 1fr)[One] After
#colbreak() #block(height: 1fr)[Two] Tail]` alinha corretamente `One` e
`Two` em `7,404`, mas o cristalino deixa `After=Tail=42,230`; o vanila usa
`220,166`. Com `Intro` anterior ao contentor, o vanila mantém ambas as caudas
em `220,166`, enquanto o cristalino usa `62,668`. Num segmento com Blocks de
`1fr` e `2fr`, o vanila põe o segundo corpo em `82,724670` e a cauda em
`220,166`; o cristalino usa `42,230` e `77,056`. A fonte ratificada entrega
ao flow de cada coluna uma `Region` finita repetida (`flow/compose.rs:230-249`),
logo a distribuição ocorre separadamente desde o cursor de entrada de cada
segmento até o fundo dessa região.

### Decisão interna

O motor de Sequence recebe uma base fracionária interna explícita: normal ou
restante-desde-o-cursor. A entrada normal preserva P1374–P1376. Uma função
interna dedicada recebe o conteúdo de um segmento, trata Sequence, singleton
e vazio sem reconstruir a árvore, e executa o mesmo iterador com a base
`max(0, page_bottom_limit - cursor_y)`. Somente essa entrada dedicada remove o
bloqueio `column_mode && page_columns.is_none()`; notas, subframes e demais
chamadores continuam fail-closed.

Cada segmento calcula independentemente itens fixos, gaps e soma de pesos.
Um `Content::Text` fixo direto anterior ao primeiro Block fracionário fecha a
linha antes do Block e aplica a diferença positiva `par.spacing - par.leading`
da fronteira parágrafo→bloco. No preflight, reserva todo o deslocamento entre o
topo de entrada e o início do Block: top-edge inicial quando pendente, avanço
que `flush_line` aplica (`top-edge + descent + leading`) e essa diferença de
fronteira. Medir apenas `vertical_metrics` reservava `13,2pt`; reservar só o
fechamento corrigia a paginação, mas mantinha o corpo em `21,792pt` contra
`27,842pt` no vanila. A fronteira acrescenta os `6,05pt` restantes em fonte
11pt e a parcela os desconta, preservando a cauda no fundo. A regra é limitada
a texto direto sem wrap; fluxo fixo composto ou com wrap permanece `Unknown`.
O restante cursor-relativo reserva uma guarda de arredondamento proporcional
à magnitude das coordenadas antes da divisão. Essa guarda é submilimétrica e
menor que a tolerância observável; impede que recompor parcela + fixos exceda
o fundo por um ULP e envie a cauda para a região seguinte.
Não há soma entre colunas, carrier novo, campo no `Layouter`, recursão ou
coleção adicional proporcional ao conteúdo. Frações aninhadas, overflow
multi-região, notas fracionárias e `grid`/`box`/`place` permanecem `Unknown`.

## P1380 — prefixo inline composto antes de fração segmentada

### Medição anterior à decisão

Nas mesmas colunas finitas de P1379, o vanila agrega como uma única linha o
prefixo que antecede o primeiro Block fracionário. Com `#text(fill: red)[Lead]`,
produz `Lead=7,404`, `One=27,842` e `After=220,166`; com `*Lead*`, produz
`Lead=7,261`, `One=27,699` e a mesma cauda; com `Lead` seguido por um fragmento
Styled `More` na mesma linha, mantém `Lead=More=7,404`, `One=27,842` e
`After=220,166`. O cristalino P1379 trata o wrapper como altura fixa opaca:
os dois primeiros casos paginam `After` para `7,404` da página seguinte; no
composto, deixa `One=21,792` e `After=201,576`.

A fonte vanila recolhe filhos inline juntos antes de produzir as linhas do
parágrafo (`flow/collect.rs:128-145`) e só então intercala Block e seu spacing
(`flow/collect.rs:240-286`). Portanto, cor, bold e a divisão em nós não criam
linhas fixas independentes.

### Decisão interna

Somente na entrada cursor-relativa de segmentos explícitos, o preflight pode
reconhecer um prefixo formado por `Text`, `Space`, `Empty` e por exatamente um
wrapper `Styled`, `Strong` ou `Emph` cujo corpo seja `Text`. O wrapper aceito
não altera tamanho, leading, top/bottom edge, tracking, fonte, script ou canal
custom; fill, bold, italic, weight e lang permanecem permitidos. A varredura é
iterativa sobre a fatia existente e usa apenas acumuladores escalares.

Se a largura somada exceder a largura disponível, se faltar texto real ou se
aparecer outro nó/wrapper, a otimização não recebe crédito e o caminho P1379 é
preservado como `Unknown`. Para um prefixo aceito e sem wrap, sua altura fixa é
contada uma única vez: top-edge do primeiro fragmento quando a baseline é
pendente, avanço único de `flush_line` e fronteira positiva
`par.spacing - par.leading`. Na execução, a mesma fronteira é aplicada uma vez
antes do primeiro Block fracionário. A parcela continua local ao segmento e
desconta essa altura agregada.

Não há novo estado no `Layouter`, API pública, recursão, árvore reconstruída ou
coleção proporcional. P1379 direto e demais contextos permanecem inalterados;
wrap, wrappers aninhados, variação vertical de estilo, frações aninhadas e
multi-região continuam `Unknown`.

## P1381 — tamanho de fonte no prefixo inline de uma linha

### Medição anterior à decisão

Nas colunas finitas de P1380, `#text(size: 18pt)[Lead]` deixa no vanila
`Lead=5,752`, `One=40,848`, `After=220,166` e `Tail=220,166`; o cristalino
P1380 usa `One=35,554` e pagina `After` para `7,404`. Com `size: 8pt`, o
vanila usa `Lead=8,112`, `One=25,868` e a mesma cauda no fundo; o cristalino
usa `One=17,868` e também pagina. No prefixo misto `Lead` +
`#text(size: 18pt)[More]`, o vanila mantém uma linha, com `Lead=12,010`,
`More=5,752`, `One=32,448` e `After=220,166`; o cristalino deixa
`One=30,948` e `After=202,752`.

`flush_line` escolhe para ascent/descent o maior tamanho textual da linha,
partindo do estilo exterior e substituindo-o somente por tamanho estritamente
maior; resolve o leading pelo último item textual. A baseline inicial, porém,
usa o top-edge do primeiro fragmento. Esses três escalares explicam as três
diferenças medidas sem confundir `yMin` de tinta com a baseline interna.

### Decisão interna

Na mesma entrada cursor-relativa e no mesmo prefixo sem wrap de P1380,
permitir que um único fragmento `Styled(Text)` varie somente `text.size` para
um tamanho menor ou igual ao exterior. A largura é medida no estilo resolvido.
O acumulador conserva o top-edge desse texto, o fallback exterior para a
altura da linha e o leading do texto. A fronteira usa o spacing exterior menos
esse leading, limitada a zero. Prefixos compostos continuam aceitos por P1380
somente quando nenhum fragmento muda de tamanho.

Não se cria coleção proporcional, recursão, simulação nem estado no
`Layouter`: há somente acumuladores escalares e no máximo clones constantes de
`TextStyle`. Um primeiro fragmento maior exige modelar o estilo compartilhado
do parágrafo e o colapso de seu spacing; uma linha mista exige ainda alinhar o
frame composto pela maior baseline. Ambas permanecem `Unknown`, assim como
`leading`, top/bottom edge, tracking, fonte, script, canal custom, wrapper
aninhado e wrap. P1374–P1380 e contextos não segmentados permanecem inalterados.

## P1383 — prefixo fracionário misto após baseline P1382

### Medição anterior à decisão

Com o P1382 validado, o prefixo `Lead #text(size: 18pt)[More]` já coincide com
o vanila na morfologia e nas posições de tinta (`Lead=12,010`, `More=5,752`),
mas o preflight fail-closed deixa `One=26,398` e `After=198,202`; o vanila usa
`One=32,448` e `After=220,166`. O resíduo do primeiro Block é exatamente
6,05pt, a fronteira exterior `par.spacing - par.leading` já especificada em
P1379. A candidata v2 documentada pelo P1381 atingiu `One`, `After` e `Tail`
do caso misto, mas foi rejeitada porque a linha ainda ficava 4,606pt alta;
P1382 eliminou precisamente essa causa independente.

Na ordem inversa, a baseline também coincide após P1382, porém o primeiro
fragmento maior continua envolvendo estilo compartilhado e spacing não
modelados. Um único fragmento maior ainda pagina. Esses dois casos permanecem
`Unknown`.

### Decisão interna

Além do fragmento único reduzido de P1381, o preflight aceita exatamente dois
fragmentos textuais quando o primeiro é texto direto no tamanho exterior e o
segundo é um único `Styled(Text)` cujo delta contém somente `text.size` maior
que o exterior. Espaços transparentes entre ambos não contam como fragmento.
A linha deve continuar sem wrap.

Nesse recorte, são reutilizados sem nova fórmula os escalares P1381: top-edge
inicial do primeiro texto e ascent/descent do maior estilo. Como P1382 faz o
Cursor resolver o avanço misto pelo estilo ativo exterior, o preflight resolve
por esse mesmo estilo tanto o leading quanto a fronteira
`max(0, par.spacing - leading)`. A soma reservada continua `par.spacing`, mas a
boundary entregue à execução coincide com o leading que o Cursor realmente
consome. Sequence apenas concede ao preflight o caso antes fail-closed.

Três ou mais fragmentos, primeiro fragmento não exterior, ordem inversa,
redução posterior, mais de um wrapper, delta adicional (inclusive fill), wrap,
leading, top/bottom edge, tracking, fonte, script e canal custom continuam
`Unknown`. Não há recursão, coleção proporcional, carrier ou API nova.

### Aceitação

O caso 11pt→18pt fica em uma página e coincide com o vanila em `Lead`, `More`,
`One`, `Two`, `After` e `Tail`. P1381 pequeno e P1380 permanecem exatos. Ordem
inversa, fragmento único maior e formas opacas continuam fail-closed.

## P1384 — ordem decrescente no prefixo fracionário misto

### Medição anterior à decisão

No segmento explícito com `#text(size: 18pt)[Lead] More` antes do primeiro
Block fracionário, vanila e cristalino P1383 já produzem uma página e a mesma
linha: `Lead=5,752` e `More=12,010` na caixa PDF, com baseline comum. O Block e
a cauda ainda divergem: vanila usa `One=32,448` e `After=Tail=220,166`, enquanto
o cristalino fail-closed usa `One=26,398`, `After=198,202` e `Tail=220,166`.

O Cursor P1382 torna a altura consumida independente da ordem desses dois
fragmentos. Se o exterior vem primeiro, fixa a baseline pelo top-edge exterior
e depois a desloca até o ascent maior; se o maior vem primeiro, fixa-a
diretamente pelo ascent maior. Nos dois casos, o avanço seguinte usa ascent e
leading exteriores. Portanto, a soma reservável continua sendo a mesma já
validada por P1383: ascent maior + próximo ascent exterior + descent maior +
leading exterior + fronteira exterior.

### Decisão interna

O preflight aceita também exatamente dois fragmentos quando o primeiro é um
único `Styled(Text)` cujo delta contém somente `text.size` maior que o exterior
e o segundo é `Text` direto no tamanho exterior. Espaços transparentes entre
ambos não contam como fragmento e a linha deve permanecer sem wrap.

Para essa ordem, a parcela inicial da fórmula é o top-edge exterior que o
Cursor usa para prever a próxima baseline, não uma segunda cópia do top-edge
maior já representado pelo máximo da linha. Leading e boundary continuam
formando o par exterior P1383. Não há constante nova nem simulação.

Fragmento único maior, dois wrappers, segundo tamanho não exterior, primeiro
delta adicional (inclusive fill), três ou mais fragmentos, wrap, leading,
top/bottom edge, tracking, fonte, script e canal custom continuam `Unknown`.
Não há recursão, coleção proporcional, carrier, API pública ou mudança de fase.

### Aceitação

O caso 18pt→11pt fica em uma página e coincide com o vanila em `Lead`, `More`,
`One`, `Two`, `After` e `Tail`. O caso crescente P1383, o fragmento único maior
e os controles P1374–P1382 preservam seus vereditos.

## P1385 — dois wrappers de tamanho em ordem decrescente

### Medição anterior à decisão

Em segmentos explícitos, quatro pares size-only foram medidos antes da
decisão. Para 18pt→8pt, 18pt→11pt e 18pt→14pt, o vanila mantém
`One=32,448`, `Two=7,404` e `After=Tail=220,166`; as linhas do cristalino P1384
já coincidem, mas o preflight opaco deixa `One=26,398` e a cauda respectivamente
em `216,758`, `213,338` e `209,918`. Em 16pt→8pt, o vanila usa
`One=31,132`, `Two=7,404` e a mesma cauda no fundo; o cristalino já coincide na
linha, mas usa `One=25,082` e `After=217,722`.

As quatro medições mostram que a posição do primeiro Block depende do máximo
da linha — o primeiro wrapper. O Cursor P1382 usa ascent e leading exteriores
para prever a linha seguinte sempre que os tamanhos textuais são distintos.
Na calibração, porém, conceder também 18pt→14pt deixou a cauda 6,05pt antes do
fundo, enquanto 18pt→8pt, 18pt→11pt e 16pt→8pt ficaram exatos. Logo um segundo
wrapper ainda maior que o exterior possui spacing não explicado por essas
duas invariantes e permanece opaco.

### Decisão interna

Além da forma P1384 com segundo texto direto, o preflight aceita exatamente
dois `Styled(Text)` quando ambos os deltas contêm somente `text.size`, o
primeiro tamanho é maior que o exterior e o segundo é menor ou igual ao
exterior. Espaços
transparentes não contam como fragmento e a linha deve permanecer sem wrap.

O máximo da linha continua sendo o primeiro estilo. A outra parcela de ascent,
o leading e a boundary permanecem exteriores, como em P1384; o tamanho do
segundo wrapper não entra novamente na altura fixa. Não há fórmula nova.

Segundo tamanho acima do exterior — mesmo abaixo do primeiro —, tamanhos
iguais, segundo maior que o primeiro, primeiro menor ou igual ao exterior,
delta adicional em qualquer wrapper, terceiro fragmento, wrapper aninhado,
wrap, leading, top/bottom edge, tracking, fonte, script e canal custom
continuam `Unknown`. Não há recursão, coleção proporcional, carrier, API
pública ou mudança de fase.

### Aceitação

Os pares 18pt→8pt, 18pt→11pt e 16pt→8pt ficam em uma página e coincidem com o vanila em
`Lead`, `More`, `One`, `Two`, `After` e `Tail`. P1384 direto, P1383 crescente,
o par 18pt→14pt opaco, o fragmento único maior e os controles P1374–P1382
preservam seus vereditos.

## P1386 — dois wrappers decrescentes ainda acima do exterior

### Medição anterior à decisão

Sob o cristalino P1385, quatro pares size-only estritamente decrescentes cujo
segundo tamanho ainda é maior que o exterior de 11pt preservam a mesma linha do
vanila, mas ficam fora do preflight. Em 18pt→12pt e 18pt→14pt, o vanila usa
`One=32,448` e `After=Tail=220,166`, enquanto o cristalino usa `One=26,398` e
respectivamente `After=212,198` e `209,918`. Em 20pt→14pt e 20pt→18pt, o
vanila usa `One=33,764` e a mesma cauda no fundo; o cristalino usa
`One=27,714` e respectivamente `After=208,954` e `204,394`.

Nos quatro casos, `Lead` e `More` têm baseline comum e o primeiro wrapper é o
máximo estrito. A candidata ampla P1385 já mostrou que reutilizar integralmente
a altura fixa do caso que retorna ao exterior deixa 18pt→14pt exatamente uma
boundary exterior de 6,05pt antes do fundo. Portanto, nesta faixa a boundary
continua sendo entregue ao runtime entre a linha e o Block, mas não é
duplicada na reserva fixa usada para repartir o espaço fracionário.

Uma sonda com métricas sintéticas rejeitou 20pt→14pt e 20pt→18pt no gate de
largura, mas a matriz produtiva final mostrou que ambos atravessam o gate real.
O observável de linguagem prevalece: os quatro pares são prova positiva; a
rejeição sintética apenas confirma que o gate continua conservador quando suas
próprias métricas não provam ausência de wrap.

### Decisão interna

O preflight aceita também exatamente dois `Styled(Text)` quando ambos os
deltas contêm somente `text.size`, o primeiro tamanho é maior que o segundo e
o segundo permanece estritamente maior que o exterior. Espaços transparentes
entre ambos não contam como fragmento e a linha deve permanecer sem wrap.

O máximo da linha continua sendo o primeiro estilo. Top-edge previsto para a
próxima baseline e leading continuam exteriores. A boundary exterior continua
no avanço real entre prefixo e Block, mas fica fora de `fixed_height` para esta
forma, porque incluí-la ali contaria duas vezes o mesmo observável. Não há
constante nova: o valor semântico da boundary continua vindo de
`paragraph_spacing - leading` e somente sua duplicação contábil é removida.
Três matrizes release mostraram que margens locais abaixo de 0,001pt na reserva
não mudam a decisão discreta: `One` já coincide, mas a baseline seguinte no
fundo é rejeitada pelo Cursor. Portanto `fixed_height` remove integralmente a
boundary duplicada; a tolerância de fitting pertence ao owner Cursor, não a
esta fórmula.

Tamanhos iguais, segundo maior que o primeiro, primeiro menor ou igual ao
exterior, delta adicional em qualquer wrapper, terceiro fragmento, wrapper
aninhado, wrap, leading, top/bottom edge, tracking, fonte, script e canal
custom continuam `Unknown`. O segundo menor ou igual ao exterior continua sob
P1385. Formas que o gate de largura existente não prova sem wrap também
continuam `Unknown`. Não há
recursão, coleção proporcional, carrier, API pública ou mudança de fase.

### Aceitação

Os pares 18pt→12pt, 18pt→14pt, 20pt→14pt e 20pt→18pt ficam em uma página e
coincidem com o vanila em `Lead`, `More`, `One`, `Two`, `After` e `Tail`.
P1385, formas realmente rejeitadas por largura, pares iguais/crescentes,
primeiro não maior, delta adicional e os controles P1374–P1384 preservam seus
vereditos.

## P1387 — wrappers uniformes acima do exterior

### Medição anterior à decisão

Cinco pares de wrappers size-only iguais foram medidos sob P1386. O vanila
mantém todos em uma página, com `After=Tail=220,166`. Para 12pt, 14pt, 16pt,
18pt e 20pt, `One` cresce respectivamente por `28,500`, `29,816`, `31,132`,
`32,448` e `33,764`. O cristalino preserva a baseline compartilhada de `Lead`
e `More`, mas produz `One=23,758/27,690/31,622/35,554/39,486` e deixa `After`
em `216,398/215,770/215,142/214,514/213,886`.

A progressão vanila é a do máximo textual, mas a linha seguinte continua
governada pelo estilo exterior. O Cursor P1382 só marca uma linha como mista
quando dois itens textuais diferem entre si; dois itens iguais a 18pt não são
mistos entre si, porém ambos diferem do estilo ativo exterior de 11pt.

### Decisão interna

O preflight aceita exatamente dois `Styled(Text)` size-only de tamanho igual e
estritamente maior que o exterior, quando o gate de largura prova ausência de
wrap. Eles usam a reserva P1386: máximo textual comum, top-edge e leading
exteriores, boundary entregue no runtime e não duplicada em `fixed_height`.

Cursor considera uma linha textualmente mista também quando algum item de
texto difere do estilo ativo do parágrafo, mesmo que todos os itens coincidam
entre si. Isso apenas seleciona a regra P1382 já existente para próximo ascent
e leading; baseline e estilo visual dos itens não mudam.

Tamanho igual ao exterior ou abaixo, ordem crescente, primeiro não maior,
delta adicional, terceiro fragmento, wrapper aninhado, wrap, leading,
top/bottom edge, tracking, fonte, script e canal custom continuam `Unknown`.
Formas que o gate de largura não prova — na matriz atual, 20→20 — também
permanecem `Unknown`. Não há recursão, coleção proporcional, carrier, API
pública ou mudança de fase.

### Aceitação

Os pares 12→12, 14→14, 16→16 e 18→18 ficam em uma página e coincidem com o
vanila em `Lead`, `More`, `One`, `Two`, `After` e `Tail`. 20→20 permanece
`Unknown` por rejeição no gate de largura e divergência de `One`. P1386
decrescente, P1385, ordem crescente e os controles P1374–P1384 preservam seus
vereditos.

## P1388 — boundary do prefixo após terminador de linha

### Medição anterior à decisão

No fixture P1387 de 200pt, cada coluna oferece 85pt. O par 20→20 permanece numa
baseline tanto no vanila quanto no cristalino, mas o vanila produz
`One=33,763990` e o cristalino `One=27,714`. A variação de largura parecia
apontar para a validação horizontal: o fallback persistia até 204pt e só
desaparecia em 205pt. Quatro candidatas nessa direção — reconstrução local do
estilo, supressão de checkpoint vazio, largura total/checkpoint e classificação
de whitespace-only — não alteraram o resultado produtivo e foram refutadas.

A redução diferencial localizou a causa real. Com a newline inicial preservada
e apenas a whitespace terminal entre `More` e o Block removida, `One` sobe para
o valor correto; removendo a newline inicial e preservando a whitespace
terminal, a divergência permanece. Na execução, esse terminador pode exceder a
margem e chamar `flush_line` antes de o iterador chegar ao Block. O preflight já
certificou o prefixo e devolveu sua boundary, mas `layout_parts` só a aplicava
se `current_line` ainda estivesse não vazia. Assim, fechar a linha cedo fazia o
handshake vertical desaparecer. Como controle, 21→21 quebra realmente no
vanila e não recebe distribuição validada.

### Decisão interna

A boundary devolvida por um `InlinePrefix { closes_line: true }` pertence ao
handshake validado entre o prefixo e o primeiro Block fracionário. Quando o
índice desse Block é alcançado, `layout_parts` fecha a linha somente se ela
ainda estiver aberta, mas aplica a boundary incondicionalmente uma vez. O
terminador que já chamou `flush_line` não consome essa boundary e, portanto,
não existe duplicação.

A regra só é alcançável quando o preflight produziu `distribution` e a boundary
correspondente. Fluxo ordinário, prefixo rejeitado, wrap real, delta adicional,
terceiro fragmento, wrapper aninhado e forma opaca não recebem deslocamento
novo. Não há tolerância horizontal, fallback nominal, recursão, coleção
proporcional, API pública ou mudança de fase.

### Aceitação

Com e sem terminador que fecha antecipadamente a linha, o mesmo prefixo
validado produz a mesma posição de `One`; 20→20 fica em uma página e
coincide com o vanila em `Lead`, `More`, `One`, `Two`, `After` e `Tail`, dentro
de 0,001pt. 21→21 continua com duas baselines e `Unknown`. P1387
12→12–18→18, P1386 decrescente, P1385 e os controles P1374–P1384 preservam
seus vereditos.

## P1389 — prefixo uniforme com um wrap entre fragmentos

### Medição anterior à decisão

Nos pares iguais 21→21, 22→22, 24→24, 30→30, 39→39 e 40→40, a soma dos dois
fragmentos ultrapassa a coluna de 85pt e o vanila quebra exatamente antes de
`More`. As posições `More` são respectivamente 26,012; 26,434; 27,278;
29,810; 33,608 e 34,030010pt, enquanto `After=Tail=220,166` em todos. O
cristalino P1388 preserva a morfologia de duas linhas, mas usa 32,512; 33,584;
35,728; 42,160; 51,808 e 52,880pt para `More`, e envia `After` à região
seguinte. A diferença cresce com o tamanho interno: cada linha singleton é
fechada pelas métricas do wrapper, embora o parágrafo exterior de 11pt governe
a baseline seguinte no vanila.

### Decisão interna

Além do prefixo sem wrap, o preflight aceita exatamente dois `Styled(Text)`
size-only iguais e estritamente maiores que o exterior quando cada body contém
uma única palavra não-whitespace, a primeira linha cabe, acrescentar o segundo
fragmento excede a largura e a reserva vertical completa ainda cabe na região.
Essa forma possui exatamente um wrap antes do segundo fragmento. Um word
overlong na segunda linha é permitido porque nasce na margem esquerda e não
cria um segundo wrap.

O carrier privado do preflight informa o índice do wrap e o índice/boundary do
Block. Na execução, Sequence fecha a primeira linha antes de entrar no segundo
wrapper e fecha a segunda antes de whitespace terminal ou do Block. Ambos os
fechamentos pedem ao Cursor o próximo ascent e o leading externos, preservando
o ajuste entre o ascent assumido exterior e o máximo da linha corrente. A
altura fixa é a reserva P1387 acrescida de máximo ascent, descent do texto e
leading exterior; a boundary continua separada e aplicada uma vez por P1388.

Texto com mais de uma palavra, tamanhos diferentes, delta adicional, mais de
dois fragmentos, wrap dentro de um fragmento, mais de um wrap, reserva que não
cabe, wrapper aninhado e forma opaca continuam `Unknown`. Não há simulação,
recursão, coleção proporcional, API pública ou mudança de fase.

### Aceitação

21→21, 22→22, 24→24 e 30→30 ficam numa página e coincidem com o vanila em
`Lead`, `More`, `One`, `Two`, `After` e `Tail`, dentro de 0,001pt. 20→20 sem
wrap e os controles P1374–P1388 permanecem exatos. Multiword e mais de um wrap
continuam `Unknown`.

## P1390 — wrap único entre wrappers de tamanhos desiguais

### Medição anterior à decisão

Nos pares size-only 21→24, 24→21, 22→30 e 30→22, ambos os motores quebram
exatamente antes do segundo fragmento. O vanila mantém uma página e
`After=Tail=220,166`; o cristalino P1389 preserva as duas linhas, mas pagina
`After`. As posições de `More` também divergem nas quatro células.

### Decisão interna

A validação de wrap único passa a aceitar também exatamente dois
`Styled(Text)` size-only desiguais e estritamente maiores que o estilo
exterior, com uma palavra não-whitespace por body, wrap somente na fronteira
entre eles e reserva vertical completa na região.

O carrier privado e os dois fechamentos exteriores de P1389 são reutilizados.
A altura fixa soma separadamente `top + descent + leading exterior` da
primeira e da segunda linha. Repetir o máximo global para ambas é proibido:
essa mecânica superestima uma das linhas quando os tamanhos diferem. A
boundary P1388 continua separada e aplicada uma vez.

Tamanho igual continua pertencendo a P1389. Tamanho igual ou abaixo do
exterior, texto multiword, delta composto, terceiro fragmento, wrapper
aninhado, wrap interno/múltiplo, forma opaca e reserva insuficiente continuam
`Unknown`. Não há API pública, simulação, recursão ou coleção proporcional.

### Aceitação

21→24, 24→21, 22→30 e 30→22 ficam em uma página e coincidem com o vanila em
`Lead`, `More`, `One`, `Two`, `After` e `Tail` dentro de 0,001pt. P1389 igual,
P1388 sem wrap e os controles P1374–P1387 conservam seus vereditos.

## P1391 — wrap único entre wrapper exterior e maior

### Medição anterior à decisão

11→39, 39→11, 11→40 e 40→11 quebram uma vez antes de `More` nos dois
motores. O vanila conserva uma página e `After=Tail`; o cristalino P1390
pagina `After`. Quando o fragmento exterior vem primeiro, `More` já coincide,
mas `One` revela reserva excessiva; na ordem inversa, o fechamento ordinário
também atrasa a segunda baseline.

### Decisão interna

A validação admite exatamente dois `Styled(Text)` size-only desiguais,
uma palavra não-whitespace por body, quando um tamanho é exatamente o
exterior e o outro é estritamente maior, existe um único wrap na fronteira e
a reserva completa cabe. O carrier e os fechamentos exteriores de P1389 e a
soma de métricas por linha de P1390 são reutilizados. Na ordem
maior→exterior, a primeira palavra pode ser horizontalmente overlong e ainda
forma uma linha válida. O scanner pode adiar essa rejeição apenas até ler o
segundo fragmento; o veredito final exige a célula P1391 completa e não amplia
P1390 nem aceita um prefixo incompleto. Como a palavra overlong fecha dentro
do layout de texto, antes de Sequence alcançar o segundo wrapper, um arm
one-shot faz esse único fechamento usar ascent e leading exteriores; o
estado é consumido no primeiro flush e não sobrevive a sub-frame/checkpoint.
A boundary de bloco não integra a reserva das duas linhas e continua aplicada
exatamente uma vez.

Um tamanho abaixo do exterior, dois tamanhos exteriores, multiword, delta
composto, terceiro fragmento, wrapper aninhado, wrap interno/múltiplo, forma
opaca e fitting insuficiente continuam `Unknown`. Não há API pública,
recursão, simulação ou coleção proporcional.

### Aceitação

11→39, 39→11, 11→40 e 40→11 ficam em uma página e coincidem com o vanila em
`Lead`, `More`, `One`, `Two`, `After` e `Tail` dentro de 0,001pt. 40→8
continua opaco; P1390, P1389 e P1388 conservam seus vereditos.

## P1392 — wrap único atravessando o tamanho exterior

### Medição anterior à decisão

Nos pares size-only 8→40, 40→8, 7→40 e 40→7, ambos os motores quebram
exatamente antes de `More`. O vanila mantém uma página, `After=Tail=220,166`
e posiciona `One` em 59,338pt para 8pt e 58,680pt para 7pt. O cristalino P1391
pagina `After`; na ordem menor→maior também antecipa `More`, enquanto na ordem
maior→menor a primeira palavra overlong fecha antes do carrier.

### Decisão interna

A validação aceita exatamente dois `Styled(Text)` size-only, uma palavra
não-whitespace por body, quando um tamanho é estritamente menor que o exterior,
o outro é estritamente maior e existe um único wrap na fronteira. A reserva
vertical completa deve caber. A soma linha-a-linha P1390, a boundary única
P1388 e os fechamentos exteriores P1389 são reutilizados.

Na ordem maior→menor, o scanner pode adiar a rejeição da primeira palavra
overlong até provar a forma P1392 completa e então arma o fechamento one-shot
P1391. Na ordem menor→maior, Sequence fecha explicitamente a primeira linha;
o singleton menor preserva baseline, ascent, alinhamento e próximo ascent
próprios, usando somente o leading exterior. Assim a reserva linha-a-linha já
usa top-edge própria e leading exterior. Quando o menor é a segunda linha, o
fechamento anterior já deixou uma baseline exterior; Sequence antecipa essa
baseline uma única vez pelo delta `top-edge exterior - top-edge menor`, antes
de visitar o segundo wrapper. A reserva já representa a mesma redução por usar
a top-edge própria da segunda linha; não há compensação adicional, recursão ou
simulação. O veredito final não amplia P1390/P1391 nem legitima um prefixo
overlong incompleto.

Tamanho igual ao exterior pertence a P1391; ambos acima pertencem a
P1389/P1390; delta composto, multiword, terceiro fragmento, wrapper aninhado,
wrap interno/múltiplo, forma opaca e fitting insuficiente continuam `Unknown`.
Não há API pública, recursão, simulação ou coleção proporcional.

### Aceitação

8→40, 40→8, 7→40 e 40→7 ficam em uma página e coincidem com o vanila
em `Lead`, `More`, `One`, `Two`, `After` e `Tail` dentro de 0,001pt. O caso
40→8 com `fill` adicional no segundo wrapper permanece opaco. P1391, P1390,
P1389 e P1388 conservam seus vereditos.

## P1493 — fronteiras neutras ao carrier HTML

Medição adversarial: um `ContextBlock` no-op antes de whitespace ou parbreak
deixava um carrier em `current_line` e acionava branches baseados apenas em
`is_empty`. Sequence usa o predicado material comum em todas as decisões de
fechamento: o carrier direto não cria linha, gap, wrap ou flush; wrappers e
itens reais preservam a decisão anterior.
