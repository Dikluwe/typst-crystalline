# L0 — compiler/eval/rules/set/text — Set text e transporte lexical
Hash do Código: 0f4067d8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/text.rs`

## Medição antes da decisão

O dispatcher em `01_core/src/compiler/eval/rules/set.rs` no estado após a extração P1499 mantém o braço text como bloco contíguo. O baseline L1 em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` e a suíte após P1499 passaram com 6401 testes e 2 ignorados. Isto é medição estrutural, não benchmark nem nova medição bilateral com o vanilla.

## Contrato de linguagem

Preservar schema de propriedades, validação de tipos e defaults, precedência, warnings de scope-out e transporte custom text.* transparente à morfologia. text.hyphenate aceita bool/auto; os custos e a política lexical seguem os núcleos pinados.

A função deste owner é chamada estaticamente pelo dispatcher, sem mudança na ordem dos braços. Preservar erros, spans, avaliação de argumentos, identidade de valores, ausência versus presença de estilos e fase de execução. As decisões históricas e medições permanecem em `00_nucleo/prompts/compiler/eval/rules.md`.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade linguística conforme ADR-0107/0108; owner 1:1 e pins conforme ADR-0129/0130.
