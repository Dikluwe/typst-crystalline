# L0 — stdlib/html/attr_cast
Hash do Código: 524d9144

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/html/attr_cast.rs`. At HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b` with a clean tree, `01_core/src/compiler/stdlib/html.rs:1490-1692` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-html-2026-09-30-before.json.gz`.

## Contract

Own exhaustive attribute-value coercion dispatch by AttrKind. Preserve public signatures, defaults, diagnostic classes, element morphology and validation order specified in `00_nucleo/prompts/core/compiler/stdlib/html.md`. This extraction only separates responsibility within L1; dispatch remains static, without I/O or mutable global state.

## Acceptance

Preserve `html` tests, the L1 suite and valid `crystalline-lint .` results with correct ownership and hashes. Do not infer performance gains from separation.


## Closed decision coverage

### Measurement before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, the source/owner hashes
and exact inspection state are in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/read-only-audit.json`.
The baseline binary build and UTC-stamped bilateral commands/output are retained
in `lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/baseline-enabled-probes.json`;
the separate HTML textual projection receipt is `baseline-html-projection.json`
in that directory. The initial feature-disabled HTML run is setup evidence only.
The oracle is pinned to commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
These observations do not infer upstream intent or certify unrelated parity.

`attr_cast.rs:105–124` delegates the non-Bool/non-None complement to
`attr_values.rs:16–33`, which rejects invalid tokens and types; it does not
silently accept them. Public HTML probes distinguish true/false/none and allowed
tokens. Error unions and general `.fields()` still differ from the oracle.

### Decision and acceptance

Close both OnOffOrNoneOrEnum and PresenceOr decisions over every current Value
variant. Keep Bool true/false, allowed None and Str semantics, omitted-versus-empty
attributes, error text, supplied span and validation order. Every other named
variant continues through the same rejecting validator. Future Value variants
must cause a local exhaustiveness failure; do not move the open fallback into a
helper or suppression. Tests cover both bool values, None, each allowed string,
invalid string, integer/content and exact error/span; existing typed HTML tests
retain attribute order/body observations. The narrower error unions and general
fields gaps are open obligations, not accepted parity. No public API/default or
phase change; this is ADR-0127 internal continuous flow.
