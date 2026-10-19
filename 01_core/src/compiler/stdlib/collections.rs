//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections.md
//! @prompt-hash 455dc104
//! @layer L1
//! @updated 2026-09-30
//!
//! Despacho exaustivo e catálogo estático dos métodos de coleção.
//! Os algoritmos residem nos módulos filhos por comportamento.

use std::cmp::Ordering;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::compiler::eval::call_dispatch::apply_func;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
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
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

mod arguments;
mod array_access;
mod array_aggregate;
mod array_predicate;
mod array_sequence;
mod bytes;
mod dictionary;
mod str_access;
mod str_search;
mod str_transform;
mod validation;

use self::arguments::{
    arguments_at, arguments_filter, arguments_len, arguments_map, arguments_named,
    arguments_pos,
};
use self::array_access::{
    array_at, array_contains, array_first, array_last, array_len, array_slice,
};
use self::array_aggregate::{array_fold, array_product, array_reduce, array_sum};
#[cfg(test)]
use self::array_predicate::array_dedup_values;
use self::array_predicate::{
    array_all, array_any, array_dedup, array_filter, array_find, array_map,
    array_position, array_sorted,
};
use self::array_sequence::{
    array_chunks, array_enumerate, array_flatten, array_intersperse, array_join,
    array_rev, array_split, array_to_dict, array_windows, array_zip,
};
#[cfg(test)]
use self::bytes::try_dispatch_bytes_method;
use self::bytes::{bytes_at, bytes_len, bytes_slice};
use self::dictionary::{
    dict_at, dict_filter, dict_insert, dict_keys, dict_len, dict_map, dict_pairs,
    dict_remove, dict_update, dict_values,
};
use self::str_access::{
    char_at, char_len, char_slice, str_at, str_clusters, str_codepoints, str_first,
    str_last, str_len, str_slice,
};
use self::str_search::{
    str_contains, str_ends_with, str_find, str_match, str_matches, str_position,
    str_starts_with,
};
use self::str_transform::{
    str_normalize, str_repeat, str_replace, str_rev, str_split, str_to_lower,
    str_to_unicode, str_to_upper, str_trim_dispatch,
};
use self::validation::{
    array_no_args, dict_no_args, ensure_no_args, expect_one_dict, expect_one_func,
    expect_one_int, expect_one_str, expect_str_value, expect_two_str, optional_default,
    str_no_args, type_as_callable,
};
#[cfg(test)]
use crate::entities::regex::Regex;

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
    match try_dispatch_collection_method_with(
        target,
        method,
        |_, _, _| Ok(args),
        call_spans,
        scopes,
        ctx,
        engine,
    ) {
        CollectionDispatch::Handled(result) => Some(result),
        CollectionDispatch::Unhandled(_) => None,
    }
}

pub(crate) enum CollectionDispatch {
    Handled(SourceResult<Value>),
    Unhandled(Value),
}

