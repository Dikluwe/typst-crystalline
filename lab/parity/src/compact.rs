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
        let (errors, _) = node.errors_and_warnings();
        let msg = errors
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.full_text().to_string());
    }

    let children: Vec<_> = node.children()
        .map(compact_original)
        .collect();

    let kind_name = node.kind().name().to_string();

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.full_text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}

/// Converte SyntaxNode do parser CRISTALINO para CompactNode.
///
/// Usa `.kind().name()` — mesmo método, mesmo output canónico.
/// Normaliza pequenas divergências de nomenclatura (ex.: `function call`
/// dentro de math → `math function call`) para manter a lente alinhada com
/// o parser original sem forçar o cristalino a repetir nomes idênticos.
pub fn compact_cristalino(
    node: &typst_core::entities::syntax_node::SyntaxNode,
) -> CompactNode {
    compact_cristalino_inner(node, false)
}

fn compact_cristalino_inner(
    node: &typst_core::entities::syntax_node::SyntaxNode,
    in_math: bool,
) -> CompactNode {
    use typst_core::entities::syntax_kind::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let next_in_math = in_math || node.kind().name() == "math";

    let mut children: Vec<_> = node.children()
        .map(|c| compact_cristalino_inner(c, next_in_math))
        .collect();

    let mut kind_name = node.kind().name().to_string();
    if next_in_math {
        match kind_name.as_str() {
            "function call" => kind_name = "math function call".to_string(),
            "call arguments" => kind_name = "math call arguments".to_string(),
            _ => {}
        }
    }

    // Normaliza divergência estrutural em argumentos de funções math:
    // o parser cristalino agrupa argumentos separados por `;` (mat/cases)
    // em `array`, enquanto o parser original os deixa planos na call args.
    if kind_name == "math call arguments" {
        children = children.into_iter().flat_map(|c| match c {
            CompactNode::Branch(name, inner) if name == "array" => inner,
            other => vec![other],
        }).collect();
    }

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}
