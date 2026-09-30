# RED regression controls for Place, empty page and font demand

The five new integration tests in `03_infra/src/integration_tests.rs` compile and fail at their intended assertion on the current working tree. They are active, not ignored. No productive code, benchmark, ADR or other L0 was changed. `00_nucleo/prompts/infra/integration_tests.md` was updated before test code, then only this owner/consumer pair was resealed. Both had existing unrelated working-tree changes, which remain intact.

| Control | Current RED observation | Required observable |
| --- | --- | --- |
| Empty page | PDF lacks requested MediaBox | 160 × 180pt page remains even with no material items |
| Placed cover image | Translated FrameItem has no clip | 80 × 40pt clipping window at (11, 13) |
| Placed text | Real-font frame baseline is 13pt | Pinned vanilla witness is 20.238pt (±0.1pt) |
| Placed rectangle | Isolated rectangle has no book demand, placed one has four accessor calls | Neither no-text case requests the book |
| Placed image | Isolated image has no book demand, placed one has four accessor calls | Neither no-text case requests the book |

The media-box matcher accepts equivalent decimal serializations; it does not require complete PDF byte equality. The cover-image fixture is generated within an isolated temporary directory and has square intrinsic dimensions so cover must clip at the requested 2:1 target. The text baseline control uses real SystemWorld fonts with explicit Libertinus Serif and the pinned vanilla geometric witness from `place-font-acquisition-study/output-controls.json`. World::book calls are counted by a test-only decorator; counts are not catalog scans or timings. These controls do not suppress explicit font validation or real text's need for fonts.

Command: `cargo test -p typst-infra --lib place_regression_ -- --test-threads=1`. Compilation succeeded; five tests ran and failed. `red.stdout` and `red.stderr` contain the exact failure output and compiler warnings. The suite was filtered to these new cases; other tests were not rerun. `crystalline-lint . --format sarif` exited successfully; the owned files have no V5/V6/V15/V26 findings. `git diff --check` on the scoped files passed. Only the appended Rust test block was formatted via a temporary snippet; no global formatting ran.

This RED state intentionally prevents a green full suite until the productive owners are corrected. The page-size, image-clip and text-placement differences also mean corresponding performance inputs do not yet have equivalent output. The tests define acceptance but do not claim a production repair or latency gain.
