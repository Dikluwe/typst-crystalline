# Prompt L0 — `compiler/eval/rules`
Hash do Código: ed0454b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/rules.rs`

## Medição e contrato

Avalia set/show rules, valida argumentos e aplica selectors com ordem e
terminação determinísticas. Show-set transporta Styles; transformações
preservam morfologia. Recursão termina por ponto-fixo morfológico ou teto
diagnosticado. Named desconhecido nunca é ignorado.

## Aceitação

Selectors, regras, warnings, casts, composição, recursão e spans seguem
`a51e02804`.

## P1286 — `#set smartquote`

### Medição anterior à decisão

O caminho vigente aceita `enabled` e apenas `quotes` string/auto/none em
`rules.rs:1881-1939`; o vanilla expõe ainda `alternative` e as formas
array/dict em `text/smartquote.rs:38-89,335-419`.

### Decisão

O set-rule aceita `enabled: bool`, `alternative: bool` e `quotes` no domínio
canônico P1286. Valida antes de inserir o delta; named desconhecido continua
nunca ignorado. `none` conserva apenas a compatibilidade cristalina vigente e
é normalizado como `auto`; não é alegado como superfície vanilla. O carrier é
o `Styles` existente, sem novo campo público ou fase.

## P1285 — show rules textuais por ocorrência

### Medição antes da decisão

O caminho literal já fatia cada `Content::Text` e chama a recipe para cada
ocorrência. O caminho regex anterior apenas testava `is_match` e entregava o
nó inteiro uma vez. No vanilla ratificado, `#show regex("f.o")` sobre
`foo fxo` chama a recipe para `foo` e `fxo`, e `regex(".")` sobre `a.b`
produz três matches; literal `"."` produz somente o ponto. Matches vazios são
rejeitados no constructor e nunca entram em loop.

### Decisão

O loop dedicado de regex usa `selector_matching::splice_regex_rule_matches`
para todas as transformações `Func`/`Content`/`Str`, entregando à recipe
somente a fatia morfológica casada. Preserva texto não casado, ordem,
revogação por `RuleId`, precedência existente e fronteira de nós. O loop
literal permanece dedicado e `is_node_rule(Text|Regex)` continua falso;
nenhum dos dois selectors passa a viajar pelo matcher de elementos.

Aceitação: literal repetido/parcial, regex repetida, distinção literal `.` vs
regex `.`, no-match e fronteira entre nós coincidem semanticamente com
`a51e02804`.
