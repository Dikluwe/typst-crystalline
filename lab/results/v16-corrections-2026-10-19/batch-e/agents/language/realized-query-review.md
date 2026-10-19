# Realized Equation occurrence regression evidence

Measured 2026-10-19 19:44:06.713704–19:44:41.487157 UTC, uncommitted working tree based on HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`. Both receipts include exact `git diff HEAD --stat`, original build receipt and all source input hashes; no build inputs changed during either run. Product SHA-256 `77c665d9cc76a7ae35a92d41621a7084abbb4544e11d8100956079348f859916`; pinned oracle commit/tree/recipe/SHA are recorded in each receipt. The sources differ only in whether numbering and alternative text are constructor arguments or captured set-rule values.

## Measurements

- `realized-query/receipt.json`, SHA-256 `695695496ee53730b4387c535babe6ad8bdef8764f8cb514a119c13bb7c91953`: 16 bilateral probes. Label-based projection of block/body/numbering/alt and each corresponding Equation `.where` filter return empty arrays from product; oracle returns the expected single field value or selected block value.
- `realized-query-controls/receipt.json`, SHA-256 `4c2a6ab14f3ad04df3441f1d81d25024122a89e46c387bbf9a567bf3c60c5c88`: 12 bilateral controls. Both unfiltered `math.equation` and label `<e>` find exactly one occurrence in both binaries. Product serializes it as Styled with child Equation defaults; oracle serializes Equation with the captured effective fields. Direct `math.equation --field block/body/numbering/alt` still returns empty product arrays. Thus the initial empty projection is not an absent label or occurrence.

All invocations completed with status zero and matching deprecation diagnostics. Expected field values are `[true]`, a one-element array containing text `x`, `["(1)"]`, and `["probe"]`. Source bytes, exact argv, outputs, time and binary hashes are retained. No product build or productive edit was performed by this audit.

## Source intent and causal classification

Pinned vanilla `crates/typst-library/src/foundations/selector.rs:132–138` defines element matching by identity and the values returned by `target.get(id, styles)` for every requested filter. `crates/typst-library/src/math/equation.rs:175–200` synthesizes the supplement using the effective style chain. These support language-level occurrence access, independent of internal wrapper representation.

Product `01_core/src/compiler/introspect/from_tags/equations.rs:125–166` deliberately publishes realized effective fields in an unmarked Styled wrapper and passes `None` as the introspected snapshot. Its L0 requires preserving captured styles and query morphology. `01_core/src/compiler/eval/selector_matching.rs:640–654` uses authoritative snapshots when present, otherwise the content field path. The newly opaque unmarked Styled boundary loses this producer's fields. The existing `equation_where_filters_match_identity_body_block_and_captured_fields` test independently detects the fallback regression.

The frozen 173-case corpus contains no realized Equation occurrence query. Candidate-final scoped acceptance remains a statement about that corpus only; it does not close this new regression. Its raw comparator failure and full-parity-false status remain unchanged.

## Repair boundary and refuters

An authoritative occurrence snapshot at the realized producer can preserve captured field values without declaring external lexical style wrappers to be Equation constructors. This is a proposed boundary, not an implementation certification. The legacy native selector fallback must also keep its independently specified behavior, while ordinary language Styled reflection stays opaque. Do not weaken the failing test or broadly add constructor markers to external styles.

After repair, rerun the frozen 39 bilateral probes, retaining the earlier receipts, and require agreement for field projection and selection. Also rerun all original 173 controls, especially the three restored lexical set-text wrappers, invalid marker controls and authoritative empty snapshot cases. Any newly exposed external Styled fields or lost snapshot authority refutes the repair.


## Omitted/default occurrence supplement and frozen replay

`realized-query-defaults/receipt.json` adds 11 bilateral probes (2026-10-19 19:45 UTC; precise per-case timestamps and final source-input differences in receipt), SHA-256 `79abdce33e5c0a250da8e9e1347040ada172bf40d8038986a791296fbff478c7`. Omitted numbering/alt and explicit none both expose realized oracle fields in this observed order: func, block, numbering, number-align, supplement, alt, body, label. Values are Equation, true, none, end+horizon, text Equation, none, text x, and the source label. Both none-valued `.where` filters select the occurrence. These are realized occurrence semantics and do not certify raw-constructor omission semantics.

Two distinct occurrences certify query sequence order and labels, with alt values first/second in document order; an equation-free document preserves the empty result. Internal Some(empty) and preexisting-snapshot authority require the separately owned domain tests because this corpus does not expose a source-level snapshot constructor.

The separate `realized-query-manifest.json` freezes all 39 cases from the three receipts. `realized-query-replay.py --output NEW_DIRECTORY --validation-state BUILD_RECEIPT` verifies frozen receipt hashes, current build input hashes and binary identities, replays exact sources and argv against both binaries serially with a 60-second subprocess timeout, preserves all diagnostics except exact source-filename substitution, and requires every decoded observation to match the unchanged oracle. It has no partial-repair exceptions. Lists preserve occurrence order, and field values preserve defaults and labels. The original 173-case manifest is unchanged. Parent runs the replay after the repair build; this audit only compiled the Python script syntax.
