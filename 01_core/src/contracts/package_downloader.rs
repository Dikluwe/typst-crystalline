//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/contracts/package_downloader.md
//! @prompt-hash 0bb930fe
//! @layer L1
//! @updated 2026-07-15
//!
//! Contrato puro para download de pacotes Typst. A rede e o filesystem
//! ficam em L3; L1 só declara o trait e os tipos de erro de domínio.

use std::fmt;
use std::path::PathBuf;

use crate::entities::package_spec::{PackageSpec, PackageVersion};

/// Capacidade de descarregar um pacote para um directório local.
///
/// O trait é intencionalmente mínimo: recebe um `PackageSpec`, devolve o
/// caminho do directório onde o pacote foi extraído, ou um erro de domínio.
/// Quem implementa este trait é responsável por toda a I/O (HTTP, tar.gz,
/// escrita atómica), mantendo L1 livre de dependências de rede.
pub trait PackageDownloader: Send + Sync {
    /// Descarrega o pacote identificado por `spec` e devolve o caminho da
    /// sua raiz no filesystem local.
    fn download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError>;
}

/// Erro de domínio produzido por um `PackageDownloader`.
///
/// Nenhum tipo de biblioteca HTTP/IO cruza a fronteira; a implementação em
/// L3 converte os seus erros internos para este enum.
#[derive(Debug)]
pub enum PackageDownloadError {
    /// O pacote não existe no registo (404 no nome).
    NotFound(PackageSpec),
    /// O pacote existe, mas a versão pedida não; indica a versão mais
    /// recente conhecida.
    VersionNotFound {
        /// Especificação exacta que falhou.
        spec: PackageSpec,
        /// Versão mais recente disponível no registo.
        latest: PackageVersion,
    },
    /// Falha de rede ou resposta inválida do servidor.
    NetworkError {
        /// URL que falhou.
        url: String,
        /// Causa legível.
        cause: String,
    },
    /// Erro de I/O ao gravar ou mover o pacote localmente.
    IoError {
        /// Caminho onde ocorreu o erro.
        path: PathBuf,
        /// Causa legível.
        cause: String,
    },
}

impl fmt::Display for PackageDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(spec) => {
                write!(f, "package not found (searched for {spec})")
            }
            Self::VersionNotFound { spec, latest } => {
                write!(
                    f,
                    "package found, but version {} does not exist (latest is {latest})",
                    spec.version
                )
            }
            Self::NetworkError { cause, .. } => {
                write!(f, "failed to download package ({cause})")
            }
            Self::IoError { path, cause } => {
                write!(
                    f,
                    "failed to download package (I/O at {}: {cause})",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for PackageDownloadError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::package_spec::{PackageSpec, PackageVersion};

    struct DummyDownloader;

    impl PackageDownloader for DummyDownloader {
        fn download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError> {
            Err(PackageDownloadError::NotFound(spec.clone()))
        }
    }

    #[test]
    fn package_download_error_display_not_found() {
        let spec = PackageSpec {
            namespace: "preview".into(),
            name: "xyz".into(),
            version: PackageVersion { major: 1, minor: 0, patch: 0 },
        };
        let err = PackageDownloadError::NotFound(spec);
        assert!(err.to_string().contains("package not found"));
    }

    #[test]
    fn package_download_error_display_version_not_found() {
        let spec = PackageSpec {
            namespace: "preview".into(),
            name: "fletcher".into(),
            version: PackageVersion { major: 99, minor: 99, patch: 99 },
        };
        let err = PackageDownloadError::VersionNotFound {
            spec,
            latest: PackageVersion { major: 0, minor: 5, patch: 8 },
        };
        let msg = err.to_string();
        assert!(msg.contains("does not exist"));
        assert!(msg.contains("latest is 0.5.8"));
    }
}
