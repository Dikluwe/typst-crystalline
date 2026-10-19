# L0 — Projeção privada de posições capturadas
Hash do Código: 5304343f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/position_marker.rs`
**Architecture:** ADR-0073, ADR-0074, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz mede posição local (0,0) invariável
sob inset/prefixo, embora a geometria mude; a invariância indevida à
translação refuta a hipótese de apenas uma baseline diferente do vanilla.
mod.rs:1131-1148/1180-1200 e realized.rs:66-75 capturam durante layout;
finish.rs:295-302 sela os mapas. html_frame.rs já transporta pontos
privados pela árvore sem API de FrameItem nova. Snapshot desta decisão:
posicoes-compostas-2026-10-14/before.json.gz. Identidade, geometria e
consulta são linguagem; índice e codificação do carrier são mecânica.

A revisão do primeiro candidato em position_marker.rs:173-192 identificou
um caso ainda não coberto: pular a última captura sem frame sobrevivente
permite que a projeção de uma escrita anterior substitua seu fallback
runtime. Preservar o valor da última escrita é requisito mesmo quando
seu frame é descartado; testar Location e Label com captura antiga viva.
O snapshot está em first-candidate-validation.json.gz deste diagnóstico.

## Contrato proprietário

Possuir o helper privado de escrita/captura e projeção final. Um enum
privado distingue Location e Label; o hub armazena a lista append-only.
Registrar os mesmos valores nos runtime maps durante layout, preservando
política de overwrite ou somente-se-ausente. Só uma escrita efetiva cria
captura. Nenhuma Location é gerada aqui; nenhuma closure é executada.

Emitir um carrier direto no fluxo corrente com índice da captura e ponto
capturado. Usar somente variantes existentes de FrameItem: Semantic
Artifact(Other)/Inline com prefixo privado NUL e uma Shape Rect de tamanho
zero, sem fill/stroke. Reconhecer estritamente esse formato, sem confundir
artefatos legítimos ou texto do produto. O índice não contém ID público
novo. Captura HTML detalhada mantém seu protocolo próprio sem duplicar o
carrier paginado; runtime maps ainda são escritos.

Na projeção, percorrer páginas/camadas e a árvore uma vez, compondo
Group.pos e matrix; wrappers transparentes não adicionam nova origem.
Coletar a primeira ocorrência geométrica de cada índice de captura e
remover os carriers. Resolver apenas a última captura lógica de cada
destino, por percurso reverso e conjuntos privados de destinos vistos,
independente da ordem de pintura; cópias visuais não viram observações.
Se essa última captura não tiver marker sobrevivente, conservar a escrita
runtime existente, sem reaplicar projeções de capturas antigas; não inventar
Location/página ou exigir uma ocorrência que tenha sido descartada.

Atualizar somente posições e páginas correspondentes dos sub-stores
existentes antes do sealing em finish. Não varrer o mapa global a cada
caixa, repetir layout, reconstruir o índice lógico, fazer I/O ou mudar
APIs/fase. O percurso é proporcional à árvore/capturas da iteração.
Preservar a âncora que o produtor capturou (não converter toda âncora para
baseline vanilla). Remoção também é utilizável no retorno visual HTML.

## Aceitação

RED→GREEN: Box com inset/prefixo/nesting projeta coordenadas, labels também;
shape no primeiro ponto mantém origem; identidade/ordem e políticas
or_insert/overwrite sobrevivem; clones não mudam o evento; matrizes,
wrappers, páginas/camadas e rollback são respeitados. Carriers não criam
tinta/baseline/extensão e não escapam ao documento público. Controles HTML
existentes preservam posições e visual. Revalidar build, lint e suítes.

## Label depois de uma linha fechada — 2026-10-14

### Medição anterior à decisão

links-captura-2026-10-14/followup-probes/results.json.gz registra HEAD,
working tree, UTC e binários das sondas page-ref e logical-ref. O título
está na página física 1; a referência aponta para página 2 e R8, enquanto
a consulta do próprio título retorna página 1 e contador 7. position_marker.rs:
129-131 envia todo carrier à linha pendente; references.rs:39-41 registra
o label depois do corpo; heading.rs:111-130 já fechou a linha do título.
O carrier isolado fica para a admissão do próximo texto. Snapshot anterior
em labels-fronteira-2026-10-14/before.json.gz. A hipótese deve ser confirmada
por RED que mantenha título e label na primeira página sob quebra automática.

### Decisão e aceitação

Um label registrado após um corpo cuja linha já foi fechada acompanha os
itens comitados quando eles possuem conteúdo e a linha pendente não tem
conteúdo causal. Nesse caso emitir seu carrier no buffer comitado, mantendo
o ponto capturado e a política de última escrita. Não o transferir para a
página que receberá o próximo texto. Se a linha ainda tem conteúdo causal,
conservar o carrier nela para que acompanhe admissão, wrap e composição.
Sem conteúdo comitado, manter o caminho pendente existente. Capturas de
Location antes do corpo não passam a seguir a regra pós-corpo de Label.

Testar título seguido de texto que exige outra página, label em texto
pendente admitido em nova página, label inline na mesma linha e ausência
de efeito sobre o conteúdo/paginação dos controles sem label. Revalidar
os controles de Box, rollback, última escrita, contexto e Pagebreak
explícito. Esta correção não altera captura do ponto antes do corpo nem
formatação de contadores; não fecha os casos não medidos de corpos
rotulados multipágina. Correção interna contínua ADR-0127.

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
consumer as construções/desestruturações antes da edição: [242].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.
