//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md
//! @prompt-hash d4b8fef0
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
fn svg_input_file_admits_original_source_and_resolved_origin() {
    let mut world = NullWorld::default();
    world.files.insert(
        "test.svg".to_string(),
        std::sync::Arc::new(b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_vec()),
    );
    let mut ctx = EvalContext::new();
    let args = p(vec![Value::Str("test.svg".into())]);
    let value = native_image(&mut ctx, &args, &world, test_file_id()).unwrap();
    let Value::Content(Content::Image(image)) = value else { panic!("image expected") };
    assert_eq!(image.source, Some(Value::Str("test.svg".into())));
    assert_eq!(image.path.as_deref(), Some("/test.svg"));
    assert_eq!(image.format, Some(crate::entities::image_format::ImageFormat::Svg));
    assert!(matches!(
        image.svg.as_ref().map(|s| &s.origin),
        Some(crate::entities::svg_image::SvgOrigin::File(_))
    ));
    assert_eq!(image.data.0.as_slice(), world.files["test.svg"].as_slice());
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

#[test]
fn issue_8890_figure_accepts_each_placement_alternative() {
    use crate::entities::content::Content;
    use crate::entities::layout_types::{Align2D, VAlign};
    for placement in [
        Value::Auto,
        Value::None,
        Value::Align(Align2D { h: None, v: Some(VAlign::Top) }),
        Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }),
    ] {
        null_ctx!(ctx);
        let args = pn(
            vec![Value::Content(Content::text("BODY"))],
            "placement",
            placement.clone(),
        );
        let result = native_figure(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_ok(), "placement={placement:?}: {result:?}");
    }
}

#[test]
fn issue_8890_figure_placement_retains_lexical_presence_and_cast_errors() {
    use crate::entities::{
        content::Content,
        elements::{figure::FigurePlacement, Element},
        layout_types::{Align2D, HAlign, VAlign},
    };
    for (value, expected) in [
        (Value::Auto, FigurePlacement::Auto),
        (Value::None, FigurePlacement::InFlow),
        (Value::Align(Align2D { h: None, v: Some(VAlign::Top) }), FigurePlacement::Top),
        (
            Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }),
            FigurePlacement::Bottom,
        ),
    ] {
        null_ctx!(ctx);
        let args =
            pn(vec![Value::Content(Content::text("X"))], "placement", value.clone());
        let Value::Content(Content::Figure(e)) =
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
        else {
            panic!("Figure")
        };
        assert_eq!(e.placement, Some(expected));
        assert_eq!(e.resolved_placement, None);
        assert_eq!(e.get_field("placement"), Some(value));
    }
    for (value, message) in [
        (
            Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }),
            "vertical floating placement must be `auto`, `top`, or `bottom`",
        ),
        (
            Value::Align(Align2D { h: Some(HAlign::Left), v: None }),
            "expected `top`, `horizon`, or `bottom`, found left",
        ),
        (Value::Int(1), "expected alignment, auto, or none, found integer"),
        (Value::Str("top".into()), "expected alignment, auto, or none, found string"),
    ] {
        null_ctx!(ctx);
        let args = pn(vec![Value::Content(Content::text("X"))], "placement", value);
        let errors =
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
        assert_eq!(errors[0].message, message);
    }
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("X"))]);
    let Value::Content(Content::Figure(e)) =
        native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("Figure")
    };
    assert_eq!(e.placement, None);
    assert_eq!(e.get_field("placement"), None);
}

