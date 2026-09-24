# Prompt L0 — cursor e fechamento de linhas
Hash do Código: c46a31e2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/cursor.rs`

## Contrato

Gere palavras/chunks, wrap, baseline, `flush_line`, paginação e footnotes.
Avanço usa arestas tipográficas, leading e extensões inline; collectors e
pending geometry recebem a mesma translação dos itens. Página auto deriva
dimensão do conteúdo real, sem constantes posicionais de oracle.

## Aceitação

Wrap, RTL, linebreak, parbreak, footnotes, páginas auto e limites permanecem
geométrica e morfologicamente equivalentes ao vanilla ratificado.

## P1382 — baseline e avanço de linha com fragmentos textuais de tamanhos mistos

### Medição anterior à decisão

Com página 200pt × 120pt, margem 10pt e a linha `Lead` em 11pt seguida de
`More` em 18pt, o vanila posicionou ambas as baselines em 21,844pt e a linha
seguinte em 36,232pt. O cristalino posicionou a primeira linha em 17,238pt e
a seguinte em 40,782pt. Com a ordem inversa, o vanila manteve 21,844pt e
36,232pt; o cristalino acertou a primeira baseline em 21,844pt, mas pôs a
seguinte em 38,888pt. A fonte ratificada constrói a linha com o maior ascent
e descent dos frames e translada cada frame para a baseline comum
(`inline/line.rs:562-632`).

### Decisão

Ao fechar uma linha, este owner distingue três grandezas:

1. o ascent da linha corrente é o máximo entre o top-edge de todos os
   fragmentos textuais e as extensões inline;
2. o descent corrente é o máximo entre o bottom-edge textual e as extensões
   inline;
3. o ascent previsto da linha seguinte e o leading são resolvidos pelo estilo
   ativo do parágrafo depois de restaurados os estilos inline.

Esta decomposição aplica-se quando a própria linha contém pelo menos dois
tamanhos textuais distintos. Linha uniforme ou com um único fragmento textual
mantém a fórmula anterior, já validada pelos casos P762/P1120/P1381; ampliar essa
regra exige uma medição própria.

Se o ascent corrente excede o ascent assumido quando a baseline foi criada,
a linha inteira e o cursor descem somente essa diferença. O avanço posterior
é `ascent_seguinte + descent_corrente + leading_do_parágrafo`; não reutiliza o
ascent máximo da linha anterior como se também governasse a seguinte. O
ascent assumido armazenado passa a ser o ascent seguinte. Verificações de
fitting usam a mesma correção de baseline, sem fórmula paralela.

Nenhuma assinatura pública, fase, carrier ou estrutura de documento muda.
Wrap, caixas inline, tamanhos uniformes e os caminhos P1374–P1381 permanecem
controles obrigatórios. Distribuição fracionária com prefixo textual misto
continua `Unknown` neste passo: a correção do cursor não relaxa o gate
fail-closed de `sequence.rs`.

### Aceitação

Nas ordens 11pt→18pt e 18pt→8pt, todos os fragmentos da primeira linha
partilham a baseline do vanila e a linha seguinte também coincide. Três
fragmentos 11pt→18pt→11pt produzem uma única baseline comum. Tamanho uniforme,
leading explícito, caixa inline e paginação não regridem.

## P1374 — fechamento da linha terminal de distribuição fracionária

### Medição anterior à decisão

Na matriz 1fr/2fr, depois de as parcelas reproduzirem exatamente as posições
do vanila, `ensure_initial_baseline` tratou a baseline terminal no limite útil
como overflow e moveu `After` para uma segunda página. Quando a linha ficou na
primeira página, `flush_line` ainda criou uma segunda página vazia porque o
avanço até à baseline hipotética seguinte ultrapassou o limite, embora a tinta
da linha terminal coubesse. A causa é distinta de P1373: aqui o finalizador
fracionário ocupa deliberadamente toda a região.

### Decisão

Uma sentinela interna, armada pelo owner Sequence apenas para a unidade
terminal de uma distribuição fracionária válida, permite igualdade com o
limite em `ensure_initial_baseline`. No primeiro `flush_line`, a sentinela é
consumida; se a tinta real cabe, o avanço de baseline seguinte não cria página
vazia. Se a tinta excede o limite, a paginação normal permanece. A sentinela
participa do checkpoint de replay e não altera sequências sem frações.

### Aceitação

A sonda 1fr/2fr permanece numa página, com `After` no limite observado no
vanila; conteúdo que realmente ultrapasse o limite continua a paginar. A
sentinela é consumida uma vez e não afeta a linha posterior.

## P1373 — progressão da primeira linha após região preenchida

### Medição anterior à decisão

Depois de `block(height: 100%)` passar a conservar e resolver sua altura, a
sonda S07 recompilada produziu duas páginas no cristalino, mas a segunda ficou
vazia e `pdftotext` observou somente `Probe`. O Block terminou no limite útil;
`ensure_initial_baseline` posicionou a linha `After` além desse limite e
`flush_line` só avançou a página depois de drenar a linha para a página
anterior. Isto explica o `TEXT_MISMATCH` sem atribuí-lo a metadata ou PDF.

### Decisão

Antes de materializar a baseline de uma primeira linha cujo cursor já atingiu
ou ultrapassou o limite inferior finito, este owner progride pela transição
normal de região se a página corrente já contém conteúdo. A linha nasce na
nova região; não é drenada fora dos bounds nem copiada depois. Página vazia e
altura automática não avançam, evitando ciclo sem progresso e página
artificial. O owner não conhece `Block`, percentagem ou seu espaçamento: usa
somente cursor, limite e ocupação correntes.

### Aceitação

Na sonda P1373, `Probe` permanece na primeira página e `After` aparece uma vez
na segunda, na mesma posição do vanila. Um documento terminado no Block não
ganha página vazia. A transição é no máximo uma por primeira linha e não cria
recursão.

## P1292 — finish/advance do prefixo de floats em `place.flush`

### Medição anterior à decisão

No baseline cristalino `cursor.rs` SHA-256
`120f04f65f53aa431f49b31288481466f4ac6bcae86815b82fc13db1ba17e932`,
`new_page` chama `flush_pending_floats` antes de fechar a página, e esse drain
toma o buffer integral e o emite na região corrente. A implementação refutada
em `flush.rs` SHA-256
`6e6f7b8474dce6de92d4d456b295f2e4036f9c6b12e3f91c15e00592d1960d51`
chamou o drain diretamente no marcador: prefix/suffix ficou em uma página em
vez das três do vanilla ratificado.

Medição black-box adicional, com páginas 100pt e tokens visíveis, confirmou:

- prefix/suffix: vanilla 3 páginas, candidato direto 1;
- sem flush: vanilla 2 páginas com o flow posterior ainda na página 1,
  candidato 1;
- top+bottom: vanilla 2 páginas, candidato 1;
- sem floats: marcador e controle são idênticos dentro de cada renderer;
- nested: marcador e controle ficam numa página e não afetam o outer;
- fronteira já inevitável: marcador e controle ficam em 2 páginas, sem quebra
  adicional, mas o candidato conserva a colocação prévia incorreta.

No upstream ratificado, `flow/distribute.rs:514-521` não emite o buffer no
marcador: enquanto existem floats pendentes, pede `Finish(false)` da região.
O observável normativo é terminar/avançar a região pelo distribuidor já
existente antes de continuar o flow; nomes de tipos e passos upstream não são
copiados como mecânica obrigatória (ADR-0107).

### Decisão — hook interno no owner paginado

Este owner fornece ao consumer `compiler/layout/flush.md` o hook interno e
não-público:

```text
finish_float_prefix_at_marker(prefix_boundary)
```

`prefix_boundary` identifica por ocorrência o comprimento do buffer no ponto
do marcador. Se é zero, retorna sem flush, item, cursor, coluna ou página. Se
é positivo, o hook mantém essa sentinela lógica ativa e manda o distribuidor
normal terminar/avançar a região corrente enquanto alguma ocorrência daquele
prefixo ainda estiver pendente. Cada transição usa a mesma finalização de
página/coluna/sub-região, reservas, clearance e posicionamento top/bottom já
vigentes; o flow posterior só recomeça depois da realização integral do
prefixo.

O mecanismo de finalização deve realizar em cada região somente as ocorrências
do prefixo elegíveis para ela e conservar as restantes pendentes para a região
seguinte. É proibido `std::mem::take` integral seguido de emissão cega na
região corrente quando parte do prefixo precisa paginar. Ordem e identidade
por ocorrência não mudam. Uma ocorrência criada depois do marker não pertence
ao boundary e nunca é antecipada.

O hook não chama `flush_line`, não cria `FrameItem`, não implementa um segundo
distribuidor e não duplica fórmulas de alinhamento/clearance em `flush.rs`.
Também não transforma o marcador em `pagebreak`: uma transição física só
ocorre quando o prefixo pendente exige terminar a região. Em sub-layout, opera
somente no estado/distribuidor ativo; não alcança buffers externos. Nenhuma
assinatura pública, entidade, fase eval/layout ou formato de documento muda.

### Aceitação focal e limites

Prefix/suffix conserva as páginas/ordem validadas: flow anterior página 1,
float-prefixo página 2, flow posterior e float-sufixo página 3. Top+bottom
conserva top/flow na página 1 e bottom/flow posterior na página 2. No-flush
continua permitindo flow posterior na página 1; no-floats é no-op; nested não
vaza; fronteira não ganha página artificial.

Refutam este amendment: drain integral na página atual; `new_page`
incondicional; flush de linha; item próprio; reordenação; antecipação do
sufixo; alteração de placement/clearance; ou acesso ao Layouter exterior.
Floats individualmente maiores que uma região vazia e combinações não medidas
permanecem `Unknown`; exigem medição própria e não podem justificar loop sem
progresso, descarte ou relaxamento destes vetores.

## P1292 amendment-9 — recomposição estabilizada antes da retomada

### Refutação medida

O hook v9 alcançou três páginas, mas retomou AFTER na página 2 ao lado do
float-prefixo; top+bottom ficou em uma página e boundary preservou duas
páginas com posição errada. Com `clearance: 1.5em` explícito, top+bottom
alcançou duas páginas, porém a posição bottom continuou divergente, e
prefix/suffix continuou com AFTER na página 2. Texto simples e
`block(breakable: false)` apresentaram a mesma retomada errada. Logo o cursor
não pode adivinhar a altura/tipo do child seguinte e um re-preflight específico
de Block não resolve o contrato.

### Decisão

`finish_float_prefix_at_marker` mantém a fronteira por ocorrência e usa o
distribuidor paginado existente. Quando `compiler/layout/place.md` admite uma
inserção do prefixo e muda reservas top/bottom, este owner restaura o
checkpoint lógico da região e a recompõe contra a área efetiva reduzida. O
replay conserva ocorrências já realizadas por identidade e repete somente a
composição necessária; não duplica conteúdo, floats, contadores ou efeitos.

O ciclo termina apenas quando a região está estável e nenhuma ocorrência do
prefixo permanece pendente nela. Só então a sentinela é consumida e o flow
posterior continua pelo processamento normal na região efetiva já reservada.
Texto, linha, Block e qualquer outro child decidem fitting por suas regras
ordinárias; o hook não os inspeciona, não calcula altura futura e não força uma
quebra depois do marker. Assim, com clearance zero o flow pode caber ao lado
do float; com default `1.5em`, o mesmo flow pode progredir para a página
seguinte por falta real de espaço.

Se a inserção não cabe, somente a regra normal de progresso termina/avança a
região, preservando o prefixo pendente. Cada avanço reinicia com checkpoint e
reservas próprios da nova região. O loop exige progresso verificável por
região ou por redução do prefixo; estado idêntico não pode girar, descartar ou
relaxar o float. Floats posteriores à fronteira nunca participam do replay.

Cursor coordena checkpoint, recomposição e avanço; Place possui fitting,
reserva e coordenadas; Flush possui apenas a política da sentinela. Não há
segundo distribuidor, fase nova, item próprio, inspeção do conteúdo posterior
ou mudança em `compiler/layout/block.md`.

### Amendment-10 — checkpoint do sufixo completo

O checkpoint citado acima é o checkpoint transacional definido pelo owner
`compiler/layout.md`, não uma cópia isolada de `current_line`. Assim que o
prefixo e suas reservas estabilizam, o Cursor abre a fronteira de retomada
antes de processar qualquer ocorrência posterior ao marker. Até a unidade de
flow ser aceita, toda mutação pós-fronteira — inclusive Place não-float já
anexado a `current_items` — permanece sufixo especulativo.

Se fitting rejeita a unidade na região reduzida, o Cursor faz rollback atômico
da cauda inteira, restaura posição/métricas/buffers, avança normalmente e
reexecuta as ocorrências na nova região. Prefix floats realizados, reservas e
items anteriores não participam do rollback. Float posterior pertence ao
sufixo e é refeito sem antecipação ou duplicação.

É proibido “migrar a linha” deixando `current_items` para trás, varrer o
próximo Content para descobrir o que salvar ou criar tratamento de Block. O
critério é temporal/por ocorrência: todo efeito depois da fronteira migra ou
commita junto, independentemente do variant que o produziu.

### Aceitação corrigida

Prefix/suffix com default `1.5em`: PREFLOW p1, FLOATPREFIX p2, AFTER e
FLOATSUFFIX p3. Com `clearance: 0pt`, a reserva menor pode manter AFTER em p2,
como no vanilla. Top+bottom e boundary devem igualar também as posições, não
somente a contagem de páginas. No-flush, no-floats, nested e fronteira sem
espaço adicional permanecem controles causais.

O controle amendment-10 acrescenta Place não-float dentro da unidade
posterior: o marker visual, o flow que o contém e o float posterior devem
aparecer juntos em p3, exatamente uma vez; nenhum item pós-marker pode ficar
em p2.

### Amendment-11 — Cursor não substitui Block nem Place

A medição sem Flush e sem float isolou duas divergências ordinárias anteriores
à transação: a transição após Block perdeu `1.2em` e Place não-float com linha
pendente ancorou no topo. Como ambas ocorrem sem marker, não pertencem a
checkpoint, replay ou avanço paginado.

Cursor conserva somente a coordenação já validada: abre a transação após marker,
faz commit/rollback, progride e reexecuta ocorrências. Ao reexecutar, usa a
mesma origem que Block e Place produziriam no caminho ordinário; não observa
genericamente o crescimento de `current_items`, não traduz caudas fora de uma
transação rejeitada e não injeta `1.2em`, altura de Block ou top-edge de Place.

O vetor principal permanece: p3 `AFTER_FLOW=-2.596±0.002pt` e
`AFTER_MARKER=4.642±0.002pt`. Os controles sem Flush e sem float permanecem em
p1, com `AFTER_FLOW=40.604±0.002pt` e
`AFTER_MARKER=47.842±0.002pt`. Tornar os controles dependentes de flag de
replay, tipo futuro ou caso especial de Block refuta este owner.

### Amendment-12 — tolerância da baseline terminal fracionária

#### Medição anterior à decisão

No prefixo P1386, `One` já coincide com o vanila após remover a boundary
duplicada da reserva, mas `After` é enviado à página seguinte. Guardas de ULPs,
0,000001pt e 0,0009pt na reserva não alteraram o resultado release. O ponto de
decisão restante é `ensure_initial_baseline`: somente quando
`fractional_flow_tail_pending` está ativo, ele aceita a baseline terminal até
`page_bottom_limit + f64::EPSILON`, limite menor que a precisão acumulada do
pipeline e muito menor que o oráculo de 0,001pt.

#### Decisão interna

Somente para uma cauda fracionária já marcada pelo distribuidor, quando a
baseline seguinte ultrapassa o fundo no máximo pela boundary exterior
`below - leading`, o Cursor fixa essa baseline no próprio fundo. Acima da
boundary mais a tolerância de 0,001pt, a quebra ordinária permanece. O ajuste
não aumenta Block, não reconhece Content e não se aplica a fluxo ordinário;
ele remove a segunda representação da mesma boundary já entregue antes do
Block pelo prefixo P1386.

#### Aceitação

18pt→12pt, 18pt→14pt, 20pt→14pt e 20pt→18pt P1386 permanecem em uma página,
com coordenadas iguais ao vanila dentro de 0,001pt. P1385 e P1374–P1384 permanecem exatos; fluxo sem
`fractional_flow_tail_pending`, caudas acima de uma boundary exterior e os
controles de largura P1386 continuam seguindo a quebra ordinária.

### Amendment-13 — mistura relativa ao estilo ativo

#### Medição anterior à decisão

Nos pares P1387 12→12 até 20→20, os itens têm tamanho igual entre si, mas
diferem do exterior 11pt. O Cursor P1382 deixa `mixed_text_sizes=false` porque
o fold compara somente cada item com o primeiro. O vanila mantém o próximo
ascent e o leading exteriores: `One` segue o máximo e `After=Tail`, enquanto o
cristalino faz ambos dependerem do tamanho uniforme.

#### Decisão interna

`mixed_text_sizes` é verdadeiro se tamanhos dos itens diferem entre si ou se,
havendo pelo menos dois itens textuais, um item difere do estilo ativo do
parágrafo. Nesse segundo caso aplicam-se
as mesmas escolhas P1382: próximo ascent e leading vêm do exterior. Conteúdo,
baseline compartilhada, estilos visuais e máximo textual não mudam.

#### Aceitação

Os quatro pares uniformes P1387 aceitos pelo gate de largura (12pt–18pt)
coincidem com o vanila dentro de 0,001pt. 20pt permanece `Unknown` no
preflight, embora a regra de Cursor feche sua cauda.
Singletons P1381, linhas realmente exteriores, linhas mistas P1382–P1386, fluxo ordinário e a
tolerância terminal Amendment-12 preservam seus resultados.

### Amendment-14 — fechamento de linha no wrap validado P1389

#### Medição anterior à decisão

No prefixo uniforme 21→21, o primeiro wrapper deixa um único item de 21pt na
linha. Ao detectar overflow dentro do segundo wrapper, o fechamento ordinário
usa esse tamanho tanto para o próximo ascent quanto para o leading e posiciona
`More=32,512pt`; o vanila usa o estilo exterior de 11pt e produz
`More=26,012pt`. O mesmo crescimento indevido aparece de 22pt a 40pt. Tornar
todo singleton relativo ao exterior foi refutado no P1387 porque altera os
singletons P1381 que não nasceram deste wrap.

#### Decisão interna

Cursor oferece um fechamento interno exclusivo para o carrier validado de
P1389. Ele reutiliza integralmente `flush_line`, mas marca a linha textual
singleton como mista em relação ao estilo exterior ativo: máximo e descent
continuam pertencendo ao item, enquanto próximo ascent e leading vêm do
exterior. Como a baseline foi inicialmente assumida pelo exterior do
parágrafo, o ajuste positivo até o ascent máximo da linha continua obrigatório;
suprimi-lo desloca `More` para cima por `top(interno)-top(exterior)`. O
fechamento ordinário não muda.

Sequence só chama essa entrada antes do segundo wrapper e antes de fechar sua
linha final no prefixo P1389. Não há flag persistente, campo em checkpoint,
inspeção de Content no Cursor, API pública ou mudança para wrap geral.

#### Aceitação

Nos positivos P1389, a distância entre `Lead` e `More` usa próximo ascent e
leading exteriores e a cauda fracionária termina no fundo. Singletons P1381,
prefixos sem wrap P1383–P1388 e todo `flush_line` ordinário preservam seus
resultados.

### Amendment-15 — consumo antecipado da palavra overlong P1391

#### Medição anterior à decisão

Na ordem 39/40→11, a primeira palavra fecha dentro do próprio layout de texto,
antes da chamada explícita P1389 de Sequence. O fechamento ordinário produz
`More` 36,624pt/37,932pt abaixo do vanila e a cauda pagina. Na ordem
11→39/40, a chamada explícita já ocorre a tempo e coincide.

#### Decisão interna

`flush_line` consome o arm one-shot privado do Layouter e, quando ativo,
executa exatamente `flush_line_with_outer_singleton(true)`. O fechamento
explícito P1389 também limpa o arm antes de usar a mesma fórmula. Nenhuma
outra parte do algoritmo, item, métrica ou decisão de paginação muda.

O Cursor não reconhece Content nem decide quando armar: Sequence é o único
dono da validação. O estado não é global, não recursa e é consumido no
primeiro flush. Os quatro pares P1391 coincidem com o vanila em página e bbox;
P1390, P1389, P1388 e singletons ordinários permanecem controles.

### Amendment-16 — singleton menor validado por P1392

#### Medição anterior à decisão

Em 8→40 e 7→40, Sequence alcança a fronteira do wrap com um único item menor
que o exterior. O fold de `flush_line` parte do estilo exterior; por isso o
máximo continua 11pt e o teste P1389 `max_font_size != outer_font_size` não
detecta que o item real é 8pt/7pt. O fechamento não seleciona a fórmula
exterior e antecipa `More` em 1,95pt/2,60pt.

#### Decisão interna

No fechamento singleton explicitamente validado, o tamanho do único item
textual real é comparado ao exterior, não apenas ao máximo que já inclui o
fallback exterior. Se esse item é menor, baseline, ascent, alinhamento e
próximo ascent continuam próprios do item; somente o leading usa o exterior.
Classificá-lo como linha mista deslocaria a baseline menor, divergindo do
vanila. Um singleton maior mantém a fórmula exterior completa de P1389.
Máximo/descent e linhas não validadas não mudam.

O arm one-shot P1391 também pode ser emitido pela validação P1392 na ordem
maior→menor; Cursor apenas o consome e não reconhece Content. Os quatro pares
P1392 coincidem com o vanila; P1391, P1390, P1389, singletons ordinários e
deltas compostos permanecem controles.

### Amendment-17 — expansão horizontal adiada de `repeat` P1407

#### Medição anterior à decisão

No baseline `ed7340ada`, a parcela de uma caixa `1fr` só existe em
`expand_fr_spacings`; `repeat.rs` já dispôs uma única peça e o suffix ainda não
foi deslocado. A sonda de 100 pt produziu 1 ponto contra 23 no vanila pinado.

#### Decisão interna

Ao consumir as parcelas fracionárias de uma linha finita, Cursor associa uma
parcela somente ao `repeat` privado cuja faixa termina na fronteira daquele
`fr`. Delega ao owner `repeat.rs` a multiplicação e não interpreta body, gap ou
justify. Primeiro desloca o suffix uma vez pela parcela; depois insere as
cópias antes dele, mantendo índices posteriores corrigidos pelo número real de
itens inseridos. Estado órfão ou largura infinita é limpo sem loop.

#### Aceitação

Uma linha com `A + box(1fr, repeat[.]) + B` mantém A/B únicos e preenche a
caixa; duas fronteiras não deslocam suffix duas vezes; o teto de 1000 pertence
ao owner de repeat e não ao Cursor.

### Amendment-18 — parcelas fracionárias gerais de `box` P1408

#### Medição anterior à decisão

O vanila ratificado aceita `box(width:)` com pesos positivos, zero e negativos.
`Fr::share` calcula cada parcela como
`max(weight / total * remaining, 0)` quando a razão e o restante são finitos;
caso contrário devolve zero. O Cursor cristalino exige hoje soma estritamente
positiva, portanto não reproduz um `-1fr` isolado mesmo que o carrier seja
injetado.

#### Decisão

Numa linha finita, Cursor soma os pesos sem normalizá-los e recompõe o espaço
distribuível adicionando ao restante as larguras naturais privadas das caixas
fracionárias. Para cada fronteira, aplica exatamente a regra de parcela medida
acima; não exige peso nem soma positivos. O deslocamento é
`parcela - largura_natural`, substituindo a ocupação provisória; `h(Nfr)` tem
largura natural zero. A parcela alimenta o repeat associado, se houver. Soma
zero, razão não finita ou restante não finito dão parcela zero e limpam o
estado sem loop. O custo é linear nas fronteiras e nos itens, independente da
magnitude dos pesos.

Esta regra acompanha a ampliação pública de P1408. O gate ADR-0127 foi
confirmado pelo humano em 2026-09-15 antes da alteração do código.

## Amendment-19 — consumo realizado e margem da última coluna P1415

### Medição anterior à decisão

No baseline `0a7de3dd9`, o Cursor só numera quando o valor é `Value::Str` e
emite `FrameItem::Text` diretamente. A prova bilateral de duas colunas mostra
que o vanila força a última coluna para `end`, enquanto o cristalino mantém os
números `3` e `4` à esquerda. A fonte pinada decide isso depois de conhecer o
índice físico da coluna.

### Decisão proposta

No mesmo ponto pós-estabilização de P1414, pattern continua imediato; função
avança o contador exatamente uma vez e consulta a resolução validada da
ocorrência. Na primeira passagem, ausência de resolução registra request com
função, número, chain e span, sem inserir número provisório. Na passagem
resolvida, o `Content` retornado é disposto em subframe marginal e alinhado à
baseline causal; `none` produz frame vazio, mas a chamada e o contador existem.

A maior largura de numeração é própria da região/coluna física. Com duas ou
mais colunas, a última coluna usa sempre margem `end`; as anteriores respeitam
`number-margin`. O contador continua documental ou por página segundo P1414 e
não reinicia por coluna. Checkpoint/rollback não duplica request nem consumo.
As coordenadas marginais nascem na largura da região física ativa e só depois
são transladadas pelo owner de colunas; usar a largura global antes dessa
translação deslocaria a última numeração para fora da página.

Cursor não chama `Func`, não converte `Content` em debug/texto e não guarda
Engine. A realização e a passagem nova aguardam o gate ADR-0127 P1415.

## Amendment-20 — supressão transversal validada P1416

### Medição anterior à decisão

Em segmentos irmãos uniformes de `#columns()`, o vanila conta cada ordinal
transversal uma vez, inclusive quando o primeiro segmento está vazio ou uma
coluna posterior tem linhas adicionais. O cristalino P1415 conta cada flush
regional e soma as colunas. O callback reproduz a mesma duplicação.

