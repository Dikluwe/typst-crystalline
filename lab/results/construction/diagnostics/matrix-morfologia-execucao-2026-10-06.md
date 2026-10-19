# D07 — morfologia de mat materializada

Estado: nome público, identidade de func(), rows/células e presença dos campos
corrigidos no recorte medido. Não se certifica paridade universal da matemática.

## Medição antes da decisão

`matrix-morfologia-2026-10-06/before.json.gz` e `additional-before.json.gz`
registram entradas, argv, cwd, ambiente, saída, status, UTC, HEAD, diffstat e
hashes das fontes. Antes, a projeção JSON era `{func: "math.matrix"}`, perdia
rows e células, fields() era vazio e func() não era a nativa mat.

Fonte pinada: `lab/typst-original/crates/typst-library/src/math/matrix.rs:91–221`
declara os campos; `:404–417` canoniza augment; `:434` converte o stroke.
A presença é distinta dos defaults de render: rows é conhecido sempre;
delim, align, augment e os dois gaps só são conhecidos quando fornecidos.
`gap` é shorthand, não campo. A canonização de augment refutou a necessidade
de um campo novo para preservar a entrada bruta.

A coleta intermediária `final.json.gz` com binário Tekt e7424d07a20d5d2517ba9903b483eb7624712e3840ecd73daecb3d700f4cb5d4
mostrou outro resíduo: stroke de comprimento acrescentava pintura preta na
projeção. Esse achado foi medido antes de editar o L0 do conversor. Os flags
já existentes em Stroke preservam a presença; a correção local marca somente
thickness para Length e paint para Color. Nenhum contrato público novo.
As coletas `final-after-stroke` substituem a coleta intermediária para fechamento.

## Implementação e linhagem

L0s atualizados antes de cada implementação, em fluxo contínuo ADR-0127.
Nove pares L0/consumer ressellados exclusivamente; oito consomem o novo núcleo
`00_nucleo/prompts/_nuclei/math/matrix-morphology.toml`. `seals.json` contém
os hashes completos de cada arquivo e os selos; grafo V15/V26 válido antes
do resselo e linter integral válido depois dele. Sem resselo global.

O getter de MathMatrixElem projeta rows e campos explícitos. Somente a projeção
das células Empty usa sequence vazio; o carrier e o padding de layout ficam
conservados. O catálogo de campos e o acesso distinguem declarado desconhecido
de nome inexistente. func() devolve a nativa real por identidade estática;
o shell serializa recursivamente o conteúdo. O ajuste de stroke limita-se à
conversão de augment de mat e conserva os outros usuários de extract_stroke.

## RED→GREEN e validação

`core-fixture-valid-red.log` reconstrói o owner content_methods do snapshot
baseline, conservando o teste corrigido, e falha na enumeração de fields.
`core-fixture-green.log` e `shell-fixture-green.log` passam no código corrigido.
As primeiras tentativas com retorno Module e expectativa incorreta de delim
foram corrigidas: o constructor Rust da fixture fornece delim explicitamente.
Essas tentativas não são usadas como prova de RED válido.

`stroke-valid-red.log` falha porque thickness não era marcado. O teste passa
na suíte core final. `stroke-red.log` tinha erro de compilação de fixture e
não é prova de RED. Logs e fontes anteriores permanecem preservados.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot `final-code.json.gz`
registra diffstat, status, conteúdo e hashes de todas as fontes/L0 alteradas.
`final-results.json` liga medidas, UTC, binários e hashes dos logs.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt release final SHA-256
`b5362b800d175487c22071db5fda63f6a2cedb129ca22591b7bfbf998ac1c9ad`.

- matrix: 28/29 controles integrais coincidem; UTC 2026-10-06T21:23:18.403964+00:00 a 2026-10-06T21:23:19.933292+00:00. Evidência: `/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/final-after-stroke.json.gz`.
- matrix-additional: 12/14 controles integrais coincidem; UTC 2026-10-06T21:23:20.277013+00:00 a 2026-10-06T21:23:21.199519+00:00. Evidência: `/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/final-after-stroke-additional.json.gz`.
- D05: 29/29 controles integrais coincidem; UTC 2026-10-06T21:23:18.343951+00:00 a 2026-10-06T21:23:23.596412+00:00. Evidência: `00_nucleo/diagnosticos/equation-where-2026-10-06/final-matrix.json.gz`.
- query: 36/36 controles integrais coincidem; UTC 2026-10-06T21:23:23.997300+00:00 a 2026-10-06T21:23:29.387074+00:00. Evidência: `00_nucleo/diagnosticos/query-compilada-2026-10-06/final-matrix.json.gz`.
- D06: 38/41 controles integrais coincidem; UTC 2026-10-06T21:23:30.661143+00:00 a 2026-10-06T21:23:41.857283+00:00. Evidência: `00_nucleo/diagnosticos/math-bare-2026-10-06/final-matrix.json.gz`.

No recorte D07 válido, os 28 controles principais e os 12 adicionais passam.
Os três controles inválidos/negativos estão explicitados abaixo.
Testes: core final 6.574 passaram e 5 ignorados; shell 90, infra 1.194 e CLI
query 10 passaram. Shell/infra/CLI foram executados antes do ajuste local de
presença do stroke; core, builds e todas as coletas bilaterais foram repetidos
após esse ajuste. Builds debug/release concluídos; git diff --check limpo.
Linter final: zero erros, 534 warnings e 1.593 notes. Não se afirma ausência
de avisos. O receipt de preservação confirma zero alteração fora dos nove
pares e do núcleo desta frente nas fontes/L0 desde baseline.

## Limites explícitos

O controle query-options usa none/left sem # dentro de math. Vanilla rejeita
none e Tekt aceita; o controle query-options-valid usa #none/#left e mede a
morfologia válida. A divergência sintática continua aberta.

augment:() fornece array inválido. Os dois controles correspondentes rejeitam
com a mesma mensagem, mas o span Tekt abrange o argumento enquanto o vanilla
marca o valor. O diagnóstico de span permanece aberto.

Em D06, root-render, raw-eval e raw-mat-eval permanecem negativos integrais.
A forma de mat está corrigida; raw-mat-eval ainda difere pelo aviso de arquivo
vazio. Não há fechamento de render, sintaxe ou diagnóstico por esta frente.

Próxima frente: D08, preservando a distinção entre query de table.cell não
locatable, selector válido e transformação por show.
