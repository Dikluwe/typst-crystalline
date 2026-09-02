# P1292 — recibo protegido da correção final de transporte SVG

**Papel:** autora independente do oráculo protegido P1292  
**Estado:** `ORACLE_TRANSPORT_CORRECTED_GREEN_AWAITING_DISCRIMINATION_AND_SEAL`  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507` (`Tekt`)  
**Regime:** Tekt segregado por capacidades e artefatos; filesystem compartilhado,
sem alegação de isolamento técnico de leitura.

## Limites e fontes

Esta autoria leu integralmente o L0 proprietário
`00_nucleo/prompts/wiring/tests/p1292_contract.md`, SHA-256
`614707e0a2d585bfec897c50f83fb52379e1b79f90dffc30c45c56caf37c1a7d`,
e o plano adversarial explicitamente pinado pelo L0,
`00_nucleo/diagnosticos/p1293-adversarial-plan.md`, SHA-256
`ecc12db3e5d9826dafdb7b42653e28d5f4f8e01772a8554e6718b1ac8129e226`.
Não consultou nem alterou implementação produtiva de matemática, recibo de
implementador, manifesto ou selo. A única edição de código foi o helper/parser
test-only em `04_wiring/tests/p1292_contract.rs`; este recibo é o único outro
artefato escrito por este papel.

O header ressellado `@prompt-hash 05d0eea8` já estava presente no preimage e
foi preservado literalmente. Fixtures, tolerâncias, asserts e expectativas
`7.513`/`21.901` permaneceram byte-idênticos.

## Preimage RED e causalidade

O preimage do oráculo tinha SHA-256
`4258112d7c679b30fa6de8a38bd456cba99b550c6968e87dfc1d4bca09be03d2`;
seu corpo canônico, removida somente a linha `@prompt-hash`, tinha SHA-256
`cf1c1953b75d39c9340e82a9169271980fd44e63564fdadd146c5e14326906d7`.

Em `2026-09-02 17:10:32-03:00`, a execução fresca

```text
RUSTFLAGS='-Awarnings' cargo test -q -p typst-wiring --test p1292_contract -- --test-threads=1
```

terminou com exit `101`: `10 passed / 1 failed`. O único RED foi
`p1292_b_plain_mixed_baselines_and_following_line_advance`, porque o helper
tratava `matrix(... 4.862)` como posição global e descartava o ancestral
`translate(10.395 2.651)`. Isso produzia duas baselines onde a linguagem tem
uma: `4.862` e `7.513`. Log `/tmp/p1292-observer-preimage-red.log`, 1006 bytes,
SHA-256 `43270484b791547036b8a9953b92c3ef323254b533e5d1e94f595242e196dd28`.

Classificação ADR-0107/0108: a baseline global é observável geométrico; o XML e
a composição afim são transporte. A expectativa não foi adaptada ao produto:
`2.651 + 4.862 = 7.513` permanece a obrigação.

## Correção do oráculo

O parser agora percorre a árvore SVG, mantém uma pilha de transforms ancestrais
e compõe afins na ordem `ancestral ∘ local`. Ele reconhece explicitamente
`translate(x[, y])` e `matrix(a b c d e f)` e mede o deslocamento global do
grupo de glifo que contém `<use>`. A seleção não depende de caractere, fonte,
fixture ou sentinela numérica.

Postimage do oráculo:

- SHA-256 integral:
  `5104b48ff89766e9910cd3a39eef3706fe0c8539a0fa6cb7dafcb3cf66a7484a`;
- SHA-256 canônico, removida somente a linha `@prompt-hash`:
  `e208b1f7da1a58550d39b92a265462e6a21ee5ca3dc3dbfba31feb69e08b5e97`.

O refutador/mutante test-only específico é “descartar a composição ancestral e
usar apenas o transform local”. O preimage executado é uma instância exata
desse mutante e foi discriminado pelo RED `10/11`; a versão corrigida é GREEN.
O registry selado P1292 contém mutações negativas de produto, não mutações da
mecânica interna do próprio oráculo. Como nenhuma obrigação produtiva foi
adicionada, o denominador congelado de 23 IDs não requer nem recebe novo ID.

## GREEN protegido e gates focais

A execução final foi bracketada por snapshots idênticos do estado integrado:

- início `2026-09-02T17:13:49-03:00`;
- fim `2026-09-02T17:14:25-03:00`;
- `git diff HEAD --binary` SHA-256
  `4c0eb169d9dd5613f5c0e3ec6d6c6da923b0d6fc3c174e1bc0c30e11c481e90e`;
- `git diff HEAD --name-only` SHA-256
  `d1fc46c3138fc45b8d237435b498e7b6fff13ba93b7bceda64b9622857cf9390`;
- `git status --short` SHA-256
  `91dfe7e0cb6dcab20142ed0e24f1eed06eb5da3f9d2d91bdacf789ffb8d0d132`;
- oráculo P1292 SHA-256 `5104b48f...a7484a` em ambos os extremos;
- oráculo P1293 SHA-256 `2505aa34...c7cb` em ambos os extremos.

O mesmo comando protegido terminou com exit `0`, `11 passed / 0 failed`, em
22.53 s. Log `/tmp/p1292-observer-final-stable-green.log`, 376 bytes, SHA-256
`809c076a3641ff70c46cedc2bbc5ed1c9b38ea863db181d314dbeb56dca6905f`.

Gates focais:

- `rustfmt --edition 2021 --check 04_wiring/tests/p1292_contract.rs`: exit `0`;
- `crystalline-lint --checks v1,v5,v15,v26 --fail-on warning .`: exit `0`,
  `No violations found`;
- `git diff --check`: exit `0`.

O corpo canônico de `04_wiring/tests/p1293_contract.rs`, removida somente a
linha `@prompt-hash`, permaneceu
`b07eb8d6eb6bfa0a892c06bb1f5599d8e2e188963dcca821d83f4a34badfb13a`.
Nenhum artefato A/B/C/D de P1293 foi editado.

## Lista exata dos arquivos rastreados alterados no snapshot GREEN

O snapshot integrado não estava commitado. `git diff HEAD --name-only`
registrou exatamente estes arquivos durante toda a execução protegida:

```text
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/eval/math.md
00_nucleo/prompts/compiler/eval/repr.md
00_nucleo/prompts/compiler/eval/tests.md
00_nucleo/prompts/compiler/layout/equation.md
00_nucleo/prompts/compiler/layout/helpers.md
00_nucleo/prompts/compiler/layout/text.md
00_nucleo/prompts/compiler/math/layout/_comum.md
00_nucleo/prompts/compiler/math/layout/attach.md
00_nucleo/prompts/compiler/stdlib/foundations/float.md
00_nucleo/prompts/compiler/stdlib/html.md
00_nucleo/prompts/compiler/stdlib/math_style.md
00_nucleo/prompts/compiler/stdlib/structural/math.md
00_nucleo/prompts/entities/content.md
00_nucleo/prompts/entities/elements/math_attach.md
00_nucleo/prompts/entities/layout_types.md
00_nucleo/prompts/entities/style_chain.md
00_nucleo/prompts/infra/export/html.md
00_nucleo/prompts/infra/export/mod.md
00_nucleo/prompts/infra/font_metrics.md
00_nucleo/prompts/infra/integration_tests.md
00_nucleo/prompts/infra/pipeline.md
00_nucleo/prompts/infra/shaper.md
00_nucleo/prompts/shell/cli.md
00_nucleo/prompts/wiring.md
00_nucleo/prompts/wiring/tests/p1292_contract.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/math.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/eval/tests.rs
01_core/src/compiler/layout/equation.rs
01_core/src/compiler/layout/helpers.rs
01_core/src/compiler/layout/text.rs
01_core/src/compiler/math/layout/accent.rs
01_core/src/compiler/math/layout/attach.rs
01_core/src/compiler/math/layout/cancel.rs
01_core/src/compiler/math/layout/cases.rs
01_core/src/compiler/math/layout/frac.rs
01_core/src/compiler/math/layout/matrix.rs
01_core/src/compiler/math/layout/mod.rs
01_core/src/compiler/math/layout/root.rs
01_core/src/compiler/math/layout/spacing.rs
01_core/src/compiler/math/layout/tests.rs
01_core/src/compiler/math/layout/underover.rs
01_core/src/compiler/math/layout/vec.rs
01_core/src/compiler/stdlib/foundations/float.rs
01_core/src/compiler/stdlib/html.rs
01_core/src/compiler/stdlib/math_style.rs
01_core/src/compiler/stdlib/structural/math.rs
01_core/src/entities/content.rs
01_core/src/entities/elements/math_attach.rs
01_core/src/entities/layout_types.rs
01_core/src/entities/style_chain.rs
02_shell/src/cli.rs
03_infra/src/export/html.rs
03_infra/src/export/mod.rs
03_infra/src/export/tests.rs
03_infra/src/font_metrics.rs
03_infra/src/integration_tests.rs
03_infra/src/pipeline.rs
03_infra/src/shaper.rs
04_wiring/src/main.rs
04_wiring/tests/p1292_contract.rs
```

Os demais itens do snapshot `git status --short` eram não rastreados e foram
congelados pelo hash de status acima; este papel não os alterou. Este recibo é
posterior ao snapshot e, por definição, ainda não fazia parte dessa lista.

## Limite do resultado

Este recibo demonstra somente a correção de transporte do oráculo P1292 e seu
GREEN protegido. Não executa discriminação de produto, não ressela manifesto ou
seal e não emite veredito de fechamento.
