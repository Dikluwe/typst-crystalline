# L0 — lang/outline_title
Hash do Código: f15a9ef3


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/outline_title.rs`

## Consumer evidence

`01_core/src/compiler/lang/outline_title.rs:48` locates `pub fn outline_title_for_lang(lang: Option<&Lang>) -> &'static str`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Contract

`outline_title_for_lang(Option<&Lang>) -> &'static str` uses exact language
lookup with the following generated titles:

| Language | Title |
| --- | --- |
| en | Contents |
| pt | Sumário |
| de | Inhaltsverzeichnis |
| fr | Table des matières |
| es | Índice |
| it | Indice |
| zh | 目录 |

Missing or uncovered language returns `Contents`. The helper is pure and
does not consult process locale. Explicit title selection and outline layout
remain with their respective owners. Acceptance preserves titles, accents,
spaces and fallback, not a particular table representation.
