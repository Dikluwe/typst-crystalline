//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/operators.md
//! @prompt-hash 86f5bee7
//! @layer L1
//! @updated 2026-03-23

use crate::entities::syntax_kind::SyntaxKind;

/// Operador unário de Typst.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnOp {
    /// `+` — positivo.
    Pos,
    /// `-` — negação.
    Neg,
    /// `not` — negação booleana.
    Not,
}

impl UnOp {
    /// Converte um token num operador unário, se aplicável.
    pub fn from_kind(token: SyntaxKind) -> Option<Self> {
        Some(match token {
            SyntaxKind::Plus => Self::Pos,
            SyntaxKind::Minus => Self::Neg,
            SyntaxKind::Not => Self::Not,
            // These syntax kinds have no unary operator role.
            SyntaxKind::End
            | SyntaxKind::Error
            | SyntaxKind::Shebang
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment
            | SyntaxKind::Markup
            | SyntaxKind::Text
            | SyntaxKind::Space
            | SyntaxKind::Linebreak
            | SyntaxKind::Parbreak
            | SyntaxKind::Escape
            | SyntaxKind::Shorthand
            | SyntaxKind::SmartQuote
            | SyntaxKind::Strong
            | SyntaxKind::Emph
            | SyntaxKind::Raw
            | SyntaxKind::RawLang
            | SyntaxKind::RawDelim
            | SyntaxKind::RawTrimmed
            | SyntaxKind::Link
            | SyntaxKind::Label
            | SyntaxKind::Ref
            | SyntaxKind::RefMarker
            | SyntaxKind::Heading
            | SyntaxKind::HeadingMarker
            | SyntaxKind::ListItem
            | SyntaxKind::ListMarker
            | SyntaxKind::EnumItem
            | SyntaxKind::EnumMarker
            | SyntaxKind::TermItem
            | SyntaxKind::TermMarker
            | SyntaxKind::Equation
            | SyntaxKind::Math
            | SyntaxKind::MathText
            | SyntaxKind::MathIdent
            | SyntaxKind::MathShorthand
            | SyntaxKind::MathAlignPoint
            | SyntaxKind::MathDelimited
            | SyntaxKind::MathAttach
            | SyntaxKind::MathPrimes
            | SyntaxKind::MathFrac
            | SyntaxKind::MathRoot
            | SyntaxKind::Hash
            | SyntaxKind::LeftBrace
            | SyntaxKind::RightBrace
            | SyntaxKind::LeftBracket
            | SyntaxKind::RightBracket
            | SyntaxKind::LeftParen
            | SyntaxKind::RightParen
            | SyntaxKind::Comma
            | SyntaxKind::Semicolon
            | SyntaxKind::Colon
            | SyntaxKind::Star
            | SyntaxKind::Underscore
            | SyntaxKind::Dollar
            | SyntaxKind::Slash
            | SyntaxKind::Hat
            | SyntaxKind::Dot
            | SyntaxKind::Eq
            | SyntaxKind::EqEq
            | SyntaxKind::ExclEq
            | SyntaxKind::Lt
            | SyntaxKind::LtEq
            | SyntaxKind::Gt
            | SyntaxKind::GtEq
            | SyntaxKind::PlusEq
            | SyntaxKind::HyphEq
            | SyntaxKind::StarEq
            | SyntaxKind::SlashEq
            | SyntaxKind::Dots
            | SyntaxKind::Arrow
            | SyntaxKind::Root
            | SyntaxKind::Bang
            | SyntaxKind::And
            | SyntaxKind::Or
            | SyntaxKind::None
            | SyntaxKind::Auto
            | SyntaxKind::Let
            | SyntaxKind::Set
            | SyntaxKind::Show
            | SyntaxKind::Context
            | SyntaxKind::If
            | SyntaxKind::Else
            | SyntaxKind::For
            | SyntaxKind::In
            | SyntaxKind::While
            | SyntaxKind::Break
            | SyntaxKind::Continue
            | SyntaxKind::Return
            | SyntaxKind::Import
            | SyntaxKind::Include
            | SyntaxKind::As
            | SyntaxKind::Code
            | SyntaxKind::Ident
            | SyntaxKind::Bool
            | SyntaxKind::Int
            | SyntaxKind::Float
            | SyntaxKind::Numeric
            | SyntaxKind::Str
            | SyntaxKind::CodeBlock
            | SyntaxKind::ContentBlock
            | SyntaxKind::Parenthesized
            | SyntaxKind::Array
            | SyntaxKind::Dict
            | SyntaxKind::Named
            | SyntaxKind::Keyed
            | SyntaxKind::Unary
            | SyntaxKind::Binary
            | SyntaxKind::FieldAccess
            | SyntaxKind::FuncCall
            | SyntaxKind::Args
            | SyntaxKind::Spread
            | SyntaxKind::Closure
            | SyntaxKind::Params
            | SyntaxKind::LetBinding
            | SyntaxKind::SetRule
            | SyntaxKind::ShowRule
            | SyntaxKind::Contextual
            | SyntaxKind::Conditional
            | SyntaxKind::WhileLoop
            | SyntaxKind::ForLoop
            | SyntaxKind::ModuleImport
            | SyntaxKind::ImportItems
            | SyntaxKind::ImportItemPath
            | SyntaxKind::RenamedImportItem
            | SyntaxKind::ModuleInclude
            | SyntaxKind::LoopBreak
            | SyntaxKind::LoopContinue
            | SyntaxKind::FuncReturn
            | SyntaxKind::Destructuring
            | SyntaxKind::DestructAssignment => return None,
        })
    }

