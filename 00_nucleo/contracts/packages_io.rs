//! Contratos de Interação Externa detectados no `packages.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Pacotes

use std::path::PathBuf;
use typst_kit::packages::SystemPackages;

/// Interface para criação do provedor global de pacotes do sistema.
pub trait IPackageRegistryFactory {
    /// Cria o registro de pacotes combinando pacotes locais, cache e Universe.
    fn create_registry(
        &self,
        custom_package_path: Option<PathBuf>,
        custom_cache_path: Option<PathBuf>,
    ) -> SystemPackages;
}
