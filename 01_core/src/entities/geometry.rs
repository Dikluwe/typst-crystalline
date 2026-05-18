//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/geometry.md
//! @prompt-hash 6aaa2cd6
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
    /// Bbox calculada analíticamente via `path_bbox` (P277, DEBT-33 CLOSED):
    /// endpoints + raízes de B'(t)=0 em (0,1) para cada segmento CubicTo;
    /// LineTo/MoveTo via endpoints directos. Algoritmo O(1) por segmento.
    Path(Vec<PathItem>),
}

// ── P277 — Bézier bbox analítica (DEBT-33 fecho CLOSED) ─────────────────
//
// Algoritmo: raízes de B'(t)=0 em cada eixo produzem extremos da curva
// paramétrica cúbica de Bézier. Bbox = min/max de {endpoints, raízes}.
//
// Pureza L1: matemática f64 pura; zero deps externas. ADR-0029 preserved.
//
// Complexidade: O(1) por segmento (≤6 candidatos a comparar).

/// **P277 — DEBT-33 CLOSED**: resolve `a*t² + b*t + c = 0` e retorna as
/// raízes que pertencem ao intervalo aberto `(0, 1)`. Endpoints
/// t=0 e t=1 são tratados separadamente pelo caller (sempre extremos
/// candidatos). Curva degenerada (a=b=0) retorna vector vazio.
fn solve_quadratic_in_unit(a: f64, b: f64, c: f64) -> Vec<f64> {
    let mut roots = Vec::new();

    if a.abs() < 1e-12 {
        // Linear: b*t + c = 0 → t = -c/b.
        if b.abs() < 1e-12 {
            return roots; // degenerada
        }
        let t = -c / b;
        if t > 0.0 && t < 1.0 {
            roots.push(t);
        }
        return roots;
    }

    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return roots; // sem raízes reais
    }
    let sqrt_disc = disc.sqrt();
    let two_a = 2.0 * a;
    for t in [(-b - sqrt_disc) / two_a, (-b + sqrt_disc) / two_a] {
        if t > 0.0 && t < 1.0 {
            roots.push(t);
        }
    }
    roots
}

/// **P277**: avalia B(t) num eixo (`x` ou `y`) usando coords `f64`.
/// Devolve o componente do ponto na curva paramétrica em `t ∈ [0,1]`.
fn bezier_at_axis(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let one_minus_t = 1.0 - t;
    one_minus_t.powi(3) * p0
        + 3.0 * one_minus_t.powi(2) * t * p1
        + 3.0 * one_minus_t * t.powi(2) * p2
        + t.powi(3) * p3
}

/// **P277 — DEBT-33 CLOSED**: calcula a AABB analítica de uma curva
/// cúbica de Bézier definida por 4 pontos de controlo.
///
/// Algoritmo: candidatos = {endpoints P₀, P₃} ∪ {raízes de B'(t)=0 em (0,1)}
/// para cada eixo independentemente. AABB = (min_x, min_y, max_x, max_y)
/// sobre todos os candidatos.
///
/// `B'(t) = 3 * (a*t² + b*t + c)` onde:
/// - `a = -P₀ + 3P₁ - 3P₂ + P₃`
/// - `b = 2P₀ - 4P₁ + 2P₂`
/// - `c = -P₀ + P₁`
///
/// **Resultado**: AABB exacto (analítico) em vez de conservador (min/max
/// dos pontos de controlo). Bbox da curva real é sempre ⊆ AABB({P₀..P₃}),
/// portanto analítico produz bbox **mais apertado**, não maior.
pub fn bezier_cubic_bbox(
    p0: crate::entities::layout_types::Point,
    p1: crate::entities::layout_types::Point,
    p2: crate::entities::layout_types::Point,
    p3: crate::entities::layout_types::Point,
) -> (f64, f64, f64, f64) {
    let mut xs: Vec<f64> = vec![p0.x.0, p3.x.0];
    let mut ys: Vec<f64> = vec![p0.y.0, p3.y.0];

    // Eixo X
    {
        let a = -p0.x.0 + 3.0 * p1.x.0 - 3.0 * p2.x.0 + p3.x.0;
        let b = 2.0 * p0.x.0 - 4.0 * p1.x.0 + 2.0 * p2.x.0;
        let c = -p0.x.0 + p1.x.0;
        for t in solve_quadratic_in_unit(a, b, c) {
            xs.push(bezier_at_axis(t, p0.x.0, p1.x.0, p2.x.0, p3.x.0));
        }
    }

    // Eixo Y
    {
        let a = -p0.y.0 + 3.0 * p1.y.0 - 3.0 * p2.y.0 + p3.y.0;
        let b = 2.0 * p0.y.0 - 4.0 * p1.y.0 + 2.0 * p2.y.0;
        let c = -p0.y.0 + p1.y.0;
        for t in solve_quadratic_in_unit(a, b, c) {
            ys.push(bezier_at_axis(t, p0.y.0, p1.y.0, p2.y.0, p3.y.0));
        }
    }

    let min_x = xs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_x = xs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_y = ys.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = ys.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    (min_x, min_y, max_x, max_y)
}

