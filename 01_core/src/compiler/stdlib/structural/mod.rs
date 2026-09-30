//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural.md
//! @prompt-hash 59507c7a
//! @layer L1
//! @updated 2026-09-28
//!
//! Funções nativas estruturais do stdlib. Extraído de `stdlib.rs` no
//! Passo 96.5 conforme ADR-0037.
//!
//! Fatiado em hub + nós por domínio conforme ADR-0109. Como em `eval/bindings`,
//! o hub não tem tabela de despacho — `structural.rs` era um agregado plano de
//! 50 nativas, chamadas por nome a partir de `stdlib/mod.rs`. O hub re-exporta e
//! aloja a suite de testes, que é transversal aos nós (um único `TestWorld`
//! partilhado).
//!
//! 2026-08-13: `flow` e `sectioning` desfeitos em 7 nós. As duas fronteiras
//! estavam sustentadas por clusters de co-mudança que não existiam (artefacto de
//! atribuição da ferramenta); a medição corrigida deixou `outline`+`lof`+`lot`
//! como único núcleo real e as outras cinco nativas sozinhas. Os nós são
//! achatados aqui, sem sub-hub: manter directórios `flow/` ou `sectioning/`
//! reafirmaria o agrupamento que a medição refutou.

mod bibliography;
mod divider;
mod document;
mod footnote;
mod heading;
mod lists;
mod markup;
mod math;
mod outline;
mod par;
mod quote;
mod table_grid;
mod table_lines;
mod title;

#[cfg(test)]
#[path = "p1429_tests.rs"]
mod p1429_tests;

#[cfg(test)]
#[path = "p1430_tests.rs"]
mod p1430_tests;

#[cfg(test)]
#[path = "p1431_tests.rs"]
mod p1431_tests;

#[cfg(test)]
#[path = "p1436_tests.rs"]
mod p1436_tests;

#[cfg(test)]
#[path = "p1437_tests.rs"]
mod p1437_tests;

#[cfg(test)]
#[path = "p1438_tests.rs"]
mod p1438_tests;

#[cfg(test)]
#[path = "p1439_tests.rs"]
mod p1439_tests;

#[cfg(test)]
#[path = "p1440_tests.rs"]
mod p1440_tests;

#[cfg(test)]
#[path = "p1441_tests.rs"]
mod p1441_tests;
#[cfg(test)]
#[path = "p1442_tests.rs"]
mod p1442_tests;
#[cfg(test)]
mod p1443_tests;
#[cfg(test)]
mod p1444_tests;
#[cfg(test)]
mod p1445_tests;
#[cfg(test)]
mod p1446_tests;
#[cfg(test)]
mod p1447_tests;
#[cfg(test)]
mod p1448_tests;
#[cfg(test)]
mod p1449_tests;
#[cfg(test)]
mod p1450_tests;
#[cfg(test)]
mod p1451_tests;

pub use bibliography::{native_bibliography, native_cite};
pub use divider::native_divider;
pub use document::{native_asset, native_document};
pub use footnote::{native_footnote, native_footnote_entry};
pub use heading::native_heading;
pub use lists::{
    native_enum, native_enum_item, native_list, native_list_item, native_terms,
    native_terms_item,
};
pub use markup::{native_emph, native_link, native_raw, native_strong};
pub(crate) use math::abs::math_abs_from_args;
pub(crate) use math::accent::math_accent_content;
pub use math::accent::native_accent;
pub(crate) use math::annotations::{
    math_overbrace_from_args, math_overbracket_from_args, math_overparen_from_args,
    math_overshell_from_args, math_underbrace_from_args, math_underbracket_from_args,
    math_underparen_from_args, math_undershell_from_args,
};
pub use math::cancel::native_cancel;
pub(crate) use math::cases::{
    apply_math_cases_option, math_cases_delim, MathCasesOptions,
};
pub use math::class::native_math_class;
pub(crate) use math::equation::{
    equation_alt, equation_content, equation_number_align, equation_supplement,
};
pub(crate) use math::lr::math_lr_from_args;
pub use math::make_math_module;
pub(crate) use math::matrix::{
    apply_math_matrix_option, math_matrix_augment, MathMatrixOptions,
};
pub(crate) use math::mid::math_mid_from_args;
pub(crate) use math::norm::math_norm_from_args;
pub use math::op::native_op;
pub(crate) use math::overline::math_overline_from_args;
pub(crate) use math::root::sqrt_content;
pub(crate) use math::round::math_round_from_args;
pub(crate) use math::stretch::math_stretch_from_args;
pub use math::underover::native_underover;
pub(crate) use outline::realize_outline_entry_page;
pub use outline::{
    native_lof, native_lot, native_outline, native_outline_entry,
    native_outline_entry_body, native_outline_entry_indented, native_outline_entry_inner,
    native_outline_entry_page, native_outline_entry_prefix,
};
pub use par::{native_par, native_par_line, native_parbreak};
pub use quote::native_quote;
pub use table_grid::{
    native_grid_cell, native_grid_footer, native_grid_header, native_table,
    native_table_cell, native_table_footer, native_table_header,
};
pub use table_lines::{
    native_grid_hline, native_grid_vline, native_table_hline, native_table_vline,
};
pub use title::native_title;

