//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/collections.md
//! @prompt-hash 548d14e4
//! @layer L1
//! @updated 2026-07-23
//!
//! Métodos de instância para os tipos de coleção `array`, `dict` e `str`.
//! Materializado no Passo 466 (P466).

use std::cmp::Ordering;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::compiler::eval::call_dispatch::apply_func;
use crate::compiler::eval::EvalContext;
use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::{
    native_bytes, native_counter, native_datetime, native_float, native_int,
    native_range, native_state, native_str, native_symbol, native_type,
};
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::regex::Regex;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

/// Metadados sintácticos internos para âncoras de diagnósticos de método.
#[derive(Debug, Clone)]
pub(crate) struct CollectionCallSpans {
    pub(crate) call: Span,
    pub(crate) positional: Vec<Span>,
    pub(crate) named: IndexMap<EcoString, Span, FxBuildHasher>,
}

impl CollectionCallSpans {
    fn call_or(&self, fallback: Span) -> Span {
        if self.call.is_detached() {
            fallback
        } else {
            self.call
        }
    }

    fn positional_or(&self, index: usize, fallback: Span) -> Span {
        self.positional.get(index).copied().unwrap_or(fallback)
    }

    fn named_or(&self, name: &str, fallback: Span) -> Span {
        self.named.get(name).copied().unwrap_or(fallback)
    }
}

