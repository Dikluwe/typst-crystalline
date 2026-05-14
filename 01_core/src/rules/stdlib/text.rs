//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f45bcc3a
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de texto (upper, lower, replace).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::{err, expect_no_named};
use crate::entities::file_id::FileId;

use crate::entities::args::Args;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────

/// `upper(str | content)` → texto em maiúsculas.
pub fn native_upper(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_uppercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_uppercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!("upper() espera string ou content, recebeu {}", other.type_name())),
        _ => err("upper() requer 1 argumento".to_string()),
    }
}

/// `lower(str | content)` → texto em minúsculas.
pub fn native_lower(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_lowercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_lowercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!("lower() espera string ou content, recebeu {}", other.type_name())),
        _ => err("lower() requer 1 argumento".to_string()),
    }
}

/// `replace(fonte, padrão, substituição, count: N)` → string ou content com substituição.
///
/// `count` é global ao documento: persiste entre nós de texto via `FnMut`.
pub fn native_replace(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    // Validar named args: apenas "count" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "count" {
            return err(format!("argumento nomeado inesperado: '{}'", key));
        }
    }

    if args.items.len() < 3 {
        return err("replace() requer 3 argumentos: fonte, padrão, substituição".to_string());
    }

    let pattern = match &args.items[1] {
        Value::Str(s) => s.to_string(),
        other => return err(format!("replace(): padrão deve ser string, recebeu {}", other.type_name())),
    };
    let replacement = match &args.items[2] {
        Value::Str(s) => s.to_string(),
        other => return err(format!("replace(): substituição deve ser string, recebeu {}", other.type_name())),
    };

    // Bloquear padrão vazio: replacen("", ...) entra em ciclo infinito.
    if pattern.is_empty() {
        return err("replace(): o padrão de busca não pode estar vazio".to_string());
    }

    let mut remaining_count: Option<i64> = args.named.get("count")
        .and_then(|v| match v {
            Value::Int(i) => Some(*i),
            _ => None,
        });

    // A closure carrega `remaining_count` como estado mutável.
    // `map_text` usa `&mut F`, portanto o estado persiste entre nós do AST.
    // Isto garante que `count: N` é global ao documento, não por nó de texto.
    let mut do_replace = |text: &str| -> String {
        match remaining_count.as_mut() {
            Some(c) if *c <= 0 => text.to_string(),
            Some(c) => {
                let limit = *c as usize;
                let count_used = text.matches(pattern.as_str()).take(limit).count();
                let result = text.replacen(pattern.as_str(), replacement.as_str(), limit);
                *c -= count_used as i64;
                result
            }
            None => text.replace(pattern.as_str(), replacement.as_str()),
        }
    };

    match &args.items[0] {
        Value::Str(s) => Ok(Value::Str(do_replace(s.as_str()).into())),
        Value::Content(c) => Ok(Value::Content(c.map_text(&mut do_replace))),
        other => err(format!("replace(): 1º argumento deve ser string ou content, recebeu {}", other.type_name())),
    }
}
