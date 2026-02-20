// -----------------------------------------------------------------------------
// Tipologia: Pure Logic (L1)
// Módulo: Compiler
// Descrição: Atomização pura dos algoritmos do motor, sem Efeitos Colaterais.
// -----------------------------------------------------------------------------

use std::collections::HashSet;
use typst_utils::hash128;
use typst_library::diag::{SourceDiagnostic, FileError};
use typst_syntax::{Span, FileId};
use ecow::{EcoVec, EcoString, eco_format, eco_vec};

/// Regra L1 - Lógica Pura: Deduplicação de diagnósticos (Erros/Avisos).
/// Sem efeitos colaterais. Transforma N diagnósticos em M diagnósticos únicos.
pub fn deduplicate_diagnostics(mut diags: EcoVec<SourceDiagnostic>) -> EcoVec<SourceDiagnostic> {
    let mut unique = HashSet::new(); // FxHashSet could be used if imported
    diags.retain(|diag| {
        // We use string and span to identify uniqueness (pure data)
        let hash = hash128(&(&diag.span, &diag.message));
        unique.insert(hash)
    });
    diags
}

/// Regra L1 - Lógica Pura: Heurística para erros clássicos de FileId
/// Gera hints de string sem ler arquivos de disco.
pub fn generate_invalid_file_hints(
    file_error: FileError,
    input: FileId,
    file_extension_exists: bool // Injected from I/O wrapper
) -> EcoVec<SourceDiagnostic> {
    let is_utf8_error = matches!(file_error, FileError::InvalidUtf8);
    let mut diagnostic = SourceDiagnostic::error(Span::detached(), EcoString::from(file_error));

    if is_utf8_error {
        match input.vpath().extension() {
            Some("typ") => return eco_vec![diagnostic],
            Some(ext) => {
                diagnostic.hint(eco_format!(
                    "a file with the `.{}` extension is not usually a Typst file",
                    ext
                ));
            }
            None => {
                diagnostic.hint("a file without an extension is not usually a Typst file");
            }
        };

        if file_extension_exists {
            diagnostic.hint("check if you meant to use the `.typ` extension instead");
        }
    }

    eco_vec![diagnostic]
}

/// Regra L1 - Lógica Pura: Validação de Funcionalidades
/// Decide de forma pure data-driven como agir diante do Target HTML.
pub fn validate_html_feature_flag(is_html_enabled: bool) -> Result<Option<SourceDiagnostic>, SourceDiagnostic> {
    const ISSUE: &str = "https://github.com/typst/typst/issues/5512";
    if is_html_enabled {
        Ok(Some(SourceDiagnostic::warning(
            Span::detached(),
            "html export is under active development and incomplete".into(),
        ).with_hint("its behaviour may change at any time".into())
         .with_hint("do not rely on this feature for production use cases".into())
         .with_hint(eco_format!("see {} for more information", ISSUE))))
    } else {
        Err(SourceDiagnostic::error(
            Span::detached(),
            "html export is only available when `--features html` is passed".into()
        ).with_hint("html export is under active development and incomplete".into())
         .with_hint(eco_format!("see {} for more information", ISSUE)))
    }
}
