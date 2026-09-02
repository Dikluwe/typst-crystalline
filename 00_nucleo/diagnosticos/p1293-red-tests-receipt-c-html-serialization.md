# P1293/C — recibo independente do oráculo reaberto e RED de serialização HTML

Data: `2026-09-02T13:39:51-03:00`

HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`

Estado: **ORÁCULO C AUTORADO; RED CAUSAL; NÃO SELADO**.

## Papel, capacidade e limitação

- Papel: `testador_c_serializacao_p1293`, autor independente do oráculo
  protegido após o gate humano ADR-0127 comunicado pelo coordenador.
- Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
  isolamento técnico de leitura no filesystem compartilhado.
- Entradas lidas: L0s confirmados do contrato C, recibos de medição e vanilla
  ratificado. O patch/diff dos consumers produtivos não foi lido.
- Escrita realizada: somente `04_wiring/tests/p1293_contract.rs`, este recibo e
  log efêmero em `/tmp`.
- Produto, L0, manifesto, selo, mutantes e veredito não foram editados.
- A/B/D foram preservados; o helper antigo de compile apenas delega ao helper
  novo com `serialization = None`, mantendo suas chamadas e observáveis.

## Entradas e saídas pinadas

| Artefato | SHA-256 |
|---|---|
| L0 `wiring/tests/p1293_contract.md` | `adf47e09192bf792ede1cf227696e6314f1b4aa059c510141d05118ebb799989` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| binário candidato executado `target/debug/typst` | `b9d02544caa4537c4999bb7166a520e4de82cec80c0e5a9a0597039fe5625612` |
| oráculo protegido novo | `2947f78ea62b518edf6753253dd5b1d8dba46e1caa60cb033566c70a6e491568` |
| corpo canônico do oráculo, removendo só `@prompt-hash` | `6f73f9cf0f2d2872c1a8f8cda8e55ca240b947f23ee9b96bb45a9cf342cdf055` |
| log RED `/tmp/p1293-c-serialization-red-v2.log` | `7ef8714be80b8d725aefe7f8156119afe4ac6e63de9c02434ebe7df17cea3ec4` |

O oráculo tem `46251` bytes; o log RED tem `34669` bytes. O header foi
resselado para `@prompt-hash b1f580e2`, valor efetivo confirmado por V5 para o
L0 vigente.

## Cobertura acrescentada

- Os cinco vetores C-P06 executam o candidato explicitamente com
  `--html-serialization vanilla`; o vanilla ratificado usa seu modo nativo.
- C-P08 prova que ausência da flag e `crystalline` explícito são byte-idênticos,
  que `vanilla` usa os escapes contextuais contratados, que ambos decodificam
  no mesmo DOM e que os modos não colapsam.
- C-P08 rejeita valor alheio, rejeita a flag em `eval`, `query`, `info` e
  `watch`, aceita ambos os valores em compile PDF/PNG/SVG e exige artefatos
  byte-idênticos fora de HTML. Para eliminar variável espúria, cada par usa o
  mesmo input e output temporários.
- C-P09 discrimina os cinco probes: valores de linguagem `none` e `auto`
  aceitos; strings `"none"` e `"auto"` rejeitadas; string `"metadata"` aceita.
- O registro protegido acrescenta MC10–MC12 e congela `29` IDs distintos;
  `Unknown` requerido ou mutante continua zero.

## Compilação e RED causal

```text
$ cargo test -p typst-wiring --test p1293_contract --no-run -q
exit 0

$ cargo test -p typst-wiring --test p1293_contract p1293_opacity_and_mutation_registry_is_closed -- --exact --test-threads=1
exit 0
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out

$ crystalline-lint --checks v5,v15,v26 --fail-on warning .
exit 0
No violations found

$ script -q -e -c 'cargo test -p typst-wiring --test p1293_contract p1293_c_serialization_modes_default_scope_and_non_html_neutrality -- --exact --test-threads=1' /tmp/p1293-c-serialization-red-v2.log
exit 101
running 1 test
test p1293_c_serialization_modes_default_scope_and_non_html_neutrality ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out
```

A falha é o RED esperado e causal ao contrato novo:

```text
C-P08 crystalline failed
code: 2
error: unexpected argument '--html-serialization' found
```

O caso default executou antes e teve sucesso; a primeira flag explícita foi
rejeitada pelo parser. Não houve timeout, crash, falha de build, falha do
parser DOM nem identidade vanilla divergente. O resultado não reivindica
score de mutação, selo ou veredito de produto.

## Proveniência da working tree no instante da medição

A árvore estava não commitada. Identidades determinísticas:

```text
git diff HEAD --binary | sha256sum
7b82ca818f788feb243c5c9534eaa00190aa0e1647f49d45522cadbba9f90924  -

git diff HEAD --name-only | sha256sum
0c483f38c365ca2fb3e83ad37404008f8bb80130867f82d96be0c0ed093defdf  -
61 tracked paths

