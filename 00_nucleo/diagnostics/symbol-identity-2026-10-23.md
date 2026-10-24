# Public symbol identity — batch E validated

Subsequent [R4 correction](math-fields-r4-2026-10-23.md): native Symbol slots and
qualified casts now retain their content identity; numeric MathText fields close
the corresponding E observation. Original measurements below retain their own
frozen state and the legacy checker policy remains separate.

**Subsequent R3 update:** [R3 geometry correction](math-geometry-r3-2026-10-23.md) resolves the seven additional geometry differences across all 19 original render probes, with full PDF/SVG checks at 0.001 pt and preservation of A–H/R1/R2. Measurements below remain their historical snapshot; additional baseline limits are documented in R3.

The 16 D5 identity differences are corrected. The [approved declaration](../prompts/core/entities/content.md) is materialized. `Content::Symbol(EcoString)` now
retains native symbol identity and the complete selected Unicode string. The
repository owner explicitly approved this public addition on 2026-10-23 before
materialization. All existing working-tree changes were preserved; no commit,
reset or oracle update was made.

## Measurement and provenance

The pre-E [baseline](../../lab/results/symbol-identity-2026-10-23/baseline.json)
and [source archive](../../lab/results/symbol-identity-2026-10-23/baseline-source.json.gz)
retain HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working
tree**, exact `git diff HEAD --stat`, productive/L0/config hashes and binary
identities. The frozen pre-E product is the validated batch-D debug binary,
SHA-256 `7d31f0f8c49d5797dc4e57211ff551dd5bb89211f1a7bb2059736d8740be3daf`.

The validated candidate is separately frozen as
`lab/results/symbol-identity-2026-10-23/tekt-debug`, SHA-256 `00a67d544c61ddd59b88ffba2b40267392354b8a1f849889ad4b26c02075f060`.
[Candidate state](../../lab/results/symbol-identity-2026-10-23/candidate-state.json),
[complete source archive](../../lab/results/symbol-identity-2026-10-23/source-final.json.gz)
and [patch against the dirty pre-E baseline](../../lab/results/symbol-identity-2026-10-23/validated-change-vs-dirty-baseline.patch)
make that state reproducible without treating HEAD as a sufficient source receipt.
Build/test commands, UTC intervals, environment and resource limits are retained
in the validation and focused-test receipts. Productive/L0 inputs match all final validation epochs. After successful tests,
the all-target build exposed the missing variant in the nonproductive lab
benchmark catalogue. Only its exhaustive classifier and fixture were corrected;
`benchmark-repair.json` records both complete files and hashes. The prior eight
passing gates are retained, and all four remaining gates were rerun in
`validation-tail/` with a fresh input freeze. `validated-state.json.gz` archives
the final complete inputs and the exact two-file difference from measurement.

The oracle remains `lab/typst-original/target/release/typst`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`, commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`. Observer/font hashes, exact argv,
expressions, input hashes, raw stdout/stderr, parsed values and per-command UTC
are retained with the [completion receipt](../../lab/results/symbol-identity-2026-10-23/completion.json)
and its linked raw measurements. No moving upstream or version-string identity
was substituted.

## Cause and correction

The shared display adapter converted `Value::Symbol` into Text; independent math
paths used MathText or MathIdent. Exporting a different JSON name alone would
have left function identity, fields, equality and insertion semantics incorrect.

The approved native leaf stores selected text only, including variation selectors;
it does not retain the raw symbol's modifier catalogue or namespace. Symbol
contents with equal selected strings compare equal, while ordinary Text and raw
Symbol values remain distinct. Its callable element function differs from the
global raw-symbol constructor, accepts complete strings such as `"xy"`, exposes
the `text` field and supports native selectors. Missing/wrong-type constructor
errors now use the measured call/value anchors.

Insertion, native reflection, field projection, equality, representation,
traversal, origin transport, layout and export handle the new closed-enum leaf
explicitly. Each changed productive unit retains its own L0; shared invariants
are pinned through the [symbol identity nucleus](../prompts/_nuclei/content/symbol-identity.toml).
No Dynamic carrier, style marker, exporter-only identity or additional public
ElementKind/struct/trait/signature was introduced.

