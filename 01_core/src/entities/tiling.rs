//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/tiling.md
//! @prompt-hash 26c4c937
//! @layer L1
//! @updated 2026-06-22
//!
//! `Tiling` — padrão de azulejos (pattern fill).
//!
//! Passo 395: modelagem do tipo L1 puro; zero I/O. Não implementa `tiling()`
//! nem render PDF de pattern fill (scope-out P396 / ADR-0054 graded).
//!
//! `TilingBody::Gradient` é placeholder — o tipo `Gradient` existe, mas o
//! consumer real de Tiling+Gradient fica para passo futuro.

use crate::entities::axes::Axes;
use crate::entities::color::Color;
use crate::entities::content::Content;
use crate::entities::elements::image::ImageElem;
use crate::entities::gradient::Gradient;
use crate::entities::layout_types::{Angle, Length, Size};
use crate::entities::rel::Rel;
use std::sync::Arc;

/// Padrão de azulejos — fill pattern para shapes, boxes e strokes.
#[derive(Debug, Clone, PartialEq)]
pub struct Tiling {
    /// Corpo do padrão: imagem, gradiente ou cor sólida.
    pub body: TilingBody,
    /// Tamanho da célula do padrão. `None` ↔ `auto` (bounds do body).
    pub size: Option<Size>,
    /// Referência espacial do padrão.
    pub relative: TilingRelative,
    /// Gap entre repetições. `None` ↔ zero.
    pub spacing: Option<Size>,
    /// Deslocamento relativo/absoluto resolvido contra o pitch no layout.
    pub offset: Axes<Rel<Length>>,
    /// Rotação horária da grade em torno do ponto offset.
    pub angle: Angle,
}

impl Tiling {
    /// Construtor com defaults: size/spacing `None`, relativo ao objeto.
    pub fn new(body: TilingBody) -> Self {
        Self {
            body,
            size: None,
            relative: TilingRelative::Auto,
            spacing: None,
            offset: Axes::new(Rel::zero(), Rel::zero()),
            angle: Angle::rad(0.0),
        }
    }

    /// Retorna uma cor representativa para fallback de render.
    ///
    /// - `Color` → a própria cor.
    /// - `Gradient` → primeira stop do gradiente.
    /// - `Image` → preto (scope-out graded; pattern fill de imagem ainda não
    ///   renderizado).
    pub fn to_color(&self) -> Color {
        match &self.body {
            TilingBody::Color(c) => c.clone(),
            TilingBody::Gradient(g) => g.first_stop_color(),
            TilingBody::Image(_) => Color::rgb(0, 0, 0),
            TilingBody::Content(_) => Color::rgb(0, 0, 0),
        }
    }
}

/// Corpo de um padrão de azulejos.
#[derive(Debug, Clone, PartialEq)]
pub enum TilingBody {
    /// Conteúdo declarativo arbitrário; layoutado somente na fase de layout.
    Content(Arc<Content>),
    /// Imagem embebida.
    Image(Box<ImageElem>),
    /// Gradient (placeholder consumer — P396+).
    Gradient(Gradient),
    /// Cor sólida — fallback simples e único consumer inicial.
    Color(Color),
}

