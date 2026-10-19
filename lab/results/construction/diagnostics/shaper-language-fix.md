# Explicit language in shaping and cache identity

The correction applies explicit TextStyle.lang to rustybuzz buffers in both width measurement and glyph emission, and includes that optional normalized language in the document-local run-cache key. Absent language remains unset; explicit English and absence have separate keys. No public API, default mode, pipeline phase, source/font lifetime or global cache changes are introduced. This is an internal parity/cache correction under ADR-0127. The three L0 owners were updated before implementation and only their consumers were resealed, including the existing nucleus dependency in the shaping owner's effective hash.

The prior sealed diagnostic `infra-performance-followup/language-control.json` demonstrated ignored ja/zh/ko language through public Tekt shaping, with a configured same-face rustybuzz control. Pinned vanilla at `9f2b6e8715237cb086899a42873660fe744622e8` configures the language at `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`. Neither that algorithmic control nor this repair claims full upstream language parity.

Independent mandatory regressions use the already versioned NotoSans-Regular.ttf fixture. Its Cyrillic `б` has Russian glyph 457/advance 599 and Serbian glyph 2090/advance 604. At 12pt/1000 units per em, this gives widths 7.188pt versus 7.248pt. No external font dependency, missing-font skip or new fixture bytes are required.

The RED run failed both emission and width controls: explicit Serbian reused Russian substitution/advance. An intermediate buffer-only change passed width but failed the mixed Russian→Serbian→Russian run, proving that applying language without extending the cache key is insufficient. The complete correction passed all three new tests: localized widths, fresh/shared emission in AB/BA/repeated orders with original text/cluster preservation, and explicit-code/absence key separation. The final source passed the entire shaper suite (57 passed, zero failed). Build succeeded. Logs preserve all stages.

Exact state: working tree at HEAD `0fea906e176acf93ae6301e4cd17ace6aa4fe349`, with existing dirty/untracked changes preserved. `provenance.json` retains start timestamp, exact incoming status and diff-stat; `incoming.patch` retains the tracked patch. `source-before.json` / `source-after.json` capture actual prompt/consumer/benchmark files, including untracked sources. `before/` and `change.patch` preserve the six owner/consumer edits independently of tracked status. `commands.json` records commands, toolchain and log timestamps (timestamps are log completion metadata, not timing samples). `completion-audit.json` records final state and integrity checks.

Changed owner/consumer pairs:

- `00_nucleo/prompts/infra/shaper/shaping.md` ↔ `03_infra/src/shaper/shaping.rs`
- `00_nucleo/prompts/infra/shaper/cache.md` ↔ `03_infra/src/shaper/cache.rs`
- `00_nucleo/prompts/infra/shaper/tests.md` ↔ `03_infra/src/shaper/tests.rs`

Validation commands were `cargo test -p typst-infra --lib language_ -- --test-threads=1`, `cargo test -p typst-infra --lib shaper:: -- --test-threads=1`, `cargo build`, and `crystalline-lint . --format sarif`. Global lint exited successfully with zero error-level findings but existing warnings, including drift outside this scope. The owned consumers have no V5/V6/V15/V26 findings; their existing V16 neutral-match warning remains. This is not a zero-warning/global-clean claim. No global formatting or hash rewrite ran; only the newly appended test block was formatted.

Every inventoried consumer/prompt/benchmark outside these six files is byte-identical to the incoming state; the tracked patch outside scope also matches. The complete prior performance dataset manifest and report digest remain intact. Production performance was not measured again. Catalog acquisition, placement-triggered font demand, TTC reconstruction and unproved forced-math-script cache identity remain separate investigations. This correctness fix does not establish a speedup or solve catalog acquisition costs.
