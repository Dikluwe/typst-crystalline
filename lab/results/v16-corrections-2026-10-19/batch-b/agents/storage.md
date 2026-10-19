# Batch B: storage and bounded fixture observers

## Scope and checkpoint

Assigned findings: V16-025, V16-075, V16-076. Starting checkpoint:
`059d1a0ffa9523dbe21af6860a215acaeef55d73`, branch
`codex/tekt-mature-layout`. Source inspections and edits occurred on
2026-10-19; authoritative exact UTC/state, diffs, hashes, commands and execution
outcomes are in the coordinator's batch receipts. This note is not an independent
test execution receipt. The agent ran no Cargo processes.

The assigned source/owner pairs are:

- `01_core/src/compiler/eval/rules.rs` and
  `00_nucleo/prompts/core/compiler/eval/rules.md`.
- `01_core/src/entities/content/tests/traversal.rs` and
  `00_nucleo/prompts/core/entities/content/tests/traversal.md`.
- `01_core/src/entities/content/traversal.rs` and
  `00_nucleo/prompts/core/entities/content/traversal.md`.

The repository guide, core guide, ADR-0108 and ADR-0127 were read. L0 changes
preceded the coordinator's restricted reseal; tests were added only after that
checkpoint. Productive traversal, public signatures, nuclei and lint configuration
are outside this patch's scope.

## Decisions and refuters

### V16-025: custom-style observer

The observer follows Styled and Sequence. Styled replaces existing custom values
in place, so inner scopes and later encountered siblings override earlier values
without reordering keys. Other variants are opaque to this fixture projection.
The nominal complement must retain that behavior; its output cannot establish
global absence of custom styles in arbitrary Content.

An initial proposal to reject unsupported wrappers noisily was rejected during
coordination before Rust changes: it would change the bounded observer's behavior
without a demonstrated obligation. The final specification preserves opacity.
Controls independently assert nested precedence, encounter order, independent
keys, leaf preservation, and a Strong/Styled decoy remaining hidden while the
following supported Styled sibling remains visible. Existing evaluated-format
fixtures and all their assertions remain.

### V16-075: bottom-up no-replacement callback

The actual existing callback still replaces Text with uppercase content and
asserts that Strong receives the already-transformed child. Its fixture now adds
an Emph/Text branch, a Space terminal, a trailing Text sibling and an explicit
Sequence. Kind, text, ordering and Strong visitation assertions qualify that
None retains transformed descendants rather than skipping a subtree. The
complement must stay at the callback and explicitly name current variants.

### V16-076: storage qualification

Source inspection found 120 current Content alternatives, 34 qualified same-kind
Arc comparisons, and 86 other left-hand classes. These counts describe the cited
checkpoint's declarations, not a measured performance or language claim.
The helper is a selected representation predicate, not universal Content identity.
Same-kind Arc comparisons must remain unchanged; unlike kinds and unqualified
classes remain false. Both Content axes must be nominally closed.

Controls compare every fixture with itself and its clone, all distinct fixture
pairs in both directions, and unqualified Text/Sequence values with their clones
and with qualified fixtures. Sequence is a deliberate negative control even when
its Arc is shared. Existing no-op, metadata, slots, child order and error tests
remain.

Peer review caught a vacuous acceptance possibility in the added changed-parent
control: deriving expected storage reuse from an observed replacement count would
let a traversal that skipped every seed child pass. The corrected control fixes
MathPrimes and MathAlignPoint as the sole childless roster members. Those require
zero replacements; every other fixture requires positive replacements and
nonempty unchanged-Strong witnesses. Parent reuse is compared to that independent
roster classification, and sibling sharing is checked separately.

## Structural and integration obligations

Behavioral preservation controls run against the wildcard baseline before the
nominal repair. In a disposable actual workspace, Content growth must be absorbed
by the old decision and rejected with attributable E0004 at each repaired site.
The pair helper needs independent new/old, old/new and new/new witnesses; another
match's failure cannot substitute for its own. The coordinator owns these runs,
final full-suite validation, path checks, forward/reverse lineage and closure.

Read-only peer review of the infra observer found its Equation-terminal,
Sequence/MathSequence and Styled boundary preserved, with meaningful direct,
ordered and opaque-decoy controls. The diagnostic path inspector's initial
nonempty assertion did not prove the fixture retained any equations; this was
reported for removal into lab evidence or strengthening from actual measurement.
No infra files were edited by this agent.

## Submitted nominal repair

After coordinator authorization following its behavioral and structural baseline,
only the three selected wildcard decisions were replaced. The two single-Content
matches enumerate their 118 unchanged alternatives. The storage tuple keeps all
34 original Arc comparisons, then partitions unqualified left-hand variants from
qualified left-hand variants with every current right-hand variant explicitly
named in both groups. Later qualified pairs are necessarily unlike or have an
unqualified right-hand class because same-kind qualified pairs were handled first.
The existing return values and recursion remain unchanged. Owned files were
formatted with Rust 2021 style; scoped diff whitespace checks passed. A read-only
comparison confirmed code before the assigned test modules unchanged apart from
resealed lineage headers. Final closure remains the coordinator's responsibility.