### Decisão interna

Depois de avançar o contador físico, Cursor consulta um contexto privado
instalado pelo owner de colunas. Se o inteiro está no prefixo já materializado,
não cria pattern, request, item nem fixup; o fechamento da linha continua
inalterado. Uma cauda nova segue exatamente o caminho P1414/P1415 e mantém o
inteiro documental alcançado pelo snapshot comum.

Para ocorrência emitida, o contexto fornece duas compensações escalares. A
escolha já resolvida de `number-margin` aplica a compensação inicial ou final
antes da translação normal da coluna, fazendo o número terminar no bordo
externo do contentor. Cursor não conhece quantidade de colunas, `colbreak`,
callback ou conteúdo irmão.

O contexto não se aplica a colunas de página nem ao fluxo contínuo e integra o
checkpoint privado do Layouter. Métricas verticais diferentes, RTL, nested
columns e estilos locais divergentes permanecem `Unknown`; não inferir
deduplicação geral por baseline a partir do recorte uniforme.

## Amendment-21 — candidatura diferida de pattern P1417

### Medição anterior à decisão

No baseline `af587e30b`, emitir o pattern durante o flush fixa contador e item
antes de o owner poder ordenar baselines de regiões irmãs. A fonte pinada toma
essa decisão somente depois de recolher e ordenar os marcadores verticais.

