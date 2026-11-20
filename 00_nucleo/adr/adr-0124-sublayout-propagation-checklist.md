# ADR-0124 — Verify affected sublayout paths

**Status:** ACCEPTED
**Date:** 2026-07-09
**Revised:** 2026-11-19

## Context

P579/P580 found line-height regressions outside the main flow. P625 found
the same pattern for RTL alignment, and P626 exposed a separate column-order
issue. The construction-era checklist required four copies of the same
layout input to guard against missed propagation.

The current call paths and test assertions are recorded in the
[sublayout review](../diagnostics/sublayout-coverage-2026-11-19.md). Grid
cells, Place bodies and isolated Box bodies share a region executor.
Box also has an inline path. Columns shares ordinary content dispatch and
line commit, with its own region transitions and explicit segmentation.
The isolated subframe's terminal line and the normal line commit have
different finalization paths.

## Decision

For a layout change, trace the affected behavior from the caller's region
and state setup through the changed code to the observable result. Credit
an existing shared test when it reaches the affected branch, supplies
equivalent relevant inputs and state, and asserts the affected behavior.
A test of a common helper gives credit for its own contract. Coverage of
a caller's routing, initialization or output handling requires an
assertion on that caller or its interaction with another path.

Require a targeted case when the change can affect a path's dispatch,
guards, geometry, state restoration, or result handling. A case can be an
existing regression, an added input to a parameterized test, or a combined
case that exercises the affected interaction. Distinguish both sides of a
changed branch when each can affect output.

| Path | Behavior that needs targeted coverage when affected |
|---|---|
| Grid and Table | Cell inset, span or alignment, measurement reuse and fallback, translation into the cell, or row fragmentation. A shared cell-engine test can cover equivalent inputs after normalization. |
| Box | Independent region sizing and output fixups, and the inline fractional path that captures a line tail from the parent. Exercise the relevant path or both if the change affects their selection. |
| Place | In-place anchoring and deferred fixups, or float capture, admission and reservation. Their shared body executor covers body behavior under equivalent region inputs. |
| Columns | Explicit segments versus continuous flow, including column/page transitions. Distinguish page-level and container columns when page-owned notes, numbering or restoration are affected. |
| Shared finalization | Separate an ordinary line commit from a subframe's pending terminal line or inline tail when the change can affect those boundaries. |

Review other reachable consumers when the mechanism extends beyond these
four features. Record why an existing test covers an affected path, identify
the additional assertion used, or cite where an unaffected path diverges.
Put that evidence in the owning L0 or associated diagnostic under ADR-0132.

The owning L0, approval rules under ADR-0127, and any applicable
component-specific verification remain in force. Preserve the source and
measurement provenance required by ADR-0121. Use equivalent inputs on both
sides of parity comparisons under ADR-0122.

## Verification

A reviewer can recover the reachable path, the relevant inputs, and a
test assertion that would distinguish the regression. For shared tests,
the review identifies the common branch and the scope of the assertion.
The fixed construction checklist is retained in Git history.
