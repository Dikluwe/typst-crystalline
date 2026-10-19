# P1488 — retificação das extras e decisão de compatibilidade

## Proveniência

Medição consolidada em 2026-09-24 14:07:00 UTC. HEAD `e7670e5a1b97fca4e7d5ff236645720027a25669`; working tree não commitada. O snapshot a seguir foi colhido antes da criação deste relatório. Ele inclui modificações herdadas de P1484–P1486, o adendo P1474 e os dois L0 P1488; não se deve atribuir todo o diff a P1488.

Produtos SHA-256: vanilla `/usr/local/bin/typst` `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` (pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`); cristalino `target/debug/typst` `81cbcb240620cfccac5677cbab236b9609cb7209252109f8ca63ba2b64275fa2`. Extratores release: vanilla `80a04a0434252814caebe48c4d09965c205aaaabf585dce3011609aa1099946f`; cristalino `449dd2cb1757ab114eb70d7eba2525a146e89a2e1044cbfb0089e062df33fb36`. O hash do produto não certifica, sozinho, todas as alterações não commitadas.

Snapshot de `git diff HEAD --stat=200`:

~~~text
 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md  |   8 +-
 00_nucleo/diagnosticos/typst-p1474-adjudicacao-42-extras.md      | 183 +++++++++++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval.md                               | 113 +++++++++++++++++++-
 00_nucleo/prompts/compiler/eval/bindings/field_access.md         |  53 ++++++++-
 00_nucleo/prompts/compiler/eval/tests.md                         |  45 +++++++-
 00_nucleo/prompts/compiler/introspect.md                         | 113 +++++++++++++++++++-
 00_nucleo/prompts/compiler/introspect/from_tags.md               |  27 ++++-
 00_nucleo/prompts/compiler/layout.md                             |  94 +++++++++++++++-
 00_nucleo/prompts/compiler/layout/cursor.md                      |  37 ++++++-
 00_nucleo/prompts/compiler/layout/html_frame.md                  |  15 ++-
 00_nucleo/prompts/compiler/stdlib/calc.md                        |  32 ++++++
 00_nucleo/prompts/compiler/stdlib/counter.md                     |  47 +++++++-
 00_nucleo/prompts/compiler/stdlib/format.md                      |  34 ++++++
 00_nucleo/prompts/compiler/stdlib/structural.md                  |  24 ++++-
 00_nucleo/prompts/compiler/stdlib/structural/document.md         |  88 +++++++++++++--
 00_nucleo/prompts/entities/compiler_features.md                  |  33 ++++--
 00_nucleo/prompts/entities/content.md                            |  38 ++++++-
 00_nucleo/prompts/entities/counter_registry.md                   |  23 ++++
 00_nucleo/prompts/entities/layout_types.md                       |  35 +++++-
 00_nucleo/prompts/entities/locator.md                            |  41 ++++++-
 00_nucleo/prompts/infra/export/builder.md                        |  82 ++++++++++++--
 00_nucleo/prompts/infra/export/tests/mod.md                      |  10 +-
 00_nucleo/prompts/infra/integration_tests.md                     |  49 ++++++++-
 00_nucleo/prompts/infra/pipeline.md                              | 235 +++++++++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md        | 130 ++++++++++++++++++++++-
 00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md |  28 ++++-
 00_nucleo/prompts/shell/cli.md                                   |  72 ++++++++++++-
 00_nucleo/prompts/shell/watch.md                                 |  75 +++++++++++++
 00_nucleo/prompts/wiring.md                                      |  60 ++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md                            |  26 +++++
 01_core/src/compiler/eval/bindings/field_access.rs               |  34 +++++-
 01_core/src/compiler/eval/mod.rs                                 |  46 ++++++--
 01_core/src/compiler/eval/repr.rs                                |  10 +-
 01_core/src/compiler/eval/rules.rs                               |   5 +-
 01_core/src/compiler/eval/tests.rs                               | 113 +++++++++++++++++++-
 01_core/src/compiler/introspect.rs                               | 182 ++++++++++++++++++++++++++++++-
 01_core/src/compiler/introspect/from_tags.rs                     | 129 ++++++++++++++++------
 01_core/src/compiler/layout/cursor.rs                            |  12 ++-
 01_core/src/compiler/layout/html_frame.rs                        |   3 +-
 01_core/src/compiler/layout/mod.rs                               |  56 +++++++---
 01_core/src/compiler/stdlib/calc.rs                              |   2 +-
 01_core/src/compiler/stdlib/counter.rs                           |  42 ++++----
 01_core/src/compiler/stdlib/format.rs                            |  32 ++++--
 01_core/src/compiler/stdlib/foundations/query.rs                 |   1 +
 01_core/src/compiler/stdlib/structural/document.rs               | 174 ++++++++++++++++++------------
 01_core/src/compiler/stdlib/structural/mod.rs                    | 172 ++++++++++++++++++++----------
 01_core/src/entities/compiler_features.rs                        |  21 +++-
 01_core/src/entities/content.rs                                  |  44 ++++----
 01_core/src/entities/counter_registry.rs                         |  10 +-
 01_core/src/entities/layout_types.rs                             |   7 +-
 01_core/src/entities/locator.rs                                  |  27 ++++-
 02_shell/src/cli.rs                                              |  80 +++++++++++---
 03_infra/src/export/builder.rs                                   |  89 +++++++++++++++-
 03_infra/src/export/fonts.rs                                     |   2 +
 03_infra/src/export/render.rs                                    |   2 +
 03_infra/src/export/stream.rs                                    |   3 +
 03_infra/src/export/svg.rs                                       |   7 ++
 03_infra/src/export/tests/mod.rs                                 | 112 ++++++++++++++++++-
 03_infra/src/export/tests/p1481_pdfua_structure_tests.rs         |   1 +
 03_infra/src/export/tests/p1483_pdf_spot_tests.rs                |   1 +
 03_infra/src/integration_tests.rs                                |  31 +++---
 03_infra/src/layout_bidi.rs                                      |   1 +
 03_infra/src/pipeline.rs                                         | 744 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/pipeline/context_stabilization.rs                   | 339 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/shaper.rs                                           |   2 +
 03_infra/src/watch.rs                                            | 283 +++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/tests/p1297_watch_capability_contract.rs                |   4 +-
 04_wiring/src/main.rs                                            | 162 +++++++++++++++++++++++++---
 04_wiring/tests/cli.rs                                           | 649 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 69 files changed, 5117 insertions(+), 367 deletions(-)
~~~

Não rastreados no mesmo instante:

~~~text
00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
00_nucleo/diagnosticos/typst-p1486-page-labels.md
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1488.md
00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md
03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs
lab/parity/corpus/p1486-page-labels.typ
~~~

## Inventário repetido e normalização

Os catálogos vanilla, cristalino e merge foram regenerados duas vezes, sequencialmente, para `default`, `html`, `a11y-extras`, `html+a11y`, `bundle` e `html+bundle`. Todos os 18 pares de arquivos foram idênticos byte a byte. Merges sem erros de proveniência, `MISSING_BINDING`, `MISSING_MEMBER`, `WRONG_KIND` ou `UNKNOWN`. `UNVERIFIED_METADATA` continua classe separada: não é ausência nem crédito de execução.

| Perfil | EXTRA_BINDING bruto | defaults `format.*` duplicados | extras normalizadas | UNVERIFIED_METADATA | SHA-256 do merge (ambos os passes) |
|---|---:|---:|---:|---:|---|
| default | 47 | 6 | 41 | 469 | `294817aa30b80b088fe92e54c0687e3f3e1ab4e8c0b17dfec3552188d0cbc16b` |
| html | 48 | 7 | 41 | 699 | `00c7dd3bb5abb8b01450135f5e20a62e708941c6bb81603e6e3ada8d6b13d938` |
| a11y-extras | 47 | 6 | 41 | 475 | `671e3e198eeb11d5c1bb1ee0d0f8e26c009e8c0d1d2578803e9eeef5147f599f` |
| html+a11y | 48 | 7 | 41 | 705 | `820e3a1ba453ff704d0f1f52a9290d000cd410b9a0b27bb6e256ad879373b0d7` |
| bundle | 47 | 6 | 41 | 472 | `02beb12fd8ae155936cbe64439bdc40ea0bd15d25baf9a8802d415448caa207b` |
| html+bundle | 48 | 7 | 41 | 702 | `dfad700d4deed9a4770ed0bf7712eef435162de605fba5e2c4b8108715fd8032` |

Conjunto normalizado atual = conjunto histórico das 42 rotas de P1474 **menos `asset`**, sem rota nova. `asset` está corretamente gated por bundle; não é `EXTRA_BINDING`. A fotografia 48/49 → 42 do P1474 continua histórica, não é o estado atual. Os defaults `format.*` são removidos somente da interpretação do catálogo, não dos produtos. Os 41 caminhos continuam visíveis no inventário.

## Sondas antes da decisão

Comando-base: `<binário> eval '(repr(type(<rota>)), repr(<rota>))'`. Os 31 caminhos não qualificados/métodos extra medidos abaixo existem como `function` no cristalino, com `repr` igual ao nome final; os globais e `replace` são `unknown variable` no vanilla, e `calc.log10/deg/rad` dão `module calc does not contain …`. A chamada concreta de cada linha foi executada no cristalino. As 17 formas `math.*` existem bilateralmente; não se infere equivalência total de uma única chamada.

| Global math | Chamada concreta cristalina | Qualificado e limite observado |
|---|---|---|
| `accent` | `accent([x], "^")` → `accent(base: [x], accent: …)` | `math.accent`; `== true` |
| `bb` | `bb([x])` → `styled(child: [x], ..)` | `math.bb`; `== true` |
| `bold` | `bold([x])` → `styled(child: [x], ..)` | `math.bold`; `== true` |
| `cal` | `cal([x])` → `styled(child: [x], ..)` | `math.cal`; `== true` |
| `cancel` | `cancel([x])` → `cancel(body: [x])` | `math.cancel` dá mesmo conteúdo na sonda, mas função `== false` |
| `display` | `display([x])` → `styled(child: [x], ..)` | `math.display`; `== true` |
| `frak` | `frak([x])` → `styled(child: [x], ..)` | `math.frak`; `== true` |
| `inline` | `inline([x])` → `styled(child: [x], ..)` | `math.inline`; `== true` |
| `italic` | `italic([x])` → `styled(child: [x], ..)` | `math.italic`; `== true` |
| `mono` | `mono([x])` → `styled(child: [x], ..)` | `math.mono`; `== true` |
| `op` | `op([x])` → `op(text: [x], limits: false)` | `math.op` igual no cristalino; vanilla omite `limits: false` na `repr` |
| `sans` | `sans([x])` → `styled(child: [x], ..)` | `math.sans`; `== true` |
| `scr` | `scr([x])` → `styled(child: [x], ..)` | `math.scr`; `== true` |
| `script` | `script([x])` → `styled(child: [x], ..)` | `math.script`; `== true` |
| `serif` | `serif([x])` → `styled(child: [x], ..)` | `math.serif`; `== true` |
| `sscript` | `sscript([x])` → `styled(child: [x], ..)` | `math.sscript`; `== true` |
| `upright` | `upright([x])` → `styled(child: [x], ..)` | `math.upright`; `== true` |
| `underover` | `underover([x], under: [i], over: [j])` → conteúdo `under/over` | `math.underover` ausente bilateralmente |

Nos 17 pares, as chamadas amostradas global/qualificada produzem conteúdo igual no cristalino; `func == math.func` é verdadeiro para 16, falso para `cancel`. `eval/mod.rs:8387-8408` registra os globais; `structural/math.rs:2859-2860` dá a `math.cancel` nome interno diferente de `cancel` global, e `entities/func.rs:385-397` explica a igualdade por nome. Essa igualdade é observável na linguagem; não a reduzir a detalhe mecânico Rust. `math.underover` está ausente dos dois produtos: a afirmação P1474 de lacuna vanilla foi retificada por adendo, sem apagar o registro histórico.

| Rota | Chamada concreta cristalina | Limite de substituição / L0 |
|---|---|---|
| `counter_at` | `("k", "missing")` → string vazia | `.at(<missing>)` → `()` cristalino; erro vanilla; L0 `counter.md:218-221` |
| `counter_display` | `("k")` → `counter.display.callback` | método `.display()` exige contexto; substituição inconclusiva |
| `counter_final` | `("k")` → string vazia | método `.final()` exige contexto; global `Str` vs método array |
| `counter_step` | `("k")` → `counter.update` | `.step()` existe bilateralmente; P1474 dizia o contrário |
| `state_at` | `("k", "missing")` → `none` | método `.at(<missing>)` erra bilateralmente; L0 preserva `None` |
| `state_display` | `("k")` → `state.display` | `.display()` ausente no vanilla e contextual no cristalino |
| `state_final` | `("k")` → `none` sem documento | método vanilla com init 7 devolve 7; cristalino exige contexto |
| `state_update` | `("k", 5)` → `state.update` | `.update(5)` funciona, mas conteúdo `== false` no cristalino |
| `state_update_with` | `("k", x => x + 1)` → `state.update` | `.update(callback)` funciona, mas conteúdo `== false` na sonda |
| `calc.log10` | `(100)` → `2.0` | `calc.log(100, base: 10)` → `2.0` bilateral; diagnósticos não provados equivalentes |
| `calc.deg` | `(calc.pi)` → `180.0` | conversão de número puro sem substituto vanilla identificado |
| `calc.rad` | `(180)` → `3.141592653589793` | conversão de número puro sem substituto vanilla identificado |
| `replace` | `([aba], "a", "x")` → `[xbx]` | método de `Str` não opera sobre `Content`; L0 `text/case.md:33-51` |

Os nove globais `counter_*`/`state_*` são contratados nos L0 `compiler/stdlib/counter.md:213-221` e `compiler/stdlib/state.md:28-32,187-200`; os resultados legacy `Str`/`None`, a diferença de contexto e a igualdade falsa dos updates proíbem deduplicação automática. `replace` é contratado por `compiler/stdlib/text/case.md:29-51` para `Str | Content`. As três `calc.*` foram preservadas explicitamente por P817-E (`diagnosticos/typst-passo-817-relatorio.md:107-118`) e são usadas em `lab/parity/corpus/p500/test-calc-rest.typ:19-21`. A busca de chamadas globais literais em outros `.typ` do workspace não encontrou ocorrências; isso não prova ausência de clientes externos ou de construção dinâmica.

## Decisão por classe (41 rotas)

| Classe atual | Rotas | Decisão |
|---|---|---|
| Extensões já deliberadas (10) | `grid_cell`, `grid_header`, `grid_footer`, `table_cell`, `table_header`, `table_footer`, `lof`, `lot`, `math.registered`, `sym.registered` | Preservar: `compiler/eval.md` P1293, `structural/outline.md`, `structural/math.md` P1283 e `sym.md`. Sem crédito de paridade. |
| Extensões numéricas P817-E (3) | `calc.log10`, `calc.deg`, `calc.rad` | Preservar decisão histórica; L0 `compiler/stdlib/calc.md` agora a explicita. Sem crédito de paridade. |
| Contratos legacy (9) | `counter_at`, `counter_display`, `counter_final`, `counter_step`, `state_at`, `state_display`, `state_final`, `state_update`, `state_update_with` | Preservar resultados e gates próprios; métodos não são substitutos gerais. Sem crédito de paridade. |
| Contrato de texto (1) | `replace` | Preservar, inclusive `Content` e `count:`. Sem crédito de paridade. |
| Globais matemáticos históricos (17) | `accent`, `bb`, `bold`, `cal`, `cancel`, `display`, `frak`, `inline`, `italic`, `mono`, `op`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright` | Preservar por compatibilidade; L0 `compiler/eval.md` P1488 explicita o desvio. `cancel` não é identidade funcional do qualificado. Sem crédito de paridade. |
| Global math sem contraparte qualificada (1) | `underover` | Preservar superfície vigente, sem inventar `math.underover` ausente bilateralmente. Sem crédito de paridade. |

Esta preservação não é inferida apenas do conforto de manter tudo: foram checados os L0 vigentes, a decisão P817, chamadas reais, igualdade pública e uso no workspace. Ela seria refutada por nova política explícita de quebra compatível aprovada pelo dono, prova de que uma rota é só interna ou sonda ampla que revele dano semântico causado pela extensão. A presença extra não altera automaticamente programas vanilla válidos, mas também não equivale a paridade total. `math.op` mantém uma divergência de `repr` frente ao vanilla, e os gaps funcionais P1480–P1486 não são fechados aqui.

Não há remoção recomendada neste estado de evidência. Por isso não se altera binding produtivo e não se aciona o gate de remoção ADR-0127 nesta rodada. Se alguma rota vier a ser retirada, a decisão tem de ser nova e individual: medição anterior, owner L0 causal, plano de migração/depreciação, confirmação humana específica, RED de ausência e GREEN do caminho substituto nos quatro perfis, seguido de build e lint. O pedido genérico de implementar P1488 não é confirmação específica de quebra.

## Comandos e gates

~~~sh
mkdir -p /tmp/p1488-inventory-codex-20260924.T6k2u0  # recriar para reprodução
for pass in 1 2; do
  for profile in default html a11y-extras html+a11y bundle html+bundle; do
    lab/typst-original/target/release/p1140-inventory /tmp/p1488-inventory-codex-20260924.T6k2u0/vanilla-$pass-$profile.json "$profile" /usr/local/bin/typst
    lab/surface-inventory/target/release/surface-inventory-p1140 /tmp/p1488-inventory-codex-20260924.T6k2u0/crystal-$pass-$profile.json /tmp/p1488-inventory-codex-20260924.T6k2u0/vanilla-$pass-$profile.json "$profile" lab/surface-inventory/extra_seeds.json target/debug/typst
    python3 lab/surface-inventory/merge.py /tmp/p1488-inventory-codex-20260924.T6k2u0/vanilla-$pass-$profile.json /tmp/p1488-inventory-codex-20260924.T6k2u0/crystal-$pass-$profile.json /tmp/p1488-inventory-codex-20260924.T6k2u0/inventory-$pass-$profile.json --profile "$profile"
  done
done
python3 -c 'from pathlib import Path; import hashlib; p=Path("/tmp/p1488-inventory-codex-20260924.T6k2u0"); profiles=("default","html","a11y-extras","html+a11y","bundle","html+bundle"); kinds=("vanilla","crystal","inventory"); [(lambda a,b: print(kind,profile,"IDENTICAL" if a==b else "DIFFERENT",hashlib.sha256(a).hexdigest()))((p/f"{kind}-1-{profile}.json").read_bytes(),(p/f"{kind}-2-{profile}.json").read_bytes()) for kind in kinds for profile in profiles]'
target/debug/typst eval '(math.cancel == cancel, repr(math.cancel), repr(cancel))'
/usr/local/bin/typst eval 'type(math.underover)'
target/debug/typst eval 'type(math.underover)'
CARGO_BUILD_JOBS=1 cargo test -p typst-core --lib p817e_log10_deg_rad_extensao_cristalina --quiet
crystalline-lint --checks v5,v15,v26 --fix-hashes .
git diff --check
~~~

O teste P817-E passou (1/1; 6142 filtrados). Os dois L0 foram ressellados, sem mudança funcional de L1: apenas headers `@prompt-hash` dos seus consumers. `CARGO_BUILD_JOBS=1 cargo build --quiet`, `crystalline-lint --quiet .` e `git diff --check` passaram após a criação deste relatório. O build ainda emite avisos preexistentes; o lint completo saiu com código 0.

Os arquivos temporários em `/tmp/p1488-inventory-codex-20260924.T6k2u0` foram removidos após os gates; não são recuperáveis diretamente, mas os hashes e comandos acima permitem regenerá-los. A verificação de processos não encontrou inventários, cargo ou rustc ativos.
