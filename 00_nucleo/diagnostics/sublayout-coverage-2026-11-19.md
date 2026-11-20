# Sublayout routes and test coverage

Source review at `d6f402b26815a6eaafbb9235e4a271d6cdd75771`, with no
changes in the product layers or owning L0 prompts. References below
are relative to the repository root. The policy is in
[ADR-0124](../adr/adr-0124-sublayout-propagation-checklist.md).

## Evidence from the shared implementation

- `01_core/src/compiler/layout/sub_frame.rs:179` and `:201` delegate to the
  same isolated executor at `:330`. Box selects ordinary or natural width.
  The other adapter selects ordinary mode and passes the caller's geometric
  origin choice. Dispatch distinguishes source routes at `:545`. The local
  terminal line uses its own finalization at `:564`, calling the shared
  pending-width helper and conditional RTL alignment before draining.
  `line_commit.rs:416` handles ordinary line commit, with column-sensitive
  geometry at `:477` and root admission guards at `:641`.
- `sub_frame.rs:987` tests bounded isolation with both origin modes.
  `bounded_subframe_retains_overflowing_lines_without_parent_pages`
  checks body text and height, parent cursor, dimensions, pending text
  and absence of parent pages. The ordinary-flow control checks pagination
  at `:1073`. The fractional-tail and repeat tests at `:1200` and `:1221`
  assert local bounds and expansion in both origin modes. The tests at
  `:1293` and `:1298` check that the parent's pending fraction and repeat
  survive the child's line flush. This covers the isolated region under
  the inputs exercised. The tests invoke it without a planned source cursor.
- `helpers.rs:782`, `:790` and `:840` assert separator visual extent,
  translation, and width/extraction of a text replacement. These tests
  cover common frame operations. They do not exercise wrapper dispatch,
  the placement of the returned frame, or a caller's choice of metrics.

## Caller-specific evidence

| Path | Source and current coverage | Boundary of that coverage |
|---|---|---|
| Grid and Table | Grid and Table normalize inputs into `layout_grid` at `grid.rs:184` and `table.rs:173`. `tests.rs:15340` compares actual text positions for equivalent cells. `grid/cell_body.rs:319` computes inset bounds, `:332` chooses whether to consume a measured product, and `:394` selects the origin for a fresh body. The tests at `tests.rs:24906` and `:24934` exercise reuse and fallback. | A cache hit, row placement or a cell's default settings need a cell or row case. |
| Box | `boxed.rs:52` selects an allocated independent region or a provisional fractional path. The isolated branch supplies inset-adjusted dimensions at `:472–512`. The other captures committed items and the pending inline tail at `:224–245` through `sub_frame.rs:834`. Box tests cover clip/baseline at `boxed.rs:1072`, inset and physical position at `:1354`, and the inner width and text placement of a nested fractional box at `:2012`. | A change to clipping, allocation, inline-tail capture or the boundary between body and following text needs a Box assertion. The inline adapter also has a caller in `style_scope.rs:151` for PDF labels. |
| Place | `place.rs:42–88` defers the float source cursor and captures at a non-geometric origin with RTL alignment disabled. In-place Place uses `placement.rs:641–652` with a geometric origin and then computes its anchor and pending fixups. `tests.rs:13945` checks a pending line's anchor. `:13992` checks that clearance changes fitting. `:5326` checks that adding Align to Place in a Grid cell preserves the x coordinate. | The combined Grid/Place case covers their origin interaction. Anchor selection, float reservation and unrelated cell behavior need their own assertions when affected. |
| Columns | `columns.rs:218–258` selects explicit segmentation only for a column break without note flow. Continuous flow installs column state and commits lines at `columns/flow.rs:104–130`. The segmented path sets a per-segment region and commits at `columns/segmented.rs:231–265`. `region_progress.rs:23–43` advances or merges columns. Page-level columns are constructed with a distinct flag at `01_core/src/entities/content/page_structure.rs:228–238`. Existing tests cover RTL in page and container columns at `tests.rs:7340` and `:7361`, multi-page flow at `:7417`, explicit breaks at `:8983`, and page-owned notes at `:9075`. | Shared line operations can use a common test for a covered branch. Segment selection, translation to the next column or page, note ownership and restoration require a case that crosses the affected boundary. |

## Validation

The layout suite ran on 2026-11-20 UTC using the existing `typst-core`
test executable from the release-integration suite. The product layers,
build files and owning prompts match the recorded Git revision. The run
passed 1,795 tests, with three ignored. The execution timestamps, local
diffstat and test binary SHA-256 are recorded in
`lab/results/construction/diagnostics/adr-0124-sublayout-2026-11-19/`.
