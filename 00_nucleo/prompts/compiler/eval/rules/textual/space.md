# L0 — compiler/eval/rules/textual/space
Hash do Código: e5f055bb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/space.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:335-463` contains space origin, neighbor visibility, wrapper text, normalization and Unicode classification. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own `normalize_markup_spaces` and its helpers in this descendant module. Before literal/regex show on realized flow/root, collapse adjacent spaces to one occurrence whose newline origin is the OR of its components. Discard only an inferred newline occurrence when adjacent internal textual characters satisfy the pure pinned Unicode nonspacing classification (Han/Bopomofo/Hiragana/Katakana/Yi and measured ideographic punctuation, not Korean or ambiguous curly quotes). Use immutable data and the authorized icu_properties dependency. Keep explicit space between supporting texts when no component has newline. Explicit Linebreak is destructive and consumes adjacent spaces, including explicit ones, but remains Linebreak. Parbreak and actual nontext content remain causal boundaries. A textless style wrapper does not hide adjacent characters for normalization. Do not use global trim/split_whitespace, document language or target-specific space heuristics.

The coordinator calls this algorithm only when its existing `normalize_spaces` flag is true. Preserve indices by replacing consumed slots with Empty. The Unicode control and styled-space tests are owned here. No phase, public signature or default changes.

## Acceptance

The existing Unicode/space tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.
