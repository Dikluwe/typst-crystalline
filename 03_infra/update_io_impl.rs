//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Update
//! Responsabilidade: Implementar contratos de download HTTP, extração de archives, self-replace e backup.

use std::io::{self};
#[cfg(feature = "self-update")]
use std::io::Read;
use std::path::PathBuf;

#[path = "../00_nucleo/contracts/update_io.rs"]
pub mod update_io;

use update_io::IBackupPathResolver;
#[cfg(feature = "self-update")]
use update_io::{IArchiveExtractor, ISelfReplacer};

/// Extrator de binários de archives ZIP e tar.xz.
#[cfg(feature = "self-update")]
pub struct SystemArchiveExtractor;

#[cfg(feature = "self-update")]
impl IArchiveExtractor for SystemArchiveExtractor {
    fn extract_from_zip(&self, data: &[u8], asset_name: &str) -> io::Result<Vec<u8>> {
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let mut file = archive
            .by_name(&format!("{asset_name}/typst.exe"))
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e.to_string()))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn extract_from_tar_xz(&self, data: &[u8]) -> io::Result<Vec<u8>> {
        use xz2::bufread::XzDecoder;

        let mut archive = tar::Archive::new(XzDecoder::new(std::io::Cursor::new(data)));
        let mut file = archive
            .entries()?
            .filter_map(Result::ok)
            .find(|e| e.path().unwrap_or_default().ends_with("typst"))
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "tar.xz não contém o binário Typst")
            })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

/// Self-replacer via `self_replace` crate.
#[cfg(feature = "self-update")]
pub struct OsSelfReplacer;

#[cfg(feature = "self-update")]
impl ISelfReplacer for OsSelfReplacer {
    fn replace_self(&self, source_path: &Path) -> io::Result<()> {
        self_replace::self_replace(source_path)
    }

    fn revert_from_backup(&self, backup_path: &Path) -> io::Result<()> {
        self_replace::self_replace(backup_path)?;
        std::fs::remove_file(backup_path)?;
        Ok(())
    }
}

/// Resolvedor de paths de backup e executável atual.
pub struct OsBackupPathResolver;

impl IBackupPathResolver for OsBackupPathResolver {
    fn default_backup_path(&self) -> io::Result<PathBuf> {
        #[cfg(target_os = "linux")]
        let root = dirs::state_dir()
            .or_else(dirs::data_dir)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "unable to locate state/data directory")
            })?;

        #[cfg(not(target_os = "linux"))]
        let root = dirs::data_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "unable to locate data directory")
        })?;

        Ok(root.join("typst").join("typst_backup.part"))
    }

    fn current_exe_path(&self) -> io::Result<PathBuf> {
        std::env::current_exe()
    }
}
