//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Watch
//! Responsabilidade: Implementar contratos de timestamps e file watching.
//! Nota: O `AnsiTerminalController` não é incluído aqui porque depende de
//! `crate::terminal` do legado. Ele será migrado quando o módulo `terminal.rs`
//! for clivado. Por ora, os contratos `ITerminalController` e `IFileWatcher`
//! ficam disponíveis para implementação futura.

#[path = "../00_nucleo/contracts/watch_io.rs"]
pub mod watch_io;

/// Provedor de timestamps via `chrono::Local`.
pub struct ChronoTimestampProvider;

impl watch_io::ITimestampProvider for ChronoTimestampProvider {
    fn now_formatted(&self) -> String {
        chrono::offset::Local::now().format("%H:%M:%S").to_string()
    }
}