#[cfg(test)]
mod tests {
    use super::*;

    use ecow::EcoString;

    use crate::entities::content::Content;

    use crate::compiler::eval::{EvalContext, EvalTarget};
    use crate::entities::args::Args;
    use crate::entities::bytes::Bytes as ValueBytes;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::source_result::SourceResult;
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    fn call_document(args: Args) -> SourceResult<Value> {
        let mut context = EvalContext::new();
        context.target = EvalTarget::Bundle;
        native_document(&mut context, &args, &NullWorld::default(), test_file_id())
    }

    fn call_asset(args: Args) -> SourceResult<Value> {
        native_asset(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn named_args(pairs: &[(&str, Value)]) -> Args {
        let mut args = Args::positional(vec![]);
        for (k, v) in pairs {
            args.named.insert((*k).into(), v.clone());
        }
        args
    }

    #[test]
    fn p1140_17_native_parbreak_contrato_de_argumentos() {
        let call = |args: &Args| {
            native_parbreak(
                &mut EvalContext::new(),
                args,
                &NullWorld::default(),
                test_file_id(),
            )
        };

        assert_eq!(
            call(&Args::positional(vec![])).unwrap(),
            Value::Content(Content::Parbreak),
        );
        let positional = call(&Args::positional(vec![Value::Int(1)])).unwrap_err();
        assert_eq!(positional[0].message, "unexpected argument");
        let named = call(&named_args(&[("foo", Value::Bool(true))])).unwrap_err();
        assert_eq!(named[0].message, "unexpected argument: foo");
    }

    fn document_args(named: &[(&str, Value)]) -> Args {
        let mut args = Args::positional(vec![
            Value::Str("a.pdf".into()),
            Value::Content(Content::text("Body")),
        ]);
        for (name, value) in named {
            args.named.insert((*name).into(), value.clone());
        }
        args
    }

    #[test]
    fn native_document_rejects_direct_construction_outside_bundle() {
        for target in [EvalTarget::Paged, EvalTarget::Html] {
            let mut context = EvalContext::new();
            context.target = target;
            let errors = native_document(
                &mut context,
                &document_args(&[]),
                &NullWorld::default(),
                test_file_id(),
            )
            .unwrap_err();
            assert_eq!(
                errors[0].message,
                "constructing a document is only supported in the bundle target",
                "{target:?}"
            );
            assert!(
                errors[0].hints.iter().any(|hint| hint.contains("bundle")),
                "{target:?}: {errors:?}"
            );
        }
    }
    #[test]
    fn native_document_path_body_and_title_content() {
        let v = call_document(document_args(&[(
            "title",
            Value::Content(Content::text("Título")),
        )]))
        .unwrap();
        let Value::Content(Content::Document {
            path,
            body,
            title,
            author,
            date,
            keywords,
            ..
        }) = v
        else {
            panic!("esperado Content::Document, recebeu {:?}", v);
        };
        assert_eq!(path.as_str(), "/a.pdf");
        assert_eq!(body.plain_text(), "Body");
        assert_eq!(title.as_ref().map(|b| b.plain_text()), Some("Título".to_string()));
        assert!(author.is_empty());
        assert!(date.is_none());
        assert!(keywords.is_empty());
    }

    #[test]
    fn native_document_author_str() {
        let v = call_document(document_args(&[("author", Value::Str("Ana".into()))]))
            .unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana")]);
    }

    #[test]
    fn native_document_author_array() {
        let v = call_document(document_args(&[(
            "author",
            Value::Array(vec![Value::Str("Ana".into()), Value::Str("Bob".into())]),
        )]))
        .unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana"), EcoString::from("Bob")]);
    }

    #[test]
    fn native_document_keywords_str() {
        let v = call_document(document_args(&[("keywords", Value::Str("typst".into()))]))
            .unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("typst")]);
    }

