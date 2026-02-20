//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Info
//! Responsabilidade: Parsing de features, construção de structs de dados puros e formatação tabular.

/// Resultado do parsing de features de runtime.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedFeatures {
    pub html: bool,
    pub a11y_extras: bool,
}

/// Faz o parsing de uma string CSV de features em uma struct tipada.
///
/// # Exemplo
/// ```
/// let features = parse_feature_list("html,a11y-extras");
/// assert!(features.result.html);
/// assert!(features.result.a11y_extras);
/// ```
pub fn parse_feature_list(feature_list: &str) -> ParseFeatureResult {
    let mut features = ParsedFeatures::default();
    let mut unknown: Vec<String> = Vec::new();

    for feature in feature_list.split(',').filter(|s| !s.is_empty()) {
        match feature.trim() {
            "html" => features.html = true,
            "a11y-extras" => features.a11y_extras = true,
            other => unknown.push(other.to_string()),
        }
    }

    ParseFeatureResult { result: features, unknown }
}

/// Resultado do parsing contendo featueres reconhecidas e desconhecidas.
#[derive(Debug, Clone)]
pub struct ParseFeatureResult {
    pub result: ParsedFeatures,
    pub unknown: Vec<String>,
}

/// Informação de plataforma (constantes de compilação).
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: &'static str,
    pub arch: &'static str,
}

impl PlatformInfo {
    pub const fn current() -> Self {
        Self {
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
        }
    }
}

/// Configurações de compilação do binário.
#[derive(Debug, Clone)]
pub struct BuildSettings {
    pub self_update: bool,
    pub http_server: bool,
}

impl BuildSettings {
    /// Retorna descrições humanas das features de compilação.
    pub fn descriptions(&self) -> Vec<(&'static str, bool, &'static str)> {
        vec![
            ("self-update", self.self_update, "Update Typst via `typst update`"),
            ("http-server", self.http_server, "Serve HTML via `typst watch`"),
        ]
    }
}

/// Separa font paths de uma string delimitada por ':'.
pub fn parse_font_paths(raw: &str) -> Vec<std::path::PathBuf> {
    raw.split(':')
        .filter(|s| !s.is_empty())
        .map(std::path::PathBuf::from)
        .collect()
}

/// Calcula o padding máximo para uma lista de chaves.
pub fn max_key_pad<'a>(keys: impl Iterator<Item = &'a str>) -> Option<usize> {
    keys.map(|k| k.len()).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_features() {
        let result = parse_feature_list("");
        assert_eq!(result.result, ParsedFeatures::default());
        assert!(result.unknown.is_empty());
    }

    #[test]
    fn test_parse_known_features() {
        let result = parse_feature_list("html,a11y-extras");
        assert!(result.result.html);
        assert!(result.result.a11y_extras);
        assert!(result.unknown.is_empty());
    }

    #[test]
    fn test_parse_unknown_feature() {
        let result = parse_feature_list("html,banana");
        assert!(result.result.html);
        assert!(!result.result.a11y_extras);
        assert_eq!(result.unknown, vec!["banana"]);
    }

    #[test]
    fn test_parse_font_paths() {
        let paths = parse_font_paths("/usr/share/fonts:/home/user/.fonts");
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], std::path::PathBuf::from("/usr/share/fonts"));
        assert_eq!(paths[1], std::path::PathBuf::from("/home/user/.fonts"));
    }

    #[test]
    fn test_parse_font_paths_empty() {
        let paths = parse_font_paths("");
        assert!(paths.is_empty());
    }

    #[test]
    fn test_max_key_pad() {
        let pad = max_key_pad(["short", "much_longer_key", "mid"].into_iter());
        assert_eq!(pad, Some(15));
    }

    #[test]
    fn test_platform_info() {
        let p = PlatformInfo::current();
        assert!(!p.os.is_empty());
        assert!(!p.arch.is_empty());
    }
}
