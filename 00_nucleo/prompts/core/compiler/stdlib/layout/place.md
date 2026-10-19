# L0 — stdlib/layout/place — place e flush
Hash do Código: 498810d7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/place.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Contrato de linguagem

place preserva alinhamento, dx/dy, scope, float e o default relativo de clearance: 1.5em. flush rejeita qualquer argumento e produz Content::flush sem executar layout.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.

## Preservar omissão vertical — 2026-10-14

### Medição antes da decisão

`01_core/src/compiler/stdlib/layout/place.rs:42–48` converte omissão em `Some(Top)`. As sondas `lab/results/construction/diagnostics/place-origem-local-2026-10-14/{before,extra-before}/results.json.gz` guardam HEAD, árvore suja, diffstat, UTC, binários e SVGs contra o pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8: após texto A, alinhamento omitido ancora na baseline; Top explícito ancora na região. `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:703–715` distingue o eixo explícito do offset de flow. Trata-se de geometria de linguagem, não de igualdade Rust. Os testes place_origin_pending_line_and_explicit_top_are_distinct reproduzem a divergência antes da correção.

### Decisão e aceitação

Usar o Option existente para preservar v=None quando o eixo vertical é omitido; manter o alinhamento explícito. Não adicionar campo ou alterar assinatura. Correção de paridade em fluxo contínuo ADR-0127, sem novo modo, flag ou fase. Layout decide a âncora segundo o núcleo place-coordinate-origin. Preservar clearance, scope, validação e floats; validar testes de linha pendente, Top explícito, geometria, floats e suíte completa. A etapa B de âncora após fluxo já comitado permanece aberta no diagnóstico de origem local.
