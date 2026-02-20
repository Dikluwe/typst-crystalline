//! Contratos de Interação Externa detectados no `watch.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Watch

use std::io::Result;
use std::path::PathBuf;

/// Interface para controle do terminal (limpeza, cores, escrita).
pub trait ITerminalController {
    /// Limpa a tela do terminal.
    fn clear_screen(&mut self) -> Result<()>;

    /// Escreve texto com uma cor/estilo específico.
    fn write_colored(&mut self, text: &str, style: TermStyle) -> Result<()>;

    /// Escreve texto sem formatação.
    fn write_plain(&mut self, text: &str) -> Result<()>;

    /// Escreve uma linha vazia.
    fn write_newline(&mut self) -> Result<()>;

    /// Faz flush do buffer do terminal.
    fn flush(&mut self) -> Result<()>;
}

/// Estilos de terminal suportados pela abstração.
#[derive(Debug, Clone, Copy)]
pub enum TermStyle {
    Error,
    Warning,
    Note,
    Reset,
}

/// Interface para obtenção de timestamps.
pub trait ITimestampProvider {
    /// Retorna o timestamp local formatado como "HH:MM:SS".
    fn now_formatted(&self) -> String;
}

/// Interface para observação de mudanças no filesystem.
pub trait IFileWatcher {
    /// Atualiza a lista de arquivos observados.
    fn update_watched(&mut self, paths: Vec<PathBuf>) -> Result<()>;

    /// Bloqueia até que uma mudança relevante ocorra.
    fn wait_for_change(&mut self) -> Result<()>;
}
