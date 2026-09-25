# Prompt L0 — comando `typst completions`
Hash do Código: b41bb03c

**Camada:** L2
**Ficheiro alvo:** `02_shell/src/completions.rs`
**Estado:** adendo P1501 proposto; gate ADR-0127 pendente

O parsing e o carrier público pertencem a
`00_nucleo/prompts/shell/cli.md` (`02_shell/src/cli.rs`). O dispatch
existente pertence a `00_nucleo/prompts/wiring.md`
(`04_wiring/src/main.rs`). Este prompt legitima somente o gerador L2,
sem ownership compartilhado (ADR-0129).

## Medição anterior à decisão

Na medição histórica P1137, o vanilla gerava o script a partir da
própria árvore clap; o Bash continha o comando `query` oculto no help.
O cristalino de então já usava `CommandFactory`, mas ainda não tinha
o subcomando. `hide` no help não implica remoção da completion.

Em 2026-09-25T12:28:14Z–12:29:26Z, no HEAD cristalino
`2accc929159f3a744d4965400bdbb47d34d33e17`, working tree
limpa antes/depois das sondas, duas execuções `LC_ALL=C` por caso
foram determinísticas. O vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8` (binário
`lab/typst-original/target/release/typst` SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`)
aceitou `completions nushell`: exit 0, stderr vazio, stdout não vazio
(16 263 bytes), com `module completions {`, `export extern typst`
e entradas de compile, fonts, completions, watch e query. O cristalino
`target/debug/typst` (SHA-256
`509a3de9ab945b3f52c15ad720752e2795090087bb5abee16385aa2074a095c5`)
rejeitou `nushell` no parser (exit 2). `bash` gerou script não vazio
com exit 0 em ambos; shell inválido terminou exit 2 em ambos.

A fonte vanilla
`lab/typst-original/crates/typst-cli/src/args.rs:282-315` usa
enum próprio de seis shells e delega Nushell a
`clap_complete_nushell::Nushell`; o gerador recebe a árvore
`CliArguments::command()` em
`lab/typst-original/crates/typst-cli/src/completions.rs:8-12`.
No cristalino, `02_shell/src/completions.rs:6-11` já gera da árvore
real, mas recebe `clap_complete::Shell`, cujo enum instalado não
inclui Nushell. `Cargo.lock:354-382` fixa `clap` 4.5.60 e
`clap_complete` 4.6.9; o gerador Nushell 4.6.2 disponível localmente
exige `clap` 4.6.6 ou posterior. O bump compatível do lockfile é
mecânica da dependência, não critério de byte-paridade do script.

**Classificação:** aceitação do valor é sintaxe pública (owner CLI);
o script é saída pública do gerador, cuja estrutura deve seguir a
árvore clap corrente. Bytes de versões distintas de geradores não
são a unidade de paridade (ADR-0107).

## Contrato do gerador P1501

`generate(shell: CompletionShell) -> Vec<u8>` consome o carrier tipado
de `00_nucleo/prompts/shell/cli.md`. Para bash, elvish, fish,
powershell e zsh, conserva os geradores oficiais de `clap_complete`;
para nushell, usa `clap_complete_nushell::Nushell`. O mapeamento é
exaustivo; não há template manual nem árvore de comandos paralela.
A fonte única dos scripts é `crate::cli::command()`, construída
de `Args::command()` sem filtro próprio; `query` oculto pode
permanecer como no vanilla.

`clap_complete_nushell` entra somente nas dependências L2. Se a
versão exigir atualizar `clap` e `Cargo.lock`, preservar a sintaxe
dos cinco shells existentes e validar regressão; nenhuma dependência
entra em L1. L3 não participa e `04_wiring/src/main.rs` mantém o
dispatch e a escrita existentes sem alteração produtiva.

O script é não vazio, determinístico entre duas execuções do mesmo
build e produzido em memória. L4 escreve-o integralmente em stdout;
stderr fica vazio no sucesso. Nenhum ficheiro ou configuração do
utilizador é criado ou alterado; erro de escrita em stdout continua
a devolver exit não zero no owner L4.

## Testes e aceitação

Verificar Nushell estruturalmente (`module completions {`,
`export extern typst`, comandos correntes), stdout não vazio,
stderr vazio e determinismo. Regressão Bash e controle de shell
inválido pertencem também à suíte externa, sob o contrato de parsing
de `00_nucleo/prompts/shell/cli.md`. Não exigir igualdade dos
bytes entre vanilla e cristalino nem fixar um snapshot textual
de `clap_complete_nushell` entre versões.

## Gate

P1501 amplia a sintaxe pública e muda a assinatura pública do
gerador; confirmar este L0 e
`00_nucleo/prompts/shell/cli.md` com seus hashes antes de testes
RED, dependências ou código (ADR-0127).
