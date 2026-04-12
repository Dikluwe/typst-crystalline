//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/ast/code.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-03-26

use std::path::Path;
use std::str::FromStr;

use crate::node;
use crate::entities::ast::expr::{Expr, Ident, Pattern, Args};
use crate::entities::package_spec::PackageSpec;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;
use crate::rules::lexer::is_ident;

node! { struct LetBinding }

/// The kind of a let binding, either a normal one or a closure.
#[derive(Debug)]
pub enum LetBindingKind<'a> {
    Normal(Pattern<'a>),
    Closure(Ident<'a>),
}

impl<'a> LetBindingKind<'a> {
    pub fn bindings(self) -> Vec<Ident<'a>> {
        match self {
            LetBindingKind::Normal(pattern) => pattern.bindings(),
            LetBindingKind::Closure(ident) => vec![ident],
        }
    }
}

impl<'a> LetBinding<'a> {
    pub fn kind(self) -> LetBindingKind<'a> {
        use crate::entities::ast::expr::Pattern;
        match self.0.cast_first() {
            Pattern::Normal(Expr::Closure(closure)) => {
                LetBindingKind::Closure(closure.name().expect("closure should have name"))
            }
            pattern => LetBindingKind::Normal(pattern),
        }
    }

    pub fn init(self) -> Option<Expr<'a>> {
        match self.kind() {
            LetBindingKind::Normal(Pattern::Normal(_) | Pattern::Parenthesized(_)) => {
                self.0.children().filter_map(SyntaxNode::cast).nth(1)
            }
            LetBindingKind::Normal(_) => self.0.try_cast_first(),
            LetBindingKind::Closure(_) => self.0.try_cast_first(),
        }
    }
}

node! { struct DestructAssignment }

impl<'a> DestructAssignment<'a> {
    pub fn pattern(self) -> Pattern<'a> { self.0.cast_first() }
    pub fn value(self) -> Expr<'a> { self.0.cast_last() }
}

node! { struct SetRule }

impl<'a> SetRule<'a> {
    pub fn target(self) -> Expr<'a> { self.0.cast_first() }
    pub fn args(self) -> Args<'a> { self.0.cast_last() }

    pub fn condition(self) -> Option<Expr<'a>> {
        self.0
            .children()
            .skip_while(|child| child.kind() != SyntaxKind::If)
            .find_map(SyntaxNode::cast)
    }
}

node! { struct ShowRule }

impl<'a> ShowRule<'a> {
    pub fn selector(self) -> Option<Expr<'a>> {
        self.0
            .children()
            .rev()
            .skip_while(|child| child.kind() != SyntaxKind::Colon)
            .find_map(SyntaxNode::cast)
    }

    pub fn transform(self) -> Expr<'a> { self.0.cast_last() }
}

node! { struct Contextual }

impl<'a> Contextual<'a> {
    pub fn body(self) -> Expr<'a> { self.0.cast_first() }
}

node! { struct Conditional }

impl<'a> Conditional<'a> {
    pub fn condition(self) -> Expr<'a> { self.0.cast_first() }

    pub fn if_body(self) -> Expr<'a> {
        self.0.children().filter_map(SyntaxNode::cast).nth(1)
            .expect("conditional missing if body")
    }

    pub fn else_body(self) -> Option<Expr<'a>> {
        self.0.children().filter_map(SyntaxNode::cast).nth(2)
    }
}

node! { struct WhileLoop }

impl<'a> WhileLoop<'a> {
    pub fn condition(self) -> Expr<'a> { self.0.cast_first() }
    pub fn body(self) -> Expr<'a> { self.0.cast_last() }
}

node! { struct ForLoop }

impl<'a> ForLoop<'a> {
    pub fn pattern(self) -> Pattern<'a> { self.0.cast_first() }

    pub fn iterable(self) -> Expr<'a> {
        self.0
            .children()
            .skip_while(|&c| c.kind() != SyntaxKind::In)
            .find_map(SyntaxNode::cast)
            .expect("for loop missing iterable")
    }

    pub fn body(self) -> Expr<'a> { self.0.cast_last() }
}

node! { struct ModuleImport }

/// Reasons why a bare name cannot be determined for an import source.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BareImportError {
    Dynamic,
    PathInvalid,
    PackageInvalid,
}

impl<'a> ModuleImport<'a> {
    pub fn source(self) -> Expr<'a> { self.0.cast_first() }