/// Tenta despachar uma chamada de método de coleção (`array`, `dict`, `str`).
///
/// Retorna `Some(Result)` se o par `(target, method)` for reconhecido;
/// `None` caso contrário, permitindo que o caller continue com o dispatch
/// genérico de funções.
pub(crate) fn try_dispatch_collection_method(
    target: Value,
    method: &str,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> Option<SourceResult<Value>> {
    match (target, method) {
        // ── array ────────────────────────────────────────────────────────────
        (Value::Array(arr), "len") => Some(array_len(arr, args)),
        (Value::Array(arr), "first") => Some(array_first(arr, args)),
        (Value::Array(arr), "last") => Some(array_last(arr, args)),
        (Value::Array(arr), "at") => Some(array_at(arr, args)),
        (Value::Array(arr), "slice") => Some(array_slice(arr, args)),
        (Value::Array(arr), "rev") => Some(array_no_args(arr, args, array_rev)),
        (Value::Array(arr), "sum") => Some(array_sum(arr, args)),
        (Value::Array(arr), "product") => Some(array_product(arr, args)),
        (Value::Array(arr), "contains") => Some(array_contains(arr, args)),
        (Value::Array(arr), "sorted") => {
            Some(array_sorted(arr, args, scopes, ctx, engine))
        }
        (Value::Array(arr), "filter") => {
            Some(array_filter(arr, args, scopes, ctx, engine))
        }
        (Value::Array(arr), "map") => Some(array_map(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "find") => Some(array_find(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "position") => {
            Some(array_position(arr, args, scopes, ctx, engine))
        }
        (Value::Array(arr), "any") => Some(array_any(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "all") => Some(array_all(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "zip") => Some(array_zip(arr, args)),
        (Value::Array(arr), "enumerate") => Some(array_enumerate(arr, args)),
        (Value::Array(arr), "dedup") => Some(array_dedup(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "chunks") => Some(array_chunks(arr, args)),
        (Value::Array(arr), "windows") => Some(array_windows(arr, args)),
        (Value::Array(arr), "flatten") => Some(array_no_args(arr, args, array_flatten)),
        (Value::Array(arr), "fold") => Some(array_fold(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "reduce") => {
            Some(array_reduce(arr, args, scopes, ctx, engine))
        }
        (Value::Array(arr), "intersperse") => Some(array_intersperse(arr, args)),
        (Value::Array(arr), "split") => Some(array_split(arr, args)),
        (Value::Array(arr), "to-dict") => Some(array_to_dict(arr, args)),
        // P843 (#60) — `array.join(separator?, last:?, default:?)`.
        (Value::Array(arr), "join") => Some(array_join(arr, args)),

        // ── dict ─────────────────────────────────────────────────────────────
        (Value::Dict(dict), "keys") => Some(dict_no_args(dict, args, dict_keys)),
        (Value::Dict(dict), "values") => Some(dict_no_args(dict, args, dict_values)),
        (Value::Dict(dict), "pairs") => Some(dict_no_args(dict, args, dict_pairs)),
        (Value::Dict(dict), "remove") => Some(dict_remove(dict, args)),
        (Value::Dict(dict), "update") => Some(dict_update(dict, args)),
        (Value::Dict(dict), "insert") => Some(dict_insert(dict, args)),
        (Value::Dict(dict), "at") => Some(dict_at(dict, args)),
        (Value::Dict(dict), "len") => Some(dict_no_args(dict, args, dict_len)),
        (Value::Dict(dict), "map") => Some(dict_map(dict, args, scopes, ctx, engine)),
        (Value::Dict(dict), "filter") => {
            Some(dict_filter(dict, args, scopes, ctx, engine))
        }

        // ── str ──────────────────────────────────────────────────────────────
        (Value::Str(s), "len") => Some(str_no_args(s, args, str_len)),
        (Value::Str(s), "first") => Some(str_first(s, args)),
        (Value::Str(s), "last") => Some(str_last(s, args)),
        (Value::Str(s), "at") => Some(str_at(s, args)),
        (Value::Str(s), "slice") => Some(str_slice(s, args)),
        (Value::Str(s), "char-len") => Some(Ok(char_len(s))),
        (Value::Str(s), "char-at") => Some(char_at(s, args)),
        (Value::Str(s), "char-slice") => Some(char_slice(s, args)),
        (Value::Str(s), "clusters") => Some(str_no_args(s, args, str_clusters)),
        (Value::Str(s), "contains") => Some(str_contains(s, args)),
        (Value::Str(s), "starts-with") => Some(str_starts_with(s, args)),
        (Value::Str(s), "ends-with") => Some(str_ends_with(s, args)),
        (Value::Str(s), "find") => Some(str_find(s, args)),
        (Value::Str(s), "replace") => Some(str_replace(s, args)),
        (Value::Str(s), "trim") => Some(str_trim_dispatch(s, args)),
        (Value::Str(s), "split") => Some(str_split(s, args)),
        (Value::Str(s), "repeat") => Some(str_repeat(s, args)),
        (Value::Str(s), "to-upper") => Some(Ok(str_to_upper(s))),
        (Value::Str(s), "to-lower") => Some(Ok(str_to_lower(s))),
        (Value::Str(s), "to-unicode") => {
            Some(ensure_no_args(&args).and_then(|_| str_to_unicode(s)))
        }
        (Value::Str(s), "rev") => Some(str_no_args(s, args, str_rev)),
        (Value::Str(s), "codepoints") => Some(str_no_args(s, args, str_codepoints)),
        (Value::Str(s), "normalize") => Some(str_normalize(s, args)),
        (Value::Str(s), "position") => Some(str_position(s, args)),
        (Value::Str(s), "match") => Some(str_match(s, args)),
        (Value::Str(s), "matches") => Some(str_matches(s, args)),

        // ── bytes (P1214) ───────────────────────────────────────────────────
        (Value::Bytes(bytes), method) => {
            try_dispatch_bytes_method(bytes, method, args, call_spans)
        }

        // ── arguments (P1284) ──────────────────────────────────────────────
        (Value::Args(arguments), "len") => Some(arguments_len(arguments, args)),
        (Value::Args(arguments), "at") => Some(arguments_at(arguments, args)),
        (Value::Args(arguments), "pos") => Some(arguments_pos(arguments, args)),
        (Value::Args(arguments), "named") => Some(arguments_named(arguments, args)),
        (Value::Args(arguments), "filter") => {
            Some(arguments_filter(arguments, args, scopes, ctx, engine))
        }
        (Value::Args(arguments), "map") => {
            Some(arguments_map(arguments, args, scopes, ctx, engine))
        }

        _ => None, // neutro: N16[β] — Value não-indexável retorna None na projecção de colecção
    }
}

fn try_dispatch_bytes_method(
    bytes: Bytes,
    method: &str,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> Option<SourceResult<Value>> {
    match method {
        "len" => Some(bytes_len(bytes, args, call_spans)),
        "at" => Some(bytes_at(bytes, args, call_spans)),
        "slice" => Some(bytes_slice(bytes, args, call_spans)),
        _ => None,
    }
}

fn unexpected_named(
    args: &Args,
    allowed: &[&str],
    call_spans: Option<&CollectionCallSpans>,
) -> Option<SourceDiagnostic> {
    args.named
        .keys()
        .find(|name| !allowed.contains(&name.as_str()))
        .map(|name| {
            let span = call_spans
                .map(|spans| spans.named_or(name.as_str(), args.span))
                .unwrap_or(args.span);
            SourceDiagnostic::error(span, format!("unexpected argument: {name}"))
        })
}

fn bytes_len(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    if let Some(err) = unexpected_named(&args, &[], call_spans) {
        return Err(vec![err]);
    }
    if !args.items.is_empty() {
        let span = call_spans
            .map(|spans| spans.positional_or(0, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    Ok(Value::Int(bytes.len() as i64))
}

fn bytes_at(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    let call_span = call_spans.map(|spans| spans.call_or(args.span)).unwrap_or(args.span);
    let Some(index_value) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(call_span, "missing argument: index")]);
    };
    let index = match index_value {
        Value::Int(index) => *index,
        other => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(0, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    if args.items.len() > 1 {
        let span = call_spans
            .map(|spans| spans.positional_or(1, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    if let Some(err) = unexpected_named(&args, &["default"], call_spans) {
        return Err(vec![err]);
    }

    let len = bytes.len() as i64;
    let resolved = if index >= 0 { Some(index) } else { len.checked_add(index) };
    if let Some(byte) = resolved
        .filter(|&value| value >= 0 && value < len)
        .and_then(|value| bytes.as_slice().get(value as usize))
    {
        return Ok(Value::Int((*byte).into()));
    }
    if let Some(default) = args.named.get("default") {
        return Ok(default.clone());
    }
    Err(vec![SourceDiagnostic::error(
        call_span,
        format!(
            "byte index out of bounds (index: {index}, len: {len}) and no default value was specified"
        ),
    )])
}

fn bytes_slice(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    let call_span = call_spans.map(|spans| spans.call_or(args.span)).unwrap_or(args.span);
    let Some(start_value) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(call_span, "missing argument: start")]);
    };
    let start = match start_value {
        Value::Int(start) => *start,
        other => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(0, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    let end = match args.items.get(1) {
        None | Some(Value::None) => None,
        Some(Value::Int(end)) => Some(*end),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(1, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer or none, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    if args.items.len() > 2 {
        let span = call_spans
            .map(|spans| spans.positional_or(2, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    if let Some(err) = unexpected_named(&args, &["count"], call_spans) {
        return Err(vec![err]);
    }
    let count = match args.named.get("count") {
        None => None,
        Some(Value::Int(count)) => Some(*count),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.named_or("count", args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };

    let len = bytes.len() as i64;
    let locate = |index: i64| -> Result<usize, Vec<SourceDiagnostic>> {
        let wrapped = if index >= 0 { Some(index) } else { len.checked_add(index) };
        wrapped
            .and_then(|value| usize::try_from(value).ok())
            .filter(|&value| value <= bytes.len())
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    call_span,
                    format!("byte index out of bounds (index: {index}, len: {len})"),
                )]
            })
    };

    let start_index = locate(start)?;
    let end_raw = end
        .or(count.map(|count| (start_index as i64).wrapping_add(count)))
        .unwrap_or(len);
    let end_index = locate(end_raw)?.max(start_index);
    Ok(Value::Bytes(Bytes::new(bytes.as_slice()[start_index..end_index].to_vec())))
}

/// **P1142** — membros não ligados expostos nos valores-tipo `array` e `str`.
/// As funções delegam aos mesmos helpers das formas de instância.
pub(crate) fn collection_type_field(t: Type, field: &str) -> Option<Value> {
    let function = match (t, field) {
        (Type::Array, "range") => Func::native("range", native_range),
        (Type::Array, "all") => Func::native_with_engine("all", array_all_static),
        (Type::Array, "any") => Func::native_with_engine("any", array_any_static),
        (Type::Array, "at") => Func::native_with_engine("at", array_at_static),
        (Type::Array, "chunks") => {
            Func::native_with_engine("chunks", array_chunks_static)
        }
        (Type::Array, "contains") => {
            Func::native_with_engine("contains", array_contains_static)
        }
        (Type::Array, "dedup") => Func::native_with_engine("dedup", array_dedup_static),
        (Type::Array, "enumerate") => {
            Func::native_with_engine("enumerate", array_enumerate_static)
        }
        (Type::Array, "filter") => {
            Func::native_with_engine("filter", array_filter_static)
        }
        (Type::Array, "find") => Func::native_with_engine("find", array_find_static),
        (Type::Array, "first") => Func::native_with_engine("first", array_first_static),
        (Type::Array, "flatten") => {
            Func::native_with_engine("flatten", array_flatten_static)
        }
        (Type::Array, "fold") => Func::native_with_engine("fold", array_fold_static),
        (Type::Array, "insert") => {
            Func::native_with_engine("insert", array_insert_static)
        }
        (Type::Array, "intersperse") => {
            Func::native_with_engine("intersperse", array_intersperse_static)
        }
        (Type::Array, "join") => Func::native_with_engine("join", array_join_static),
        (Type::Array, "last") => Func::native_with_engine("last", array_last_static),
        (Type::Array, "len") => Func::native_with_engine("len", array_len_static),
        (Type::Array, "map") => Func::native_with_engine("map", array_map_static),
        (Type::Array, "pop") => Func::native_with_engine("pop", array_pop_static),
        (Type::Array, "position") => {
            Func::native_with_engine("position", array_position_static)
        }
        (Type::Array, "product") => {
            Func::native_with_engine("product", array_product_static)
        }
        (Type::Array, "push") => Func::native_with_engine("push", array_push_static),
        (Type::Array, "reduce") => {
            Func::native_with_engine("reduce", array_reduce_static)
        }
        (Type::Array, "remove") => {
            Func::native_with_engine("remove", array_remove_static)
        }
        (Type::Array, "rev") => Func::native_with_engine("rev", array_rev_static),
        (Type::Array, "slice") => Func::native_with_engine("slice", array_slice_static),
        (Type::Array, "sorted") => {
            Func::native_with_engine("sorted", array_sorted_static)
        }
        (Type::Array, "split") => Func::native_with_engine("split", array_split_static),
        (Type::Array, "sum") => Func::native_with_engine("sum", array_sum_static),
        (Type::Array, "to-dict") => {
            Func::native_with_engine("to-dict", array_to_dict_static)
        }
        (Type::Array, "windows") => {
            Func::native_with_engine("windows", array_windows_static)
        }
        (Type::Array, "zip") => Func::native_with_engine("zip", array_zip_static),

        (Type::Dictionary, "at") => Func::native_with_engine("at", dict_at_static),
        (Type::Dictionary, "filter") => {
            Func::native_with_engine("filter", dict_filter_static)
        }
        (Type::Dictionary, "insert") => {
            Func::native_with_engine("insert", dict_insert_static)
        }
        (Type::Dictionary, "keys") => Func::native_with_engine("keys", dict_keys_static),
        (Type::Dictionary, "len") => Func::native_with_engine("len", dict_len_static),
        (Type::Dictionary, "map") => Func::native_with_engine("map", dict_map_static),
        (Type::Dictionary, "pairs") => {
            Func::native_with_engine("pairs", dict_pairs_static)
        }
        (Type::Dictionary, "remove") => {
            Func::native_with_engine("remove", dict_remove_static)
        }
        (Type::Dictionary, "values") => {
            Func::native_with_engine("values", dict_values_static)
        }

        (Type::Str, "at") => Func::native_with_engine("at", str_at_static),
        (Type::Str, "clusters") => {
            Func::native_with_engine("clusters", str_clusters_static)
        }
        (Type::Str, "codepoints") => {
            Func::native_with_engine("codepoints", str_codepoints_static)
        }
        (Type::Str, "contains") => {
            Func::native_with_engine("contains", str_contains_static)
        }
        (Type::Str, "ends-with") => {
            Func::native_with_engine("ends-with", str_ends_with_static)
        }
        (Type::Str, "find") => Func::native_with_engine("find", str_find_static),
        (Type::Str, "first") => Func::native_with_engine("first", str_first_static),
        (Type::Str, "last") => Func::native_with_engine("last", str_last_static),
        (Type::Str, "len") => Func::native_with_engine("len", str_len_static),
        (Type::Str, "match") => Func::native_with_engine("match", str_match_static),
        (Type::Str, "matches") => Func::native_with_engine("matches", str_matches_static),
        (Type::Str, "normalize") => {
            Func::native_with_engine("normalize", str_normalize_static)
        }
        (Type::Str, "position") => {
            Func::native_with_engine("position", str_position_static)
        }
        (Type::Str, "replace") => Func::native_with_engine("replace", str_replace_static),
        (Type::Str, "rev") => Func::native_with_engine("rev", str_rev_static),
        (Type::Str, "slice") => Func::native_with_engine("slice", str_slice_static),
        (Type::Str, "split") => Func::native_with_engine("split", str_split_static),
        (Type::Str, "starts-with") => {
            Func::native_with_engine("starts-with", str_starts_with_static)
        }
        (Type::Str, "to-unicode") => {
            Func::native_with_engine("to-unicode", str_to_unicode_static)
        }
        (Type::Str, "trim") => Func::native_with_engine("trim", str_trim_static),

        (Type::Bytes, "at") => Func::native_with_engine("at", bytes_at_static),
        (Type::Bytes, "len") => Func::native_with_engine("len", bytes_len_static),
        (Type::Bytes, "slice") => Func::native_with_engine("slice", bytes_slice_static),

        (Type::Arguments, "at") => Func::native_with_engine("at", args_at_static),
        (Type::Arguments, "filter") => {
            Func::native_with_engine("filter", args_filter_static)
        }
        (Type::Arguments, "len") => Func::native_with_engine("len", args_len_static),
        (Type::Arguments, "map") => Func::native_with_engine("map", args_map_static),
        (Type::Arguments, "named") => {
            Func::native_with_engine("named", args_named_static)
        }
        (Type::Arguments, "pos") => Func::native_with_engine("pos", args_pos_static),
        _ => return None,
    };
    Some(Value::Func(function))
}

fn static_collection_call(
    expected: Type,
    method: &str,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if args.named.contains_key("self") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `self` is positional".to_string(),
        )]);
    }
    let Some((target, rest)) = args.items.split_first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")]);
    };
    if target.type_of() != expected {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "expected {}, found {}",
                match expected {
                    Type::Str => "string",
                    _ => expected.name(),
                },
                crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                    target
                )
            ),
        )]);
    }
    let delegated = Args {
        items: rest.to_vec(),
        named: args.named.clone(),
        span: args.span,
    };
    try_dispatch_collection_method(
        target.clone(),
        method,
        delegated,
        None,
        scopes,
        ctx,
        engine,
    )
    .unwrap_or_else(|| {
        Err(vec![SourceDiagnostic::error(
            args.span,
            format!("type {} has no method `{method}`", expected.name()),
        )])
    })
}

macro_rules! collection_static {
    ($rust:ident, $ty:expr, $method:literal) => {
        fn $rust(
            ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            scopes: &mut Scopes<'_>,
            engine: &mut Engine<'_>,
        ) -> SourceResult<Value> {
            static_collection_call($ty, $method, args, scopes, ctx, engine)
        }
    };
}

collection_static!(array_all_static, Type::Array, "all");
collection_static!(array_any_static, Type::Array, "any");
collection_static!(array_at_static, Type::Array, "at");
collection_static!(array_chunks_static, Type::Array, "chunks");
collection_static!(array_contains_static, Type::Array, "contains");
collection_static!(array_dedup_static, Type::Array, "dedup");
collection_static!(array_enumerate_static, Type::Array, "enumerate");
collection_static!(array_filter_static, Type::Array, "filter");
collection_static!(array_find_static, Type::Array, "find");
collection_static!(array_first_static, Type::Array, "first");
collection_static!(array_flatten_static, Type::Array, "flatten");
collection_static!(array_fold_static, Type::Array, "fold");
collection_static!(array_insert_static, Type::Array, "insert");
collection_static!(array_intersperse_static, Type::Array, "intersperse");
collection_static!(array_join_static, Type::Array, "join");
collection_static!(array_last_static, Type::Array, "last");
collection_static!(array_len_static, Type::Array, "len");
collection_static!(array_map_static, Type::Array, "map");
collection_static!(array_pop_static, Type::Array, "pop");
collection_static!(array_position_static, Type::Array, "position");
collection_static!(array_product_static, Type::Array, "product");
collection_static!(array_push_static, Type::Array, "push");
collection_static!(array_reduce_static, Type::Array, "reduce");
collection_static!(array_remove_static, Type::Array, "remove");
collection_static!(array_rev_static, Type::Array, "rev");
collection_static!(array_slice_static, Type::Array, "slice");
collection_static!(array_sorted_static, Type::Array, "sorted");
collection_static!(array_split_static, Type::Array, "split");
collection_static!(array_sum_static, Type::Array, "sum");
collection_static!(array_to_dict_static, Type::Array, "to-dict");
collection_static!(array_windows_static, Type::Array, "windows");
collection_static!(array_zip_static, Type::Array, "zip");

collection_static!(dict_at_static, Type::Dictionary, "at");
collection_static!(dict_filter_static, Type::Dictionary, "filter");
collection_static!(dict_insert_static, Type::Dictionary, "insert");
collection_static!(dict_keys_static, Type::Dictionary, "keys");
collection_static!(dict_len_static, Type::Dictionary, "len");
collection_static!(dict_map_static, Type::Dictionary, "map");
collection_static!(dict_pairs_static, Type::Dictionary, "pairs");
collection_static!(dict_remove_static, Type::Dictionary, "remove");
collection_static!(dict_values_static, Type::Dictionary, "values");

collection_static!(str_at_static, Type::Str, "at");
collection_static!(str_clusters_static, Type::Str, "clusters");
collection_static!(str_codepoints_static, Type::Str, "codepoints");
collection_static!(str_contains_static, Type::Str, "contains");
collection_static!(str_ends_with_static, Type::Str, "ends-with");
collection_static!(str_find_static, Type::Str, "find");
collection_static!(str_first_static, Type::Str, "first");
collection_static!(str_last_static, Type::Str, "last");
collection_static!(str_len_static, Type::Str, "len");
collection_static!(str_match_static, Type::Str, "match");
collection_static!(str_matches_static, Type::Str, "matches");
collection_static!(str_normalize_static, Type::Str, "normalize");
collection_static!(str_position_static, Type::Str, "position");
collection_static!(str_replace_static, Type::Str, "replace");
collection_static!(str_rev_static, Type::Str, "rev");
collection_static!(str_slice_static, Type::Str, "slice");
collection_static!(str_split_static, Type::Str, "split");
collection_static!(str_starts_with_static, Type::Str, "starts-with");
collection_static!(str_to_unicode_static, Type::Str, "to-unicode");
collection_static!(str_trim_static, Type::Str, "trim");

collection_static!(bytes_at_static, Type::Bytes, "at");
collection_static!(bytes_len_static, Type::Bytes, "len");
collection_static!(bytes_slice_static, Type::Bytes, "slice");

collection_static!(args_at_static, Type::Arguments, "at");
collection_static!(args_filter_static, Type::Arguments, "filter");
collection_static!(args_len_static, Type::Arguments, "len");
collection_static!(args_map_static, Type::Arguments, "map");
collection_static!(args_named_static, Type::Arguments, "named");
collection_static!(args_pos_static, Type::Arguments, "pos");

fn native_array_all_static(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the arguments `self` and `test` are positional".to_string(),
        )]);
    }
    let [Value::Array(arr), pred] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.as_slice() {
                [] => "missing argument: self".to_string(),
                [Value::Array(_)] => "missing argument: test".to_string(),
                [other, ..] if !matches!(other, Value::Array(_)) => format!(
                    "expected array, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
                _ => "unexpected argument".to_string(),
            },
        )]);
    };
    let pred = expect_one_func(Args::positional(vec![pred.clone()]), "array.all()")?;
    array_all_with_pred(arr.clone(), pred, scopes, _ctx, engine)
}

