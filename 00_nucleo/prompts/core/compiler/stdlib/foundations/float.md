# L0 — stdlib/foundations/float
Hash do Código: d902511c

**Baseline**: vanilla ratificado `a51e02804`

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/float.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/float.rs:20`, `01_core/src/compiler/stdlib/foundations/float.rs:59`, `01_core/src/compiler/stdlib/foundations/float.rs:100` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own the closed float field/method dispatch and the shared pure native formulas. The constructor belongs to `00_nucleo/prompts/core/compiler/stdlib/foundations/cast.md`. Preserve constants `float.inf` (positive infinity, repr `float.inf`) and `float.nan` (NaN, repr `float.nan`, not equal to itself); these are values, not functions.

Static functions have the short public names/reprs `is-infinite`, `is-nan`, `signum`, `from-bytes` and `to-bytes`. Float/Int is the parameter cast for self; do not use the constructor's broader cast. is-infinite is true only for either infinity; finite values, ±0 and NaN are false. is-nan is true only for NaN. signum returns Float +1 for positives and +0, -1 for negatives and -0, and NaN for NaN. Preserve Int.signum's separate Int result.

from-bytes requires Bytes and optional endian ("little" default or "big"). Four bytes decode binary32 and promote to binary64; eight decode binary64, preserving encoded infinities, NaN, subnormals and signed zero. Other lengths fail `bytes must have a length of 4 or 8`. to-bytes accepts self, endian and u32 size (default 8, only 4/8 valid), returns Bytes, never Array. Size 4 rounds to binary32 with its underflow/overflow/subnormal/signed-zero behavior; size 8 preserves binary64. Invalid u32 size fails `size must be either 4 or 8`.

Bound Float calls insert the receiver exactly once and delegate to the same native. Float.from-bytes must reject the injected Float as Bytes (`expected bytes, found float`); Bytes acquires no from-bytes method. Static coercion must not give Float methods to Int. Uncalled bound extraction remains `cannot access fields on type float` at the field name. With occurrence carriers, prepend one positional receiver whose span/value_span are detached, retain every original occurrence/order/origin, and rebuild using Args::from_occurrences; absent occurrence carriers use Args::from_parts. Preserve the aggregate diagnostic anchor; never invent lexical origins for a supplied f64.

For signum/from-bytes/to-bytes, evaluate arguments once; consume required self/bytes, then endian, then size where applicable, then reject the first causal leftover. Missing positional is `missing argument: self` or `missing argument: bytes` on the whole call. Required supplied only by name reports `the argument \`self\` is positional` / `the argument \`bytes\` is positional` on the complete named occurrence with the corresponding `try removing \`self:\`` / `try removing \`bytes:\`` hint. Cast errors use value_span: `expected float, found <type>`, `expected bytes, found <type>`; endian rejects invalid strings with `expected "big" or "little"`, other types add `, found <type>`. Size casts precede domain validation: `expected integer, found <type>`, `number must be at least zero`, `number too large`. Extra positional uses `unexpected argument`; unconsumed named uses `unexpected argument: <name>` at the full occurrence. Length/size domain errors use the whole call.

Keep alias/With behavior based on resolved native identity, causal spread order and duplicate-named handling. Do not read source, recompute offsets, discard known origins, introduce reflection, registry fallback, dynamic dispatch or a second formula/parser. Exact IEEE bytes are observable results of these APIs, not a requirement to copy upstream internal representation.

Predicates retain their established closed arity/named validation and aggregate Args.span diagnostics; do not apply the later conversion-native parser to them implicitly.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
