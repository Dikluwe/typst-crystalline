//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/selector.md
//! @prompt-hash af81893d
//! @layer L1
//! @updated 2026-09-30

use super::*;

fn selector_literal_pattern(text: &str) -> String {
    let mut pattern = String::with_capacity(text.len());
    for character in text.chars() {
        if "\\.+*?()|[]{}^$#&-~".contains(character) {
            pattern.push('\\');
        }
        pattern.push(character);
    }
    pattern
}

/// Convert a selector operand without executing an element constructor.
/// The native constructor's Kind policy and bare identity are retained; literal
/// text operands are escaped, independently of Tekt's String-to-Kind extension.
/// Receiver eligibility is separately bounded by `selector_receiver`.
pub(in crate::compiler::eval) fn value_to_query_selector(
    value: &Value,
) -> Option<Selector> {
    match value {
        Value::Selector(selector) => Some(selector.clone()),
        Value::Func(function) => {
            crate::compiler::stdlib::element_kind_of_native_func(function)
                .map(Selector::Kind)
                .or_else(|| {
                    native_element_fields(function)
                        .map(|_| Selector::NativeElement(function.clone()))
                })
        }
        Value::Str(text) => {
            if text.is_empty() {
                None
            } else {
                crate::entities::regex::Regex::new(&selector_literal_pattern(
                    text.as_str(),
                ))
                .ok()
                .map(Selector::Regex)
            }
        }
        Value::Regex(regex) => {
            if regex.pattern().is_empty() || regex.is_match("") {
                None
            } else {
                Some(Selector::Regex(regex.clone()))
            }
        }
        Value::Label(label) => Some(Selector::Label(label.clone())),
        Value::Location(location) => Some(Selector::Location(*location)),
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Gradient(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    }
}

fn selector_receiver(value: &Value) -> Option<Selector> {
    match value {
        Value::Selector(_) => value_to_query_selector(value),
        Value::Func(function) => function.native_fn_addr().and_then(|call| {
            if std::ptr::fn_addr_eq(
                call,
                crate::compiler::stdlib::native_heading as fn(_, _, _, _) -> _,
            ) {
                Some(Selector::Kind(ElementKind::Heading))
            } else if std::ptr::fn_addr_eq(
                call,
                crate::compiler::stdlib::native_figure as fn(_, _, _, _) -> _,
            ) {
                Some(Selector::Kind(ElementKind::Figure))
            } else {
                None
            }
        }),
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    }
}

pub(in crate::compiler::eval) fn eval_selector_or_and<'a>(
    target: &Value,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = selector_receiver(target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            "unexpected argument",
        )]);
    }
    let mut selectors = vec![base];
    for other in args.items {
        let Some(other_sel) = value_to_query_selector(&other) else {
            return Err(vec![SourceDiagnostic::error(
                args_node.span(),
                format!(
                    "selector.{}() espera um selector, recebeu {}",
                    method,
                    other.type_name()
                ),
            )]);
        };
        selectors.push(other_sel);
    }

    match method {
        "or" => Ok(Some(Selector::Or(EcoVec::from(selectors)))),
        "and" => Ok(Some(Selector::And(EcoVec::from(selectors)))),
        _ => Ok(None),
    }
}

/// **P504** — Intercepta `selector(base).within(ancestor)`.
pub(in crate::compiler::eval) fn eval_selector_within<'a>(
    target: &Value,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = selector_receiver(target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            "selector.within() requer um argumento posicional".to_string(),
        )]
    })?;
    let Some(ancestor) = value_to_query_selector(&other) else {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            format!(
                "selector.within() espera um selector, recebeu {}",
                other.type_name()
            ),
        )]);
    };

    crate::compiler::eval::call_dispatch::validate_contextual_selector(
        &ancestor,
        args_node.span(),
    )?;
    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))
}

