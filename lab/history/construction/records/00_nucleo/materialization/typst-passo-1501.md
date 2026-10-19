# Passo 1501 — completion Nushell

**Estado:** IMPLEMENTADO; gate humano ADR-0127 confirmado em 2026-09-25.

## Medição antes da decisão

P1497 mediu `typst completions nushell` no vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`: exit 0, script
não vazio em stdout e stderr vazio. No cristalino pós-P1496:
exit 2, `invalid value 'nushell'`. `completions bash` é controle
positivo bilateral (exit 0, script não vazio). A diferença é de
aceitação e geração pública, não de igualdade dos bytes do script.
Proveniência completa no diagnóstico P1497.

O L0 `00_nucleo/prompts/shell/completions.md:25-43` ainda enumera
somente bash, elvish, fish, powershell e zsh; seu consumer é
`02_shell/src/completions.rs`. A enumeração de argumentos reside
também em `02_shell/src/cli.rs`, proprietário de
`00_nucleo/prompts/shell/cli.md`. Não criar um segundo owner
compartilhado: auditar e atualizar cada prompt somente para seu
consumer, respeitando ADR-0129.

## Plano e gate (executados)

1. Repetir a sonda para Nushell, Bash e shell inválido; verificar
   determinismo e estrutura do script, sem exigir byte-paridade entre
   versões de `clap_complete`.
2. Atualizar os L0s proprietários afetados antes de código e
   **PARAR** para confirmação humana da ampliação da sintaxe pública
   e dos hashes (ADR-0127).
3. Após confirmação: testes RED de parsing/dispatch/geração, implementação
   em L2/L4 sem I/O em L1, resselo após V15/V26, build, lint e
   sonda bilateral GREEN.

**Aceitação:** `completions nushell` produz script não vazio em stdout
com exit 0 e stderr vazio, não cria arquivos, `bash` continua válido
e shell inválido continua a falhar no parsing.

## Execução e validação

O humano confirmou em 2026-09-25 os dois L0s públicos antes de testes ou
código: shell/cli.md SHA-256 bruto
`4168f2480c701e2bfb1ab9cd806566088c3b1443c3db1ba704334700c1e163fa`
e shell/completions.md SHA-256 bruto
`986ab54cb32edaff76f3b2aa0c1dec9497c35c8339e9f6cbfb10e23c1441a189`.
Após o resselo mecânico, os SHA-256 brutos passaram a
`64608e749a937b219ec94c193b0e541221a9b13d612d4861ca875b45ab0cbb31`
e `20bbfa00396d0ff43f01b8d579f85f1f8b08024dc3e23131e3adb1c915012482`.

O RED ocorreu em 2026-09-25: snapshot às 14:02:18Z, HEAD
`2accc929159f3a744d4965400bdbb47d34d33e17`, working tree não
commitado com shell/cli.md (+55), shell/completions.md (+105 líquidos),
02_shell/src/cli.rs (+18) e 04_wiring/tests/cli.rs (+57);
`git diff HEAD --stat` totalizou 205 inserções e 30 remoções.
O teste unitário P1501 terminou exit 101: nushell era valor inválido.
Às 14:03:25Z, no mesmo snapshot de HEAD e quatro ficheiros, o teste
externo Nushell também terminou exit 101, com exit 2 em vez de 0.

A implementação produtiva ficou em L2: CompletionShell enumera os seis
valores públicos em cli.rs e completions.rs mapeia cada um ao gerador
oficial sobre a árvore clap real. O ramo Nushell usa
clap_complete_nushell 4.6.2; seu requisito elevou clap a 4.6.6 no
Cargo.lock, com as atualizações transitivas resolvidas offline.
L4 produtivo, L3 e L1 não mudaram.

O GREEN focado passou para o teste unitário e o teste externo Nushell.
A suíte L2 completa passou 87/87. A primeira corrida completa da CLI
passou 104/105: Clap 4.6 mudou a anotação visível do alias watch de
`[aliases: w]` para `[alias: w]`; o comando w permaneceu funcional.
O teste passou a aceitar ambas as grafias somente na linha de watch,
sem relaxar o teste de execução do alias. O L0 proprietário da suíte
(wiring/tests/cli.md) recebeu essa obrigação e a prova de Nushell;
após V15/V26, foi ressellado com Hash do Código `8d62ac6e` e
@prompt-hash `6e763ffe` (SHA-256 bruto
`7c08f111e6eba7ef9b9f430f8241c50cc7e23ebb1785481a5a0910d766ae8905`).
A segunda corrida completa da CLI passou 105/105.

`cargo build --offline --quiet`, `crystalline-lint .`,
`crystalline-lint --checks v5,v6,v15,v26 --fail-on warning .` e
`git diff --check` terminaram exit 0. V15/V26 passaram antes dos
resselos; `crystalline-lint --fix-hashes --dry-run .` terminou
com `Nothing to fix` após eles.

A sonda bilateral final foi registrada em 2026-09-25T14:20:08Z, HEAD
`2accc929159f3a744d4965400bdbb47d34d33e17`, working tree não
commitado com shell/cli.md, shell/completions.md, wiring/tests/cli.md,
02_shell/Cargo.toml, 02_shell/src/cli.rs, 02_shell/src/completions.rs,
04_wiring/tests/cli.rs, Cargo.lock e Cargo.toml; `git diff HEAD --stat`
deu 344 inserções e 55 remoções. Os binários foram o vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8` (SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`)
e o cristalino target/debug/typst (SHA-256
`71cb185fd49afc169ad2c16a7dfcaa121ef4767a57432d7f7e5a48a6810826e5`).
Receita: duas invocações por binário/caso sob LC_ALL=C, NO_COLOR=1,
cwd e HOME/XDG isolados, capturando status/stdout/stderr e entradas
criadas. Nushell saiu 0, stderr vazio e script estrutural válido
(vanilla 16 263 bytes, cristalino 14 478 bytes); Bash saiu 0 com
script não vazio; invalid-shell saiu 2 e stdout vazio. Todos foram
determinísticos no mesmo build e não criaram ficheiros. Não se exigiu
igualdade de bytes entre geradores.
