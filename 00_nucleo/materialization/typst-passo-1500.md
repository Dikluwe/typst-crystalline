# Passo 1500 — valor vazio de --font-path no CLI

**Estado:** PLANEADO; gate humano ADR-0127 pendente antes de código produtivo.

## Medição antes da decisão

P1497, no pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`,
executou `typst fonts --font-path '' --ignore-system-fonts`: vanilla
saiu 0 sem stderr; cristalino saiu 2 com
`a value is required for '--font-path <DIR>'`. O controle
`typst fonts --ignore-system-fonts` saiu 0 em ambos. A diferença é
aceitação de sintaxe pública, não bytes da listagem. HEAD, worktree,
UTC e SHA dos produtos constam no diagnóstico P1497.

O L0 proprietário é `00_nucleo/prompts/shell/cli.md` (consumer
`02_shell/src/cli.rs`); a nota de `font_paths` em
`cli.md:780-805` especifica append, env e precedência, mas não o valor
explícito vazio. O delta upstream do parser está em
`lab/typst-original/crates/typst-cli/src/args.rs`.

## Trabalho futuro e gate

1. Repetir a sonda com flag vazia, flag sem argumento, flag com diretório
   real e `TYPST_FONT_PATHS` vazio; distinguir ausência, valor vazio
   e lista de diretórios.
2. Atualizar primeiro `00_nucleo/prompts/shell/cli.md` com o contrato
   de parsing e precedência de valor vazio, sem deslocar I/O de fontes
   para L2.
3. **PARAR** para confirmação humana do contrato público e hash
   (ADR-0127); só então testes RED, implementação, resselo após V15/V26,
   `cargo build`, `crystalline-lint .` e `git diff --check`.

**Aceitação:** flag explicitamente vazia não produz erro de parser e
tem efeito de caminho vazio medido; controles de omissão, path real e
ambiente não regridem. Não exigir identidade de bytes da listagem.