The realization boundary is contextual: non-math symbols become text before
native show rules and textual grouping; mathematical symbols receive regex per
element and retain native identity for native element rules. Compatible captured
text/symbol/text runs coalesce outside math. Mathematical glyph styling maps each
character of a Symbol string, preserving variation selectors; selected text is
not reinterpreted as a symbol name. Legacy MathIdent/MathText carriers and numeric
or quoted math text retain their existing behavior. No pipeline phase moved.

## Acceptance

- The original 198 expressions were rerun: all 16 carrier differences now match,
  all 146 prior matches remain matches, and the 36 local extensions retain their
  success/error classification. Extensions are not counted as parity successes.
  Original stderr is preserved for all 198 expressions.
- Markup, actual code tails, emoji, runtime symbols, math graphemes/shorthands,
  styled bodies, callable identity, equality, fields and selectors are covered.
  The eight additional primary controls match exact stdout/stderr and exit,
  including constructor errors and rejection of raw-symbol methods on content.
- Nineteen render controls cover all 16 symbol names, Unicode selectors,
  mathematical styles, multi-character strings, native/text/regex show rules,
  adjacent text and per-element mathematical regex behavior. All text results
  match. The complete original 16-symbol inventory passes independent SVG
  glyph/rule positions at 0.001pt; additional geometry failures remain failures
  as described below.
- The 16 math continuation, 16 prose continuation and eight prior layout controls
  preserve line/page membership and text. Mathematical SVG glyph origins and rule
  geometry pass independently. The previous PDF `prefix-text` xMax precision
  failure remains a failure; it was not relabeled as a pass.
- Seven Rust regressions have recorded RED evidence and pass together in the
  [focused GREEN run](../../lab/results/symbol-identity-2026-10-23/focused-green.log).
  The complete validation passes: 9,845 Rust tests, zero failures,
  7 existing ignored. Python CI, structural checks, strict/reverse
  lineage, workspace/all-target builds, doctests, strict Clippy, formatting and
  rustdoc all pass. The 34 Python CI tests and eight benchmark-harness tests
  also pass after the benchmark repair. Reverse lineage covers 1,003 consumers. Full crystalline-lint
  has zero errors/warnings; informational notes remain. `git diff --check` passes.

## Bounds and preserved evidence

The original 16 names are `chess.king.white`, `keyboard.tab.r`,
`ordinal.feminine`, `eq.gt.slant`, `equiv.lt`, `gt.eq.lt.slant`, `lat.eq.slant`,
`lt.eq.gt.slant`, `not.inv`, `prec.eq.slant`, `smt.eq.slant`, `succ.eq.slant`,
`tack.r.double`, `tilde.lt`, `trademark.mc` and `spacebar`. This is finite
acceptance of D5, not a global parity percentage or certification of every
symbol-related constructor/field.

Additional probes retain existing debts outside D5: numeric MathText's missing
`text` field, incomplete `fields()` projection for some enclosing mathematical
elements and rejection of raw symbols by the qualified fraction constructor.
Their pre-E replay and current observations are retained; they are not silently
counted as passing. The 36 extensions and the A–D PDF/hyphenation limitations also
remain separately classified.

Of the 19 additional render probes, 12 pass the strict 0.001pt SVG-origin
comparison and seven fail. The mixed prose/variation-selector control retains
its pre-E 2.136pt horizontal discrepancy. Five standalone inline-math controls
retain a 0.9211875pt baseline discrepancy; their glyph origins are unchanged from
pre-E even where the intended text changes. The new multi-character Symbol
constructor probe has correct glyph codepoints but a remaining 0.492pt advance
mismatch against upstream (the pre-E `.func()` was a different text constructor,
so it does not establish that this is an existing native-Symbol defect). These
are measured render debts beyond the original 16 identity differences. They are
not certified as geometry parity, and no tolerance was widened. The final
completion receipt retains all values and counterfactual observations.

Failed/interrupted candidates and their source/binary receipts remain in
`candidate1/`, `candidate2-interrupted/` and `candidate3-adjacent/`. Initial RED,
realization RED, multi-character RED, diagnostics RED and environment/sealing
attempt logs remain alongside them. `validation/build-all-targets.log` retains
the benchmark compile failure; its replacement pass is in `validation-tail/`. No interrupted or failed gate certifies the
final candidate. The initially named code-tail control was literal markup; the
separate corrected control tests actual evaluated code tails. Likewise, the
initial single-letter math `f(...)` probe was literal mathematical notation; only
the corrected `#f(...)` source supports the multi-character constructor finding.
