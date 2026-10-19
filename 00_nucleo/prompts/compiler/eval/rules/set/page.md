# L0 — compiler/eval/rules/set/page — Set page e transporte de geometria
Hash do Código: 4cdd2cc0

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/page.rs`

## Medição antes da decisão

O dispatcher em `01_core/src/compiler/eval/rules/set.rs` no estado após a extração P1499 mantém o braço page como bloco contíguo. O baseline L1 em `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` e a suíte após P1499 passaram com 6401 testes e 2 ignorados. Isto é medição estrutural, não benchmark nem nova medição bilateral com o vanilla.

## Contrato de linguagem

Preservar width/height/paper/flipped/binding, margens, numbering, header/footer, bleed/fill/background/foreground, casting, estilo lexical e o SetPageDelta produzido. O tamanho em em resolve contra estilo vigente em eval; a geometria posterior pertence a layout.

A função deste owner é chamada estaticamente pelo dispatcher, sem mudança na ordem dos braços. Preservar erros, spans, avaliação de argumentos, identidade de valores, ausência versus presença de estilos e fase de execução. As decisões históricas e medições permanecem em `00_nucleo/prompts/compiler/eval/rules.md`.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade linguística conforme ADR-0107/0108; owner 1:1 e pins conforme ADR-0129/0130.
