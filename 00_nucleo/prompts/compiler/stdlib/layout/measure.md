# L0 — stdlib/layout/measure — layout e measure
Hash do Código: 7bf5da79

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/measure.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/compiler/stdlib/layout.md`.

## Contrato de linguagem

extract_measure_body preserva seleção do corpo; layout preserva callback; measure mantém a medição isolada na fase e o valor/diagnóstico da linguagem.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.
