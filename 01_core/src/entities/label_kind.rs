//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/label_kind.md
//! @prompt-hash 705dfb37
//! @layer L1
//! @updated 2026-07-23
//!
//! **P856** — classificação de labels existentes mas não referenciáveis
//! (texto, raw, etc.) para emitir mensagens de erro paridade vanilla.

/// Tipo de conteúdo associado a uma label que existe no documento mas
/// não pode ser referenciada por `#ref(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnreferencableKind {
    /// Texto simples, parágrafos, listas, enums, terms, etc.
    Text,
    /// Bloco raw/código.
    Raw,
    /// Equation com numbering desactivado (mensagem específica do vanilla).
    EquationWithoutNumbering,
    /// Fallback para tipos não mapeados explicitamente.
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_sao_distintas() {
        assert_ne!(UnreferencableKind::Text, UnreferencableKind::Raw);
        assert_ne!(UnreferencableKind::Raw, UnreferencableKind::Other);
        assert_ne!(UnreferencableKind::Text, UnreferencableKind::Other);
        assert_ne!(
            UnreferencableKind::EquationWithoutNumbering,
            UnreferencableKind::Text
        );
    }
}
