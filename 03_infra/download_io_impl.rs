//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Download
//! Responsabilidade: Implementar contratos de download HTTP e progresso.
//! Nota: A implementação concreta do downloader delega para `typst_kit::downloader`
//! que já é gerenciado pelo legado. Aqui expomos apenas o contrato e o wrapper
//! minimal necessário.

#[path = "../00_nucleo/contracts/download_io.rs"]
pub mod download_io;

// Os adaptadores concretos (SystemDownloader + ProgressDownloader) são
// construídos pelo código legado em `20_lab/crates/typst-cli/src/download.rs`
// que já usa `typst_kit::downloader`. Quando o legado for completamente
// migrado, os adaptadores concretos serão implementados aqui.
//
// Por ora, o contrato fica disponível para injeção futura.
