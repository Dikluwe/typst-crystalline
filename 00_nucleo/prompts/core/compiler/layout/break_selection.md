# L0 — break selection
Hash do Código: 6f4c9873

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/break_selection.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `lab/results/construction/diagnostics/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `lab/results/construction/diagnostics/separacao-texto-linhas-regioes-2026-09-28/`.

correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json preserves 52 bilateral inputs/sources/commands/boxes and state in initial.json. Absent/auto/false/true were identical in Tekt. Historical oracle `9f2b6e8715237cb086899a42873660fe744622e8` rejects integer/string hyphenate, integer/auto justify and linebreaks outside simple/optimized/auto, exact diagnostics/spans in stderr. `eval/rules.rs:2972-3010,3407-3420` discards options; `stdlib/structural/par.rs:119-126` accepts/ignores; `break_selection.rs:39` ignores policy. Pinned text/mod.rs:545-565 declares Smart<bool>, model/par.rs:227-242,627-636 declares justify/Linebreaks. This measures types/behavior, not historical prompt intent.

Bilateral exact-state probes in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ distinguish U+002D/SHY with coincident geometry across eight controls; approved-before/ preserves this consumer's FrameItem inspection. Owner confirmed public `00_nucleo/prompts/core/entities/layout_types.md`.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/margin-before/results.json.gz` preserves exact state/commands/binaries/bilateral inspected images. With 70pt margin, plain-box/first-overlong leave following words outside Tekt Box instead of wrapping; `word_layout.rs:44,81` supplies page margin while `sub_frame.rs:196–217` uses local zero; `break_selection.rs:25` suppresses overflow until crossing supplied threshold. This explains implementation/reflow-green-first.log RED, not permission to weaken it; old atomization page-margin preservation is refuted for local regions.

C016 word at break_selection.rs:27 has 8 Clippy args versus 7. Structural snapshot `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`/classification.json preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state, UTC2026-10-15T16:39:18.873477+00:00, source/L0/signatures/hashes/status/diffstat and checked spans. Pure isolated selector explicitly needs measured word/width, metrics/style, cursor/edge/real-origin and hyphenation policy; reinference recreates local-origin defect. Arity alone proves no functional fault.

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` preserves 79 warnings; nominal-exception source/span/reason/signature/hash/HEAD/uncommitted-state/UTC/diffstat reconfirmation is in `clippy-excecoes-2026-10-16/before.json.gz`. Mechanics, not new parity/cognitive-benefit proof.

## Exclusive ownership and current contract

Own private Decision { Keep, Wrap, Hyphenate { prefix, width, rest } }, should_wrap and word selection. Receive injected metrics/style, word, its already measured width and numeric cursor/right-edge/wrap-threshold values; access no Layouter or mutable state. Use the exact overflow test cursor_x + width > right_edge AND cursor_x > the supplied line origin. The local-origin correction below supersedes the historical page-margin threshold. If no overflow, Keep. At overflow, use the existing lang/hyphenation points in reverse order, measure each prefix plus literal hyphen through text_measure::word, choose the first that fits and return an owned prefix/rest split by Unicode character indices. If no usable point, Wrap. Chunk selection uses only should_wrap and does not hyphenate. Do not flush, paginate, alter style, consume text, add optimized selection or honor previously discarded properties. No public strategy or trait is introduced.

The private word selector additionally receives resolved hyphenation bool. After existing overflow predicate, false returns Wrap without pattern lookup/prefix measurement; true retains Unicode path. Keep/progress/bounds/no-pattern behavior is unchanged; Chunk never hyphenates. This measured policy slice supersedes earlier atomization prohibition on honoring discarded properties only to this extent; shared paragraph-break-policy nucleus owns no code.

Private Decision::Hyphenate adds replacement String: lexical prefix followed by U+00AD, distinct from visual prefix measured with literal hyphen. Produce both at the same selected point, with no later heuristic.

Progress threshold is actual line origin in same coordinates as cursor/right edge, received numerically from caller; never access Layouter or infer margin. Strict cursor+width>edge AND cursor>origin permits first overlong-word progress without blank line but later wraps. No new hyphenation algorithm/public API/phase; preserve tracking and overlong/exact-fit/chunk/unbounded-region controls.

Preserve word signature/order/semantics; only the materialized nominal annotation `#[expect(clippy::too_many_arguments, reason="C016: Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização")]` is authorized on word. No module/impl/crate/global suppression, threshold increase or other lint. Review/remove if arity no longer exceeds limit; never hide unfulfilled_lint_expectations. Later C016 materialization supersedes audit-only pending annotation for this item only; preserve body/calls/cfg/unrelated attributes, with other required corrections including conditional C047/C066 still pending.

## Acceptance and limits

Preserve the existing observable text, positions, styles, pages and errors in the controlled corpus and current suites. Structural tests refute reintroduction of algorithms in adapters. Pure-selection tests discriminate no-wrap, wrap and Unicode-safe prefix decisions; measurement tests retain overrides and shaped/fallback paths. Each consumer is owned exclusively by this prompt. This is internal atomization, not completion of the audit findings.

Enabled Unicode control stays explicitly enabled; disabling same case must yield Wrap. Replacement acceptance is text/boxes/causal tests/recursive transport/literal-hyphen control, never PDF bytes/new selection. Internal parity/maintenance follows continuous-flow ADR-0127 with unchanged public entities/signatures/default/phase; L0 before code/restricted bilateral reseal, valid V15/V26.

For C016 confirm workspace/all-targets warning then no unfulfilled expectation; preserve calls/functional controls. Removing added attributes/lineage header recovers rustfmt-normalized prior source. Establish metadata RED first, then exactly 46 arities+five test warnings fulfilled by 49 expectations, no new warning/unfulfilled-expectation error;11 required corrections remain visible. Build/fmt/strict-reverse lineage/Tekt no new findings/Counter tests/coverage under 12GiB/zero swap/one job. Prior full suite remains evidence for unchanged executable code; no attribute-mirror tests/new metadata performance requirement. Private-signature changes migrate callers/L0 first; public/compatibility changes need new concrete ADR-0127 confirmation, not this exception.
