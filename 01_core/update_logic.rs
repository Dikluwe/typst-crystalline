//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Update
//! Responsabilidade: Comparação de versões semver, construção de URLs de release e resolução de paths de backup.

use semver::Version;

const TYPST_GITHUB_ORG: &str = "typst";
const TYPST_REPO: &str = "typst";

/// Verifica se uma atualização é necessária comparando versões semver.
///
/// Faz o parse da tag do release (com ou sem prefixo 'v') e compara com a versão atual.
pub fn is_update_needed(current: &Version, release_tag: &str) -> Result<bool, String> {
    let new_tag: Version = release_tag
        .strip_prefix('v')
        .unwrap_or(release_tag)
        .parse()
        .map_err(|_| "release tag not in semver format".to_string())?;

    Ok(new_tag > *current)
}

/// Verifica se a versão alvo é um downgrade relativo à versão atual.
pub fn is_downgrade(target: &Version, current: &Version) -> bool {
    target < current
}

/// Verifica se a versão alvo possui o comando `update` disponível (>= 0.8.0).
pub fn has_update_command(version: &Version) -> bool {
    *version >= Version::new(0, 8, 0)
}

/// Constrói a URL da API do GitHub para buscar informações de release.
///
/// Se `tag` for `None`, retorna a URL do release mais recente.
/// Se `tag` for `Some(version)`, retorna a URL do release específico.
pub fn build_release_url(tag: Option<&Version>) -> String {
    match tag {
        Some(tag) => format!(
            "https://api.github.com/repos/{TYPST_GITHUB_ORG}/{TYPST_REPO}/releases/tags/v{tag}"
        ),
        None => format!(
            "https://api.github.com/repos/{TYPST_GITHUB_ORG}/{TYPST_REPO}/releases/latest"
        ),
    }
}

/// Resolve o caminho de backup padrão para uma plataforma.
///
/// Recebe o diretório raiz (state/data dir dependendo da plataforma)
/// e retorna o caminho completo do arquivo de backup.
pub fn resolve_backup_path(root_dir: &std::path::Path) -> std::path::PathBuf {
    root_dir.join("typst").join("typst_backup.part")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_needed_newer() {
        let current = Version::new(0, 12, 0);
        assert!(is_update_needed(&current, "v0.13.0").unwrap());
    }

    #[test]
    fn test_update_not_needed_same() {
        let current = Version::new(0, 12, 0);
        assert!(!is_update_needed(&current, "v0.12.0").unwrap());
    }

    #[test]
    fn test_update_not_needed_older() {
        let current = Version::new(0, 12, 0);
        assert!(!is_update_needed(&current, "v0.11.0").unwrap());
    }

    #[test]
    fn test_update_needed_no_prefix() {
        let current = Version::new(0, 12, 0);
        assert!(is_update_needed(&current, "0.14.0").unwrap());
    }

    #[test]
    fn test_update_needed_invalid_tag() {
        let current = Version::new(0, 12, 0);
        assert!(is_update_needed(&current, "not-a-version").is_err());
    }

    #[test]
    fn test_is_downgrade() {
        assert!(is_downgrade(&Version::new(0, 10, 0), &Version::new(0, 12, 0)));
        assert!(!is_downgrade(&Version::new(0, 14, 0), &Version::new(0, 12, 0)));
    }

    #[test]
    fn test_has_update_command() {
        assert!(!has_update_command(&Version::new(0, 7, 0)));
        assert!(has_update_command(&Version::new(0, 8, 0)));
        assert!(has_update_command(&Version::new(0, 14, 0)));
    }

    #[test]
    fn test_build_release_url_latest() {
        let url = build_release_url(None);
        assert!(url.contains("releases/latest"));
    }

    #[test]
    fn test_build_release_url_tagged() {
        let v = Version::new(0, 12, 0);
        let url = build_release_url(Some(&v));
        assert!(url.contains("releases/tags/v0.12.0"));
    }

    #[test]
    fn test_resolve_backup_path() {
        let root = std::path::Path::new("/home/user/.local/state");
        let path = resolve_backup_path(root);
        assert_eq!(path, std::path::PathBuf::from("/home/user/.local/state/typst/typst_backup.part"));
    }
}
