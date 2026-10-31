# Imagens compartilhadas: validação por identidade — 2026-10-26

Implementada a alteração 2 autorizada pelo usuário: cada identidade de
`Arc<Vec<u8>>` validada com sucesso é visitada pelo decoder uma vez por chamada
de validação do documento. O conjunto privado abrange todas as páginas,
background/items/foreground, grupos e recursos de tiling em fill/stroke.
O conjunto é descartado no retorno. Nova chamada valida novamente; Arcs
distintos com bytes iguais permanecem distintos. Primeiro erro, decoders,
limites, cache de payload e bytes exportados permanecem com o contrato anterior.

## Proveniência e gates

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**, estado recebido,
UTC, lista exata de alterações/diffstat e 2165 hashes em
`lab/results/construction/diagnostics/image-validation-dedup-2026-10-26/before.json`.
O baseline é o release corrigido da alteração 1, não o binário anterior àquela
otimização. `after-source-state.json` congela a materialização: apenas
`03_infra/src/export/images/scan.rs` e seu proprietário
`00_nucleo/prompts/infra/export/images/scan.md` mudaram nesses inputs.
Fechamento `2026-10-27T00:20:01.649115+00:00`; execuções têm UTC/comandos/hashes individuais.

L0 atualizado antes do código, correção interna em fluxo contínuo ADR-0127,
sem mudança de API pública, fase, dependência, flag ou modo. Ownership/Núcleos
verificados antes da resselagem; somente o proprietário foi resselado.
Decodificação é mecânica; aceitação, primeiro erro e conteúdo visível são
observáveis da linguagem. A alteração 1 foi preservada e seus testes passaram.

## RED→GREEN

O contador de alocações do decoder real, num subprocesso isolado, encontrou
**sete decodificações antes e uma depois** para sete ocorrências do mesmo Arc
entre páginas, grupos, fundos/primeiro plano e células de tiling em fill/stroke.
O GREEN também confirmou duas decodificações para dois Arcs distintos com bytes
iguais e duas decodificações para duas chamadas sobre o documento compartilhado.
Os registros estão em `red.log`, `green.log` e `green-final.log`.

Teste adicional preserva a ordem de dois erros distintos e verifica que alterar
um Arc anteriormente válido entre chamadas força nova validação, incluindo uma
falha seguida de restauração dos bytes. O contador é exclusivo dos testes Linux;
não há estado de instrumentação no binário produtivo. O conjunto produtivo é
local à chamada e grava identidades somente após sucesso. Nenhum payload ou Arc
é mantido pelo conjunto; os próprios documentos garantem a vida dos ponteiros.

## Medições

Foram executadas 132 compilações alternadas sobre dez fontes.
Cada caso positivo teve sete amostras por binário; o caso de erro, três.
Comandos, UTC, ambiente, hashes e amostras estão em `paired-results.json`.
Tempo é wall time do processo CLI completo, RSS é pico em KiB. Medianas:

| Caso | Tempo antes → depois, ms | Variação tempo | RSS antes → depois, KiB | Variação RSS |
|---|---:|---:|---:|---:|
| shared-rgb-16 | 1724.1 → 228.5 | -86.7% | 191880 → 128556 | -33.0% |
| shared-rgb-pages | 1731.9 → 229.3 | -86.8% | 210596 → 128592 | -38.9% |
| shared-progressive-8 | 551.4 → 155.8 | -71.7% | 67184 → 67028 | -0.2% |
| shared-png-16 | 484.5 → 142.2 | -70.7% | 34224 → 34240 | 0.0% |
| shared-tiling | 945.1 → 219.9 | -76.7% | 192264 → 128872 | -33.0% |
| single-rgb | 124.7 → 124.6 | -0.1% | 122004 → 121992 | -0.0% |
| distinct-rgb-4 | 547.2 → 544.6 | -0.5% | 187144 → 187128 | -0.0% |
| single-png | 54.6 → 54.4 | -0.5% | 27216 → 27432 | 0.8% |

`shared-rgb-16` usa 16 ocorrências de um JPEG 6000×4000; `shared-rgb-pages`
distribui 16 ocorrências por quatro páginas. O progressivo é 3000×2000 e o PNG
1600×1200. Os controles usam um JPEG, um PNG e quatro arquivos JPEG com bytes
iguais mas Arcs distintos. No PDF do baseline, pdfimages confirmou um recurso
para 16 ocorrências compartilhadas e quatro recursos para os quatro arquivos
distintos. Para tiling, pdfimages não lista imagens de padrões; qpdf confirmou
um objeto de imagem no baseline e os rasters são verificados integralmente.

O ganho esperado é evitar trabalho repetido, não reduzir o número de pixels
de uma imagem única. Não inferir melhoria universal ou redução obrigatória de
pico RSS a partir das contagens. São fixtures sintéticas congeladas, em uma
máquina, não um corpus amplo de documentos/fotografias. Diferenças pequenas
nos controles precisam ser interpretadas como variação local, sem promessa de
ganho para imagens únicas ou distintas.

## Preservação e validação final

- Status e stderr iguais em todas as amostras pareadas, inclusive no erro.
- qpdf validou os PDFs positivos. Todas as páginas foram rasterizadas;
  75 arquivos de raster,
  texto e JPEG extraídos permaneceram idênticos entre as versões.
- Bytes JPEG comprimidos originais encontrados nos PDFs de todos os sete
  cenários JPEG positivos, incluindo tiling; páginas e payloads registrados em
  `payload-and-pages.json`. O caso multipágina mantém quatro páginas.
- 9960 testes Rust aprovados, 7 ignorados, três novos testes;
  42 testes Python de CI aprovados. Os 12 gates passaram: build, testes,
  doctests, all-targets, Clippy, fmt, rustdoc e verificações arquiteturais.
  Linter completo sem warning/error; notas informativas permanecem.
- Todas as alterações recebidas fora do proprietário/L0 foram preservadas;
  o índice de diagnósticos recebeu somente uma nova entrada. Sem commit/push.

Baseline SHA-256 `f88539918c3ad59491e3203d3241128b807ce3037c955909313d5bd4a1629332`, preservado como
`typst-before`. Novo release SHA-256 `e2fdcf17339d5cb16e5f62da6a38e04797758a7d56159e1a57cbdcdeb9a32711`.
O oráculo continua fixado em `64c5e8c02fe0394d913ead36e42ef8c1334d2107`.
Build com um job, teto de 12 GiB e zero swap; receita, Rust, recursos e tamanhos
em `release-build-receipt.json`.

Recibos/scripts: `lab/results/construction/diagnostics/image-validation-dedup-2026-10-26/`.
