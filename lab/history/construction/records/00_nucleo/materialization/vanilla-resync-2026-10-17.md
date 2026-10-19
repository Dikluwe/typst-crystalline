# Re-sync explícito da referência vanilla — 2026-10-17

Autorização: pedido humano «Commite, e atulaize a referencia vanilla para a mais atual».
Escopo: commit do trabalho validado; consulta oficial do upstream; atualização do
snapshot de quarentena, pin fixo do CI e ferramentas de paridade; construção e
recibo verificável; revalidação; segundo commit local. Sem push ou release.

O checkpoint anterior é `e2d7f6cdc93c5f6b948ca80167411d390ccc714d`.
O pin medido em HEAD/main oficial é `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
árvore `cac836c07e4dec2a89219370428292393a3216b5`. Substitui o pin P1489
`9f2b6e8715237cb086899a42873660fe744622e8`. O snapshot e a receita de coleta
estão em `00_nucleo/diagnosticos/vanilla-resync-2026-10-17/`.

Preservar o overlay do extrator e seu lockfile; conferir cada arquivo upstream
por hash de blob. A identidade do novo binário inclui commit, árvore, receita,
lockfile, compilador e SHA-256. A string de versão é auxiliar; sondar sys.version
antes de decidir sobre a superfície de linguagem. Não alterar contratos públicos
do compilador neste re-sync; registrar diferenças de linguagem para etapas próprias.

Prompts e relatórios antigos continuam vinculados às medições originais. Atualizar
a documentação da referência vigente, sem trocar pins históricos em massa.
Comparadores e esperados não se relaxam para acomodar o novo oráculo. Relatar
as falhas encontradas e separar diferenças novas das pendências já conhecidas.

Jobs pesados sequenciais, CARGO_BUILD_JOBS=1, MemoryHigh=10 GiB,
MemoryMax=12 GiB, MemorySwapMax=0. Capturar UTC, HEAD, status, diffstat,
hashes e comandos. Respeitar a pausa do objetivo geral após esta entrega delimitada.
