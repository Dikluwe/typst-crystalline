//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Init
//! Responsabilidade: Lógica de decisão associada à inicialização de templates.

use std::path::{Path, PathBuf};

/// Determina o diretório de destino do novo projeto.
/// Se um diretório alvo foi passado pelo usuário, usa ele, caso contrário,
/// o default é o nome do próprio pacote.
pub fn resolve_project_dir(arg_dir: Option<&str>, package_name: &str) -> PathBuf {
    Path::new(arg_dir.unwrap_or(package_name)).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_project_dir_with_arg() {
        let path = resolve_project_dir(Some("my-project"), "preview/fancy-template");
        assert_eq!(path, PathBuf::from("my-project"));
    }

    #[test]
    fn test_resolve_project_dir_without_arg() {
        let path = resolve_project_dir(None, "fancy-template");
        assert_eq!(path, PathBuf::from("fancy-template"));
    }
}
