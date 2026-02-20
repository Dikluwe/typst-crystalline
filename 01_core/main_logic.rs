//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Main
//! Responsabilidade: Lógica de domínio pura do entry point — mensagens de fallback e stub de update.

use typst::diag::{StrResult, bail};

/// Mensagem de erro padrão para quando o self-update não está disponível.
pub const UPDATE_UNAVAILABLE_MSG: &str =
    "self-updating is not enabled for this executable, \
     please update with the package manager or mechanism \
     used for initial installation";

/// Retorna um erro de domínio indicando que o self-update não está habilitado.
/// Equivalente ao stub `mod update` compilado quando `cfg(not(feature = "self-update"))`.
pub fn update_unavailable() -> StrResult<()> {
    bail!("{}", UPDATE_UNAVAILABLE_MSG)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_unavailable_returns_error() {
        let result = update_unavailable();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("self-updating is not enabled"));
    }

    #[test]
    fn test_update_unavailable_msg_constant() {
        assert!(UPDATE_UNAVAILABLE_MSG.contains("self-updating"));
        assert!(UPDATE_UNAVAILABLE_MSG.contains("package manager"));
    }
}
