# L0 — compiler/stdlib/tests/transforms
Hash do Código: 5c7dccf2

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/transforms.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the transforms regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## Independent numeric expectation

### Evidence before decision

stdlib/tests/transforms.rs:80 contains the disallowed_methods call flagged by
current Clippy. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working
tree; `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz`
retains source/owner/config/libm/compiler/UTC/status/diffstat. host-math compares
backends before choosing: 4968 cases, 69 trig/atan2 differences up to 1 ULP,
without numeric-class or signed-zero change; powers/hypot agree in the sample.
These are host measurements, not universal proof or new vanilla parity.

### Contract and acceptance

The 15-degree skew test uses independent analytic tan(15°)=2-sqrt(3), preserving
1e-9 tolerance and the 45-degree check. Never use the candidate's trig function
to build its oracle. This ADR-0127 internal correction adds no public contract,
default or phase. Preserve geometry/language tests and compare measured SVGs
before/after, reporting differences; geometry comparison does not certify all
cases/platforms. Validate build/all targets/core/Clippy/lineage without lint
suppression or compiler-byte identity as acceptance.

## Approved typed transformation tests and incomplete scope

### Evidence before decision

`lab/results/construction/diagnostics/transformacoes-2026-10-14/results.json.gz`
records lost identity/presence, named x affecting y and incorrect casts.
Supplemental controls confirm string/none body, factor after body, two-auto error
only at layout and omitted fields distinct from explicit. Manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`
was approved, with receipt under
`lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14`.

### Test contract

Test operations through language fields/identity, without requiring an early
matrix. Replace obsolete fixtures accepting Float as Angle/Length/Ratio, named
rotate angle or two scale factors: the pin rejects those calls. Cover omission,
zero, false, auto, partial axes, units, leftovers, causal spans, string/none body
and maps preserving operation. X3–X5 remain in the approved contract and are
still to be completed within that same materialization scope; they are not
silently closed by X1/X2 approval or this documentary consolidation.
