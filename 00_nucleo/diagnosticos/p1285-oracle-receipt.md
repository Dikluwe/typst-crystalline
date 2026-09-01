# P1285 — receipt dos oráculos black-box independentes

**Estado:** `FROZEN`  
**Regime:** protocolo completo de materialização segregada; papel restrito de
autor dos oráculos, executado sem atestação de isolamento ambiental forte.  
**Baseline:** vanilla ratificado `a51e02804`, exclusivamente
`/usr/local/bin/typst`.  
**Instante do congelamento:** `2026-08-30T17:29:21.772163+00:00`.  
**Revalidação após restart causal:** `2026-08-30T14:45:21-03:00`.  
**Revalidação após segundo restart causal:** `2026-08-30T15:02:10-03:00`.  
**Revalidação após terceiro restart semântico:** `2026-08-30T15:27:19-03:00`.  
**HEAD hospedeiro observado:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este receipt **invalida e substitui causalmente** a revisão anterior do mesmo
caminho, SHA-256
`e7bc3b0671934e7deb9cce88051f6ab01b9a304e4619ce6838bf66b2d6933b99`.
Essa revisão v3 já havia invalidado v2
(`2737492c3f1a32dbf201ab1687883ddbf53ef028f52acf636a6f8ebe6ecccd0a`),
que invalidara v1
(`4ee6d07c38aff7a216d00ea317d5759aaa26669fbdfc2eb0f179201476e2e1b9`).
O terceiro restart foi causado pela inclusão do owner
`compiler/eval/math.md`; não por falha ou mudança dos bytes da
suíte/baseline.

Este receipt é diagnóstico. Não é Prompt L0, implementação, selo do contrato,
gate de mutações, certificado nem veredito sobre a candidata P1285.

## 1. Papel, capacidades e reinício causal

Papel exercido: **autor independente dos oráculos**. Entradas permitidas:
AGENTS, skill e duas referências diretas, Passo 1285 explicitamente autorizado,
ADRs 0107/0108/0127/0129, os nove L0 finais indicados, receipt do contrato e
fonte/binário vanilla ratificados. Escritas permitidas somente em:

- `lab/surface-inventory/run_p1285_oracles.py`;
- `lab/surface-inventory/p1285-oracle-baseline.json`;
- este receipt.

Não foram lidos código ou diff candidatos cristalinos. Não foi executado nem
lido `target/release/typst`. Não foi escrita implementação. O estado global da
working tree não foi usado como entrada do oráculo e não foi auditado, pois
isso violaria a restrição de capacidade; a reprodutibilidade da medição vem dos
hashes exatos da runner, do baseline e do binário externo pinado.

Durante a autoria, `shell/cli.md` e `infra/query-helpers.md` mudaram para
clarificar o transporte de labels. O congelamento foi interrompido antes de
qualquer escrita, os cinco L0 foram relidos e repinados, e a derivação reiniciou
a partir das entradas finais. O caso `query_label_figure_json` verifica a nova
obrigação: o elemento interno conserva `func + fields`, recebe `label`, não vira
`func: label` e não altera cardinalidade.

Antes da candidata, `wiring.md` foi atualizado para tornar explícitos o
transporte de `QueryFormat` e a fonte markup transitória quando `input == "-"`.
Essa nova entrada protegida invalidou o receipt acima. O owner adicional foi
lido integralmente e repinado; os outros cinco pins foram revalidados. A runner
já exercia todas as queries por stdin `-`, anunciava/selecionava JSON e YAML e
comparava semanticamente esses formatos. Portanto não houve lacuna observável,
e runner/baseline foram preservados byte a byte antes da nova execução vanilla.

No segundo restart, os três L0 novos/alterados foram lidos integralmente e os
oito pins foram revalidados. A runner já continha sete testemunhas públicas de
show literal/regex — incluindo splice por múltiplas ocorrências, distinção
literal/regex e fronteira de nós — e os três erros públicos de selector vazio.
O contexto recebido sobre uma candidata não foi usado para escolher, remover ou
alterar casos; nenhuma saída, código ou diff candidato foi observado. Como a
cobertura já correspondia integralmente aos L0, runner/baseline foram novamente
preservados byte a byte.

