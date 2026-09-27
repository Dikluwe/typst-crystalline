# L0 — infra/shaper/bidi
Hash do Código: 4524c646

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/bidi.rs`

## Current evidence

- `03_infra/src/shaper/bidi.rs:11` — `BidiRun`.
- `03_infra/src/shaper/bidi.rs:30` — `bidi_runs`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/bidi.rs`.

## Owned contract

- Own BidiRun and bidi_runs only: logical paragraph order, visual run order, original substrings, direction and absolute byte_start.
- Use unicode-bidi with automatic base direction and its visual_runs for every paragraph. Retain empty paragraphs and omit trailing class-B separators from shaping ranges.
- Empty input yields no runs. UTF-8 clusters refer to the original string. Font selection, run shaping and position adjustment belong to other owners; no vertical-text policy is added.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