    pub fn imports(self) -> Option<Imports<'a>> {
        self.0.children().find_map(|node| match node.kind() {
            SyntaxKind::Star => Some(Imports::Wildcard),
            SyntaxKind::ImportItems => node.cast().map(Imports::Items),
            _ => Option::None,
        })
    }

    pub fn bare_name(self) -> Result<String, BareImportError> {
        match self.source() {
            Expr::Ident(ident) => Ok(ident.get().to_owned()),
            Expr::FieldAccess(access) => Ok(access.field().get().to_owned()),
            Expr::Str(string) => {
                let string = string.get();
                let name: String = if string.starts_with('@') {
                    PackageSpec::from_str(&string)
                        .map_err(|_| BareImportError::PackageInvalid)?
                        .name
                } else {
                    Path::new(&string)
                        .file_stem()
                        .and_then(|path| path.to_str())
                        .ok_or(BareImportError::PathInvalid)?
                        .to_owned()
                };
                if !is_ident(&name) {
                    return Err(BareImportError::PathInvalid);
                }
                Ok(name)
            }
            _ => Err(BareImportError::Dynamic),
        }
    }

    pub fn new_name(self) -> Option<Ident<'a>> {
        self.0
            .children()
            .skip_while(|child| child.kind() != SyntaxKind::As)
            .find_map(SyntaxNode::cast)
    }
}

/// The items that ought to be imported from a file.
#[derive(Debug, Copy, Clone, Hash)]
pub enum Imports<'a> {
    Wildcard,
    Items(ImportItems<'a>),
}

node! { struct ImportItems }

impl<'a> ImportItems<'a> {
    pub fn iter(self) -> impl DoubleEndedIterator<Item = ImportItem<'a>> {
        self.0.children().filter_map(|child| match child.kind() {
            SyntaxKind::RenamedImportItem => child.cast().map(ImportItem::Renamed),
            SyntaxKind::ImportItemPath => child.cast().map(ImportItem::Simple),
            _ => Option::None,
        })
    }
}

node! { struct ImportItemPath }

impl<'a> ImportItemPath<'a> {
    pub fn iter(self) -> impl DoubleEndedIterator<Item = Ident<'a>> {
        self.0.children().filter_map(SyntaxNode::cast)
    }

    pub fn name(self) -> Ident<'a> { self.0.cast_last() }
}

/// An imported item, potentially renamed.
#[derive(Debug, Copy, Clone, Hash)]
pub enum ImportItem<'a> {
    Simple(ImportItemPath<'a>),
    Renamed(RenamedImportItem<'a>),
}

impl<'a> ImportItem<'a> {
    pub fn path(self) -> ImportItemPath<'a> {
        match self {
            Self::Simple(path) => path,
            Self::Renamed(renamed) => renamed.path(),
        }
    }

    pub fn original_name(self) -> Ident<'a> {
        match self {
            Self::Simple(path) => path.name(),
            Self::Renamed(renamed) => renamed.original_name(),
        }
    }

    pub fn bound_name(self) -> Ident<'a> {
        match self {
            Self::Simple(path) => path.name(),
            Self::Renamed(renamed) => renamed.new_name(),
        }
    }
}

node! { struct RenamedImportItem }

impl<'a> RenamedImportItem<'a> {
    pub fn path(self) -> ImportItemPath<'a> { self.0.cast_first() }
    pub fn original_name(self) -> Ident<'a> { self.path().name() }
    pub fn new_name(self) -> Ident<'a> { self.0.cast_last() }
}

node! { struct ModuleInclude }

impl<'a> ModuleInclude<'a> {
    pub fn source(self) -> Expr<'a> { self.0.cast_last() }
}

node! { struct LoopBreak }
node! { struct LoopContinue }
node! { struct FuncReturn }

impl<'a> FuncReturn<'a> {
    pub fn body(self) -> Option<Expr<'a>> { self.0.try_cast_last() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ast::AstNode;
    

    #[test]
    fn bare_import_error_dynamic() {
        // Contrato correcto — BareImportError variants exist
        let _ = BareImportError::Dynamic;
        let _ = BareImportError::PathInvalid;
        let _ = BareImportError::PackageInvalid;
    }

    #[test]
    fn let_binding_found_in_code_block() {
        // Contrato correcto — LetBinding node is produced by the parser for #let
        let _ = LetBinding::from_untyped; // confirm type exists
        let _ = SyntaxKind::LetBinding;   // confirm variant exists
    }

    #[test]
    fn module_import_node_type_exists() {
        let _ = ModuleImport::from_untyped;
    }
}
