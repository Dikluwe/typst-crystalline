# L0 — lang/heading_supplement
Hash do Código: d1e0d1af


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:9dda215eac16294572a1775094e5ecf6dc4b1118d9b2e5300e3ce668dcf65494

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/heading_supplement.rs`

## Consumer evidence

`01_core/src/compiler/lang/heading_supplement.rs:61` locates `pub fn heading_supplement_for_lang(lang: Option<&Lang>) -> &'static str`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Contract and public boundary

`pub fn heading_supplement_for_lang(Option<&Lang>) -> &'static str` is the
single pure heading-supplement lookup, publicly reachable through
`typst_core::compiler::lang`. The module and table stay private. Preserve
the existing base-language translations, including French `Section`;
missing or uncovered language returns `Section`.

Lookup is exact on `Lang::as_str()`, without regional aliasing, locale,
runtime access to `lab/`, evaluation or layout. The translation repertoire
does not enlarge the language entity's representable domain.
Compiler callers reuse this function rather than keeping successor literals,
`is_pt` branches or duplicate tables. L2 may call this public helper;
it does not acquire ownership of the translation.

Target-language causality, explicit/empty supplement precedence and U+00A0
joining are integration contracts of their existing introspection/reference
owners, not algorithms implemented here. Acceptance preserves every existing
table entry and the missing/uncovered-language fallbacks. Removing the public
reexport or helper requires compatibility review.

## Preserved translation table

| Language | Supplement |
| --- | --- |
| ar | الفصل |
| bg | Раздел |
| ca | Secció |
| cs | Kapitola |
| cy | Adran |
| da | Afsnit |
| de | Abschnitt |
| el | Κεφάλαιο |
| en | Section |
| eo | Sekcio |
| es | Sección |
| et | Peatükk |
| eu | Atala |
| fi | Osio |
| fr | Section |
| ga | Ceannteideal |
| gl | Sección |
| he | חלק |
| hr | Odjeljak |
| hu | Fejezet |
| id | Bagian |
| is | Kafli |
| it | Sezione |
| ja | 節 |
| la | Caput |
| lt | Skyrius |
| lv | Sadaļa |
| nb | Kapittel |
| nl | Hoofdstuk |
| nn | Kapittel |
| no | Avsnitt |
| pl | Rozdział |
| pt | Seção |
| ro | Secțiunea |
| ru | Раздел |
| sk | Kapitola |
| sl | Poglavje |
| sq | Kapitull |
| sr | Поглавље |
| sv | Avsnitt |
| tl | Seksyon |
| tr | Bölüm |
| uk | Розділ |
| vi | Phần |
| zh | 小节 |
