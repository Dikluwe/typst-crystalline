//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/show.md
//! @prompt-hash 59184b04
//! @layer L1
//! @updated 2026-04-19

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::func::Func;
use crate::entities::style::Styles;

/// Identificador único de uma show rule por sessão de avaliação.
pub type RuleId = u64;

/// Tipo de nó de conteúdo para selectorção por tipo (Passo 69 — DEBT-19 encerrado).
///
/// Conjunto completo: Heading, Figure, Strong, Emph, Raw, Equation, ListItem.
/// Outros tipos (EnumItem, Link, etc.) adicionados em passos futuros.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Heading,
    Figure,
    Strong,
    Emph,
    Raw,
    Equation,
    ListItem,
}

/// Selector de uma show rule.
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Substitui ocorrências literais de um texto.
    /// Ex: `#show "A": "B"`
    Text(String),
    /// Interceta nós de um tipo específico.
    /// Ex: `#show heading: it => ...`
    NodeKind(NodeKind),
    /// **Lote F-3 inc-2** — interceta `Content::Dynamic` de um **kind dinâmico**
    /// (elemento de utilizador, fronteira E1). Ex: `#show callout: it => ...`.
    /// O kind é o nome do elemento (`dyn_kind`), não um endereço de função (que
    /// o elemento não tem). Casado no `apply_show_rules` pelo mesmo caminho dos
    /// nativos, com o mesmo guard por `RuleId` + depth-64.
    DynKind(String),
}

/// A transformação que uma show rule aplica. Materializa o **S5** do spike-2
/// (`Transformation = Content | Func | Style`, `f_fronteira_e1.md §3c`).
/// Substitui o antigo `transform: Value` solto por um vocabulário fechado.
#[derive(Debug, Clone)]
pub enum Transformation {
    /// Closure `it => …` — recebe o nó, devolve `Content` (ou `Str`, promovido
    /// a `Content::text`). Consome o passe de show.
    Func(Func),
    /// Substituição estática direta. Consome o passe.
    Content(Content),
    /// Substituição literal de texto — **apenas** para `Selector::Text` (via
    /// `map_text`). Sobre `NodeKind`/`DynKind` é erro (requer função ou Content).
    Str(EcoString),
    /// **Show-set** (`#show k: set …`, P352). Os styles do `set` são capturados
    /// na declaração (sem mutar `engine.styles` globalmente) e transportados para
    /// o nó casado embrulhando-o num `Content::Styled(elem, styles)` — o
    /// carregador `StyledElem`-scoped da fatia 1 (`f_fronteira_e1.md §3a.8`).
    /// **NÃO consome o passe** (espelha `map.apply; continue` do vanilla).
    Style(Styles),
}

/// Uma regra de transformação declarada com `#show selector: transform`.
#[derive(Debug, Clone)]
pub struct ShowRule {
    pub id:        RuleId,
    pub selector:  Selector,
    pub transform: Transformation,
}