No terceiro restart, `compiler/eval/math.md` foi lido integralmente e os nove
owners foram repinados. A runner já continha `query_equation_json` sobre a fonte
`$ x^2 $`: o snapshot semântico preserva a base como
`{func: symbol, text: x}` e o expoente como `{func: text, text: 2}`. Isso
discrimina a obrigação pública Grapheme→MathIdent/Number→MathText sem exigir a
representação Rust interna. Nenhum caso ou byte de baseline foi alterado.

## 2. Entradas congeladas

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `tekt-materializacao-segregada/SKILL.md` | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `materialization/typst-passo-1285.md` | `5a69234ba5340c92e7cfe523212b102869e8a7fcefafb4647e4acf1dc3690a6c` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| `prompts/shell/cli.md` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `prompts/compiler/eval/rules.md` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` |
| `prompts/compiler/eval/math.md` | `7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2` |
| `prompts/compiler/stdlib/foundations/selector.md` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` |
| `prompts/infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `diagnosticos/p1285-contract-receipt.md` | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |

Retificação documental pós-adendo de precisão: o pin acima identifica o
receipt do contrato atual, que incorpora `CONTRACT_V4_PRECISION_ADDENDUM`. O pin
anterior,
`9897a6f893734b66b2d1c93ee653bc1155ab0c51f705c89f22011ee73a3c77ec`,
permanece registrado apenas como proveniência histórica do contrato anterior ao
adendo. Esta retificação não regenerou nem modificou runner, baseline, testes,
produção ou contrato, não reexecutou a suíte e não altera a evidência congelada
de 42/42 `Preserved`, zero `Violated` e zero `Unknown`.

### 2.1 Drift mecânico de selo

Entre v3 e v4, os SHA-256 completos de três L0 mudaram exclusivamente porque
`Hash do Código` foi ressellado. Isso foi verificado reconstruindo em stream
somente o valor anterior dessa linha: os hashes resultantes reproduziram
exatamente os pins v3. Nenhuma outra linha foi revertida ou ignorada.

| L0 | SHA-256 completo v3 | SHA-256 completo v4 | SHA-256 normativo sem `Hash do Código` |
|---|---|---|---|
| `compiler/eval/selector_matching.md` | `fd7a0b50308e119932d9fee90f9aa972d4a6ebab55ad31b15aa9decead59cae6` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` | `557abd0a6fb52f8b198606e928ba23fd65234e3b75c2a14703b07a9975ba1b11` |
| `compiler/eval/rules.md` | `c97c2f6fc0275e37667b1d183ada5437e523ab3248b2549536da7b038bba875f` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` | `3655f2922835fd0bfcf0a1857a6d12d70ac9c30eae23f9f7b9eea1f69d9fc7c0` |
| `compiler/stdlib/foundations/selector.md` | `6849e281548fd239321dab6fdf31a9a6473ac8bd7a9bc6d7fd8a78c4be63d7d8` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` | `9bea0284d242754ca1103baf45c53b7614817cfff1e5358fd708980f971f7c22` |

O hash completo continua a identificar os bytes efetivos; o hash normativo é
registrado apenas para provar que esse drift específico não alterou a obrigação.

Fontes vanilla decisivas permaneceram as pinadas no receipt do contrato:
`typst-cli/{args.rs,eval.rs,main.rs,query.rs}`,
`foundations/{value.rs,content/mod.rs,selector.rs}` e
`typst-realize/src/lib.rs`. O oráculo executável é identificado por:

| Artefato | SHA-256 |
|---|---|
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| `lab/surface-inventory/run_p1285_oracles.py` | `08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16` |
| `lab/surface-inventory/p1285-oracle-baseline.json` | `5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7` |

## 3. Forma da suíte

A runner contém o corpus Typst e congela somente contra o caminho e SHA-256
ratificados. Cada caso preserva separadamente:

- exit code;
- stdout em bytes e SHA-256;
- stderr em bytes e SHA-256;
- valor semântico normalizado quando estruturado.

