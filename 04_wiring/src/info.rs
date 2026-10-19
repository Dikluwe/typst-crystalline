//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/info.md
//! @prompt-hash e27528ff
//! @layer L4
//! @updated 2026-09-26

use std::io::Write;
use std::process::ExitCode;
use typst_shell::cli::{self, InfoFormat, InfoIntent};

pub(super) fn run_info(intent: InfoIntent) -> ExitCode {
    use std::collections::BTreeMap;
    use typst_core::entities::version::PARITY_VERSION;
    use typst_shell::info::{
        BuildInfo, FeatureInfo, FontInfo, InfoData, PackageInfo, PlatformInfo,
    };

    let snapshot = typst_infra::runtime_info::snapshot();
    let info = InfoData {
        version: format!(
            "{}.{}.{}",
            PARITY_VERSION.0, PARITY_VERSION.1, PARITY_VERSION.2
        ),
        build: BuildInfo {
            commit: cli::build_commit().map(str::to_owned),
            platform: PlatformInfo {
                os: std::env::consts::OS.to_owned(),
                arch: std::env::consts::ARCH.to_owned(),
            },
        },
        features: FeatureInfo { html: false, a11y_extras: false, bundle: false },
        fonts: FontInfo {
            system: true,
            font_paths: snapshot
                .font_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
        },
        packages: PackageInfo {
            data_path: snapshot.package_data_path.map(|path| path.display().to_string()),
            cache_path: snapshot
                .package_cache_path
                .map(|path| path.display().to_string()),
            custom_ca_configured: intent.cert_path.is_some()
                || snapshot.custom_cert_configured,
        },
        env: snapshot.env.into_iter().collect::<BTreeMap<_, _>>(),
    };
    let output = match intent.format {
        Some(InfoFormat::Json) => {
            match typst_shell::info::format_json(&info, intent.pretty) {
                Ok(output) => output,
                Err(error) => {
                    eprintln!("error: failed to serialize info: {error}");
                    return ExitCode::from(2);
                }
            }
        }
        None => typst_shell::info::format_human(&info),
    };
    if let Err(error) = std::io::stdout().lock().write_all(&output) {
        eprintln!("error: failed to write info: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}
