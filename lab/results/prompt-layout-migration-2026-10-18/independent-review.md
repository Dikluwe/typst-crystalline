# Independent prompt-layout migration review

Reviewed at UTC 2026-10-19T00:26:38.445204+00:00 against checkpoint `d330c717eb06c86a560c4cf3d8d0ba40224609b9`, on an uncommitted migration tree. The exact changed-file stat is `/tmp/tekt-final-review-working-tree-stat.txt`. Governance documentation and hash resealing were still being finalized concurrently; this review does not claim those later final bytes were tested.

## Verdict

The identified documented-command and active-title findings are now resolved. All 13 exact command corrections and 103 title-only replacements were independently verified in the final files. No unresolved migration blocker remains in this review's scope. The earlier canonical-path scan missed bare Cargo target tokens; that limitation and the subsequent correction remain recorded below. This structural review does not claim zero global lint debt.

## Evidence

- All 85 source-rename destinations and all manifested owner destinations exist. No old renamed source file remains. `python3 lab/tools/ci/check_mature_layout.py` passed.
- Independently resolved all 52 changed external module declarations using their actual containing module directory and explicit `#[path]` where present. All resolved to existing files. This covered the duplicate old basenames in eval, math layout, structural constructors, and HTML/SVG export independently, avoiding cross-theme substitutions.
- All 29 existing Rust `#[path]` attributes resolve. The 11 wiring → L3 support references still resolve; ten were renamed and the CLI support target remained unchanged.
- `cargo metadata --offline --locked --no-deps --format-version 1` discovered all 23 renamed top-level integration-test targets. No manifest target is missing because of autodiscovery or an old explicit target name.
- Compared current product Rust files with `before-rust.json.gz`: at inspection, 873 files differed, of which only 22 had changed noncomment lines. Those lines were solely external module declarations and `#[path]` values. Test assertions, input literals, function bodies, public signatures and cfg attributes were unchanged by this rename. `/tmp/tekt-rust-review-executable.json` preserves the inspected line classification, and `/tmp/tekt-rust-review.diff` the complete diff. Forward seals had not yet been refreshed at this point.
- All 109 nucleus files match `/tmp/tekt-migration-nuclei-before.json` byte-for-byte by SHA-256. There were no nucleus additions, removals or content changes.
- Compared every L0 with `before-prompts.json.gz`, normalizing full canonical source/owner paths and reverse-seal metadata. The only residual changes were 12 `Owned test module declarations` appendices and three corresponding local module/path references. These additions enumerate separately owned test modules, preserve each test module's independent owner, and do not absorb test-body ownership. No requirement body was silently merged, deleted or assigned to a different theme. The residual comparison is `/tmp/tekt-prompt-noncanonical-diff.txt`.
- Scanned active L0, ADRs, current diagnostics, L1–L4, root files, `.cargo`, `.github`, `lab/tools`, and `lab/benches` for manifested old canonical paths. Only two matches remained: the deliberately synthetic `p1286_contract.rs` fixture in `lab/tools/ci/test_mature_layout.py`, testing the configured exclusion rule. These are fixture identities, not operational references to the removed source. Historical receipts were intentionally excluded.
- The renamed black-box wiring contract remains explicitly excluded from productive owner requirements in `crystalline.toml`; the exemption was preserved rather than converted into an unowned productive consumer.

## Scope and remaining work

The directory and file identities are improved; this does not consolidate every old prose clause. In particular, `00_nucleo/prompts/core/compiler/layout/set_page.md` still contains the preexisting multi-file P537b construction-era description previously identified by the audit. It was path-migrated without implying that this ownership-scope debt was resolved. Retain this item in the follow-up diagnostic.

Numbered inline test module names and construction references inside bodies remain separate content cleanup. Their continued presence does not invalidate the renamed external file mapping. Historical fixtures and evidence identities were intentionally preserved.

Parent final validation still must reseal, run strict/reverse lineage, compare full lint against the existing V16 baseline, and run build/tests/quality checks on the finalized inputs. Review results above do not substitute for those gates.

## Follow-up: documented Cargo selectors and active titles

Audited at UTC 2026-10-19T00:29:05.394273+00:00, while the parent validation input freeze was active. No active source or L0 was edited. The exact correction map is `/tmp/tekt-test-target-fixes.json`.

All 13 active documented `cargo test ... --test ...` commands found in the scanned source/documentation/tool roots belong to `00_nucleo/prompts/wiring/tests/`. Cargo metadata confirms each current target belongs to `typst-wiring`. Eleven targets still have the removed numbered names; eleven package selectors incorrectly name `typst` (nine overlap the renamed targets, plus `cli` and `crystalline_lint`). The two existing `typst-wiring` selectors are correct. The map provides exact old/new command strings and target source identities. No additional unresolved documented `--test` command was found in this scan.

The same map contains 103 first-line title replacements for owners whose own basenames changed and whose title still uses an old filename/module identity or numbered construction identity. Proposed replacements use `# L0 — <layer/module identity>`. They replace the first line only; historical origins, approvals, requirements, and body text are preserved. Three unrelated historical step mentions in owners with unchanged basenames were deliberately excluded.

These title/command corrections need L0 resealing afterward. Their changes should not alter test bodies; prove the resulting Rust differences are only forward seals before reusing the frozen functional-suite result.

## Final closeout — resolved

At UTC 2026-10-19T00:49:52.524233+00:00, independently verified every exact replacement in `document-fixes-proposal.json`: all 13 documented commands use their correct current targets and `typst-wiring`; all 103 first-line titles match the proposed semantic identities. Historical body text and origins remain intact.

The full functional/quality suite ran before these final documentation corrections. `validation-summary.json`, `test-summary.json`, and the `validation/` receipts report the frozen run: 9,128 unit/integration tests and five doctests passed, seven ignored, plus 34 Python tests and the build/quality gates. This reviewer did not rerun that suite.

Independently repeated the exact source comparison against `tested-rust-before-document-fixes.json.gz`: all 992 Rust files match except for 105 single leading `@prompt-hash` comment lines; every other byte is unchanged. This confirms `post-test-rust-proof.json` and justifies retaining the frozen behavioral result after resealing. It does not imply the finalized L0 text was part of the earlier functional-test input.

The parent's final isolated snapshot was checked after these corrections; `fresh-checkout.json` records successful layout, reverse-lineage, strict V1/V5/V15/V26 and Cargo-metadata checks. Known broader limits remain: preexisting V16 lint debt and the separately documented prose-consolidation items, including the set-page owner.
