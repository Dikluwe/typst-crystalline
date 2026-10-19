# Prompt layout migration

The repository now locates L0 owners by layer and consumer module path, without
leading `src/`, and names active Rust test files by responsibility. ADR-0131
records the user-authorized convention. The change is on
`codex/tekt-mature-layout`; checkpoint `d330c717eb06c86a560c4cf3d8d0ba40224609b9`
is preserved by tag `tekt-before-prompt-layout-2026-10-18`.

## Organization and preservation

The [manifest](../../lab/results/prompt-layout-migration-2026-10-18/migration-map.json)
records 85 Rust file renames and 868 L0 path moves. All 991 owner/consumer pairs
remain separate. Source `mod.rs`, `lib.rs`, `main.rs`, `build.rs` and `tests/`
retain their identities; only the leading `src/` is omitted from the owner path.
The 109 shared nuclei retain their paths and exact bytes.

The before-state, audit inventories, complete pre-migration Rust/L0 text, rename
map, independent review and validation receipts are preserved under
[the migration evidence directory](../../lab/results/prompt-layout-migration-2026-10-18/).
Historical lab records were not rewritten. Existing fixture identities and
numbered inline tests remain traceable to their construction history.

The explicit black-box exclusion formerly associated with the numbered wiring
contract now follows `04_wiring/tests/smart_quotes_pdf_attachment_artifact.rs`.
No owner was invented for that excluded test.

## Validation and boundaries

The independent structural review resolved all 52 renamed external module
declarations, all 29 `#[path]` attributes, and all 23 renamed Cargo integration
targets. Executable changes are module declarations and path attributes;
assertions, function bodies and public signatures are preserved. The helper
suite adds checks for owner paths derived from layer and module names.

The [frozen validation run](../../lab/results/prompt-layout-migration-2026-10-18/validation/validation-commands.json)
passed all twelve jobs: 34 Python tests, layout, strict lineage, reverse seals,
workspace build, domain/infra/shell tests, wiring tests, doctests, all-target
build, Clippy with warnings denied, formatting, and Rustdoc with warnings
denied. There were 9,128 passing unit/integration tests and five passing doctests,
seven ignored tests, and no failures. Its source freeze records no changed inputs.
This was local Linux validation, not a remote CI, Windows/i686 or Docker run.

A follow-up audit found thirteen documented Cargo commands with obsolete target
or package names. Those commands and 103 owner titles were corrected after the
frozen run, then resealed. The
[source comparison](../../lab/results/prompt-layout-migration-2026-10-18/post-test-rust-proof.json)
proves exact equality of every Rust file with the tested source after excluding
only the leading forward-seal comment: 105 seal comments changed and no other
Rust bytes changed. The
[document verification](../../lab/results/prompt-layout-migration-2026-10-18/document-fixes-verification.json)
checks all corrected commands against Cargo metadata and reads back every title.
These documentary corrections did not require repeating unchanged behavior tests.

The [isolated snapshot](../../lab/results/prompt-layout-migration-2026-10-18/fresh-checkout.json)
also passed layout, strict lineage, reverse seals for 991 consumers, and Cargo
metadata resolution. It was assembled from versioned Git objects, without
copying ignored local files. The receipt identifies the exact staged tree.

The [full lint comparison](../../lab/results/prompt-layout-migration-2026-10-18/lint-comparison.json)
retains exactly the prior 105 V16 errors and unchanged informational findings.
There are no new full-lint findings. Full architecture CI is still blocked by
those existing errors; a successful layout/lineage gate is not global lint success.

The owner at `00_nucleo/prompts/core/compiler/layout/set_page.md` retains known
construction-era multi-file prose. Consolidating those clauses is a separate
semantic audit, as is completing English translation of the old corpus. This
path migration does not imply that those debts are resolved. Numbered inline
module/function names and provenance asset names were preserved deliberately.
