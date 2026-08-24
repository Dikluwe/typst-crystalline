//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural.md
//! @prompt-hash 55a05906
//! @layer L1
//! @updated 2026-08-12
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

pub use bibliography::{native_bibliography, native_cite};
pub use divider::native_divider;
pub use document::{native_asset, native_document};
pub use footnote::native_footnote;
pub use heading::native_heading;
pub use lists::{native_enum, native_list, native_terms};
pub use markup::{native_emph, native_link, native_raw, native_strong};
pub(crate) use math::{equation_content, sqrt_content};
pub use math::{
    make_math_module, native_accent, native_cancel, native_math_class, native_op,
    native_underover,
};
pub use outline::{native_lof, native_lot, native_outline};
pub use par::native_par;
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

    use crate::compiler::eval::EvalContext;
    use crate::entities::args::Args;
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
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    fn call_document(args: Args) -> SourceResult<Value> {
        native_document(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
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
    fn native_document_title_content() {
        let mut args = Args::positional(vec![]);
        args.named
            .insert("title".into(), Value::Content(Content::text("Título")));
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { title, author, date, keywords }) = v
        else {
            panic!("esperado Content::Document, recebeu {:?}", v);
        };
        assert_eq!(title.as_ref().map(|b| b.plain_text()), Some("Título".to_string()));
        assert!(author.is_empty());
        assert!(date.is_none());
        assert!(keywords.is_empty());
    }

    #[test]
    fn native_document_author_str() {
        let args = named_args(&[("author", Value::Str("Ana".into()))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana")]);
    }

    #[test]
    fn native_document_author_array() {
        let args = named_args(&[(
            "author",
            Value::Array(vec![Value::Str("Ana".into()), Value::Str("Bob".into())]),
        )]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana"), EcoString::from("Bob")]);
    }

    #[test]
    fn native_document_keywords_str() {
        let args = named_args(&[("keywords", Value::Str("typst".into()))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("typst")]);
    }

    #[test]
    fn native_document_keywords_array() {
        let args = named_args(&[(
            "keywords",
            Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
        )]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("a"), EcoString::from("b")]);
    }

    #[test]
    fn native_document_date() {
        let dt = Datetime::new_date(2026, 6, 22).unwrap();
        let args = named_args(&[("date", Value::Datetime(dt))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { date, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(date, Some(dt));
    }

    #[test]
    fn native_document_no_args() {
        let args = Args::positional(vec![]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { title, author, date, keywords }) = v
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
        let args = named_args(&[("foo", Value::Str("x".into()))]);
        assert!(call_document(args).is_err());
    }

    #[test]
    fn native_document_rejects_bad_author_type() {
        let args = named_args(&[("author", Value::Int(1))]);
        assert!(call_document(args).is_err());
    }

    #[test]
    fn native_asset_path_positional() {
        let args = Args::positional(vec![Value::Str("logo.png".into())]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, kind }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path, EcoString::from("logo.png"));
        assert_eq!(kind, Some(EcoString::from("image")));
    }

    #[test]
    fn native_asset_path_named() {
        let args = named_args(&[("path", Value::Str("font.ttf".into()))]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, kind }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path, EcoString::from("font.ttf"));
        assert_eq!(kind, Some(EcoString::from("font")));
    }

    #[test]
    fn native_asset_kind_explicit() {
        let mut args = Args::positional(vec![Value::Str("x".into())]);
        args.named.insert("kind".into(), Value::Str("custom".into()));
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { kind, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(kind, Some(EcoString::from("custom")));
    }

    #[test]
    fn native_asset_kind_unknown_extension() {
        let args = Args::positional(vec![Value::Str("file.xyz".into())]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { kind, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(kind, None);
    }

    #[test]
    fn native_asset_rejects_missing_path() {
        let args = Args::positional(vec![Value::Int(1)]);
        assert!(call_asset(args).is_err());
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
    fn p806_par_body_content_devolvido_directamente() {
        // P806 — `#par[...]` como função (achado #12 de P798): o body é
        // devolvido directamente (parágrafos implícitos; standalone idêntico
        // ao vanilla).
        let args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        let v = call_par(args).unwrap();
        let Value::Content(c) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert_eq!(c.plain_text(), "abc");
    }

    #[test]
    fn p806_par_body_str_convertida() {
        let args = Args::positional(vec![Value::Str("abc".into())]);
        let v = call_par(args).unwrap();
        let Value::Content(c) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert_eq!(c.plain_text(), "abc");
    }

    #[test]
    fn p806_par_leading_embrulha_styled_custom() {
        // `leading:` viaja pelo mesmo canal custom do `#set par(leading:)`.
        use crate::entities::layout_types::Length;
        let mut args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        args.named.insert("leading".into(), Value::Length(Length::pt(20.0)));
        let v = call_par(args).unwrap();
        let Value::Content(Content::Styled(inner, _styles)) = v else {
            panic!("esperado Content::Styled, recebeu {:?}", v);
        };
        assert_eq!(inner.plain_text(), "abc");
    }

    #[test]
    fn p806_par_justify_aceite_e_ignorado() {
        // Propriedade vanilla conhecida mas não honrada: aceite sem erro.
        let mut args = Args::positional(vec![Value::Content(Content::text("abc"))]);
        args.named.insert("justify".into(), Value::Bool(true));
        let v = call_par(args).unwrap();
        let Value::Content(c) = v else {
            panic!("esperado Content, recebeu {:?}", v);
        };
        assert_eq!(c.plain_text(), "abc");
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
        let Value::Content(Content::Styled(inner, styles)) = v else {
            panic!("esperado Content::Styled, recebeu {:?}", v);
        };
        assert!(matches!(inner.as_ref(), Content::Heading(h) if h.level == 2));
        let custom = &styles.delta().custom;
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
        let args = Args { items: vec![], named, span: Span::detached() };
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
        let args = Args { items: vec![], named, span: Span::detached() };
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
}
