# Prompt L0 — regressões do motor de layout
Hash do Código: e19a69a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/layout/tests.rs`

## Contrato e aceitação

Concentrar Worlds/métricas test-only e regressões de geometria e morfologia do
layout. Testes cobrem referencial absoluto, rebase, regiões, paginação, texto,
containers e aninhamento; números decisórios registram estado e vanilla.

## P1293.final — observação recursiva em coordenadas globais

### Medição anterior à decisão

O receipt bloqueante independente SHA-256
`6dbf19c3ff89a0d10324ef9b5e273645e673eaf107a0cc268cc171b489708d2c`
mediu `cargo test --workspace -q` em working tree não commitada sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`: no crate core, `5403` testes
passaram e `13` falharam. Todos os witnesses usam o helper privado
`frame_items_recursive` de `01_core/src/compiler/layout/tests.rs:31-48`, que
atravessa `Semantic`, `Group` e `Link`, mas devolve itens filhos no referencial
local sem transportar posição ou matriz ancestral. O consumer medido tem
SHA-256 `92a2239354c3f82b364671ffa2d5d6a26b4767adfe3756571f7cf73b5f4f1c6c`
e `@prompt-hash 9eaf493c`.

Os testemunhos cruzam P813, P896, P987, P1088, inline baseline e limites:
coordenadas locais como `x=0` ou `y≈7.7` são comparadas a coordenadas de página
como `x≈294.34` ou `y≈99.47`. A definição vigente em
`01_core/src/entities/layout_types.rs:484-502` declara `Group.pos` no espaço do
pai, `Group.matrix` como transformação afim e filhos no espaço local;
`TransformMatrix::concat/apply` em `:1045-1093` fornece a composição existente.
O Núcleo `layout-coordinates` pinado acima exige o mesmo rebase para FrameItems,
grupos e emissões diferidas.

Medição: o helper descarta a transformação exatamente na descida ao grupo.
Inferência: compor a cadeia ancestral restaura o referencial global já usado
pelas expectativas sem mudar layout. Refutador: depois da composição completa,
qualquer um dos 13 testes ainda divergir, quantidade/morfologia de itens mudar,
ou a correção exigir tocar produção; nesse caso este owner test-only não mascara
a falha e o owner produtivo causal deve ser reaberto separadamente.

O receipt bloqueante registra internamente um hash antigo do owner com prefixo
`deee338…`; a leitura atual mede o L0 pré-decisão como
`deee32e8fadf311c9a8686004e27b098230b7ef348a4d8b09dd91f9f72d4f157`.
A divergência documental não é usada como prova causal: linhas do helper,
consumer raw e resultados foram conferidos diretamente neste estado.

### Decisão test-only

Todo helper recursivo deste consumer que compare geometria deve carregar a
transformação afim acumulada do ancestral até o item observado. Ao entrar em
`FrameItem::Group`, compõe, na ordem definida pelos tipos/exporters vigentes, o
transform ancestral, a translação de `Group.pos` e `Group.matrix`; grupos
aninhados compõem transitivamente. Posições de `Text`, `TextShaped`, `Glyph`,
`Image` e `Shape`, extremos de `Line` e qualquer ponto usado numa asserção são
projetados uma única vez para coordenadas globais. `Semantic` continua envelope
transparente; `Link` conserva a semântica vigente sem deslocamento inventado ou
dupla aplicação.

A implementação pode usar apenas carrier/cópia privada test-only para manter a
API interna conveniente; não adiciona entidade, campo, variante ou assinatura
pública. Testes que apenas contam/classificam variantes preservam essa função,
mas toda comparação geométrica usa o referencial global. As 13 expectativas,
fixtures, tolerâncias e ordem dos itens ficam byte-conceitualmente inalteradas.
É proibido adaptar números locais, remover o `Group`, ou alterar
`01_core/src/compiler/layout/equation.rs` e seu L0.

Classificação ADR-0107/0108: geometria global é o observável; referências Rust,
recursão e cópia do helper são transporte test-only, medido antes da decisão.
ADR-0127: não há contrato público, default, fase ou quebra de compatibilidade;
fluxo contínuo após resselo. ADR-0129: este Prompt continua owner 1:1 exclusivo
de `01_core/src/compiler/layout/tests.rs` e não legitima produto.

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
alteração do default `1.5em`; o autor independente de testes materializa esta
obrigação somente após o resselo.

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
