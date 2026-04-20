//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/geometry.md
//! @prompt-hash e36ca1cd
//! @layer L1
//! @updated 2026-04-20

use crate::entities::layout_types::Color;

/// Contorno de uma forma: cor e espessura.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub paint:     Color,
    /// Espessura do contorno em pontos tipográficos.
    pub thickness: f64,
}

/// Tipo de forma geométrica primitiva.
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind {
    /// Rectângulo alinhado aos eixos.
    Rect,
    /// Elipse. Exportador PDF usa rectângulo placeholder (DEBT-31).
    Ellipse,
    /// Segmento de recta com deslocamento relativo à origem.
    ///
    /// `dx`/`dy`: deslocamentos no sistema de layout (Y positivo = baixo).
    /// Bounding box usa `abs()` dos deltas.
    Line { dx: f64, dy: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Color;

    #[test]
    fn stroke_clone_e_partialeq() {
        let s = Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 };
        assert_eq!(s.clone(), s);
    }

    #[test]
    fn shapekind_line_bounding_box_abs() {
        let line = ShapeKind::Line { dx: -50.0, dy: 30.0 };
        if let ShapeKind::Line { dx, dy } = line {
            assert_eq!(dx.abs(), 50.0);
            assert_eq!(dy.abs(), 30.0);
        }
    }
}
