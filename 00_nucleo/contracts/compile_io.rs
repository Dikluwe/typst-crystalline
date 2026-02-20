//! Contratos de Interação Externa e Infraestrutura detectados no `compile.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Compilação

use std::path::Path;
use std::io::Result;
use chrono::{DateTime, Local};

/// Interface abstrata para interação com processos externos Desktop.
/// Quebra o acoplamento do sistema tipst com a *crate* `open`.
pub trait IProcessOpener {
    /// Abre uma UI (Visualizador de PDF, Browser para HTML, etc.) desconectado do shell atual.
    fn open_detached(&self, target: &str) -> Result<()>;
    
    /// Abre arquivo utilizando programa customizado fornecido pela string viewer.
    fn open_with_viewer(&self, target: &str, viewer: &str) -> Result<()>;
}

/// Interface de cronologia para abstrair o Relógio Físico do Sistema (System Clock).
/// Essencial para poder mockar instantes na injeção de L1.
pub trait IClockProvider {
    /// Fornece a Data e Hora local do sistema.
    fn now_local(&self) -> DateTime<Local>;
}

/// Um publicador de diagnósticos (Printer do Terminal Colorido).
/// Centraliza as chamadas sujas para `codespan_reporting` ou stderr nativo.
pub trait IDiagnosticPublisher {
    fn print_success(&self, duration_ms: u64);
    fn print_error(&self, error_message: &str);
}
