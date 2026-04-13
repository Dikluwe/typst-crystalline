//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style_chain.md
//! @prompt-hash 4f9f20b5
//! @layer L1
//! @updated 2026-04-01

use std::sync::Arc;

use crate::entities::layout_types::{Pt, TextStyle};

/// Um delta de estilo — apenas as propriedades que este nó define explicitamente.
/// Propriedades ausentes são herdadas do nó pai na cadeia.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,   // em pontos tipográficos
}

impl StyleDelta {
    pub const fn empty() -> Self {
        Self { bold: None, italic: None, size: None }
    }
}

/// Nó interno da lista ligada.
#[derive(Debug, Clone)]
struct StyleNode {
    delta:  StyleDelta,
    parent: Option<Arc<StyleNode>>,
}

/// Lista ligada imutável de deltas de estilo.
///
/// Clone é O(1) — apenas o `Arc` do nó de topo é clonado.
/// Leitura é O(N) percorrendo a cadeia até encontrar o primeiro delta
/// que define a propriedade (N tipicamente < 10).
///
/// Equivalente simplificado de `StyleChain` do Typst original.
/// Suporta `#set text(bold: true)` e herança em blocos aninhados.
///
/// Pureza L1: não usa I/O de sistema — apenas `Arc` e `Vec` em memória.
#[derive(Debug, Clone)]
pub struct StyleChain(Option<Arc<StyleNode>>);

impl StyleChain {
    /// Cadeia vazia — resolve para os defaults codificados em cada accessor.
    pub const fn empty() -> Self {
        StyleChain(None)
    }

    /// Cadeia com os valores por defeito do motor Typst.
    /// bold: false, italic: false, size: 11.0pt
    pub fn default_chain() -> Self {
        let root = StyleNode {
            delta: StyleDelta {
                bold:   Some(false),
                italic: Some(false),
                size:   Some(11.0),
            },
            parent: None,
        };
        StyleChain(Some(Arc::new(root)))
    }

    /// Cria uma nova cadeia que herda desta e aplica `delta` por cima.
    /// Custo: O(1) — cria um novo `Arc`.
    pub fn push(&self, delta: StyleDelta) -> Self {
        let node = StyleNode {
            delta,
            parent: self.0.clone(),
        };
        StyleChain(Some(Arc::new(node)))
    }

    /// Resolve `bold` percorrendo a cadeia até ao primeiro delta que o define.
    pub fn bold(&self) -> bool {
        self.resolve_bool(|d| d.bold).unwrap_or(false)
    }

    /// Resolve `italic`.
    pub fn italic(&self) -> bool {
        self.resolve_bool(|d| d.italic).unwrap_or(false)
    }

    /// Resolve `size` em pontos tipográficos.
    pub fn size(&self) -> f64 {
        self.resolve_f64(|d| d.size).unwrap_or(11.0)
    }

    fn resolve_bool(&self, f: impl Fn(&StyleDelta) -> Option<bool>) -> Option<bool> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    fn resolve_f64(&self, f: impl Fn(&StyleDelta) -> Option<f64>) -> Option<f64> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }
}

/// Conversão para `TextStyle` plano — bridge para layout e export actuais
/// enquanto a migração completa para StyleChain não está feita.
impl From<&StyleChain> for TextStyle {
    fn from(chain: &StyleChain) -> Self {
        TextStyle {
            bold:   chain.bold(),
            italic: chain.italic(),
            size:   Pt(chain.size()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_chain_defaults() {
        let chain = StyleChain::default_chain();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(),   11.0);
    }

    #[test]
    fn style_chain_push_herda() {
        let base  = StyleChain::default_chain();
        let child = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
        assert!(child.bold());
        assert!(!child.italic());  // herdado
        assert_eq!(child.size(),   11.0);   // herdado
    }

    #[test]
    fn style_chain_push_multiplos_niveis() {
        let base  = StyleChain::default_chain();
        let mid   = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
        let child = mid.push(StyleDelta { bold: None, italic: None, size: Some(14.0) });
        // bold herdado de mid, size de child, italic do root
        assert!(child.bold());
        assert!(!child.italic());
        assert_eq!(child.size(),   14.0);
    }

    #[test]
    fn style_chain_clone_e_o1() {
        let base  = StyleChain::default_chain();
        let chain = base.push(StyleDelta { bold: Some(true), italic: None, size: None });
        let clone = chain.clone();
        assert!(clone.bold());
        // Clone correcto — mesmo que O(1) não seja verificável directamente
    }

    #[test]
    fn text_style_from_style_chain() {
        let chain = StyleChain::default_chain()
            .push(StyleDelta { bold: Some(true), italic: None, size: Some(14.0) });
        let ts = TextStyle::from(&chain);
        assert!(ts.bold);
        assert!(!ts.italic);
        assert_eq!(ts.size.val(),   14.0);
    }

    #[test]
    fn empty_chain_usa_defaults() {
        let chain = StyleChain::empty();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(),   11.0);
    }
}
