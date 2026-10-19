# Serialization owner audit

This audit covers only `02_shell/src/cli.rs` and its exclusive owner
`00_nucleo/prompts/shell/cli.md`, cases V16-083–085. Exact state and time are in
`tests.json`; this phase changes L0 first, then co-located preservation tests.
No Cargo, seal repair, oracle binary, or productive implementation was run here.

## Evidence and classification

Baseline source SHA-256 is `2cc68094c6d39b89a93c9e08681c95156369c264c4eff0de7c1356e20b4753ec`;
L0 SHA-256 is `70f379446d3fc1ece1d0c8aab0cf8e95294a52bf4d53846875c8fa6492ca279a`.
Prompt header was `c74ad6bd`, reverse seal `a0874afb`. Baseline HEAD is
`5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`.
The complete owner and ADR-0107/0108/0127/0129/0131 were read before editing.

| Case | Measured branch | Preserved responsibility | Refuter |
|---|---|---|---|
| 083 | cli.rs:1156–1177 four early returns, then Label | Carrier transparency; Realized.visual; append Label after projected body | A current continue variant actually requires transparent projection |
| 084 | cli.rs:1182–1418 specialized projection, optional body fallback | Keep specialized fields; nominal body-probe and func-only classes | Getter or a promised field contradicts membership; investigate under its owner |
| 085 | cli.rs:1426–1445 overrides then elem_name | Domain-owned names, including operation-sensitive Transform and constructor-heading Styled | Promised public name differs from core name |

The body-probe class is Boxed, Block, Transform, MathStretch, MathLr,
MathUnderbrace, MathUnderbracket, MathUnderparen, MathUndershell, PdfArtifact,
and Dynamic. Content getters are at `entities/content/fields.rs:156–265,316–429`.
Delegates are `elements/transform.rs:101–105` (Matrix has no body),
`math_delimited.rs:104–111`, `math_stretch.rs:60–65`,
`math_underover.rs:217,251,285,319`, and `pdf_artifact.rs:102–107`.
Dynamic delegates `dyn_get_field` and must retain conditional presence.
MathPrimes and Linebreak expose other known fields but not body; the legacy
serializer remains bounded, not a global enumeration of known fields.
Many stored body-bearing variants (ListItem, Link, MathCancel, etc.) do not
expose body through the getter and remain func-only here. This is evidence of
existing bounded behavior, not certification of language-field completeness.
The pre-handled carriers and Label will have nominal separately described arms
in the field match; no unreachable panic is necessary.

Some snapshots bypass the whole legacy field match at cli.rs:1521–1539.
Some(empty) and Some(known) therefore remain authoritative. Preserve specialist
Raw, Empty, Entry, Caption, Cell, Mat, SmartQuote, typed MathAttach slots, Figure,
Heading, Equation and Shape cases. Keep the exact ShapeKind decision. Do not
introduce universal field enumeration or a shell-owned duplicate name catalogue.

The generic L0 Styled body bullet disagrees with current child/styles code;
primary current-pin evidence is delegated to the language agent before any
clarification. No undocumented implementation blessing is accepted.

## Validation plan

`tests.json` lists co-located preservation controls, expected to pass before
and after correction. These validate language morphology, optional field
presence, recursively structured content, label/order, conditional body,
canonical names, query filtering/cardinality/errors, and JSON/YAML projection.
The semantic vector key-order assertion checks the promised declaration order;
it does not equate rendered bytes. External YAML interpretation is delegated to
the language evidence report. Existing regressions listed there remain required.

`anchors.json` identifies exact removable positive arms for paired structural
experiments. Because changing Content in core blocks the shell dependency first,
use locked complete-workspace paired omissions, not a standalone duplicate enum.
Archive baseline and corrected source plus each actual mutation. Baseline
wildcards compile; corrected decisions must independently issue primary E0004.
This proves the local classification brake without pretending to have compiled a
cross-crate new-variant extension. Parent owns these executions and receipts.


## Constructor-origin extension and implementation

The parent added the same-cause constructor-origin correction after current-pin
external-Styled refuters. Read and pinned `math/equation-constructor-origin.toml`
SHA-256 `4e65f005a2f18d9eeb926b5077102ca9d72856446b151a15ea3d079c1560a53b`
in shell, producer and identity L0 before co-located tests. Parent-controlled
baseline captured the semantic REDs; implementation was explicitly authorized
only after these observations. The YAML label control initially asserted one
spelling; it now invokes the existing native YAML decoder on bytes through an
I/O-rejecting memory World and tests the exact parsed label, independently of
semantic key order. This correction changes no product expectation.

Producer adds only a private marker to the already nonempty explicit-option
Styles transport. Bare/block-only calls keep bare Equation. The identity and
shell helpers are private nominal exhaustive classifiers, with reverse exact
marker lookup and Bool(true), causal carriers, valid nested transport, and
opaque ordinary Styled. Identity uses this classification; shell emits only
known getter fields for marked Equation in existing declaration order, never
constructor metadata or fabricated defaults. Bare Equation's existing legacy
default expansion remains separately visible debt. Ordinary Styled retains
child/styles; the L0 summary was corrected only after primary upstream required
field declarations and bilateral JSON/YAML evidence were read.

`implementation.json` records the source hashes, current anchors, static complete
helper inventory and cross-review findings. No Cargo, sealing, or heavy binary
execution was performed by this agent. Parent owns those validation results.


The first YAML-decoder test correction exposed a test type mismatch: Value::Bytes
requires entities::bytes::Bytes, while World::file returns world_types::Bytes.
The parent corrected only that constructor in the productive workspace and
baseline copy and archived the failed attempt as label-baseline-type-error.
Earlier report hashes remain timestamped snapshots, not final-state hashes.
Final source formatting, seals, baseline confirmation and GREEN validation are
owned by the parent receipts.
