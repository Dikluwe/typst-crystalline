# Passo 1501 — completion Nushell

**Estado:** PLANEADO; gate humano ADR-0127 pendente antes de código produtivo.

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

## Trabalho futuro e gate

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
