// 💎 Tipologia Tekt: L1 (Lógica Pura - Atomizada)
// Módulo de Origem: typst-ide/src/analyze.rs
// ZERO I/O. ZERO dependências de World/Engine/IdeWorld.

use ecow::{EcoString, EcoVec, eco_vec};
use rustc_hash::FxHashSet;
use typst::AsDocument;
use typst::foundations::{Label, Scope, Styles, Value};
use typst::model::{BibliographyElem, FigureElem};
use typst::syntax::{LinkedNode, SyntaxKind, ast};

/// Resultado da tentativa de análise pura de uma expressão AST.
/// Se o nó foi resolvido, contém o Value. Se requer recursão, indica o próximo nó.
/// Se requer fallback impuro (tracing), sinaliza `NeedsTracing`.
pub enum ExprAnalysis<'a> {
    /// Valor literal resolvido puramente.
    Resolved(EcoVec<(Value, Option<Styles>)>),
    /// O nó requer recursão para outro LinkedNode (Down em Contextual ou Up em FieldAccess).
    Recurse(LinkedNode<'a>),
    /// Nenhum literal reconhecido. O L2 deve acionar o tracing impuro.
    NeedsTracing,
    /// Cast AST inválido (nó não é uma `Expr`). Retornar vazio.
    NotAnExpr,
}

/// Tenta resolver a expressão abstrata num literal Typst (`Value`) puro sem efetuar
/// chamadas à Engine de Tracing pesado. Funciona como primeira barreira L1.
pub fn analyze_basic_expr<'a>(node: &LinkedNode<'a>) -> ExprAnalysis<'a> {
    let Some(expr) = node.cast::<ast::Expr>() else {
        return ExprAnalysis::NotAnExpr;
    };

    let result = match expr {
        ast::Expr::None(_) => Value::None,
        ast::Expr::Auto(_) => Value::Auto,
        ast::Expr::Bool(v) => Value::Bool(v.get()),
        ast::Expr::Int(v) => Value::Int(v.get()),
        ast::Expr::Float(v) => Value::Float(v.get()),
        ast::Expr::Numeric(v) => Value::numeric(v.get()),
        ast::Expr::Str(v) => Value::Str(v.get().into()),
        _ => {
            // Regra pura de navegação Down: Contextual -> último filho
            if node.kind() == SyntaxKind::Contextual {
                if let Some(child) = node.children().next_back() {
                    return ExprAnalysis::Recurse(child);
                }
            }

            // Regra pura de navegação Up: filho de FieldAccess -> parent
            if let Some(parent) = node.parent() {
                if parent.kind() == SyntaxKind::FieldAccess && node.index() > 0 {
                    return ExprAnalysis::Recurse(parent.clone());
                }
            }

            return ExprAnalysis::NeedsTracing;
        }
    };

    ExprAnalysis::Resolved(eco_vec![(result, None)])
}

/// Busca de escopo best-effort em dead code. Recebe o Scope de globals
/// já resolvido externamente (injetado) e tenta resolver Ident ou FieldAccess.
pub fn analyze_scope_fallback(node: &LinkedNode, globals: &Scope) -> Option<Value> {
    let expr = node.cast::<ast::Expr>()?;

    let value = match expr {
        ast::Expr::Ident(ident) => globals.get(&ident)?.read(),
        ast::Expr::FieldAccess(access) => match access.target() {
            ast::Expr::Ident(target) => {
                globals.get(&target)?.read().scope()?.get(&access.field())?.read()
            }
            _ => return None,
        },
        _ => return None,
    };

    Some(value.clone())
}

/// Verifica se o valor do `analyze_expr` pode ser diretamente usado como fonte de import
/// (se já possui escopo próprio, é um módulo pré-carregado). Retorna a string de caminho caso contrário.
pub fn classify_import_source(source: Value) -> ImportSourceKind {
    if source.scope().is_some() {
        return ImportSourceKind::PreloadedModule(source);
    }
    match source {
        Value::Str(path) => ImportSourceKind::Path(path.into()),
        _ => ImportSourceKind::Invalid,
    }
}

/// Resultado da classificação da fonte de import.
pub enum ImportSourceKind {
    PreloadedModule(Value),
    Path(String),
    Invalid,
}

/// Mecanismo reducionista puro de raspagem de referências. Transforma o output
/// da introspecção do documento em representações legíveis e deduplicadas.
pub fn extract_document_labels(document: impl AsDocument) -> (Vec<(Label, Option<EcoString>)>, usize) {
    let introspector = document.as_document().introspector();

    let mut output = vec![];
    let mut seen_labels = FxHashSet::default();

    for elem in introspector.all() {
        let Some(label) = elem.label() else { continue };
        if !seen_labels.insert(label) {
            continue;
        }

        let details = elem
            .to_packed::<FigureElem>()
            .and_then(|figure| match figure.caption.as_option() {
                Some(Some(caption)) => Some(caption.pack_ref()),
                _ => None,
            })
            .unwrap_or(elem)
            .get_by_name("body")
            .ok()
            .and_then(|field| match field {
                Value::Content(content) => Some(content),
                _ => None,
            })
            .as_ref()
            .unwrap_or(elem)
            .plain_text();

        output.push((label, Some(details)));
    }

    let split = output.len();

    // Bibliografia
    output.extend(BibliographyElem::keys(comemo::Track::track(introspector)));

    (output, split)
}
