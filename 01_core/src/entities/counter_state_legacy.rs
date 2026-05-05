//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_state_legacy.md
//! @prompt-hash 702f4cea
//! @layer L1
//! @updated 2026-05-05
//!
//! **P190I (M6 fechado)** — `CounterStateLegacy` ELIMINADA.
//!
//! Histórico (P190B-H + P190I):
//! - P190B: bib_entries, bib_numbers eliminados.
//! - P190C: label_pages, known_page_numbers movidos para
//!   LayouterRuntimeState.
//! - P190D: has_outline, is_readonly eliminados/movidos.
//! - P190G: numbering_active, resolved_labels, headings_for_toc,
//!   auto_label_counter eliminados.
//! - P190H: figure_numbers, figure_label_numbers,
//!   local_figure_counters eliminados.
//! - **P190I (M6 fechado)**: hierarchical, flat, lang eliminados.
//!   Struct, impl, Default eliminados. `display_value` migrado para
//!   `materialize_time` (intr-based). Walk fn drop `state` parameter.
//!   Layouter `counter` field eliminado.
//!
//! Caminho Introspector path puro via `TagIntrospector`
//! (entities/introspector.rs) é única fonte da verdade para counters,
//! state, labels, headings_for_toc, etc. (sub-stores 9, populated
//! por populate_intr_from_tag_start P191B durante walk).
//!
//! Módulo preservado para histórico arquivístico do prompt-hash;
//! body vazio. Re-export de `CounterUpdate` como `CounterAction`
//! preservado para callers historicamente dependentes do alias.
//!
//! **Marco arquitectural**: F1 fechado.

// Re-export histórico — CounterUpdate alias para callers existentes.
pub use crate::entities::counter_update::CounterUpdate as CounterAction;
