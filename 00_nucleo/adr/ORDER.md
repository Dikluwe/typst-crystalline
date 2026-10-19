# Canonical ADR reading order

This file orders the active ADR corpus by authority and architectural reach.
It does not renumber decisions. Every numbered file in 00_nucleo/adr/ must
appear exactly once below. Revoked, superseded, and misclassified records are
listed through the archive manifest instead. Classification uses the canonical
English primary statuses defined in README.md; historical qualifiers do not
promote a decision. ADR-0130 governs L0 references: use full repository-relative
paths including `00_nucleo/prompts/` in prose. Rust `@prompt` remains unchanged.

## 1. Constitutional constraints

- [ADR-0129](adr-0129-l0-owners-and-shared-tekt-nuclei.md)
- [ADR-0130](adr-0130-canonical-l0-prompt-references.md)
- [ADR-0127](adr-0127-l0-gate-stop-vs-continuous-flow.md)
- [ADR-0029](adr-0029-physical-l1-purity.md)
- [ADR-0030](adr-0030-ram-performance-in-l1-domain.md)
- [ADR-0018](adr-0018-rustc-hash-l1-dependency.md)
- [ADR-0032](adr-0032-unsafe-policy-in-l1.md)
- [ADR-0107](adr-0107-language-parity-not-mechanics.md)
- [ADR-0033](adr-0033-functional-parity-with-vanilla.md)
- [ADR-0108](adr-0108-anti-drift-measure-before-deciding.md)
- [ADR-0109](adr-0109-atomization.md)
- [ADR-0037](adr-0037-domain-cohesion.md)
- [ADR-0100](adr-0100-domain-cohesion-in-l3.md)
- [ADR-0104](adr-0104-agent-atomicity.md)
- [ADR-0119](adr-0119-verification-discipline.md)
- [ADR-0121](adr-0121-measurement-provenance.md)
- [ADR-0125](adr-0125-implementation-vs-language-difference.md)

## 2. Evidence and decision process

- [ADR-0034](adr-0034-diagnose-vanilla-types-before-materialization.md)
- [ADR-0065](adr-0065-inventory-before-decisions.md)
- [ADR-0084](adr-0084-conditional-audit.md)
- [ADR-0085](adr-0085-immutable-diagnostics.md)
- [ADR-0114](adr-0114-probe-before-specification.md)
- [ADR-0117](adr-0117-probe-a0-verification-mechanism.md)
- [ADR-0122](adr-0122-parity-defaults-in-comparison-tests.md)
- [ADR-0124](adr-0124-sublayout-propagation-checklist.md)
- [ADR-0093](adr-0093-adr-evolution-methodology.md)
- [ADR-0094](adr-0094-specification-operations.md)
- [ADR-0098](adr-0098-single-source-of-truth-anti-bug-invariant.md)
- [ADR-0113](adr-0113-transparent-stub-vs-software-fallback.md)
- [ADR-0080](adr-0080-minimal-l0-for-refactors.md)

## 3. Core domain and representation

- [ADR-0001](adr-0001-migration-strategy.md)
- [ADR-0005](adr-0005-packagespec-world-trait.md)
- [ADR-0009](adr-0009-default-math-class.md)
- [ADR-0024](adr-0024-ecow-for-value-string.md)
- [ADR-0025](adr-0025-integer-float-equality.md)
- [ADR-0026](adr-0026-content-intentional-divergence.md)
- [ADR-0026-R1](adr-0026-r1-content-arc-revision-1.md)
- [ADR-0031](adr-0031-early-source-hashing.md)
- [ADR-0036](adr-0036-progressive-atomization.md)
- [ADR-0038](adr-0038-style-system-in-l1.md)
- [ADR-0039](adr-0039-frameitem-text-style.md)
- [ADR-0052](adr-0052-lang-semantic-type-in-l1.md)
- [ADR-0053](adr-0053-font-composite-type-in-l1.md)
- [ADR-0058](adr-0058-simplified-value-type.md)
- [ADR-0059](adr-0059-args-as-separate-type.md)
- [ADR-0064](adr-0064-smart-option-default-translation.md)
- [ADR-0101](adr-0101-ieee754-stdlib-restriction.md)
- [ADR-0102](adr-0102-math-style-mechanism.md)
- [ADR-0103](adr-0103-math-style-composition.md)
- [ADR-0105](adr-0105-element-model-d-to-f-destination.md)
- [ADR-0106](adr-0106-extension-boundary-e1.md)
- [ADR-0112](adr-0112-rust-decimal-in-l1.md)

## 4. Compiler runtime and pipeline

