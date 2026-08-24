//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/package_downloader.md
//! @prompt-hash 1a4a4691
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
use std::sync::Arc;

use flate2::read::GzDecoder;
use rustls_pki_types::{pem::PemObject, CertificateDer};
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
    /// Path da CA customizada. Os bytes só são lidos ao construir o agente.
    custom_ca_path: Option<PathBuf>,
}

impl HttpPackageDownloader {
    /// Cria um novo downloader para o registo oficial, gravando pacotes em
    /// `cache_dir`.
    pub fn new(cache_dir: PathBuf) -> Self {
        Self::with_url(cache_dir, DEFAULT_REGISTRY_URL)
    }

    /// Cria um novo downloader com URL base configurável (mirror).
    pub fn with_url(cache_dir: PathBuf, base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            cache_dir,
            custom_ca_path: None,
        }
    }

    /// Acrescenta uma ou mais CAs PEM às roots normais.
    pub fn with_custom_ca(mut self, path: Option<PathBuf>) -> Self {
        self.custom_ca_path = path;
        self
    }

    /// Configura um agente HTTP respeitando `HTTPS_PROXY`/`https_proxy`.
    fn build_agent(&self) -> Result<ureq::Agent, String> {
        self.build_agent_with_proxy(true)
    }

    fn build_agent_with_proxy(&self, use_proxy: bool) -> Result<ureq::Agent, String> {
        let mut builder = ureq::AgentBuilder::new();
        if use_proxy {
            if let Some(proxy_url) = proxy_from_env() {
                if let Ok(proxy) = ureq::Proxy::new(proxy_url) {
                    builder = builder.proxy(proxy);
                }
            }
        }
        if let Some(path) = &self.custom_ca_path {
            let pem = fs::read(path)
                .map_err(|_| "failed to read custom CA certificate".to_string())?;
            if pem.is_empty() {
                return Err("custom CA certificate is empty".to_string());
            }
            let certificates = CertificateDer::pem_slice_iter(&pem)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "custom CA certificate contains invalid PEM".to_string())?;
            if certificates.is_empty() {
                return Err("custom CA certificate contains no certificates".to_string());
            }
            let mut roots =
                rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
            for certificate in certificates {
                roots
                    .add(certificate)
                    .map_err(|_| "custom CA certificate is invalid".to_string())?;
            }
            let tls = rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            builder = builder.tls_config(Arc::new(tls));
        }
        Ok(builder.build())
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
    pub fn latest_version(&self, name: &str) -> Result<Option<PackageVersion>, String> {
        let url = self.index_url();
        let data = match self.build_agent()?.get(&url).call() {
            Ok(response) => {
                let mut data = Vec::new();
                let mut reader = response.into_reader();
                if reader.read_to_end(&mut data).is_err() {
                    return Err("failed to read package index".into());
                }
                data
            }
            Err(_) => return Err("failed to download package index".into()),
        };

        let latest = serde_json::from_slice::<Vec<serde_json::Value>>(&data)
            .map_err(|_| "package index is invalid".to_string())?
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
            .max();
        Ok(latest)
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

        let agent =
            self.build_agent()
                .map_err(|cause| PackageDownloadError::NetworkError {
                    url: url.clone(),
                    cause,
                })?;
        let data = match agent.get(&url).call() {
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
                return match self.latest_version(&spec.name).ok().flatten() {
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
    use rustls_pki_types::{pem::PemObject, PrivateKeyDer};
    use std::io::Write;
    use std::net::{SocketAddr, TcpListener};
    use std::thread;
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

    #[test]
    fn custom_ca_path_inexistente_falha_sem_expor_path() {
        let secret_path = PathBuf::from("/tmp/SEGREDO-cert-inexistente.pem");
        let dl = HttpPackageDownloader::new(PathBuf::from("/tmp/cache"))
            .with_custom_ca(Some(secret_path));
        let error = dl.build_agent().unwrap_err();
        assert!(error.contains("failed to read custom CA certificate"));
        assert!(!error.contains("SEGREDO"));
    }

    #[test]
    fn custom_ca_vazia_e_pem_invalido_falham_antes_da_request() {
        let dir = std::env::temp_dir();
        let empty = dir.join(format!("typst-cert-empty-{}.pem", std::process::id()));
        let invalid = dir.join(format!("typst-cert-invalid-{}.pem", std::process::id()));
        fs::write(&empty, []).unwrap();
        fs::write(&invalid, b"not a certificate").unwrap();

        let empty_error = HttpPackageDownloader::new(dir.clone())
            .with_custom_ca(Some(empty.clone()))
            .build_agent()
            .unwrap_err();
        let invalid_error = HttpPackageDownloader::new(dir.clone())
            .with_custom_ca(Some(invalid.clone()))
            .build_agent()
            .unwrap_err();
        assert!(empty_error.contains("empty"));
        assert!(
            invalid_error.contains("no certificates")
                || invalid_error.contains("invalid PEM")
        );

        let _ = fs::remove_file(empty);
        let _ = fs::remove_file(invalid);
    }

    fn tls_server() -> std::io::Result<(SocketAddr, thread::JoinHandle<()>)> {
        let certs = CertificateDer::pem_slice_iter(include_bytes!(
            "../tests/fixtures/p1137-server.pem"
        ))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
        let key = PrivateKeyDer::from_pem_slice(include_bytes!(
            "../tests/fixtures/p1137-server-key.pem"
        ))
        .unwrap();
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let task = thread::spawn(move || {
            let Ok((socket, _)) = listener.accept() else { return };
            let Ok(connection) = rustls::ServerConnection::new(Arc::new(config)) else {
                return;
            };
            let mut tls = rustls::StreamOwned::new(connection, socket);
            let mut request = [0_u8; 1024];
            if tls.read(&mut request).is_ok() {
                let _ = tls.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok");
            }
        });
        Ok((address, task))
    }

    #[test]
    fn custom_ca_autoriza_cadeia_local_mas_nao_hostname_incorreto() {
        let ca =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/p1137-ca.pem");
        let downloader = HttpPackageDownloader::new(PathBuf::from("/tmp/cache"))
            .with_custom_ca(Some(ca));

        let (address, task) = match tls_server() {
            Ok(server) => server,
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "sonda TLS local não executada: runner proíbe bind em loopback"
                );
                return;
            }
            Err(error) => panic!("falha ao criar servidor TLS local: {error}"),
        };
        let response = downloader
            .build_agent_with_proxy(false)
            .unwrap()
            .get(&format!("https://localhost:{}/", address.port()))
            .call();
        assert!(
            response.is_ok(),
            "CA customizada deve autorizar localhost: {response:?}"
        );
        task.join().unwrap();

        let (address, task) = match tls_server() {
            Ok(server) => server,
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "sonda TLS local não executada: runner proíbe bind em loopback"
                );
                return;
            }
            Err(error) => panic!("falha ao criar servidor TLS local: {error}"),
        };
        let response = downloader
            .build_agent_with_proxy(false)
            .unwrap()
            .get(&format!("https://127.0.0.1:{}/", address.port()))
            .call();
        assert!(response.is_err(), "hostname incorreto não pode ser aceito");
        task.join().unwrap();
    }
}
