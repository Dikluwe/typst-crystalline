# L0 — stdlib/numbering
Hash do Código: 7f644758

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/numbering.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/numbering.rs:22`, `01_core/src/compiler/stdlib/numbering.rs:55`, `01_core/src/compiler/stdlib/numbering.rs:134` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_numbering, format_pattern, application of entities::numbering::Numbering and conversion of its result to Content. numbering(pattern, ..numbers) requires a pattern and nonnegative integer numbers; wrong types/negative numbers fail. String patterns recognize 1, a, A, i, I, א and ①, preserving prefixes, numeral pieces and suffix. Drop unused pattern pieces; when numbers exceed pieces repeat the last piece using its prefix, or its suffix when its prefix is empty. Patterns without a counting symbol fail.

Hebrew zero warns `the numeral system \`hebrew\` cannot represent zero` and emits decimal 0. Circled zero is ⓪; 1–50 use ①–㊿; larger values warn `the number {n} is too large to be represented with the \`arabic.o\` numeral system` and fall back to decimal. Preserve alphabetic, Roman and Hebrew formatting, including 15 -> טו.

Func patterns invoke apply_func once with numbers as positionals and return any resulting Value; content realization uses normal markup morphology (42 displays 42; none is empty). format_pattern is shared with counter.display, including circled numerals. realize_numbering is exposed for L3 orchestration; layouter/PageStore must not receive Engine or execute callbacks, and consumers must not duplicate Pattern/Func dispatch.

Calculated usize/u32-to-Int callback arguments use Args::from_parts with empty named and absent occurrences; retain aggregate span, order and callback count without invented lexical origins. Do not incidentally fix the existing u32 conversion or claim general diagnostic parity for synthetic callbacks. Preserve existing public helper visibility/signatures.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.


## Internal iteration maintenance

### Evidence before decision

compiler/stdlib/numbering.rs:168 contains the compiler's unused/visibility item.
Before-edit usage/active-dispatch research is in
`lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`,
with sources/L0, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working
tree, UTC, status and diffstat. This is not new bilateral parity evidence.

### Contract and acceptance

Traverse prefix/symbol pairs without enumerating an unused index. Preserve
order, numbers_iter consumption, formatter calls, errors and stopping when
numbers run out. Emitting apostrophe with push(char) preserves the same text.
This ADR-0127 internal continuous-flow correction adds no public signature,
product default or phase. Preserve language results/test expectations and
revalidate all targets, core suite, build, Clippy and lineage. Do not replace
the correction with warning suppression.
