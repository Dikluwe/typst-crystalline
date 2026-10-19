# Math fixture observer audit

Scope: V16-013, V16-014, V16-015 and V16-016. Exclusive consumers are the
four `math_{cases,frac,matrix,root}_evaluation_tests.rs` files under
`01_core/src/compiler/eval/`, with their canonical individual L0 owners.
Inspection checkpoint: `059d1a0ffa9523dbe21af6860a215acaeef55d73`.
The owner revisions record inspection UTC, original source SHA-256 and original
L0 SHA-256. Subsequent compilation and lineage results belong to coordinator
receipts; this audit does not independently claim a passing compilation.

## Cause and preservation boundary

Each private fixture observer previously used a neutral wildcard to stop at
unsupported Content variants. That allowed future enum growth to escape review
at the observer decision. The wildcard did not authorize a general visitor, nor
did its negative result establish absence of a target inside a stopped wrapper.

Cases, fractions and matrices descend only through Equation, Styled,
MathDelimited, Sequence and MathSequence. Roots descend only through Equation,
Sequence and MathSequence. In particular, Styled and MathDelimited remain opaque
for roots. A matching target is terminal even when its children contain another
target. Sequence search remains depth first in encounter order, with first-hit
selection. These are bounded test-observer mechanics, not a language identity
or parity definition.

Each L0 names every current nominal miss and its common reason: the variant is
outside that observer's allowed fixture paths. Existing constructor semantics,
styles, explicitness, argument order, errors and observation assertions remain
unchanged. Existing cases/matrix tests compare bare evaluation with qualified
native constructor calls; fraction tests exercise bare, slash and qualified
source; root tests exercise bare syntax and its observation relation.

## Controls added before the decision changes

Each consumer adds independently scoped `v16_b_` controls for:

- Direct target identification and selection of the outer target over a nested
  target. Pointer comparison is confined to this mechanical fixture observation.
- Every allowed wrapper, including nested wrappers and both sequence kinds.
- Distinct first/later targets, reversed sequence order, and a stopped decoy
  before a supported hit.
- Strong and MathStyled wrappers containing actual target decoys; root also
  tests Styled and MathDelimited decoys. Each negative is paired with a supported
  later sibling so a missing observer cannot satisfy the whole control.
- Empty content, plain text, empty sequences, and an allowed wrapper without a
  target.

The helper bodies were checked byte-for-byte against HEAD after tests were added:
all original wildcard decisions remained unchanged for behavioral baseline
measurement. Only coordinator resealing had changed lineage headers. No existing
math assertion was deleted or weakened. Formatting was restricted to these four
files with workspace style edition 2021.

Runtime preservation must pass on both old and repaired decisions. A separate
structural proof must inject a temporary variant into the real Content domain
in a disposable complete workspace and attribute E0004 to each repaired observer
match; unrelated errors do not close these findings. The temporary sentinel must
never enter productive sources. All results must be read from the coordinator's
frozen-state receipts before closure.

## Cross-review

Read-only review of the storage and custom-style observer changes found a vacuity
risk in the new per-fixture changed-parent control: using the observed replacement
count alone could accept skipped children. This was reported to the coordinator
with the independent distinction that MathPrimes and MathAlignPoint are the
childless fixtures. No edits to another worker's files were made. No widening or
weakened prior assertions were found in the custom-style or bottom-up controls.