JSON usa parser com rejeição de chaves duplicadas; YAML usa `safe_load` do
PyYAML. Objetos são comparados semanticamente, arrays preservam ordem e floats
não-finitos recebem forma canônica. Saída raw é comparada byte a byte, incluindo
`00 0a ff 41`. Ausência do parser YAML, timeout, binário inacessível ou baseline
incoerente produz `Unknown`; `Unknown` nunca é sucesso e o processo termina não
zero.

Cada execução faz dois passes: o segundo em ordem inversa e exige a mesma tripla
stdout/stderr/exit. Relações adicionais exigem equivalência semântica entre
JSON/YAML, neutralidade byte-exata de `--pretty` em YAML e igualdade entre query
por kind e por label da figure.

## 4. Cobertura observável

Os 42 casos congelados cobrem:

- inventário exato de formatos: eval `json|yaml|raw`, query `json|yaml`, defaults
  e rejeição de formatos inválidos com exit 2;
- compostos JSON/YAML, `Version`, symbol, bytes, `Content`, float não-finito e
  fallbacks públicos por `repr` (auto, length, color, label, datetime, decimal,
  duration, function, type, gradient, tiling, regex e selector);
- raw de string e bytes como bytes exatos, além da rejeição nominal de Version;
- query de heading, figure, equation e metadata rotulados; JSON/YAML; transporte
  de label; ordem; `--field`; `--one`; cardinalidade 0/2 e field ausente; todos
  esses casos entregam a fonte pelo argumento público stdin `-`; a equation
  sentinela preserva folhas distintas para `x` e `2`;
- show literal e regex: repetição, match parcial, metacaractere literal versus
  regex, não-match e fronteira entre nós — sete testemunhas;
- negativos de selector vazio, regex vazia/que casa vazio e query textual/regex
  não locatável; os três primeiros são as testemunhas diretas do owner do
  constructor.

Os casos de query bem-sucedidos exigem warning de deprecação em stderr e stdout
estruturado limpo. Erros de query foram medidos sem esse warning e são validados
pelas mensagens semânticas correspondentes.

## 5. Comandos, resultados e estado

Congelamento:

```sh
python3 lab/surface-inventory/run_p1285_oracles.py \
  --typst /usr/local/bin/typst \
  --freeze \
  --baseline lab/surface-inventory/p1285-oracle-baseline.json
```

Resultado: `FROZEN`, 42/42 `Preserved`, zero `Violated`, zero `Unknown`.
Foram duas passagens em ordens opostas; todas as relações internas passaram.

Reprodução após o congelamento:

```sh
python3 lab/surface-inventory/run_p1285_oracles.py \
  --typst /usr/local/bin/typst \
  --baseline lab/surface-inventory/p1285-oracle-baseline.json
```

Resultado: `Preserved=42, Violated=0, Unknown=0`, exit 0.

Após a inclusão de `wiring.md`, o mesmo comando foi reexecutado exclusivamente
contra `/usr/local/bin/typst` em `2026-08-30T14:45:21-03:00`. Resultado:
`Preserved=42, Violated=0, Unknown=0`, exit 0. A runner continuou com SHA-256
`08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16`
e o baseline com SHA-256
`5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7`.

Após o segundo restart, o mesmo comando foi reexecutado exclusivamente contra
`/usr/local/bin/typst` em `2026-08-30T15:02:10-03:00`. Resultado:
`Preserved=42, Violated=0, Unknown=0`, exit 0. Os hashes da runner e do
baseline permaneceram, respectivamente,
`08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16`
e `5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7`.

Após o terceiro restart, o mesmo comando foi reexecutado exclusivamente contra
`/usr/local/bin/typst` em `2026-08-30T15:27:19-03:00`. Resultado:
`Preserved=42, Violated=0, Unknown=0`, exit 0. A runner permaneceu com SHA-256
`08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16`
e o baseline com SHA-256
`5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7`.

## 6. Limites da alegação

`FROZEN` significa que a suíte e o baseline vanilla estão imutavelmente
identificados e reproduzíveis no fragmento P1285. Não significa mutation score,
selo do contrato, equivalência funcional geral ou aprovação de implementação.
Nenhuma candidata foi observada. O gate discriminatório com mutantes e o
veredito contra uma implementação pertencem a autoridades posteriores.