    #[test]
    fn native_document_keywords_array() {
        let v = call_document(document_args(&[(
            "keywords",
            Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
        )]))
        .unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("a"), EcoString::from("b")]);
    }

    #[test]
    fn native_document_date() {
        let dt = Datetime::new_date(2026, 6, 22).unwrap();
        let v = call_document(document_args(&[("date", Value::Datetime(dt))])).unwrap();
        let Value::Content(Content::Document { date, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(date, Some(dt));
    }

    #[test]
    fn native_document_requires_path_and_body_and_preserves_metadata_defaults() {
        let missing_path = call_document(Args::positional(vec![])).unwrap_err();
        assert_eq!(missing_path[0].message, "missing argument: path");
        let missing_body =
            call_document(Args::positional(vec![Value::Str("a.pdf".into())]))
                .unwrap_err();
        assert_eq!(missing_body[0].message, "missing argument: body");

        let v = call_document(document_args(&[])).unwrap();
        let Value::Content(Content::Document { title, author, date, keywords, .. }) = v
        else {
            panic!("esperado Document")
        };
        assert!(title.is_none());
        assert!(author.is_empty());
        assert!(date.is_none());
        assert!(keywords.is_empty());
    }

    #[test]
    fn native_document_rejects_unknown_named() {
        let errors =
            call_document(document_args(&[("foo", Value::Str("x".into()))])).unwrap_err();
        assert!(errors[0].message.contains("foo"), "{errors:?}");
    }

    #[test]
    fn native_document_rejects_bad_author_type() {
        let errors =
            call_document(document_args(&[("author", Value::Int(1))])).unwrap_err();
        assert!(errors[0].message.contains("author"), "{errors:?}");
    }

    #[test]
    fn native_asset_path_positional_and_bytes_intact() {
        let args = Args::positional(vec![
            Value::Str("logo.png".into()),
            Value::Bytes(ValueBytes::new(vec![0, 255, 10])),
        ]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, data, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path.as_str(), "/logo.png");
        assert_eq!(data.as_ref(), &[0, 255, 10]);
    }

    #[test]
    fn native_asset_path_named_is_rejected() {
        let mut args = Args::positional(vec![Value::Str("font.ttf".into())]);
        args.named.insert("path".into(), Value::Str("other.ttf".into()));
        args.items.push(Value::Str("payload".into()));
        let errors = call_asset(args).unwrap_err();
        assert!(errors[0].message.contains("positional"), "{errors:?}");
    }

    #[test]
    fn native_asset_kind_explicit_is_rejected() {
        let mut args =
            Args::positional(vec![Value::Str("x".into()), Value::Str("payload".into())]);
        args.named.insert("kind".into(), Value::Str("custom".into()));
        let errors = call_asset(args).unwrap_err();
        assert_eq!(errors[0].message, "unexpected argument: kind");
    }

    #[test]
    fn native_asset_unknown_extension_preserves_utf8_data() {
        let args = Args::positional(vec![
            Value::Str("file.xyz".into()),
            Value::Str("café".into()),
        ]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, data, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path.as_str(), "/file.xyz");
        assert_eq!(data.as_ref(), "café".as_bytes());
    }

    #[test]
    fn native_asset_rejects_missing_path_data_and_bad_path_type() {
        let missing_path = call_asset(Args::positional(vec![])).unwrap_err();
        assert_eq!(missing_path[0].message, "missing argument: path");
        let missing_data =
            call_asset(Args::positional(vec![Value::Str("x".into())])).unwrap_err();
        assert_eq!(missing_data[0].message, "missing argument: data");
        let bad_path = call_asset(Args::positional(vec![
            Value::Int(1),
            Value::Str("payload".into()),
        ]))
        .unwrap_err();
        assert_eq!(bad_path[0].message, "expected string, found integer");
    }

    fn call_heading(args: Args) -> SourceResult<Value> {
        native_heading(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    // ── P806 — `native_par` ───────────────────────────────────────────────

    fn call_par(args: Args) -> SourceResult<Value> {
        native_par(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    #[test]
    fn p806_par_body_content_preserva_elemento() {
        // O constructor conserva a identidade de par e o conteúdo.
        let args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        let v = call_par(args).unwrap();
        let Value::Content(c) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert!(matches!(c, Content::Par { .. }));
        assert_eq!(c.plain_text(), "abc");
    }

    #[test]
    fn p806_par_body_str_convertida() {
        let args = Args::positional(vec![Value::Str("abc".into())]);
        let v = call_par(args).unwrap();
        let Value::Content(c) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert!(matches!(c, Content::Par { .. }));
        assert_eq!(c.plain_text(), "abc");
    }

    #[test]
    fn p806_par_leading_embrulha_styled_custom() {
        // `leading:` viaja pelo mesmo canal custom do `#set par(leading:)`.
        use crate::entities::layout_types::Length;
        let mut args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        args.named.insert("leading".into(), Value::Length(Length::pt(20.0)));
        let v = call_par(args).unwrap();
        let Value::Content(Content::Styled(styled)) = v else {
            panic!("esperado Content::Styled, recebeu {:?}", v);
        };
        assert!(matches!(styled.body, Content::Par { .. }));
        assert_eq!(styled.body.plain_text(), "abc");
    }

    #[test]
    fn p806_par_justify_transportado() {
        // A opção lexical é transportada sem perder o texto do body.
        let mut args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        args.named.insert("justify".into(), Value::Bool(true));
        let v = call_par(args).unwrap();
        let Value::Content(Content::Styled(c)) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert!(matches!(c.body, Content::Par { .. }));
        assert_eq!(c.body.plain_text(), "abc");
        assert!(c
            .styles
            .delta()
            .custom
            .iter()
            .any(|(k, v)| k == "par.justify" && v == &Value::Bool(true)));
    }

    #[test]
    fn p806_par_sem_body_erro_literal_vanilla() {
        let args = Args::positional(vec![]);
        let err = call_par(args).unwrap_err();
        assert!(
            format!("{:?}", err).contains("missing argument: body"),
            "mensagem literal do vanilla, obtido: {:?}",
            err
        );
    }

    #[test]
    fn p806_par_tipo_errado_erro_expected_content() {
        let args = Args::positional(vec![Value::Int(5)]);
        let err = call_par(args).unwrap_err();
        assert!(
            format!("{:?}", err).contains("expected content, found int"),
            "padrão expected/found, obtido: {:?}",
            err
        );
    }

    #[test]
    fn p806_par_named_desconhecido_erro() {
        let mut args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        args.named.insert("foo".into(), Value::Int(1));
        let err = call_par(args).unwrap_err();
        assert!(
            format!("{:?}", err).contains("argumento nomeado inesperado em par(): 'foo'"),
            "obtido: {:?}",
            err
        );
    }

    #[test]
    fn native_heading_sem_numbering_emite_heading_simples() {
        let args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("Título")),
        ]);
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert_eq!(h.level, 1);
        assert_eq!(h.body.plain_text(), "Título");
    }

    #[test]
    fn native_heading_com_numbering_emite_styled() {
        let mut args = Args::positional(vec![Value::Int(2), Value::Str("Sub".into())]);
        args.named.insert("numbering".into(), Value::Str("1.1".into()));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Styled(styled)) = v else {
            panic!("esperado Content::Styled, recebeu {:?}", v);
        };
        assert!(matches!(&styled.body, Content::Heading(h) if h.level == 2));
        let custom = &styled.styles.delta().custom;
        assert!(
            custom
                .iter()
                .any(|(k, v)| k == "heading.numbering" && matches!(v, Value::Bool(true))),
            "gate heading.numbering deve estar activo"
        );
        assert!(
            custom.iter().any(|(k, v)| k == "heading.numbering.pattern"
                && matches!(v, Value::Str(s) if s == "1.1")),
            "pattern deve ser transportado na chain"
        );
    }

    #[test]
    fn native_heading_rejeita_level_fora_do_range() {
        let args =
            Args::positional(vec![Value::Int(0), Value::Content(Content::text("X"))]);
        assert!(call_heading(args).is_err());
    }

    #[test]
    fn native_heading_rejeita_body_invalido() {
        let args = Args::positional(vec![Value::Int(1), Value::Int(42)]);
        assert!(call_heading(args).is_err());
    }

    #[test]
    fn native_heading_outlined_false_mantem_bookmarked_auto() {
        let mut args =
            Args::positional(vec![Value::Int(1), Value::Content(Content::text("X"))]);
        args.named.insert("outlined".into(), Value::Bool(false));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(!h.outlined);
        assert_eq!(h.bookmarked, None, "bookmarked deve ficar auto (None)");
        assert!(!h.is_bookmarked(), "bookmarked efectivo segue outlined");
    }

    #[test]
    fn native_heading_bookmarked_false_mantem_outlined_true() {
        let mut args =
            Args::positional(vec![Value::Int(1), Value::Content(Content::text("X"))]);
        args.named.insert("bookmarked".into(), Value::Bool(false));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(h.outlined, "outlined default deve permanecer true");
        assert_eq!(h.bookmarked, Some(false));
        assert!(!h.is_bookmarked());
    }

    #[test]
    fn native_heading_outlined_false_bookmarked_true_separados() {
        let mut args =
            Args::positional(vec![Value::Int(1), Value::Content(Content::text("X"))]);
        args.named.insert("outlined".into(), Value::Bool(false));
        args.named.insert("bookmarked".into(), Value::Bool(true));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(!h.outlined);
        assert_eq!(h.bookmarked, Some(true));
        assert!(h.is_bookmarked());
    }

    #[test]
    fn native_heading_rejeita_outlined_nao_bool() {
        let mut args =
            Args::positional(vec![Value::Int(1), Value::Content(Content::text("X"))]);
        args.named.insert("outlined".into(), Value::Int(1));
        assert!(call_heading(args).is_err());
    }

    // ── Testes native_list / native_enum (P470) ──────────────────────────────

    fn call_list(args: Args) -> SourceResult<Value> {
        native_list(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    fn call_enum(args: Args) -> SourceResult<Value> {
        native_enum(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    #[test]
    fn list_sem_itens_devolve_empty() {
        let result = call_list(Args::positional(vec![])).unwrap();
        assert_eq!(result, Value::Content(Content::Empty));
    }

    #[test]
    fn list_dois_itens_sem_marker() {
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let result = call_list(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            assert_eq!(seq.len(), 2);
            for item in seq.iter() {
                if let Content::ListItem(li) = item {
                    assert_eq!(li.marker, None);
                } else {
                    panic!("esperado ListItem");
                }
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn list_com_marker_custom() {
        use crate::entities::list_marker::ListMarker;
        let mut named = indexmap::IndexMap::default();
        named.insert("marker".into(), Value::Str("→".into()));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("x"))],
            named,
            span: Span::detached(),
        };
        let result = call_list(args).unwrap();
        if let Value::Content(Content::ListItem(li)) = result {
            assert_eq!(li.marker, Some(ListMarker::Custom("→".into())));
        } else {
            panic!("esperado ListItem com marker");
        }
    }

    #[test]
    fn list_marker_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("marker".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("x"))],
            named,
            span: Span::detached(),
        };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn enum_sem_itens_devolve_empty() {
        let result = call_enum(Args::positional(vec![])).unwrap();
        assert_eq!(result, Value::Content(Content::Empty));
    }

    #[test]
    fn enum_dois_itens_sem_numbering() {
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            assert_eq!(seq.len(), 2);
            let first = &seq[0];
            if let Content::EnumItem(ei) = first {
                assert_eq!(ei.number, Some(1));
                assert_eq!(ei.numbering, None);
            } else {
                panic!("esperado EnumItem");
            }
            let second = &seq[1];
            if let Content::EnumItem(ei) = second {
                assert_eq!(ei.number, Some(2));
            } else {
                panic!("esperado EnumItem");
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn enum_com_lower_alpha() {
        use crate::entities::enum_numbering::EnumNumbering;
        let mut named = indexmap::IndexMap::default();
        named.insert("numbering".into(), Value::Str("a)".into()));
        let args = Args {
            occurrences: None,
            items: vec![
                Value::Content(Content::text("x")),
                Value::Content(Content::text("y")),
            ],
            named,
            span: Span::detached(),
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            if let Content::EnumItem(ei) = &seq[0] {
                assert_eq!(ei.numbering, Some(EnumNumbering::LowerAlpha));
            } else {
                panic!("esperado EnumItem");
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn enum_numbering_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("numbering".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("x"))],
            named,
            span: Span::detached(),
        };
        assert!(call_enum(args).is_err());
    }

    #[test]
    fn enum_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        assert!(call_enum(args).is_err());
    }

    // ── Testes P505 — indentação e tight em list/enums ───────────────────────

    #[test]
    fn list_indent_body_indent_tight_sao_propagados() {
        use crate::entities::layout_types::Length;
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Length(Length::em(1.5)));
        named.insert("body-indent".into(), Value::Length(Length::em(0.5)));
        named.insert("tight".into(), Value::Bool(false));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_list(args).unwrap();
        if let Value::Content(Content::ListItem(li)) = result {
            assert_eq!(li.indent, Some(Length::em(1.5)));
            assert_eq!(li.body_indent, Some(Length::em(0.5)));
            assert_eq!(li.tight, Some(false));
        } else {
            panic!("esperado ListItem");
        }
    }

    #[test]
    fn list_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Str("x".into()));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_body_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("body-indent".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_tight_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("tight".into(), Value::Int(1));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn enum_indent_body_indent_tight_sao_propagados() {
        use crate::entities::layout_types::Length;
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Length(Length::em(1.5)));
        named.insert("body-indent".into(), Value::Length(Length::em(0.5)));
        named.insert("tight".into(), Value::Bool(false));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::EnumItem(ei)) = result {
            assert_eq!(ei.indent, Some(Length::em(1.5)));
            assert_eq!(ei.body_indent, Some(Length::em(0.5)));
            assert_eq!(ei.tight, Some(false));
        } else {
            panic!("esperado EnumItem");
        }
    }

    #[test]
    fn enum_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Str("x".into()));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        assert!(call_enum(args).is_err());
    }

    #[test]
    fn enum_start_maior_que_um_propaga_numeracao() {
        let mut named = indexmap::IndexMap::default();
        named.insert("start".into(), Value::Int(3));
        let args = Args {
            occurrences: None,
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::EnumItem(ei)) = result {
            assert_eq!(ei.number, Some(3));
        } else {
            panic!("esperado EnumItem");
        }
    }

    // ── Passo 512 — grid/table hline/vline ───────────────────────────────────

    fn call_grid_hline(args: Args) -> SourceResult<Value> {
        native_grid_hline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_table_hline(args: Args) -> SourceResult<Value> {
        native_table_hline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_grid_vline(args: Args) -> SourceResult<Value> {
        native_grid_vline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_table_vline(args: Args) -> SourceResult<Value> {
        native_table_vline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    #[test]
    fn grid_hline_defaults() {
        let v = call_grid_hline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::GridHLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.row, 0);
            assert_eq!(e.position.as_str(), "auto");
            assert_eq!(e.stroke.as_ref().expect("stroke default").thickness, 1.0);
        } else {
            panic!("esperado GridHLine");
        }
    }

    #[test]
    fn grid_hline_position_bottom_alignment() {
        use crate::entities::layout_types::{Align2D, VAlign};
        let mut args = Args::positional(vec![]);
        args.named.insert(
            "position".into(),
            Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }),
        );
        let v = call_grid_hline(args).unwrap();
        if let Value::Content(Content::GridHLine(e)) = v {
            assert_eq!(e.position.as_str(), "bottom");
        } else {
            panic!("esperado GridHLine");
        }
    }

    #[test]
    fn grid_vline_defaults() {
        let v = call_grid_vline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::GridVLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.col, 0);
            assert_eq!(e.position.as_str(), "left");
        } else {
            panic!("esperado GridVLine");
        }
    }

    #[test]
    fn table_hline_defaults() {
        let v = call_table_hline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::TableHLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.row, 0);
            assert_eq!(e.position.as_str(), "auto");
        } else {
            panic!("esperado TableHLine");
        }
    }

    #[test]
    fn table_vline_defaults() {
        let v = call_table_vline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::TableVLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.col, 0);
            assert_eq!(e.position.as_str(), "left");
        } else {
            panic!("esperado TableVLine");
        }
    }

    /// **P895** (Parte B — catálogo de terceiros) — `math.op(...)` deve
    /// resolver ao mesmo `native_op` já registado no scope global (`op(...)`
    /// bare), não apenas no scope global — vanilla reexpõe a função também
    /// como `math.op`.
    #[test]
    fn p895_math_op_existe_no_modulo_math() {
        let module = match make_math_module() {
            Value::Module(m) => m,
            other => {
                panic!("make_math_module() deve devolver Value::Module, obteve {other:?}")
            }
        };
        let op = module.scope().get("op");
        assert!(
            matches!(op, Some(Value::Func(_))),
            "math.op deve existir como Value::Func, obteve {op:?}"
        );
    }

    // ── P825 (sub-achado A de P810 §12) — domínio do cast de MathClass ────
    //
    // O vanilla (`foundations/cast.rs:502-520`) aceita apenas 10 strings em
    // `math.class(str, body)`; `alphabetic`, `diacritic`, `glyph-part`,
    // `space` e `special` são variantes INTERNAS (atribuídas automaticamente
    // a símbolos), rejeitadas no cast com a mensagem verbatim
    // `expected "normal", ..., or "vary"`. Medido em P825 (sonda
    // `temp/p825/cls-*.typ`): o cristalino compilava as 15.

    const CAST_MSG: &str = "expected \"normal\", \"punctuation\", \"opening\", \
         \"closing\", \"fence\", \"large\", \"relation\", \"unary\", \"binary\", \
         or \"vary\"";

    fn call_class(args: Args) -> SourceResult<Value> {
        native_math_class(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn class_args(class: Value) -> Args {
        Args {
            occurrences: None,
            items: vec![class, Value::Str("x".into())],
            named: indexmap::IndexMap::default(),
            span: Span::detached(),
        }
    }

    #[test]
    fn p825a_aceita_as_10_classes_do_cast_vanilla() {
        for name in [
            "normal",
            "punctuation",
            "opening",
            "closing",
            "fence",
            "large",
            "relation",
            "unary",
            "binary",
            "vary",
        ] {
            assert!(
                call_class(class_args(Value::Str(name.into()))).is_ok(),
                "{name} deve ser aceite (cast vanilla)"
            );
        }
    }

    #[test]
    fn p825a_rejeita_variantes_internas_fora_do_cast() {
        for name in ["alphabetic", "diacritic", "glyph-part", "space", "special"] {
            let err = call_class(class_args(Value::Str(name.into())))
                .expect_err(&format!("{name} deve ser rejeitado (cast vanilla)"));
            assert!(
                err[0].message.contains(CAST_MSG),
                "{name}: mensagem deve ser o cast verbatim do vanilla; obteve: {}",
                err[0].message
            );
            assert!(
                !err[0].message.contains(", found"),
                "string fora do domínio não leva sufixo found: {}",
                err[0].message
            );
        }
    }

    #[test]
    fn p825a_nome_desconhecido_tem_mesmo_erro_de_cast() {
        let err = call_class(class_args(Value::Str("banana".into()))).unwrap_err();
        assert!(err[0].message.contains(CAST_MSG), "obteve: {}", err[0].message);
    }

    #[test]
    fn p825a_arg_nao_string_reporta_tipo_vanilla() {
        // Vanilla: `expected "normal", ..., or "vary", found integer`.
        let err = call_class(class_args(Value::Int(3))).unwrap_err();
        assert!(err[0].message.contains(CAST_MSG), "obteve: {}", err[0].message);
        assert!(
            err[0].message.contains(", found integer"),
            "sufixo de tipo vanilla; obteve: {}",
            err[0].message
        );
    }
    #[test]
    fn break_policy_native_par_validates_and_transports_justify() {
        let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
        args.named.insert("justify".into(), Value::Bool(true));
        let Value::Content(Content::Styled(s)) = call_par(args.clone()).unwrap() else {
            panic!("policy not captured")
        };
        assert!(s
            .styles
            .delta()
            .custom
            .iter()
            .any(|(k, v)| k == "par.justify" && v == &Value::Bool(true)));
        assert_eq!(s.body.plain_text(), "x");
        args.named.insert("justify".into(), Value::Int(1));
        assert_eq!(
            call_par(args).unwrap_err()[0].message,
            "expected boolean, found integer"
        );
    }

    #[test]
    fn break_policy_native_par_checks_strategy_without_claiming_optimization() {
        for value in
            [Value::Auto, Value::Str("simple".into()), Value::Str("optimized".into())]
        {
            let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
            args.named.insert("linebreaks".into(), value);
            assert_eq!(
                match call_par(args).unwrap() {
                    Value::Content(c) => c.plain_text(),
                    _ => panic!("content expected"),
                },
                "x"
            );
        }
        let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
        args.named.insert("linebreaks".into(), Value::Str("nonsense".into()));
        assert_eq!(
            call_par(args).unwrap_err()[0].message,
            "expected \"simple\", \"optimized\", or auto"
        );
    }
}
