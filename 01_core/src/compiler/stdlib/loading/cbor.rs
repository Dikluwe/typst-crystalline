//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/cbor.md
//! @prompt-hash 7edcc9c6
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// CBOR — ciborium. byte-strings → graded Err (Value::Bytes ausente, DEBT-62).
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_cbor(bytes: &[u8]) -> SourceResult<Value> {
    decode_cbor_with_source(bytes, None)
}

/// `path`: caminho da fonte (quando veio de ficheiro) para o sufixo
/// ` in {ficheiro}` da mensagem — paridade vanilla `load_err_in_binary`
/// (`diag.rs`): fonte path ganha o sufixo, fonte bytes fica sem ele.
fn decode_cbor_with_source(bytes: &[u8], path: Option<&str>) -> SourceResult<Value> {
    let v: ciborium::value::Value =
        ciborium::from_reader(bytes).map_err(|e| match path {
            Some(p) => {
                err(format!("failed to parse CBOR ({} in {p})", cbor_error_reason(&e)))
            }
            None => err(format!("failed to parse CBOR ({})", cbor_error_reason(&e))),
        })?;
    cbor_to_value(v)
}

/// Razão do erro no formato do vanilla `format_cbor_error`
/// (`loading/cbor.rs:88-98`): o `Display` do `ciborium::de::Error` delega em
/// `Debug`, por isso a razão extrai-se por variante (P823 — antes o cristalino
/// expunha esse `Debug`, ex.: `Semantic(None, "...")`).
fn cbor_error_reason(e: &ciborium::de::Error<std::io::Error>) -> String {
    use ciborium::de::Error;
    match e {
        Error::Io(e) => format!("IO error: {e}"),
        Error::Syntax(_) => "syntax error".to_string(),
        Error::Semantic(_, s) => s.clone(),
        Error::RecursionLimitExceeded => "recursion limit exceeded".to_string(),
    }
}

fn cbor_to_value(v: ciborium::value::Value) -> SourceResult<Value> {
    use ciborium::value::Value as C;
    match v {
        C::Null => Ok(Value::None),
        C::Bool(b) => Ok(Value::Bool(b)),
        C::Integer(i) => {
            let n: i128 = i.into();
            i64::try_from(n)
                .map(Value::Int)
                .map_err(|_| err("cbor: inteiro fora de i64"))
        }
        C::Float(f) => Ok(Value::Float(f)),
        C::Text(s) => Ok(Value::Str(s.into())),
        // P398: byte-strings CBOR → Value::Bytes (fecha DEBT-62).
        C::Bytes(b) => Ok(Value::Bytes(Bytes::new(b))),

        C::Tag(_, inner) => cbor_to_value(*inner),
        C::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for item in a {
                out.push(cbor_to_value(item)?);
            }
            Ok(Value::Array(out))
        }
        C::Map(m) => {
            let mut d = new_dict();
            for (k, val) in m {
                let key = match cbor_to_value(k)? {
                    Value::Str(s) => s,
                    _ => return Err(err("cbor: chave de mapa não-string sem suporte")),
                };
                d.insert(key, cbor_to_value(val)?);
            }
            Ok(Value::Dict(d))
        }
        _ => Err(err("cbor: variante sem suporte")),
    }
}

/// P701 — `cbor.encode(value)`: `Value` → bytes CBOR. Direcção inversa de
/// `cbor_to_value`. Paridade vanilla (`foundations/value.rs:345-366`,
/// `foundations/bytes.rs:364-374`): `None/Bool/Int/Float/Str/Bytes/Array/Dict`
/// mapeiam 1:1; tudo o resto (inclui `Symbol`/`Content`, que o vanilla
/// serializa de forma dedicada) cai no fallback `Text(repr(v))` — ver
/// `loading.md` §3.4 para a divergência documentada.
fn value_to_cbor(v: &Value) -> ciborium::value::Value {
    use ciborium::value::Value as C;
    match v {
        Value::None => C::Null,
        Value::Bool(b) => C::Bool(*b),
        Value::Int(i) => C::Integer((*i).into()),
        Value::Float(f) => C::Float(*f),
        Value::Str(s) => C::Text(s.to_string()),
        Value::Bytes(b) => C::Bytes(b.as_slice().to_vec()),
        Value::Array(a) => C::Array(a.iter().map(value_to_cbor).collect()),
        Value::Dict(d) => C::Map(
            d.iter()
                .map(|(k, val)| (C::Text(k.to_string()), value_to_cbor(val)))
                .collect(),
        ),
        other => C::Text(crate::compiler::eval::repr::repr_value(other)),
    }
}

/// `cbor.encode(value)` — acedido por field access em `cbor` (namespace,
/// P701; registo em `rules/eval/mod.rs`).
pub fn native_cbor_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args, "cbor.encode")?;
    match args.items.as_slice() {
        [v] => {
            let cbor_val = value_to_cbor(v);
            let mut buf = Vec::new();
            ciborium::into_writer(&cbor_val, &mut buf)
                .map_err(|e| err(format!("cbor.encode(): falha ao codificar: {e}")))?;
            Ok(Value::Bytes(Bytes::new(buf)))
        }
        _ => Err(err(format!(
            "cbor.encode() requer 1 argumento, recebeu {}",
            args.items.len()
        ))),
    }
}

/// `cbor(path | bytes)` — fora da macro `native_loader!` (P823): a mensagem
/// de erro precisa de saber se a fonte foi um caminho, para o sufixo
/// ` in {ficheiro}` do vanilla.
pub fn native_cbor(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args, "cbor")?;
    let path = match args.items.first() {
        Some(Value::Str(s)) => Some(s.to_string()),
        _ => None,
    };
    let data = resolve_data(args, world, current_file, "cbor")?;
    decode_cbor_with_source(&data[..], path.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_binary_byte_string() {
        let Value::Bytes(bytes) = decode_cbor(&[0x41, 0x7f]).unwrap() else {
            panic!("bytes expected")
        };
        assert_eq!(bytes.as_slice(), &[0x7f]);
    }
}
