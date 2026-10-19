# L0 — Escopos privados de captura de links
Hash do Código: 3c43281e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/link_capture.rs`

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

Possuir registro privado de destinos e escopos lexicais, limites neutros de
abertura/fechamento em variantes existentes de FrameItem (Semantic Artifact
Other Inline, prefixo NUL próprio, uma Shape Rect zero sem fill/stroke).
Reconhecer estritamente esse formato. O registro não cria Location nem
executa closures. Início/fim de escopo acrescenta limites independentes à
linha e ao buffer comitado; fechamento de linha segmenta ambos antes de
transferir conteúdo e reabre continuações após a transferência. Fechamento
de região segmenta o buffer comitado. Subframes têm pares locais e os
buffers exteriores mantêm seus próprios pares.

Resolver a árvore depois dos fixups e antes de publicar o documento:
consumir limites, preservar todos os frames na ordem/região e construir
FrameItem::Link por fragmento não vazio. Não confundir pertencimento com
intervalos geométricos nem coletar buffers antigos por seu comprimento.
Descer por grupos/wrappers; bounds usam o helper do owner de Link no
referencial de cada vetor. Mesma pertença herdada dentro/fora de um grupo
não exige links redundantes aninhados do mesmo destino; conservar destinos
distintos. Segmentos só de metadados não produzem área clicável. Não remover
posições, identidade textual ou wrappers semânticos do conteúdo.

Continuar capturas nas mudanças de linha/página/coluna, reter pertença em
frames diferidos, excluir marginais/nota/float alheios e respeitar rollback.
A fachada pública e a fase continuam sendo layout. Não silenciar uma
captura perdida descartando texto nem fabricar um retângulo na última página.

## Verificação local do resolver

O lint registrado em links-captura-2026-10-14/architecture.sarif aponta V2:
o módulo novo não contém testes próprios; os testes do fluxo estão no
owner de Link. Cobrir aqui preservação dos vizinhos sem destino, bounds
do fragmento, ausência de área para escopo vazio e preservação de artefato
visível cujo alt se parece com o marcador privado. Estes controles não
substituem os testes de paginação, subframes e rollback do fluxo completo.