    /// Precedência deste operador.
    pub fn precedence(self) -> u8 {
        match self {
            Self::Pos | Self::Neg => 7,
            Self::Not => 4,
        }
    }

    /// Representação em string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pos => "+",
            Self::Neg => "-",
            Self::Not => "not",
        }
    }
}

/// Operador binário de Typst.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
    Assign,
    In,
    NotIn,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl BinOp {
    /// Converte um token num operador binário, se aplicável.
    pub fn from_kind(token: SyntaxKind) -> Option<Self> {
        Some(match token {
            SyntaxKind::Plus => Self::Add,
            SyntaxKind::Minus => Self::Sub,
            SyntaxKind::Star => Self::Mul,
            SyntaxKind::Slash => Self::Div,
            SyntaxKind::And => Self::And,
            SyntaxKind::Or => Self::Or,
            SyntaxKind::EqEq => Self::Eq,
            SyntaxKind::ExclEq => Self::Neq,
            SyntaxKind::Lt => Self::Lt,
            SyntaxKind::LtEq => Self::Leq,
            SyntaxKind::Gt => Self::Gt,
            SyntaxKind::GtEq => Self::Geq,
            SyntaxKind::Eq => Self::Assign,
            SyntaxKind::In => Self::In,
            SyntaxKind::PlusEq => Self::AddAssign,
            SyntaxKind::HyphEq => Self::SubAssign,
            SyntaxKind::StarEq => Self::MulAssign,
            SyntaxKind::SlashEq => Self::DivAssign,
            // These syntax kinds have no binary token role; NotIn is composed.
            SyntaxKind::End
            | SyntaxKind::Error
            | SyntaxKind::Shebang
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment
            | SyntaxKind::Markup
            | SyntaxKind::Text
            | SyntaxKind::Space
            | SyntaxKind::Linebreak
            | SyntaxKind::Parbreak
            | SyntaxKind::Escape
            | SyntaxKind::Shorthand
            | SyntaxKind::SmartQuote
            | SyntaxKind::Strong
            | SyntaxKind::Emph
            | SyntaxKind::Raw
            | SyntaxKind::RawLang
            | SyntaxKind::RawDelim
            | SyntaxKind::RawTrimmed
            | SyntaxKind::Link
            | SyntaxKind::Label
            | SyntaxKind::Ref
            | SyntaxKind::RefMarker
            | SyntaxKind::Heading
            | SyntaxKind::HeadingMarker
            | SyntaxKind::ListItem
            | SyntaxKind::ListMarker
            | SyntaxKind::EnumItem
            | SyntaxKind::EnumMarker
            | SyntaxKind::TermItem
            | SyntaxKind::TermMarker
            | SyntaxKind::Equation
            | SyntaxKind::Math
            | SyntaxKind::MathText
            | SyntaxKind::MathIdent
            | SyntaxKind::MathShorthand
            | SyntaxKind::MathAlignPoint
            | SyntaxKind::MathDelimited
            | SyntaxKind::MathAttach
            | SyntaxKind::MathPrimes
            | SyntaxKind::MathFrac
            | SyntaxKind::MathRoot
            | SyntaxKind::Hash
            | SyntaxKind::LeftBrace
            | SyntaxKind::RightBrace
            | SyntaxKind::LeftBracket
            | SyntaxKind::RightBracket
            | SyntaxKind::LeftParen
            | SyntaxKind::RightParen
            | SyntaxKind::Comma
            | SyntaxKind::Semicolon
            | SyntaxKind::Colon
            | SyntaxKind::Underscore
            | SyntaxKind::Dollar
            | SyntaxKind::Hat
            | SyntaxKind::Dot
            | SyntaxKind::Dots
            | SyntaxKind::Arrow
            | SyntaxKind::Root
            | SyntaxKind::Bang
            | SyntaxKind::Not
            | SyntaxKind::None
            | SyntaxKind::Auto
            | SyntaxKind::Let
            | SyntaxKind::Set
            | SyntaxKind::Show
            | SyntaxKind::Context
            | SyntaxKind::If
            | SyntaxKind::Else
            | SyntaxKind::For
            | SyntaxKind::While
            | SyntaxKind::Break
            | SyntaxKind::Continue
            | SyntaxKind::Return
            | SyntaxKind::Import
            | SyntaxKind::Include
            | SyntaxKind::As
            | SyntaxKind::Code
            | SyntaxKind::Ident
            | SyntaxKind::Bool
            | SyntaxKind::Int
            | SyntaxKind::Float
            | SyntaxKind::Numeric
            | SyntaxKind::Str
            | SyntaxKind::CodeBlock
            | SyntaxKind::ContentBlock
            | SyntaxKind::Parenthesized
            | SyntaxKind::Array
            | SyntaxKind::Dict
            | SyntaxKind::Named
            | SyntaxKind::Keyed
            | SyntaxKind::Unary
            | SyntaxKind::Binary
            | SyntaxKind::FieldAccess
            | SyntaxKind::FuncCall
            | SyntaxKind::Args
            | SyntaxKind::Spread
            | SyntaxKind::Closure
            | SyntaxKind::Params
            | SyntaxKind::LetBinding
            | SyntaxKind::SetRule
            | SyntaxKind::ShowRule
            | SyntaxKind::Contextual
            | SyntaxKind::Conditional
            | SyntaxKind::WhileLoop
            | SyntaxKind::ForLoop
            | SyntaxKind::ModuleImport
            | SyntaxKind::ImportItems
            | SyntaxKind::ImportItemPath
            | SyntaxKind::RenamedImportItem
            | SyntaxKind::ModuleInclude
            | SyntaxKind::LoopBreak
            | SyntaxKind::LoopContinue
            | SyntaxKind::FuncReturn
            | SyntaxKind::Destructuring
            | SyntaxKind::DestructAssignment => return None,
        })
    }

    /// Precedência deste operador.
    pub fn precedence(self) -> u8 {
        match self {
            Self::Mul | Self::Div => 6,
            Self::Add | Self::Sub => 5,
            Self::Eq
            | Self::Neq
            | Self::Lt
            | Self::Leq
            | Self::Gt
            | Self::Geq
            | Self::In
            | Self::NotIn => 4,
            Self::And => 3,
            Self::Or => 2,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign => 1,
        }
    }

    /// Associatividade deste operador.
    pub fn assoc(self) -> Assoc {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign => Assoc::Right,
            _ => Assoc::Left,
        }
    }

    /// Representação em string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::And => "and",
            Self::Or => "or",
            Self::Eq => "==",
            Self::Neq => "!=",
            Self::Lt => "<",
            Self::Leq => "<=",
            Self::Gt => ">",
            Self::Geq => ">=",
            Self::In => "in",
            Self::NotIn => "not in",
            Self::Assign => "=",
            Self::AddAssign => "+=",
            Self::SubAssign => "-=",
            Self::MulAssign => "*=",
            Self::DivAssign => "/=",
        }
    }
}

