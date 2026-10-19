# L0 — stdlib/primitives-constructors/version
Hash do Código: 185dcd10

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:b7a180bc63908fdeadc405b120c0db8b1c2587f2217d815980b9db35f0e282c4

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/version.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/primitives_constructors/version.rs:17`, `01_core/src/compiler/stdlib/primitives_constructors/version.rs:34`, `01_core/src/compiler/stdlib/primitives_constructors/version.rs:42` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_version rejects named arguments and accepts one dotted numeric string through Version::from_str, one array of nonnegative integer components, or any number of nonnegative integer positionals. Zero components returns Version::default. Reject malformed strings, negative/non-integer components and pre/build components. Return Value::Version(Arc<Version>); preserve pure callable-type delegation.

Own the single internal version.at adapter and its static/bound glue, not the Version::at formula. version_type_field discovers only at; the module remains private and integration helpers pub(crate). The public short function name/repr is at. Consume strict Version self, then strict Int/i64 index, reject leftovers, then delegate value and index error to Version::at. No additional string/Float/Bool conversion. Positive out-of-range indices extend by zero; negative indices are relative to explicit components. Preserve the entity's out-of-bounds message on the whole call.

Required supplied only by name reports `the argument \`self\` is positional` / `the argument \`index\` is positional`, on the full named occurrence with the respective remove-self:/index: hint. Missing reports `missing argument: self` / `missing argument: index` on the whole call. Cast reports `expected version, found <type>` / `expected integer, found <type>` at value_span. Reject the first causal remaining occurrence: `unexpected argument` positional or `unexpected argument: <name>` named. Preserve origins with aggregate fallback only when detached.

Bound calls inject the receiver once and delegate to the same adapter; evaluation/spreads happen once beforehand. Static aliases/With preserve resolved-function identity and full-call anchoring, not callee spelling. Extraction of bound .at without calling remains `unknown version component`, owned by field access. Do not create a second formula/parser, registry or entity, or change major/minor/patch, repr/display/comparison/PARITY_VERSION. Existing absent-component tensions remain separate debt.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
