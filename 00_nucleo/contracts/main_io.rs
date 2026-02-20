//! Contratos de Interação Externa detectados no `main.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais do Entry Point

use std::io::Result;

/// Interface para impressão formatada de erros e dicas.
pub trait IAppErrorPrinter {
    /// Imprime uma mensagem de erro no terminal com formatação adequada.
    fn print_error(&mut self, msg: &str) -> Result<()>;

    /// Imprime uma dica associada a um erro.
    fn print_hint(&mut self, msg: &str) -> Result<()>;
}

/// Interface para controle do Exit Code do processo.
pub trait IExitCodeController {
    /// Sinaliza que o processo falhou.
    fn set_failed(&self);

    /// Retorna o exit code corrente.
    fn get_exit_code(&self) -> std::process::ExitCode;
}