/// Associatividade de um operador binário.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Assoc {
    /// Associativo à esquerda: `a + b + c` ≡ `(a + b) + c`.
    Left,
    /// Associativo à direita: `a = b = c` ≡ `a = (b = c)`.
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unop_from_kind() {
        assert_eq!(UnOp::from_kind(SyntaxKind::Plus), Some(UnOp::Pos));
        assert_eq!(UnOp::from_kind(SyntaxKind::Minus), Some(UnOp::Neg));
        assert_eq!(UnOp::from_kind(SyntaxKind::Not), Some(UnOp::Not));
        assert_eq!(UnOp::from_kind(SyntaxKind::Star), None);
    }

    #[test]
    fn unop_precedencia() {
        assert_eq!(UnOp::Pos.precedence(), 7);
        assert_eq!(UnOp::Not.precedence(), 4);
    }

    #[test]
    fn binop_from_kind() {
        assert_eq!(BinOp::from_kind(SyntaxKind::Plus), Some(BinOp::Add));
        assert_eq!(BinOp::from_kind(SyntaxKind::EqEq), Some(BinOp::Eq));
        assert_eq!(BinOp::from_kind(SyntaxKind::Eq), Some(BinOp::Assign));
        assert_eq!(BinOp::from_kind(SyntaxKind::Not), None);
    }

    #[test]
    fn binop_precedencia_e_assoc() {
        assert!(BinOp::Mul.precedence() > BinOp::Add.precedence());
        assert!(BinOp::Add.precedence() > BinOp::And.precedence());
        assert_eq!(BinOp::Add.assoc(), Assoc::Left);
        assert_eq!(BinOp::Assign.assoc(), Assoc::Right);
    }

    #[test]
    fn not_in_nao_tem_from_kind_directo() {
        // NotIn é construído pelo parser com lógica especial (Not + In)
        assert_eq!(BinOp::NotIn.precedence(), 4);
        assert_eq!(BinOp::NotIn.assoc(), Assoc::Left);
    }

    #[test]
    fn v16_operator_maps_preserve_distinct_type_families() {
        use crate::entities::ast::expr::{BinOp as AstBinOp, UnOp as AstUnOp};
        for (kind, expected) in [
            (SyntaxKind::Plus, UnOp::Pos),
            (SyntaxKind::Minus, UnOp::Neg),
            (SyntaxKind::Not, UnOp::Not),
        ] {
            let op = UnOp::from_kind(kind).expect("admitted unary token");
            assert_eq!(op, expected);
            assert!(crate::entities::syntax_set::UNARY_OP.contains(kind));
            let ast = AstUnOp::from_kind(kind).expect("AST unary map agrees");
            assert_eq!(op.as_str(), ast.as_str());
            assert_eq!(op.precedence(), ast.precedence());
        }
        for (kind, expected) in [
            (SyntaxKind::Plus, BinOp::Add),
            (SyntaxKind::Minus, BinOp::Sub),
            (SyntaxKind::Star, BinOp::Mul),
            (SyntaxKind::Slash, BinOp::Div),
            (SyntaxKind::And, BinOp::And),
            (SyntaxKind::Or, BinOp::Or),
            (SyntaxKind::EqEq, BinOp::Eq),
            (SyntaxKind::ExclEq, BinOp::Neq),
            (SyntaxKind::Lt, BinOp::Lt),
            (SyntaxKind::LtEq, BinOp::Leq),
            (SyntaxKind::Gt, BinOp::Gt),
            (SyntaxKind::GtEq, BinOp::Geq),
            (SyntaxKind::Eq, BinOp::Assign),
            (SyntaxKind::In, BinOp::In),
            (SyntaxKind::PlusEq, BinOp::AddAssign),
            (SyntaxKind::HyphEq, BinOp::SubAssign),
            (SyntaxKind::StarEq, BinOp::MulAssign),
            (SyntaxKind::SlashEq, BinOp::DivAssign),
        ] {
            let op = BinOp::from_kind(kind).expect("admitted binary token");
            assert_eq!(op, expected);
            assert!(crate::entities::syntax_set::BINARY_OP.contains(kind));
            let ast = AstBinOp::from_kind(kind).expect("AST binary map agrees");
            assert_eq!(op.as_str(), ast.as_str());
            assert_eq!(op.precedence(), ast.precedence());
            assert_eq!(
                op.assoc() == Assoc::Right,
                ast.assoc() == crate::entities::ast::expr::Assoc::Right
            );
        }
        for kind in [
            SyntaxKind::End,
            SyntaxKind::Error,
            SyntaxKind::Space,
            SyntaxKind::LineComment,
            SyntaxKind::Ident,
            SyntaxKind::Int,
            SyntaxKind::LeftParen,
        ] {
            assert_eq!(UnOp::from_kind(kind), None, "unary nonoperator {kind:?}");
            assert_eq!(BinOp::from_kind(kind), None, "binary nonoperator {kind:?}");
        }
        assert_eq!(UnOp::from_kind(SyntaxKind::Star), None);
        assert_eq!(BinOp::from_kind(SyntaxKind::Not), None);
        assert_eq!(BinOp::NotIn.as_str(), "not in");
        assert_eq!(BinOp::NotIn.precedence(), 4);
        assert_eq!(BinOp::NotIn.assoc(), Assoc::Left);
    }

    #[test]
    fn v16_operator_assignment_remains_right_associative_in_language() {
        use crate::entities::ast::expr::Binary;
        use crate::entities::ast::AstNode;
        let root = crate::compiler::parse::parse_code("a = b = c");
        assert!(!root.erroneous(), "{:?}", root.errors());
        let outer = root
            .children()
            .find_map(Binary::from_untyped)
            .expect("outer assignment");
        assert_eq!(outer.op().as_str(), "=");
        assert_eq!(outer.lhs().to_untyped().text_str(), "a");
        let inner =
            Binary::from_untyped(outer.rhs().to_untyped()).expect("right assignment");
        assert_eq!(inner.op().as_str(), "=");
        assert_eq!(inner.lhs().to_untyped().text_str(), "b");
        assert_eq!(inner.rhs().to_untyped().text_str(), "c");
    }
}
