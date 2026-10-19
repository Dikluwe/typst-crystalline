# Batch D layout audit — phase 1

Exact state: `audit-before.json` (HEAD, UTC, full changed-file diffstat, ten source/L0 SHA-256), then `phase1-state.json`. Selected original cases are copied verbatim into `selected-cases.json`. This is static local evidence, not a new vanilla measurement.

Read current owning L0s and ADR-0107/0108/0109/0127 plus 01_core/CLAUDE.md. L0 additions preceded all tests. No implementation, Cargo, global formatting, hashes/seals or external-owner changes were performed. Test status: pending coordinator baseline. The runtime controls preserve existing semantics; their expected baseline is green, while future-variant structural baseline is red (selected match does not require a decision).

- 039: numbering admission is Str/Func only; segmented patterns bypass it. Suppression occurs after counter increment and creates reachable outer None. Other Some payloads are excluded by admission, not valid counted inputs. Two added tests cover absence/Value::None/wrong primitive/wrong content/pattern/suppression/deferral and unresolved versus consumed callback; panic native proves no callback execution in layout.
- 041: Source/Located/Realized/ShowRecipe have different causal delegation. Ordinary complement forwards through locator once. Added Source→Located→Metadata and following raw Metadata test checks no duplicate allocation, preserved text and explicit ID.
- 042: observer covers Text/Space/SpaceFromNewline; remainder is absence of an extra event at this observer, not absence of downstream behavior. Added quote control covers transparent Source, metadata neutrality and both space origins.
- 043: all current margin×alignment pairs are representable; producer normalizes margins to Left/Right. Added full product test checks four shifted pairs, manual unused margins and all other zero shifts, item ranges/order, stable baselines and drain/idempotence.
- 044: complete optional horizontal domain remains physical Left/Start zero, Center half-free, Right/End free, then clamp. Added six-state × narrower/oversized × top/bottom test verifies body position, insertion-index rebase and orphan/decor frame agreement. No RTL/dx/dy/scope expansion.
- 045: independently nested CounterKey/Selector/ElementKind domain plus figure-prefix false residual. Added all current ElementKinds, every non-Kind selector, Page and string boundaries, empty/unknown figure suffix and explicit no-execution native. No inferred supplements for filtered Heading or Figure kind.

New names: `test-inventory.json`; selected exact fn/anchor/domain: `structural-anchors.json`. Final implementation will keep the original match anchors except references may split into nested `match selector` and `match kind`; coordinator will receive the exact final anchors before mutation tests.

## Existing functional controls requested from coordinator

The following exact names live in `01_core/src/compiler/layout/tests.rs`; do not substitute compile success for their acceptance:

- `sincronizacao_locator_layouter_iguala_walk_introspect`, `gating_locator_apenas_em_locatables`: occurrence sequence synchronization and nonlocatable neutrality.
- `prepared_realized_origins_do_not_rebuild_introspection`, `prepared_realized_origins_keep_descendant_anchors`, `unprepared_realized_origins_keep_fresh_index_fallback`: realized origin and visual causality.
- `smartquote_semantic_render_uses_context_not_independent_toggles`, `smartquote_semantic_render_styles_disabled_custom_and_checkpoint`: quote prime/context/style and rollback preservation.
- `p245_place_float_true_bottom_renderiza_no_fundo_da_pagina`, `p245_place_float_true_top_renderiza_no_topo_da_pagina`, `p1292_bottom_float_anchor_fisico_independente_do_clearance`, `p1292_bottom_float_clearance_altera_reserva_e_fitting`, `p1292_flush_checkpoint_atomico_move_todo_o_sufixo`: float physical anchors, fitting and atomic replay.
- `p1140_25_supplement_none_remove_prefixo_e_nbsp`, `ref_supplement_explicit_overrides_default`, `p1073_ref_table_supplements_en_e_pt`, `p1078_ref_heading_supplement_pt_secao`, `p1073_ref_supplements_pt`: explicit empty/override precedence, NBSP and localized implicit supplement.
- `p1458_figure_reference_uses_owner_localization_for_all_kinds`, `p1459_heading_reference_uses_canonical_owner_localization` in references.rs preserve language-owner delegation.

Remaining verification: coordinator runs baseline controls, authorizes implementation, then functional regressions and exact-match future-variant diagnostics (including both HAlign product axes, inner Option HAlign, Value and nested Selector/ElementKind). No runtime fixture change is warranted solely because the correction is structural.

## Implementation candidate

Coordinator baseline accepted all eight layout controls; implementation was authorized after the sealed L0 and archived future-variant baseline. The candidate closes both Content decisions in place, the optional Value realization with distinct outer None, the complete HAlign product and optional HAlign, and nested CounterKey/Selector/ElementKind with the figure-prefix guard residual intact. No tests were changed after baseline, no callback execution or public/default/phase changes were introduced, and no productive owner outside the five assigned files was edited. No Cargo, global formatting or resealing was run by this agent.

`implementation-state.json` freezes HEAD/UTC/full working-tree diffstat/SHA-256. `structural-anchors-final.json` identifies all six original matches and the two nested supplemental decisions. Structural checks must mutate Selector and ElementKind independently and attribute E0004 to their respective nested match in default_supplement_for_key, as well as the original CounterKey match. HAlign must still be tested on the optional domain and each product axis. Functional and structural final validation remain coordinator-owned; candidate readiness does not certify their completion.
