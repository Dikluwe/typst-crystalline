//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Download
//! Responsabilidade: Construção de user-agent e classificação de tipos de download.

/// Constrói a string de User-Agent para requisições HTTP.
///
/// Formato: `typst/{version}`
pub fn build_user_agent(version: &str) -> String {
    format!("typst/{version}")
}

/// Tipo de recurso sendo baixado.
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadKind {
    /// Download de um pacote Typst.
    Package(String),
    /// Download de um release do binário.
    Release,
    /// Tipo desconhecido (sem progress label).
    Unknown,
}

impl DownloadKind {
    /// Retorna o rótulo de exibição para o tipo de download.
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Package(name) => Some(name),
            Self::Release => Some("release"),
            Self::Unknown => None,
        }
    }

    /// Determina se o progresso deve ser exibido.
    pub fn should_show_progress(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Formata uma mensagem de progresso de download.
///
/// Se `total` é conhecido, mostra porcentagem. Caso contrário, mostra apenas bytes.
pub fn format_progress(downloaded: u64, total: Option<u64>) -> String {
    match total {
        Some(total) if total > 0 => {
            let percent = (downloaded as f64 / total as f64 * 100.0).min(100.0);
            let downloaded_kb = downloaded / 1024;
            let total_kb = total / 1024;
            format!("{downloaded_kb}/{total_kb} KB ({percent:.0}%)")
        }
        _ => {
            let downloaded_kb = downloaded / 1024;
            format!("{downloaded_kb} KB")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user_agent() {
        assert_eq!(build_user_agent("0.14.0"), "typst/0.14.0");
        assert_eq!(build_user_agent("1.0.0-rc1"), "typst/1.0.0-rc1");
    }

    #[test]
    fn test_download_kind_package() {
        let kind = DownloadKind::Package("@preview/tablex:0.0.8".into());
        assert_eq!(kind.label(), Some("@preview/tablex:0.0.8"));
        assert!(kind.should_show_progress());
    }

    #[test]
    fn test_download_kind_release() {
        let kind = DownloadKind::Release;
        assert_eq!(kind.label(), Some("release"));
        assert!(kind.should_show_progress());
    }

    #[test]
    fn test_download_kind_unknown() {
        let kind = DownloadKind::Unknown;
        assert_eq!(kind.label(), None);
        assert!(!kind.should_show_progress());
    }

    #[test]
    fn test_format_progress_with_total() {
        let s = format_progress(512 * 1024, Some(1024 * 1024));
        assert_eq!(s, "512/1024 KB (50%)");
    }

    #[test]
    fn test_format_progress_complete() {
        let s = format_progress(1024 * 1024, Some(1024 * 1024));
        assert_eq!(s, "1024/1024 KB (100%)");
    }

    #[test]
    fn test_format_progress_no_total() {
        let s = format_progress(256 * 1024, None);
        assert_eq!(s, "256 KB");
    }

    #[test]
    fn test_format_progress_zero_total() {
        let s = format_progress(100 * 1024, Some(0));
        assert_eq!(s, "100 KB");
    }
}