/// **P1401** — Intercepta os métodos contextuais de selector.
pub(in crate::compiler::eval) fn eval_selector_before_after<'a>(
    target: &Value,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = selector_receiver(target) else {
        return Ok(None);
    };
    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    crate::compiler::eval::call_dispatch::selector_before_after(base, method, args)
        .map(Some)
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────

#[cfg(test)]
mod batch_c_selector_tests {
    use super::*;
    use crate::compiler::stdlib::*;
    use crate::entities::func::Func;

    #[test]
    fn v16_c_selector_preserves_literal_metacharacters_and_unicode() {
        let mut literals: Vec<String> = "\\.+*?()|[]{}^$#&-~"
            .chars()
            .map(|character| character.to_string())
            .collect();
        literals.extend(
            [
                r"\d+",
                r"\n",
                r"[a-z&&[^q]]",
                r"(?x)a # note",
                r"a{2,3}|b?",
                "é中🙂",
                "e\u{301}",
                "á.中[🙂]",
                " \t\n\r",
                "\0",
                "plain text",
            ]
            .into_iter()
            .map(str::to_string),
        );
        for literal in literals {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.clone().into()))
            else {
                panic!("nonempty literal was refused: {literal:?}");
            };
            assert!(!regex.is_match(""), "{literal:?}");
            let haystack = format!("PREFIX{literal}SUFFIX");
            let matched = regex.captures_first(&haystack).expect("literal substring");
            assert_eq!(matched.text, literal, "{literal:?}");
            assert!(matched.captures.is_empty(), "{literal:?}");
            let anchored = crate::entities::regex::Regex::new(&format!(
                r"\A(?:{})\z",
                regex.pattern()
            ))
            .unwrap();
            assert!(anchored.is_match(&literal), "{literal:?}");
            assert!(!anchored.is_match(&format!("{literal}x")), "{literal:?}");
        }
        assert!(value_to_query_selector(&Value::Str("".into())).is_none());
        for (literal, nonliteral) in [
            ("a.b", "axb"),
            (r"\d+", "123"),
            ("a|b", "a"),
            ("[a-z]", "q"),
            ("(?i)a", "A"),
            ("a{2}", "aa"),
        ] {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.into()))
            else {
                panic!("literal refused: {literal:?}");
            };
            assert!(!regex.is_match(nonliteral), "{literal:?}");
        }
    }

    #[test]
    fn v16_c_selector_preserves_literal_pattern_identity() {
        for (literal, expected) in [
            (r"\.+*?()|[]{}^$#&-~", r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-\~"),
            ("á.中[🙂]", r"á\.中\[🙂\]"),
            (r"\d+", r"\\d\+"),
            ("ordinary/!%=:", "ordinary/!%=:"),
        ] {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.into()))
            else {
                panic!("literal refused: {literal:?}");
            };
            assert_eq!(regex.pattern(), expected);
        }
    }

    #[test]
    fn v16_c_selector_preserves_heading_alias_and_refuses_scalar_or_homonym() {
        let alias = Func::native("unrelated", native_heading);
        assert_eq!(
            value_to_query_selector(&Value::Func(alias)),
            Some(Selector::Kind(ElementKind::Heading))
        );
        for value in [
            Value::Int(1),
            Value::None,
            Value::Bool(false),
            Value::Content(crate::entities::content::Content::Empty),
            Value::Func(Func::native("heading", |_, _, _, _| Ok(Value::None))),
        ] {
            assert!(value_to_query_selector(&value).is_none());
        }
        let base = Selector::Or(vec![Selector::Kind(ElementKind::Heading)].into());
        assert_eq!(value_to_query_selector(&Value::Selector(base.clone())), Some(base));
    }

    #[test]
    fn v16_c_selector_accepts_actual_native_operands_and_preserves_kind_policy() {
        for (function, kind) in [
            (Func::native("alias", native_table), ElementKind::Table),
            (Func::native("alias", native_footnote), ElementKind::Footnote),
            (Func::native("alias", native_metadata), ElementKind::Metadata),
            (Func::native("alias", native_outline_entry), ElementKind::OutlineEntry),
            (Func::native("alias", native_math_equation), ElementKind::Equation),
        ] {
            assert_eq!(
                value_to_query_selector(&Value::Func(function)),
                Some(Selector::Kind(kind))
            );
        }
        for function in [
            Func::native("alias", native_table_cell),
            Func::native("alias", native_par),
            Func::native_with_engine("alias", native_page),
        ] {
            let Some(Selector::NativeElement(actual)) =
                value_to_query_selector(&Value::Func(function.clone()))
            else {
                panic!("native element operand")
            };
            assert_eq!(actual, function);
        }
    }

    #[test]
    fn v16_c_selector_text_label_location_admission_is_not_legacy_kind_coercion() {
        let Some(Selector::Regex(regex)) =
            value_to_query_selector(&Value::Str("a.b".into()))
        else {
            panic!("literal operand")
        };
        assert!(regex.is_match("a.b"));
        assert!(!regex.is_match("axb"));
        assert!(matches!(
            value_to_query_selector(&Value::Str("table".into())),
            Some(Selector::Regex(_))
        ));
        let label = crate::entities::label::Label("x".into());
        assert_eq!(
            value_to_query_selector(&Value::Label(label.clone())),
            Some(Selector::Label(label))
        );
        let location = crate::entities::location::Location::from_raw(9);
        assert_eq!(
            value_to_query_selector(&Value::Location(location)),
            Some(Selector::Location(location))
        );
        assert!(matches!(
            value_to_query_selector(&Value::Regex(
                crate::entities::regex::Regex::new("x+").unwrap()
            )),
            Some(Selector::Regex(_))
        ));
        for value in [
            Value::Str("".into()),
            Value::Regex(crate::entities::regex::Regex::new("").unwrap()),
            Value::Regex(crate::entities::regex::Regex::new("a*").unwrap()),
        ] {
            assert!(value_to_query_selector(&value).is_none());
        }
    }

    struct MemoryWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
        source: crate::entities::source::Source,
    }
    impl crate::contracts::world::World for MemoryWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            self.source.id()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Ok(self.source.clone())
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
    fn evaluate(text: &str) -> SourceResult<crate::entities::module::Module> {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let world = MemoryWorld {
            library: crate::entities::world_types::Library::new(),
            book: crate::entities::font_book::FontBook::new(),
            source: crate::entities::source::Source::new(id, text.into()),
        };
        crate::compiler::eval::tests::eval_for_test(&world, &world.source)
    }

    #[test]
    fn v16_c_selector_preserves_receiver_and_argument_once_and_refusal_precedence() {
        evaluate("#let n = 0\n#let s = { n += 1; selector(heading) }.or({ n += 10; heading })\n#assert(n == 11)").unwrap();
        let errors = evaluate("#let s = 1.or(panic(\"argument ran\"))").unwrap_err();
        assert!(!errors.iter().any(|error| error.message.contains("argument ran")));
    }

    #[test]
    fn v16_c_selector_native_composition_effects_and_contextual_rejection() {
        evaluate("#let n = 0\n#let s = selector(heading).or({ n += 1; table.cell })\n#assert(n == 1)").unwrap();
        for expression in [
            "selector(heading).before(table.cell)",
            "selector(heading).after(selector(page))",
            "selector(heading).within(table.cell)",
            "selector(heading).within(selector(heading).or(table.cell))",
        ] {
            let errors = evaluate(&format!("#let s = {expression}")).unwrap_err();
            assert!(
                errors.iter().any(|error| error.message.contains("is not locatable")),
                "{expression}: {errors:?}"
            );
        }
        let errors =
            evaluate("#let s = \"text\".or(panic(\"argument ran\"))").unwrap_err();
        assert!(!errors.iter().any(|error| error.message.contains("argument ran")));
    }
}
