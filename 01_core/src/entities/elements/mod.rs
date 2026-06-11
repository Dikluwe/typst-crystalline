//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/_comum.md
//! @prompt-hash 44210c74
//! @layer L1
//! @updated 2026-06-10
//!
//! Trait `Element` — modelo D (ADR-0105), lote piloto P316.
//!
//! Cada variante migrada do `Content` passa a `Nome(Arc<nome::Nome>)`,
//! e o struct `NomeElem` implementa este trait, absorvendo a lógica
//! por-variante dos 6 matches gigantes de `content.rs`. Refatoração de
//! **comportamento idêntico**: a lógica muda de morada, a semântica não.
//!
//! `eq`/`hash` NÃO são métodos do trait (ver `_comum.md` §A.1.1.b):
//! `PartialEq` é derivado em cada `NomeElem` (estrutural via `Arc`), e o
//! `Content` despacha `(Nome(a), Nome(b)) => a == b`. `hash` continua via
//! `content_hash::hash_content` (Debug).
//!
//! Layout NÃO entra no trait (topologia: `entities` não depende de
//! `rules`); fica em `rules/layout` / `rules/math/layout`.

pub mod divider;
pub mod heading;
pub mod math_styled;
// Lote 2 P317 — família math element-shaped (11 variantes).
pub mod math_accent;
pub mod math_align_point;
pub mod math_attach;
pub mod math_cancel;
pub mod math_cases;
pub mod math_delimited;
pub mod math_frac;
pub mod math_matrix;
pub mod math_op;
pub mod math_root;
pub mod math_underover;
// Lote 3 P318 — família lista/termos (5 variantes element-shaped).
pub mod enum_item;
pub mod link;
pub mod list_item;
pub mod term_item;
pub mod terms;
// Lote 4 P319 — decorações de texto (3 variantes element-shaped).
pub mod overline;
pub mod strike;
pub mod underline;
// Lote 5 P320 — quebras/espaços + grid/table header/footer (9 variantes).
pub mod colbreak;
pub mod grid_footer;
pub mod grid_header;
pub mod h_space;
pub mod linebreak;
pub mod pagebreak;
pub mod table_footer;
pub mod table_header;
pub mod v_space;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Comportamento por-variante absorvido dos matches do hub `content.rs`.
///
/// Despacho estático (o trait não é object-safe por causa dos métodos
/// genéricos `map_*` — e não precisa de ser: o `match Content` conhece o
/// tipo concreto de cada braço).
pub trait Element: Clone + PartialEq + std::hash::Hash + std::fmt::Debug {
    /// Texto plano (consumido por `Content::plain_text`).
    fn plain_text(&self) -> String;

    /// Vazio estrutural. Default `false`.
    fn is_empty(&self) -> bool {
        false
    }

    /// Reconstrói-se com filhos transformados; devolve `Content`
    /// (re-embrulha a própria variante).
    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>;

    /// Transformação de texto terminal; devolve `Content`.
    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String;

    /// Acesso a campo nomeado (show rules). Default `None`.
    fn get_field(&self, _field: &str) -> Option<Value> {
        None
    }

    /// Discriminador de introspecção (absorção do locatável). Default `None`.
    fn element_kind(&self) -> Option<ElementKind> {
        None
    }

    /// Payload de introspecção (absorção do locatável). Default `None`.
    fn to_payload(&self) -> Option<ElementPayload> {
        None
    }
}
