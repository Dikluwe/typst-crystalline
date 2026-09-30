# L0 — stdlib/layout/casting — conversões de medidas e lados
Hash do Código: 644c9363

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/casting.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/compiler/stdlib/layout.md`.

## Contrato de linguagem

Conversores de length, largura/altura de block e box, sides, corners e stroke preservam casts, defaults, precedência, tipos relativos e diagnósticos. extract_stroke e extract_corners_length_value continuam disponíveis aos consumers estruturais.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.