/// **P277 — DEBT-33 CLOSED**: calcula a AABB analítica de um Path
/// completo, walking sobre os PathItems com estado `current_point`.
///
/// `MoveTo` / `LineTo` contribuem com endpoints directamente; `CubicTo`
/// usa `bezier_cubic_bbox` para AABB analítico exacto; `ClosePath` é
/// no-op para bbox.
///
/// **Resultado**: AABB exacto preserved para LineTo-only paths (paridade
/// bit-exact com cálculo polygon() original); apertado para Paths com
/// CubicTo (analítico em vez de min/max dos pontos de controlo).
pub fn path_bbox(items: &[PathItem]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut current = crate::entities::layout_types::Point::ZERO;

    let mut update = |x: f64, y: f64, min_x: &mut f64, min_y: &mut f64,
                      max_x: &mut f64, max_y: &mut f64| {
        if x < *min_x { *min_x = x; }
        if y < *min_y { *min_y = y; }
        if x > *max_x { *max_x = x; }
        if y > *max_y { *max_y = y; }
    };

    for item in items {
        match item {
            PathItem::MoveTo(p) | PathItem::LineTo(p) => {
                update(p.x.0, p.y.0, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
                current = *p;
            }
            PathItem::CubicTo(p1, p2, p3) => {
                let (mn_x, mn_y, mx_x, mx_y) = bezier_cubic_bbox(current, *p1, *p2, *p3);
                if mn_x < min_x { min_x = mn_x; }
                if mn_y < min_y { min_y = mn_y; }
                if mx_x > max_x { max_x = mx_x; }
                if mx_y > max_y { max_y = mx_y; }
                current = *p3;
            }
            PathItem::ClosePath => {
                // sem efeito no bbox
            }
        }
    }

    (min_x, min_y, max_x, max_y)
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

    // ── P277 — DEBT-33 CLOSED: Bézier bbox analítica ────────────────────

    use crate::entities::layout_types::{Point, Pt};

    fn pt(x: f64, y: f64) -> Point {
        Point { x: Pt(x), y: Pt(y) }
    }

    /// 1) P0=P1=P2=P3 colineares (linha recta) → bbox = endpoints.
    #[test]
    fn p277_bezier_bbox_linha_recta() {
        let (min_x, min_y, max_x, max_y) = bezier_cubic_bbox(
            pt(0.0, 0.0), pt(5.0, 5.0), pt(10.0, 10.0), pt(15.0, 15.0),
        );
        assert!((min_x - 0.0).abs() < 1e-9);
        assert!((min_y - 0.0).abs() < 1e-9);
        assert!((max_x - 15.0).abs() < 1e-9);
        assert!((max_y - 15.0).abs() < 1e-9);
    }

    /// 2) Curva monotonicamente crescente (control points entre endpoints).
    /// Endpoints sempre extremos.
    #[test]
    fn p277_bezier_bbox_endpoints_unicos_extremos() {
        let (min_x, min_y, max_x, max_y) = bezier_cubic_bbox(
            pt(0.0, 0.0), pt(2.0, 2.0), pt(8.0, 8.0), pt(10.0, 10.0),
        );
        assert!((min_x - 0.0).abs() < 1e-9);
        assert!((min_y - 0.0).abs() < 1e-9);
        assert!((max_x - 10.0).abs() < 1e-9);
        assert!((max_y - 10.0).abs() < 1e-9);
    }

    /// 3) Curva U-shape em Y: P0=(0,0), P1=(0,10), P2=(10,10), P3=(10,0).
    /// Control points têm y_max=10, mas curva real tem y_max em t=0.5 que é
    /// mais baixo (curva é puxada para os control points mas não os atinge).
    /// Bbox analítica deve ter max_y < 10 (mais apertado).
    #[test]
    fn p277_bezier_bbox_curva_tighter_em_y() {
        let (_min_x, _min_y, _max_x, max_y) = bezier_cubic_bbox(
            pt(0.0, 0.0), pt(0.0, 10.0), pt(10.0, 10.0), pt(10.0, 0.0),
        );
        // Cálculo: B'_y(t) = 3*(a*t² + b*t + c) onde
        // a = -0 + 3*10 - 3*10 + 0 = 0
        // b = 2*0 - 4*10 + 2*10 = -20
        // c = -0 + 10 = 10
        // Linear: t = -10/-20 = 0.5
        // B_y(0.5) = (0.5)³*0 + 3*(0.5)²*0.5*10 + 3*0.5*(0.5)²*10 + (0.5)³*0
        //         = 0 + 3*0.25*0.5*10 + 3*0.5*0.25*10 + 0
        //         = 3.75 + 3.75 = 7.5
        assert!((max_y - 7.5).abs() < 1e-9,
            "Esperado max_y analítico = 7.5; got {}", max_y);
        // Min/max simples teria max_y = 10. Analítica < 10 confirma tighter.
        assert!(max_y < 10.0);
    }

    /// 4) Curva U-shape em X: análogo eixo x. Control points têm x extremos.
    #[test]
    fn p277_bezier_bbox_curva_tighter_em_x() {
        let (_min_x, _min_y, max_x, _max_y) = bezier_cubic_bbox(
            pt(0.0, 0.0), pt(10.0, 0.0), pt(10.0, 10.0), pt(0.0, 10.0),
        );
        // B'_x(t) análogo a §y test: max_x = 7.5 em t=0.5.
        assert!((max_x - 7.5).abs() < 1e-9);
        assert!(max_x < 10.0);
    }

    /// 5) Curva degenerada P0=P1=P2=P3 → bbox = ponto único.
    #[test]
    fn p277_bezier_bbox_curva_degenerada_a_zero() {
        let (min_x, min_y, max_x, max_y) = bezier_cubic_bbox(
            pt(5.0, 5.0), pt(5.0, 5.0), pt(5.0, 5.0), pt(5.0, 5.0),
        );
        assert!((min_x - 5.0).abs() < 1e-9);
        assert!((min_y - 5.0).abs() < 1e-9);
        assert!((max_x - 5.0).abs() < 1e-9);
        assert!((max_y - 5.0).abs() < 1e-9);
    }

    /// 6) solve_quadratic_in_unit: a=0, b≠0 → caso linear; verificar raiz
    /// em (0, 1).
    #[test]
    fn p277_solve_quadratic_in_unit_a_zero_linear() {
        // b*t + c = 0 → t = -c/b
        let roots = solve_quadratic_in_unit(0.0, -20.0, 10.0);
        // t = -10 / -20 = 0.5
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - 0.5).abs() < 1e-9);

        // Caso a=0, b=0 (degenerada) → sem raízes.
        let roots = solve_quadratic_in_unit(0.0, 0.0, 5.0);
        assert!(roots.is_empty());

        // Caso a=0, b≠0, raiz fora de (0,1) → sem raízes filtradas.
        let roots = solve_quadratic_in_unit(0.0, 1.0, -2.0);
        // t = 2; > 1 → filtrado.
        assert!(roots.is_empty());
    }

    /// 7) path_bbox com apenas MoveTo+LineTo → equivalente a min/max
    /// simples (preserva polygon() behavior bit-exact).
    #[test]
    fn p277_path_bbox_polygon_lineto_preserva() {
        let path = vec![
            PathItem::MoveTo(pt(0.0, 0.0)),
            PathItem::LineTo(pt(10.0, 0.0)),
            PathItem::LineTo(pt(10.0, 5.0)),
            PathItem::LineTo(pt(0.0, 5.0)),
            PathItem::ClosePath,
        ];
        let (min_x, min_y, max_x, max_y) = path_bbox(&path);
        assert!((min_x - 0.0).abs() < 1e-9);
        assert!((min_y - 0.0).abs() < 1e-9);
        assert!((max_x - 10.0).abs() < 1e-9);
        assert!((max_y - 5.0).abs() < 1e-9);
    }

    /// 8) path_bbox com CubicTo: bbox usa analítica para o segmento curvo,
    /// produz resultado tighter do que min/max de todos os control points.
    #[test]
    fn p277_path_bbox_cubic_usa_analitica() {
        // Path: MoveTo(0,0) → CubicTo(0,10; 10,10; 10,0)
        // Curva U-shape; analítica max_y = 7.5 (per p277_bezier_bbox_curva_tighter_em_y).
        let path = vec![
            PathItem::MoveTo(pt(0.0, 0.0)),
            PathItem::CubicTo(pt(0.0, 10.0), pt(10.0, 10.0), pt(10.0, 0.0)),
        ];
        let (min_x, min_y, max_x, max_y) = path_bbox(&path);
        assert!((min_x - 0.0).abs() < 1e-9);
        assert!((min_y - 0.0).abs() < 1e-9);
        assert!((max_x - 10.0).abs() < 1e-9);
        // max_y é 7.5 analítico (não 10 que seria min/max dos control points).
        assert!((max_y - 7.5).abs() < 1e-9,
            "path_bbox deve usar bezier_cubic_bbox analítico para CubicTo; \
             max_y esperado 7.5 (analítico), got {}", max_y);
    }
}