pub(crate) fn try_dispatch_collection_method_with(
    target: Value,
    method: &str,
    evaluate_args: impl FnOnce(
        &mut Scopes<'_>,
        &mut EvalContext,
        &mut Engine<'_>,
    ) -> SourceResult<Args>,
    call_spans: Option<&CollectionCallSpans>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> CollectionDispatch {
    macro_rules! bind_args {
        ($args:ident, $body:expr) => {{
            let $args = match evaluate_args(scopes, ctx, engine) {
                Ok(args) => args,
                Err(errors) => return CollectionDispatch::Handled(Err(errors)),
            };
            CollectionDispatch::Handled($body)
        }};
    }
    match target {
        Value::Array(arr) => match method {
            "len" => bind_args!(args, array_len(arr, args)),
            "first" => bind_args!(args, array_first(arr, args)),
            "last" => bind_args!(args, array_last(arr, args)),
            "at" => bind_args!(args, array_at(arr, args)),
            "slice" => bind_args!(args, array_slice(arr, args)),
            "rev" => bind_args!(args, array_no_args(arr, args, array_rev)),
            "sum" => bind_args!(args, array_sum(arr, args)),
            "product" => bind_args!(args, array_product(arr, args)),
            "contains" => bind_args!(args, array_contains(arr, args)),
            "sorted" => bind_args!(args, array_sorted(arr, args, scopes, ctx, engine)),
            "filter" => bind_args!(args, array_filter(arr, args, scopes, ctx, engine)),
            "map" => bind_args!(args, array_map(arr, args, scopes, ctx, engine)),
            "find" => bind_args!(args, array_find(arr, args, scopes, ctx, engine)),
            "position" => {
                bind_args!(args, array_position(arr, args, scopes, ctx, engine))
            }
            "any" => bind_args!(args, array_any(arr, args, scopes, ctx, engine)),
            "all" => bind_args!(args, array_all(arr, args, scopes, ctx, engine)),
            "zip" => bind_args!(args, array_zip(arr, args)),
            "enumerate" => bind_args!(args, array_enumerate(arr, args)),
            "dedup" => bind_args!(args, array_dedup(arr, args, scopes, ctx, engine)),
            "chunks" => bind_args!(args, array_chunks(arr, args)),
            "windows" => bind_args!(args, array_windows(arr, args)),
            "flatten" => bind_args!(args, array_no_args(arr, args, array_flatten)),
            "fold" => bind_args!(args, array_fold(arr, args, scopes, ctx, engine)),
            "reduce" => bind_args!(args, array_reduce(arr, args, scopes, ctx, engine)),
            "intersperse" => bind_args!(args, array_intersperse(arr, args)),
            "split" => bind_args!(args, array_split(arr, args)),
            "to-dict" => bind_args!(args, array_to_dict(arr, args)),
            "join" => bind_args!(args, array_join(arr, args)),
            _ => CollectionDispatch::Unhandled(Value::Array(arr)),
        },
        Value::Dict(dict) => match method {
            "keys" => bind_args!(args, dict_no_args(dict, args, dict_keys)),
            "values" => bind_args!(args, dict_no_args(dict, args, dict_values)),
            "pairs" => bind_args!(args, dict_no_args(dict, args, dict_pairs)),
            "remove" => bind_args!(args, dict_remove(dict, args)),
            "update" => bind_args!(args, dict_update(dict, args)),
            "insert" => bind_args!(args, dict_insert(dict, args)),
            "at" => bind_args!(args, dict_at(dict, args)),
            "len" => bind_args!(args, dict_no_args(dict, args, dict_len)),
            "map" => bind_args!(args, dict_map(dict, args, scopes, ctx, engine)),
            "filter" => bind_args!(args, dict_filter(dict, args, scopes, ctx, engine)),
            _ => CollectionDispatch::Unhandled(Value::Dict(dict)),
        },
        Value::Str(s) => match method {
            "len" => bind_args!(args, str_no_args(s, args, str_len)),
            "first" => bind_args!(args, str_first(s, args)),
            "last" => bind_args!(args, str_last(s, args)),
            "at" => bind_args!(args, str_at(s, args)),
            "slice" => bind_args!(args, str_slice(s, args)),
            "char-len" => bind_args!(_args, Ok(char_len(s))),
            "char-at" => bind_args!(args, char_at(s, args)),
            "char-slice" => bind_args!(args, char_slice(s, args)),
            "clusters" => bind_args!(args, str_no_args(s, args, str_clusters)),
            "contains" => bind_args!(args, str_contains(s, args)),
            "starts-with" => bind_args!(args, str_starts_with(s, args)),
            "ends-with" => bind_args!(args, str_ends_with(s, args)),
            "find" => bind_args!(args, str_find(s, args)),
            "replace" => bind_args!(args, str_replace(s, args)),
            "trim" => bind_args!(args, str_trim_dispatch(s, args)),
            "split" => bind_args!(args, str_split(s, args)),
            "repeat" => bind_args!(args, str_repeat(s, args)),
            "to-upper" => bind_args!(_args, Ok(str_to_upper(s))),
            "to-lower" => bind_args!(_args, Ok(str_to_lower(s))),
            "to-unicode" => {
                bind_args!(args, ensure_no_args(&args).and_then(|_| str_to_unicode(s)))
            }
            "rev" => bind_args!(args, str_no_args(s, args, str_rev)),
            "codepoints" => bind_args!(args, str_no_args(s, args, str_codepoints)),
            "normalize" => bind_args!(args, str_normalize(s, args)),
            "position" => bind_args!(args, str_position(s, args)),
            "match" => bind_args!(args, str_match(s, args)),
            "matches" => bind_args!(args, str_matches(s, args)),
            _ => CollectionDispatch::Unhandled(Value::Str(s)),
        },
        Value::Bytes(bytes) => match method {
            "len" => bind_args!(args, bytes_len(bytes, args, call_spans)),
            "at" => bind_args!(args, bytes_at(bytes, args, call_spans)),
            "slice" => bind_args!(args, bytes_slice(bytes, args, call_spans)),
            _ => CollectionDispatch::Unhandled(Value::Bytes(bytes)),
        },
        Value::Args(arguments) => match method {
            "len" => bind_args!(args, arguments_len(arguments, args)),
            "at" => bind_args!(args, arguments_at(arguments, args)),
            "pos" => bind_args!(args, arguments_pos(arguments, args)),
            "named" => bind_args!(args, arguments_named(arguments, args)),
            "filter" => {
                bind_args!(args, arguments_filter(arguments, args, scopes, ctx, engine))
            }
            "map" => {
                bind_args!(args, arguments_map(arguments, args, scopes, ctx, engine))
            }
            _ => CollectionDispatch::Unhandled(Value::Args(arguments)),
        },
        // Values outside the collection receiver families continue in the caller.
        Value::None
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Float(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(..)
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => CollectionDispatch::Unhandled(target),
    }
}

/// **P1142** — membros não ligados expostos nos valores-tipo `array` e `str`.
/// As funções delegam aos mesmos helpers das formas de instância.
pub(crate) fn collection_type_field(t: Type, field: &str) -> Option<Value> {
    match t {
        Type::Array => match field {
            "range" => Some(Value::Func(Func::native("range", native_range))),
            "all" => Some(Value::Func(Func::native_with_engine("all", array_all_static))),
            "any" => Some(Value::Func(Func::native_with_engine("any", array_any_static))),
            "at" => Some(Value::Func(Func::native_with_engine("at", array_at_static))),
            "chunks" => {
                Some(Value::Func(Func::native_with_engine("chunks", array_chunks_static)))
            }
            "contains" => Some(Value::Func(Func::native_with_engine(
                "contains",
                array_contains_static,
            ))),
            "dedup" => {
                Some(Value::Func(Func::native_with_engine("dedup", array_dedup_static)))
            }
            "enumerate" => Some(Value::Func(Func::native_with_engine(
                "enumerate",
                array_enumerate_static,
            ))),
            "filter" => {
                Some(Value::Func(Func::native_with_engine("filter", array_filter_static)))
            }
            "find" => {
                Some(Value::Func(Func::native_with_engine("find", array_find_static)))
            }
            "first" => {
                Some(Value::Func(Func::native_with_engine("first", array_first_static)))
            }
            "flatten" => Some(Value::Func(Func::native_with_engine(
                "flatten",
                array_flatten_static,
            ))),
            "fold" => {
                Some(Value::Func(Func::native_with_engine("fold", array_fold_static)))
            }
            "insert" => {
                Some(Value::Func(Func::native_with_engine("insert", array_insert_static)))
            }
            "intersperse" => Some(Value::Func(Func::native_with_engine(
                "intersperse",
                array_intersperse_static,
            ))),
            "join" => {
                Some(Value::Func(Func::native_with_engine("join", array_join_static)))
            }
            "last" => {
                Some(Value::Func(Func::native_with_engine("last", array_last_static)))
            }
            "len" => Some(Value::Func(Func::native_with_engine("len", array_len_static))),
            "map" => Some(Value::Func(Func::native_with_engine("map", array_map_static))),
            "pop" => Some(Value::Func(Func::native_with_engine("pop", array_pop_static))),
            "position" => Some(Value::Func(Func::native_with_engine(
                "position",
                array_position_static,
            ))),
            "product" => Some(Value::Func(Func::native_with_engine(
                "product",
                array_product_static,
            ))),
            "push" => {
                Some(Value::Func(Func::native_with_engine("push", array_push_static)))
            }
            "reduce" => {
                Some(Value::Func(Func::native_with_engine("reduce", array_reduce_static)))
            }
            "remove" => {
                Some(Value::Func(Func::native_with_engine("remove", array_remove_static)))
            }
            "rev" => Some(Value::Func(Func::native_with_engine("rev", array_rev_static))),
            "slice" => {
                Some(Value::Func(Func::native_with_engine("slice", array_slice_static)))
            }
            "sorted" => {
                Some(Value::Func(Func::native_with_engine("sorted", array_sorted_static)))
            }
            "split" => {
                Some(Value::Func(Func::native_with_engine("split", array_split_static)))
            }
            "sum" => Some(Value::Func(Func::native_with_engine("sum", array_sum_static))),
            "to-dict" => Some(Value::Func(Func::native_with_engine(
                "to-dict",
                array_to_dict_static,
            ))),
            "windows" => Some(Value::Func(Func::native_with_engine(
                "windows",
                array_windows_static,
            ))),
            "zip" => Some(Value::Func(Func::native_with_engine("zip", array_zip_static))),
            _ => None,
        },
        Type::Dictionary => match field {
            "at" => Some(Value::Func(Func::native_with_engine("at", dict_at_static))),
            "filter" => {
                Some(Value::Func(Func::native_with_engine("filter", dict_filter_static)))
            }
            "insert" => {
                Some(Value::Func(Func::native_with_engine("insert", dict_insert_static)))
            }
            "keys" => {
                Some(Value::Func(Func::native_with_engine("keys", dict_keys_static)))
            }
            "len" => Some(Value::Func(Func::native_with_engine("len", dict_len_static))),
            "map" => Some(Value::Func(Func::native_with_engine("map", dict_map_static))),
            "pairs" => {
                Some(Value::Func(Func::native_with_engine("pairs", dict_pairs_static)))
            }
            "remove" => {
                Some(Value::Func(Func::native_with_engine("remove", dict_remove_static)))
            }
            "values" => {
                Some(Value::Func(Func::native_with_engine("values", dict_values_static)))
            }
            _ => None,
        },
        Type::Str => match field {
            "at" => Some(Value::Func(Func::native_with_engine("at", str_at_static))),
            "clusters" => Some(Value::Func(Func::native_with_engine(
                "clusters",
                str_clusters_static,
            ))),
            "codepoints" => Some(Value::Func(Func::native_with_engine(
                "codepoints",
                str_codepoints_static,
            ))),
            "contains" => Some(Value::Func(Func::native_with_engine(
                "contains",
                str_contains_static,
            ))),
            "ends-with" => Some(Value::Func(Func::native_with_engine(
                "ends-with",
                str_ends_with_static,
            ))),
            "find" => {
                Some(Value::Func(Func::native_with_engine("find", str_find_static)))
            }
            "first" => {
                Some(Value::Func(Func::native_with_engine("first", str_first_static)))
            }
            "last" => {
                Some(Value::Func(Func::native_with_engine("last", str_last_static)))
            }
            "len" => Some(Value::Func(Func::native_with_engine("len", str_len_static))),
            "match" => {
                Some(Value::Func(Func::native_with_engine("match", str_match_static)))
            }
            "matches" => {
                Some(Value::Func(Func::native_with_engine("matches", str_matches_static)))
            }
            "normalize" => Some(Value::Func(Func::native_with_engine(
                "normalize",
                str_normalize_static,
            ))),
            "position" => Some(Value::Func(Func::native_with_engine(
                "position",
                str_position_static,
            ))),
            "replace" => {
                Some(Value::Func(Func::native_with_engine("replace", str_replace_static)))
            }
            "rev" => Some(Value::Func(Func::native_with_engine("rev", str_rev_static))),
            "slice" => {
                Some(Value::Func(Func::native_with_engine("slice", str_slice_static)))
            }
            "split" => {
                Some(Value::Func(Func::native_with_engine("split", str_split_static)))
            }
            "starts-with" => Some(Value::Func(Func::native_with_engine(
                "starts-with",
                str_starts_with_static,
            ))),
            "to-unicode" => Some(Value::Func(Func::native_with_engine(
                "to-unicode",
                str_to_unicode_static,
            ))),
            "trim" => {
                Some(Value::Func(Func::native_with_engine("trim", str_trim_static)))
            }
            _ => None,
        },
        Type::Bytes => match field {
            "at" => Some(Value::Func(Func::native_with_engine("at", bytes_at_static))),
            "len" => Some(Value::Func(Func::native_with_engine("len", bytes_len_static))),
            "slice" => {
                Some(Value::Func(Func::native_with_engine("slice", bytes_slice_static)))
            }
            _ => None,
        },
        Type::Arguments => match field {
            "at" => Some(Value::Func(Func::native_with_engine("at", args_at_static))),
            "filter" => {
                Some(Value::Func(Func::native_with_engine("filter", args_filter_static)))
            }
            "len" => Some(Value::Func(Func::native_with_engine("len", args_len_static))),
            "map" => Some(Value::Func(Func::native_with_engine("map", args_map_static))),
            "named" => {
                Some(Value::Func(Func::native_with_engine("named", args_named_static)))
            }
            "pos" => Some(Value::Func(Func::native_with_engine("pos", args_pos_static))),
            _ => None,
        },
        // These types have no collection catalogue in this owner.
        Type::None
        | Type::Auto
        | Type::Bool
        | Type::Int
        | Type::Float
        | Type::Module
        | Type::Datetime
        | Type::Function
        | Type::Content
        | Type::Length
        | Type::Ratio
        | Type::Relative
        | Type::Angle
        | Type::Color
        | Type::Stroke
        | Type::Fraction
        | Type::Alignment
        | Type::Location
        | Type::Gradient
        | Type::Regex
        | Type::Tiling
        | Type::Decimal
        | Type::Duration
        | Type::Version
        | Type::Selector
        | Type::Symbol
        | Type::State
        | Type::Counter
        | Type::Label
        | Type::Direction
        | Type::Path
        | Type::Type
        | Type::SpotColorant => None,
    }
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
    let Some(target) = args.items.first() else {
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
    let mut delegated = args.clone();
    delegated.remove_positional(0);
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

// ── array helpers ───────────────────────────────────────────────────────────

// ── dict helpers ────────────────────────────────────────────────────────────

// ── str helpers ─────────────────────────────────────────────────────────────

// ── argument helpers ────────────────────────────────────────────────────────

// ── value comparison (para sorted) ──────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(items: Vec<Value>, named: Option<(&str, Value)>) -> Args {
        let mut args = Args::positional(items);
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

#[cfg(test)]
mod catalogue_tests {
    use super::*;

    #[test]
    fn catalogue_same_named_collection_fields_keep_distinct_native_receivers() {
        use crate::entities::func::FuncRepr;
        for (kind, expected) in [
            (Type::Array, Func::native_with_engine("len", array_len_static)),
            (Type::Str, Func::native_with_engine("len", str_len_static)),
            (Type::Dictionary, Func::native_with_engine("len", dict_len_static)),
        ] {
            let Some(Value::Func(found)) = collection_type_field(kind, "len") else {
                panic!("missing catalogue member")
            };
            assert_eq!(found.name(), Some("len"));
            let (FuncRepr::NativeWithEngine(found), FuncRepr::NativeWithEngine(expected)) =
                (found.repr(), expected.repr())
            else {
                panic!("native ABI changed")
            };
            assert!(std::ptr::fn_addr_eq(found.call, expected.call));
        }
    }

    #[test]
    fn catalogue_absence_is_scoped_to_type_and_open_field_name() {
        for (kind, field) in
            [(Type::Bytes, "at"), (Type::Arguments, "named"), (Type::Array, "range")]
        {
            let Some(Value::Func(func)) = collection_type_field(kind, field) else {
                panic!("missing {}.{field}", kind.name())
            };
            assert_eq!(func.name(), Some(field));
            assert!(collection_type_field(kind, "nonexistent").is_none());
        }
        assert!(collection_type_field(Type::Bytes, "first").is_none());
        assert!(collection_type_field(Type::Str, "range").is_none());
        assert!(collection_type_field(Type::Int, "len").is_none());
        assert!(
            collection_type_field(Type::Function, "with").is_none(),
            "not this owner"
        );
    }
}