fn native_str_clusters_static(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `self` is positional".to_string(),
        )]);
    }
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(str_clusters(s.clone())),
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: self".to_string(),
        )]),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "expected string, found {}",
                crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                    other
                )
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]),
    }
}

// ── array helpers ───────────────────────────────────────────────────────────

fn ensure_no_args(args: &Args) -> SourceResult<()> {
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    Ok(())
}

fn array_no_args(
    arr: Vec<Value>,
    args: Args,
    operation: fn(Vec<Value>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(arr))
}

fn dict_no_args(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    operation: fn(IndexMap<EcoString, Value, FxBuildHasher>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(dict))
}

fn str_no_args(
    value: EcoString,
    args: Args,
    operation: fn(EcoString) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(value))
}

fn optional_default(args: &Args) -> SourceResult<Option<Value>> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for name in args.named.keys() {
        if name != "default" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    Ok(args.named.get("default").cloned())
}

fn array_len(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Int(arr.len() as i64))
}

fn array_first(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    arr.first().cloned().or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "array is empty and no default value was specified",
        )]
    })
}

fn array_last(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    arr.last().cloned().or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "array is empty and no default value was specified",
        )]
    })
}

/// `array.at(index, default: value)` — P714. Paridade com o vanilla
/// (`foundations/array.rs:207-221`, `locate_opt`): índice negativo conta a
/// partir do fim (`len + index`); fora de limites usa `default` se
/// fornecido, senão erro (mesma mensagem do vanilla,
/// `out_of_bounds_no_default`).
fn array_at(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if args.named.len() > 1
        || (args.named.len() == 1 && !args.named.contains_key("default"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "default")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("array.at() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: index")]);
    }
    let default = args.named.get("default").cloned();
    let index = expect_one_int(args, "array.at()")?;
    let len = arr.len() as i64;
    let resolved = if index >= 0 { Some(index) } else { len.checked_add(index) };
    let value = resolved
        .filter(|&v| v >= 0 && v < len)
        .and_then(|v| arr.get(v as usize).cloned());
    match value {
        Some(v) => Ok(v),
        None => match default {
            Some(v) => Ok(v),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "array index out of bounds (index: {index}, len: {len}) \
                     and no default value was specified"
                ),
            )]),
        },
    }
}

fn array_rev(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().rev().collect())
}

/// `array.slice(start, end?, count:)` — P730. Paridade com o vanilla
/// (`foundations/array.rs:279-300`, sobre `locate(index, end_ok: true)`
/// de `array.rs:122-137`): índices negativos contam a partir do fim
/// (`len + index`, `checked_add`); `start`/`end` efetivos admitem
/// `index == len`; `count:` equivale a `end = start_resolvido + count`
/// (mutuamente exclusivo com `end`); `end < start` → sub-array vazio
/// (clamp `max(start)`); fora de limites → erro com a mensagem exacta
/// do vanilla (`out_of_bounds`, com o índice original).
/// Medido: `(1,2,3,4).slice(1, 3)` → `(2, 3)`; `.slice(-2)` → `(3, 4)`;
/// `.slice(0, count: 2)` → `(1, 2)`; `.slice(1, count: -1)` → `()`.
fn array_slice(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "array.slice(): start espera int".to_string(),
        )]
    })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "`end` and `count` are mutually exclusive".to_string(),
        )]);
    }

    let len = arr.len() as i64;
    // `locate(index, end_ok: true)` do vanilla: negativo → len + index;
    // admite index == len; o erro reporta o índice original.
    let locate = |index: i64| -> Result<usize, Vec<SourceDiagnostic>> {
        let wrapped = if index >= 0 { Some(index) } else { len.checked_add(index) };
        wrapped
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v <= len as usize)
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("array index out of bounds (index: {index}, len: {len})"),
                )]
            })
    };

    let start_idx = locate(start)?;
    let end_raw = end.or(count.map(|c| start_idx as i64 + c)).unwrap_or(len);
    let end_idx = locate(end_raw)?.max(start_idx);
    Ok(Value::Array(arr[start_idx..end_idx].to_vec()))
}

fn array_sum(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    if arr.is_empty() {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "cannot sum an empty array without a default value",
            )]
        });
    }
    let mut sum_int = 0i64;
    let mut sum_float = 0.0f64;
    let mut has_float = false;
    for v in arr {
        match v {
            Value::Int(i) => {
                if has_float {
                    sum_float += i as f64;
                } else {
                    sum_int = sum_int.checked_add(i).ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            Span::detached(),
                            "array.sum(): overflow de inteiro".to_string(),
                        )]
                    })?;
                }
            }
            Value::Float(f) => {
                sum_float += f + sum_int as f64;
                sum_int = 0;
                has_float = true;
            }
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("array.sum() não suporta valor {}", v.type_name()),
                )]);
            }
        }
    }
    Ok(if has_float { Value::Float(sum_float) } else { Value::Int(sum_int) })
}

fn array_product(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    if arr.is_empty() {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "cannot calculate the product of an empty array without a default value",
            )]
        });
    }
    let mut product = Value::Int(1);
    for value in arr {
        product = crate::compiler::eval::operators::eval_binary_op(
            crate::entities::ast::expr::BinOp::Mul,
            product,
            value,
        )
        .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])?;
    }
    Ok(product)
}

fn array_contains(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: value"
            } else {
                "unexpected argument"
            },
        )]);
    }
    Ok(Value::Bool(arr.iter().any(|item| item == &args.items[0])))
}

fn array_sorted(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    let mut unknown = args.named.keys().filter(|k| !matches!(k.as_str(), "key" | "by"));
    if let Some(bad) = unknown.next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {}", bad.as_str()),
        )]);
    }

    let callable = |name: &str| -> SourceResult<Option<Func>> {
        match args.named.get(name) {
            None => Ok(None),
            Some(Value::Func(func)) => Ok(Some(func.clone())),
            Some(Value::Type(ty)) => type_as_callable(*ty).map(Some).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected function, found {}", ty.name()),
                )]
            }),
            Some(other) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected function, found {}", other.type_name()),
            )]),
        }
    };
    let key = callable("key")?;
    let by = callable("by")?;

    let mut keyed = Vec::with_capacity(arr.len());
    for value in arr {
        let key_value = match &key {
            Some(func) => apply_func(
                func.clone(),
                Args::positional(vec![value.clone()]),
                scopes,
                ctx,
                engine,
            )?,
            None => value.clone(),
        };
        keyed.push((key_value, value));
    }

    if let Some(by) = by {
        // Stable insertion sort keeps callback evaluation deterministic and
        // lets evaluation errors escape without hiding them in `Ord` glue.
        for index in 1..keyed.len() {
            let mut cursor = index;
            while cursor > 0 {
                let result = apply_func(
                    by.clone(),
                    Args::positional(vec![
                        keyed[cursor].0.clone(),
                        keyed[cursor - 1].0.clone(),
                    ]),
                    scopes,
                    ctx,
                    engine,
                )?;
                match result {
                    Value::Bool(true) => keyed.swap(cursor, cursor - 1),
                    Value::Bool(false) => break,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                crate::compiler::eval::operators::error_formatting::vanilla_type_name(&other)
                            ),
                        )])
                    }
                }
                cursor -= 1;
            }
        }
    } else {
        let mut cmp_err: Option<EcoString> = None;
        keyed.sort_by(|a, b| match value_cmp(&a.0, &b.0) {
            Ok(ordering) => ordering,
            Err(error) => {
                cmp_err = Some(error);
                Ordering::Equal
            }
        });
        if let Some(error) = cmp_err {
            return Err(vec![SourceDiagnostic::error(args.span, error)]);
        }
    }

    Ok(Value::Array(keyed.into_iter().map(|(_, value)| value).collect()))
}

fn array_filter(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.filter()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let result = apply_func(
            pred.clone(),
            Args::positional(vec![v.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match result {
            Value::Bool(true) => out.push(v),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Array(out))
}

fn array_map(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "array.map()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let mapped =
            apply_func(func.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        out.push(mapped);
    }
    Ok(Value::Array(out))
}

fn array_find(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.find()")?;
    for v in arr {
        let result = apply_func(
            pred.clone(),
            Args::positional(vec![v.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match result {
            Value::Bool(true) => return Ok(v),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::None)
}

fn array_position(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.position()")?;
    for (index, value) in arr.into_iter().enumerate() {
        let result =
            apply_func(pred.clone(), Args::positional(vec![value]), scopes, ctx, engine)?;
        match result {
            Value::Bool(true) => return Ok(Value::Int(index as i64)),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::None)
}

fn array_any(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.any()")?;
    for v in arr {
        let result =
            apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        match result {
            Value::Bool(true) => return Ok(Value::Bool(true)),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Bool(false))
}

fn array_all(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.all()")?;
    array_all_with_pred(arr, pred, scopes, ctx, engine)
}

fn array_all_with_pred(
    arr: Vec<Value>,
    pred: Func,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    for v in arr {
        let result =
            apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        match result {
            Value::Bool(false) => return Ok(Value::Bool(false)),
            Value::Bool(true) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other,
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Bool(true))
}

fn array_zip(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for name in args.named.keys() {
        if name != "exact" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let exact = match args.named.get("exact") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let mut arrays = Vec::with_capacity(args.items.len() + 1);
    arrays.push(arr);
    for value in args.items {
        match value {
            Value::Array(array) => arrays.push(array),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected array, found {}", other.type_name()),
                )])
            }
        }
    }
    let first_len = arrays[0].len();
    if exact && arrays.iter().any(|array| array.len() != first_len) {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "array lengths differ with exact: true",
        )]);
    }
    let len = arrays.iter().map(Vec::len).min().unwrap_or(0);
    Ok(Value::Array(
        (0..len)
            .map(|index| {
                Value::Array(arrays.iter().map(|array| array[index].clone()).collect())
            })
            .collect(),
    ))
}

fn array_enumerate(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let start = match args.items.first() {
        None => 0,
        Some(Value::Int(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", other.type_name()),
            )])
        }
    };
    Ok(Value::Array(
        arr.into_iter()
            .enumerate()
            .map(|(i, v)| Value::Array(vec![Value::Int(start + i as i64), v]))
            .collect(),
    ))
}

fn array_dedup(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if !args.items.is_empty() || args.named.keys().any(|name| name.as_str() != "key") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let key = match args.named.get("key") {
        None => None,
        Some(Value::Func(func)) => Some(func.clone()),
        Some(Value::Type(t)) => type_as_callable(*t),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected function, found {}", other.type_name()),
            )])
        }
    };
    if key.is_none() {
        return Ok(array_dedup_values(arr));
    }
    let mut result = Vec::new();
    let mut keys = Vec::new();
    for item in arr {
        let item_key = if let Some(func) = &key {
            apply_func(
                func.clone(),
                Args::positional(vec![item.clone()]),
                scopes,
                ctx,
                engine,
            )?
        } else {
            item.clone()
        };
        if !keys.contains(&item_key) {
            keys.push(item_key);
            result.push(item);
        }
    }
    Ok(Value::Array(result))
}

