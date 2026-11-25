# L0 — eval/rules/set/schema
Hash do Código: 0be2b72b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/set/declared-properties.toml sha256:b4b40ae19e8c22e62a22af00459e257fa4a6c9d8c518d101cd0798cbf4f97744

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/set/schema.rs`
**Governance:** ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Own the closed table of settable properties per element and the types each admits,
in the order and wording of the language's own diagnostic, and the pure check of
evaluated arguments against it. Element keys come from native identity, never from
source text; distinct elements with the same public name have distinct keys.

A positional argument fails with `unexpected argument` at its span, except for the
elements whose language schema admits one. An unlisted name fails with
`unexpected argument: NAME` at the argument span. A value outside the listed types
fails with `expected TYPES, found TYPE` at the value span; `, found TYPE` is omitted
for a string when the list contains string literals. String and Symbol satisfy
string; content admits content, string, symbol and none; relative length admits
length, ratio and relative; function admits callable types. An integer where a length is
listed adds the hint `a length needs a unit - did you mean Npt?`. A property with no listed
types has only its name checked. The check decides no effect, performs no conversion
and changes no style. Crystalline extension names are listed explicitly.

The table belongs to the pinned reference. An element absent from it is not checked.

## Acceptance

Each element's names and types are compared with the pinned reference by probe:
no listed name is refused there, no unlisted name is accepted there. Unit controls
cover each type group, literals, positional arguments, duplicate public names and spans.

## Evidence

Approved on 2026-11-24 (“Aprovo A–F”): `00_nucleo/diagnostics/set-contract-package-2026-11-24.pt-BR.md`.
The table was extracted from the diagnostics of the pinned reference with an
inadmissible value for every parameter.
