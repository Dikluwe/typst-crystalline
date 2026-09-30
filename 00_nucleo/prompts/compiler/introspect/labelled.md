# L0 — introspect/labelled
Hash do Código: c80dfa03

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/labelled.rs`.
The pure compute_labelled body at `:24` reads typed, location-aware counters.
The auto Label branch at `01_core/src/compiler/introspect.rs:2300` passes
the causal wrapper chain's language, with the explicit language argument only
as fallback. Real occurrence Start/End and generic snapshot handling remain parent-owned.
The Label descendant owns its complete walk branch and derived label handling.
Heading language text belongs to the canonical helper at
`00_nucleo/prompts/compiler/lang/heading_supplement.md`.

## Walk-branch refinement measurement

At the uncommitted state captured in
`00_nucleo/diagnosticos/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2280-2350` owns the Label walk branch,
while this consumer already owns location-aware resolution. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Contract

Preserve the pure descendant free function over introspector, target Location,
Content and optional language, returning (optional resolved text, optional
figure number). Do not mutate stores or allocate identities.

For Heading, read its element-function hierarchical counter at Location and
join canonical localized supplement, a space and the number when available.
For a block Equation, read its element-function flat counter; a positive number
produces the existing compatibility text `Equação (N)`, otherwise no text.
For Figure, read its per-kind counter (default image); a positive number
produces localized figure supplement plus number and returns that figure number.
Otherwise return present empty text and no figure number. Other targets return
neither. Do not silently unwrap arbitrary Styled targets or infer a missing
numbering gate from the body's render.

This helper preserves compatibility resolved text; it is not the authoritative
numeric-reference path and must not override its precedence. It does not replace
the equation runtime supplement owner with its compatibility literal.
The Label walk branch reuses target Location, captures causal language and
publishes the derived association only when applicable.

## Derived-label walk

The Label descendant walks its target first with the causal label. For an auto
label it finds the first newly emitted real Start, resolves Heading/Equation/
Figure text with the wrapper chain language before the explicit fallback, and
publishes a derived Labelled Start/zero-hash End at that same Location only when
resolution is available. If the target has no real Start, classify it as
unreferencable. For an explicit user label, walk the body once with the label,
compare known associations before/after and classify unsupported targets when
no association appears. Preserve numeric mapping precedence; do not remove a
counter key to force legacy text. The parent match remains exhaustive and
calls this descendant statically, without a second traversal or new Location.

## Acceptance and phase boundary

Check hierarchical/flat counters at the supplied target Location, missing
values, captioned/numbered Figure paths and unsupported targets without invented
associations. Check mixed-language and forward/backward heading references
against the causal store, including an explicit synthetic language fallback.
A language present in the wrapper chain always wins over that fallback;
reference/layout/environment language is not a substitute.

A show-set that changes language only in the rendered tree is not a pre-show
introspection correction. Its parity is unresolved at this boundary and any
phase change requires its own L0 and ADR-0127 approval. Preserve current
fallbacks without promising a completed language-parity result for them.
