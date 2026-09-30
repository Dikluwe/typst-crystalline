# L0 — infra/font_metrics/resolution
Hash do Código: 4b7254ca

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/resolution.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
```

## Current evidence

- `03_infra/src/font_metrics/resolution.rs:17` — `FontCandidate`.
- `03_infra/src/font_metrics/resolution.rs:27` — `resolve_primary`.
- `03_infra/src/font_metrics/resolution.rs:86` — `resolve_primary_with_math_fallback`.
- `03_infra/src/font_metrics/resolution.rs:118` — `covering`.
- `03_infra/src/font_metrics/resolution.rs:193` — `resolve_font_combo`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/resolution.rs`.

## Owned contract

- Own FontCandidate and primary/MATH/covering/FontKey resolution. Preserve declared-family order, class fallback when primary lookup fails and first-resolved-family similarity reference.
- Append explicit math fallback faces when style.math is true, with slot deduplication; preserve the existing readable-face check without adding a MATH-table filter at insertion. Covering lookup first tries MATH-covering candidates, then primary coverage; preserve None and ordering.
- Global fallback uses World::candidates_for_char and FontBook::select_fallback. Coverage is authoritative for global filtering; load the winning eligible face only, not intervening slots or a second full catalog scan.
- Resolved identity includes family, variant and variations. Do not infer math semantics from family spelling; consume the pinned shared fallback-selection nucleus.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Cadeia comum alinhada com shaping — 2026-09-29

Medição: `03_infra/src/font_metrics/resolution.rs:27-65` usa só a primária quando ela resolve, enquanto `text::families()` do pin vanilla (`lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101`) acrescenta a cadeia comum antes da busca global. A mesma sonda hebraica do diagnóstico `correcao-rtl-restante-2026-09-29/after-leading` mostra face diferente na emissão. Inferência: métricas separadas também podem divergir se não consultarem a cadeia comum; refutar por face medida já idêntica sem alteração.

Acrescentar a cadeia comum ordenada e deduplicada depois das famílias declaradas legíveis no modo textual; em math, manter a cadeia matemática antes da comum. A medição de cobertura e a emissão não devem escolher famílias distintas para o mesmo caractere por causa da ordem dos catálogos.
