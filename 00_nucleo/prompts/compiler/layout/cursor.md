# Prompt L0 — cursor e fechamento de linhas
Hash do Código: b90b2853

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

Prefix/suffix conserva as páginas/ordem seladas: flow anterior página 1,
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

Cursor conserva somente a coordenação já selada: abre a transação após marker,
faz commit/rollback, progride e reexecuta ocorrências. Ao reexecutar, usa a
mesma origem que Block e Place produziriam no caminho ordinário; não observa
genericamente o crescimento de `current_items`, não traduz caudas fora de uma
transação rejeitada e não injeta `1.2em`, altura de Block ou top-edge de Place.

O vetor principal permanece: p3 `AFTER_FLOW=-2.596±0.002pt` e
`AFTER_MARKER=4.642±0.002pt`. Os controles sem Flush e sem float permanecem em
p1, com `AFTER_FLOW=40.604±0.002pt` e
`AFTER_MARKER=47.842±0.002pt`. Tornar os controles dependentes de flag de
replay, tipo futuro ou caso especial de Block refuta este owner.
