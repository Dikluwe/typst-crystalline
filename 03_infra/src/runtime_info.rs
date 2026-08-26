//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/runtime_info.md
//! @prompt-hash 23f6b273
//! @layer L3

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeInfoSnapshot {
    pub package_data_path: Option<PathBuf>,
    pub package_cache_path: Option<PathBuf>,
    pub font_paths: Vec<PathBuf>,
    pub env: Vec<(String, Option<String>)>,
    pub custom_cert_configured: bool,
}

/// Captura somente configuração e paths; não toca rede nem varre fontes.
pub fn snapshot() -> RuntimeInfoSnapshot {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let package_data_path = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| home.as_ref().map(|path| path.join(".local/share")))
        .map(|path| path.join("typst/packages"));
    let package_cache_path = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| home.as_ref().map(|path| path.join(".cache")))
        .map(|path| path.join("typst/packages"));
    let font_paths = std::env::var_os("TYPST_FONT_PATHS")
        .map(|paths| std::env::split_paths(&paths).collect())
        .unwrap_or_default();
    let custom_cert_configured = std::env::var_os("TYPST_CERT").is_some();
    let env = [
        "TYPST_FEATURES",
        "TYPST_FONT_PATHS",
        "TYPST_ROOT",
        "XDG_CACHE_HOME",
        "XDG_DATA_HOME",
    ]
    .into_iter()
    .map(|key| (key.to_string(), std::env::var(key).ok()))
    .collect();
    RuntimeInfoSnapshot {
        package_data_path,
        package_cache_path,
        font_paths,
        env,
        custom_cert_configured,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_does_not_expose_proxy_or_cert_value() {
        let snapshot = snapshot();
        assert!(snapshot.env.iter().all(|(key, _)| !key.contains("PROXY")));
        assert!(snapshot.env.iter().all(|(key, _)| key != "TYPST_CERT"));
    }
}