### Decisão interna

Quando o contexto privado P1417 de `#columns()` está ativo e `numbering` é
string, Cursor não incrementa contador, não formata e não cria item/fixup.
Registra uma candidata com baseline estabilizada, página causal, pattern e
chain. Funções continuam no caminho P1415/P1416: o Cursor não as executa nem
presume a altura do retorno.

Fora desse contexto, patterns preservam o caminho imediato P1414. O buffer é
cauda transacional do Layouter e rollback o trunca; flush vazio e subframe não
registram candidata.

## Amendment-22 — candidatura de pattern em coluna de página P1418

### Medição anterior à decisão

No baseline `573c42c14`, o caminho imediato P1415 formata cada número no estilo
local da linha e não dispõe da altura compartilhada necessária para suprimir
baselines próximas dentro da coluna física.

### Decisão interna

O mesmo registro diferido P1417 fica ativo para pattern em coluna de página,
inclusive no fluxo contínuo. Cursor continua registrando baseline, página,
pattern e chain sem incrementar contador ou criar item. Ao fechar uma coluna
contínua, Cursor delega a consolidação da sua cauda ao owner de columns antes
de aplicar fixups e translação. No modo segmentado direto, o próprio owner faz
a drenagem equivalente.

Callbacks continuam imediatos e nunca entram nessa cauda. Fora de contexto
segmentado, P1414 permanece inalterado.

