# L0 — math/layout/underover

Hash do Código: db9ba9a5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underover.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/underover.rs:20` locates `pub(super) fn layout_overbrace(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Spreaders and optional annotations

Own the geometry of MathUnderover and the public over/under brace, bracket,
paren and shell entrypoints. Public identity/presence is preserved by the
core; local lowering does not replace the language carrier.

Glyph pairs (above/below) are brace U+23DE/U+23DF, bracket U+23B4/U+23B5,
paren U+23DC/U+23DD and shell U+23E0/U+23E1. Stretch the support to the body
width at 100% with zero shortfall; an annotation is not the stretch target.
Support glyphs use full-size accent geometry. Optional annotations retain
their absence/empty-present distinction, use superscript/subscript style,
and may widen the final frame without redefining the support's target.
Under annotations are cramped; over annotations inherit the appropriate
superscript style.

For an above support:
`support_y=-base.ascent+min(base.ascent,accent_base_height)`.
For below:
`support_y=base.descent+max(0,support.ink_up)`.
Use signed actual ink extents; do not substitute overbar/underbar constants
or global font ascent. Union support/base ink for compound extents.

Exact-frame-width placement uses attachments:
`pre=support_attach-base_attach`;
`post=(support.width-support_attach)-(base.width-base_attach)`;
`width=max(pre,0)+base.width+max(post,0)`.
If pre<0, base_x=0 and support_x=-pre; otherwise base_x=pre, support_x=0.
Atomic glyphs use applicable TopAccentAttachment; composite bases use
half-width. A support with one glyph uses its attachment; multi-glyph
assembly uses half the assembled width. Preserve the existing symmetric
single-glyph asymmetry compensation
`2*abs(support_attach-support.width/2)` at the support level only, not
again in an annotation wrapper.

Annotations use limit shifts, not tight ink stacking:
under baseline = `base.descent+max(lower_drop_min,
lower_gap_min+annotation.ascent)`;
over baseline = `-(base.ascent+max(upper_rise_min,
upper_gap_min+annotation.descent))`.
The support and annotation paths remain distinguishable; historical generic
one-character dispatch does not redefine an explicit public annotation.

Acceptance covers selected support variants/assemblies, signed ink gaps,
nested supports, exact width/attachments, optional annotations and font-derived
limit placement without hardcoded positions or serialized-byte equality.

## Identidade dos suportes elásticos

Um suporte de um caractere retém esse caractere como identidade extraível
única em `TextReplacement` envolvendo seus itens visuais, inclusive quando
uma montagem horizontal usa várias peças. `.typ/sec_10.typ` mostra que
extrair as peças de under/overbracket cria caracteres que não existem no
conteúdo autoral. A substituição não altera geometria ou anotação.

## Atomização interna do owner

A seleção de suporte versus anotação, o estilo da anotação, a leitura de
attachments e o envelope textual do suporte devem ser operações privadas
distintas dentro de `underover.rs`. A composição final conserva as fórmulas
geométricas acima e aplica a identidade textual apenas aos itens visuais da
peça. Nenhum novo consumer, API ou modo de produto decorre desta separação.
