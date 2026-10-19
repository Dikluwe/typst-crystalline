# L0 — lang/figure_supplement
Hash do Código: d6b56900


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:9dda215eac16294572a1775094e5ecf6dc4b1118d9b2e5300e3ce668dcf65494

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/figure_supplement.rs`

## Consumer evidence

`01_core/src/compiler/lang/figure_supplement.rs:84` locates `pub fn figure_supplement_for_lang(kind: &str, lang: Option<&Lang>) -> String`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Contract

`figure_supplement_for_lang(kind, Option<&Lang>) -> String` first matches
the exact kind/language pair. Preserve the generated strings:

| Language | image | table | raw |
| --- | --- | --- | --- |
| pt | Figura | Tabela | Listagem |
| en | Figure | Table | Listing |
| de | Abbildung | Tabelle | Listing |
| fr | Figure | Tableau | Listing |
| es | Figura | Tabla | Listado |
| it | Figura | Tabella | Listato |

Missing or uncovered language uses `Figure`, `Table` or `Listing` for
these known kinds. For an unknown kind, uppercase its first Unicode scalar and
retain the remaining UTF-8 suffix; an empty kind returns an empty string.
Do not consult the environment or reinterpret a language code as a region.
The helper owns generated text only, not the caller's explicit supplement policy.
