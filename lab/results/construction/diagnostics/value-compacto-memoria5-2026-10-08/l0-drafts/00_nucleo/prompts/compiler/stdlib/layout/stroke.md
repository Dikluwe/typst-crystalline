# L0 — stdlib/layout/stroke — stroke
Hash do Código: f5a0b819

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/stroke.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/compiler/stdlib/layout.md`.

## Contrato de linguagem

stroke preserva forma posicional e named, paint, thickness, cap, join, dash, miter-limit e overhang; miter-limit exige valor finito positivo.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Construir Value pelo From<Stroke> existente; ao receber Value::Stroke, produzir Stroke próprio para completar componentes posicionais, sem mutar o payload compartilhado. Preservar parsed fields, specified, defaults, validação e precedência de erros.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.
