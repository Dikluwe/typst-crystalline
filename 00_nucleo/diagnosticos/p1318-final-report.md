# P1318 — localizar o erro dentro do CSV fornecido em Bytes

Estado: concluído no recorte P1318; testes, build, A/B e revisão independente aprovados.

## Efeito implementado e limites

O diagnóstico `found 1 instead of 2 fields in line 2` já informa qual registro
falhou, mas antes não trazia a posição dentro dos dados. P1318 acrescenta
`at linha:coluna` em `csv(Bytes)`, sem alterar o ordinal, a causa Utf8 ou a
origem do argumento que recebe o sublinhado. With/Args e origem detached
também conservam essa distinção nos testes locais.

A posição é a que o parser fornece ao vanilla ratificado, não uma localização
inventada do byte ruim. Por exemplo, `a,b\r\n1` indica `at 1:5`, pois o offset
fica entre CR/LF. O buffer inválido usa outra conversão e pode indicar `1:1`
para o mesmo prefixo CRLF. Não apresentar essas peculiaridades como posição
intuitiva corrigida: a obrigação é reproduzir o diagnóstico de referência.

Path/Str e o decoder público puro permanecem sem o sufixo novo. Diagnóstico
de arquivo, coerções Symbol, unknown/missing/excesso e csv.encode ausente
continuam pendentes. Parsing com positional excedente mantém sua precedência
legada; o sufixo nesse caso é efeito normativo, não paridade com vanilla que
rejeita excesso antes. Não há promessa de paridade geral CSV.

## Medição que motivou a decisão

`p1318-measurement.json`, SHA-256
`c19f749ad22b06e07d3078d6d9a72017e3c2e1f2b2546f738042f34362302378`,
registra 70 observações bilaterais, expressões/bytes e saídas completas.
A sonda com caminho absoluto é resolvida como caminho virtual e dá I/O;
não serve como prova de parsing Path. Casos Bytes cobrem LF/CRLF/CR,
multilinha, vazios, BOM, separadores Unicode, Utf8 e precedência.

`p1318-measurement-columns.json`, SHA-256
`5e8b0b29ca91a1ba0c951e33a4016d06ff3ca954f8e97826e1ecb6f118eb4e97`,
acrescenta oito observações: a coluna `2:5` após caracteres multibyte,
inclusive emoji, refuta contar bytes/UTF-16 ou sempre retornar coluna 1.

Na fonte ratificada `a51e02804`, `loading/csv.rs:138-157` escolhe offset do
parser e fallback ordinal/1. `diag.rs:845-925,1025-1035` distingue validade
do buffer inteiro; `typst-syntax/src/lines.rs:88-95,252-274` e
`lexer.rs:1144-1152` definem linhas de texto e CRLF. A medição com byte inválido
posterior a UnequalLengths impede escolher a conversão pelo tipo de erro.
No owner cristalino `loading.rs:944-1006,1304-1319`, a posição se perde antes
da composição Bytes. Mensagem é observável da linguagem (ADR-0108).

## Proveniência e divisão de trabalho

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree P1315/P1316/P1317
não commitado. A medição inicial preserva fonte/L0 completos, hashes dos
diagnósticos anteriores, diff/stat, status e UTC. Recibos registram estados
antes/depois; não atribuir o diff acumulado inteiro ao P1318.

