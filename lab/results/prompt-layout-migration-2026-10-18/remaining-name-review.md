# Remaining semantic Rust names — read-only review

Base: `d330c717eb06c86a560c4cf3d8d0ba40224609b9`; input repository was clean. Source and L0 SHA-256 are recorded per proposed move in `tekt-remaining-rust-renames.json`. This audit read source headers, declarations/test cases and exclusive-owner clauses. It proposes filenames only, not changes to behavior or assertions.

The 48 remaining numbered Rust files, plus the prior 37 content-reviewed candidates, account for all 85 numbered Rust filenames found under L1–L4. Combined destination paths have zero duplicates and none exists in the current tree. Prompt destinations reflect layer plus source path with `src` omitted. Forty-seven of these remaining files have an L0 owner; the existing `p1286_contract` exemption is the forty-eighth.

## Decisions that require more than prefix removal

- Fraction integration is split intentionally: `math_fraction_public_controls` retains routes/signatures/MATH extraction/binom isolation; `math_fraction_representation_provenance` retains decoded repr and evaluated carrier bits. Do not merge them.
- `math_cancel_callback_runtime` is specifically cancel-angle callbacks, not all callbacks.
- `math_decoration_and_float_flush` covers the existing cancel/underline/vec/flush suite. `language_and_export_contract` covers the existing float/math/HTML/grid/table suite. These are broad because the actual units are broad. A future focused-unit split requires separate ownership review, not cosmetic renaming alone.
- `format_pdf_namespace` concerns callable namespace, members and gates; it is distinct from tagged/pretty/pages/standard document export suites.
- `pdfa_core_metadata_tests` avoids claiming complete PDF/A conformance for metadata/trailer/output-intent checks.
- Keep paired support and wiring basenames aligned. Their L3 helper operations and differing cleanup policies stay separate.
- Rename `p1286_contract.rs` to `smart_quotes_pdf_attachment_artifact.rs` and update the existing `crystalline.toml` excluded-file key/path. It has no owning prompt. This must remain an explicit existing exemption, not a new silent exemption or invented owner.

## Names and strings outside this move

Existing numbered test-function names, fixture-case IDs, temporary-directory prefixes, `UPDATE_P307B_SNAPSHOTS`, historical report paths and exact diagnostics should remain byte-identical unless separately reviewed. Global replacement of `pNNNN` tokens would corrupt historical identity and alter test filters or control strings. Replace actual Rust module declarations/paths and active command targets explicitly.

## Fixture assets

`03_infra/fixtures/p307b/` is an active controlled PDF snapshot corpus, not an immutable construction record. A semantic directory such as `pdf-snapshots/` would be useful in a separate explicitly inventoried asset move, with every source/reference byte hash unchanged and all `include_bytes`, runtime paths, benchmark references and the owner updated. Its numbered input stems already combine stable corpus order with meaning (`01-markup-plain`, `05-gradient-linear`); keep those paired identities. Its MANIFEST contains historical regeneration observations and should not be rewritten wholesale.

Six synthetic font fixtures retain numbered prefixes. Source `03_infra/src/fonts.rs:1102–1112,1243` documents their actual purpose. Potential semantic filenames are `missing-name-records.ttf`, `family-style-suffix-bold.ttf`, `typographic-family-preferred.ttf`, `mac-roman-family.ttf`, `full-name-oblique.ttf`, and `fandolhei-bold-weight-exception.ttf`. Their font-internal family records and binaries must remain unchanged. For this Rust/L0 organization, preserve their current identities and report them as a separate asset cleanup rather than pretending all numbered names have disappeared.

## Required integration verification

Update parent module declarations and path attributes, cross-suite module references, Cargo explicit targets if present, CI command filters and active L0 references. Confirm a one-to-one reverse map, no missing static fixtures, no ownership changes, exact test bodies modulo imports/module names, and unchanged binary fixtures. Re-run lineage and the existing functional suite on final renamed sources.
