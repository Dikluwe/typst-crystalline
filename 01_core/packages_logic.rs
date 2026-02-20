//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Packages
//! Responsabilidade: Definição de configurações base para armazenamento de pacotes.
//! 
//! Nota: Como a resolução real de pacotes pertence a `typst_kit`, este
//! módulo atua predominantemente em L3 como uma fábrica. Este arquivo define
//! apenas as propriedades de configuração isentas de I/O.

use std::path::PathBuf;

/// Configuração pura para inicialização do repositório de pacotes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackageStorageConfig {
    pub custom_package_path: Option<PathBuf>,
    pub custom_cache_path: Option<PathBuf>,
}

impl PackageStorageConfig {
    /// Constrói uma nova configuração de pacotes.
    pub fn new(package_path: Option<PathBuf>, cache_path: Option<PathBuf>) -> Self {
        Self {
            custom_package_path: package_path,
            custom_cache_path: cache_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_storage_config_creation() {
        let config = PackageStorageConfig::new(Some(PathBuf::from("/pkg")), None);
        assert_eq!(config.custom_package_path, Some(PathBuf::from("/pkg")));
        assert_eq!(config.custom_cache_path, None);
    }
}