git ls-files --others --exclude-standard | sha256sum
5f970abd44575eb0ef1a316385c45351b14fe7fe4768de896bca8b99139e041a  -
40 untracked paths
```

`git diff HEAD --stat` no mesmo instante:

```text
 00_nucleo/prompts/compiler/eval.md                 |  46 +-
 .../prompts/compiler/eval/bindings/field_access.md |  41 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 189 ++++++-
 00_nucleo/prompts/compiler/eval/math.md            |  63 ++-
 00_nucleo/prompts/compiler/eval/repr.md            | 192 ++++++-
 00_nucleo/prompts/compiler/eval/tests.md           |  50 +-
 00_nucleo/prompts/compiler/layout/equation.md      | 114 ++++-
 00_nucleo/prompts/compiler/layout/helpers.md       |  67 ++-
 00_nucleo/prompts/compiler/layout/text.md          |  35 +-
 00_nucleo/prompts/compiler/math/layout/_comum.md   | 399 ++++++++++++++-
 00_nucleo/prompts/compiler/math/layout/attach.md   | 361 ++++++++++++-
 .../prompts/compiler/stdlib/foundations/float.md   |  83 ++-
 00_nucleo/prompts/compiler/stdlib/html.md          | 141 +++++-
 00_nucleo/prompts/compiler/stdlib/math_style.md    | 106 +++-
 .../prompts/compiler/stdlib/structural/math.md     | 123 ++++-
 00_nucleo/prompts/entities/content.md              |  34 +-
 00_nucleo/prompts/entities/elements/math_attach.md |  77 ++-
 00_nucleo/prompts/entities/layout_types.md         | 156 +++++-
 00_nucleo/prompts/entities/style_chain.md          |  37 ++
 00_nucleo/prompts/infra/export/html.md             |  52 +-
 00_nucleo/prompts/infra/font_metrics.md            | 115 ++++-
 00_nucleo/prompts/infra/pipeline.md                |  43 +-
 00_nucleo/prompts/infra/shaper.md                  |  59 +++
 00_nucleo/prompts/shell/cli.md                     |  83 ++-
 00_nucleo/prompts/wiring.md                        |  36 ++
 01_core/src/compiler/eval/bindings/field_access.rs |  11 +-
 01_core/src/compiler/eval/call_dispatch.rs         | 262 +++++++++-
 01_core/src/compiler/eval/math.rs                  | 215 +++-----
 01_core/src/compiler/eval/mod.rs                   |   2 +-
 01_core/src/compiler/eval/repr.rs                  | 254 ++++++++--
 01_core/src/compiler/eval/tests.rs                 | 175 +++++--
 01_core/src/compiler/layout/equation.rs            | 180 ++++++-
 01_core/src/compiler/layout/helpers.rs             |  64 ++-
 01_core/src/compiler/layout/text.rs                |   5 +-
 01_core/src/compiler/math/layout/accent.rs         |   2 +-
 01_core/src/compiler/math/layout/attach.rs         | 281 ++++++++++-
 01_core/src/compiler/math/layout/cancel.rs         |   2 +-
 01_core/src/compiler/math/layout/cases.rs          |   2 +-
 01_core/src/compiler/math/layout/frac.rs           |   2 +-
 01_core/src/compiler/math/layout/matrix.rs         |   2 +-
 01_core/src/compiler/math/layout/mod.rs            | 330 ++++++++++--
 01_core/src/compiler/math/layout/root.rs           |   2 +-
 01_core/src/compiler/math/layout/spacing.rs        |  11 +-
 01_core/src/compiler/math/layout/tests.rs          | 452 +++++++----------
 01_core/src/compiler/math/layout/underover.rs      |   2 +-
 01_core/src/compiler/math/layout/vec.rs            |   2 +-
 01_core/src/compiler/stdlib/foundations/float.rs   | 235 ++++++++-
 01_core/src/compiler/stdlib/html.rs                | 487 +++++++++++++++++-
 01_core/src/compiler/stdlib/math_style.rs          |  54 +-
 01_core/src/compiler/stdlib/structural/math.rs     | 560 ++++++++++++++++++++-
 01_core/src/entities/content.rs                    |  36 +-
 01_core/src/entities/elements/math_attach.rs       | 146 ++++--
 01_core/src/entities/layout_types.rs               |  18 +-
 01_core/src/entities/style_chain.rs                |   6 +-
 02_shell/src/cli.rs                                |  44 +-
 03_infra/src/export/html.rs                        |   2 +-
 03_infra/src/export/tests.rs                       |  17 +-
 03_infra/src/font_metrics.rs                       |  99 +++-
 03_infra/src/pipeline.rs                           |   2 +-
 03_infra/src/shaper.rs                             |  55 +-
 04_wiring/src/main.rs                              |   2 +-
 61 files changed, 6029 insertions(+), 694 deletions(-)
```

## Próximo gate causal

O adversário/verificador separado deve executar o gate discriminatório fresco
`29/29`, direto e reverso, com score `1.0`, survivors `0` e `Unknown=0`. Só
depois pode existir selo novo que autorize implementação produtiva C.