#[cfg(test)]
mod png_bytes_source_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::bytes::Bytes as ValueBytes;

    struct NoImageIo(NullWorld);
    impl World for NoImageIo {
        fn library(&self) -> &crate::entities::world_types::Library {
            self.0.library()
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            self.0.book()
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            test_file_id()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            panic!("Image bytes must not read source")
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            panic!("Image bytes must not read a file")
        }
        fn resolve_path(
            &self,
            _: crate::entities::file_id::FileId,
            _: &str,
        ) -> Result<crate::entities::path::RootedPath, String> {
            panic!("Image bytes must not resolve a path")
        }
        fn read_path(
            &self,
            _: &crate::entities::path::RootedPath,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            panic!("Image bytes must not load a path")
        }
        fn read_bytes(
            &self,
            _: crate::entities::file_id::FileId,
            _: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            panic!("Image bytes must not use a legacy path")
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
    #[test]
    fn svg_input_svg_bytes_preserve_source_without_io() {
        assert_raster_bytes_source(&[
            60, 115, 118, 103, 32, 120, 109, 108, 110, 115, 61, 34, 104, 116, 116, 112,
            58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 48,
            48, 47, 115, 118, 103, 34, 32, 119, 105, 100, 116, 104, 61, 34, 49, 48, 46,
            53, 34, 32, 104, 101, 105, 103, 104, 116, 61, 34, 53, 46, 50, 53, 34, 62, 60,
            114, 101, 99, 116, 32, 119, 105, 100, 116, 104, 61, 34, 49, 48, 46, 53, 34,
            32, 104, 101, 105, 103, 104, 116, 61, 34, 53, 46, 50, 53, 34, 32, 102, 105,
            108, 108, 61, 34, 114, 101, 100, 34, 47, 62, 60, 47, 115, 118, 103, 62,
        ]);
    }
    #[test]
    fn svg_input_svgz_bytes_preserve_source_without_io() {
        assert_raster_bytes_source(&[
            31, 139, 8, 0, 0, 0, 0, 0, 2, 3, 125, 204, 219, 13, 128, 32, 12, 0, 192, 85,
            72, 7, 104, 81, 195, 143, 1, 150, 17, 4, 18, 124, 4, 26, 235, 248, 234, 2,
            254, 95, 206, 246, 43, 169, 123, 171, 123, 119, 144, 153, 207, 153, 72, 68,
            80, 38, 60, 90, 162, 81, 107, 77, 175, 0, 37, 37, 112, 118, 48, 104, 52, 160,
            114, 44, 41, 179, 3, 131, 163, 1, 111, 91, 92, 248, 7, 168, 181, 212, 234,
            160, 197, 0, 228, 237, 215, 249, 7, 250, 154, 167, 24, 118, 0, 0, 0,
        ]);
    }
    const HEADER: &[u8] = b"\x89PNG\r\n\x1a\n";

    #[test]
    fn png_bytes_constructor_preserves_source_without_image_io() {
        let world = NoImageIo(NullWorld::default());
        let source = Value::Bytes(ValueBytes::new(HEADER.to_vec()));
        let value = native_image(
            &mut EvalContext::new(),
            &p(vec![source.clone()]),
            &world,
            test_file_id(),
        )
        .expect("PNG bytes admitted without IO");
        let Value::Content(Content::Image(image)) = value else {
            panic!("expected image content")
        };
        assert_eq!(image.path, None, "Image bytes must not invent a file origin");
        assert_eq!(image.source.as_ref(), Some(&source));
        assert_eq!(image.data.0.as_slice(), HEADER);
        let content = Content::Image(image);
        assert_eq!(content.get_field("source"), Some(source.clone()));
        let cloned = content.map_text(&mut |text: &str| text.to_string());
        assert_eq!(cloned.get_field("source"), Some(source));
    }

    #[test]
    fn png_bytes_file_source_keeps_lexical_value() {
        let mut world = NullWorld::default();
        world
            .files
            .insert("foto.png".to_string(), std::sync::Arc::new(HEADER.to_vec()));
        let rooted = world.resolve_path(test_file_id(), "foto.png").unwrap();
        for source in [Value::Str("./foto.png".into()), Value::Path(rooted)] {
            let result = native_image(
                &mut EvalContext::new(),
                &p(vec![source.clone()]),
                &world,
                test_file_id(),
            )
            .unwrap();
            let Value::Content(content) = result else { panic!("expected image") };
            assert_eq!(content.get_field("source"), Some(source));
        }
    }

    #[test]
    fn png_bytes_empty_and_unknown_fail_without_image_io() {
        let world = NoImageIo(NullWorld::default());
        for data in [vec![], b"unknown".to_vec()] {
            let errors = native_image(
                &mut EvalContext::new(),
                &p(vec![Value::Bytes(ValueBytes::new(data))]),
                &world,
                test_file_id(),
            )
            .unwrap_err();
            assert_eq!(errors[0].message, "unknown image format");
        }
    }

    fn assert_raster_bytes_source(data: &[u8]) {
        let world = NoImageIo(NullWorld::default());
        let source = Value::Bytes(ValueBytes::new(data.to_vec()));
        let result = native_image(
            &mut EvalContext::new(),
            &p(vec![source.clone()]),
            &world,
            test_file_id(),
        )
        .expect("recognized raster bytes admitted");
        let Value::Content(Content::Image(image)) = result else {
            panic!("expected image")
        };
        assert_eq!(image.path, None);
        assert_eq!(image.source.as_ref(), Some(&source));
        assert_eq!(image.data.0.as_slice(), data);
        assert_eq!(Content::Image(image).get_field("source"), Some(source));
    }
    #[test]
    fn raster_bytes_jpeg_admits_source_without_image_io() {
        assert_raster_bytes_source(&[0xff, 0xd8, 0xff]);
    }
    #[test]
    fn raster_bytes_gif_admits_source_without_image_io() {
        assert_raster_bytes_source(b"GIF89a");
    }
    #[test]
    fn raster_bytes_webp_admits_source_without_image_io() {
        assert_raster_bytes_source(b"RIFF0000WEBP");
    }
}

#[test]
fn png_bytes_invalid_source_diagnostic_includes_bytes() {
    let errors = native_image(
        &mut EvalContext::new(),
        &p(vec![Value::Int(1)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap_err();
    assert_eq!(errors[0].message, "expected path, string, or bytes, found integer");
}
