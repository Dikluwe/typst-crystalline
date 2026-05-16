//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/geometry.md
//! @prompt-hash 7dbda723
//! @layer L1
//! @updated 2026-04-20

use crate::entities::layout_types::Point;
use crate::entities::paint::Paint;

/// Segmento de um caminho vectorial.
#[derive(Debug, Clone, PartialEq)]
pub enum PathItem {
    /// Mover o cursor para o ponto sem traçar.
    MoveTo(Point),
    /// Traçar um segmento de recta até ao ponto.
    LineTo(Point),
    /// Curva de Bézier cúbica: dois pontos de controlo e o ponto final.
    CubicTo(Point, Point, Point),
    /// Fechar o sub-path com uma recta de volta ao último MoveTo.
    ClosePath,
}

/// Contorno de uma forma: cor, espessura, e overhang.
///
/// **P252 (M9d / M7+5; ADR-0079 Categoria A.4 Boxed COMPLETO 6/6;
/// cita ADR-0082 PROPOSTO N=3 terceira aplicação citante)** — `overhang:
/// bool` controla se o stroke se sobrepõe ao corner da bounding box
/// (default cristalino `false` divergente da vanilla `true` —
/// divergência consciente documentada em ADR-0054 §"Promoções reais
/// cumulativas"; paridade vanilla restaurada via stdlib `extract_stroke`).
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    /// **P261** — `Paint` wrapper enum (Solid only) substitui
    /// `Color` directo per ADR-0086. Abre caminho para Gradient
    /// real consumer em P262+.
    pub paint:     Paint,
    /// Espessura do contorno em pontos tipográficos.
    pub thickness: f64,
    /// **P252** — `true` expande bounds Shape por `thickness/2` em
    /// todos os lados quando emit em Layouter (paridade vanilla
    /// overhang). `false` preserva bounds literais (default
    /// construtor Rust cristalino; backward compat literal estrita
    /// pré-P252).
    pub overhang:  bool,
}

/// Tipo de forma geométrica primitiva.
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind {
    /// Rectângulo alinhado aos eixos.
    Rect,
    /// **P242 (M9d / M7+5)** — rectângulo com cantos arredondados.
    /// `radii: Corners<Length>` define o raio de cada canto independente
    /// (paralelo vanilla `layout/shape.rs::RoundedRect`).
    ///
    /// **Degeneração**: quando todos os 4 radii são zero, semantic é
    /// equivalente a `Rect` mas a distinção estrutural é preservada
    /// (não normaliza para `Rect`). PDF exporter (P242) emite Bezier
    /// 4 corners path via `emit_shape_path_local` arm RoundedRect.
    ///
    /// Consumer principal: `Content::Block.radius` + `Content::Boxed.radius`
    /// (`Corners<Length>` per-corner via stdlib `radius:` Length
    /// uniforme OR dict por canto). Quando `clip == true` E pelo
    /// menos um canto não-zero, Layouter emite `FrameItem::Group`
    /// com `clip_mask: Some(ShapeKind::RoundedRect { radii })`.
    RoundedRect {
        radii: crate::entities::corners::Corners<crate::entities::layout_types::Length>,
    },
    /// Elipse. Exportador PDF usa rectângulo placeholder (DEBT-31).
    Ellipse,
    /// Segmento de recta com deslocamento relativo à origem.
    ///
    /// `dx`/`dy`: deslocamentos no sistema de layout (Y positivo = baixo).
    /// Bounding box usa `abs()` dos deltas.
    Line { dx: f64, dy: f64 },
    /// Caminho geométrico livre — polígonos e formas arbitrárias.
    ///
    /// A bounding box é calculada pelos pontos de controlo (DEBT-33:
    /// pode ser conservadora para segmentos CubicTo).
    Path(Vec<PathItem>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;

    #[test]
    fn stroke_clone_e_partialeq() {
        let s = Stroke { paint: Paint::Solid(Color::rgb(0, 0, 0)), thickness: 1.0, overhang: false };
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

    // ── Passo 242 (M9d / M7+5; ADR-0081 IMPLEMENTADO parcial 3/5) ──
    //     ShapeKind::RoundedRect { radii: Corners<Length> }

    #[test]
    fn p242_shapekind_rounded_rect_radii_zero_eq_rect_distinguivel() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // Decisão 2 §3 P242: degeneração estrutural preserva distinção
        // (não normaliza para Rect). Length não impl Default; usar
        // Corners::uniform(Length::ZERO) que é o mesmo valor.
        let rounded_zero = ShapeKind::RoundedRect {
            radii: Corners::uniform(Length::ZERO),
        };
        let rect = ShapeKind::Rect;
        assert_ne!(rounded_zero, rect, "RoundedRect{{0,0,0,0}} ≠ Rect estrutural");
    }

    #[test]
    fn p242_shapekind_rounded_rect_radii_uniforme_pt_5() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let r5 = Length { abs: crate::entities::layout_types::Abs(5.0), em: 0.0 };
        let rounded = ShapeKind::RoundedRect {
            radii: Corners::uniform(r5),
        };
        if let ShapeKind::RoundedRect { radii } = rounded {
            assert_eq!(radii.top_left,     r5);
            assert_eq!(radii.top_right,    r5);
            assert_eq!(radii.bottom_right, r5);
            assert_eq!(radii.bottom_left,  r5);
        } else {
            panic!("esperado ShapeKind::RoundedRect");
        }
    }
}
