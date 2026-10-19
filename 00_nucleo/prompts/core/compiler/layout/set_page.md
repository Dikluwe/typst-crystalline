# L0 — Page configuration adapter
Hash do Código: c2f82370

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/set_page.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130, ADR-0131.

## Evidence before specification

The historical construction specification measured `01_core/src/compiler/eval/rules.rs:743-779`: page read width/height/margin/numbering but ignored columns; `layout/mod.rs:858-860` delegated SetPage to an adapter updating those fields; `entities/layout_types.rs:433-449` had no PageConfig.columns; `layout/columns.rs:59-161` rendered real side-by-side columns split by Colbreak; `eval/mod.rs:407-566` built Sequence with SetPage followed by sibling content. The original construction interface examples and test scaffolding are preserved in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/set_page.md`.

The historical footnote correction distinguishes explicit columns() from page(columns:). Source documentation at historical ratified snapshot `e0e8ca4d`, `crates/typst-library/src/layout/columns.rs:28-35`, also published at `typst.app/docs/reference/layout/columns/#page-level`, says page-level columns avoid an enclosing layout container and therefore keep pagebreaks, footnotes and line numbers working as expected. This supports the distinction and rationale for page_columns, but does not specify exact explicit-container footnote geometry. The claim that explicit columns() stacks notes at the container end in the left column remains an inference without a recorded direct bilateral measurement; direct measurement of both forms against the ratified reference could refute it. See `00_nucleo/prompts/core/entities/elements/columns.md` for the same qualification. These historical snapshots do not certify the current pinned reference without new measurement.

Consolidation source inspection found the current exclusive consumer at `01_core/src/compiler/layout/set_page.rs:17-139` accepts `&SetPageDelta`, not the historical independent parameter list. It folds paper/flip/binding/dimensions/margins through page_geometry, plus numbering/alignment/marginals/supplement/columns/bleed/fill/canvas. On changed configuration it flushes and opens a page if material exists, then installs configuration/region dimensions and margins. Initial-baseline-pending uses margin.top only; otherwise current text_edges supplies top. `entities/content.rs:169-183,684` has SetPageDelta.columns: Option<usize> and SetPage(Arc<SetPageDelta>); `entities/layout_types.rs:803` has PageConfig.columns; `eval/mod.rs:934` applies wrap_page_columns. This inspection establishes observed current code, not upstream intent, new parity or authorization to discard conflicting historical requirements.

## Exclusive ownership and adapter contract

This prompt owns only `01_core/src/compiler/layout/set_page.rs`, layer L1. Architecture: ADR-0107/0108/0109/0127/0129/0130/0131 and the graded-scope principle cited historically as ADR-0054. Shared public entities, eval and tests have their own owners; this contract does not authorize their reimplementation under a shared prompt.

Apply supplied typed page-configuration deltas through the existing static adapter. Propagate columns to PageConfig and mark change when it differs. Reuse the existing real-column consumer rather than recreating column mechanics. Preserve the page-level versus explicit-container distinction: the synthetic ColumnsElem emitted by wrap_page_columns carries page_columns:true. The original broad equivalence claim applies only to real N-column content/colbreak behavior, not to identical footnote placement.

The observed current adapter also preserves existing geometry-helper delegation, lexical direction resolution, independent optional marginal/numbering/supplement/bleed/fill/canvas updates and auto-margin refresh. State synchronization follows the existing region width/height/cursor/line-start boundary. Do not add I/O, callbacks or a new column algorithm.

## Cross-owner boundary requirements retained from construction

`00_nucleo/prompts/core/entities/content.md` owns the public content/delta representation; `00_nucleo/prompts/core/entities/layout_types.md` owns PageConfig. Historical columns:Option<usize> requirements remain: None represents absence/clear behavior, default None, and Some(0) must not be produced by validated eval. Historical struct-shaped SetPage and PageConfig snippets are archival examples, not instructions to regress the current delta/Arc or other existing fields.

