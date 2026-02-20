//! Contratos de Interação Externa detectados no `update.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Atualização

use std::io::Result;
use std::path::{Path, PathBuf};

/// Interface para download de recursos HTTP.
pub trait IHttpDownloader {
    /// Faz o download de um recurso a partir de uma URL.
    /// Retorna os bytes crus ou um erro de I/O.
    fn download(&self, label: &str, url: &str) -> Result<Vec<u8>>;
}

/// Interface para extração de binários de archives compactados.
pub trait IArchiveExtractor {
    /// Extrai o binário de um archive ZIP (Windows).
    fn extract_from_zip(&self, data: &[u8], asset_name: &str) -> Result<Vec<u8>>;

    /// Extrai o binário de um archive tar.xz (Linux/macOS).
    fn extract_from_tar_xz(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// Interface para operações de self-replace do binário.
pub trait ISelfReplacer {
    /// Substitui o executável atual pelo binário em `source_path`.
    fn replace_self(&self, source_path: &Path) -> Result<()>;

    /// Reverte para a versão de backup.
    fn revert_from_backup(&self, backup_path: &Path) -> Result<()>;
}

/// Interface para resolução de paths de backup no sistema.
pub trait IBackupPathResolver {
    /// Retorna o caminho padrão de backup do binário.
    fn default_backup_path(&self) -> Result<PathBuf>;

    /// Retorna o caminho do executável atual.
    fn current_exe_path(&self) -> Result<PathBuf>;
}
