//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/show.md
//! @prompt-hash 21e02485
//! @layer L1
//! @updated 2026-04-19

use crate::entities::value::Value;

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
}

/// Uma regra de transformação declarada com `#show selector: transform`.
#[derive(Debug, Clone)]
pub struct ShowRule {
    pub id:        RuleId,
    pub selector:  Selector,
    pub transform: Value,
}