`00_nucleo/prompts/core/compiler/eval/rules.md` owns page argument handling: the historical specification requires Int(n>=1)→Some(n as usize), Int(n<1)→`columns must be at least 1`, None→None, and other types ignored under the existing inherited/ignore behavior. Preserve that requirement as a boundary criterion rather than silently inventing a new validation policy here.

`00_nucleo/prompts/core/entities/content.md` and `00_nucleo/prompts/core/compiler/eval/mod.md` own the pure wrap_page_columns transformation and its invocation. Recursively traverse Content. In a Sequence at any level, each SetPage with columns:Some(n) groups subsequent siblings until another SetPage (with/without columns), Pagebreak or Sequence end into Columns with count n and Sequence body. Keep original SetPage immediately before Columns so configuration is installed before consumption. Recurse into produced body for nested set-pages and ordinary containers. A single body bridges bodyless set-rule to the existing column consumer with limited scope; the transformation remains pure L1. It wraps the remainder of lexical scope, not each physical page.

The eval invocation is after run_pass(true), before module.set_content(rendered_content), and affects rendered_content only. Historical rationale: original_content serves TagIntrospector locatable elements; SetPage is not locatable and Columns is not expected to change heading/label location observables. This is an explicit inference/limit: revise the alignment decision if discrepant introspection is measured, rather than assuming a second transformation already occurs.

## Explicit unresolved conflicts and scope

The original adapter requirement said not to force a fresh page solely for columns changes; its certified case was a top-of-document set on an already-empty page. Current source marks differing columns changed and takes the same nonempty-page transition path as other changed configuration. No explicit superseding decision was found in this owner. Preserve this as an unresolved columns-only transition contract conflict; inspection alone does not settle it or authorize a behavior change.

The historical cursor requirement used margin + FontMetrics::cap_height(style.size), explicitly excluding ascender. Current source instead defers pending baseline to later effective-font admission and otherwise uses active text_edges. This is another explicit unresolved historical-versus-current contract conflict; do not reinstate cap-height, add top twice or certify one formula solely from source. General baseline ownership remains `00_nucleo/prompts/core/compiler/layout/mod.md` and relevant text-baseline owners; a substantive resolution needs evidence before decision.

Mid-document real pagination when page columns change remains outside the original certified scope; another SetPage/Pagebreak terminates lexical grouping. page(gutter:...) is out of scope. Explicit columns:none clearing was declared supported, not future work. The inferred exact explicit-container footnote layout and rendered/original-content location alignment remain qualified as above.

## Acceptance and provenance

`00_nucleo/prompts/core/compiler/layout/tests.md` owns integration tests. Preserve an eval→layout source case `#set page(columns: 2)` followed by A, colbreak(), B: A/B must have distinct x positions, or use equivalent direct Content construction if E2E is too costly. Preserve the source test identifier p537b_set_page_columns_produz_colunas_reais and its archival scaffold without treating it as this owner's test-file ownership.

For page-level per-column footnotes, construct SetPage(columns:Some(2)), Hello/Nota A/world., colbreak(false), Goodbye/Nota B/moon., transform/evaluate, and check distinct note x positions and y near each column bottom under the established p537_footnotes_columns_colbreak_posicionam_por_coluna criterion. Keep regression controls p537_footnotes_columns_colbreak_posicionam_por_coluna and p220_colbreak_dentro_columns_separa_colunas_reais unchanged. Reject page(columns:0) with exact `columns must be at least 1`; page(columns:-1) errors.

Require clean cargo test --workspace and crystalline-lint, valid @prompt/@prompt-hash lineage in every affected owner and retained report `lab/results/construction/diagnostics/paridade-producao-p537b.md`. Historical tests/report paths and identifiers are provenance, not execution-step organization in the active owner. These acceptance obligations do not close the explicit conflicts or scope exclusions above.