Baseline `/tmp/p1317-target.y5u9ah/release/typst`, SHA-256
`9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab`.
Vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`, não inferido da string de versão.
Target exclusivo `/tmp/p1318-target.eAgQwp`, cópia de cache sem hardlinks;
RAM livre insuficiente para cópia completa. Nenhum baseline sobrescrito.

L0 atualizado antes de testes/código. Regime A/B **sem atestação técnica de
isolamento**: root escreve L0/testes locais/implementação; testador independente
`/root/p1318_tests` não lê código candidato; `/root/p1318_review` julga sem
editar artefatos julgados. Filesystem é compartilhado. Sem selo geral,
mutation score ou equivalência funcional ampla. Unknown bloqueia; duas
revisões sem ganho na mesma causa exigem rever desenho.

## Testes e gates

Revisão `p1318-review-preflight.md`: GO de classe ADR-0127 e escopo, com
delegação privada no mesmo owner e API pública intacta. V15/V26 passaram;
recibo `p1318-lineage-preflight.json`, SHA-256
`8a3eb13048a580167835f0d1e27acee83b1fa7b09f61ba918535d562379054a8`.

Expectativas antigas mudaram **antes do candidato**, pois agora o texto
Bytes deve conter posição. Manifesto `p1318-test-delta.json`, SHA-256
`bfd34662ab8f82365127eacd264a88a5581064788de84e4cd5aff01f8920716f`:
seis testes atualizados, nenhum removido; mantidas causas e asserções de
origem/hints/traces. A mensagem pura antiga recebeu uma asserção adicional.
Três testes novos cobrem posições, origem distinta e preservação pure/Path.
A revisão conferiu o delta contra os snapshots antes do candidato; o prefixo
produtivo ainda era literalmente o baseline.

RED genuíno do módulo loading: oito falhas exclusivamente pela ausência do
sufixo (seis testes atualizados e dois novos), 59 testes passam. Recibo
`p1318-unit-red.json`, SHA-256
`4a746ed84042f69917f25b58bef1e6eea417b957e1bb5cf74422f3b099cef910`,
exit 101; fonte/L0 estáveis. O controle novo de decoder puro/Path já passa.

Antes do freeze, o writer do A/B corrompeu oito saídas com U+2028 ao usar
splitlines() para gerar o patch de JSON. Isso não foi falha do produto.
O bruto foi preservado; o writer passou a separar somente LF, e o reparo foi
cotejado com oito reexecuções focais. Nenhuma expectativa mudou; recibos
específicos do A/B preservam a cadeia, os bytes, hashes e custo.

Freeze `p1318-ab-freeze.json`, SHA-256
`aede13464afe720b4dffe501a9cab59f22f92a6101c12e9be7878c083abcb334`:
513 casos, quatro perfis, 2.052 expectativas e 788 RED. Os 387 replays P1317
coincidiram integralmente antes da transformação. Nas classes paritárias,
baseline acrescido somente do sufixo já coincide integralmente com vanilla;
as colisões de excesso mantêm expectativas normativas separadas.
Revisão pré-patch aprovou o RED e o freeze reparado antes da implementação.

## Implementação e validação do candidato

O decoder continua com um único parser. `decode_csv_impl` recebe uma opção
privada de diagnóstico, habilitada somente pela rota nativa Bytes. A conversão
do offset consulta a validade do buffer inteiro somente após uma falha do
parser; não antecipa Utf8 nem muda o erro vencedor. O decoder público delega
com essa opção desligada. A remarcação de origem do argumento permanece
separada da linha/coluna interna dos dados.

Identidade estável do candidato, sobre o HEAD e working tree acima:

- fonte SHA-256 `870f861651c35f44afd5f3f68548bb0364463b2ca3d4d2087b5460bdbc986f5a`;
- L0 bruto SHA-256 `8cfa574ea121eb053598a5a45b7d54d4c1546ed920f519a3e2341b833f3e8246`;
- L0 normativo congelado `2b23a5adf4c7b62afbfb5ee9b8899bb756f36c2eb158c2ddb7e2795e49312ee6`
  (exclui somente a linha canônica Hash do Código);
- linhagem A `cf196a42`, B `ae096892`.

GREEN: 67 testes passam, zero falhas/ignorados. Recibo
`p1318-unit-green.json`, SHA-256
`0c8e81b7213ffc5d88e744552f84005956aa471244fc02728c8ac63a75a2682c`.
O comando começou em `2026-09-08T15:35:55.515580+00:00`; fonte/L0 antes e
depois coincidem com as identidades acima.

Gates estáticos, todos com exit 0 e a mesma fonte/L0 antes/depois:

| Recibo | Resultado | SHA-256 |
|---|---|---|
| `p1318-lineage-final.json` | dry-run: Nothing to fix | `c30e06978c10b88ff44b6a57789b0c2a08ed1e1aa69b37787fa87273e55c148b` |
| `p1318-lint.json` | 0 errors, 240 warnings, 1.137 infos | `feb752598600aa68506781741961e8a99b35d2f92270e4ccf2d4fd60c68300e7` |
| `p1318-fmt.json` | cargo fmt --all --check | `755b9841efcdd681387ba178bbf486782eb0de225cab73547fa6e44ddcb5de82` |
| `p1318-diff-check.json` | git diff --check | `27133c6e57a2e00cef1e2c05ebca216eaf5b708a330c4060d0e6c37f2d91a3d8` |

Os avisos e infos do lint não equivalem a uma árvore sem dívida.

Build `cargo build --workspace --release`: exit 0, recibo
`p1318-build.json`, SHA-256
`d6ca0c8fe77669f8be94d6fbebe8cb003a9e7e402c0aa1747ddb2b733fbc8fe0`.
Executado entre `2026-09-08T15:39:43.583825+00:00` e
`2026-09-08T15:41:12.632764+00:00`, fonte/L0 estáveis.
Binário `/tmp/p1318-target.eAgQwp/release/typst`, SHA-256
`0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c`.
Baseline e vanilla foram re-hasheados e permanecem intactos.

`p1318-review-candidate.md` aprovou o código sem achado bloqueante. O revisor
reconstruiu o source RED pelo diff arquivado e verificou que o módulo inteiro
de testes não mudou entre RED e candidato. Confirmou também a equivalência
da substituição UTF8 usada no ramo binário, consultando `utf8_iter` local.
Workspace `cargo test --workspace --release --no-fail-fast`: 6.659 testes
passam, zero falhas, três doctests ignorados; exit 0. Recibo
`p1318-workspace-tests.json`, SHA-256
`1b9ca1c3cb4032a51b16dcb9c489980a0d083d5d7f756c2ccabef94dd066c2dc`.
Executado entre `2026-09-08T15:40:43.107238+00:00` e
`2026-09-08T15:45:45.177481+00:00`, com fonte/L0 estáveis.

A/B executado sobre o binário identificado acima: 6.156 comparações em
normal/repeat/reverse, zero falhas e zero Unknown. As 2.052 expectativas do
freeze são verificadas em cada ordem, nos perfis default/html/a11y/html+a11y.
São 788 expectativas de alteração do sufixo e 1.264 de preservação literal.
Das 788, 756 coincidem integralmente com vanilla; 32 colisões com excesso
posicional são normativas e não atestam essa paridade bilateral.

`p1318-ab-candidate-runs.json`, SHA-256
`c5907504691f3d2f524804624b3db343058e2db93d4004422332e8ecba168ef0`,
preserva comandos, saídas e proveniência. A comparação em
`2026-09-08T15:46:32.311001+00:00`, `p1318-ab-comparison.json`, SHA-256
`9ed11dfe247403eb6fd58942c4a774693e6fdc105b4ba425b2051667b686f1a1`,
registra PASS e revalidação das entradas congeladas. O custo medido do lote
foi 240,932 segundos; não é benchmark de desempenho do produto.

## Fechamento e decisão

PASS independente em `p1318-review-final.md`, SHA-256
`d3e838c9f26a7d0da8762b9a06d102282c3d842c73078278469a591dad842e97`.
A auditoria de `2026-09-08T15:47:51.052Z`,
`p1318-review-final-audit.json`, SHA-256
`49a57df740bdee3089d3567ca4cf85a6899af91cb2fd027184fe1acfcf7a3ac3`,
reconstruiu todas as comparações sem diferenças, ausências ou duplicatas;
confirmou testes intactos desde o RED e integridade de fonte/L0, inputs,
binário e artefatos anteriores. Sem achado bloqueante.

O recibo `p1318-ab-receipt.md`, SHA-256
`bc09dcbf6de75c879f439d9a93b992202dcac62ed02c83bf4060c33c835f378c`,
traz os comandos de reprodução com saídas novas, os custos e o incidente
do writer. A skill `tekt-materializacao-segregada` determinou a separação
entre implementação, expectativas congeladas e veredito; essa divisão não
é apresentada como isolamento técnico do ambiente compartilhado.

Fecha-se a posição textual dos erros de parsing em CSV Bytes, não o CSV
inteiro. A melhora concreta é mostrar onde começa o registro indicado pelo
parser, segundo a conversão ratificada, sem perder qual registro falhou nem
qual argumento originou a chamada. Os limites descritos no início continuam
abertos; em particular, Path/Str não recebeu o sufixo desta etapa.

Sem stage, commit, push, limpeza de temporários ou alteração dos artefatos
P1315/P1316/P1317. L0 e fonte finais conservam as identidades validadas.
