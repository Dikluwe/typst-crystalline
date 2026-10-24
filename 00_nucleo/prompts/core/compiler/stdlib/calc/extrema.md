# L0 — stdlib/calc/extrema
Hash do Código: e08e2b19

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/calc/extrema.rs`. At HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` with a clean tree, `01_core/src/compiler/stdlib/calc.rs:517-600` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. This decision only separates algorithms by behavior.

## Contract

Implement numeric min, max and clamp, preserving mixed promotion, nonempty input, and the requirement that the lower bound not exceed the upper bound, with existing diagnostics.

Parity details, types, diagnostics and bindings remain in `00_nucleo/prompts/core/compiler/stdlib/calc.md`. Extraction preserves public signatures, module functions, phase, results and error policy. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and calc tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains from this separation.


## Ordered clamp bounds without runtime panic

### Measurement before decision

The frozen current-pin bilateral receipt
`lab/results/panic-elimination-2026-10-22/clamp-before/receipt.json` (SHA-256 `7ff3c091276ac803d01f1a79b64b958ef32743f5db48dc7a3811a46d1368178b`)
ran from 2026-10-22T18:51:49.193690+00:00 through 2026-10-22T18:51:54.264526+00:00 at HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`,
uncommitted working tree; it retains the exact dirty-file stat, productive-input
hashes, fixture text, argv, outputs and candidate/oracle identities. The candidate
SHA-256 was `6c029a4b48ab09601880d93f484ae1981987ec48e9a0f932dae75c0e51769163`; the oracle commit was
`bd5ebec285505889e8ff1d4f68804ab147a699f3` and its executable SHA-256 was
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

`#calc.clamp(5, 10, 0)` exits 101 with a Rust integer-clamp panic in Tekt;
vanilla returns an ordinary diagnostic, `max must be greater than or equal to min`.
A NaN minimum or maximum makes both binaries exit 101 in floating clamp. This
upstream defect is not a language requirement (ADR-0107); the user's explicit
no-panic requirement authorizes rejecting unordered bounds as an ordinary
error. The existing float-reversed branch already returns Err but with an older
diagnostic. The measured valid controls retain integer/mixed results, an equal
interval, signed zero, infinities and a NaN value with ordered bounds.

`01_core/src/compiler/stdlib/calc/extrema.rs:75-85` calls integer clamp without
checking bound order, and checks only `lof > hif` before floating clamp, which
cannot exclude NaN. This is an input-validation defect, not a request to change
all IEEE propagation. ADR-0101 remains categorical; do not route the value
through guard_float or reject valid infinite bounds.

### Required behavior

Keep native signatures, argument binding, arity, named/type precedence and
integer versus mixed floating promotion. Before clamping, reject reversed
integer or floating bounds with `max must be greater than or equal to min`.
Reject a NaN minimum with `min must not be NaN`; reject a NaN maximum with
`max must not be NaN`. After existing type validation, check the minimum NaN
before the maximum NaN. Return exactly one SourceDiagnostic through the existing
SourceResult; no panic capture, process handler, silent default, synthetic success
or new warning is allowed.

Anchor reversed-bound and NaN-maximum errors at the third positional occurrence's
value_span, and a NaN-minimum error at the second. When origin data is absent,
use detached; an explicitly detached value span stays detached. Never fabricate
an origin by searching document text or substituting the aggregate Args span.
Retain args and occurrence order untouched; dispatcher owns outer call traces.

Use total ordered comparisons/selection rather than a standard clamp with a
runtime precondition. A NaN value with ordered bounds remains NaN; infinities
and signed-zero identity within a valid interval remain unchanged. Existing
min/max behavior is out of scope and must remain intact.

### Acceptance and classification

Add failing module controls for reversed integers and floats, NaN bounds,
ordered-bound NaN value, infinities, signed-zero bits, extreme integer values,
origin accuracy and existing validation precedence. Preserve all old tests.
Re-run the exact bilateral sources: all three formerly crashing Tekt inputs
must return ordinary exit-1 diagnostics without a Rust panic; valid controls
must retain their measured behavior. Retain vanilla's NaN failures as evidence,
not as expected Tekt behavior, and retain unrelated diagnostic debts explicitly.

This correction follows ADR-0127 continuous flow: existing private native
SourceResult, no public signature/field/trait/default mode or phase change.
Reseal this exclusive owner, pass meaningful RED→GREEN tests, the core suite,
workspace build, Clippy and Tekt/path/reverse-lineage gates. A finite clamp
qualification does not certify the entire project free of panic.
