# Align — consumo vertical e continuidade

Lote interno de paridade, ainda dentro do objetivo geral aberto. HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Os resultados conservam UTC, status, diffstat exato, fontes e SHA-256 dos
binários. Tekt final:
`a353e810dd526a9e974145ccebfa811600340afd4d714743a518d04638136ee4`.
Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
árvore e receita P1489.

## O que mudou e por quê

`placement.rs` confundia a seleção do algoritmo horizontal (`flow_aligned`)
com a obrigação de consumir altura. Fora desse certificado, Align não
publicava o próprio fundo e podia ignorar a margem do bloco anterior.
A página Auto conservava o fundo anterior; Horizon/Bottom eram então
reposicionados contra uma altura que nem continha o corpo de Align.

O frame de altura positiva agora consome o gap causal de entrada e publica
seu próprio fim, independentemente do certificado horizontal. A margem
seguinte continua pendente. Página finita conserva o consumo do restante
por Horizon/Bottom. Em página Auto, irmãos posteriores não são tratados
como folga disponível para recentrar um frame anterior. Os fixups de Place
e subframes conservam suas rotas. A estratégia horizontal e o certificado
textual não foram ampliados.

Os refutadores adicionais corrigiram espaço forte inicial recebendo margem
artificial e altura de página anterior vazando para a página nova. Align
vazio continua sem inventar altura; um Block de altura declarada sem tinta
continua consumindo sua alocação. P898 passou a incluir a margem de parágrafo
medida: preservar sua expectativa antiga de gap zero esconderia o defeito.

L0 de placement e tests atualizado antes do código; ADR-0127 em fluxo
contínuo, sem campo público, API, fase ou modo novo. Resselo limitado aos
owners. Quatro testes novos foram RED, seguidos do refutador adicional
RED de espaço/página; os cinco passaram na rodada final.

## Validação e limites

`implementation/align-flow-layout-final-{pre,result}.json.gz` e `.log`,
UTC **20:37:38.957403Z–20:38:22.624549Z**:
**1.204 aprovados, 1 falha, 3 ignorados** no filtro workspace
`compiler::layout`. A falha é P1381 (prefixo de fonte maior), já existente;
não é resultado da suíte completa. Guard de fontes verdadeiro.

Build offline/locked, fmt de workspace, V1/V5/V15/V26 estritos e reversa
passaram em `implementation/align-flow-{build-final,fmt,lineage,reverse}`.
A reversa validou 970 consumers sem erro. Linter arquitetural completo
em `align-flow-architecture.log`: **0 erros, 380 avisos, 1.662 notas**.
Isso não aprova CI com avisos proibidos. Clippy, suíte completa, outros
hosts e release não foram certificados neste lote. `git diff --check` passou.

`align-flow-final/results.json.gz` registra 48 documentos bilaterais em PDF,
96 compilações, entre **20:38:37.960877Z e 20:39:19.637932Z**. Páginas e
posições textuais coincidem em 43 documentos dentro de 0.0001 pt. Os cinco
restantes estão identificados abaixo; contagem e palavras não substituem
uma verificação da posição das formas.

`align-flow-raster-final/results.json.gz` verifica os 42 documentos cujos
PDFs possuem uma única página em ambos os binários, sem usar `{p}` do CLI.
São 168 compilações SVG/PNG, sem erros ou diferenças de dimensões, entre
**20:39:29.455319Z e 20:40:45.970208Z**. Os 39 SVGs sem o caso Place-only
coincidem pixel a pixel após rasterização. Em PNG, 27 coincidem; além dos
três Place-only, nove têm diferenças de um nível de canal em texto e três
posicionam a borda de retângulo em coordenada fracionária (80 pixels, canal
máximo 101). O SVG coincide nesses casos: conservar a distinção entre
geometria e cobertura nativa, sem afirmar que um rasterizador é superior.
Todos os guards finais são verdadeiros. Métricas integrais em `summary.json`.

## Pendências verificadas

1. **Align contendo apenas Place**: tail19/23/27 preservam página e POST,
   mas MARK diverge por 13.2, 2.981 e 7.238 pt respectivamente. A rota sem
   altura causal ainda alinha um overlay contra uma região errada. Corrigir
   com L0 e testes próprios; não converter overlay em altura de fluxo.
2. **Texto simples antes de pagebreak em página Auto**: fresh32/33 já
   divergem na primeira página encerrada (Tekt 41.626 pt, pin 27.238 pt).
   A segunda página, onde está Align, coincide. O fechamento da página
   textual anterior permanece aberto; não atribuir esse erro ao novo Align.
3. **Espaçamento do parágrafo e Block**: a sonda `paragraph-gap-before`
   (estado do candidato anterior, com fontes/binários próprios) mostra que
   um único escopo `text(size:18pt)[Lead More]` exige 8.4 pt adicionais antes
   de Block/Align, mas dois wrappers de 18 pt separados por espaço exterior
   não exigem. Texto simples inline continua coincidindo. Logo a correção
   não pode ser apenas usar o maior tamanho de fonte observado.
   `Block` ainda ignora `par.spacing` explícito: controles 0 pt, 5 pt e 2 em
   divergem; o caso 2 em depende também do escopo textual correto.
   P1381 exige historicamente duas páginas, enquanto ambos os binários agora
   produzem uma; One ainda diverge verticalmente. Trocar só a contagem
   esperada continuaria errado. Investigar escopo compartilhado antes de
   qualquer patch; `paragraph.rs::shared_direction` já distingue escopo
   lexical de consenso de valores resolvidos, mas não resolve spacing.
4. Fragmentação de Block, dimensões automáticas, RTL, transformações,
   exportadores, avisos de CI e demais frentes do objetivo geral continuam
   abertas. Não houve commit, push ou publicação.

## Evidência preservada

`evidence.json.gz` contém 1144 entradas com caminho, SHA-256 e bytes
base64: snapshots das fontes, scripts, logs, documentos e renders, inclusive
arquivos ignorados. Foi reaberto e comparado com os arquivos em disco em
`2026-10-14T20:40:54.984396+00:00`.
SHA-256: `f7d708cf57c7fa6988fb8e45f972efba052bc3963c06fe9322f5898e71440ba0`.
Receipt em `archive-receipt.json`.