## Amendment-23 — estilo ao fechar coluna vazia P1419

### Medição anterior à decisão

No baseline `3a3d16604`, a primeira coluna vazia não fornece candidata a
P1418. A coluna seguinte passa então a doar a sua chain local, embora o vanila
conserve o estilo ativo antes do `colbreak()`. No fluxo contínuo, Cursor ainda
observa esse estilo no fechamento físico da coluna vazia.

### Decisão interna

Ao fechar uma coluna contínua, Cursor mantém a preferência P1418 pela chain da
primeira candidata da coluna. Se a cauda está vazia e o estilo compartilhado
ainda não foi capturado, guarda uma vez o `TextStyle` ativo antes de aplicar
fixups, translação e avanço regional. Uma candidata posterior não substitui o
snapshot.

Contador, scope, deduplicação, margem e callbacks permanecem inalterados. O
modo segmentado direto sem evento de fechamento vazio continua `Unknown`.

## P1414 — numeração no fechamento físico (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `df196b35b`, `flush_line_with_outer_singleton` conhece o conjunto
final de items, baseline, página e StyleChain depois de fitting. A prova vanila
numera tanto linebreak explícito quanto wrap automático; síntese em eval seria
semanticamente incorreta.

### Decisão proposta

Depois da eventual migração e correção de baseline, mas antes de drenar a linha,
o fechamento consulta `par.line.numbering`. Para pattern string, avança o estado
privado uma vez, formata pelo formatter canônico e insere `FrameItem::Text` na
margem externa, alinhado à baseline. `none`/ausente não emite nem incrementa.
Scope `page` reinicia quando a página causal muda; `document` preserva.

