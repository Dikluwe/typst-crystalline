# Caminhos dos diagnósticos — 2026-10-06

O cálculo do nome de arquivos físicos agora cobre descendentes, ancestrais e
irmãos da cwd. As cinco diferenças de caminho da matriz macro foram eliminadas;
seus diagnósticos completos coincidem com o vanilla sem projeção de caminhos.

## Medição anterior e implementação

`diagnostic-paths-2026-10-06/before.json` conserva fontes, comandos, cwd, UTC,
HEAD e git diff HEAD --stat/status. Em work/a, um erro em project/src/main.typ
saía com nome absoluto no Tekt e ../../project/src/main.typ no oracle.
`04_wiring/src/diagnostics.rs:62-66` usava strip_prefix, que só cobre filhos.
A fonte pinada `lab/typst-original/crates/typst-cli/src/world.rs:157-168` compõe
pathdiff::diff_paths. Seu lock usa pathdiff 0.2.3, checksum
`df94ce210e5bc13cb6651479fa48d14f601d9858cfe0467f43ae157023b938d3`.

O L0 foi atualizado antes dos testes/código, como correção interna de paridade
ADR-0127. O wiring compõe esse mesmo utilitário puro, pinado em =0.2.3 no seu
Cargo.toml/lock. Não acrescenta algoritmo de domínio no L4, resolução de arquivos,
canonicalização para apresentação, I/O adicional, API pública ou unsafe.
As fontes, spans, traces, cores, ordem e fallbacks anteriores são preservados.

O RED está em red.log: dois testes falham nos nomes de arquivos externos à cwd;
o controle descendente já passa. green.log conserva também a expectativa inicial
incorreta que tratava o trace como cabeçalho primário. O oracle usa uma linha
while calling broken at main.typ:2:1; a expectativa foi corrigida pela saída
medida. green-delivered.log registra os três testes focados passando, com nomes
físicos e posições verificadas. A coleta adversarial de eval permanece DIFF:
não se valida seu nome atual incorreto como requisito.

## Proveniência e validação

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitada.
A conclusão foi registrada em 2026-10-06T15:31:18.736842+00:00.
`diagnostic-paths-2026-10-06/execution.json.gz` contém sources/L0s/Cargo/lock,
lista exata de alterações, comandos nas coleções e hashes dos logs.

Tekt congelado em `/tmp/tekt-diagnostic-paths/typst-final`, SHA-256
`7c61e9b4cad5aee8995f201ce721dcce66775d222e02ccbb0aedf9e4f74a23ce`.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`lab/typst-original/target/release/typst`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O baseline anterior congelado tem SHA-256
`859abc64883582a3686a3ab5d3d537cb02bbe0e63af33ea1c52d6f3ea039d8c2`.

- **108/108 testes CLI passaram**, serializados, com loopback autorizado para watch.
  A execução completa está em cli.log. Após delimitar o teste físico, os três
  testes focados foram executados novamente em green-delivered.log; o código
  produtivo permaneceu igual.
- Builds debug/release offline concluídos em build.log/release.log.
- Linter: **zero erros, 531 warnings e 1.577 notes**. Os owners alterados não têm
  V5/V6/V15/V26; não se afirma zero violations globais.
- `macro.json.gz`: **36 MATCH em 36 casos**, contra 31 MATCH/5 DIFF antes deste
  reparo. Critérios originais por modo permanecem: valores estruturados para
  eval/query, DOM para HTML, texto/caixas/raster com tolerâncias anteriores, e
  stderr completo nos cinco casos de erro. Não significa igualdade byte a byte
  de todos os PDFs nem de warnings que o critério query não compara.
- `measurement.json.gz`: **9/15 controles** têm status e stderr completamente
  iguais. Inclui entradas absolutas/relativas, irmãos/ancestrais/descendentes,
  import/trace, Unicode/espaços e ausência de erro. Seis diferenças adicionais
  ficam explicitamente abertas. additional-gaps-baseline.json conserva sua
  reprodução no baseline anterior; não foram introduzidas pelo cálculo relativo.

Não houve novo benchmark de tempo nem nova certificação global de paridade.

## Outras defasagens encontradas

**Identidade de Source em eval contextual.**
`identity-next-audit.json` registra três controles bilaterais. Erro e warning do
arquivo de contexto perdem a localização no Tekt; erro da expressão recebe o nome
do arquivo de contexto em vez de <input-expression>. A fonte causal permanece
separada no oracle. `eval/mod.rs` constrói a expressão com world.main(), e
ExpressionWorld substitui source desse mesmo id; `wiring/eval.rs` reconstrói outra
Source com esse id e a fornece como primary para todos os resultados acumulados
por pipeline.rs. Documento e expressão têm textos distintos com identidade igual.
O L0 vigente de compiler/eval também determina world.main(): esta frente exige
revisão da especificação, não apenas alteração da legenda do formatter.

**Avisos e stdin.** A coleta preserva diferenças de aviso de depreciação/ordem em
query, ordem de aviso/erro em compile, range de aviso de stars e compile por stdin.
Os contratos atuais de compile/query legitimam a ordem existente; a decisão de
compatibilidade precisa ser revista antes de mudá-la. eval por stdin repete o
problema de identidade/nome transiente acima. Não tratar todas essas diferenças
como um único erro de caminho.

**Matemática.** `next-audit.json` mede matrix/frac construídos fora de $...$,
controles inline/block, contexto e table.cell. Fora da equação, equation.rs usa
plain_text como fallback; o oracle envolve Mathy em Equation na realização
(typst-realize/src/lib.rs:329-333). O Tekt também não oferece a equação automática
à query. Mesmo nos controles com desenho equivalente, query body de matriz no
Tekt contém apenas func math.matrix, enquanto o oracle conserva mat e rows/cells.
O contexto query(math.equation) é rejeitado no Tekt. São obrigações distintas:
realização matemática, reconhecimento do selector e morfologia serializada.
Uma troca textual de delimitadores não fecha esta frente.

**Table.cell.** Continua rejeitado como selector no controle bilateral. A entidade
NodeKind pública não contém esse caso e os adapters de células precisam ser
medidos antes da especificação da solução. A aprovação anterior de show por
ocorrência não autoriza implicitamente ampliar esse enum público.