fn array_dedup_values(arr: Vec<Value>) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    Value::Array(result)
}

fn array_chunks(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for name in args.named.keys() {
        if name != "exact" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let exact = match args.named.get("exact") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let n = expect_one_int(args, "array.chunks()")?;
    if n <= 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "array.chunks() requer inteiro positivo".to_string(),
        )]);
    }
    let n = n as usize;
    let chunks: Vec<Value> = arr
        .chunks(n)
        .filter(|chunk| !exact || chunk.len() == n)
        .map(|c| Value::Array(c.to_vec()))
        .collect();
    Ok(Value::Array(chunks))
}

fn array_windows(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "array.windows()")?;
    if n <= 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "array.windows() requer inteiro positivo".to_string(),
        )]);
    }
    let n = n as usize;
    let windows: Vec<Value> = arr.windows(n).map(|w| Value::Array(w.to_vec())).collect();
    Ok(Value::Array(windows))
}

fn array_flatten(arr: Vec<Value>) -> Value {
    fn push_flat(value: Value, output: &mut Vec<Value>) {
        match value {
            Value::Array(inner) => {
                for value in inner {
                    push_flat(value, output);
                }
            }
            other => output.push(other),
        }
    }
    let mut result = Vec::new();
    for item in arr {
        push_flat(item, &mut result);
    }
    Value::Array(result)
}

/// **P843 (#60)** — `array.join(separator?, last:?, default:?)` — paridade
/// vanilla `Array::join` (foundations/array.rs:754-786), medida em
/// `temp/p843/join*.typ`: separador posicional opcional (default `none`);
/// `last:` separador alternativo antes do último elemento; `default:`
/// devolvido para array vazio (vazio sem default → `none`). A concatenação
/// usa a op `join` da linguagem (mesma de code blocks), incluindo o erro
/// `cannot join X with Y` para tipos incompatíveis.
fn array_join(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if key.as_str() != "last" && key.as_str() != "default" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {}", key.as_str()),
            )]);
        }
    }
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    let span = args.span;
    let separator = args.items.into_iter().next().unwrap_or(Value::None);
    let default = args.named.get("default").cloned();
    let mut last = args.named.get("last").cloned();

    let len = arr.len();
    if len == 0 {
        return Ok(default.unwrap_or(Value::None));
    }

    let mut result = Value::None;
    for (i, value) in arr.into_iter().enumerate() {
        if i > 0 {
            let sep = if i + 1 == len && last.is_some() {
                last.take().unwrap()
            } else {
                separator.clone()
            };
            result = crate::compiler::eval::operators::join(result, sep)
                .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
        }
        result = crate::compiler::eval::operators::join(result, value)
            .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
    }
    Ok(result)
}

fn array_fold(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut items = args.items.into_iter();
    let start = items.next().unwrap_or(Value::None);
    let reducer = match items.next() {
        Some(Value::Func(f)) => f,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "array.fold() espera função como reducer, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "array.fold() requer start e reducer como argumentos posicionais"
                    .to_string(),
            )]);
        }
    };
    let mut acc = start;
    for item in arr {
        acc = apply_func(
            reducer.clone(),
            Args::positional(vec![acc, item]),
            scopes,
            ctx,
            engine,
        )?;
    }
    Ok(acc)
}

fn array_reduce(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let reducer = expect_one_func(args, "array.reduce()")?;
    let mut values = arr.into_iter();
    let Some(mut acc) = values.next() else {
        return Ok(Value::None);
    };
    for item in values {
        acc = apply_func(
            reducer.clone(),
            Args::positional(vec![acc, item]),
            scopes,
            ctx,
            engine,
        )?;
    }
    Ok(acc)
}

fn array_intersperse(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: separator"
            } else {
                "unexpected argument"
            },
        )]);
    }
    let separator = args.items[0].clone();
    let mut output = Vec::with_capacity(arr.len().saturating_mul(2).saturating_sub(1));
    for (index, value) in arr.into_iter().enumerate() {
        if index > 0 {
            output.push(separator.clone());
        }
        output.push(value);
    }
    Ok(Value::Array(output))
}

fn array_split(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: at"
            } else {
                "unexpected argument"
            },
        )]);
    }
    let separator = &args.items[0];
    let mut groups = Vec::new();
    let mut current = Vec::new();
    for value in arr {
        if &value == separator {
            groups.push(Value::Array(std::mem::take(&mut current)));
        } else {
            current.push(value);
        }
    }
    groups.push(Value::Array(current));
    Ok(Value::Array(groups))
}

fn array_to_dict(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    let mut output = IndexMap::with_hasher(FxBuildHasher);
    for entry in arr {
        match entry {
            Value::Array(pair) if pair.len() == 2 => match pair.as_slice() {
                [Value::Str(key), value] => {
                    output.insert(key.clone(), value.clone());
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "expected pair with string key",
                    )])
                }
            },
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "expected array of pairs",
                )])
            }
        }
    }
    Ok(Value::Dict(output))
}

// ── dict helpers ────────────────────────────────────────────────────────────

fn dict_keys(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_keys().map(Value::Str).collect())
}

fn dict_values(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_values().collect())
}

fn dict_pairs(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(
        dict.into_iter()
            .map(|(k, v)| Value::Array(vec![Value::Str(k), v]))
            .collect(),
    )
}

fn dict_remove(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let key = expect_one_str(args.clone(), "dict.remove()")?;
    dict.shift_remove(&key).or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            format!("dictionary does not contain key \"{key}\""),
        )]
    })
}

fn dict_update(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let other = expect_one_dict(args, "dict.update()")?;
    dict.extend(other);
    Ok(Value::Dict(dict))
}

/// `dict.at(key, default: value)` — P491.
/// Retorna o valor associado a `key` ou `default` se a chave não existir.
fn dict_at(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    // Validar named args antes de consumir `args`.
    if args.named.len() > 1
        || (args.named.len() == 1 && !args.named.contains_key("default"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "default")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("dict.at() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }
    let default = args.named.get("default").cloned();
    let key = expect_one_str(args, "dict.at()")?;
    match dict.get(&key) {
        Some(value) => Ok(value.clone()),
        None => match default {
            Some(value) => Ok(value),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("dicionário não contém a chave {:?}", key.as_str()),
            )]),
        },
    }
}

// ── str helpers ─────────────────────────────────────────────────────────────

fn str_len(s: EcoString) -> Value {
    // Paridade vanilla (P690): `str.len()` conta bytes UTF-8, não chars.
    Value::Int(s.len() as i64)
}

fn str_first(s: EcoString, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    s.chars()
        .next()
        .map(|c| Value::Str(c.to_string().into()))
        .or(default)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "string is empty and no default value was specified",
            )]
        })
}

fn str_last(s: EcoString, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    s.chars()
        .last()
        .map(|c| Value::Str(c.to_string().into()))
        .or(default)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "string is empty and no default value was specified",
            )]
        })
}

fn str_at(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P690): índice em bytes. Negativo conta bytes do fim.
    // Erro se fora de limites ou se o índice não for fronteira de carácter.
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let index = expect_one_int(args.clone(), "str.at()")?;
    let st = s.as_str();
    let len = st.len() as i64;
    let idx = if index < 0 { len + index } else { index };
    if idx < 0 || idx >= len {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!("str.at(): índice fora de limites (índice {index}, len {len})"),
            )]
        });
    }
    let i = idx as usize;
    if !st.is_char_boundary(i) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.at(): índice {index} não é uma fronteira de carácter"),
        )]);
    }
    let ch = st[i..].chars().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "str.at(): índice fora de limites".to_string(),
        )]
    })?;
    Ok(Value::Str(ch.to_string().into()))
}

fn str_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P690): índices em bytes. Sem clamp — índice fora de
    // limites ou fora de fronteira de carácter é erro. `start > end` → "".
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "str.slice(): start espera int".to_string(),
        )]
    })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.slice(): não pode especificar end e count simultaneamente".to_string(),
        )]);
    }

    let st = s.as_str();
    let len = st.len() as i64;
    let resolve = |x: i64| if x < 0 { len + x } else { x };
    let start_idx = resolve(start);
    let end_idx = match (end, count) {
        (Some(e), None) => resolve(e),
        (None, Some(c)) => start_idx + c.max(0),
        (None, None) => len,
        (Some(_), Some(_)) => unreachable!("end e count simultâneos já rejeitados acima"),
    };

    if start_idx < 0 || start_idx > len {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.slice(): índice fora de limites (start {start}, len {len})"),
        )]);
    }
    if end_idx < 0 || end_idx > len {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.slice(): índice fora de limites (end, len {len})"),
        )]);
    }
    let si = start_idx as usize;
    let ei = end_idx as usize;
    if !st.is_char_boundary(si) || !st.is_char_boundary(ei) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.slice(): índice não é uma fronteira de carácter".to_string(),
        )]);
    }
    if si > ei {
        return Ok(Value::Str(EcoString::new()));
    }
    Ok(Value::Str(st[si..ei].into()))
}

/// **P690** — `str.char-len()`: número de chars (codepoints). Extensão cristalina
/// não-portável; preserva a convenção pré-P690 de `str.len()`.
fn char_len(s: EcoString) -> Value {
    Value::Int(s.chars().count() as i64)
}

/// **P690** — `str.char-at(index)`: char no índice em chars. Extensão cristalina
/// não-portável; preserva a convenção pré-P690 de `str.at()`.
fn char_at(s: EcoString, args: Args) -> SourceResult<Value> {
    let index = expect_one_int(args, "str.char-at()")?;
    let chars: Vec<char> = s.chars().collect();
    let idx =
        if index < 0 { (chars.len() as i64 + index) as usize } else { index as usize };
    if idx >= chars.len() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.char-at(): índice fora de limites".to_string(),
        )]);
    }
    Ok(Value::Str(chars[idx].to_string().into()))
}

/// **P690** — `str.char-slice(start, end?, count?)`: substring em chars (clamp aos
/// limites). Extensão cristalina não-portável; preserva a convenção pré-P690 de
/// `str.slice()`.
fn char_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "str.char-slice(): start espera int".to_string(),
        )]
    })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.char-slice(): não pode especificar end e count simultaneamente"
                .to_string(),
        )]);
    }

    let chars: Vec<char> = s.chars().collect();
    let start_idx =
        if start < 0 { (chars.len() as i64 + start) as usize } else { start as usize };
    let start_idx = start_idx.min(chars.len());
    let end_idx = match (end, count) {
        (Some(e), None) => {
            let e = if e < 0 { (chars.len() as i64 + e) as usize } else { e as usize };
            e.min(chars.len())
        }
        (None, Some(c)) => (start_idx + c.max(0) as usize).min(chars.len()),
        (None, None) => chars.len(),
        (Some(_), Some(_)) => unreachable!("end e count simultâneos já rejeitados acima"),
    };
    let result: String = chars[start_idx..end_idx.max(start_idx)].iter().collect();
    Ok(Value::Str(result.into()))
}

