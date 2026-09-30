# L0 — Embedded font sets
Hash do Código: 24bd12e2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/embedded_fonts.rs`

## Current evidence

- `03_infra/src/embedded_fonts.rs:31` — `EmbeddedFontSets`.
- `03_infra/src/embedded_fonts.rs:66` — `embedded_font_group`.
- `03_infra/src/embedded_fonts.rs:85` — `load_embedded_fonts`.
- `Cargo.lock` — typst-assets revision `c0ae970f64413cf8e39cecd56d15ee50de42eaf4`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own loading and classification of the compiled `typst_assets::fonts()` bytes. `EmbeddedFontSets` contains `text_slots`, `text_book`, `math_code_slots` and `math_code_book`. Extract metadata through the fonts owner, create embedded slots with display path `<embedded>`, and insert valid metadata/slots together. Loading does not read the filesystem.

Classify the normalized metadata family name, not the raw filename or uncorrected nameID 1. Case-insensitive names starting with `Libertinus Serif` or exactly `New Computer Modern` belong to text. Exactly `New Computer Modern Math`, names starting with `DejaVu Sans Mono`, and unrecognized names belong to math/code. The metadata exception table belongs to `00_nucleo/prompts/infra/fonts.md`; preserve its documented family names with spaces.

The embedded asset set makes Libertinus Serif and the bundled math/code faces available independently of host installation. `00_nucleo/prompts/infra/system-world/fonts.md` exclusively owns their insertion relative to system/project fonts and the existing builders. The language's default text family belongs to L1 style owners; neither those defaults nor CLI composition are redefined here.

## Verification boundary

Verify embedded slots load without disk access, Libertinus Serif and New Computer Modern occur in text, and New Computer Modern Math/DejaVu Sans Mono occur in math/code. Positive membership controls must accompany exclusion checks; an absent family must not make an exclusion test pass vacuously.
