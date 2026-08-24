//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/selector.md
//! @prompt-hash fd563aa1
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
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
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
        [Value::Func(f)] => {
            let addr = f.native_fn_addr().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa esperada",
                )]
            })?;
            if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Heading)))
            } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Figure)))
            } else {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa não suportada como selector",
                )])
            }
        }
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
            _: Option<i64>,
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
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
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
