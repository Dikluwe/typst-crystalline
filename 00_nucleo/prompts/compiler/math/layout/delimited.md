# L0 — math/layout/delimited

Hash do Código: 37b841ba

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/delimited.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/delimited.rs:21` locates `fn positional_lr_delimiters(body: &Content) -> Option<(char, Content, char)>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Fenced groups and lr/mid

Own mechanical MathDelimited composition plus public MathLr/MathMid layout.
For a balanced group, target height is
`2*max(body.ascent-axis,body.descent+axis)`.
The binomial's line=false fraction stack is unbalanced and targets body.height.
Stretch delimiters using their own axis-centered geometry and horizontally
concatenate opening/body/closing without moving the body's baseline or
axis-centering the complete group again.

MathLr retains public identity and body. Resolve size by explicit >
math.lr.size > 100%, with vertical shortfall 0.1em. For a group with fences,
measure the natural body and derive its relative target from the same
extent as an implicit fenced group: normally the axis-balanced
`2*max(body.ascent-axis, body.descent+axis)`, or the raw height for the
barless binomial exception above. Apply the effective percentage
or absolute length to that extent before stretch selection; an explicit
`lr` wrapper at 100% must not select a smaller delimiter variant than the
equivalent implicit group. Absolute length resolves against causal font size.
A body without fences remains visible; an item-only group preserves its
existing semantics. Nondefault nested sizes must not be flattened or baked
into a MathDelimited carrier. Only an implicit inner lr whose effective size
is 100% may be absorbed according to the approved rule.

Select first/last eligible fences by positional role, including inverted
pairs such as U+005D ... U+005B. Role changes do not replace original chars,
base_char, glyph variant, advance or order. Reuse the normal group's target.

MathMid classifies actual body items as Relation. Outside lr it keeps natural
geometry; inside, use the nearest lr's vertical target, including size and
shortfall, without crossing an inner lr boundary or independently selecting
a target. Preserve public body/identity.

Acceptance discriminates natural/balanced/binomial targets, same neighboring
baseline, 100%/150%/absolute/mixed/cascaded sizes, nested lr, mid scope and
normal/inverted fence variants. Current implementation breadth alone does
not demonstrate full language parity or retroactively prove a historical gate.
