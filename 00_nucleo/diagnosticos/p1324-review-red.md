# P1324 — RED independente confirmado

O recibo `p1324-unit-red-r2.json`, SHA-256
`0901f2121f663ba59e25ddd370050650dd3b1087686988836275c43c9567325b`,
executou cargo test typst-core release locked com filtro p1324_ entre
2026-09-09T00:12:42.811078Z e 00:14:39.203993Z, na working tree não
commitada de HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`.
Inventário produtivo, HEAD e diff antes/depois são idênticos. Proveniência
e diff/stat integrais estão no recibo; não confundir atualização de arquivos
diagnósticos untracked durante a execução com alteração de produto.

Foram executados cinco testes: três controles passaram e dois testes do
diagnóstico Some ausente falharam em asserções de mensagem esperada, exit 101.
Houve compilação e execução reais; não falha de adaptação, import ou zero testes.
O snippet integrado corresponde literalmente ao SHA-256 congelado
`b1b612f6169ae2efbb659a7018838a85493fe6e9d0f284f8388c08bdb2ae8cd8`.
Inspeção prepatch independente confirma corpo produtivo inteiro, do helper
plain_native_name ao EOF, byte-idêntico ao baseline; apenas header, snippet
e as duas expectativas Some de P1311 foram sucedidos.

A primeira execução `p1324-unit-red.json` não é RED: terminou exit 0 com
zero testes, iniciada antes da integração e atravessando mudança do source.
Ela deve permanecer documentada como tentativa inválida, nunca convertida
em sucesso do gate. R2 sucede somente a execução defeituosa.

O freeze CLI foi emitido antes de C e fixa comparações dos canais públicos,
controles de baseline distintos de paridade e limites explícitos. Calibração
usa cópias de observáveis, não mutantes produtivos. Candidato pode prosseguir
contra L0 e testes congelados; este parecer ainda não aprova GREEN ou fechamento.
Revisor escreveu somente artefatos p1324-review; A/B sem atestação de isolamento.
