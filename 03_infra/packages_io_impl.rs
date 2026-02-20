//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Packages
//! Responsabilidade: Implementar contratos para fábrica de provedores de pacotes Typst.

use std::path::PathBuf;
use typst_kit::packages::{FsPackages, SystemPackages, UniversePackages};

#[path = "../00_nucleo/contracts/packages_io.rs"]
pub mod packages_io;

// Precisamos do contrato do downloader
use packages_io::IPackageRegistryFactory;

/// Construtor de `SystemPackages` acoplado ao SO e rede.
pub struct OsPackageRegistryFactory;

impl IPackageRegistryFactory for OsPackageRegistryFactory {
    fn create_registry(
        &self,
        custom_package_path: Option<PathBuf>,
        custom_cache_path: Option<PathBuf>,
    ) -> SystemPackages {
        // Obtenção do downloader legado (pois UniversePackages exige um tipo específico)
        let downloader = crate::download::downloader();

        SystemPackages::from_parts(
            custom_package_path
                .map(FsPackages::new)
                .or_else(FsPackages::system_data),
            custom_cache_path
                .map(FsPackages::new)
                .or_else(FsPackages::system_cache),
            UniversePackages::new(downloader),
        )
    }
}
