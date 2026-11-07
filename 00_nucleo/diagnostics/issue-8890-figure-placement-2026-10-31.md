# #8890 — Figure placement e continuidade de bloco

O usuário aprovou o contrato de FigurePlacement e seus dois campos opcionais
em 2026-10-31. A materialização na branch compartilhada Tekt mantém o default
in-flow; auto/top/bottom capturam o frame uma vez e reservam a sucessora antes
do texto. A admissão de texto revalida sucessoras reduzidas pelo float, com
guarda de progresso. Mapas/getters e congelamento de contexto preservam a
presença lexical e a resolução. Não houve commit/push nem outra branch/worktree.

O vanilla ratificado64c5e8c0 perde D no original; o Tekt anterior rejeitava
placement. O corpus corrigido conserva A–E em PDF/SVG, inclusive D com link;
o controle com legenda também conserva E. Não se copia a perda de linha nem
se fixa a paginação defeituosa do oracle como aceitação.

Evidências, aprovação, estados exatos/UTC/hashes/dirty stat, RED→GREEN, comandos
e limites RAM estão no [relatório](../../lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/report.md).
A [lista exata de arquivos](../../lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/changed-files.json)
individualiza os27 pares L0/consumidor e o novo núcleo declarativo.

A validação é do corpus/contrato declarado, sem certificação geral de Figure,
scopes/footnotes/HTML ou de todas as frentes concorrentes do checkout. Clippy de
produção passa; o check que inclui os testes encontra aviso preexistente em
grid/row_groups.rs, fora desta tarefa e inalterado desde seu início.
