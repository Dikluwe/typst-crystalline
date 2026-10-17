//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/csv.md
//! @prompt-hash 09f810a3
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// CSV — csv. Array 2D; row-type array (default) ou dictionary.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum RowType {
    Array,
    Dictionary,
}

#[cfg(test)]
pub fn decode_csv(bytes: &[u8], delimiter: u8, row_type: RowType) -> SourceResult<Value> {
    decode_csv_impl(bytes, delimiter, row_type, CsvErrorContext::Pure)
}

#[derive(Clone, Copy)]
enum CsvErrorContext<'a> {
    #[cfg(test)]
    Pure,
    Bytes,
    File(&'a crate::entities::path::RootedPath, Span),
}

fn decode_csv_impl(
    bytes: &[u8],
    delimiter: u8,
    row_type: RowType,
    context: CsvErrorContext<'_>,
) -> SourceResult<Value> {
    let mut reader = ::csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        // P787 — sem `.flexible(true)`: linhas com nº de campos divergente
        // são rejeitadas (paridade vanilla; antes aceites em silêncio, P786 B1).
        .from_reader(bytes);

    // P1315 — o número de UnequalLengths é o ordinal do registro,
    // incluindo o cabeçalho, não a linha física do Position do parser.
    let map_csv_err = |e: ::csv::Error, line: usize| {
        let mut cause = match e.kind() {
            ::csv::ErrorKind::Utf8 { .. } => "file is not valid UTF-8".to_string(),
            ::csv::ErrorKind::UnequalLengths { expected_len, len, .. } => {
                format!("found {len} instead of {expected_len} fields in line {line}")
            }
            _ => e.to_string(),
        };
        let position = || match e.kind().position() {
            Some(position) => csv_error_position(bytes, position.byte()),
            None => Some((line, 1)),
        };
        let mut span = Span::detached();
        match context {
            CsvErrorContext::Bytes => {
                if let Some((line, column)) = position() {
                    cause.push_str(&format!(" at {line}:{column}"));
                }
            }
            // P1319: validade integral seleciona apresentação, depois da falha.
            CsvErrorContext::File(path, origin)
                if std::str::from_utf8(bytes).is_err() =>
            {
                use crate::entities::path::VirtualRoot;
                span = origin;
                cause.push_str(" in ");
                match path.root() {
                    VirtualRoot::Project => {
                        cause.push_str(
                            path.vpath().get_with_slash().trim_start_matches('/'),
                        );
                    }
                    VirtualRoot::Package(package) => {
                        cause.push_str(&package.to_string());
                        cause.push_str(path.vpath().get_with_slash());
                    }
                }
                if let Some((line, column)) = position() {
                    cause.push_str(&format!(":{line}:{column}"));
                }
            }
            #[cfg(test)]
            CsvErrorContext::Pure => {}
            CsvErrorContext::File(_, _) => {}
        }
        vec![SourceDiagnostic::error(span, format!("failed to parse CSV ({cause})"))]
    };

    let mut records = reader.records().enumerate();

    // row-type dictionary: a 1ª linha são as chaves (paridade vanilla).
    let header: Option<Vec<EcoString>> = if row_type == RowType::Dictionary {
        match records.next() {
            Some((index, r)) => {
                let r = r.map_err(|e| map_csv_err(e, index + 1))?;
                Some(r.iter().map(EcoString::from).collect())
            }
            None => None,
        }
    } else {
        None
    };

    let mut rows = Vec::new();
    for (index, rec) in records {
        let rec = rec.map_err(|e| map_csv_err(e, index + 1))?;
        match &header {
            None => {
                rows.push(Value::Array(
                    rec.iter().map(|c| Value::Str(c.into())).collect(),
                ));
            }
            Some(keys) => {
                let mut d = new_dict();
                for (i, cell) in rec.iter().enumerate() {
                    let key = keys
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| EcoString::from(i.to_string()));
                    d.insert(key, Value::Str(cell.into()));
                }
                rows.push(Value::Dict(d));
            }
        }
    }
    Ok(Value::Array(rows))
}

/// P1318: posição do parser, convertida somente após a falha. A validade
/// do buffer inteiro escolhe texto vs binário, não a variante do erro CSV.
fn csv_error_position(bytes: &[u8], offset: u64) -> Option<(usize, usize)> {
    let offset = usize::try_from(offset.min(u64::from(u32::MAX))).ok()?;
    let prefix = bytes.get(..offset)?;
    if let Ok(text) = std::str::from_utf8(bytes) {
        text.get(..offset)?;
        let mut chars = text.char_indices().peekable();
        let mut line = 1;
        let mut start = 0;
        while let Some((index, c)) = chars.next() {
            if index >= offset {
                break;
            }
            // Um offset entre CR/LF ainda pertence à linha anterior.
            if c == '\r' && chars.peek().is_some_and(|(_, next)| *next == '\n') {
                continue;
            }
            if matches!(
                c,
                '\n' | '\x0b' | '\x0c' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}'
            ) {
                line += 1;
                start = index + c.len_utf8();
            }
        }
        Some((line, text.get(start..offset)?.chars().count() + 1))
    } else {
        let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
        let start = prefix
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(prefix.len(), |i| i + 1);
        let column = String::from_utf8_lossy(&prefix[start..]).chars().count() + 1;
        Some((line, column))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Funções nativas — compõem L3 (read_bytes) + L1 (decode). ABI per ADR-0044:
// (ctx, args, world, current_file). Ver `figure_image::native_image`.
// ─────────────────────────────────────────────────────────────────────────────

fn arg_csv_source(args: &Args) -> SourceResult<&Value> {
    match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_) | Value::Bytes(_))) => Ok(value),
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
        None => {
            if let Some(argument) = args
                .occurrence_sequence()
                .iter()
                .find(|argument| argument.name.as_deref() == Some("source"))
            {
                return Err(vec![SourceDiagnostic::error(
                    argument.span,
                    "the argument `source` is positional",
                )
                .with_hint("try removing `source:`")]);
            }
            Err(vec![SourceDiagnostic::error(args.span, "missing argument: source")])
        }
    }
}

