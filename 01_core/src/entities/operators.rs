//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 49b90d8f
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
            SyntaxKind::Plus  => Self::Pos,
            SyntaxKind::Minus => Self::Neg,
            SyntaxKind::Not   => Self::Not,
            _ => return None,
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
    Add, Sub, Mul, Div,
    And, Or,
    Eq, Neq, Lt, Leq, Gt, Geq,
    Assign, In, NotIn,
    AddAssign, SubAssign, MulAssign, DivAssign,
}

impl BinOp {
    /// Converte um token num operador binário, se aplicável.
    pub fn from_kind(token: SyntaxKind) -> Option<Self> {
        Some(match token {
            SyntaxKind::Plus    => Self::Add,
            SyntaxKind::Minus   => Self::Sub,
            SyntaxKind::Star    => Self::Mul,
            SyntaxKind::Slash   => Self::Div,
            SyntaxKind::And     => Self::And,
            SyntaxKind::Or      => Self::Or,
            SyntaxKind::EqEq    => Self::Eq,
            SyntaxKind::ExclEq  => Self::Neq,
            SyntaxKind::Lt      => Self::Lt,
            SyntaxKind::LtEq    => Self::Leq,
            SyntaxKind::Gt      => Self::Gt,
            SyntaxKind::GtEq    => Self::Geq,
            SyntaxKind::Eq      => Self::Assign,
            SyntaxKind::In      => Self::In,
            SyntaxKind::PlusEq  => Self::AddAssign,
            SyntaxKind::HyphEq  => Self::SubAssign,
            SyntaxKind::StarEq  => Self::MulAssign,
            SyntaxKind::SlashEq => Self::DivAssign,
            _ => return None,
        })
    }

    /// Precedência deste operador.
    pub fn precedence(self) -> u8 {
        match self {
            Self::Mul | Self::Div => 6,
            Self::Add | Self::Sub => 5,
            Self::Eq | Self::Neq | Self::Lt | Self::Leq
            | Self::Gt | Self::Geq | Self::In | Self::NotIn => 4,
            Self::And => 3,
            Self::Or  => 2,
            Self::Assign | Self::AddAssign | Self::SubAssign
            | Self::MulAssign | Self::DivAssign => 1,
        }
    }

    /// Associatividade deste operador.
    pub fn assoc(self) -> Assoc {
        match self {
            Self::Assign | Self::AddAssign | Self::SubAssign
            | Self::MulAssign | Self::DivAssign => Assoc::Right,
            _ => Assoc::Left,
        }
    }

    /// Representação em string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Add       => "+",
            Self::Sub       => "-",
            Self::Mul       => "*",
            Self::Div       => "/",
            Self::And       => "and",
            Self::Or        => "or",
            Self::Eq        => "==",
            Self::Neq       => "!=",
            Self::Lt        => "<",
            Self::Leq       => "<=",
            Self::Gt        => ">",
            Self::Geq       => ">=",
            Self::In        => "in",
            Self::NotIn     => "not in",
            Self::Assign    => "=",
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
        assert_eq!(UnOp::from_kind(SyntaxKind::Plus),  Some(UnOp::Pos));
        assert_eq!(UnOp::from_kind(SyntaxKind::Minus), Some(UnOp::Neg));
        assert_eq!(UnOp::from_kind(SyntaxKind::Not),   Some(UnOp::Not));
        assert_eq!(UnOp::from_kind(SyntaxKind::Star),  None);
    }

    #[test]
    fn unop_precedencia() {
        assert_eq!(UnOp::Pos.precedence(), 7);
        assert_eq!(UnOp::Not.precedence(), 4);
    }

    #[test]
    fn binop_from_kind() {
        assert_eq!(BinOp::from_kind(SyntaxKind::Plus),  Some(BinOp::Add));
        assert_eq!(BinOp::from_kind(SyntaxKind::EqEq),  Some(BinOp::Eq));
        assert_eq!(BinOp::from_kind(SyntaxKind::Eq),    Some(BinOp::Assign));
        assert_eq!(BinOp::from_kind(SyntaxKind::Not),   None);
    }

    #[test]
    fn binop_precedencia_e_assoc() {
        assert!(BinOp::Mul.precedence() > BinOp::Add.precedence());
        assert!(BinOp::Add.precedence() > BinOp::And.precedence());
        assert_eq!(BinOp::Add.assoc(),    Assoc::Left);
        assert_eq!(BinOp::Assign.assoc(), Assoc::Right);
    }

    #[test]
    fn not_in_nao_tem_from_kind_directo() {
        // NotIn é construído pelo parser com lógica especial (Not + In)
        assert_eq!(BinOp::NotIn.precedence(), 4);
        assert_eq!(BinOp::NotIn.assoc(), Assoc::Left);
    }
}
