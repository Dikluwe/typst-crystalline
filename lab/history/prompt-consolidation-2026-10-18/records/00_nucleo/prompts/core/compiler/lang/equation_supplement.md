# L0 — lang/equation_supplement
Hash do Código: c8df8566


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/equation_supplement.rs`

## Consumer evidence

`01_core/src/compiler/lang/equation_supplement.rs:18` locates `pub fn equation_supplement_for_lang(lang: Option<&Lang>) -> &'static str`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Contract

`equation_supplement_for_lang(Option<&Lang>) -> &'static str` performs exact,
case-sensitive lookup of `Lang::as_str()`. Preserve these generated strings:

| Language | Supplement |
| --- | --- |
| en | Equation |
| pt | Equação |
| de | Gleichung |
| fr | Équation |
| es | Ecuación |
| it | Equazione |

Missing or uncovered language returns `Equation`. The helper is pure,
deterministic and independent of process locale, evaluation and introspection.
Explicit supplement selection belongs to callers, not this table.
Acceptance checks the strings and both fallback paths, not the lookup mechanism.
