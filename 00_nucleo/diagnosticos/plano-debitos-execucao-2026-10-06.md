# Plano de débitos — execução inicial, 2026-10-06

Foi consolidado o inventário em plano-debitos-2026-10-06.md, com classes,
frentes, decisões vigentes, owners e critérios de saída. Cache-inventory.md
registra chaves, lifetime e invalidação; proxima-medicao.md define os controles
antes de qualquer API de retenção/incremental. O plano amplo permanece em curso.

## Correções realizadas

A CLI exige output explícito para stdin antes de qualquer aquisição do documento
ou derivação do output. Preserva -o/--output, positional, precedência da flag
nas duas ordens e default derivado para arquivo físico. O primeiro RED é
output-red.log; o refinamento de usage perdeu inicialmente o nome typst e
mostrava opcionais, foi medido, ganhou RED em output-usage-red.log e corrigido
somente na árvore de apresentação do erro. Output-final.log mantém GREEN.

L1 passa a reconhecer a função nativa math.equation pela mesma identidade
estática das funções já mapeadas. O hub oferece bridge interno pub(super)
somente para o subtree stdlib. Um teste valida parse/native_selector, alias
com nome diferente e outra função com o nome math.equation; equation-red.log
falha antes do reparo e equation-green.log passa depois. Equation-cli.log
observa equações já existentes por query/selector/alias. Não houve novo enum,
assinatura pública, unsafe ou fase. Somente os seguintes owners foram atualizados e selados:

- 00_nucleo/prompts/shell/cli.md → 02_shell/src/cli.rs.
- 00_nucleo/prompts/wiring/tests/cli.md → 04_wiring/tests/cli.rs.
- 00_nucleo/prompts/compiler/stdlib/foundations/selector.md → 01_core/src/compiler/stdlib/foundations/selector.rs.
- 00_nucleo/prompts/compiler/stdlib/structural.md → 01_core/src/compiler/stdlib/structural/mod.rs.

Ownership/núcleos foram verificados antes de seal.py. Seals.json conserva o
escopo e hashes bidirecionais; nenhum reparo global foi executado.

## Proveniência e validação

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitada.
Coleta entregue começou em UTC 2026-10-06T17:35:10.429556+00:00 e terminou em
2026-10-06T17:35:12.877669+00:00. Delivered.json.gz e math-delivered.json.gz conservam
git diff HEAD --stat/status, hashes de arquivos alterados, fontes, cwd,
comandos, stdout/stderr/status e observações PDF. Binários:

- Tekt release: 243ecb5384d98826aaa815cddbf3b6a93b1eedbbed01cc2cecb5c0a5ee008221.
- Vanilla: 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin 9f2b6e8715237cb086899a42873660fe744622e8,
  árvore/receita ratificadas P1489.

Core-shell.log: 6554 testes L1 passaram e cinco foram ignorados; não houve
mudança de L1 depois dessa coleta. Shell-final.log: 87 testes L2 passaram
após o refinamento de usage. Cli-final.log: 118 testes CLI passaram no estado
entregue, com loopback autorizado para watch/HTTP. As coletas distintas têm
seus logs e snapshots; não constituem soma de denominadores de paridade.
Builds debug/release passaram; diff-check passou. Linter final: zero erros,
528 warnings e 1578 notes; owners alterados sem V5/V6/V15/V26. V5 herdados fora
do escopo permanecem, sem resselo para ocultá-los. Não é zero violations global.
Preservation.json verifica que fontes/L0s anteriores fora do escopo não mudaram.
Execution-state-final.json.gz conserva o estado dos arquivos entregue.

Summary.json mantém as diferenças cruas e os critérios: output obrigatório tem
status/corpo correto, mas stderr usage ainda diverge; invocation implícita é
extensão do Tekt, não paridade com subcomando do vanilla. Os três controles de
query/selector/alias de equation coincidem integralmente. Matriz explícita
preserva texto, mas corpo JSON ainda perde rows/cells; .where, matemática bare
e table.cell continuam separados e abertos. Não houve benchmark de desempenho
nem novo percentual global de paridade ou reexecução da matriz macro histórica.

## Gate pendente e classificação corrigida

ADR-0045 adendo P1139 §8 exige preservar warnings → errors. A coleta nova e a
fonte pinada fazem errors → warnings. Alterar a decisão existente exige revisão
concreta; o código da ordem foi preservado. O pacote em
00_nucleo/materialization/warnings-cli-2026-10-06.md inclui L0s completos e
manifesto de hashes para nova ordem e função pública L2
query_deprecation_hint(&QueryIntent) -> String. A aprovação geral do plano
não ratifica essa assinatura ainda não apresentada. A confirmação ADR-0127
foi solicitada e continua pendente; nenhum código dependente foi escrito.

A auditoria refutou table.cell como elemento consultável: vanilla rejeita
query(table.cell) por não ser locatable, aceita selector(table.cell) e aplica
show table.cell. Também invalidou os primeiros testemunhos de render com
argumentos que não eram Content; eles foram preservados como rejeições, e a
análise positiva foi refeita com mat([a])/frac([1],[2]). Não promover enum/API
ou aceitar query indiscriminadamente a partir da descrição antiga do débito.

## Atualização após confirmação do pacote warnings

O gate pendente descrito acima é histórico: o dono respondeu confirmo, com
recibo warnings-cli-aprovacao-2026-10-06.json. A materialização e validação
estão em warnings-cli-execucao-2026-10-06.md. D02 corrigido; D03 fecha origem
direta/import, com callback contextual ainda aberto. O plano amplo continua.
