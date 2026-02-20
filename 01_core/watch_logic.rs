//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Watch
//! Responsabilidade: Formatação de mensagens de status, mapeamento de cores e validações de modo watch.

use std::time::Duration;

/// Status de compilação no modo watch.
#[derive(Debug, Clone, PartialEq)]
pub enum WatchStatus {
    Compiling,
    Success(Duration),
    PartialSuccess(Duration),
    Error,
}

/// Estilo de cor associado ao status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusColor {
    Note,
    Warning,
    Error,
}

impl WatchStatus {
    /// Retorna a mensagem de texto para o status atual.
    pub fn message(&self) -> String {
        match self {
            Self::Compiling => "compiling ...".into(),
            Self::Success(duration) => {
                format!("compiled successfully in {}", format_duration(*duration))
            }
            Self::PartialSuccess(duration) => {
                format!("compiled with warnings in {}", format_duration(*duration))
            }
            Self::Error => "compiled with errors".into(),
        }
    }

    /// Retorna a cor associada ao status.
    pub fn color(&self) -> StatusColor {
        match self {
            Self::Error => StatusColor::Error,
            Self::PartialSuccess(_) => StatusColor::Warning,
            _ => StatusColor::Note,
        }
    }
}

/// Formata uma `Duration` de modo legível (equivalente a `typst::utils::format_duration`).
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    if secs > 0 {
        format!("{secs}.{millis:03}s")
    } else {
        format!("{millis}ms")
    }
}

/// Verifica se o modo watch pode operar com o output dado.
/// Watch mode não suporta escrita para stdout.
pub fn is_valid_watch_output(output_is_path: bool) -> bool {
    output_is_path
}

/// Verifica se o input é stdin, indicando necessidade de warning.
pub fn should_warn_stdin(input_is_stdin: bool) -> bool {
    input_is_stdin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiling_message() {
        let status = WatchStatus::Compiling;
        assert_eq!(status.message(), "compiling ...");
        assert_eq!(status.color(), StatusColor::Note);
    }

    #[test]
    fn test_success_message() {
        let status = WatchStatus::Success(Duration::from_millis(350));
        assert_eq!(status.message(), "compiled successfully in 350ms");
        assert_eq!(status.color(), StatusColor::Note);
    }

    #[test]
    fn test_success_message_with_seconds() {
        let status = WatchStatus::Success(Duration::from_millis(2500));
        assert_eq!(status.message(), "compiled successfully in 2.500s");
        assert_eq!(status.color(), StatusColor::Note);
    }

    #[test]
    fn test_partial_success_message() {
        let status = WatchStatus::PartialSuccess(Duration::from_millis(120));
        assert_eq!(status.message(), "compiled with warnings in 120ms");
        assert_eq!(status.color(), StatusColor::Warning);
    }

    #[test]
    fn test_error_message() {
        let status = WatchStatus::Error;
        assert_eq!(status.message(), "compiled with errors");
        assert_eq!(status.color(), StatusColor::Error);
    }

    #[test]
    fn test_valid_watch_output() {
        assert!(is_valid_watch_output(true));
        assert!(!is_valid_watch_output(false));
    }

    #[test]
    fn test_should_warn_stdin() {
        assert!(should_warn_stdin(true));
        assert!(!should_warn_stdin(false));
    }
}
