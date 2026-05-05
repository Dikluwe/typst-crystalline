//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_counters.md
//! @prompt-hash dd2f700b
//! @layer L1
//! @updated 2026-05-05
//!
//! **P190I (M6 fechado)** — módulo esvaziado.
//!
//! Helpers `layout_set_heading_numbering`, `layout_set_equation_numbering`,
//! `layout_counter_update`, `format_counter_display` ELIMINADOS:
//! mutavam `CounterStateLegacy` que foi eliminada em P190I (M6
//! fechado). Caminho Introspector path puro via `self.introspector`
//! em Layouter para todos os consumers (P184D / P190G/H/I migrations).
//!
//! Módulo preservado para histórico arquivístico do prompt-hash;
//! body vazio. Eliminação total em refactor futuro de housekeeping.
