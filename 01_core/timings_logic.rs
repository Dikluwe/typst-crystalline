//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Timings
//! Responsabilidade: Lógica de formatação do iterador `{n}` de paths de log e decodificação de Span.

use std::path::{Path, PathBuf};
use typst::diag::{bail, StrResult};
use typst::syntax::Span;
use typst::World;

/// Substitui a sub-string `{n}` no caminho alvo pelo indíce atual do loop de watch.
/// Impede que sobrescrevamos logs sucessivos sem o template explícito.
pub fn format_recording_path(path: &Path, index: usize) -> StrResult<PathBuf> {
    let string = path.to_str().unwrap_or_default();
    let numbered = string.contains("{n}");
    
    if !numbered && index > 0 {
        bail!("cannot export multiple recordings without `{{n}}` in path");
    }
    
    if numbered {
        Ok(PathBuf::from(string.replace("{n}", &index.to_string())))
    } else {
        Ok(path.to_path_buf())
    }
}

/// Transforma um Span bruto em um par contendo a identificação do arquivo fonte 
/// originário do `World` e a respectiva linha computada a partir do byte index.
pub fn resolve_span(world: &dyn World, span: Span) -> Option<(String, u32)> {
    let id = span.id()?;
    let source = world.source(id).ok()?;
    let range = source.range(span)?;
    let line = source.lines().byte_to_line(range.start)?;
    Some((format!("{id:?}"), line as u32 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_recording_path_with_pattern() {
        let path = Path::new("profile-{n}.json");
        assert_eq!(format_recording_path(path, 0).unwrap(), PathBuf::from("profile-0.json"));
        assert_eq!(format_recording_path(path, 42).unwrap(), PathBuf::from("profile-42.json"));
    }

    #[test]
    fn test_format_recording_path_single_run() {
        let path = Path::new("profile.json");
        assert_eq!(format_recording_path(path, 0).unwrap(), PathBuf::from("profile.json"));
    }

    #[test]
    fn test_format_recording_path_watch_error_on_missing_pattern() {
        let path = Path::new("profile.json");
        assert!(format_recording_path(path, 1).is_err());
    }
}