/// After parameter casts, reject the first unconsumed causal argument.
fn csv_finish_args(args: &Args) -> SourceResult<()> {
    let mut source_consumed = false;
    for argument in args.occurrence_sequence() {
        let message = match argument.name.as_deref() {
            Some("delimiter" | "row-type") => continue,
            None if !source_consumed => {
                source_consumed = true;
                continue;
            }
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(argument.span, message)]);
    }
    Ok(())
}

/// Validate every causal occurrence; only fully valid options may overwrite.
fn csv_named_option<T>(
    args: &Args,
    name: &str,
    default: T,
    cast: fn(&Value) -> SourceResult<T>,
) -> SourceResult<T> {
    let mut result = default;
    if let Some(occurrences) = &args.occurrences {
        for occurrence in
            occurrences.iter().filter(|item| item.name.as_deref() == Some(name))
        {
            result = cast(&occurrence.value).map_err(|mut errors| {
                for error in &mut errors {
                    error.span = occurrence.value_span;
                }
                errors
            })?;
        }
    } else if let Some(value) = args.named.get(name) {
        // Synthetic Args has no causal origin: the casts retain detached errors.
        result = cast(value)?;
    }
    Ok(result)
}

fn csv_delimiter(value: &Value) -> SourceResult<u8> {
    match value {
        Value::Str(s) => {
            // P787 — casts do vanilla (`loading/csv.rs:103-111`): 1 char ≠ →
            // "expected exactly one character"; não-ASCII → "delimiter must
            // be an ASCII character" (a mensagem anterior culpava o
            // comprimento, que estava certo — P786 D2).
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) if c.is_ascii() => Ok(c as u8),
                (Some(_), None) => Err(err("delimiter must be an ASCII character")),
                _ => Err(err("expected exactly one character")),
            }
        }
        other => Err(err(format!("expected string, found {}", vanilla_type_name(other)))),
    }
}

fn csv_row_type(value: &Value) -> SourceResult<RowType> {
    match value {
        // P787 — API vanilla: `row-type` recebe o TIPO (`dictionary`/`array`
        // como `Value::Type`), não a string (P786 D4 — divergência nos dois
        // sentidos). Tipo errado → "expected `array` or `dictionary`";
        // valor que não é tipo → "expected type, found ...".
        Value::Type(t) => match t {
            crate::entities::value::Type::Array => Ok(RowType::Array),
            crate::entities::value::Type::Dictionary => Ok(RowType::Dictionary),
            _ => Err(err("expected `array` or `dictionary`")),
        },
        other => Err(err(format!("expected type, found {}", vanilla_type_name(other)))),
    }
}

/// `csv(path | bytes, delimiter: ",", row-type: array)`. Aceita named `delimiter`
/// (Str de 1 char ASCII) e `row-type` (**tipo** `array`/`dictionary` —
/// API vanilla medida em P787; a forma string é rejeitada).
/// Subset graded: sem `escape`/`encoding` cosméticos.
pub fn native_csv(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    let source = arg_csv_source(args)?;

    let delimiter = csv_named_option(args, "delimiter", b',', csv_delimiter)?;
    let row_type = csv_named_option(args, "row-type", RowType::Array, csv_row_type)?;
    csv_finish_args(args)?;

    if let Value::Bytes(data) = source {
        return decode_csv_impl(
            data.as_slice(),
            delimiter,
            row_type,
            CsvErrorContext::Bytes,
        )
        .map_err(|mut errors| {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| {
                    occurrences.iter().find(|occurrence| occurrence.name.is_none())
                })
                .map_or(Span::detached(), |occurrence| occurrence.value_span);
            for error in &mut errors {
                error.span = span;
            }
            errors
        });
    }
    let (path, data) =
        crate::compiler::stdlib::read_path_value(source, world, current_file)
            .map_err(|message| err(format!("csv(): não foi possível ler: {message}")))?;
    let origin = args
        .occurrences
        .as_ref()
        .and_then(|occurrences| {
            occurrences.iter().find(|occurrence| occurrence.name.is_none())
        })
        .map_or(Span::detached(), |occurrence| occurrence.value_span);
    decode_csv_impl(&data, delimiter, row_type, CsvErrorContext::File(&path, origin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_csv_rows_in_order() {
        let Value::Array(rows) = decode_csv(b"a,b\n1,2", b',', RowType::Array).unwrap()
        else {
            panic!("rows expected")
        };
        assert_eq!(rows.len(), 2);
        assert!(matches!(&rows[1], Value::Array(fields) if fields.len() == 2));
    }
}
