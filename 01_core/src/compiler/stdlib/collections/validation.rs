//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/validation.md
//! @prompt-hash 189884bd
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::validation`.

use super::*;

pub(super) fn ensure_no_args(args: &Args) -> SourceResult<()> {
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

pub(super) fn array_no_args(
    arr: Vec<Value>,
    args: Args,
    operation: fn(Vec<Value>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(arr))
}

pub(super) fn dict_no_args(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    operation: fn(IndexMap<EcoString, Value, FxBuildHasher>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(dict))
}

pub(super) fn str_no_args(
    value: EcoString,
    args: Args,
    operation: fn(EcoString) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(value))
}

pub(super) fn optional_default(args: &Args) -> SourceResult<Option<Value>> {
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

pub(super) fn expect_str_value(
    args: Args,
    context: &str,
) -> SourceResult<(EcoString, Value)> {
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
pub(super) fn type_as_callable(t: Type) -> Option<Func> {
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
        Type::Decimal => Func::native("decimal", crate::compiler::stdlib::native_decimal),
        Type::Duration => {
            Func::native("duration", crate::compiler::stdlib::native_duration)
        }
        Type::Regex => Func::native("regex", crate::compiler::stdlib::native_regex),
        Type::Selector => {
            Func::native("selector", crate::compiler::stdlib::native_selector)
        }
        Type::Stroke => Func::native("stroke", crate::compiler::stdlib::native_stroke),
        Type::Tiling => Func::native("tiling", crate::compiler::stdlib::native_tiling),
        Type::Version => Func::native("version", crate::compiler::stdlib::native_version),
        Type::Label => Func::native("label", crate::compiler::stdlib::native_label),
        Type::Array => Func::native("array", crate::compiler::stdlib::native_array_bytes),
        Type::Arguments => Func::native("arguments", native_arguments_adapter),
        Type::SpotColorant => {
            Func::native("spot", crate::compiler::stdlib::native_spot_colorant_construct)
        }
        Type::None
        | Type::Auto
        | Type::Bool
        | Type::Dictionary
        | Type::Module
        | Type::Function
        | Type::Content
        | Type::Length
        | Type::Ratio
        | Type::Relative
        | Type::Angle
        | Type::Color
        | Type::Fraction
        | Type::Alignment
        | Type::Location
        | Type::Gradient
        | Type::Direction
        | Type::Path => return None,
    })
}

// Arguments is a causal value; conversion must preserve the complete carrier.
fn native_arguments_adapter(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Ok(Value::Args(args.clone()))
}

pub(super) fn expect_one_func(args: Args, context: &str) -> SourceResult<Func> {
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

pub(super) fn expect_one_dict(
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

pub(super) fn expect_one_str(args: Args, context: &str) -> SourceResult<EcoString> {
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

pub(super) fn expect_two_str(
    args: Args,
    context: &str,
) -> SourceResult<(EcoString, EcoString)> {
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

pub(super) fn expect_one_int(args: Args, context: &str) -> SourceResult<i64> {
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

#[cfg(test)]
mod batch_f_tests {
    use super::*;

    #[derive(Default)]
    struct TestWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for TestWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            test_file()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }
    fn test_file() -> crate::entities::file_id::FileId {
        crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    fn evaluate(expression: &str) -> Value {
        crate::compiler::eval::eval_expression(&TestWorld::default(), expression)
            .0
            .unwrap_or_else(|errors| panic!("{expression}: {errors:?}"))
    }

    #[test]
    fn batch_f_higher_order_decimal() {
        assert_eq!(
            evaluate("(\"2.5\",).map(decimal).first() == decimal(\"2.5\")"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_regex() {
        assert_eq!(
            evaluate("(\"a\",).map(regex).first() == regex(\"a\")"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_selector() {
        assert_eq!(
            evaluate("(heading,).map(selector).first() == selector(heading)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_stroke() {
        assert_eq!(
            evaluate("(red,).map(stroke).first() == stroke(red)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_tiling() {
        assert_eq!(
            evaluate("type(([x],).map(tiling).first()) == tiling"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_version() {
        assert_eq!(
            evaluate("(2,).map(version).first() == version(2)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_label() {
        assert_eq!(evaluate("(\"x\",).map(label).first() == <x>"), Value::Bool(true));
    }

    #[test]
    fn batch_f_higher_order_array() {
        assert_eq!(
            evaluate("(bytes((0,255)),).map(array).first() == (0,255)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_arguments() {
        assert_eq!(
            evaluate("(1,).map(arguments).first().pos() == (1,)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_spot() {
        assert_eq!(evaluate("().map(color.spot) == ()"), Value::Bool(true));
        let errors = crate::compiler::eval::eval_expression(
            &TestWorld::default(),
            "(\"x\",).map(color.spot)",
        )
        .0
        .unwrap_err();
        assert_eq!(errors[0].message, "missing argument: fallback");
    }

    #[test]
    fn batch_f_higher_order_duration() {
        assert_eq!(evaluate("().map(duration) == ()"), Value::Bool(true));
    }

    #[test]
    fn batch_f_higher_order_sorted_key() {
        assert_eq!(
            evaluate("(\"10\", \"2\").sorted(key: decimal) == (\"2\", \"10\")"),
            Value::Bool(true)
        );
        assert_eq!(
            evaluate("(10, 2).sorted(key: version) == (2, 10)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn batch_f_higher_order_predicate_retains_result_validation() {
        let errors = crate::compiler::eval::eval_expression(
            &TestWorld::default(),
            "(\"x\",).filter(label)",
        )
        .0
        .unwrap_err();
        assert_eq!(errors[0].message, "expected boolean, found label");
    }
    #[test]
    fn batch_f_higher_order_legacy_alias_and_unsupported_types() {
        assert_eq!(
            evaluate("{ let alias = int; (\"2\",).map(alias) == (2,) }"),
            Value::Bool(true)
        );
        for ty in [
            Type::None,
            Type::Auto,
            Type::Bool,
            Type::Content,
            Type::Path,
            Type::Dictionary,
        ] {
            assert!(type_as_callable(ty).is_none(), "{ty:?}");
        }
        let errors = crate::compiler::eval::eval_expression(
            &TestWorld::default(),
            "(\"bad\",).map(int)",
        )
        .0
        .unwrap_err();
        assert_eq!(errors[0].message, "int() não consegue parsear \"bad\"");
    }
    #[test]
    fn batch_f_higher_order_constructors_remain_lazy() {
        for ty in [
            Type::Decimal,
            Type::Duration,
            Type::Regex,
            Type::Selector,
            Type::Stroke,
            Type::Tiling,
            Type::Version,
            Type::Label,
            Type::Array,
            Type::Arguments,
            Type::SpotColorant,
        ] {
            assert!(type_as_callable(ty).is_some(), "{ty:?}");
        }
    }
}
