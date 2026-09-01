//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/_comum.md
//! @prompt-hash 4477c7e5
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
//! `rules`); fica em `compiler/layout` / `rules/math/layout`.

pub mod divider;
pub mod emph;
pub mod flush;
pub mod heading;
pub mod math_styled;
pub mod strong;
pub mod title;
// Lote 2 P317 — família math element-shaped (11 variantes).
pub mod math_accent;
pub mod math_align_point;
pub mod math_attach;
pub mod math_cancel;
pub mod math_cases;
pub mod math_class_override; // P772y
pub mod math_delimited;
pub mod math_frac;
pub mod math_limits_override; // P992
pub mod math_matrix;
pub mod math_op;
pub mod math_root;
pub mod math_underline;
pub mod math_underover;
pub mod math_vec;
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
pub mod page_run;
pub mod pagebreak;
pub mod table_footer;
pub mod table_header;
pub mod v_space;
// Lote 6 P321 — família state/counter + Metadata (7 variantes; 6 locatáveis).
pub mod counter_display;
pub mod counter_display_callback;
pub mod counter_update;
pub mod metadata;
pub mod state;
pub mod state_display;
pub mod state_update;
// P506 — context block (delayed evaluation).
pub mod context_block;
// Lote 7 P322 — por largura (5 variantes element-shaped).
pub mod align;
pub mod hide;
pub mod image;
pub mod raw;
pub mod repeat;
// Lote 8 P323 — por largura (4 variantes element-shaped).
pub mod columns;
pub mod outline;
pub mod quote;
pub mod r#ref;
// Lote 9 P324 — por largura (5 variantes element-shaped).
pub mod cite;
pub mod pdf_artifact;
pub mod pdf_attach;
pub mod place;
pub mod smartquote;
pub mod stack;
pub mod transform;
// Lote 10 P325 — por largura (3 variantes element-shaped).
pub mod bibliography;
pub mod equation;
pub mod pad;
// Lote 11 P326 — por largura (2 variantes element-shaped).
pub mod footnote;
pub mod shape;
// Lote 12 P327 — bloco grid/table cell (4 variantes element-shaped).
pub mod grid;
pub mod grid_cell;
pub mod table;
pub mod table_cell;
// Passo 512 — linhas em grid/table.
pub mod grid_hline;
pub mod grid_vline;
pub mod table_hline;
pub mod table_vline;
// Passo 513 — curve elements (move/line/cubic/quad/close).
pub mod curve;
// Lote 13 P328 — o último element-shaped.
pub mod figure;
// Lote 14 P329 — reclassificados da triagem DEBT-58 (wrappers densos).
// P464: `labelled` consolidado em `label` (campo `auto`).
pub mod boxed;
pub mod label;
// Lote 15 P330 — o último lote.
pub mod block;

// Lote F-1 P334 — a fronteira de extensão E1 (object-safe `DynElement`).
// L0 próprio: `entities/f_fronteira_e1.md` (não `_comum.md`).
pub mod dynamic;
pub use dynamic::DynElement;
// Fixture do `callout` (elemento de utilizador de teste; fora dos 65).
#[cfg(test)]
pub mod test_callout;

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

    /// Id estável do kind para o match de seletor `#show` (S1; F-2+).
    /// Default `""` = elemento sem kind distinto matchável — os 65 nativos,
    /// que são despachados **estaticamente** pelo `match Content` e nunca
    /// usam este id. Os elementos **dinâmicos** (utilizador, `Content::Dynamic`)
    /// sobrepõem com o seu nome registrado. Defaultado aqui para **não tocar
    /// os 65 módulos** (ADR-0106; L0 `f_fronteira_e1.md` §3a, ajuste de Fase A
    /// P334: `dyn_kind_name` mora em `Element` com default, não só em
    /// `DynElement`, para o blanket ser trivial).
    fn dyn_kind_name(&self) -> &'static str {
        ""
    }

    /// **F-item3 (P368, `f_fronteira_e1.md` §3a.11)** — resolve campos **setáveis**
    /// a partir da chain (o mapa aberto). `get(prop)` devolve o valor de
    /// `#set <kind>(prop:)` na chain (já namespaced pelo kind via o custom). O
    /// elemento aplica-o aos seus campos **opcionais não-construídos**
    /// (precedência **construído explícito > chain**) e devolve-se resolvido.
    /// Default: **sem campos setáveis** — devolve-se inalterado (os 65 nativos e os
    /// elementos sem props opcionais). Espelha a resolução `instância → chain →
    /// default` do §3b.1 / o getter `#[ghost]` do vanilla.
    fn resolve_settable(&self, _get: &dyn Fn(&str) -> Option<Value>) -> Content
    where
        Self: Sized + Send + Sync + 'static,
    {
        Content::dynamic(self.clone())
    }
}