`number-margin` resolve left/start para a margem inicial e right/end para a
final neste recorte LTR. `number-align` governa o alinhamento horizontal do
número; `auto` cresce para fora do texto. `number-clearance` usa length ou
default proporcional ao font-size. Linhas sem items, flushes de segurança e
avanços hipotéticos não contam.

Ao sair do carrier Styled que introduziu `par.line.*`, uma linha terminal ainda
pendente é fechada antes do pop para não perder a configuração; isso não cria
linebreak adicional nem muda o conteúdo. Callbacks e coordenação de última
coluna ficam explicitamente para P1415. Gate público pendente.
## P1486 — número lógico inicial ao fechar página

### Medição anterior à decisão

`01_core/src/compiler/layout/cursor.rs:1229-1245` fecha a página
corrente com `page_number` físico; o número lógico validado só
existe depois da realização do `PageStore` em L3
(`infra/pipeline.md` P1486). O campo público `Page.logical_number`
foi confirmado pelo dono em 2026-09-24.

### Decisão

Inicializar o campo com `page_number` no fecho de cada página.
Não ler counter de runtime nem alterar o timing de paginação;
o overlay lógico posterior pertence exclusivamente à pipeline.

## P1486.1 — margem automática no fecho de página intermédia

### Medição anterior à decisão

`01_core/src/compiler/layout/cursor.rs:1100-1170` formata a margem com
`page_number` físico; `runtime.known_page_store` já é lido neste owner
para `Numbering::Func` (`:1187-1198`). A sonda bilateral no diagnóstico
P1486 extrai `1, 3` do cristalino após `update(10)` contra `10, 12` do
vanilla. Inferência: o snapshot lógico realizado basta ao Pattern da
página local; refutação seria mudança de contagem entre snapshot e layout.

### Decisão

Usar `known_page_store.logical_number_for_page` na formatação Pattern
de uma página intermédia quando disponível; antes da store, manter o
índice físico. O número lógico de Pattern composto fica no pending e
seu total será resolvido por `finish` conforme `compiler/layout.md`
P1486.1. Não alterar o campo provisório `Page.logical_number`, a
geometria explícita nem a fase de paginação.

## P1493 — flush material do carrier HTML

Medição adversarial: `flush_line` contava o carrier privado como item e
fabricava altura tipográfica; após um flush, a mesma árvore podia tornar uma
região falsamente não vazia. Cursor usa o predicado material comum para
`had_items`, página ou região vazia e linha terminal. O carrier ainda é
drenado e transportado para extração, mas sozinho não move cursor, baseline
ou página.
