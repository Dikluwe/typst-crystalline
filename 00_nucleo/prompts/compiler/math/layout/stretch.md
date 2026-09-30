# L0 — math/layout/stretch

Hash do Código: 67416594

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/stretch.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/stretch.rs:15` locates `fn leaf_char(content: &Content) -> Option<char>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Public stretch

Own the free function for MathStretchElem, in the render layer rather than
the entity. Preserve body identity and size_explicit. Resolve effective size
by explicit > math.stretch.size > 100%, retaining zero and mixed length as
values rather than absence.

Measure natural body, select its native extensible axis and apply size with
zero shortfall. Percentage references the natural maximum dimension including
attachments; absolute length uses causal font size. Delegate variant/assembly
selection to `00_nucleo/prompts/compiler/math/layout/stretchy.md`;
do not replicate MATH tables or continuously scale glyph ink.
Preserve body classes/boundaries. A nonextensible body retains natural layout
and the applicable already-ratified diagnostic; do not invent a new error.

Acceptance covers both axes, attachments, default/absolute/mixed/cascaded
sizes, nesting and fallback. Current leaf-only eligibility and silent
fallbacks do not narrow this approved contract; unresolved implementation/
diagnostic coverage is recorded in the diagnostic.
