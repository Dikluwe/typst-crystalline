# L0 — infra/shaper/font_selection
Hash do Código: 4938047f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/font_selection.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
```

## Current evidence

- `03_infra/src/shaper/font_selection.rs:16` — `primary_candidates`.
- `03_infra/src/shaper/font_selection.rs:116` — `CandidateSet`.
- `03_infra/src/shaper/font_selection.rs:326` — `intern_fallback_slot`.
- `03_infra/src/shaper/font_selection.rs:363` — `split_run_by_font`.
- `03_infra/src/shaper/font_selection.rs:439` — `is_p1404_format_control`.

These are local source anchors, not a new upstream parity measurement.
`00_nucleo/diagnosticos/shaper-prompt-audit/results.json` demonstrates sparse scoring against the wrong slots and a same-face Latin/Greek run merged across its script boundary.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/font_selection.rs`.

## Owned contract

- Own FontList resolution, primary/class/MATH fallback, lazy coverage, similarity preference, dense local interning and script/font segmentation. Measurement and emission use the same primary chain.
- Try declared families with the actual TextStyle variant; accept only readable faces. If none resolve, consume the shared class fallback catalog before global fallback. Catalog spelling/order belongs to `00_nucleo/prompts/infra/fallback_fonts.md`.
- In the existing resolved-declared-list branch, append the math chain when style.math or a primary face has MATH; do not invent a second append branch. Family names do not define math content semantics.
- Global candidates come from World::candidates_for_char, are lazily loaded and confirmed by glyph lookup. Preserve longest-covering-run preference, with FontBook similarity using the first resolved primary to break existing preference ties. Never load uncovered catalog entries or intervening slots.
- The local fallback vector is a sparse interner: one dense local position per distinct global slot; result indices use primary.len()+local position. Resolve through that candidate's associated slot, never treat the local index as a global FontBook index or allocate proportional to the greatest slot. Translate candidates to their associated global slots before FontBook scoring, then translate the selected global slot back to its local candidate position without loading intervening faces.
- U+2060/U+202B/U+202C formatting controls stay in content and bypass coverage lookup, joining the current sub-run or first primary when isolated. Preserve their extractable codepoints and exact recognized set; other invisible characters are not generalized.
- Common/Inherited scripts inherit the active script, initially the first non-generic script in the bidi run when one exists. Update the active script when transitioning; a script or font change splits the sub-run even if both scripts use the same face. Preserve original cluster offsets and existing missing-glyph fallback, without a new color-font/render policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Fallback comum após famílias declaradas — 2026-09-29

Medição: `lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101` inclui a cadeia comum depois da família declarada; `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:904-948` tenta as famílias antes do fallback global. Em `correcao-rtl-restante-2026-09-29/after-leading`, os seis probes hebraicos mantêm 7,68 pt de divergência; `pdffonts` identifica Libertinus Serif no vanilla e Noto Sans Hebrew no cristalino. O primeiro cobre o texto hebraico e o segundo aparece pelo fallback global. A hipótese de causa é a omissão da cadeia comum no ramo em que a primária resolve; refutação: fonte ou posição ainda diferente com a cadeia presente. Isto é paridade tipográfica observável, não exigência de bytes iguais.

Após as famílias declaradas legíveis, oferecer a cadeia comum na ordem do catálogo, sem duplicar slots. A primária declarada mantém prioridade. A cadeia matemática continua a preceder a comum em contexto matemático; a recuperação por classe quando nenhuma declarada resolve conserva sua política. A busca global por cobertura e similaridade vem depois das cadeias explícitas. Medição e emissão compartilham essa ordem.
