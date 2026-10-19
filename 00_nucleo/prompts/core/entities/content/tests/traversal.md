# L0 — entities/content/tests/traversal
Hash do Código: 07c2a4c3

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for bottom-up `map_content`, recursive `map_text`, terminal preservation, and payload reconstruction. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

## Closed bottom-up callback decisions

### Evidence before decision

Source inspection at HEAD `059d1a0ff` on 2026-10-19:
`01_core/src/entities/content/tests/traversal.rs:71–94` uppercases Text and
asserts that Strong observes its already-transformed body; its wildcard returns
no replacement. `01_core/src/entities/content/traversal.rs:670–674` retains
processed descendants when a callback returns None. Thus no replacement does not
mean skipping a subtree. This is a local traversal-mechanics qualification; it
does not independently certify language parity or upstream intent.

### Decision and acceptance

Retain Text replacement and the independent Strong assertion. Enumerate every
other current nominal Content alternative in the callback with the same explicit
no-replacement policy. Keep the decision here, not behind a helper or predicate.
Extend the actual callback fixture with a Space terminal, an Emph container with
a Text descendant, and siblings in a Sequence. Assert transformed text, retained
container kinds, space preservation, child order and the Strong callback count.
All existing traversal tests and assertions remain. Descendants lost after None,
reprocessed replacement text, or changed sibling order refute the repair.

Before changing the callback, run the expanded behavior test against the old
implementation. Structural RED is the old decision accepting enum growth; a
Content sentinel in a disposable real workspace must make the repaired callback
fail with E0004 at its own site. Preserve the source state, UTC and compiler
diagnostics. This is ADR-0127 continuous flow after owner sealing, with no public
contract, product default, phase or compatibility change.
