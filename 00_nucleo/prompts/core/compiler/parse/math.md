# L0 — parse/math
Hash do Código: 9c712e67

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:b6a95a7684745e45903417a81eabc019a17eb3b8fab9fe246e8840978d5d42c8

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/math.rs`

## Evidence and classification

`01_core/src/compiler/parse/math.rs:24` (pubsuper) fn mathp: &mut Parser, stop_set: SyntaxSet) {); `01_core/src/compiler/parse/math.rs:32` (pubsuper) fn math_exprsp: &mut Parser, stop_set: SyntaxSet) -> usize {); `01_core/src/compiler/parse/math.rs:50` (fn math_exprp: &mut Parser) {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own math expressions, attachments, fractions, roots, delimiter groups, implicit calls and math arguments. Preserve Math, MathAttach, MathFrac, MathRoot and MathDelimited morphology according to the existing token/trivia and precedence rules.

Preserve hat/underscore/primes chains, alphabetic-function recognition and implicit call behavior. Math function/root precedence remains 2 in this consumer. Retain unparenthesizing, delimiter classification and positional/named/semicolon-array distinctions. Duplicate named arguments retain their diagnostic and location. Embedded code delegates to `00_nucleo/prompts/core/compiler/parse/code.md`; mode/lookahead restoration belongs to `00_nucleo/prompts/core/compiler/parse/parser/state.md`. No evaluation or mathematical layout is owned here.

## Closed math-operator recognition

### Source evidence before classification

Inspection at clean HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
2026-10-19T10:22:03Z, reads `01_core/src/compiler/parse/math.rs:197–210`
and its full caller `:56–189`. The caller checks its stop set, then uses
`math_op`'s `None` to end the infix/postfix loop without consuming that token.
`01_core/src/entities/syntax_kind.rs:12–293` defines the closed token domain.
The existing guard-refused MathPrimes/Bang cases reach the same `None` as
other nonoperators. This is static implementation evidence, not an oracle run
or a claim of upstream intent. The baseline occurrence is V16-048 in
`lab/results/v16-cause-analysis-2026-10-19/eval-realize-inventory.json`.

### Preservation and exclusive decision

Close the SyntaxKind decision at `math_op` itself. Preserve Slash as
MathFrac/Left/1; Underscore and Hat as MathAttach/Right/2; adjacent MathPrimes
as MathAttach/no infix association/2; adjacent Bang as Math/no infix
association/3. Put the trivia condition inside the respective nominal
MathPrimes/Bang arm: either token with trivia returns `None`. The complete
remaining current SyntaxKind domain explicitly returns `None` because it is
not an operator of this Pratt subgrammar. Those tokens can be valid operands,
delimiters, markup/code syntax, trivia, end markers or errors in their own
routes; the result neither rejects them globally nor discards them. Preserve
stop-set precedence, cursor/text/span coverage, attachment chaining,
unparenthesizing and implicit function/root precedence 2.

This repairs an internal evolution obligation under ADR-0127 continuous flow;
it changes no public contract, default, grammar admission, phase or
compatibility. Preserve static dispatch without suppression, a hidden
`if let`/`matches!`/helper fallback, or an optimistic default for a future
variant. A grouped nonoperator domain is permitted only with the rationale
above and remains visible to the relevant Tekt checks. A current language
fixture changing grouping, diagnostics, consumed tokens or spans refutes the
behavior-preserving classification; resolve that before proceeding.

### Regression and exact-site enum-growth acceptance

Add colocated tests with explicit successful fixture extraction: inspect
parsed math for `a/b/c`, `a^b_c`, `a^b'_c^d`, adjacent versus spaced primes and
Bang, and implicit-call/root inputs such as `a(b)/c` and `√x/2`. Check language
syntax grouping/attachment and fraction operands, no parser errors for valid
inputs, and preserved text coverage; a test that merely finds some nested
fraction is insufficient. Test the exact five operator mappings and rejected
trivia paths, plus nonoperator End/Error/Space/MathIdent/delimiters that must
return `None` without consumption. Retain existing
`compiler::parse::tests::parse_math_basico` and
`compiler::parse::tests::expressao_matematica` as integration controls; they
are not substitutes for the new grouping and trivia assertions.

Use disposable complete copies of the actual workspace for the enum-growth
proof, preserving its manifests, modules, real enum definitions, dependencies
and function bodies. Add a fresh SyntaxKind sentinel only in those copies and
compile the real crate. Do not extract `math_op`, substitute an enum, stub its
dependencies or use a toy harness. Record the original wildcard's lack of an
exhaustiveness diagnostic at this exact decision, then require E0004 at the
repaired `math_op` SyntaxKind match. Other exhaustive matches may also fail:
their errors neither prove nor replace this site's obligation. If compilation
cannot produce an attributable diagnostic here, the proof is incomplete.
SyntaxKind growth must force review of this operator/nonoperator decision;
no sentinel enters productive source. Record both workspace snapshots, source
digests, sentinel-only changes, compiler command/version and site diagnostics.
Run regression controls against the original implementation before changing
it, then revalidate after repair. Structural RED and the exact-site growth
proof remain separate from passing runtime controls. Require valid 1:1
ownership and nucleus preflight before restricted resealing. These are enduring
acceptance requirements, not results already obtained.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
