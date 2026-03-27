//! DTO de comparação neutro para testes de paridade de parsing.
//!
//! Elimina spans estruturalmente. Normaliza SyntaxKind via .name()
//! (nome canónico minúsculo) para independência do formato Debug.

/// Representação neutra de um SyntaxNode para comparação de paridade.
#[derive(Debug, PartialEq)]
pub enum CompactNode {
    /// Nó folha: (kind_name canónico, texto exacto do token)
    Leaf(String, String),
    /// Nó interior: (kind_name canónico, filhos)
    Branch(String, Vec<CompactNode>),
    /// Nó de erro: (mensagem de erro, texto original)
    Error(String, String),
}

/// Converte SyntaxNode do parser ORIGINAL para CompactNode.
///
/// Usa `.kind().name()` para normalização canónica de SyntaxKind.
pub fn compact_original(node: &typst_syntax::SyntaxNode) -> CompactNode {
    use typst_syntax::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let children: Vec<_> = node.children()
        .map(compact_original)
        .collect();

    let kind_name = node.kind().name().to_string();

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}

/// Converte SyntaxNode do parser CRISTALINO para CompactNode.
///
/// Usa `.kind().name()` — mesmo método, mesmo output canónico.
pub fn compact_cristalino(
    node: &typst_core::entities::syntax_node::SyntaxNode,
) -> CompactNode {
    use typst_core::entities::syntax_kind::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let children: Vec<_> = node.children()
        .map(compact_cristalino)
        .collect();

    let kind_name = node.kind().name().to_string();

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}
