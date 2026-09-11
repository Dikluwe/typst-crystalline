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

## P1339 — transporte da base nativa aprovada

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`rules.rs:2403-2408` delega Value::Selector ao conversor; `:476-480` e
`:829-838` reconhecem somente NodeKind::Par nas rotas de parágrafo.
As fixtures e recibos `p1339-full-show-final-*` medem aplicação real de
strong/emph/text no vanilla ratificado `a51e02804`; os recibos fixam binários,
working tree e UTC. A limitação CLI dos perfis cristalinos está separada em
`p1339-full-a2.md` e não constitui evidência positiva de matching.

### Decisão de integração

Manter a delegação única de Value::Selector para selector_matching, incluindo
Element. NativeElement e Where com essa base seguem a travessia de nós; os
caminhos literais/Regex/Label continuam distintos. Não recriar filtros ou
reconhecer função pelo nome neste owner. A receita recebe o nó morfológico
correto; não texto concatenado dos descendentes nem fatia de regex.

As duas seleções especiais de regras de parágrafo devem reconhecer também
NativeElement da função par, inclusive seus filtros Where, por predicado
interno tipado do owner selector_matching. Não ampliar genericamente o
caminho de parágrafos a outros elementos ou a todas as composições antigas.
Preservar os momentos de síntese/aplicação já existentes, sem mudar fase ou
aplicar regras de texto/elementos novamente por causa do caminho especial.

Show-set, precedência, RuleId, guardas, ponto-fixo e teto mantêm seu contrato.
Testes devem distinguir transformação efetiva de simples repr não realizado,
cobrir match/miss/vazio e coexistência de bases antiga/nova. Um caso que exija
mudar fase ou informação pública adicional reabre o gate; não se resolve
por aplicação extra. A extensão de dados foi aprovada no P1339; selo e RED
ainda são condições anteriores ao código.

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
