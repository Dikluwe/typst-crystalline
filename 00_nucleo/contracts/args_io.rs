//! Contratos de Interação Externa (I/O, SO) abstraídos do `args.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais

use std::path::Path;
use std::io::Result;

/// Interface abstrata para saída de dados (Stdout ou File System).
/// Esta `trait` quebra o acoplamento físico existente no enum `Output`/`OpenOutput` legado.
pub trait IOutputWriter {
    /// Escreve em um arquivo do sistema (substitui local do std::fs::write).
    fn write_to_file(&self, path: &Path, content: &[u8]) -> Result<()>;
    
    /// Escreve na saída de terminal padrão (substitui o lock no std::io::stdout).
    fn write_to_stdout(&self, content: &[u8]) -> Result<()>;
}

/// Interface abstrata para leitura de configurações do Sistema Operacional.
/// Destinada a abstrair `std::env` atrelados às flags macros do `clap`.
pub trait IEnvProvider {
    /// Obtém uma variável de ambiente pelo seu nome / chave.
    fn get_var(&self, key: &str) -> Option<String>;
}