/// Referência espacial do padrão.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TilingRelative {
    /// Decisão contextual: self para shapes, parent para texto.
    #[default]
    Auto,
    /// Relativo ao objeto preenchido.
    Itself,
    /// Relativo ao pai / página.
    Parent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;

    fn red() -> Color {
        Color::rgb(255, 0, 0)
    }

    fn mock_image() -> ImageElem {
        ImageElem {
            format: None,
            svg: None,
            path: Some("pat.png".into()),
            source: None,
            alt: None,
            data: PtrEqArc(Arc::new(vec![1, 2, 3])),
            width: None,
            height: None,
            fit: "cover".into(),
        }
    }

    #[test]
    fn tiling_new_color() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert_eq!(t.body, TilingBody::Color(red()));
        assert!(t.size.is_none());
        assert!(t.spacing.is_none());
        assert_eq!(t.relative, TilingRelative::Auto);
    }

    #[test]
    fn tiling_new_image() {
        let img = mock_image();
        let t = Tiling::new(TilingBody::Image(Box::new(img.clone())));
        assert_eq!(t.body, TilingBody::Image(Box::new(img)));
    }

    #[test]
    fn tiling_equality() {
        let a = Tiling::new(TilingBody::Color(red()));
        let b = Tiling::new(TilingBody::Color(red()));
        let c = Tiling::new(TilingBody::Color(Color::rgb(0, 0, 0)));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn tiling_clone_preserves_equality() {
        let a = Tiling::new(TilingBody::Color(red()));
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn tiling_relative_default_is_auto() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert!(matches!(t.relative, TilingRelative::Auto));
    }

    #[test]
    fn tiling_size_none_means_auto() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert!(t.size.is_none());
    }

    #[test]
    fn tiling_to_color_color() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert_eq!(t.to_color(), red());
    }

    #[test]
    fn tiling_to_color_image_fallback_black() {
        let t = Tiling::new(TilingBody::Image(Box::new(mock_image())));
        assert_eq!(t.to_color(), Color::rgb(0, 0, 0));
    }
}

#[cfg(test)]
mod boxed_image_body_tests {
    use super::*;
    use crate::entities::{
        bytes::Bytes,
        file_id::FileId,
        image_format::ImageFormat,
        path::{RootedPath, VirtualPath, VirtualRoot},
        ptr_eq_arc::PtrEqArc,
        span::Span,
        svg_image::{SvgOrigin, SvgSource},
        value::Value,
    };
    use std::num::NonZeroU16;

    fn image(format: ImageFormat, source: Value) -> ImageElem {
        ImageElem {
            format: Some(format),
            svg: None,
            path: None,
            source: Some(source),
            alt: Some("original alternative".into()),
            data: PtrEqArc(Arc::new(vec![9, 8, 7, 6])),
            width: Some(Box::new(Value::Int(24))),
            height: Some(Box::new(Value::Int(12))),
            fit: "contain".into(),
        }
    }

    #[test]
    fn boxed_raster_construction_and_clone_retain_original_bytes_and_format() {
        for format in
            [ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::Gif, ImageFormat::WebP]
        {
            let source = Value::Bytes(Bytes::new(vec![1, 3, 5, 7]));
            let image = image(format, source.clone());
            let data = Arc::clone(&image.data.0);
            let original = Tiling::new(TilingBody::Image(Box::new(image)));
            let cloned = original.clone();
            assert_eq!(original, cloned);
            let TilingBody::Image(original_image) = &original.body else {
                panic!("image body")
            };
            let TilingBody::Image(cloned_image) = &cloned.body else {
                panic!("cloned image body")
            };
            assert!(!std::ptr::eq(original_image.as_ref(), cloned_image.as_ref()));
            for body in [original_image.as_ref(), cloned_image.as_ref()] {
                assert_eq!(body.format, Some(format));
                assert_eq!(body.source, Some(source.clone()));
                assert!(body.path.is_none());
                assert!(body.svg.is_none());
                assert_eq!(body.alt.as_deref(), Some("original alternative"));
                assert_eq!(body.width.as_deref(), Some(&Value::Int(24)));
                assert_eq!(body.height.as_deref(), Some(&Value::Int(12)));
                assert_eq!(body.fit.as_str(), "contain");
                assert!(Arc::ptr_eq(&body.data.0, &data));
                assert_eq!(body.data.0.as_slice(), &[9, 8, 7, 6]);
            }
            assert!(original.size.is_none());
            assert!(original.spacing.is_none());
            assert_eq!(original.relative, TilingRelative::Auto);
        }
    }

