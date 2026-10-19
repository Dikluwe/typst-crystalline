# Infrastructure and wiring L0 sanitation

## Result and scope

The six requested families now contain English normative contracts, thin facade
parents, canonical references and explicit one-to-one consumer ownership.
The batch covers 61 prompts/consumers, including separately owned wiring
integration suites. No productive body was edited and no formatting command ran.
Only `//! @prompt-hash` source metadata was resealed; `Hash do Código` was updated
where the existing code seal required it.

Historical descriptions, line totals, temporary working-tree inventories,
RED/GREEN reports, gate narratives, retired coordinates and obsolete dependency
claims are preserved under `00_nucleo/diagnosticos/l0-infra-wiring-history/`.
Each archive retains the original prompt bytes below its archival preamble.
The archive is evidence only, not a materializable prompt or gate approval.

## Provenance

- Baseline HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
- Baseline state: uncommitted working tree, captured at `2026-09-26T23:36:11Z`.
- Validation receipt generated at `2026-09-26T23:51:55Z`.
- Linter: `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256 `e47974fe903b225e96fec580040604d80380edda054a7b2187bbf5b18da9a30b`.
- Exact initial `git diff HEAD --stat`, `git status --short --untracked-files=all`,
  prompt/body digests and final seals are recorded in
  `00_nucleo/diagnosticos/l0-infra-wiring-history/manifest.json`.

These are measurements of this batch, not a new language-parity measurement.
Other dirty/untracked families were preserved and were not resealed by this task.

## Evidence and contract treatment

- Current source symbol/line anchors precede contracts and include a reproducible
  `rg` discovery command. The audit verified that every anchor resolves.
- Child prompts own only their declared source. Parent lists route to child
  owners rather than repeat their algorithms. Test prompts own tests, not
  productive implementation.
- Dead/unpinned historical references remain only in the archives; they do not
  prove current behavior. Old vanilla pins and historical gate labels are not
  promoted to current approvals or claims against the current upstream pin.
- Superseded PNG document/CLI unit asymmetry, per-item math reconciliation,
  future-only eviction integration and planned/placeholder test labels were
  replaced by current bounded contracts without changing productive behavior.
- Existing nucleus identities and SHA-256 pins were preserved byte-for-byte;
  no nucleus was edited, created or repinned. Normative prose is English.
  `Hash do Código` and `Núcleos Tekt` remain canonical parser metadata, not
  untranslated normative prose. The latter is displayed as a metadata code
  block: the current parser requires exact spelling and adjacent pin lines
  (`/repos/Antigravity/tekt-linter/03_infra/nucleus.rs`,
  `parse_prompt_nucleus_refs`). Intermediate formatting errors were corrected
  before final resealing and validation.

## Validation

| Check | Result |
| --- | --- |
| `crystalline-lint --checks v15,v26 --fail-on warning .` | Exit 0, zero violations globally |
| `crystalline-lint --checks v5,v15,v26 --format sarif --fail-on warning .` | Exit 1: 38 V5 findings outside the batch; zero V15/V26 and zero findings in the batch |
| `crystalline-lint --fix-hashes --dry-run .` after scoped resealing | No pending repair for any batch consumer; unrelated repairs not applied |
| Body SHA-256 excluding only the `@prompt-hash` line | All 61 consumers match the captured pre-edit bytes |
| Original archive SHA-256 | All 61 archived texts match the captured prompts exactly |
| Reference/anchor/pin audit | Canonical paths resolve, source anchors match, all original pins retained |

No cargo build, behavior tests or global formatting ran: the user requested a
documentation/lineage batch without productive changes. Test commands in L0
are reproducible verification instructions, not claims that those tests ran.

Global V5 failure is not silently waived: its exact remaining paths are listed
below and in the receipt. Repairing them requires a separate scoped request.

## V5 findings outside this batch

- `03_infra/src/export/builder/fonts.rs`.
- `03_infra/src/export/builder/metadata.rs`.
- `03_infra/src/export/builder/navigation.rs`.
- `03_infra/src/export/builder/resources.rs`.
- `03_infra/src/export/builder/serialization.rs`.
- `03_infra/src/export/builder/streams.rs`.
- `03_infra/src/export/builder.rs`.
- `03_infra/src/export/html/document.rs`.
- `03_infra/src/export/html/encoding.rs`.
- `03_infra/src/export/html/inline.rs`.
- `03_infra/src/export/html/p1360_ab_tests.rs`.
- `03_infra/src/export/html/pretty.rs`.
- `03_infra/src/export/html/table.rs`.
- `03_infra/src/export/html/tags.rs`.
- `03_infra/src/export/html/tests.rs`.
- `03_infra/src/export/html/whitespace.rs`.
- `03_infra/src/export/html.rs`.
- `03_infra/src/export/images/icc.rs`.
- `03_infra/src/export/images/jpeg.rs`.
- `03_infra/src/export/images/png.rs`.
- `03_infra/src/export/images/scan.rs`.
- `03_infra/src/export/images/xobjects.rs`.
- `03_infra/src/export/images.rs`.
- `03_infra/src/export/stream/draw.rs`.
- `03_infra/src/export/stream/page.rs`.
- `03_infra/src/export/stream/shape.rs`.
- `03_infra/src/export/stream/text.rs`.
- `03_infra/src/export/stream.rs`.
- `03_infra/src/export/svg/geometry.rs`.
- `03_infra/src/export/svg/glyphs.rs`.
- `03_infra/src/export/svg/groups.rs`.
- `03_infra/src/export/svg/images.rs`.
- `03_infra/src/export/svg/navigation.rs`.
- `03_infra/src/export/svg/p1360_ab_tests.rs`.
- `03_infra/src/export/svg/paints.rs`.
- `03_infra/src/export/svg/serialization.rs`.
- `03_infra/src/export/svg/tests.rs`.
- `03_infra/src/export/svg.rs`.
