//! Contratos de Interação Externa detectados no `deps.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Deps

use std::io::Result;
use std::path::PathBuf;

/// Interface para escrita de dependências em um destino.
pub trait IDepsWriter {
    /// Escreve bytes no destino (arquivo ou stdout).
    fn write_all(&mut self, data: &[u8]) -> Result<()>;

    /// Faz flush do buffer de saída.
    fn flush(&mut self) -> Result<()>;
}

/// Interface para obtenção do diretório de trabalho atual.
pub trait IWorkingDirectory {
    /// Retorna o diretório de trabalho atual.
    fn current_dir(&self) -> Result<PathBuf>;
}
