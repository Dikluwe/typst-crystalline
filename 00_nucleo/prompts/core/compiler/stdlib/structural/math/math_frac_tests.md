# L0 — core/compiler/stdlib/structural/math/math_frac_tests
Hash do Código: d2d52ad3

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/math_frac_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/math/math_frac_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only tests of the frozen math.frac contract cases P01–P07, P16 and N01–N11, without candidate inspection. Observe the closed binding and positional required num/denom, content casts (none/Str accepted, Int rejected), only named style with vertical/skewed/horizontal values, omission remaining None versus explicit vertical, canonical repr and all causal fields. Duplicate named values must use ArgOccurrence sequence, never a collapsing map. Clone/equality/hash/map_content/map_text preserve style, both unparenthesizing bits and line, transforming only contents recursively. repr must not expose the bits even on a directly constructed carrier with both true. Missing binding, non-callability or inability to observe the carrier fails; no textual approximation, other math binding or productive implementation is authorized.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.


## Correct duplicate boundary after current-pin measurement — 2026-10-29

### Measurement before decision

raw-frac-core-all3 fails only p1427_binding_assinatura_casts_e_diagnosticos:
its origin-free Args sequence vertical then skewed calls the native directly,
bypassing source syntax, and expects duplicate rejection. The exact tested
sources/inputs/HEAD/dirty stat/UTC are retained by that receipt and its focused
source archive. math-frac-arguments-before2 records native accepting bound plus
call-site styles and rejecting literal source duplicates; an invalid bound value
still fails before a later valid one. Pinned native foundations/args.rs:222–237
casts every named occurrence and keeps the last; parser math.rs:480 and
patterns.rs:279 reject literal duplicates before the constructor. Args L0
requires retaining every causal value; missing origin is not evidence of a
literal argument list and cannot choose different semantic validation.

### Exclusive test correction, without dropping the negative

Supersede only the old duplicate-constructor expectation: preserve its exact
ArgOccurrence sequence with both valid styles and detached origins, then demand
Some(Skewed), the last valid value. Add a first-invalid/last-valid sequence at
the same synthetic boundary and demand boolean failure, proving no losing value
was discarded. Preserve every other input and assertion in the original test.
The actual-source literal duplicate negative remains required for both math and
code syntax in the frac owner's new tests; it is not removed or reclassified
as success. No suppression, tolerance widening or accepted wrong source case.

This repairs a stale test abstraction against measured current language
semantics; it is not a new public contract/default/phase or generic Args/With
change. L0 before test edit, archive the failed full-core receipt, then require
focused/full core and unchanged final build/lint/lineage/history/JPEG gates.
