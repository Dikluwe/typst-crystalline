//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/info.md
//! @prompt-hash aa72f455
//! @layer L2

use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InfoData {
    pub version: String,
    pub build: BuildInfo,
    pub features: FeatureInfo,
    pub fonts: FontInfo,
    pub packages: PackageInfo,
    pub env: BTreeMap<String, Option<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildInfo {
    pub commit: Option<String>,
    pub platform: PlatformInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeatureInfo {
    pub html: bool,
    pub bundle: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FontInfo {
    pub system: bool,
    pub font_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PackageInfo {
    pub data_path: Option<String>,
    pub cache_path: Option<String>,
    pub custom_ca_configured: bool,
}

pub fn format_json(info: &InfoData, pretty: bool) -> Result<Vec<u8>, String> {
    let mut output =
        if pretty { serde_json::to_vec_pretty(info) } else { serde_json::to_vec(info) }
            .map_err(|error| error.to_string())?;
    output.push(b'\n');
    Ok(output)
}

pub fn format_human(info: &InfoData) -> Vec<u8> {
    let commit = info.build.commit.as_deref().unwrap_or("unknown");
    let short = &commit[..commit.len().min(8)];
    let mut output = format!(
        "Version: {}\nCommit: {}\nPlatform: {} {}\n\nFeatures:\n  HTML: {}\n  Bundle: {}\n\nFonts:\n  System: {}\n",
        info.version,
        short,
        info.build.platform.os,
        info.build.platform.arch,
        info.features.html,
        info.features.bundle,
        info.fonts.system,
    );
    for path in &info.fonts.font_paths {
        output.push_str(&format!("  Path: {path}\n"));
    }
    output.push_str("\nPackages:\n");
    output.push_str(&format!(
        "  Data path: {}\n  Cache path: {}\n  Custom CA: {}\n",
        info.packages.data_path.as_deref().unwrap_or("unavailable"),
        info.packages.cache_path.as_deref().unwrap_or("unavailable"),
        info.packages.custom_ca_configured,
    ));
    output.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> InfoData {
        InfoData {
            version: "0.15.1".into(),
            build: BuildInfo {
                commit: Some("1234567890".into()),
                platform: PlatformInfo { os: "linux".into(), arch: "x86_64".into() },
            },
            features: FeatureInfo { html: true, bundle: false },
            fonts: FontInfo { system: true, font_paths: vec![] },
            packages: PackageInfo {
                data_path: None,
                cache_path: None,
                custom_ca_configured: false,
            },
            env: BTreeMap::new(),
        }
    }

    #[test]
    fn json_keeps_full_commit() {
        let text = String::from_utf8(format_json(&fixture(), false).unwrap()).unwrap();
        assert!(text.contains("1234567890"));
    }

    #[test]
    fn human_truncates_commit() {
        let text = String::from_utf8(format_human(&fixture())).unwrap();
        assert!(text.contains("Commit: 12345678"));
        assert!(!text.contains("1234567890"));
    }
}
