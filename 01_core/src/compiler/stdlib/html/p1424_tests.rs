//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/p1424-tests.md
//! @prompt-hash f5ae1796
//! @layer L1
#![cfg(test)]
//!
//! Testes A/B independentes P1424. O implementador faz somente o wiring deste
//! módulo depois do selo; este ficheiro não lê símbolos privados candidatos.

use super::*;

use crate::compiler::eval::repr_value;
use crate::compiler::layout::layout;
use crate::entities::func::FuncRepr;

use std::num::NonZeroU16;

#[derive(Default)]
struct P1424World {
    library: crate::entities::world_types::Library,
    book: crate::entities::font_book::FontBook,
}

impl crate::contracts::world::World for P1424World {
    fn library(&self) -> &crate::entities::world_types::Library {
        &self.library
    }
    fn book(&self) -> &crate::entities::font_book::FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        p1424_file_id()
    }
    fn source(
        &self,
        _: FileId,
    ) -> crate::entities::world_types::FileResult<crate::entities::source::Source> {
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

fn p1424_file_id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1424).unwrap())
}

fn frame_binding() -> Value {
    make_html_module()
        .scope()
        .get("frame")
        .cloned()
        .expect("html.frame deve existir no módulo HTML")
}

fn call_frame(args: Args) -> SourceResult<Value> {
    let Value::Func(func) = frame_binding() else {
        panic!("html.frame deve ser função nativa")
    };
    let FuncRepr::Native(native) = func.repr() else {
        panic!("html.frame deve conservar a chamada nativa")
    };
    (native.call)(&mut EvalContext::new(), &args, &P1424World::default(), p1424_file_id())
}

fn frame(value: Value) -> Arc<crate::entities::html::HtmlFrameElem> {
    match value {
        Value::Content(Content::HtmlFrame(elem)) => elem,
        Value::Content(Content::HtmlElem(_)) => panic!("html.frame não é HtmlElem"),
        other => panic!("esperava Content::HtmlFrame, recebeu {other:?}"),
    }
}

fn diagnostics(args: Args) -> String {
    call_frame(args)
        .expect_err("a chamada inválida deve falhar")
        .into_iter()
        .map(|diag| diag.message)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn p1424_ab_binding_assinatura_e_carrier_distinto() {
    assert_eq!(repr_value(&frame_binding()), "frame");
    assert!(!TYPED_TAGS.iter().any(|(name, _)| *name == "frame"));

    let body = Content::text("x");
    let elem =
        frame(call_frame(Args::positional(vec![Value::Content(body.clone())])).unwrap());
    assert_eq!(elem.body, body);

    let repr = repr_value(&Value::Content(Content::HtmlFrame(elem.clone())));
    assert!(repr.starts_with("frame(body:"), "morfologia inesperada: {repr}");
    assert!(!repr.contains("html.elem"));

    assert!(diagnostics(Args::positional(vec![])).contains("missing argument: body"));

    let mut named = Args::positional(vec![]);
    named.named.insert("body".into(), Value::Content(Content::text("x")));
    assert!(
        diagnostics(named).contains("the argument `body` is positional"),
        "named body deve preservar o diagnóstico público"
    );

    assert!(diagnostics(Args::positional(vec![
        Value::Content(Content::text("x")),
        Value::Content(Content::text("y")),
    ]))
    .contains("unexpected argument"));
}

#[test]
fn p1424_ab_walkers_preservam_variante_e_descem_no_body() {
    let original = Content::HtmlFrame(Arc::new(crate::entities::html::HtmlFrameElem {
        body: Content::text("abc"),
    }));
    assert_eq!(original.elem_name(), "frame");
    assert_eq!(original.plain_text(), "abc");

    let mapped_text = original.map_text(&mut |text| text.to_uppercase());
    assert!(
        matches!(&mapped_text, Content::HtmlFrame(elem) if elem.body == Content::text("ABC")),
        "map_text deve preservar HtmlFrame e transformar seu body"
    );

    let mapped_content = original
        .map_content(&mut |node| {
            Ok(match node {
                Content::Text(text) if text == "abc" => Some(Content::text("walked")),
                _ => None,
            })
        })
        .unwrap();
    assert!(
        matches!(&mapped_content, Content::HtmlFrame(elem) if elem.body == Content::text("walked")),
        "map_content deve preservar HtmlFrame e visitar seu body"
    );
}

#[test]
fn p1424_ab_layout_paginado_simples_e_aninhado_sao_transparentes() {
    let body = Content::text("transparente");
    let simple = Content::HtmlFrame(Arc::new(crate::entities::html::HtmlFrameElem {
        body: body.clone(),
    }));
    let nested = Content::HtmlFrame(Arc::new(crate::entities::html::HtmlFrameElem {
        body: simple.clone(),
    }));

    let bare_doc = layout(&body);
    let simple_doc = layout(&simple);
    let nested_doc = layout(&nested);

    let projection = |doc: &crate::entities::layout_types::PagedDocument| {
        doc.pages
            .iter()
            .map(|page| {
                (page.width, page.height, page.plain_text(), format!("{:?}", page.items))
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(projection(&simple_doc), projection(&bare_doc));
    assert_eq!(projection(&nested_doc), projection(&bare_doc));
}
