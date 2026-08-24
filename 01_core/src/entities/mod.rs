//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/mod.md
//! @prompt-hash e9739221
//! @layer L1
//! @updated 2026-04-30

pub mod args;
pub mod ast;
pub mod bib_entry;
pub mod bib_store;
pub mod citation_form;
pub mod color;
pub mod file_id;
pub mod font_book;
pub mod math_class;
pub mod module;
pub mod operators;
pub mod package_spec;
pub mod resolved_label_store;
pub mod scope;
pub mod source;
pub mod source_result;
pub mod span;
pub mod syntax_kind;
pub mod syntax_mode;
pub mod syntax_node;
pub mod syntax_set;
pub mod syntax_text;
// P468 — estilos de citação bibliográfica (numeric/author-date/alphabetic).
pub mod citation_style;
pub mod content;
pub mod dir;
pub mod elements;
pub mod parity;
pub mod sides;
// P242 — Corners<T> paralelo Sides<T> para radius rounded-rect.
pub mod corners;
pub mod counter;
pub mod counter_registry;
pub mod counter_state_legacy;
pub mod counter_update;
// P536 — metadados do documento definidos por `#set document(...)`.
pub mod document_info;
// P451 — formatação de counters hierárquicos com patterns.
pub mod content_hash;
pub mod counter_format;
pub mod element_info;
pub mod element_kind;
pub mod element_payload;
pub mod layouter_runtime_state;
// Lote F-1 P334 — registro de elementos dinâmicos (fronteira E1).
pub mod element_registry;
pub mod engine;
pub mod font_list;
pub mod font_variations;
pub mod func;
pub mod glyph_variants;
pub mod introspector;
pub mod label;
pub mod label_kind;
pub mod label_registry;
pub mod lang;
pub mod layout_types;
pub mod location;
pub mod locator;
pub mod math_constants;
pub mod metadata_store;
pub mod page_canvas;
pub mod page_geometry;
pub mod page_running;
pub mod page_store;
pub mod page_supplement;
pub mod plugin_func;
pub mod position;
pub mod regex;
pub mod region;
pub mod sealed_positions;
pub mod selector;
pub mod shaped_glyph;
pub mod state;
pub mod state_registry;
pub mod state_update;
pub mod tag;
// P311b.1 — MathStyleKind + map_glyph (Caminho I diagnóstico P311a).
pub mod geometry;
pub mod image_format;
pub mod image_sizer;
pub mod math_style;
pub mod show;
pub mod sink;
pub mod style;
pub mod style_chain;
// P261 — Paint wrapper enum (Solid only) per ADR-0086.
pub mod paint;
// P262 — Gradient Linear-only per ADR-0087; activa Paint::Gradient.
pub mod gradient;
// P395 — Tiling (pattern fill) per ADR-0017.
pub mod tiling;
// P398 — Bytes binários; fecha DEBT-62.
pub mod bytes;
// P399 — Decimal precisão fixa; tipo S puro.
pub mod decimal;
// P400 — Duration (intervalo de tempo); tipo S puro.
pub mod duration;
// P401 — Version (semver); tipo S puro.
pub mod version;
// P264 — Axes<T> minimal per ADR-0088 + ADR-0080; consumer Radial.center.
pub mod axes;
pub mod ptr_eq_arc;
pub mod rel;
pub mod value;
pub mod world_types;
// P470 — Tipos de marcador de list/enum.
pub mod enum_numbering;
pub mod list_marker;
// P471 — Símbolo Unicode nomeado.
pub mod symbol;
pub mod frame_visitor {
    pub use super::layout_types::{walk_frame_items, FrameVisitor};
}
