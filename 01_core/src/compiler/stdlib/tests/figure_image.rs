//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md
//! @prompt-hash 7931e164
//! @layer L1
//! @updated 2026-09-26

use super::*;

fn assert_omitted_figure_kind_previews_as(
    figure: &std::sync::Arc<crate::entities::elements::figure::FigureElem>,
    function: crate::entities::func::Func,
) {
    use crate::entities::content::Content;
    use crate::entities::elements::figure::FigureKind;
    assert_eq!(figure.kind, None, "construction must retain lexical omission");
    assert_eq!(figure.resolved_kind, None);
    assert_eq!(figure.resolved_supplement, None);
    let preview = crate::compiler::realize::figure::preview(
        &Content::Figure(figure.clone()),
        &crate::entities::style_chain::StyleChain::default_chain(),
    )
    .unwrap();
    let Content::Figure(prepared) = preview else { panic!("expected Figure preview") };
    assert_eq!(prepared.resolved_kind, Some(FigureKind::Element(function)));
    assert_eq!(prepared.kind, None, "preview must preserve lexical omission");
    assert_eq!(
        prepared.resolved_supplement, None,
        "kind preview cannot prepare supplement effects"
    );
}

#[test]
fn native_figure_com_body_e_caption() {
    null_ctx!(ctx);
    use crate::entities::content::Content;
    let body_content = Content::text("Gráfico");
    let caption_content = Content::text("Legenda");
    let args = pn(
        vec![Value::Content(body_content)],
        "caption",
        Value::Content(caption_content),
    );
    let result = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_some()),
        "figure com caption deve ter Some(caption): {:?}",
        result
    );
}

#[test]
fn native_figure_sem_caption() {
    null_ctx!(ctx);
    use crate::entities::content::Content;
    let body_content = Content::text("Diagrama");
    let args = p(vec![Value::Content(body_content)]);
    let result = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_none()),
        "figure sem caption deve ter None: {:?}",
        result
    );
}

#[test]
fn native_figure_caption_none_value() {
    null_ctx!(ctx);
    use crate::entities::content::Content;
    // caption: none → ausência de legenda
    let body_content = Content::text("Corpo");
    let args = pn(vec![Value::Content(body_content)], "caption", Value::None);
    let result = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_none()),
        "figure com caption: none deve ter caption None"
    );
}

#[test]
fn native_figure_sem_body_retorna_err() {
    null_ctx!(ctx);
    let args = p(vec![]);
    assert!(
        native_figure(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
        "figure sem body deve retornar Err"
    );
}

#[test]
fn figure_auto_detect_image() {
    // Lexical omission is retained; the occurrence preview infers image identity.
    null_ctx!(ctx);
    use crate::entities::content::Content;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;
    let img =
        Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
    let args = p(vec![Value::Content(img)]);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_omitted_figure_kind_previews_as(
            &e,
            crate::entities::func::Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ),
        );
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_auto_detect_table() {
    // The occurrence preview infers the actual table element identity.
    null_ctx!(ctx);
    use crate::entities::content::Content;
    use crate::entities::layout_types::TrackSizing;
    let tab = Content::table(vec![TrackSizing::Auto], vec![], vec![]);
    let args = p(vec![Value::Content(tab)]);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_omitted_figure_kind_previews_as(
            &e,
            crate::entities::func::Func::native(
                "table",
                crate::compiler::stdlib::native_table,
            ),
        );
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_auto_detect_raw() {
    // The occurrence preview infers the actual raw element identity.
    null_ctx!(ctx);
    use crate::entities::content::Content;
    let raw = Content::raw("fn x() {}", None, false);
    let args = p(vec![Value::Content(raw)]);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_omitted_figure_kind_previews_as(
            &e,
            crate::entities::func::Func::native(
                "raw",
                crate::compiler::stdlib::native_raw,
            ),
        );
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_kind_explicit_override_auto_detect() {
    // P158A: `kind:` explícito vence auto-detecção
    // (precedência absoluta).
    null_ctx!(ctx);
    use crate::entities::content::Content;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;
    let img =
        Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
    let args = pn(vec![Value::Content(img)], "kind", Value::Str("custom-kind".into()));
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_eq!(
            e.kind,
            Some(Some(crate::entities::elements::figure::FigureKind::Name(
                "custom-kind".into()
            )))
        );
        assert_eq!(e.resolved_kind, None);
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_kind_auto_explicito_devolve_none() {
    // P158C: `kind: auto` explícito produz None (paridade
    // ADR-0064 Caso A: vanilla Auto ↔ cristalino None).
    null_ctx!(ctx);
    use crate::entities::content::Content;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;
    let img =
        Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
    let args = pn(vec![Value::Content(img)], "kind", Value::Auto);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_eq!(e.kind, Some(None), "explicit auto must remain lexically present");
        assert_eq!(e.resolved_kind, None);
        let Content::Figure(preview) = crate::compiler::realize::figure::preview(
            &Content::Figure(e.clone()),
            &crate::entities::style_chain::StyleChain::default_chain(),
        )
        .unwrap() else {
            panic!("expected Figure preview")
        };
        assert_eq!(
            preview.resolved_kind,
            Some(crate::entities::elements::figure::FigureKind::Element(
                crate::entities::func::Func::native(
                    "image",
                    crate::compiler::stdlib::native_image
                )
            ))
        );
        assert_eq!(preview.kind, Some(None));
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_kind_none_quando_body_nao_detectavel() {
    // A noninferable body retains omission and previews the actual image default.
    null_ctx!(ctx);
    use crate::entities::content::Content;
    let args = p(vec![Value::Content(Content::text("apenas texto"))]);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_omitted_figure_kind_previews_as(
            &e,
            crate::entities::func::Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ),
        );
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn figure_auto_detect_image_dentro_de_sequence() {
    // P158A §8: recursão limitada a Sequence — figure([
    // ..., image(...), ...]) detecta Image via primeiro
    // child detectável.
    null_ctx!(ctx);
    use crate::entities::content::Content;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;
    let img =
        Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
    // Sequence começa com Text (não detectável) e contém Image.
    let seq = Content::Sequence(Arc::from(vec![Content::text("legenda"), img]));
    let args = p(vec![Value::Content(seq)]);
    let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Figure(e)) = r {
        assert_omitted_figure_kind_previews_as(
            &e,
            crate::entities::func::Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ),
        );
    } else {
        panic!("esperado Content::Figure");
    }
}

#[test]
fn native_image_retorna_content_image() {
    let mut world = NullWorld::default();
    // P772p — `native_image` valida a assinatura do formato; bytes
    // arbitrários já não passam (antes eram aceites sem verificação).
    world.files.insert(
        "foto.png".to_string(),
        std::sync::Arc::new(vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
    );
    let _dummy_id =
        crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("foto.png".into())]);
    let result = native_image(&mut ctx, &args, &world, test_file_id()).unwrap();
    assert!(matches!(result, Value::Content(Content::Image(_))));
}

#[test]
fn native_image_ficheiro_inexistente_gera_erro() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("naoexiste.png".into())]);
    assert!(native_image(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn native_image_formato_desconhecido_gera_erro() {
    let mut world = NullWorld::default();
    world.files.insert(
        "bogus.png".to_string(),
        std::sync::Arc::new(b"\x00\x01\x02\x03garbage-not-an-image".to_vec()),
    );
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("bogus.png".into())]);
    let err = native_image(&mut ctx, &args, &world, test_file_id()).unwrap_err();
    assert_eq!(err[0].message, "unknown image format");
}

