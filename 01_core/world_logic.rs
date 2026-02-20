//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: World
//! Responsabilidade: Lógica pura de cálculo de datetime, resolução de nomes para diagnósticos e validação de offsets.

use chrono::{DateTime, Datelike, FixedOffset, Local, Utc};

/// Calcula a data "hoje" do Typst a partir de um instante UTC e um offset opcional.
///
/// Se `offset` for `None`, usa o fuso local. Se fornecido, aplica o offset em segundos.
/// Retorna `None` se o offset for inválido (infinito, out-of-range).
pub fn calculate_today(
    now_utc: DateTime<Utc>,
    offset_seconds: Option<f64>,
    use_local_if_no_offset: bool,
) -> Option<DateComponents> {
    let now_fixed = if use_local_if_no_offset && offset_seconds.is_none() {
        now_utc.with_timezone(&Local).fixed_offset()
    } else {
        now_utc.fixed_offset()
    };

    let with_offset = match offset_seconds {
        None => now_fixed,
        Some(secs) => {
            let trunc = secs.trunc();
            if !trunc.is_finite()
                || trunc < f64::from(i32::MIN)
                || trunc > f64::from(i32::MAX)
            {
                return None;
            }
            let tz = FixedOffset::east_opt(trunc as i32)?;
            now_fixed.with_timezone(&tz)
        }
    };

    Some(DateComponents {
        year: with_offset.year(),
        month: with_offset.month(),
        day: with_offset.day(),
    })
}

/// Componentes de data extraídos para serialização.
#[derive(Debug, Clone, PartialEq)]
pub struct DateComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Resolve o nome de exibição de um arquivo para diagnósticos.
///
/// - Se é um arquivo de projeto, tenta expressar relativo ao `workdir`.
/// - Se é de pacote, formata como `@package/vpath`.
pub fn resolve_display_name(
    is_project: bool,
    package_name: Option<&str>,
    vpath_slash: &str,
    vpath_no_slash: &str,
    rooted_path: &std::path::Path,
    workdir: &std::path::Path,
) -> String {
    if is_project {
        pathdiff::diff_paths(rooted_path, workdir)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| vpath_no_slash.to_string())
    } else if let Some(pkg) = package_name {
        format!("{pkg}{vpath_slash}")
    } else {
        vpath_no_slash.to_string()
    }
}

/// Valida se um offset em segundos pode ser convertido para i32.
pub fn is_valid_offset(seconds: f64) -> bool {
    let trunc = seconds.trunc();
    trunc.is_finite()
        && trunc >= f64::from(i32::MIN)
        && trunc <= f64::from(i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_calculate_today_no_offset() {
        let now = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
        let result = calculate_today(now, None, false);
        assert_eq!(result, Some(DateComponents { year: 2025, month: 6, day: 15 }));
    }

    #[test]
    fn test_calculate_today_with_positive_offset() {
        // UTC+9 (Tokyo) at midnight UTC = 9:00 AM same day
        let now = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
        let result = calculate_today(now, Some(9.0 * 3600.0), false);
        assert_eq!(result, Some(DateComponents { year: 2025, month: 6, day: 15 }));
    }

    #[test]
    fn test_calculate_today_with_negative_offset_crosses_day() {
        // UTC-12 at 2:00 UTC = previous day 14:00
        let now = Utc.with_ymd_and_hms(2025, 6, 15, 2, 0, 0).unwrap();
        let result = calculate_today(now, Some(-12.0 * 3600.0), false);
        assert_eq!(result, Some(DateComponents { year: 2025, month: 6, day: 14 }));
    }

    #[test]
    fn test_calculate_today_invalid_offset() {
        let now = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
        assert_eq!(calculate_today(now, Some(f64::INFINITY), false), None);
        assert_eq!(calculate_today(now, Some(f64::NAN), false), None);
    }

    #[test]
    fn test_resolve_display_name_project() {
        let result = resolve_display_name(
            true,
            None,
            "/main.typ",
            "main.typ",
            std::path::Path::new("/home/user/project/main.typ"),
            std::path::Path::new("/home/user/project"),
        );
        assert_eq!(result, "main.typ");
    }

    #[test]
    fn test_resolve_display_name_package() {
        let result = resolve_display_name(
            false,
            Some("@preview/tablex:0.0.8"),
            "/lib.typ",
            "lib.typ",
            std::path::Path::new("/cache/tablex/lib.typ"),
            std::path::Path::new("/home/user"),
        );
        assert_eq!(result, "@preview/tablex:0.0.8/lib.typ");
    }

    #[test]
    fn test_is_valid_offset() {
        assert!(is_valid_offset(3600.0));
        assert!(is_valid_offset(-43200.0));
        assert!(!is_valid_offset(f64::INFINITY));
        assert!(!is_valid_offset(f64::NAN));
    }
}
