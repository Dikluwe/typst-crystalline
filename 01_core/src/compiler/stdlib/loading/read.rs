//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/read.md
//! @prompt-hash a41d86d4
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Read accepts PathOrStr; CSV has its separate DataSource cast.
fn arg_read_path(args: &Args) -> SourceResult<&Value> {
    match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_))) => Ok(value),
        Some(other) => {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| {
                    occurrences.iter().find(|occurrence| occurrence.name.is_none())
                })
                .map_or(Span::detached(), |occurrence| occurrence.value_span);
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected path or string, found {}", vanilla_type_name(other)),
            )])
        }
        None => Err(err("read() requer 1 argumento posicional (caminho)")),
    }
}

pub(super) fn reject_named(args: &Args, fname: &str) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(format!("argumento nomeado inesperado em {fname}(): '{k}'")));
    }
    Ok(())
}

fn read_bytes(
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    path: &Value,
    fname: &str,
) -> SourceResult<(String, std::sync::Arc<Vec<u8>>)> {
    crate::compiler::stdlib::read_path_value(path, world, current_file)
        .map(|(path, bytes)| (path.vpath().get_with_slash().to_string(), bytes))
        .map_err(|msg| err(format!("{fname}(): não foi possível ler: {msg}")))
}

/// P701 — resolve o 1º posicional de `json`/`yaml`/`toml`/`cbor`/`xml` como
/// **caminho (`Str`, lido via `World::read_bytes`) ou dados crus (`Bytes`,
/// usados directamente, sem I/O)**. Paridade vanilla: estas 5 funções usam
/// `DataSource` (`Str | Bytes`) no vanilla — a mesma dualidade aqui.
pub(super) fn resolve_data(
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    fname: &str,
) -> SourceResult<std::sync::Arc<Vec<u8>>> {
    match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_))) => {
            read_bytes(world, current_file, value, fname).map(|(_, bytes)| bytes)
        }
        Some(Value::Bytes(b)) => Ok(std::sync::Arc::new(b.as_slice().to_vec())),
        Some(other) => {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| {
                    occurrences.iter().find(|occurrence| occurrence.name.is_none())
                })
                .map_or(Span::detached(), |occurrence| occurrence.value_span);
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected path, string, or bytes, found {}",
                    vanilla_type_name(other)
                ),
            )])
        }
        None => Err(err(format!(
            "{fname}() requer 1 argumento posicional (caminho ou bytes)"
        ))),
    }
}

/// `read(path, encoding: "utf8" | none)` → `Str` (utf8, default) ou `Bytes`
/// (`encoding: none`). Ficheiro não-UTF8 sem `encoding: none` → **erro**
/// (paridade vanilla medida em P824: `read.rs:24-47` + `diag.rs:797-807`).
/// O comentário que aqui existia ("heurística vanilla: tenta UTF-8; se
/// falhar, retorna bytes opacos") estava **refutado por medição** (P810 §11)
/// e foi removido — o vanilla não tem essa heurística.
pub fn native_read(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    for k in args.named.keys() {
        if k.as_str() != "encoding" {
            return Err(err(format!("argumento nomeado inesperado em read(): '{k}'")));
        }
    }
    let path_value = arg_read_path(args)?;
    let (path, data) = read_bytes(world, current_file, path_value, "read")?;
    match args.named.get("encoding") {
        None => read_utf8(&data, &path),
        Some(Value::None) => Ok(Value::Bytes(Bytes::new(data.to_vec()))),
        Some(Value::Str(s)) if s.as_str() == "utf8" => read_utf8(&data, &path),
        // Cast do vanilla de `Option<Encoding>`: string fora do domínio →
        // sem sufixo "found"; outro tipo → `, found {tipo}` (medido em P824).
        Some(Value::Str(_)) => Err(err("expected \"utf8\" or none")),
        Some(other) => Err(err(format!(
            "expected \"utf8\" or none, found {}",
            vanilla_type_name(other)
        ))),
    }
}

/// UTF-8 → `Str`; inválido → erro no formato do vanilla
/// (`diag.rs:797-807` + `load_err_in_binary`): posição `:{linha}:{col}` do
/// byte problemático via `LineCol::try_from_byte_pos` (`diag.rs:1027-1039`).
fn read_utf8(data: &[u8], path: &str) -> SourceResult<Value> {
    match std::str::from_utf8(data) {
        Ok(text) => Ok(Value::Str(text.into())),
        Err(e) => {
            let (line, col) = vanilla_line_col(&data[..e.valid_up_to()]);
            Err(err(format!(
                "failed to convert to string (file is not valid UTF-8 in {path}:{line}:{col})"
            )))
        }
    }
}

/// (linha, coluna) 1-based do fim de `prefix`, replicando
/// `LineCol::try_from_byte_pos` + `numbers()`: linha = nº de `'\n'`; coluna =
/// nº de chars desde o último `'\n'` — **1 quando não há `'\n'`** (quirk do
/// vanilla: `unwrap_or(bytes.len())` faz a coluna contar do fim do prefixo).
fn vanilla_line_col(prefix: &[u8]) -> (usize, usize) {
    let line = prefix.iter().filter(|&&b| b == b'\n').count();
    let line_start = prefix
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|i| i + 1)
        .unwrap_or(prefix.len());
    // `prefix` é UTF-8 válido (prefixo até `valid_up_to`).
    let col = std::str::from_utf8(&prefix[line_start..])
        .map(|s| s.chars().count())
        .unwrap_or(0);
    (line + 1, col + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_utf8_position_uses_vanilla_line_and_column() {
        let errors = read_utf8(b"a\n\xff", "sample.typ").unwrap_err();
        assert!(errors[0].message.contains("sample.typ:2:1"));
    }
}
