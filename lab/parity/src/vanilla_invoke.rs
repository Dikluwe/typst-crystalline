//! Helper de invocação vanilla typst CLI — P206C.
//!
//! Executa `typst query <typ_path> <selector> --format json`
//! via `std::process::Command` e captura output JSON.
//!
//! Per ADR-0075 PROPOSTO §"Mecanismo": vanilla CLI é
//! dependência ambiental externa (não compilada na
//! quarentena). Skip graceful se ausente.
//!
//! Pre-condição confirmada por P206B vanilla_cli_smoke.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Erros possíveis ao invocar vanilla CLI.
#[derive(Debug)]
pub enum VanillaInvokeError {
    /// Binário `typst` ausente em PATH.
    NotInstalled(String),
    /// Vanilla CLI presente mas comando falhou (exit != 0).
    CommandFailed { exit_code: Option<i32>, stderr: String },
    /// Output stdout não é JSON válido.
    JsonParseError(String),
    /// I/O error durante invocação.
    IoError(String),
}

impl std::fmt::Display for VanillaInvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VanillaInvokeError::NotInstalled(s) => write!(f, "vanilla typst CLI not installed: {}", s),
            VanillaInvokeError::CommandFailed { exit_code, stderr } => write!(
                f, "vanilla CLI failed (exit {:?}): {}", exit_code, stderr
            ),
            VanillaInvokeError::JsonParseError(s) => write!(f, "JSON parse error: {}", s),
            VanillaInvokeError::IoError(s) => write!(f, "I/O error: {}", s),
        }
    }
}

impl std::error::Error for VanillaInvokeError {}

/// Executa `typst query <typ_path> <selector> --format json`.
///
/// Retorna parsed JSON value. `Err(NotInstalled)` se
/// vanilla CLI ausente — caller deve skip graceful.
pub fn run_typst_query(
    typ_path: &Path,
    selector: &str,
) -> Result<serde_json::Value, VanillaInvokeError> {
    let output = Command::new("typst")
        .arg("query")
        .arg(typ_path)
        .arg(selector)
        .arg("--format")
        .arg("json")
        .output();

    let output = match output {
        Ok(o)  => o,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(VanillaInvokeError::NotInstalled(e.to_string()));
        }
        Err(e) => return Err(VanillaInvokeError::IoError(e.to_string())),
    };

    if !output.status.success() {
        return Err(VanillaInvokeError::CommandFailed {
            exit_code: output.status.code(),
            stderr:    String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| VanillaInvokeError::JsonParseError(e.to_string()))
}

/// Confirma que vanilla `typst` está disponível em PATH.
/// Útil como guard antes de iterar sobre corpus inteiro —
/// permite skip global se ausente.
pub fn vanilla_cli_available() -> bool {
    Command::new("typst")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Tempo máximo razoável para uma invocação `typst query`
/// num ficheiro corpus pequeno. Não enforced (Command::output
/// não suporta timeout nativamente; futuras iterações podem
/// usar `wait_timeout` crate).
#[allow(dead_code)]
pub const QUERY_TIMEOUT: Duration = Duration::from_secs(10);
