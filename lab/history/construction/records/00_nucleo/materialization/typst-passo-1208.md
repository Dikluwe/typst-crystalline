# P1208 — aplicar o lote transacional de hashes L0

**Estado:** EXECUTADO — GREEN (2026-08-26)
**Dependência:** P1207 GREEN, com preflight determinístico desbloqueado.
**Classe ADR-0127:** reparo mecânico de linhagem; não altera contrato público,
default, fase de pipeline nem comportamento de produto.

## Baseline reproduzível

- Hora: `2026-08-26T09:37:34-03:00`.
- HEAD: `7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568`.
- Working tree não commitado; `git diff HEAD --stat` registra 9 ficheiros
  rastreados, 9 inserções e 9 remoções, além dos ficheiros novos acumulados que
  o stat sem `--no-index` não contabiliza.
- Digest do diff rastreado: 
  `6751900d7bd8bac01e930d5dc15002f1745fdb0b5f76cb5af1142ad87f61cee4`.
- Índice Git vazio.
- V7=0, V15=0, V26=0 e V5=311.
- O valor inicialmente tratado como digest do preview era o digest da mensagem
  de preflight rejeitado; o pipe havia mascarado exit 2. A execução corrigiu
  essa prova antes de qualquer escrita.

## Execução

1. Confirmar novamente V7/V15/V26=0 imediatamente antes da escrita.
2. Executar uma única vez `crystalline-lint --fix-hashes .`.
3. Não editar corpos produtivos nem Prompts L0 durante o lote: somente metadata
   `@prompt-hash` e campos `Hash do Código` que o reparador transacional governa.
4. Rodar o mesmo comando em `--dry-run` depois da escrita; deve produzir lote
   vazio e não modificar a árvore.
5. Confirmar V5=0, V7=0, V15=0 e V26=0.
6. Executar `cargo build`, a suíte focal do linter disponível,
   `git diff --check` e confirmar índice vazio.
7. Registrar relatório P1208 com contagem antes/depois, output do reparador,
   idempotência, testes e proveniência final.

## Fronteira

V16–V21 permanecem fora do lote: são avisos heurísticos e exigem correções
semânticas próprias. Se o reparador tentar alterar qualquer coisa além da
metadata que declara no preview, interromper e não normalizar manualmente.

## Aceitação

P1208 é GREEN quando o reparo real termina, um segundo dry-run não propõe
mudanças, V5/V7/V15/V26 ficam zerados, build e diff-check passam e o índice
continua vazio.
