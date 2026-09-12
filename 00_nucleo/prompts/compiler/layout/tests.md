# Prompt L0 — regressões do motor de layout
Hash do Código: ce6786c3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/layout/tests.rs`

## Contrato e aceitação

Concentrar Worlds/métricas test-only e regressões de geometria e morfologia do
layout. Testes cobrem referencial absoluto, rebase, regiões, paginação, texto,
containers e aninhamento; números decisórios registram estado e vanilla.

## Observação recursiva em coordenadas globais

Helpers test-only que comparam geometria carregam a transformação afim
acumulada. Ao entrar em `Group`, compõem transform ancestral, translação de
`Group.pos` e `Group.matrix` na ordem do exporter. Grupos aninhados
compõem transitivamente.

Pontos de Text, TextShaped, Glyph, Image e Shape, extremos de Line e qualquer
coordenada comparada são projetados uma única vez para o referencial global.
`Semantic` é envelope transparente; `Link` conserva sua convenção.

A correção fica no observador. Não adaptar expectativas locais, remover Group
ou alterar produção. Testes que apenas contam variantes podem continuar a
observá-las sem projeção geométrica.

## P1292 amendment-10 — Place, clearance e checkpoint atômico

### Medição anterior à decisão

O legado P245 `p245_place_float_com_clearance_adiciona_espaco_y` trata
clearance bottom como deslocamento físico do frame para cima. A medição
ratificada refuta essa expectativa: numa página 100pt × 100pt contendo somente
o bottom float `ANCHOR`, `clearance: 0pt` e `clearance: 20pt` produzem uma
página e o mesmo bbox do token, `yMin=90.166`, no vanilla e no candidato v10.
Clearance reserva espaço adjacente para flow; não muda o anchor no fundo.

Controle separado de fitting colocou um Block de 80pt antes do bottom float.
Com `0pt`, `ADJACENT` e `FLOAT` permanecem na página 1, com `yMin=-2.596` e
`90.166`. Com `20pt`, `ADJACENT` permanece p1 e `FLOAT` migra para p2,
conservando `yMin=90.166`. Vanilla e candidato v10 foram bilaterais nos dois
controles. Isto prova reserva/fitting sem confundir com coordenada do frame.

Um terceiro controle reproduziu a falha de checkpoint: após prefix float e
Flush, um Block posterior contém texto `AFTER_FLOW` e Place não-float
`AFTER_MARKER`. No candidato refutado, o Place escapou para p2 enquanto o
texto migrou a p3; no vanilla ambos ficaram em p3. A ocorrência posterior
`FLOAT_AFTER` também ficou p3.

### Obrigação test-only

Substituir a expectativa refutada de P245 por dois testes independentes:

1. **anchor físico:** para o mesmo bottom float isolado, variar clearance entre
   `0pt` e `20pt` não muda página, posição ou bbox do frame/token;
2. **reserva/fitting:** com flow anterior de 80pt, `0pt` mantém o float em p1 e
   `20pt` o move para p2, sem alterar seu anchor físico dentro da página.

Adicionar a regressão do checkpoint atômico: PREFLOW p1, FLOAT_BEFORE p2, e
AFTER_MARKER + AFTER_FLOW + FLOAT_AFTER p3; todos os tokens aparecem uma vez.
Ela deve falhar se somente `current_line` migrar, se `current_items` pós-marker
ficar para trás ou se o float posterior for antecipado.

Os testes comparam observáveis de linguagem — página, bbox, presença e ordem —
e não nomes de buffers, formato do checkpoint ou número de replays. O teste
owner não autoriza ajuste produtivo, Block especial, coordenada afrouxada ou
alteração do default `1.5em`.

## P1292 amendment-11 — controles causais sem marker e sem float

### Medição anterior à decisão

Dois sources independentes, em página 100pt × 100pt e fonte efetiva 11pt,
produziram os mesmos valores dentro de cada renderer:

- candidato: `AFTER_FLOW=27.404pt`, `AFTER_MARKER=-9.834pt`;
- vanilla: `AFTER_FLOW=40.604pt`, `AFTER_MARKER=47.842pt`.

Um source conserva o float e remove Flush; o outro remove float e Flush. Logo
nem a sentinela nem a transação explicam o delta. A diferença de
`AFTER_FLOW`, `13.2pt`, coincide com o default Block `1.2em`; o Place vanilla
fica após o frame in-flow já composto.

### Obrigação test-only

Adicionar controles independentes que congelem, por página e bbox:

1. Block 30pt seguido de Block 10pt, sem float nem Flush:
   `AFTER_FLOW=40.604±0.002pt` e `AFTER_MARKER=47.842±0.002pt`, uma vez;
2. o mesmo com bottom float anterior, mas sem Flush: os dois tokens mantêm os
   mesmos valores e página do controle sem float;
3. variação explícita de `block.spacing`/`below` que prove resolução relativa
   e impeça hardcode de `13.2pt`;
4. preservação do vetor transacional já selado em p3
   (`AFTER_FLOW=-2.596±0.002pt`, `AFTER_MARKER=4.642±0.002pt`).

Os testes devem matar separadamente: gap omitido; Place ancorado no topo;
correção apenas em replay; constante em pt; e translação global de itens em
Cursor. Não podem ler campos internos como substituto do observável.
