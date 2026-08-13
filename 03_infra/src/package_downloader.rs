//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/package_downloader.md
//! @prompt-hash b0ca100b
//! @layer L3
//! @updated 2026-07-15
//!
//! Implementação HTTP do download de pacotes `@preview`, seguindo o
//! comportamento observado no vanilla CLI 0.15.0 (P763):
//!
//! - URL: `https://packages.typst.org/preview/<nome>-<versão>.tar.gz`
//! - Índice: `https://packages.typst.org/preview/index.json`
//! - Download para directório temporário, extracção com tar.gz, rename
//!   atómico para o destino final.
//! - Sem verificação de checksum/assinatura (paridade com o vanilla).

use std::fs;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;
use typst_core::contracts::package_downloader::{
    PackageDownloadError, PackageDownloader,
};
use typst_core::entities::package_spec::{PackageSpec, PackageVersion};

/// URL base do registo oficial Typst Universe.
const DEFAULT_REGISTRY_URL: &str = "https://packages.typst.org";

/// Namespace servido pelo registo oficial.
const PREVIEW_NAMESPACE: &str = "preview";

/// Cliente HTTP síncrono para descarregar pacotes `@preview`.
pub struct HttpPackageDownloader {
    /// URL base do registo (por omissão `https://packages.typst.org`).
    base_url: String,
    /// Directório base onde os pacotes são guardados (tipicamente a cache
    /// dir do utilizador).
    cache_dir: PathBuf,
}

impl HttpPackageDownloader {
    /// Cria um novo downloader para o registo oficial, gravando pacotes em
    /// `cache_dir`.
    pub fn new(cache_dir: PathBuf) -> Self {
        Self::with_url(cache_dir, DEFAULT_REGISTRY_URL)
    }

    /// Cria um novo downloader com URL base configurável (mirror).
    pub fn with_url(cache_dir: PathBuf, base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), cache_dir }
    }

    /// Configura um agente HTTP respeitando `HTTPS_PROXY`/`https_proxy`.
    fn build_agent(&self) -> ureq::Agent {
        let mut builder = ureq::AgentBuilder::new();
        if let Some(proxy_url) = proxy_from_env() {
            if let Ok(proxy) = ureq::Proxy::new(proxy_url) {
                builder = builder.proxy(proxy);
            }
        }
        builder.build()
    }

    /// Caminho final onde uma dada versão de um pacote vive.
    fn package_dir(&self, spec: &PackageSpec) -> PathBuf {
        self.cache_dir
            .join(&spec.namespace)
            .join(&spec.name)
            .join(format!("{}", spec.version))
    }

    /// URL do arquivo `.tar.gz` de uma dada versão.
    fn package_url(&self, spec: &PackageSpec) -> String {
        format!(
            "{}/{}/{}-{}.tar.gz",
            self.base_url, spec.namespace, spec.name, spec.version
        )
    }

    /// URL do índice JSON do namespace.
    fn index_url(&self) -> String {
        format!("{}/{}/index.json", self.base_url, PREVIEW_NAMESPACE)
    }

    /// Devolve a versão mais recente de um pacote no índice remoto, ou
    /// `None` se o pacote não existir ou o índice não for acessível.
    fn latest_version_remote(&self, name: &str) -> Option<PackageVersion> {
        let url = self.index_url();
        let data = match self.build_agent().get(&url).call() {
            Ok(response) => {
                let mut data = Vec::new();
                let mut reader = response.into_reader();
                if reader.read_to_end(&mut data).is_err() {
                    return None;
                }
                data
            }
            Err(_) => return None,
        };

        serde_json::from_slice::<Vec<serde_json::Value>>(&data)
            .ok()?
            .into_iter()
            .filter_map(|value| {
                let obj = value.as_object()?;
                let pkg_name = obj.get("name")?.as_str()?;
                if pkg_name != name {
                    return None;
                }
                let version_str = obj.get("version")?.as_str()?;
                version_str.parse::<PackageVersion>().ok()
            })
            .max()
    }

    /// Cria um nome de directório temporário único dentro de `base_dir`.
    fn make_temp_dir(&self, base_dir: &Path, version: &PackageVersion) -> PathBuf {
        let rand = fastrand::u32(..);
        base_dir.join(format!(".tmp-{version}-{rand:08x}"))
    }
}