fn str_clusters(s: EcoString) -> Value {
    Value::Array(s.graphemes(true).map(|g| Value::Str(g.into())).collect())
}

/// **P689** — `str.codepoints()`: array de strings, um por char (scalar value).
/// Distinto de `clusters`: um item por scalar Unicode, inclusive quando vários
/// scalars formam um único grapheme cluster.
fn str_codepoints(s: EcoString) -> Value {
    Value::Array(s.chars().map(|c| Value::Str(c.to_string().into())).collect())
}

/// **P689** — `str.position(hay)`: índice em **bytes** da primeira ocorrência,
/// ou `none`. Aceita `str` (substring literal) ou `regex` (primeiro match).
fn str_position(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(sub)] => Ok(s
            .find(sub.as_str())
            .map(|i| Value::Int(i as i64))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| Value::Int(m.start as i64))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.position() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.position() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// Constrói o dict `{start, end, text, captures}` de um match (índices em bytes).
/// Partilhado por `str.match` (P689) e `str.matches` (P692).
fn match_dict(
    start: usize,
    end: usize,
    text: &str,
    captures: Vec<Option<String>>,
) -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("start".into(), Value::Int(start as i64));
    dict.insert("end".into(), Value::Int(end as i64));
    dict.insert("text".into(), Value::Str(text.to_string().into()));
    dict.insert(
        "captures".into(),
        Value::Array(
            captures
                .into_iter()
                .map(|c| match c {
                    Some(s) => Value::Str(s.into()),
                    None => Value::None,
                })
                .collect(),
        ),
    );
    Value::Dict(dict)
}

/// **P689** — `str.match(pattern)`: primeiro match, como dict
/// `{start, end, text, captures}` com índices em **bytes**, ou `none`.
/// Aceita `str` (literal; `captures` vazio) ou `regex` (capturas em ordem
/// posicional, grupos nomeados inclusive). (**P693**: alargado de `regex`-only
/// para `str | regex`, espelhando `str_matches`.)
fn str_match(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(pat)] => Ok(s
            .match_indices(pat.as_str())
            .next()
            .map(|(i, m)| match_dict(i, i + m.len(), m, vec![]))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| match_dict(m.start, m.end, &m.text, m.captures))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.match() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.match() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// **P692** — `str.matches(pattern)`: array de dicts `{start, end, text, captures}`
/// (índices em **bytes**), um por ocorrência não sobreposta; `[]` se nenhuma.
/// Aceita `str` (literal, via `match_indices`) ou `regex` (via `captures_all`).
fn str_matches(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(pat)] => {
            let arr: Vec<Value> = s
                .match_indices(pat.as_str())
                .map(|(i, m)| match_dict(i, i + m.len(), m, vec![]))
                .collect();
            Ok(Value::Array(arr))
        }
        [Value::Regex(re)] => {
            let arr: Vec<Value> = re
                .captures_all(s.as_str())
                .into_iter()
                .map(|m| match_dict(m.start, m.end, &m.text, m.captures))
                .collect();
            Ok(Value::Array(arr))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.matches() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.matches() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// **P692** — `str.normalize(form:)`: normalização Unicode. `form` (named) ∈
/// {"nfc", "nfd", "nfkc", "nfkd"}; default "nfc". Não aceita argumentos posicionais.
fn str_normalize(s: EcoString, args: Args) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.normalize(): não aceita argumentos posicionais (use form:)".to_string(),
        )]);
    }
    let form = match args.named.get("form") {
        None => "nfc".to_string(),
        Some(Value::Str(f)) => f.to_string(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "str.normalize(): form espera str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    let normalized: String = match form.as_str() {
        "nfc" => s.nfc().collect(),
        "nfd" => s.nfd().collect(),
        "nfkc" => s.nfkc().collect(),
        "nfkd" => s.nfkd().collect(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "str.normalize(): forma desconhecida '{other}' (esperado nfc, nfd, nfkc, nfkd)"
                ),
            )])
        }
    };
    Ok(Value::Str(normalized.into()))
}

fn str_contains(s: EcoString, args: Args) -> SourceResult<Value> {
    let substr = expect_one_str(args, "str.contains()")?;
    Ok(Value::Bool(s.contains(substr.as_str())))
}

fn str_starts_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let prefix = expect_one_str(args, "str.starts-with()")?;
    Ok(Value::Bool(s.starts_with(prefix.as_str())))
}

fn str_ends_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let suffix = expect_one_str(args, "str.ends-with()")?;
    Ok(Value::Bool(s.ends_with(suffix.as_str())))
}

fn str_find(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P691): devolve a **substring** encontrada (texto do
    // primeiro match), ou `none`. Aceita `str` (substring literal) ou `regex`
    // (texto do match). O índice da ocorrência continua disponível via
    // `str.position()`.
    match args.items.as_slice() {
        [Value::Str(sub)] => Ok(s
            .find(sub.as_str())
            .map(|i| Value::Str(s[i..i + sub.len()].to_string().into()))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| Value::Str(m.text.into()))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.find() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.find() requer 1 argumento posicional".to_string(),
        )]),
    }
}

fn str_replace(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "count") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let count = match args.named.get("count") {
        None => None,
        Some(Value::Int(count)) if *count >= 0 => Some(*count as usize),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected non-negative integer, found {}", other.type_name()),
            )])
        }
    };
    let (old, new) = expect_two_str(args, "str.replace()")?;
    let replaced = match count {
        Some(count) => s.replacen(old.as_str(), new.as_str(), count),
        None => s.replace(old.as_str(), new.as_str()),
    };
    Ok(Value::Str(replaced.into()))
}

fn str_trim(s: EcoString) -> Value {
    Value::Str(s.trim().into())
}

fn str_trim_dispatch(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for name in args.named.keys() {
        if !matches!(name.as_str(), "at" | "repeat") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }

    let repeat = match args.named.get("repeat") {
        None | Some(Value::Bool(true)) => true,
        Some(Value::Bool(false)) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let (trim_start, trim_end) = match args.named.get("at") {
        None | Some(Value::None) => (true, true),
        Some(Value::Align(align)) => {
            use crate::entities::layout_types::HAlign;
            match (align.h, align.v) {
                (Some(HAlign::Start), None) => (true, false),
                (Some(HAlign::End), None) => (false, true),
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "expected start or end alignment",
                    )])
                }
            }
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected alignment, found {}", other.type_name()),
            )])
        }
    };

    let Some(pattern) = args.items.first() else {
        let result = match (trim_start, trim_end) {
            (true, true) => s.trim(),
            (true, false) => s.trim_start(),
            (false, true) => s.trim_end(),
            (false, false) => s.as_str(),
        };
        return Ok(Value::Str(result.into()));
    };
    let Value::Str(pattern) = pattern else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("expected string, found {}", pattern.type_name()),
        )]);
    };
    if pattern.is_empty() {
        return Ok(Value::Str(s));
    }

    let mut result = s.as_str();
    if trim_start {
        result = if repeat {
            result.trim_start_matches(pattern.as_str())
        } else {
            result.strip_prefix(pattern.as_str()).unwrap_or(result)
        };
    }
    if trim_end {
        result = if repeat {
            result.trim_end_matches(pattern.as_str())
        } else {
            result.strip_suffix(pattern.as_str()).unwrap_or(result)
        };
    }
    Ok(Value::Str(result.into()))
}

fn str_split(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.items.is_empty() && args.named.is_empty() {
        return Ok(Value::Array(vec![Value::Str(s)]));
    }
    let sep = expect_one_str(args, "str.split()")?;
    Ok(Value::Array(s.split(sep.as_str()).map(|part| Value::Str(part.into())).collect()))
}

fn str_repeat(s: EcoString, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "str.repeat()")?;
    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.repeat() requer inteiro não-negativo".to_string(),
        )]);
    }
    Ok(Value::Str(s.repeat(n as usize).into()))
}

fn str_to_upper(s: EcoString) -> Value {
    Value::Str(s.to_uppercase().into())
}

fn str_to_lower(s: EcoString) -> Value {
    Value::Str(s.to_lowercase().into())
}

fn str_to_unicode(s: EcoString) -> SourceResult<Value> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(character), None) => Ok(Value::Int(character as i64)),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected exactly one character",
        )]),
    }
}

fn str_rev(s: EcoString) -> Value {
    Value::Str(s.chars().rev().collect::<String>().into())
}

fn dict_insert(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let (key, value) = expect_str_value(args, "dict.insert()")?;
    dict.insert(key, value);
    Ok(Value::Dict(dict))
}

fn dict_len(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Int(dict.len() as i64)
}

/// `dict.map(v => ...)` — aplica função a cada valor e preserva a chave.
fn dict_map(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "dict.map()")?;
    let mut result = IndexMap::with_capacity_and_hasher(dict.len(), FxBuildHasher);
    for (key, value) in dict {
        let mapped =
            apply_func(func.clone(), Args::positional(vec![value]), scopes, ctx, engine)?;
        result.insert(key, mapped);
    }
    Ok(Value::Dict(result))
}

/// `dict.filter(v => bool)` — mantém pares cujo valor satisfaz o predicado.
fn dict_filter(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "dict.filter()")?;
    let mut result = IndexMap::with_capacity_and_hasher(dict.len(), FxBuildHasher);
    for (key, value) in dict {
        let keep = apply_func(
            func.clone(),
            Args::positional(vec![value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match keep {
            Value::Bool(true) => {
                result.insert(key, value);
            }
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Dict(result))
}

// ── argument helpers ────────────────────────────────────────────────────────

fn arguments_len(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Int((arguments.items.len() + arguments.named.len()) as i64))
}

fn arguments_pos(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Array(arguments.items))
}

fn arguments_named(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Dict(arguments.named))
}

fn arguments_at(arguments: Args, args: Args) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let Some(key) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: key")]);
    };
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let value = match key {
        Value::Int(index) => {
            let len = arguments.items.len() as i64;
            let resolved =
                if *index >= 0 { Some(*index) } else { len.checked_add(*index) };
            resolved
                .filter(|index| *index >= 0 && *index < len)
                .and_then(|index| arguments.items.get(index as usize).cloned())
        }
        Value::Str(name) => arguments.named.get(name).cloned(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer or string, found {}", other.type_name()),
            )])
        }
    };
    value.or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "argument key is invalid and no default value was specified",
        )]
    })
}

fn arguments_filter(
    arguments: Args,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let test = expect_one_func(args, "arguments.filter()")?;
    let mut items = Vec::new();
    let mut named = IndexMap::with_hasher(FxBuildHasher);
    for value in arguments.items {
        let keep = apply_func(
            test.clone(),
            Args::positional(vec![value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match keep {
            Value::Bool(true) => items.push(value),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("expected boolean, found {}", other.type_name()),
                )])
            }
        }
    }
    for (name, value) in arguments.named {
        let keep = apply_func(
            test.clone(),
            Args::positional(vec![value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match keep {
            Value::Bool(true) => {
                named.insert(name, value);
            }
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("expected boolean, found {}", other.type_name()),
                )])
            }
        }
    }
    Ok(Value::Args(Args { items, named, span: arguments.span }))
}

fn arguments_map(
    arguments: Args,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mapper = expect_one_func(args, "arguments.map()")?;
    let mut items = Vec::with_capacity(arguments.items.len());
    for value in arguments.items {
        items.push(apply_func(
            mapper.clone(),
            Args::positional(vec![value]),
            scopes,
            ctx,
            engine,
        )?);
    }
    let mut named =
        IndexMap::with_capacity_and_hasher(arguments.named.len(), FxBuildHasher);
    for (name, value) in arguments.named {
        named.insert(
            name,
            apply_func(
                mapper.clone(),
                Args::positional(vec![value]),
                scopes,
                ctx,
                engine,
            )?,
        );
    }
    Ok(Value::Args(Args { items, named, span: arguments.span }))
}

