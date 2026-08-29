# P1243 — medir o contrato existente de tiling SVG

**Estado:** EXECUTADO — PRESEAL INVALIDADO; FRONTEIRA NEGATIVA RETIDA  
**Predecessor:** P1242  
**Saída:** universo modelável pelo contrato atual, sem ampliar `TilingBody`.

Comparar vanilla e cristalino para tiling de cor, imagem e gradient usando apenas
campos já públicos: size, spacing, relative e corpo representável. Medir pattern
units, bbox, repetição, offset implícito, transforms, fill/stroke e alpha.
Conteúdo arbitrário, angle ou offset ausentes ficam `CONTRACT-GAP`, não convite
a inventar API.

Congelar fixtures, orçamento e ataques antes do candidato. Se o universo atual
se limitar a cor com tamanho, registrar isso e avançar; nenhuma mudança pública
é autorizada durante execução desacompanhada.

## Tentativa histórica v1

O probe não foi executado. Oito ataques sobreviveram, incluindo geometria
inventada, opacity ausente, spacing/relative/transform não provados, role
reduzido a booleano, fallback genérico e equivalência de construtor por mero
sucesso de compilação. P1244 fica pausado.

O preseal foi materializado em
`00_nucleo/diagnosticos/p1243-{tiling-contract,tiling-observables,tiling-preseal-ataques}.tsv`,
`p1243-tekt-manifesto.tsv` e `p1243-tekt-preseal-receipt.tsv`. Score
`1/9 = 0.1111111111`; nenhum valor runtime foi admitido e nenhum ficheiro
produtivo ou Prompt L0 foi alterado por P1243.

## Tentativa histórica v2

Uma segunda tentativa separou autor de contrato, autor de oráculos, adversário
e verificador em agentes sem contexto herdado. O contrato v2, 15 oráculos e 28
ataques foram congelados por hash antes de o verificador receber a candidata.
O verificador rejeitou o preseal com `17/28 = 0.6071428571`: onze expectativas
dos ataques contradiziam as próprias condições `Unknown`/`CONTRACT-GAP` do
contrato; três positivos não chegaram a SVG; e os metamórficos não provaram
determinismo semântico. A candidata também viola C02 ao mapear
`relative=self` para `objectBoundingBox`.

Os subprocessos de probe foram confinados por `bwrap`, mas os agentes
continuaram com filesystem compartilhado e permissões amplas do workspace.
Hashes e ausência de writes proibidos foram verificados; capacidade isolada
não foi provada. Veredito de isolamento: `false`. P1244 permanece pausado.

## Tentativa histórica v3

Após o então alegado selo P1241, o contrato foi refeito sem leitura da candidata e limitado
ao universo realmente executável pelos tipos públicos vigentes. Expectativas
que contradiziam `Unknown`/`CONTRACT-GAP` foram corrigidas; positivos SVG sem
fixture compartilhada executável deixaram de ser tratados como sucesso.

Contrato, 20 oráculos e 34 casos foram congelados antes da verificação. O
verificador acertou `34/34` classificações e confirmou os hashes antes
e depois. Dois controles do construtor cristalino passaram duas vezes em ordem
direta e inversa. O vanilla rejeita ambos, portanto eles provam apenas o
construtor cristalino atual, não paridade compartilhada nem morfologia SVG.

O universo SVG adicional provado por P1243 é vazio: corpo Image, Gradient,
conteúdo arbitrário, size auto, relative parent, stroke e offset/angle
permanecem `CONTRACT-GAP`; os observáveis SVG de Color sem fixture bilateral
válida permanecem `Unknown`. P1244 pode avançar para encerramento sem código,
salvo nova evidência selada.

Esse preseal histórico foi posteriormente invalidado porque entradas protegidas
mudaram e porque `34/34` não é mutation score: 19 casos terminaram `Unknown`,
10 `CONTRACT-GAP`, três `PASS` e somente dois `FAIL`.

## Saneamento atual

O auditor determinístico revalidou os artefatos v3 contra P1242 e os L0 atuais.
Detectou drift no passo/inventário P1242 e nos prompts `entities/tiling.md` e
`compiler/stdlib/tiling-stdlib.md`. Pelo protocolo Tekt, isso invalida o selo.

O resultado substantivo estreito continua válido: nenhum caso SVG adicional foi
provado `Preserved`. P1244 deve encerrar sem código, salvo nova evidência
bilateral. Não há mutation score atual nem autorização de implementação.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`: autoridades e artefactos históricos foram
segregados, mas executores reutilizados conservaram metadados anteriores e o
filesystem permaneceu compartilhado.
