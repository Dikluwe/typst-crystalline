# P1360 — preflight de `compile --pretty`

## Proveniência

- medição: `2026-09-12T23:15:50-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- estado: working tree não commitado; `git diff HEAD --stat` registava 11
  ficheiros, 663 inserções e 122 remoções, provenientes de P1356–P1359;
- vanila ratificado: commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino medido: `target/debug/typst`, SHA-256
  `53fab14eb8708083610c88c4e65a6c4946c3bdd62f0524c5fe38b0bbde7664f3`.

## Segurança da medição

Somente o subcomando terminante `compile` foi executado. Cada processo foi
limitado por `timeout 20s`, stdout/stderr foram redirecionados para ficheiros e
nenhum `watch`, alias `w`, teste filtrado ou captura ilimitada foi iniciado.
Os artefactos medidos somaram menos de 1 MiB; o maior foi o controlo PNG de
936603 bytes.

## Medição antes da decisão

A fonte do vanila em `typst-cli/src/args.rs:329-340` declara `pretty:
Option<bool>` com bare igual a `true`, no máximo um valor e valor explícito
exigindo `=`. `compile.rs:643-684` encaminha a opção separadamente a PDF,
HTML e SVG e não a encaminha a PNG.

Uma medição corretiva posterior mostrou uma ambiguidade que a leitura da fonte
sozinha não capturou. Às 2026-09-12T23:57-03:00, no mesmo HEAD e com o mesmo
binário vanila pinado acima, `timeout 20s /usr/local/bin/typst compile
/tmp/p1360-probe.typ --pretty false` saiu com código 1 e informou que não podia
inferir o formato para o caminho `false`. Logo `require_equals` não transforma
o token separado em valor booleano nem obriga erro de parsing: o bare continua
`true` e `false` fica disponível como `output` posicional. O diretório finito da
medição foi `/tmp/p1360-vanila-separated.LLrvec`; stdout teve 0 bytes e stderr,
111 bytes. Esta observação substitui a inferência anterior de que a grafia
inteira teria necessariamente de sair com erro equivalente ao clap.

Na fixture finita `p1360-probe.typ`, ausência e `--pretty=false` produziram a
mesma morfologia compacta em SVG (0 quebras de linha, 8852 bytes) e HTML (0
quebras, 191 bytes). `--pretty=true` produziu SVG indentado (53 linhas, 9111
bytes) e HTML indentado (10 linhas, 221 bytes). No PDF, ausência/false
produziram 151 linhas e 7589 bytes; true produziu 506 linhas e 11287 bytes. A
fonte `typst-pdf/src/convert.rs:55-70` confirma que pretty desliga compressão
de content streams, usa representação ASCII-compatible e ativa a escrita
legível. PNG com `--pretty=true` terminou em sucesso sem receber opção de
formatação no exporter.

No cristalino, SVG já possui `SvgOptions.pretty`, mas a pipeline fixa
`SvgOptions::default()`. HTML só possui serialização compacta. PDF comprime
content streams em `builder.rs:489-505`, independentemente de `StreamMode`.
`StreamMode::{Verbose,Compact}` controla operadores PDF e permanece eixo
distinto; não é sinónimo de pretty.

## Decisão

P1360 materializa `compile --pretty[=BOOL]` em PDF, SVG e HTML, com ausência e
`false` compactos e `true` legível. PNG aceita a opção e permanece byte/morfologia
neutro. `watch` e `w` reutilizam a estrutura de argumentos, mas só podem ser
verificados pelo parser in-process; nenhum processo contínuo integra o gate.

No PDF, o recorte observável é: content streams de página ficam sem
`FlateDecode` em pretty e seus operadores permanecem legíveis, sem acoplar a
decisão a `StreamMode`, tagging ou compressão de fontes/imagens. Não se alega
igualdade byte a byte, identidade de objeto ou replicação integral do serializer
krilla. Em HTML e SVG, pretty altera somente whitespace estrutural; DOM/grafo,
texto, atributos, escapes e geometria permanecem iguais.

O pedido explícito do dono para escrever e implementar o próximo passo em
2026-09-12 confirma o gate ADR-0127 para a nova flag e o novo campo público.
