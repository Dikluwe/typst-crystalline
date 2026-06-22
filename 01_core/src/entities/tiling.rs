//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/tiling.md
//! @prompt-hash e64f1c6c
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

use crate::entities::color::Color;
use crate::entities::elements::image::ImageElem;
use crate::entities::gradient::Gradient;
use crate::entities::layout_types::Size;

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
}

impl Tiling {
    /// Construtor com defaults: size/spacing `None`, relativo ao objeto.
    pub fn new(body: TilingBody) -> Self {
        Self {
            body,
            size: None,
            relative: TilingRelative::Itself,
            spacing: None,
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
            TilingBody::Color(c) => *c,
            TilingBody::Gradient(g) => g.first_stop_color(),
            TilingBody::Image(_) => Color::rgb(0, 0, 0),
        }
    }
}

/// Corpo de um padrão de azulejos.
#[derive(Debug, Clone, PartialEq)]
pub enum TilingBody {
    /// Imagem embebida.
    Image(ImageElem),
    /// Gradient (placeholder consumer — P396+).
    Gradient(Gradient),
    /// Cor sólida — fallback simples e único consumer inicial.
    Color(Color),
}

/// Referência espacial do padrão.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TilingRelative {
    /// Relativo ao objeto preenchido (default).
    #[default]
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
            path: "pat.png".into(),
            data: PtrEqArc(Arc::new(vec![1, 2, 3])),
            width: None,
            height: None,
        }
    }

    #[test]
    fn tiling_new_color() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert_eq!(t.body, TilingBody::Color(red()));
        assert!(t.size.is_none());
        assert!(t.spacing.is_none());
        assert_eq!(t.relative, TilingRelative::Itself);
    }

    #[test]
    fn tiling_new_image() {
        let img = mock_image();
        let t = Tiling::new(TilingBody::Image(img.clone()));
        assert_eq!(t.body, TilingBody::Image(img));
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
    fn tiling_relative_default_is_self() {
        let t = Tiling::new(TilingBody::Color(red()));
        assert!(matches!(t.relative, TilingRelative::Itself));
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
        let t = Tiling::new(TilingBody::Image(mock_image()));
        assert_eq!(t.to_color(), Color::rgb(0, 0, 0));
    }
}
