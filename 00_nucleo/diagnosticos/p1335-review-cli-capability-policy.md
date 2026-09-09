# P1335 — duas rejeições públicas no suplemento de warning

Medição anterior à decisão: `p1335-transversal-r3.json`, cases `html-legacy` e
`preserve-pdf`, conserva argv, binário SHA-256 ratificado, stdout vazio, exit 2 e
stderr integral nos três percursos. A primeira rejeita o path da fonte como
subcommand, com Usage de `typst [OPTIONS] <COMMAND>`; a segunda rejeita
`--document-id`, incluindo tip, Usage completo de compile e instrução de help.
Nenhuma alcança o export/warning no vanilla. O controle cristalino alcança o
comando autorizado e seu contrato próprio é avaliado separadamente.

O leitor de R3 só reconhecia exit 0/1 para esse suplemento, portanto preservou
essas seis observações como incomplete. `p1335-review-transversal-full-r3.json`
registra explicitamente esses Unknown; o dado original não deve ser sobrescrito.

Decisão focal do leitor: um transcript exato e autenticado desses comandos é
`PUBLIC_CLI_ABSENT`, diferença de capacidade CLI. Não é MATCH de warning nem do
PDF/HTML. Não inferir existência de artefato vanilla depois da rejeição. Exigir
binário correto, comando reconstruído integralmente e stdout vazio; no legado,
o argumento rejeitado precisa ser exatamente a fonte compilável pinada; no PDF,
a flag e seu valor devem constar do comando que causou a rejeição.

Controles congelados antes do predicado: cada transcript genuíno deve produzir
PUBLIC_CLI_ABSENT; remover stderr, truncá-lo, trocar por erro genérico, simular
crash, inserir stdout ou trocar o comando deve produzir Unknown. Ausência de
warning nesses casos não viola o contrato cristalino nem satisfaz paridade.
Essa exceção é restrita a estas duas rejeições, assim como a política de query;
exit 2 não se torna genericamente uma diferença conhecida.

Uma projeção sucessora pode ler os mesmos canais já medidos e declarar a causa,
sem repetir todo o corpus. Ataques só em cópias dos dados, sem mutation score
produtivo e sem aumentar os 14 ataques planejados. Esta correção de leitura não
deve ocultar os Unknown do leitor anterior nem a perda dos PDFs focais R0.
