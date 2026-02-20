//! Contratos de Interação Externa detectados no `world.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Sistema

use std::io::Result;
use std::path::{Path, PathBuf};

/// Interface para acesso ao sistema de arquivos e ambiente.
pub trait IFileSystem {
    /// Retorna o diretório de trabalho atual.
    fn get_current_dir(&self) -> Result<PathBuf>;

    /// Canonicaliza um caminho (resolve links e caminhos relativos).
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;

    /// Lê os bytes de um arquivo no disco.
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;
}

/// Interface para leitura de Stdin.
pub trait IStdinReader {
    /// Lê todo o conteúdo disponível no stdin.
    fn read_to_end(&self) -> Result<Vec<u8>>;
}

/// Interface para varredura de fontes.
pub trait IFontScanner {
    /// Descobre fontes nos caminhos fornecidos e no sistema.
    fn scan(&self, font_paths: &[PathBuf]) -> typst_kit::fonts::FontStore;
}

/// Interface paralela para configuração do ambiente de execução.
pub trait IRuntimeConfig {
    /// Configura o número de threads para processamento paralelo.
    fn set_num_threads(&self, num: usize);
}
