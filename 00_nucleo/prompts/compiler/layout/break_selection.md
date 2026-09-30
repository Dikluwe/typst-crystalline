# L0 — break selection
Hash do Código: 2ff3429d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/break_selection.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Own private Decision { Keep, Wrap, Hyphenate { prefix, width, rest } }, should_wrap and word selection. Receive injected metrics/style, word, its already measured width and numeric cursor/right-edge/wrap-threshold values; access no Layouter or mutable state. Preserve the exact overflow test cursor_x + width > right_edge AND cursor_x > page margin left (including the first-word/cell conventions, not line_start_x). If no overflow, Keep. At overflow, use the existing lang/hyphenation points in reverse order, measure each prefix plus literal hyphen through text_measure::word, choose the first that fits and return an owned prefix/rest split by Unicode character indices. If no usable point, Wrap. Chunk selection uses only should_wrap and does not hyphenate. Do not flush, paginate, alter style, consume text, add optimized selection or honor previously discarded properties. No public strategy or trait is introduced.

## Acceptance

Preserve the existing observable text, positions, styles, pages and errors in the controlled corpus and current suites. Structural tests refute reintroduction of algorithms in adapters. Pure-selection tests discriminate no-wrap, wrap and Unicode-safe prefix decisions; measurement tests retain overrides and shaped/fallback paths. Each consumer is owned exclusively by this prompt. This is internal atomization, not completion of the audit findings.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

O seletor privado word recebe também o boolean resolvido. Depois do predicado atual de overflow, false devolve Wrap sem consultar padrões ou medir prefixos; true mantém o percurso Unicode existente. Keep, limites, progressão na origem da linha e ausência de padrões ficam iguais. Chunk não hifeniza. Testes mantêm o controle Unicode habilitado explicitamente e comprovam que o mesmo problema resolve Wrap quando desabilitado.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Decision::Hyphenate privado ganha replacement String, prefixo lexical seguido de U+00AD, separado do prefix visual medido com hífen literal. Produzir ambos na escolha do mesmo ponto, sem heurística posterior.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
