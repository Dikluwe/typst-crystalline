# P1293/C — recibo independente do oráculo e RED da fachada HTML L3

Data da medição: `2026-09-02T14:22:54-03:00`

HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`

Estado: **ORÁCULO REABERTO; RED CAUSAL; NÃO SELADO**.

## Papel e segregação

- Papel: `testador_c_serializacao_p1293`, autor independente do oráculo
  protegido do lote C.
- Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
  isolamento técnico de leitura no filesystem compartilhado.
- Foram lidos integralmente `infra/export/mod.md`, `infra/export/html.md`,
  `infra/pipeline.md` e `wiring/tests/p1293_contract.md`.
- Não foram lidos o patch/diff produtivo atual nem o recibo do implementador.
- Escrita realizada somente no oráculo, neste recibo e em `/tmp`.
- Produto, L0, manifesto, selo, ataques e veredito não foram alterados.

## Entradas e artefatos

| Artefato | SHA-256 |
|---|---|
| `infra/export/mod.md` | `de16b4085aa8d72ace3b4a8b62087a7d9f54c7ae9fd6012891bd3160aeaa08f5` |
| `infra/export/html.md` | `505895669df4a1d7ce39f4c3236d6ac4e39fc9c61ce14d8cf578b0d394a2d237` |
| `infra/pipeline.md` | `ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8` |
| owner do oráculo `wiring/tests/p1293_contract.md` | `adf47e09192bf792ede1cf227696e6314f1b4aa059c510141d05118ebb799989` |
| oráculo protegido atualizado | `ced9f5e8a73fec57950db5acad9b7a9bd183b6b1374197ebdfbe14175215b4ea` |
| corpo canônico, removendo só `@prompt-hash` | `c3034029a5190da08d87cd9bb9b04f7cd04f89e4373cae8e9bfdb80601502ff3` |
| bloco C-P10 | `bb1a3e822889399085b5f88597c9be3d04fde1ea2166d6fbffc9fc42015a9261` |
| registro de mutantes | `417d82202439b39388d223fe46c9dc193e41b705a951dfeb19c80eddfd75bd8b` |
| log RED `/tmp/p1293-c-export-facade-red.log` | `6ecd0411fed68274d3c9599d96d0e01a683c90867692e325c30530e264f8a0ca` |

O oráculo tem `47961` bytes e o log RED `34392` bytes.

## Refinamento do oráculo

C-P10 importa pela API pública do crate:

```rust
use typst_infra::export::{
    HtmlSerializationMode, export_html, export_html_with_serialization,
};
```

O caso prova:

- os três símbolos são nomeáveis pela fachada `typst_infra::export`;
- o `type_name` de cada item permanece em `typst_infra::export::html`,
  discriminando reexport direto de wrapper ou enum duplicado na fachada;
- `export_html` equivale ao modo `Crystalline` explícito;
- os modos continuam semanticamente equivalentes no DOM;
- a neutralidade PDF/PNG/SVG permanece coberta por C-P08, sem novo caminho ou
  expectativa sobre esses exporters.

A/B/D e todos os casos C anteriores permanecem inalterados. O registro recebe
somente:

```text
MC13 -> C-P10-public-export-facade
MC14 -> C-P10-direct-reexport-not-wrapper
```

Denominador congelado: `31`; IDs distintos: `31`; `Unknown` requerido ou
mutante: `0`. Os mutantes anteriores não foram removidos nem enfraquecidos.

## RED causal do estado produtivo recebido

```text
$ script -q -e -c 'cargo test -p typst-wiring --test p1293_contract --no-run -q' /tmp/p1293-c-export-facade-red.log
exit 101

error[E0603]: module `html` is private
03_infra/src/pipeline.rs:168:24
03_infra/src/pipeline.rs:181:24
03_infra/src/pipeline.rs:189:26
03_infra/src/pipeline.rs:213:24

error: could not compile `typst-infra` (lib) due to 4 previous errors
```

As quatro testemunhas tentam nomear `HtmlSerializationMode` e
`export_html_with_serialization` atravessando o submódulo privado. Isso é
exatamente a ausência da fachada exigida pelo L0; não é falha do oráculo,
timeout, crash, dependência externa ou `Unknown`. O compile chega ao produto
L3 antes de alcançar o import externo C-P10, portanto o RED antecede e reforça
a mesma obrigação pública.

Verificações independentes que não exigem produto GREEN:

```text
$ rustfmt --edition 2021 --check 04_wiring/tests/p1293_contract.rs
exit 0
$ crystalline-lint --checks v15,v26 --fail-on warning .
exit 0; No violations found
$ git diff --check
exit 0
```

Este papel não executou gate discriminatório nem reivindica selo,
`mutation_score` ou veredito de produto.

## Proveniência da working tree

A árvore estava não commitada. Identidades no instante da medição:

```text
git diff HEAD --binary | sha256sum
a96cd421e25fbcb05050536fbb30f98ec7d85a07bf71c36a3751face8dbd8d76  -

git diff HEAD --name-only | sha256sum
6d6fb9913dafa3be6587076a92f5a19265bc45547c0541d4d9a9d52a48c9fa36  -
63 tracked paths

git ls-files --others --exclude-standard | sha256sum
7842e2b4a71c4cc1f8332f0caf1c1a7491162f77fee4702e8901a899e4ef520f  -
43 untracked paths
```

Lista exata dos 63 paths tracked alterados:

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
00_nucleo/prompts/infra/pipeline.md
00_nucleo/prompts/infra/shaper.md
00_nucleo/prompts/shell/cli.md
00_nucleo/prompts/wiring.md
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
03_infra/src/pipeline.rs
03_infra/src/shaper.rs
04_wiring/src/main.rs
```

## Próximo gate causal

Após a correção produtiva por autoridade separada, o adversário/verificador
deve executar o registro fresco `31/31` nas duas ordens, exigir score `1.0`,
survivors `0` e `Unknown=0`, além dos gates arquiteturais. Só então um novo
selo pode ser emitido.
