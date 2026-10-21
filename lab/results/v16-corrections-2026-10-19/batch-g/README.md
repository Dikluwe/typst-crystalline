# Batch G — closed decisions and preserved causal identity

The eighteen remaining V16 findings are repaired on `codex/tekt-mature-layout`.
The complete pinned linter reports **0 errors and 0 warnings**, with **1,339 V19
and 905 V20 informational notes still enabled**. All 2,174 diagnostic objects
outside the thirteen source/owner pairs are identical to the baseline, including
messages, multiplicities and locations. This closes the recorded V16 inventory,
not complete Typst language parity. No linter, configuration, severity or
suppression was changed.

## Provenance and scope

Measurements used HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd` plus the exact uncommitted changes recorded
in each receipt. The final [build receipt](candidate-build-3/receipt.json),
[full-suite state](validation-final-3/validation-state.json.gz) and
[API/lineage audit](agents/core/api-scope-review-final13-3.md) bind all 2,142
inputs, the changed-file list, UTC and commands. The final product SHA-256 is
`ced5125ee4b42a6ad9ac36db432d01b0e6bdafd93d2e042f8ea56b3a966f3fd9`. Measurements precede the closure commit; source
hashes, rather than a version string, identify the tested implementation.

The oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Its [ratified build receipt](../../construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md)
is the language reference. The linter remains commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.

[The immutable baseline](baseline-inputs.json.gz), [original eighteen entries](cases.json),
[final scope](scope.json) and [closure ledger](closure-ledger.json) retain exact
ownership and evidence. V16-100 was already removed in F; it is not counted again.
Thirteen productive files have thirteen canonical L0 owners. Their twenty-five
effective Nuclei and all forward/reverse seals were checked. No public signature,
entity field, trait, default, compatibility boundary or pipeline phase changed.
The two scope extensions repair measured internal causes under ADR-0127's
continuous flow, with L0 first and actual RED→GREEN evidence.

## Causes repaired

| Findings | Decision and implementation |
|---|---|
| 049–050 | A failed bare-math absence proof triggers full traversal; it never proves absence. A mapper's `None` descends rather than discarding children. Preserve equation/location boundaries and selected Table visuals. |
| 051–052 | Explicit normalization and observation catalogues retain protected products, original/visual identity reservations, recipe floors and canonical sequence markers. |
| 074 | Structural Title/HTML/frame comparisons, ordered HTML attributes and Context-owned mechanical IDs are explicit. Both tuple axes demand a decision for future kinds. Language-only Context opacity is a private comparison projection; callbacks and original IDs remain untouched. |
| 088–089, 092, 095 | Reclassify suppressed visual/callback roots locally; retain strict unresolved/textual errors. Project actual Table row-cell versus standalone visuals correctly. Preserve frame islands and once-only frame/style/ID collection. |
| 093–094, 099 | Both independent contextual collectors recover Source-wrapped producers below Located roots without broadening the global mapper. Duplicate IDs retain first encounter order and last payload/style chain. |
| 096–098 | Font, context and retained-prefix checks use explicit conservative proof boundaries; the prefix proof preserves Arc identity and both future tuple axes. |
| 101–103 | Explicit ElementKind/list/enum complements preserve documented approximate query behavior; this does not certify compiled-query parity. |

An additional real-root public probe exposed a pre-existing Eq/Hash defect:
invisible carriers consumed Debug's visible depth budget. Across the same 366
fixed comparisons, public-hash violations changed from **248 to 0** and standard
Hash violations from **168 to 0**. The fix borrows and iterates four invisible
carriers before accounting for visible depth. It retains the visible depth limit,
nominal formatting, algorithms, seeds and causal/observation identity. Exact
[before](hash-before/receipt.json) and [after](hash-candidate-3/receipt.json)
receipts bind a real-root compiled library and unchanged probe source; this is
bounded evidence, not a universal Hash proof. The suspected nested Value Float
counterexample was **refuted** by actual execution and is not an observed debt.

The raw Table renderer's existing footer omission was also measured. Retained
footer data is still converted, but an unrendered branch cannot create a phantom
deep-heading diagnostic. A private emitted-state flag remains monotonic through
all twenty-one recursive edges, independently of cell selection. Positive
standalone and actual public resolved-footer controls still emit the exact
warning once. This neither suppresses emitted diagnostics nor certifies raw
footer language support; the omission remains open.

## Executable acceptance

- [Focused GREEN](integration-3/commands.json): **20 core + 36 infra tests**,
  no ignored tests, compiler errors or input drift. Valid semantic RED runs and
  all unsuccessful harnesses remain separate under `focused-*`.
- [Complete final run](validation-final-3/validation-commands.json): **12/12 gates**,
  including canonical paths, strict forward/reverse lineage, ownership/Nuclei,
  workspace build, domain/infra/wiring tests, doctests, all targets, strict Clippy,
  formatting and warnings-as-errors Rustdoc. The [freeze](validation-final-3/validation-freeze.json)
  records no input changes.
- [Actual enum evolution](structural-green-3/receipt.json): all **18 original
  selected matches** produce primary E0004 diagnostics. The immutable baseline
  absorbed each. [Six isolated tuple-axis checks](tuple-axes-green-3/receipt.json)
  separately demand decisions for new/old, old/new and new/new on both matches.
  Only copied core dependencies received compiler-suggested bridges, after raw
  witness recording; no copied infra target was repaired or executed. Two
  additional identity matches are reviewed separately, without inventing V16 IDs.
- [Equality CLI](equality-candidate-3/receipt.json) and [HTML CLI](html-candidate-3/receipt.json)
  replay fixed source-backed policies against the exact fresh product. Existing
  representation, unsupported Link and diagnostic differences remain explicitly
  open. DOM comparison retains DOCTYPE and checks successful diagnostic errors;
  its heading-warning allowance is confined to the previously declared debt.
- [Figure policies](figure-policy-replay-3/receipt.json) retain **84 accepted policy
  cases: 77 required, 5 debts and 2 negative controls**. [Contextual references](reference-corpora-3/receipt.json)
  retain **40 required cases across six matrices**. [Target controls](figure-target-controls-3/receipt.json)
  pass **10/11**: the existing SetPage header failure remains a strict failure.
  These counts are policy acceptance, not complete oracle equality.
- [Full diagnostic delta](lint-delta-3.json): eighteen V16 removals; twenty-five
  V19 and four V20 relocations retain identical payloads. Twenty-four additional
  V19 reminders remain visible. Grouped alternatives are not claimed individually
  covered at runtime.

The first integrated run exposed the public Table cell-selection defect; its
four actual CLI probes and failure are preserved. The first full-suite run
failed strict Clippy on five identity conversions in new fixtures; only those
conversions were removed, without changing assertions or adding allowances.
Earlier selector/private-Location/private-export harness errors and the artifact
path-guard failure did not execute semantic tests and are not labeled RED.

The next complete run passed all twelve gates and the actual hash/equality
controls, but the frozen HTML replay still rejected one required case:
`bundle-cell-heading` emitted `<p><h2>H</h2></p>` instead of a block heading.
That [failed replay](html-candidate/receipt.json) and the preceding successful
[twelve-gate snapshot](validation-final-2/validation-commands.json) remain
historical. Four additional controls reproduced two semantic failures while two
preservation controls passed. The document owner now selects the same borrowed
standalone visual/body as the existing inline renderer when deciding block flow,
without unwrapping cells globally or altering Table row selection. The unchanged
HTML policy is replayed against the newly rebuilt product, and all gates are
repeated for that exact final source state.

Independent verifier review also exposed a JSON-parser false-positive path.
The original Lab scripts and their failing negative controls were preserved;
candidate output parsing and complete prior-policy cardinality checks were
hardened before the final replay. No case, oracle value or acceptance policy was
changed to accommodate a candidate result.

## Remaining work and history

[Open obligations](open-obligations.json) retain the bounded G gaps and hash-pinned
inherited ledgers. Zero V16 does not close unsupported HTML Link, approximate
queries, diagnostic/source-origin differences, raw-footer support, existing
Metadata/State mechanical comparisons, full Figure/Grid/physical-page parity or
earlier math callback decisions. No new runtime or performance improvement was
measured. Source/Located orientation controls establish a valid domain case,
not that the frontend currently generates every tested orientation.

The organization helped trace each repair to its exclusive owner and separate
current L0 contracts from historical measurements. All build/probe artifacts and
unsuccessful attempts stay in `lab/`; the same branch and ancestry are preserved.
The [previous diagnostic](diagnostic-history/v16-correction-progress-2026-10-19.md)
and [its provenance](diagnostic-history/provenance.json) retain the A–F checkpoint.
