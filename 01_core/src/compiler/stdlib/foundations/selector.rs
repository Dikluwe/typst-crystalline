//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/selector.md
//! @prompt-hash b9d735c6
//! @layer L1
//! @updated 2026-08-23

use std::ptr::fn_addr_eq;

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::{
    expect_no_named, native_figure, native_heading,
    native_metadata as native_metadata_func, native_table,
};
use crate::entities::args::Args;
use crate::entities::element_kind::ElementKind;
use crate::entities::file_id::FileId;
use crate::entities::label::Label;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub fn native_selector(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Selector(selector)] => Ok(Value::Selector(selector.clone())),
        [Value::Regex(regex)] if regex.pattern().is_empty() => {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "regex selector is empty",
            )])
        }
        [Value::Regex(regex)] if regex.is_match("") => {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "regex matches empty text",
            )])
        }
        [Value::Regex(regex)] => Ok(Value::Selector(Selector::Regex(regex.clone()))),
        [Value::Str(text)] if text.is_empty() => {
            Err(vec![SourceDiagnostic::error(Span::detached(), "text selector is empty")])
        }
        [Value::Str(s)]
            if matches!(
                (s.len() >= 2, s.starts_with('<'), s.ends_with('>')),
                (true, true, true)
            ) =>
        {
            Ok(Value::Selector(Selector::Label(Label(s[1..s.len() - 1].to_string()))))
        }
        [Value::Str(kind_str)] => ElementKind::from_name(kind_str.as_str())
            .map(|kind| Value::Selector(Selector::Kind(kind)))
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("selector(): kind '{}' não reconhecido", kind_str),
                )]
            }),
        [Value::Func(f)] => element_kind_of_native_func(f)
            .map(|kind| Value::Selector(Selector::Kind(kind)))
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    "only element functions can be used as selectors",
                )]
            }),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector(): argumento inválido ({})", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector() requer 1 argumento, recebeu {}", args.items.len()),
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::selector::Selector;

    fn ctx() -> EvalContext {
        EvalContext::new()
    }

    #[test]
    fn p1339_selector_and_parser_preserve_element_group() {
        let expected = Selector::Element {
            function: crate::entities::func::Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            ),
            fields: [("text".into(), Value::Str("body".into()))].into_iter().collect(),
        };
        let input = Value::Selector(expected.clone());
        let result = native_selector(
            &mut ctx(),
            &Args::positional(vec![input.clone()]),
            &NullWorld::default(),
            tfid(),
        )
        .unwrap();
        assert_eq!(result, input);
        assert_eq!(parse_selector_arg(&[input], "query").unwrap(), expected);
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
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

    // ── P1043: Pares de independência testcase() para query.rs:164 e query.rs:238 ─

    #[test]
    fn p1043_selector_str_label_valid_isolada() {
        // 164 - C1=T, C2=T, C3=T: s.len() >= 2 && starts_with('<') && ends_with('>') -> Selector::Label
        let mut c = ctx();
        let args = Args::positional(vec![Value::Str("<my-label>".into())]);
        let res = native_selector(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Selector(Selector::Label(l)) if l.0 == "my-label"));
    }

    #[test]
    fn p1043_selector_str_label_too_short_isolada() {
        // 164 - C1=F, C2=_, C3=_: s.len() < 2 -> falls through to Kind check -> Err
        let mut c = ctx();
        let args = Args::positional(vec![Value::Str("<".into())]);
        let res = native_selector(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }

    #[test]
    fn p1043_selector_str_element_kind_isolada() {
        // 164 - C1=T, C2=F, C3=_: s.len() >= 2 && !starts_with('<') -> Selector::Kind
        let mut c = ctx();
        let args = Args::positional(vec![Value::Str("heading".into())]);
        let res = native_selector(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Selector(Selector::Kind(ElementKind::Heading))));
    }

    #[test]
    fn p1043_selector_str_label_missing_close_isolada() {
        // 164 - C1=T, C2=T, C3=F: s.len() >= 2 && starts_with('<') && !ends_with('>') -> falls through -> Err
        let mut c = ctx();
        let args = Args::positional(vec![Value::Str("<heading".into())]);
        let res = native_selector(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }

    #[test]
    fn p1043_query_str_label_valid_isolada() {
        // 238 - C1=T, C2=T, C3=T: s.len() >= 2 && starts_with('<') && ends_with('>') -> Selector::Label
        let res = parse_selector_arg(&[Value::Str("<sec>".into())], "query").unwrap();
        assert!(matches!(res, Selector::Label(l) if l.0 == "sec"));
    }

    #[test]
    fn p1043_query_str_label_too_short_isolada() {
        // 238 - C1=F, C2=_, C3=_: s.len() < 2 -> falls through to Kind check -> Err
        let res = parse_selector_arg(&[Value::Str("<".into())], "query");
        assert!(res.is_err());
    }

    #[test]
    fn p1043_query_str_element_kind_isolada() {
        // 238 - C1=T, C2=F, C3=_: s.len() >= 2 && !starts_with('<') -> Selector::Kind
        let res = parse_selector_arg(&[Value::Str("figure".into())], "query").unwrap();
        assert!(matches!(res, Selector::Kind(ElementKind::Figure)));
    }

    #[test]
    fn p1043_query_str_label_missing_close_isolada() {
        // 238 - C1=T, C2=T, C3=F: s.len() >= 2 && starts_with('<') && !ends_with('>') -> falls through -> Err
        let res = parse_selector_arg(&[Value::Str("<figure".into())], "query");
        assert!(res.is_err());
    }

    #[test]
    fn p1285_native_selector_preserva_selector_por_identidade() {
        let expected = Selector::Where {
            base: Box::new(Selector::Kind(ElementKind::Heading)),
            field: "level".into(),
            value: Box::new(Value::Int(1)),
        };
        let args = Args::positional(vec![Value::Selector(expected.clone())]);
        let actual =
            native_selector(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap();

        assert_eq!(actual, Value::Selector(expected));
    }

    #[test]
    fn p1285_native_selector_converte_regex_valida_em_selector() {
        let regex = crate::entities::regex::Regex::new("f.o").unwrap();
        let args = Args::positional(vec![Value::Regex(regex.clone())]);
        let actual =
            native_selector(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap();

        assert_eq!(actual, Value::Selector(Selector::Regex(regex)));
    }

    #[test]
    fn p1285_native_selector_distingue_tres_diagnosticos_de_vazio() {
        fn message(value: Value) -> String {
            let args = Args::positional(vec![value]);
            native_selector(&mut ctx(), &args, &NullWorld::default(), tfid())
                .unwrap_err()
                .remove(0)
                .message
        }

        assert_eq!(message(Value::Str("".into())), "text selector is empty");
        assert_eq!(
            message(Value::Regex(crate::entities::regex::Regex::new("").unwrap())),
            "regex selector is empty"
        );
        assert_eq!(
            message(Value::Regex(crate::entities::regex::Regex::new("a*").unwrap())),
            "regex matches empty text"
        );
    }
}

fn element_kind_of_native_func(f: &crate::entities::func::Func) -> Option<ElementKind> {
    let addr = f.native_fn_addr()?;
    if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
        Some(ElementKind::Heading)
    } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
        Some(ElementKind::Figure)
    } else if fn_addr_eq(addr, native_table as fn(_, _, _, _) -> _) {
        Some(ElementKind::Table)
    } else if fn_addr_eq(addr, native_metadata_func as fn(_, _, _, _) -> _) {
        Some(ElementKind::Metadata)
    } else {
        None
    }
}

