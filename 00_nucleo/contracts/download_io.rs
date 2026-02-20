//! Contratos de Interação Externa detectados no `download.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Download

use std::io::Result;
use std::path::PathBuf;

/// Interface para criação de downloaders HTTP.
pub trait IDownloaderFactory {
    /// Cria um downloader com user-agent e certificado opcional.
    /// O downloader retornado deve ser capaz de baixar URLs e reportar progresso.
    fn create_downloader(
        &self,
        user_agent: String,
        cert_path: Option<PathBuf>,
    ) -> Box<dyn IDownloadExecutor>;
}

/// Interface para execução de downloads.
pub trait IDownloadExecutor {
    /// Faz download de uma URL e retorna os bytes.
    fn download(&self, label: &str, url: &str) -> Result<Vec<u8>>;
}

/// Interface para exibição de progresso de download no terminal.
pub trait IDownloadProgressReporter {
    /// Inicia a exibição de progresso com o rótulo dado.
    fn start(&mut self, label: &str);

    /// Atualiza a exibição de progresso.
    fn update(&mut self, bytes_downloaded: u64, total_bytes: Option<u64>);

    /// Finaliza a exibição de progresso.
    fn finish(&mut self);
}