impl PackageDownloader for HttpPackageDownloader {
    fn download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError> {
        if spec.namespace != PREVIEW_NAMESPACE {
            return Err(PackageDownloadError::NotFound(spec.clone()));
        }

        let url = self.package_url(spec);
        let package_dir = self.package_dir(spec);

        // Já existe (escrita concorrente terminou primeiro)? Reutiliza.
        if package_dir.is_dir() {
            return Ok(package_dir);
        }

        let base_dir = self.cache_dir.join(&spec.namespace).join(&spec.name);

        fs::create_dir_all(&base_dir).map_err(|e| PackageDownloadError::IoError {
            path: base_dir.clone(),
            cause: e.to_string(),
        })?;

        let temp_dir = self.make_temp_dir(&base_dir, &spec.version);

        // Garante limpeza se algo falhar antes do rename.
        let _guard = TempDirGuard(&temp_dir);

        fs::create_dir_all(&temp_dir).map_err(|e| PackageDownloadError::IoError {
            path: temp_dir.clone(),
            cause: e.to_string(),
        })?;

        let data = match self.build_agent().get(&url).call() {
            Ok(response) => {
                let mut data = Vec::new();
                let mut reader = response.into_reader();
                reader.read_to_end(&mut data).map_err(|e| {
                    PackageDownloadError::NetworkError {
                        url: url.clone(),
                        cause: e.to_string(),
                    }
                })?;
                data
            }
            Err(ureq::Error::Status(404, _)) => {
                // O servidor devolveu 404. Para distinguir "pacote
                // inexistente" de "versão inexistente", consultamos o
                // índice remoto — paridade com a mensagem do vanilla.
                return match self.latest_version_remote(&spec.name) {
                    Some(latest) if latest != spec.version => {
                        Err(PackageDownloadError::VersionNotFound {
                            spec: spec.clone(),
                            latest,
                        })
                    }
                    _ => Err(PackageDownloadError::NotFound(spec.clone())),
                };
            }
            Err(e) => {
                return Err(PackageDownloadError::NetworkError {
                    url: url.clone(),
                    cause: e.to_string(),
                });
            }
        };

        let decompressed = GzDecoder::new(Cursor::new(data));
        let mut archive = Archive::new(decompressed);
        archive.unpack(&temp_dir).map_err(|e| PackageDownloadError::IoError {
            path: temp_dir.clone(),
            cause: e.to_string(),
        })?;

        // Rename atómico. Se outra instância já moveu, o destino não está
        // vazio e o vanilla aceita a versão que lá está.
        match fs::rename(&temp_dir, &package_dir) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::DirectoryNotEmpty => {
                // Outra instância ganhou a corrida; limpa o nosso temp dir.
            }
            Err(e) => {
                return Err(PackageDownloadError::IoError {
                    path: package_dir.clone(),
                    cause: e.to_string(),
                });
            }
        }

        // Guarda é libertado; não tentamos apagar o temp dir porque já foi
        // movido ou tratado acima.
        std::mem::forget(_guard);

        Ok(package_dir)
    }
}

/// Lê a variável de ambiente de proxy HTTPS, preferindo `HTTPS_PROXY` e
/// caindo para `https_proxy`. Não lê outras formas de configuração.
fn proxy_from_env() -> Option<String> {
    std::env::var("HTTPS_PROXY")
        .ok()
        .or_else(|| std::env::var("https_proxy").ok())
}

/// Remove o directório temporário em caso de abandono antes do rename.
struct TempDirGuard<'a>(&'a Path);

impl<'a> Drop for TempDirGuard<'a> {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::package_spec::{PackageSpec, PackageVersion};

    #[test]
    fn package_url_format() {
        let dl = HttpPackageDownloader::new(PathBuf::from("/tmp/cache"));
        let spec = PackageSpec {
            namespace: "preview".into(),
            name: "fletcher".into(),
            version: PackageVersion { major: 0, minor: 5, patch: 4 },
        };
        assert_eq!(
            dl.package_url(&spec),
            "https://packages.typst.org/preview/fletcher-0.5.4.tar.gz"
        );
    }

    #[test]
    fn package_dir_format() {
        let dl = HttpPackageDownloader::new(PathBuf::from("/tmp/cache"));
        let spec = PackageSpec {
            namespace: "preview".into(),
            name: "fletcher".into(),
            version: PackageVersion { major: 0, minor: 5, patch: 4 },
        };
        assert_eq!(
            dl.package_dir(&spec),
            PathBuf::from("/tmp/cache/preview/fletcher/0.5.4")
        );
    }
}
