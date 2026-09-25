# Passo 1500 — valor vazio de --font-path no CLI

**Estado:** IMPLEMENTADO; gate humano ADR-0127 confirmado em 2026-09-25
para o L0 `00_nucleo/prompts/shell/cli.md` com SHA-256
`c4a7504db45f5330664358c904fdc4ecaaff5eed86f0bd5610f925d24f370e3a`.

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

## Plano e gate (executados)

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

## Execução e validação

O humano confirmou o L0 acima antes de qualquer teste ou código produtivo.
O contrato foi implementado somente em L2: os dois parsers privados guardam
`Option<PathBuf>` por token, e `CompileIntent`/`FontsIntent` recebem a
sequência filtrada de paths não vazios. L3/L4 e os intents públicos não mudaram.

O RED foi executado em 2026-09-25 com snapshot às 11:58:04Z: HEAD
`5d2798dc473346ea7169d1bf171425db734056ad`, working tree não
commitado com apenas `00_nucleo/prompts/shell/cli.md` (60 linhas de delta) e
`04_wiring/tests/cli.rs` (+195 linhas); `git diff HEAD --stat` totalizou
247 inserções e 8 remoções. O comando
`cargo test -p typst-wiring --test cli p1500_ --offline -- --nocapture`
terminou exit 101: quatro testes falharam no parser de valor vazio e o
controle da flag sem argumento passou. Após a mudança L2, os cinco
testes P1500 passaram; o teste unitário adicional confirmou ordem e
duplicatas de diretórios não vazios no intent.

A suíte CLI completa passou 104/104 testes; a suíte L2 final passou
85/85. `cargo build --offline --quiet`, `crystalline-lint .`,
`crystalline-lint --checks v5,v6,v15,v26 --fail-on warning .` e
`git diff --check` terminaram exit 0. V15/V26 passaram antes do resselo;
`crystalline-lint --fix-hashes .` atualizou somente `cli.rs` e o
`Hash do Código` do L0 para `7227d084`; o header final usa
`@prompt-hash 3c8d1348`. SHA-256 bruto do L0 após resselo:
`ac3a2d99ff5fedd83e85db8390abfa80e820a09b2c5da9b53113ef70b1067b13`.

A sonda bilateral final usou o snapshot de 2026-09-25T12:17:15Z:
mesmo HEAD, working tree não commitado com
`00_nucleo/materialization/typst-passo-1500.md` (54 linhas de delta),
`00_nucleo/prompts/shell/cli.md` (62 linhas de delta),
`02_shell/src/cli.rs` (55) e `04_wiring/tests/cli.rs` (+195);
`git diff HEAD --stat`: 349 inserções, 17 remoções. SHA-256 do
cristalino `target/debug/typst` =
`509a3de9ab945b3f52c15ad720752e2795090087bb5abee16385aa2074a095c5`,
vanilla pinado `lab/typst-original/target/release/typst` =
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Receita: `env -u TYPST_FONT_PATHS <bin> fonts --ignore-system-fonts`
(no vanilla, também `--ignore-embedded-fonts`), variando flag omitida,
`--font-path ''`, `--font-path=`, `TYPST_FONT_PATHS=''`, diretório real
`03_infra/fixtures/fonts`, flag vazia sobre env real e flag sem valor.
Nos dois binários, os quatro casos vazios e a omissão saíram 0 e
produziram a mesma listagem dentro do respectivo perfil; diretório
real saiu 0 e alterou a listagem; flag sem argumento saiu 2. Não se
compararam bytes das listagens entre perfis de fontes diferentes.

`cargo fmt --all --check` continua a falhar por formatação anterior
fora do delta P1500; nenhuma reformatação global foi aplicada.