fn expect_str_value(args: Args, context: &str) -> SourceResult<(EcoString, Value)> {
    match args.items.as_slice() {
        [Value::Str(key), value] => Ok((key.clone(), value.clone())),
        [other, ..] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: primeiro argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 2 argumentos posicionais", context),
        )]),
    }
}

/// P881 — converte um tipo chamável no construtor nativo correspondente,
/// permitindo `(0, 1).map(str)`, `(1, 2).map(int)`, etc., como no vanilla.
fn type_as_callable(t: Type) -> Option<Func> {
    Some(match t {
        Type::Int => Func::native("int", native_int),
        Type::Float => Func::native("float", native_float),
        Type::Str => Func::native("str", native_str),
        Type::Type => Func::native("type", native_type),
        Type::Counter => Func::native("counter", native_counter),
        Type::State => Func::native("state", native_state),
        Type::Symbol => Func::native("symbol", native_symbol),
        Type::Bytes => Func::native("bytes", native_bytes),
        Type::Datetime => Func::native("datetime", native_datetime),
        _ => return None, // neutro: N16[β] — Value sem representação iterável retorna None
    })
}

fn expect_one_func(args: Args, context: &str) -> SourceResult<Func> {
    let parameter = match context {
        "array.all()" | "array.any()" | "array.filter()" => "test",
        "array.find()" | "array.position()" => "searcher",
        "array.map()" => "mapper",
        "dict.map()" | "dict.filter()" | "arguments.map()" | "arguments.filter()" => {
            "mapper"
        }
        _ => "function",
    };
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("the argument `{parameter}` is positional"),
        )]);
    }
    match args.items.as_slice() {
        [Value::Func(f)] => Ok(f.clone()),
        [Value::Type(t)] => type_as_callable(*t).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{} espera função, recebeu {}", context, t.name()),
            )]
        }),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera função, recebeu {}", context, other.type_name()),
        )]),
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("missing argument: {parameter}"),
        )]),
        _ => Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    }
}

