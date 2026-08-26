# P1208 — aplicação transacional de hashes

## Resultado

O lote global foi aplicado com sucesso. O linter terminou a transação com
`0 drift warnings remaining`; a execução posterior em `--dry-run` respondeu
`Nothing to fix` e exit 0. V5, V7, V15 e V26 estão zerados.

## Proveniência

- HEAD: `7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568`.
- Início: `2026-08-26T09:37:34-03:00`.
- Verificação final: `2026-08-26T09:42:35-03:00`.
- Working tree não commitado, acumulando P1181–P1208.
- Digest final do diff rastreado:
  `370331e629a1e1f0eb97f0f749739fb7b7895f60f954a7567bce47a29fddf1b9`.
- Stat final rastreado: 701 ficheiros, 703 inserções e 695 remoções.
- Índice Git vazio.

## Saneamento exigido pelo linter atualizado

A primeira tentativa real não escreveu nada: o preflight recusou metadata que
não ocorria exatamente uma vez na região canônica. A investigação da versão
instalada do linter mostrou que:

- `Hash do Código` só é canônico no preâmbulo anterior à primeira linha vazia;
- 46 prompts proprietários tinham um único hash válido, mas fora dessa região;
- 8 prompts não tinham a linha; outros 4 usavam texto pendente/travessão em vez
  de oito hexadecimais;
- 4 consumers ainda tinham `@prompt-hash 00000000`.

As 46 linhas foram movidas byte-identicamente para imediatamente após o título.
As 12 lacunas receberam metadata temporária canônica e os quatro seals neutros
foram obtidos focalmente pelo V5. Nenhum corpo normativo ou produtivo mudou
nessa preparação. Só depois de um dry-run com exit 0 o fix real foi executado.

## Gates finais

- `crystalline-lint --fix-hashes --dry-run .`: `Nothing to fix`, exit 0.
- `crystalline-lint --checks v5,v7,v15,v26 .`: zero violações.
- Linter completo: somente V16=210, V17=36, V18=2, V19=349, V20=600 e
  V21=24, todos fora deste lote.
- `cargo build`: GREEN, com warnings de compilação preexistentes.
- `git diff --check`: GREEN.
- Índice Git: vazio.

P1208 fecha o eixo de drift/hash. Os próximos passos podem tratar V16–V21 por
classe sem carregar V5 ou bloqueios estruturais.
