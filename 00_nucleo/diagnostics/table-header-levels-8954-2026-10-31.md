# Table header levels — issue 8954

O pacote público foi aprovado por `autorizo` e materializado com L0s proprietários,
reseal restrito e RED→GREEN. O caso oficial agora conserva `0 A B C 1` sem panic.
Grupos lexicais distintos mantêm nível/repeat; um posterior substitui ativos de
nível igual/maior e `repeat:false` não restaura os substituídos. A emissão preserva
as bordas uniformes de #8777 também com repetição física por página.

O [relatório completo](../../lab/results/construction/diagnostics/issue-8954-2026-10-31/result.md)
contém os limites, comandos, identidade do oracle e receipts de working tree
não commitido, HEAD/diffstat/UTC, fontes/L0 e executável. `after.json.gz` e
`validation.json.gz` são a proveniência dos resultados finais; não houve commit,
branch, worktree nem reseal global de mudanças concorrentes.

Resultado registrado: core 8.284 aprovados; infra 1.586 aprovados e uma falha
concorrente de #8583 (`side stroke lost`, fora do escopo). Não declarar a suíte
global verde. Build, probes PDF/SVG/HTML, raster de bordas, lint sem erros/warnings
e linhagem reversa global estão aprovados nos receipts finais.

Contextos em Table, rowspan com alturas automáticas, gutters, linhas explícitas
ricas e associações PDF/UA não recebem certificação geral. Efeitos de células
ocorrem por membro lógico, conforme o pacote aprovado: o probe tem treze no
Tekt contra quinze no oracle; esta é uma divergência semântica explícita.

Oracle fixado: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, tree
`483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256
`22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`.
Os números acima pertencem ao estado/horário dos receipts, não apenas à versão.
