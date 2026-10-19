# L0 — compiler/eval/math/delimiters
Hash do Código: 35140159

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/delimiters.rs`. At HEAD `e754ab364a221dbe344852178e84bd42b02c463e` with a clean tree, `01_core/src/compiler/eval/math.rs:1555-1671` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contract

Parse delimiters and vector-syntax options with existing errors. Preserve signatures, language semantics and diagnostics from `00_nucleo/prompts/core/compiler/eval/math.md`; preserve pipeline phase, defaults and compatibility. Extraction remains internal to L1, without I/O or mutable global state.

## Acceptance

The L1 suite and math tests must pass; `crystalline-lint .` must validate ownership and hashes. Do not infer performance gains.

## Nominal Value exhaustiveness

### Evidence before decision

M1 at compiler/eval/math/delimiters.rs:19 and M2 at :28 were read with current
L0 before prescription. The uncommitted tree has HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`;
`lab/results/construction/diagnostics/tekt-v16-math-value-2026-10-16/before.json.gz`
retains UTC, status, git diff HEAD --stat and full hashes. Value has 39 variants;
the measured fallback arms are internal and currently uniform. These are Rust
evolution warnings, not evidence of functional failure or improved AI reading.
Do not infer upstream intent.

### Contract and acceptance

M1 parse_delim_val accepts Str/Symbol, None and Array; explicitly group the other
35 current variants to return None in the same arm. M2 parse_delim_char accepts
Str/Symbol and None; the other 36 return None. Preserve string aliases, Unicode
pairs, the first character of Symbol/string and every array default: invalid/
empty first entry becomes NUL, absent/invalid second repeats the first, excess
entries are ignored.

Replace only each selected wildcard with the nominal complement of current
variants and retain one shared result body. Informational V19 is legitimate for
identical results/effects and remains visible. Do not duplicate bodies, create
a facade helper, use a catchall binding/exemption or change configuration to
hide V16. Unused payloads may keep internal `_` patterns within named variants.
Public signatures, carriers, phase, defaults and diagnostics remain unchanged.

Before implementation write local accepted/rejected and specified morphology/
presence controls. Preservation controls pass before/after; RED is current V16
and evolution in a real workspace copy: a sentinel added to copied Value must
be rejected at each selected decision (the current fallback absorbs it).
GREEN requires E0004 at those points, zero selected V16, only equivalent new
V19 and no other new diagnostic. Controls do not claim execution coverage of
all 39 variants or global vanilla parity.

Reseal direct/reverse lineage only for this owner/consumer; validate build, full
workspace, all-targets Clippy with warnings/unfulfilled expectations as errors,
fmt, strict Rustdoc, V1/V5/V15/V26 and reverse lineage. Heavy commands run
sequentially under 12 GiB, zero swap and one Cargo job. Preserve commands,
source/hashes and UTC, and unrelated source.