#[test]
fn native_image_svg_gera_erro_nao_suportado() {
    let mut world = NullWorld::default();
    world.files.insert(
        "test.svg".to_string(),
        std::sync::Arc::new(b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_vec()),
    );
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("test.svg".into())]);
    let err = native_image(&mut ctx, &args, &world, test_file_id()).unwrap_err();
    assert_eq!(err[0].message, "SVG images are not supported yet");
}

#[test]
fn p835_image_page_positivo_aceito_sem_efeito() {
    // Vanilla: `#image("x.png", page: 2)` compila (ignorado — só afecta PDF).
    let world = world_com_png();
    let mut ctx = EvalContext::new();
    let args = pn(vec![Value::Str("ok.png".into())], "page", Value::Int(2));
    let r = native_image(&mut ctx, &args, &world, test_file_id());
    assert!(matches!(r, Ok(Value::Content(_))), "page: 2 deve ser aceite: {r:?}");
}

#[test]
fn p835_image_page_zero_erro_vanilla() {
    // Vanilla: `error: number must be positive` (exit 1).
    let world = world_com_png();
    let mut ctx = EvalContext::new();
    let args = pn(vec![Value::Str("ok.png".into())], "page", Value::Int(0));
    let err = native_image(&mut ctx, &args, &world, test_file_id()).unwrap_err();
    assert_eq!(err[0].message, "number must be positive");
}

#[test]
fn p835_image_page_tipo_errado_erro_vanilla() {
    // Vanilla: `error: expected integer, found string` (exit 1).
    let world = world_com_png();
    let mut ctx = EvalContext::new();
    let args = pn(vec![Value::Str("ok.png".into())], "page", Value::Str("2".into()));
    let err = native_image(&mut ctx, &args, &world, test_file_id()).unwrap_err();
    assert_eq!(err[0].message, "expected integer, found string");
}

#[test]
fn native_image_pdf_gera_erro_nao_suportado() {
    let mut world = NullWorld::default();
    world
        .files
        .insert("test.pdf".to_string(), std::sync::Arc::new(b"%PDF-1.7\n%%EOF".to_vec()));
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("test.pdf".into())]);
    let err = native_image(&mut ctx, &args, &world, test_file_id()).unwrap_err();
    assert_eq!(err[0].message, "PDF images are not supported yet");
}

#[test]
fn native_image_jpeg_valido_sem_regressao() {
    let mut world = NullWorld::default();
    world.files.insert(
        "foto.jpg".to_string(),
        std::sync::Arc::new(vec![0xFF, 0xD8, 0xFF, 0xE0]),
    );
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("foto.jpg".into())]);
    let result = native_image(&mut ctx, &args, &world, test_file_id()).unwrap();
    assert!(matches!(result, Value::Content(Content::Image(_))));
}

#[test]
fn native_image_rejeita_named_arg_invalido() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Str("foto.png".into())], "cor", Value::Str("red".into()));
    assert!(native_image(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}