    #[test]
    fn boxed_svg_clone_preserves_file_and_inline_origin_without_fabricating_path() {
        let id = FileId::from_raw(NonZeroU16::new(17).unwrap());
        let path = RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("/art/pattern.svg").unwrap(),
        );
        for origin in [SvgOrigin::File(path), SvgOrigin::Inline(id)] {
            let source = Value::Str("<svg xmlns='http://www.w3.org/2000/svg'/>".into());
            let mut image = image(ImageFormat::Svg, source.clone());
            let metadata = SvgSource { origin, span: Span::from_range(id, 3..19) };
            image.svg = Some(metadata.clone());
            let original = TilingBody::Image(Box::new(image));
            let cloned = original.clone();
            assert_eq!(original, cloned);
            let TilingBody::Image(body) = cloned else { panic!("SVG image body") };
            assert_eq!(body.format, Some(ImageFormat::Svg));
            assert_eq!(body.source, Some(source));
            assert_eq!(body.svg, Some(metadata));
            assert!(body.path.is_none());
            let TilingBody::Image(original) = original else { unreachable!() };
            assert!(Arc::ptr_eq(&body.data.0, &original.data.0));
        }
    }

    #[test]
    fn boxed_clone_has_independent_owned_metadata_and_source() {
        let original = TilingBody::Image(Box::new(image(
            ImageFormat::Png,
            Value::Bytes(Bytes::new(vec![1, 2, 3])),
        )));
        let mut cloned = original.clone();
        let TilingBody::Image(changed) = &mut cloned else { panic!("image clone") };
        changed.path = Some("changed.png".into());
        changed.alt = Some("changed alternative".into());
        changed.fit = "stretch".into();
        **changed.width.as_mut().expect("owned width") = Value::Int(99);
        **changed.height.as_mut().expect("owned height") = Value::Int(88);
        let Some(Value::Bytes(source)) = changed.source.as_mut() else {
            panic!("byte source")
        };
        source.0.push(4);
        changed.format = Some(ImageFormat::Jpeg);
        let TilingBody::Image(body) = &original else { unreachable!() };
        assert!(body.path.is_none());
        assert_eq!(body.alt.as_deref(), Some("original alternative"));
        assert_eq!(body.fit.as_str(), "contain");
        assert_eq!(body.width.as_deref(), Some(&Value::Int(24)));
        assert_eq!(body.height.as_deref(), Some(&Value::Int(12)));
        assert_eq!(body.format, Some(ImageFormat::Png));
        assert_eq!(body.source, Some(Value::Bytes(Bytes::new(vec![1, 2, 3]))));
        assert!(Arc::ptr_eq(&body.data.0, &changed.data.0));
        assert_ne!(original, cloned);
    }

    #[test]
    fn boxed_svg_metadata_mutation_does_not_change_original_origin() {
        let id = FileId::from_raw(NonZeroU16::new(21).unwrap());
        let mut image = image(ImageFormat::Svg, Value::Str("original.svg".into()));
        image.path = Some("original.svg".into());
        let metadata = SvgSource {
            origin: SvgOrigin::Inline(id),
            span: Span::from_range(id, 2..8),
        };
        image.svg = Some(metadata.clone());
        let original = TilingBody::Image(Box::new(image));
        let mut cloned = original.clone();
        let TilingBody::Image(changed) = &mut cloned else { panic!("SVG clone") };
        let svg = changed.svg.as_mut().expect("owned SVG metadata");
        svg.origin = SvgOrigin::File(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("/replacement.svg").unwrap(),
        ));
        svg.span = Span::detached();
        changed.source = None;
        changed.path = None;
        let TilingBody::Image(body) = &original else { unreachable!() };
        assert_eq!(body.svg, Some(metadata));
        assert_eq!(body.source, Some(Value::Str("original.svg".into())));
        assert_eq!(body.path.as_deref(), Some("original.svg"));
        assert_eq!(body.format, Some(ImageFormat::Svg));
        assert!(Arc::ptr_eq(&body.data.0, &changed.data.0));
        assert_ne!(original, cloned);
    }
}
