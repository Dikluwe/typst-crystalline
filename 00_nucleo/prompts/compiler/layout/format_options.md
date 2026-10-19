# L0 — format options
Hash do Código: b235e502

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/format_options.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/format_options.rs:215`, `01_core/src/compiler/layout/format_options.rs:249`, `01_core/src/compiler/layout/format_options.rs:19`, `01_core/src/compiler/layout/format_options.rs:20`, `01_core/src/compiler/layout/format_options.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own pure traversal collecting effective document.format.* options. Keep the canonical keys and defaults for SVG pretty, PNG ppi 144, HTML pretty, PDF pretty/tagged/pages/standards. Preserve independent option folding and lexical precedence: later effective values win; an empty set delta is neutral rather than resetting unrelated fields.

Traverse only the current transparent root Sequence/Styled paths; a document-format set nested in an unsupported container remains diagnosed through the existing error path, not silently applied. Preserve validated typed projections including tagged Auto/bool, positive PDF page selectors/ranges/arrays with empty selection distinct from absence, and accepted standard names. Reconstruct standards through the existing pure validated types and fail closed for impossible input forms.

The hub only reexports this helper. Target-specific export, feature flags, CLI options, PNG raster resizing and format I/O belong elsewhere; this collector does not change layout phase or page geometry.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-seventh/bundle refuta Source como container de formato: coleta de opções documentais de raiz atravessa apenas body sem aumentar profundidade/container, conservando validação de estilos/campos e rejeição real de opções aninhadas.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Projeções de Value — exaustividade nominal, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/layout/format_options.rs:34 (typst_type_name, emitted-v16). O enum atual em 01_core/src/entities/value.rs:59–195 tem 39 variantes materializadas; o catálogo e forma de payload estão no inventário desta etapa, não na tabela histórica de variantes futuras. before.json.gz em lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16 conserva HEAD, working tree não commitado, UTC, status/diff HEAD --stat, hashes e os textos originais dos owners/consumers. contexts.json foi extraído do AST atual conferido com o SARIF pinado; classification.json distingue nove V16 emitidos e oito projeções latentes. Forma Rust/exaustividade é mecânica; nomes, conversões, empréstimos e observáveis de linguagem mantêm o contrato vigente. Não há nova medição de paridade ou desempenho.

### Decisão interna

typst_type_name conserva os três aliases de diagnóstico Bool→boolean, Int→integer e Str→string. As outras 36 variantes Value compõem grupo nominal que delega a value.type_name(), sem alteração de corpo. Não unificar esta política com a do constructor document, que não renomeia Str. Não mudar coleta, chaves, ordem/precedência, validação, render, opções, targets, default ou travessia de containers. Teste C6: aliases e delegação em none/auto/float/array/content/length e Value::Type; nomes são texto observável de erro, não alegação nova de intenção upstream.

As variantes que compartilham resultado conservam um corpo comum. A lista nominal exige decisão se Value cresce; não duplicar corpos para remover V19. Notas V19 permanecem informativas e registram alternativas escritas, não testes executados, combinações alcançáveis ou déficit N×. Controles positivos/negativos e limites são escolhidos pelo comportamento, sem tratar a execução de um braço como prova de cada alternativa. Nenhum allow/expect ou relaxamento de severidade/exceção para ocultar V16.

### Gate e aceitação

Fluxo contínuo ADR-0127: nenhuma assinatura/trait/campo público, nova variante produtiva, default de produto, quebra de compatibilidade ou fase diferente. L0 antes de testes e implementação, resselo bilateral restrito e V1/V5/V15/V26 válidos. Escrever controles no módulo, confirmar comportamento antes e depois; RED estrutural é ausência de E0004 nos 17 pontos selecionados numa cópia do core com variante sentinela acrescentada somente ao Value copiado. GREEN exige E0004 em cada um dos 17 pontos, sem stubs, e remoção exata dos nove V16 selecionados com os demais diagnósticos preservados. Build, suíte workspace, Clippy estrito, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job Cargo. O gate global permanece aberto enquanto os V16 externos existirem. O falso positivo do parser sobre None explícito é investigação separada e não será escondido neste consumer.