pub(super) fn parse_selector_arg(
    items: &[Value],
    func_name: &str,
) -> SourceResult<Selector> {
    let msg = |s: String| Err(vec![SourceDiagnostic::error(Span::detached(), s)]);
    match items {
        [Value::Str(s)]
            if matches!(
                (s.len() >= 2, s.starts_with('<'), s.ends_with('>')),
                (true, true, true)
            ) =>
        {
            Ok(Selector::Label(Label(s[1..s.len() - 1].to_string())))
        }
        [Value::Str(kind_str)] => ElementKind::from_name(kind_str.as_str())
            .map(Selector::Kind)
            .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(), format!(
                "{}(): kind '{}' não reconhecido (válidos: heading, figure, citation, metadata, state, state_update, outline, bibliography, equation, counter_update, table, list, enum, par, link, raw, quote, footnote). Para label, use `<nome>` syntax.",
                func_name, kind_str
            ))]),
        [Value::Location(loc)] => Ok(Selector::Location(*loc)),
        [Value::Selector(sel)] => Ok(sel.clone()),
        [Value::Label(label)] => Ok(Selector::Label(label.clone())),
        [Value::Func(f)] => element_kind_of_native_func(f)
            .map(Selector::Kind)
            .ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(), "only element functions can be used as selectors",
            )]),
        [other] => msg(format!(
            "{}() requer string ou location, recebeu {}. Tipos suportados: \"kind\", \"<label>\", Value::Location. (Regex requer P209D; And/Or ainda só Rust API.)",
            func_name, other.type_name()
        )),
        _ => msg(format!("{}() requer 1 argumento (selector), recebeu {}", func_name, items.len())),
    }
}