fn expect_one_array(args: Args, context: &str) -> SourceResult<Vec<Value>> {
    match args.items.as_slice() {
        [Value::Array(a)] => Ok(a.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera array, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

fn expect_one_dict(
    args: Args,
    context: &str,
) -> SourceResult<IndexMap<EcoString, Value, FxBuildHasher>> {
    match args.items.as_slice() {
        [Value::Dict(d)] => Ok(d.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera dicionário, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

fn expect_one_str(args: Args, context: &str) -> SourceResult<EcoString> {
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(s.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera string, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

fn expect_two_str(args: Args, context: &str) -> SourceResult<(EcoString, EcoString)> {
    match args.items.as_slice() {
        [Value::Str(a), Value::Str(b)] => Ok((a.clone(), b.clone())),
        [Value::Str(_), other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: segundo argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        [other, ..] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: primeiro argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 2 argumentos posicionais", context),
        )]),
    }
}

fn expect_one_int(args: Args, context: &str) -> SourceResult<i64> {
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(*i),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera inteiro, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

// ── value comparison (para sorted) ──────────────────────────────────────────

/// Comparação parcial entre valores, alinhada com o Typst vanilla para os
/// tipos primitivos suportados em P466: `int`, `float` e `str`.
///
/// **P652** — devolve `Err` quando os valores não são comparáveis (tipos
/// incompatíveis ou `NaN`), em vez de assumir `Ordering::Equal` em silêncio.
fn value_cmp(a: &Value, b: &Value) -> Result<Ordering, EcoString> {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => {
            a.partial_cmp(b).ok_or_else(|| "cannot compare integers".into())
        }
        (Value::Int(i), Value::Float(f)) => (*i as f64)
            .partial_cmp(f)
            .ok_or_else(|| "cannot compare integer and float".into()),
        (Value::Float(f), Value::Int(i)) => f
            .partial_cmp(&(*i as f64))
            .ok_or_else(|| "cannot compare float and integer".into()),
        (Value::Float(a), Value::Float(b)) => {
            a.partial_cmp(b).ok_or_else(|| "cannot compare floats".into())
        }
        (Value::Str(a), Value::Str(b)) => Ok(a.cmp(b)),
        _ => Err(format!(
            "cannot compare {} and {}",
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(b),
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(a)
        )
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(items: Vec<Value>, named: Option<(&str, Value)>) -> Args {
        let mut args = Args {
            items,
            named: IndexMap::default(),
            span: Span::detached(),
        };
        if let Some((k, v)) = named {
            args.named.insert(k.into(), v);
        }
        args
    }

    // ── P1214 — bytes.len / at / slice ──────────────────────────────────────

    #[test]
    fn p1214_bytes_len_conta_octetos_utf8() {
        let bytes = crate::entities::bytes::Bytes::new("é".as_bytes().to_vec());
        assert_eq!(
            bytes_len(bytes, make_args(vec![], None), None).unwrap(),
            Value::Int(2)
        );
    }

    #[test]
    fn p1214_bytes_at_negativo_default_e_limite() {
        let bytes = crate::entities::bytes::Bytes::new(vec![0, 127, 255]);
        assert_eq!(
            bytes_at(bytes.clone(), make_args(vec![Value::Int(-1)], None), None,)
                .unwrap(),
            Value::Int(255)
        );
        assert_eq!(
            bytes_at(
                bytes.clone(),
                make_args(vec![Value::Int(3)], Some(("default", Value::Int(99)))),
                None,
            )
            .unwrap(),
            Value::Int(99)
        );
        let err =
            bytes_at(bytes, make_args(vec![Value::Int(3)], None), None).unwrap_err();
        assert_eq!(
            err[0].message,
            "byte index out of bounds (index: 3, len: 3) and no default value was specified"
        );
    }

    #[test]
    fn p1214_bytes_slice_preserva_conteudo_e_precedencia_end() {
        let bytes = crate::entities::bytes::Bytes::new(vec![1, 2, 3]);
        let mut args = make_args(vec![Value::Int(0), Value::Int(2)], None);
        args.named.insert("count".into(), Value::Int(1));
        let Value::Bytes(slice) = bytes_slice(bytes, args, None).unwrap() else {
            panic!("esperado bytes");
        };
        assert_eq!(slice.as_slice(), &[1, 2]);
    }

    #[test]
    fn p1214_bytes_nao_inventa_first_last() {
        let bytes = Value::Bytes(crate::entities::bytes::Bytes::new(vec![1, 2, 3]));
        let no_args = make_args(vec![], None);
        let Value::Bytes(bytes) = bytes else { unreachable!() };
        assert!(try_dispatch_bytes_method(bytes.clone(), "first", no_args.clone(), None)
            .is_none());
        assert!(try_dispatch_bytes_method(bytes, "last", no_args, None).is_none());
    }

    #[test]
    fn p495_dict_at_default_named() {
        let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        dict.insert("a".into(), Value::Int(1));
        dict.insert("b".into(), Value::Int(2));

        // key existente → valor
        let args = make_args(vec![Value::Str("a".into())], None);
        assert_eq!(dict_at(dict.clone(), args).unwrap(), Value::Int(1));

        // key ausente com default → default
        let args =
            make_args(vec![Value::Str("z".into())], Some(("default", Value::Int(99))));
        assert_eq!(dict_at(dict.clone(), args).unwrap(), Value::Int(99));

        // key ausente sem default → erro
        let args = make_args(vec![Value::Str("z".into())], None);
        assert!(dict_at(dict, args).is_err());
    }

    #[test]
    fn p495_dict_at_named_desconhecido_rejeitado() {
        let dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        let mut args = make_args(vec![Value::Str("a".into())], None);
        args.named.insert("foo".into(), Value::Int(1));
        assert!(dict_at(dict, args).is_err());
    }

    // ── P714 — array.at(index, default:) ──────────────────────────────────────

    #[test]
    fn p714_array_at_indice_positivo() {
        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(1)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(20));
    }

    #[test]
    fn p714_array_at_indice_negativo_conta_do_fim() {
        // Paridade `locate_opt`: -1 -> len + (-1) = último elemento.
        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(-1)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(30));

        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(-3)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(10));
    }

    #[test]
    fn p714_array_at_fora_de_limites_com_default() {
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(5)], Some(("default", Value::Int(99))));
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(99));

        // Negativo além do início também usa default.
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(-5)], Some(("default", Value::Int(-1))));
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(-1));
    }

    #[test]
    fn p714_array_at_fora_de_limites_sem_default_erra() {
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(5)], None);
        let err = array_at(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds")
                && err[0].message.contains("no default"),
            "mensagem inesperada: {:?}",
            err[0].message
        );
    }

    #[test]
    fn p714_array_at_named_desconhecido_rejeitado() {
        let arr = vec![Value::Int(1)];
        let mut args = make_args(vec![Value::Int(0)], None);
        args.named.insert("foo".into(), Value::Int(1));
        assert!(array_at(arr, args).is_err());
    }

    #[test]
    fn p714_array_at_vazio_sem_default_erra() {
        let arr: Vec<Value> = vec![];
        let args = make_args(vec![Value::Int(0)], None);
        assert!(array_at(arr, args).is_err());
    }

    // ── P730 — array.slice(start, end?, count:) ──────────────────────────────
    // Paridade vanilla `foundations/array.rs:279-300` (sobre
    // `locate(index, end_ok: true)`). Medições vanilla no corpo dos testes.

    fn ints(v: &[i64]) -> Vec<Value> {
        v.iter().map(|&i| Value::Int(i)).collect()
    }

    #[test]
    fn p730_array_slice_start_end() {
        // Vanilla: (1,2,3,4).slice(1, 3) → (2, 3)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1), Value::Int(3)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[2, 3])));
    }

    #[test]
    fn p730_array_slice_so_start() {
        // Vanilla: (1,2,3,4).slice(1) → (2, 3, 4)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[2, 3, 4])));
    }

    #[test]
    fn p730_array_slice_negativos_contam_do_fim() {
        // Vanilla: (1,2,3,4).slice(-2) → (3, 4); .slice(-3, -1) → (2, 3)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(-2)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[3, 4])));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(-3), Value::Int(-1)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[2, 3])));
    }

    #[test]
    fn p730_array_slice_count() {
        // Vanilla: (1,2,3,4).slice(0, count: 2) → (1, 2)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0)], Some(("count", Value::Int(2))));
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[1, 2])));
    }

    #[test]
    fn p730_array_slice_count_negativo_vazio() {
        // Vanilla: (1,2,3,4).slice(1, count: -1) → ()
        // (end efetivo = 1 + (-1) = 0 → locate ok → max(start) → [1..1])
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1)], Some(("count", Value::Int(-1))));
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p730_array_slice_end_menor_que_start_vazio() {
        // Vanilla: (1,2,3,4).slice(3, 1) → (); .slice(4) → (); .slice(0, 4) → tudo
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(3), Value::Int(1)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(4)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0), Value::Int(4)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[1, 2, 3, 4])));
    }

    #[test]
    fn p730_array_slice_array_vazio() {
        // Vanilla: ().slice(0) → ()
        let arr: Vec<Value> = vec![];
        let args = make_args(vec![Value::Int(0)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p730_array_slice_end_e_count_mutuamente_exclusivos() {
        // Vanilla: "`end` and `count` are mutually exclusive"
        let arr = ints(&[1, 2, 3, 4]);
        let args =
            make_args(vec![Value::Int(1), Value::Int(2)], Some(("count", Value::Int(2))));
        let err = array_slice(arr, args).unwrap_err();
        assert!(err[0].message.contains("mutually exclusive"), "msg: {}", err[0].message);
    }

    #[test]
    fn p730_array_slice_fora_de_limites_erra() {
        // Vanilla: "array index out of bounds (index: 10, len: 4)"
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(10)], None);
        let err = array_slice(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds (index: 10, len: 4)"),
            "msg: {}",
            err[0].message
        );
    }

    #[test]
    fn p730_array_slice_count_fora_de_limites_erra() {
        // Vanilla: (1,2,3,4).slice(0, count: 99) →
        // "array index out of bounds (index: 99, len: 4)"
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0)], Some(("count", Value::Int(99))));
        let err = array_slice(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds (index: 99, len: 4)"),
            "msg: {}",
            err[0].message
        );
    }

    // ── P496 — métodos estruturais de array ───────────────────────────────────

    #[test]
    fn p1284_array_dedup_mantem_primeira_ocorrencia_global() {
        let arr = vec![
            Value::Int(3),
            Value::Int(1),
            Value::Int(4),
            Value::Int(1),
            Value::Int(5),
            Value::Int(9),
            Value::Int(2),
            Value::Int(6),
        ];
        assert_eq!(
            array_dedup_values(arr),
            Value::Array(vec![
                Value::Int(3),
                Value::Int(1),
                Value::Int(4),
                Value::Int(5),
                Value::Int(9),
                Value::Int(2),
                Value::Int(6),
            ])
        );

        let arr2 = vec![
            Value::Int(1),
            Value::Int(1),
            Value::Int(2),
            Value::Int(2),
            Value::Int(2),
            Value::Int(3),
        ];
        assert_eq!(
            array_dedup_values(arr2),
            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn p496_array_chunks_divide_em_blocos() {
        let arr = vec![
            Value::Int(3),
            Value::Int(1),
            Value::Int(4),
            Value::Int(1),
            Value::Int(5),
            Value::Int(9),
            Value::Int(2),
            Value::Int(6),
        ];
        let result = array_chunks(arr, make_args(vec![Value::Int(3)], None)).unwrap();
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Int(3), Value::Int(1), Value::Int(4)]),
            Value::Array(vec![Value::Int(1), Value::Int(5), Value::Int(9)]),
            Value::Array(vec![Value::Int(2), Value::Int(6)]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn p496_array_windows_janelas_deslizantes() {
        let arr = vec![Value::Int(3), Value::Int(1), Value::Int(4), Value::Int(1)];
        let result = array_windows(arr, make_args(vec![Value::Int(3)], None)).unwrap();
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Int(3), Value::Int(1), Value::Int(4)]),
            Value::Array(vec![Value::Int(1), Value::Int(4), Value::Int(1)]),
        ]);
        assert_eq!(result, expected);
    }

    // ── P689 — str.codepoints / str.position / str.match ────────────────────

    #[test]
    fn p689_str_codepoints_chars() {
        assert_eq!(
            str_codepoints("abc".into()),
            Value::Array(vec![
                Value::Str("a".into()),
                Value::Str("b".into()),
                Value::Str("c".into()),
            ])
        );
        // multibyte (café precomposto) → 4 codepoints, paridade vanilla
        assert_eq!(
            str_codepoints("café".into()),
            Value::Array(vec![
                Value::Str("c".into()),
                Value::Str("a".into()),
                Value::Str("f".into()),
                Value::Str("é".into()),
            ])
        );
        assert_eq!(str_codepoints("".into()), Value::Array(vec![]));
    }

    #[test]
    fn p689_str_position_str_e_regex_byte_indices() {
        // str: índice em bytes; não encontrado → none
        let a = make_args(vec![Value::Str("b".into())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::Int(1));
        let a = make_args(vec![Value::Str("z".into())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::None);
        // multibyte: "xéy" — y no byte 3 (x=1, é=2)
        let a = make_args(vec![Value::Str("y".into())], None);
        assert_eq!(str_position("xéy".into(), a).unwrap(), Value::Int(3));
        // regex: paridade — também byte index
        let a = make_args(vec![Value::Regex(Regex::new("y").unwrap())], None);
        assert_eq!(str_position("xéy".into(), a).unwrap(), Value::Int(3));
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::None);
    }

    #[test]
    fn p1075_str_match_optional_group_returns_none() {
        // 1. Grupo opcional não participante devolve Value::None (P1075 / paridade vanilla)
        let a = make_args(vec![Value::Regex(Regex::new("a(x)?(b)").unwrap())], None);
        let d = match str_match("ab".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(
            d.get("captures"),
            Some(&Value::Array(vec![Value::None, Value::Str("b".into()),]))
        );

        // 2. Grupo nomeado opcional não participante
        let a =
            make_args(vec![Value::Regex(Regex::new(r"(?P<name>x)?(a)").unwrap())], None);
        let d = match str_match("a".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(
            d.get("captures"),
            Some(&Value::Array(vec![Value::None, Value::Str("a".into()),]))
        );

        // 3. str_matches com participantes e não participantes
        let a = make_args(vec![Value::Regex(Regex::new("a(x)?(b)").unwrap())], None);
        let arr = match str_matches("ab axb".into(), a).unwrap() {
            Value::Array(arr) => arr,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 2);
        assert_eq!(
            dict_get(&arr[0], "captures"),
            &Value::Array(vec![Value::None, Value::Str("b".into())])
        );
        assert_eq!(
            dict_get(&arr[1], "captures"),
            &Value::Array(vec![Value::Str("x".into()), Value::Str("b".into())])
        );
    }

    #[test]
    fn p689_str_match_dict_e_captures() {
        // sem match → none
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_match("abc".into(), a).unwrap(), Value::None);

        // match simples → dict {start, end, text, captures}
        let a = make_args(vec![Value::Regex(Regex::new("b").unwrap())], None);
        let d = match str_match("abc".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(d.get("start"), Some(&Value::Int(1)));
        assert_eq!(d.get("end"), Some(&Value::Int(2)));
        assert_eq!(d.get("text"), Some(&Value::Str("b".into())));
        assert_eq!(d.get("captures"), Some(&Value::Array(vec![])));

        // multibyte: "xéy" match é → start 1, end 3 (bytes)
        let a = make_args(vec![Value::Regex(Regex::new("é").unwrap())], None);
        let d = match str_match("xéy".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(d.get("start"), Some(&Value::Int(1)));
        assert_eq!(d.get("end"), Some(&Value::Int(3)));

        // captures posicionais (inclui grupos nomeados em ordem)
        let a = make_args(vec![Value::Regex(Regex::new("(a)(b)(c)").unwrap())], None);
        let d = match str_match("abc".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(
            d.get("captures"),
            Some(&Value::Array(vec![
                Value::Str("a".into()),
                Value::Str("b".into()),
                Value::Str("c".into()),
            ]))
        );
    }

    #[test]
    fn p689_str_position_match_tipo_errado() {
        // position com Int → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_position("abc".into(), a).is_err());
        // match com Int (tipo errado) → erro (P693: match aceita str | regex; Int continua a ser rejeitado)
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_match("abc".into(), a).is_err());
    }

    // ── P690 — str.len/at/slice em bytes; char-len/char-at/char-slice ─────────

    #[test]
    fn p690_str_len_bytes() {
        // ASCII: char == byte, sem efeito visível
        assert_eq!(str_len("abc".into()), Value::Int(3));
        assert_eq!(str_len("".into()), Value::Int(0));
        // multi-byte: "café" = 5 bytes (c-a-f-é, é ocupa 2)
        assert_eq!(str_len("café".into()), Value::Int(5));
        assert_eq!(str_len("éabc".into()), Value::Int(5));
    }

    #[test]
    fn p690_str_at_bytes() {
        // ASCII
        assert_eq!(
            str_at("abc".into(), make_args(vec![Value::Int(1)], None)).unwrap(),
            Value::Str("b".into())
        );
        // multi-byte: "éabc" (é=0-1, a=2, b=3, c=4) → byte 2 = 'a'
        assert_eq!(
            str_at("éabc".into(), make_args(vec![Value::Int(2)], None)).unwrap(),
            Value::Str("a".into())
        );
        assert_eq!(
            str_at("éabc".into(), make_args(vec![Value::Int(0)], None)).unwrap(),
            Value::Str("é".into())
        );
        // negativo conta bytes do fim: "café" (c=0,a=1,f=2,é=3-4), -1 → byte 4
        // (2º byte do 'é', non-boundary) → erro
        assert!(str_at("café".into(), make_args(vec![Value::Int(-1)], None)).is_err());
        // non-boundary: byte 1 de "éabc" (meio do 'é') → erro
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(1)], None)).is_err());
        // out of bounds: índice >= len bytes
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(10)], None)).is_err());
        // negativo além do início
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(-6)], None)).is_err());
    }

    #[test]
    fn p690_str_slice_bytes() {
        // ASCII
        assert_eq!(
            str_slice(
                "abcdef".into(),
                make_args(vec![Value::Int(1), Value::Int(4)], None)
            )
            .unwrap(),
            Value::Str("bcd".into())
        );
        // multi-byte: "éabc".slice(2,4) bytes = "ab"
        assert_eq!(
            str_slice("éabc".into(), make_args(vec![Value::Int(2), Value::Int(4)], None))
                .unwrap(),
            Value::Str("ab".into())
        );
        // count em bytes: "éabc".slice(0, count: 2) = "é"
        assert_eq!(
            str_slice(
                "éabc".into(),
                make_args(vec![Value::Int(0)], Some(("count", Value::Int(2))))
            )
            .unwrap(),
            Value::Str("é".into())
        );
        // negativo: "éabc".slice(-3, -1) → bytes 2..4 = "ab"
        assert_eq!(
            str_slice(
                "éabc".into(),
                make_args(vec![Value::Int(-3), Value::Int(-1)], None)
            )
            .unwrap(),
            Value::Str("ab".into())
        );
        // start > end → ""
        assert_eq!(
            str_slice(
                "abcdef".into(),
                make_args(vec![Value::Int(4), Value::Int(2)], None)
            )
            .unwrap(),
            Value::Str("".into())
        );
        // end + count simultâneos → erro
        assert!(str_slice("abc".into(), {
            let mut a = make_args(vec![Value::Int(0), Value::Int(2)], None);
            a.named.insert("count".into(), Value::Int(1));
            a
        })
        .is_err());
        // out of bounds (sem clamp): end > len → erro
        assert!(str_slice(
            "éabc".into(),
            make_args(vec![Value::Int(0), Value::Int(100)], None)
        )
        .is_err());
        // negativo além do início → erro
        assert!(str_slice(
            "éabc".into(),
            make_args(vec![Value::Int(-10), Value::Int(2)], None)
        )
        .is_err());
        // non-boundary → erro
        assert!(str_slice(
            "éabc".into(),
            make_args(vec![Value::Int(1), Value::Int(4)], None)
        )
        .is_err());
    }

    #[test]
    fn p690_str_at_position_consistencia() {
        // A prova: combinar position (bytes) com at (bytes) sobre texto não-ASCII
        // produz o carácter correcto. Pré-P690, at era char → dava "a".
        let s: EcoString = "café mais texto".into();
        let pos = str_position(s.clone(), make_args(vec![Value::Str("m".into())], None))
            .unwrap();
        assert_eq!(pos, Value::Int(6)); // byte 6 (c-a-f-é-space-m)
        let ch = str_at(s, make_args(vec![pos], None)).unwrap();
        assert_eq!(ch, Value::Str("m".into()));
    }

    #[test]
    fn p690_char_len_at_slice_preserva_chars() {
        // char-len conta chars
        assert_eq!(char_len("café".into()), Value::Int(4));
        assert_eq!(char_len("abc".into()), Value::Int(3));
        // char-at indexa por char: "éabc" char 2 = 'b'
        assert_eq!(
            char_at("éabc".into(), make_args(vec![Value::Int(2)], None)).unwrap(),
            Value::Str("b".into())
        );
        // char-at negativo do fim: "café" char -1 = 'é'
        assert_eq!(
            char_at("café".into(), make_args(vec![Value::Int(-1)], None)).unwrap(),
            Value::Str("é".into())
        );
        assert!(char_at("éabc".into(), make_args(vec![Value::Int(10)], None)).is_err());
        // char-slice por char: "éabc".char-slice(2, 4) = "bc"
        assert_eq!(
            char_slice(
                "éabc".into(),
                make_args(vec![Value::Int(2), Value::Int(4)], None)
            )
            .unwrap(),
            Value::Str("bc".into())
        );
        // char-slice com count e clamp (comportamento pré-P690)
        assert_eq!(
            char_slice(
                "éabc".into(),
                make_args(vec![Value::Int(0)], Some(("count", Value::Int(2))))
            )
            .unwrap(),
            Value::Str("éa".into())
        );
    }

    // ── P691 — str.find devolve substring / none (str ou regex) ───────────────

    #[test]
    fn p691_str_find_substring_e_none() {
        // substring literal encontrada → devolve a substring
        let a = make_args(vec![Value::Str("mais".into())], None);
        assert_eq!(
            str_find("café mais texto".into(), a).unwrap(),
            Value::Str("mais".into())
        );
        // multi-byte: "xéy".find("é") → "é"
        let a = make_args(vec![Value::Str("é".into())], None);
        assert_eq!(str_find("xéy".into(), a).unwrap(), Value::Str("é".into()));
        let a = make_args(vec![Value::Str("fé".into())], None);
        assert_eq!(str_find("café".into(), a).unwrap(), Value::Str("fé".into()));
        // não encontrado → none
        let a = make_args(vec![Value::Str("inexistente".into())], None);
        assert_eq!(str_find("café mais texto".into(), a).unwrap(), Value::None);
        // string vazia → "" (paridade vanilla)
        let a = make_args(vec![Value::Str("".into())], None);
        assert_eq!(str_find("abc".into(), a).unwrap(), Value::Str("".into()));
    }

    #[test]
    fn p691_str_find_regex() {
        // regex → texto do primeiro match
        let a = make_args(vec![Value::Regex(Regex::new("m..s").unwrap())], None);
        assert_eq!(
            str_find("café mais texto".into(), a).unwrap(),
            Value::Str("mais".into())
        );
        // regex sem match → none
        let a = make_args(vec![Value::Regex(Regex::new("z+").unwrap())], None);
        assert_eq!(str_find("café".into(), a).unwrap(), Value::None);
    }

    #[test]
    fn p691_str_find_tipo_e_aridade() {
        // tipo errado (Int) → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_find("abc".into(), a).is_err());
        // aridade errada (0 args) → erro
        let a = make_args(vec![], None);
        assert!(str_find("abc".into(), a).is_err());
    }

    // ── P692 — str.matches / str.normalize ────────────────────────────────────

    fn dict_get<'a>(v: &'a Value, key: &str) -> &'a Value {
        match v {
            Value::Dict(d) => d.get(key).unwrap(),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p692_str_matches_regex() {
        let a = make_args(vec![Value::Regex(Regex::new(r"\w+").unwrap())], None);
        let arr = match str_matches("um dois três quatro".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 4);
        assert_eq!(dict_get(&arr[0], "start"), &Value::Int(0));
        assert_eq!(dict_get(&arr[0], "end"), &Value::Int(2));
        assert_eq!(dict_get(&arr[0], "text"), &Value::Str("um".into()));
        // "três" começa no byte 8 (dois=3..7, espaço=7, três=8..13 — ê ocupa 2 bytes)
        assert_eq!(dict_get(&arr[2], "start"), &Value::Int(8));
        assert_eq!(dict_get(&arr[2], "end"), &Value::Int(13));
        assert_eq!(dict_get(&arr[2], "text"), &Value::Str("três".into()));
    }

    #[test]
    fn p692_str_matches_str_literal_e_vazio() {
        // literal: "abab".matches("ab") → 2 matches, captures vazias
        let a = make_args(vec![Value::Str("ab".into())], None);
        let arr = match str_matches("abab".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 2);
        assert_eq!(dict_get(&arr[0], "start"), &Value::Int(0));
        assert_eq!(dict_get(&arr[1], "start"), &Value::Int(2));
        assert_eq!(dict_get(&arr[0], "captures"), &Value::Array(vec![]));
        // sem match → array vazio
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_matches("abc".into(), a).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p692_str_matches_captures() {
        let a = make_args(vec![Value::Regex(Regex::new(r"([a-z])(\d)").unwrap())], None);
        let arr = match str_matches("a1b2".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 2);
        assert_eq!(dict_get(&arr[0], "text"), &Value::Str("a1".into()));
        assert_eq!(
            dict_get(&arr[0], "captures"),
            &Value::Array(vec![Value::Str("a".into()), Value::Str("1".into())])
        );
        assert_eq!(
            dict_get(&arr[1], "captures"),
            &Value::Array(vec![Value::Str("b".into()), Value::Str("2".into())])
        );
    }

    #[test]
    fn p692_str_normalize_forms() {
        let none = make_args(vec![], None);
        let nfc = make_args(vec![], Some(("form", Value::Str("nfc".into()))));
        let nfd = make_args(vec![], Some(("form", Value::Str("nfd".into()))));
        let nfkc = make_args(vec![], Some(("form", Value::Str("nfkc".into()))));
        let nfkd = make_args(vec![], Some(("form", Value::Str("nfkd".into()))));
        // default == nfc → "café" composto (5 bytes)
        let def = str_normalize("café".into(), none).unwrap();
        assert_eq!(def, Value::Str("café".into()));
        assert_eq!(def, str_normalize("café".into(), nfc).unwrap());
        // nfd → "cafe" + combining acute (5 chars, 6 bytes)
        let d = str_normalize("café".into(), nfd).unwrap();
        assert_eq!(d, Value::Str("cafe\u{301}".into()));
        assert_eq!(
            str_len(match &d {
                Value::Str(s) => s.clone(),
                _ => panic!(),
            }),
            Value::Int(6)
        );
        // nfkc == nfc para "café"; nfkd == nfd
        assert_eq!(
            str_normalize("café".into(), nfkc).unwrap(),
            Value::Str("café".into())
        );
        assert_eq!(
            str_normalize("café".into(), nfkd).unwrap(),
            Value::Str("cafe\u{301}".into())
        );
    }

    #[test]
    fn p692_str_normalize_erros() {
        // forma desconhecida → erro
        let a = make_args(vec![], Some(("form", Value::Str("nfx".into()))));
        assert!(str_normalize("café".into(), a).is_err());
        // form com tipo errado → erro
        let a = make_args(vec![], Some(("form", Value::Int(1))));
        assert!(str_normalize("café".into(), a).is_err());
        // argumento posicional → erro
        let a = make_args(vec![Value::Str("nfc".into())], None);
        assert!(str_normalize("café".into(), a).is_err());
    }

    // ── P693 — str.match aceita str | regex ───────────────────────────────────

    #[test]
    fn p693_str_match_str_literal() {
        // "abcabc".match("bc") → primeiro "bc" em start 1, end 3, captures vazio
        let a = make_args(vec![Value::Str("bc".into())], None);
        let d = str_match("abcabc".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(1));
        assert_eq!(dict_get(&d, "end"), &Value::Int(3));
        assert_eq!(dict_get(&d, "text"), &Value::Str("bc".into()));
        assert_eq!(dict_get(&d, "captures"), &Value::Array(vec![]));

        // não encontrado → none
        let a = make_args(vec![Value::Str("z".into())], None);
        assert_eq!(str_match("abc".into(), a).unwrap(), Value::None);

        // string vazia → match no início (start 0, end 0, text "")
        let a = make_args(vec![Value::Str("".into())], None);
        let d = str_match("abc".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(0));
        assert_eq!(dict_get(&d, "end"), &Value::Int(0));
        assert_eq!(dict_get(&d, "text"), &Value::Str("".into()));
    }

    #[test]
    fn p693_str_match_regex_sem_regressao() {
        // regex com capturas continua a funcionar (paridade P689)
        let a = make_args(vec![Value::Regex(Regex::new(r"(a)(b)").unwrap())], None);
        let d = str_match("xab".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(1));
        assert_eq!(dict_get(&d, "text"), &Value::Str("ab".into()));
        assert_eq!(
            dict_get(&d, "captures"),
            &Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())])
        );
        // tipo errado (Int) → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_match("abc".into(), a).is_err());
    }

    // ── P881 — tipos chamáveis como funções de ordem superior ────────────────

    #[test]
    fn p881_type_as_callable_mapeia_tipos_construtores() {
        assert!(type_as_callable(Type::Int).is_some());
        assert!(type_as_callable(Type::Float).is_some());
        assert!(type_as_callable(Type::Str).is_some());
        assert!(type_as_callable(Type::Type).is_some());
        assert!(type_as_callable(Type::Counter).is_some());
        assert!(type_as_callable(Type::State).is_some());
        assert!(type_as_callable(Type::Symbol).is_some());
        assert!(type_as_callable(Type::Bytes).is_some());
        assert!(type_as_callable(Type::Datetime).is_some());

        // tipos não chamáveis → None
        assert!(type_as_callable(Type::Bool).is_none());
        assert!(type_as_callable(Type::Array).is_none());
        assert!(type_as_callable(Type::Length).is_none());
    }

    #[test]
    fn p881_expect_one_func_aceita_tipo_chamavel() {
        let args = make_args(vec![Value::Type(Type::Str)], None);
        let func = expect_one_func(args, "array.map()").unwrap();
        assert_eq!(func.name(), Some("str"));
    }

    #[test]
    fn p881_expect_one_func_rejeita_tipo_nao_chamavel() {
        let args = make_args(vec![Value::Type(Type::Bool)], None);
        let err = expect_one_func(args, "array.map()").unwrap_err();
        assert!(err[0].message.contains("array.map() espera função, recebeu bool"));
    }

    #[test]
    fn p881_expect_one_func_rejeita_int() {
        let args = make_args(vec![Value::Int(42)], None);
        let err = expect_one_func(args, "array.map()").unwrap_err();
        assert!(err[0].message.contains("array.map() espera função, recebeu int"));
    }

    #[test]
    fn p1284_zip_zero_e_split_default() {
        assert_eq!(
            array_zip(vec![Value::Int(1), Value::Int(2)], Args::positional(vec![]))
                .unwrap(),
            Value::Array(vec![
                Value::Array(vec![Value::Int(1)]),
                Value::Array(vec![Value::Int(2)]),
            ])
        );
        assert_eq!(
            str_split("ab".into(), Args::positional(vec![])).unwrap(),
            Value::Array(vec![Value::Str("ab".into())])
        );
    }

    #[test]
    fn p1284_trim_pattern_respeita_repeat() {
        let mut args = Args::positional(vec![Value::Str("x".into())]);
        args.named.insert("repeat".into(), Value::Bool(false));
        assert_eq!(
            str_trim_dispatch("xxabxx".into(), args).unwrap(),
            Value::Str("xabx".into())
        );
    }
}