- [ADR-0017](adr-0017-defer-eval-typst-library-strategy.md)
- [ADR-0040](adr-0040-set-rule-activation-in-eval.md)
- [ADR-0041](adr-0041-show-rule-activation.md)
- [ADR-0042](adr-0042-materialized-sink-in-l1.md)
- [ADR-0043](adr-0043-sink-output-channel.md)
- [ADR-0044](adr-0044-engine-eval-state-aggregator.md)
- [ADR-0068](adr-0068-location-aware-layouter.md)
- [ADR-0069](adr-0069-post-recursion-tag-emission.md)
- [ADR-0070](adr-0070-remove-legacy-counter-state.md)
- [ADR-0071](adr-0071-walk-pipeline-redesign.md)
- [ADR-0072](adr-0072-closed-m7-fixpoint-runtime.md)
- [ADR-0073](adr-0073-comemo-introspector.md)
- [ADR-0074](adr-0074-trackable-layouter-substores.md)
- [ADR-0076](adr-0076-introspector-completion.md)
- [ADR-0099](adr-0099-graded-later-feature-activation.md)
- [ADR-0120](adr-0120-textshaped-rustybuzz-pipeline.md)

## 5. Layout, infrastructure, and export

- [ADR-0019](adr-0019-ttf-parser-rustybuzz-in-l3.md)
- [ADR-0020](adr-0020-fontdb-in-l3.md)
- [ADR-0022](adr-0022-fontbook-in-l3.md)
- [ADR-0027](adr-0027-cidfont-subsetting.md)
- [ADR-0054](adr-0054-debt-1-closure-criterion.md)
- [ADR-0055](adr-0055-font-consumer-cidfont-pipeline.md)
- [ADR-0057](adr-0057-lang-hyphenation-in-l1.md)
- [ADR-0075](adr-0075-vanilla-integration.md)
- [ADR-0078](adr-0078-column-flow-algorithm.md)
- [ADR-0082](adr-0082-graded-scope-out-promotions.md)
- [ADR-0083](adr-0083-vanilla-color-parity-materialized-subset.md)
- [ADR-0086](adr-0086-solid-only-paint-wrapper.md)
- [ADR-0087](adr-0087-linear-gradient-only.md)
- [ADR-0088](adr-0088-radial-gradient-only.md)
- [ADR-0089](adr-0089-conic-gradient-only.md)
- [ADR-0091](adr-0091-gradient-color-space-and-cmyk-strategy.md)
- [ADR-0092](adr-0092-conic-coons-patches-rgb-cmyk.md)
- [ADR-0095](adr-0095-arc-pointer-resource-deduplication.md)
- [ADR-0115](adr-0115-scanner-benchmark-infrastructure.md)
- [ADR-0116](adr-0116-scanner-get-unchecked-exception.md)
- [ADR-0126](adr-0126-verbose-pdf-default-compact-option.md)
- [ADR-0128](adr-0128-html-as-semantic-target.md)

## 6. Shell, CLI, and diagnostics

- [ADR-0045](adr-0045-diagnostic-resolution-and-formatting.md)
- [ADR-0046](adr-0046-minimal-cli-in-l4.md)
- [ADR-0047](adr-0047-clap-cli-argument-parsing.md)
- [ADR-0048](adr-0048-ansi-colored-diagnostics.md)
- [ADR-0049](adr-0049-cli-in-l2.md)
- [ADR-0050](adr-0050-diagnostic-formatter-in-l2.md)
- [ADR-0051](adr-0051-functional-cli-flags.md)

## 7. Parsing and dependency decisions

- [ADR-0006](adr-0006-remove-typst-timing-from-l1.md)
- [ADR-0008](adr-0008-inline-defer-in-l1.md)
- [ADR-0010](adr-0010-unicode-ident-l1-dependency.md)
- [ADR-0011](adr-0011-unicode-math-class-l1-dependency.md)
- [ADR-0012](adr-0012-unicode-script-l1-dependency.md)
- [ADR-0013](adr-0013-unicode-segmentation-l1-dependency.md)
- [ADR-0014](adr-0014-unscanny-scanner.md)
- [ADR-0015](adr-0015-remove-ecow-from-parser.md)
- [ADR-0016](adr-0016-remove-lazyhash-from-l1.md)
- [ADR-0021](adr-0021-time-crate-datetime.md)
- [ADR-0023](adr-0023-indexmap-l1-dependency.md)
- [ADR-0035](adr-0035-ecovec-authorized-in-l1.md)
- [ADR-0062](adr-0062-hayagriva-bibliography-parsing.md)
- [ADR-0077](adr-0077-regex-l1-dependency.md)
- [ADR-0111](adr-0111-parsing-loading-crate-authorization.md)

## 8. Proposed or pending decisions

- [ADR-0067](adr-0067-attribute-grammar-property-scoping.md)
- [ADR-0110](adr-0110-vanilla-correspondence-marker.md)
- [ADR-0118](adr-0118-context-based-runtime-state.md)
- [ADR-0123](adr-0123-typographic-geometry.md)

## Historical records

Historical records and redirects are indexed in
`lab/results/construction/diagnostics/adr-archive/redirects.tsv`. They are
evidence, not current authority.
