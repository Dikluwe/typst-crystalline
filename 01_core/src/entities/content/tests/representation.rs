//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/tests/representation.md
//! @prompt-hash ea78cc13
//! @layer L1
//! @updated 2026-10-07

use super::super::*;
use crate::entities::style::{Style, Styles};

#[test]
fn p1505_stage_b_set_page_does_not_determine_content_size() {
    let size = std::mem::size_of::<Content>();
    assert!(
        size <= 336,
        "stage B requires Content <= 336 bytes after SetPage indirection; got {size}"
    );
}

#[test]
fn p1505_stage_c_styles_do_not_determine_content_size() {
    let size = std::mem::size_of::<Content>();
    assert!(
        size <= 120,
        "stage C requires Content <= 120 bytes after Styled indirection; got {size}"
    );
}

#[test]
fn clone_e_partial_eq() {
    let c1 = Content::text("hello");
    let c2 = c1.clone();
    assert_eq!(c1, c2);
    assert_ne!(Content::text("a"), Content::text("b"));
    assert_ne!(Content::text("a"), Content::Space);
}

#[test]
fn f5b_strong_distinto_de_set_text_bold() {
    // F-5b fatia 1 (P371): `strong` é variante própria, **distinta** do
    // `Styled[Bold]` que `#set text(bold)` produz → `*bold* ≠ #set text(bold)`
    // (fidelidade ADR-0107; o vanilla trata StrongElem ≠ StyledElem). Antes do
    // retorno ao modelo de variantes (colapso P101), ambos eram `Styled[Bold]`
    // e davam `==` (divergência).
    use crate::entities::style::{Style, Styles};
    let strong = Content::strong(Content::text("x"));
    let set_text_bold = Content::styled(
        Box::new(Content::text("x")),
        Styles::from_iter([Style::bold(true)]),
    );
    assert_ne!(
        strong, set_text_bold,
        "strong ≠ #set text(bold) — distinção de tipo (0107)"
    );
    assert!(matches!(strong, Content::Strong(_)), "strong é variante própria");
    // emph idem, e strong ≠ emph (tipos distintos).
    assert_ne!(
        Content::strong(Content::text("x")),
        Content::emph(Content::text("x")),
        "strong ≠ emph (tipos distintos, como StrongElem ≠ EmphElem no vanilla)"
    );
    // Auto-igualdade preservada (o α depende disto).
    assert_eq!(Content::strong(Content::text("x")), Content::strong(Content::text("x")));
}

#[test]
fn heading_level_clamped() {
    assert!(
        matches!(Content::heading(0, Content::Empty), Content::Heading(h) if h.level == 1)
    );
    assert!(
        matches!(Content::heading(9, Content::Empty), Content::Heading(h) if h.level == 6)
    );
    assert!(
        matches!(Content::heading(3, Content::Empty), Content::Heading(h) if h.level == 3)
    );
}

#[test]
fn content_math_partialeq() {
    let a = Content::MathIdent("x".into());
    let b = Content::MathIdent("x".into());
    let c = Content::MathIdent("y".into());
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ── Passo 26 — Content::Sequence com Arc (ADR-0026 revisão) ─────────────

#[test]
fn styled_partial_eq() {
    let s1 = Content::styled(
        Box::new(Content::text("x")),
        Styles::from_iter([Style::bold(true)]),
    );
    let s2 = Content::styled(
        Box::new(Content::text("x")),
        Styles::from_iter([Style::bold(true)]),
    );
    let s3 = Content::styled(
        Box::new(Content::text("x")),
        Styles::from_iter([Style::bold(false)]),
    );
    assert_eq!(s1, s2);
    assert_ne!(s1, s3);
}

#[test]
fn terms_partial_eq() {
    let mk = || {
        Content::terms(vec![Content::term_item(Content::text("k"), Content::text("v"))])
    };
    assert_eq!(mk(), mk());
    assert_ne!(mk(), Content::divider());
    assert_eq!(Content::divider(), Content::divider());
}

// ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ─────────────────

#[test]
fn quote_partial_eq() {
    let mk = || Content::quote(Content::text("x"), None, false, true);
    assert_eq!(mk(), mk());
    let other = Content::quote(Content::text("x"), None, true /* diferente */, true);
    assert_ne!(mk(), other);
}

// ── Passo 284 (ADR-0054 graded) — text decoration ─────────────────────

#[test]
fn decoration_variants_construtores_basicos() {
    let u = Content::underline(Content::text("hi"), None, None, None);
    let s = Content::strike(Content::text("hi"), None, None, None);
    let o = Content::overline(Content::text("hi"), None, None, None);
    assert!(matches!(u, Content::Underline(_)));
    assert!(matches!(s, Content::Strike(_)));
    assert!(matches!(o, Content::Overline(_)));
    // Variants distintos não colapsam mesmo com body idêntico.
    assert_ne!(u, s);
    assert_ne!(s, o);
    assert_ne!(u, o);
}

#[test]
fn decoration_partial_eq_distingue_cosmeticos() {
    let base = || Content::underline(Content::text("x"), None, None, None);
    assert_eq!(base(), base());
    let with_offset =
        Content::underline(Content::text("x"), None, Some(Length::pt(2.0)), None);
    assert_ne!(base(), with_offset, "offset diferente quebra igualdade");
    let with_stroke =
        Content::underline(Content::text("x"), Some(Color::rgb(255, 0, 0)), None, None);
    assert_ne!(base(), with_stroke);
}

#[test]
fn smartquote_variant_construtor_basico() {
    let sd = Content::smartquote(true);
    let ss = Content::smartquote(false);
    assert!(matches!(&sd, Content::SmartQuote(e) if e.double == Some(true)));
    assert!(matches!(&ss, Content::SmartQuote(e) if e.double == Some(false)));
    // Variants distintos por valor `double`.
    assert_ne!(sd, ss);
}

#[test]
fn smartquote_partial_eq() {
    let a = Content::smartquote(true);
    let b = Content::smartquote(true);
    let c = Content::smartquote(false);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──

#[test]
fn pad_partial_eq() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let mk = || Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(3.0))));
    assert_eq!(mk(), mk());
    // Padding diferente → diferente (bottom 5pt em vez de 3pt).
    let other = Content::pad(
        Content::text("x"),
        Sides::new(
            Some(Length::pt(3.0)),
            Some(Length::pt(3.0)),
            Some(Length::pt(3.0)),
            Some(Length::pt(5.0)),
        ),
    );
    assert_ne!(mk(), other);
    // P156L: distinção semântica nova — Some(zero) ≠ None.
    let some_zero = Content::pad(Content::text("x"), Sides::uniform(Some(Length::ZERO)));
    let none = Content::pad(Content::text("x"), Sides::uniform(None));
    assert_ne!(some_zero, none, "P156L: Some(zero) e None são semanticamente distintos");
}

#[test]
fn hide_partial_eq() {
    let a = Content::hide(Content::text("x"));
    let b = Content::hide(Content::text("x"));
    let c = Content::hide(Content::text("y"));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn hspace_constructor() {
    use crate::entities::elements::h_space::Spacing;
    use crate::entities::layout_types::Length;
    let h = Content::h_space(Length::pt(12.0), false);
    if let Content::HSpace(e) = h {
        assert_eq!(e.amount, Spacing::Absolute(Length::pt(12.0)));
        assert!(!e.weak);
    } else {
        panic!("esperado Content::HSpace");
    }
    // P842 (#38) — constructor de fração.
    let hfr = Content::h_space_fraction(1.5, true);
    if let Content::HSpace(e) = hfr {
        assert_eq!(e.amount, Spacing::Fractional(1.5));
        assert!(e.weak);
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn vspace_constructor() {
    use crate::entities::layout_types::Length;
    let v = Content::v_space(Length::pt(8.0), true);
    if let Content::VSpace(e) = v {
        assert_eq!(e.amount, Length::pt(8.0));
        assert!(e.weak);
    } else {
        panic!("esperado Content::VSpace");
    }
}

#[test]
fn hspace_partial_eq() {
    use crate::entities::layout_types::Length;
    let a = Content::h_space(Length::pt(3.0), false);
    let b = Content::h_space(Length::pt(3.0), false);
    let c = Content::h_space(Length::pt(3.0), true); // weak diferente
    let d = Content::h_space(Length::pt(4.0), false); // amount diferente
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
}

#[test]
fn vspace_partial_eq() {
    use crate::entities::layout_types::Length;
    let a = Content::v_space(Length::pt(3.0), false);
    let b = Content::v_space(Length::pt(3.0), false);
    let c = Content::v_space(Length::pt(3.0), true);
    assert_eq!(a, b);
    assert_ne!(a, c);
    // VSpace e HSpace com mesmos campos NÃO são iguais (variantes
    // diferentes).
    let h = Content::h_space(Length::pt(3.0), false);
    assert_ne!(a, h);
}

#[test]
fn pagebreak_constructor() {
    use crate::entities::parity::Parity;
    let p = Content::pagebreak(false, None);
    if let Content::Pagebreak(e) = p {
        assert!(!e.weak);
        assert_eq!(e.to, None);
    } else {
        panic!("esperado Content::Pagebreak");
    }
    let p2 = Content::pagebreak(true, Some(Parity::Even));
    if let Content::Pagebreak(e) = p2 {
        assert!(e.weak);
        assert_eq!(e.to, Some(Parity::Even));
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn pagebreak_partial_eq() {
    use crate::entities::parity::Parity;
    let a = Content::pagebreak(false, None);
    let b = Content::pagebreak(false, None);
    let c = Content::pagebreak(true, None); // weak diferente
    let d = Content::pagebreak(false, Some(Parity::Even)); // to diferente
    let e = Content::pagebreak(false, Some(Parity::Odd)); // to diferente
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
    assert_ne!(d, e);
}

#[test]
fn block_with_explicit_width_height_inset() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::block(
        Content::text("x"),
        Some(Length::pt(100.0)),
        Some(Length::pt(50.0)),
        Sides::uniform(Length::pt(8.0)),
        false,
    );
    if let Content::Block(e) = &b {
        assert_eq!(
            e.width,
            Some(crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(100.0) })
        );
        assert_eq!(
            e.height,
            Some(crate::entities::elements::block::BlockHeight::Relative(
                crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(50.0) }
            ))
        );
        assert_eq!(e.inset.left, Length::pt(8.0));
        assert!(!e.breakable);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn block_partial_eq() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let mk = || {
        Content::block(
            Content::text("x"),
            Some(Length::pt(50.0)),
            None,
            Sides::uniform(Length::pt(3.0)),
            true,
        )
    };
    assert_eq!(mk(), mk());
    // Width diferente → diferente.
    let other_width = Content::block(
        Content::text("x"),
        Some(Length::pt(60.0)),
        None,
        Sides::uniform(Length::pt(3.0)),
        true,
    );
    assert_ne!(mk(), other_width);
    // breakable diferente → diferente.
    let other_breakable = Content::block(
        Content::text("x"),
        Some(Length::pt(50.0)),
        None,
        Sides::uniform(Length::pt(3.0)),
        false,
    );
    assert_ne!(mk(), other_breakable);
}

#[test]
fn boxed_constructor_explicit_atributos() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::boxed(
        Content::text("x"),
        Some(Length::pt(60.0)),
        Some(Length::pt(20.0)),
        Sides::uniform(Length::pt(2.0)),
        Length::pt(-3.0), // baseline negativo aceito
    );
    if let Content::Boxed(e) = &b {
        assert_eq!(e.width, Some(Length::pt(60.0)));
        assert_eq!(e.height, Some(Length::pt(20.0)));
        assert_eq!(e.inset.right, Length::pt(2.0));
        assert_eq!(e.baseline, Length::pt(-3.0));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn boxed_partial_eq() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let mk = || {
        Content::boxed(
            Content::text("x"),
            Some(Length::pt(40.0)),
            None,
            Sides::uniform(Length::pt(1.0)),
            Length::pt(2.0),
        )
    };
    assert_eq!(mk(), mk());
    // baseline diferente → diferente.
    let other_baseline = Content::boxed(
        Content::text("x"),
        Some(Length::pt(40.0)),
        None,
        Sides::uniform(Length::pt(1.0)),
        Length::pt(5.0),
    );
    assert_ne!(mk(), other_baseline);
}

#[test]
fn stack_constructor_default() {
    use crate::entities::dir::Dir;
    let s = Content::stack(
        vec![Content::text("a"), Content::text("b")],
        Dir::default(),
        None,
    );
    if let Content::Stack(e) = &s {
        assert_eq!(e.children.len(), 2);
        assert_eq!(e.dir, Dir::TTB);
        assert_eq!(e.spacing, None);
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn stack_constructor_explicit_dir_spacing() {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::Length;
    let s = Content::stack(
        vec![Content::text("x"), Content::text("y")],
        Dir::LTR,
        Some(Length::pt(5.0)),
    );
    if let Content::Stack(e) = &s {
        assert_eq!(e.dir, Dir::LTR);
        assert_eq!(e.spacing, Some(Length::pt(5.0)));
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn stack_partial_eq() {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::Length;
    let mk = || {
        Content::stack(
            vec![Content::text("a"), Content::text("b")],
            Dir::TTB,
            Some(Length::pt(3.0)),
        )
    };
    assert_eq!(mk(), mk());
    // Dir diferente → diferente.
    let other_dir = Content::stack(
        vec![Content::text("a"), Content::text("b")],
        Dir::LTR,
        Some(Length::pt(3.0)),
    );
    assert_ne!(mk(), other_dir);
    // Spacing diferente → diferente.
    let other_spacing =
        Content::stack(vec![Content::text("a"), Content::text("b")], Dir::TTB, None);
    assert_ne!(mk(), other_spacing);
    // Children diferentes → diferente.
    let other_children = Content::stack(
        vec![Content::text("a"), Content::text("c")],
        Dir::TTB,
        Some(Length::pt(3.0)),
    );
    assert_ne!(mk(), other_children);
}

#[test]
fn repeat_constructor_explicit_gap_justify_false() {
    use crate::entities::layout_types::Length;
    let r = Content::repeat(Content::text("a"), Some(Length::pt(2.0)), false);
    if let Content::Repeat(e) = &r {
        assert_eq!(e.gap, Some(Length::pt(2.0)));
        assert!(!e.justify);
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn repeat_partial_eq_cobre_todos_os_fields() {
    use crate::entities::layout_types::Length;
    let mk = || Content::repeat(Content::text("."), Some(Length::pt(3.0)), true);
    assert_eq!(mk(), mk());
    // Body diferente → diferente.
    let other_body = Content::repeat(Content::text("o"), Some(Length::pt(3.0)), true);
    assert_ne!(mk(), other_body);
    // Gap diferente → diferente.
    let other_gap = Content::repeat(Content::text("."), None, true);
    assert_ne!(mk(), other_gap);
    // Justify diferente → diferente.
    let other_justify = Content::repeat(Content::text("."), Some(Length::pt(3.0)), false);
    assert_ne!(mk(), other_justify);
}

#[test]
fn p217_columns_partial_eq_3_fields() {
    use crate::entities::layout_types::Length;
    let mk = || Content::columns(Content::text("."), 2, Some(Length::pt(8.0)));
    assert_eq!(mk(), mk());
    // Count diferente → diferente.
    let other_count = Content::columns(Content::text("."), 3, Some(Length::pt(8.0)));
    assert_ne!(mk(), other_count);
    // Gutter diferente → diferente.
    let other_gutter = Content::columns(Content::text("."), 2, Some(Length::pt(12.0)));
    assert_ne!(mk(), other_gutter);
    // Body diferente → diferente.
    let other_body = Content::columns(Content::text("o"), 2, Some(Length::pt(8.0)));
    assert_ne!(mk(), other_body);
}

// ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — Colbreak ──────────

#[test]
fn p220_colbreak_variant_existe() {
    let c = Content::colbreak(false);
    if let Content::Colbreak(e) = &c {
        assert!(!e.weak);
    } else {
        panic!("esperado Content::Colbreak");
    }
    let c2 = Content::colbreak(true);
    if let Content::Colbreak(e) = &c2 {
        assert!(e.weak);
    } else {
        panic!("esperado Content::Colbreak");
    }
}

#[test]
fn p220_colbreak_partial_eq_1_field() {
    // Eq compara `weak` (1 field — paridade Pagebreak sem to).
    assert_eq!(Content::colbreak(false), Content::colbreak(false));
    assert_eq!(Content::colbreak(true), Content::colbreak(true));
    assert_ne!(Content::colbreak(false), Content::colbreak(true));
}

#[test]
fn p223_place_variant_aceita_float_clearance() {
    // P223 refino: variant aceita 2 fields novos float + clearance.
    use crate::entities::layout_types::{Align2D, HAlign, Length, PlaceScope, VAlign};
    let p = Content::place(
        Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
        0.0,
        0.0,
        PlaceScope::Column,
        true,
        Some(Length::pt(10.0)),
        Content::text("body"),
    );
    if let Content::Place(e) = &p {
        assert!(e.float);
        assert_eq!(e.clearance, Some(Length::pt(10.0)));
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_place_default_float_false_clearance_none() {
    // Defaults stdlib: float=false, clearance=None.
    use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
    let p = Content::place(
        Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
        0.0,
        0.0,
        PlaceScope::Column,
        false,
        None,
        Content::text("body"),
    );
    if let Content::Place(e) = &p {
        assert!(!e.float, "default float == false");
        assert!(e.clearance.is_none(), "default clearance == None");
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_place_partial_eq_inclui_float_clearance() {
    // Eq compara 7 fields agora (P223 +2 fields).
    use crate::entities::layout_types::{Align2D, HAlign, Length, PlaceScope, VAlign};
    let mk = |float: bool, clearance: Option<Length>| {
        Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            float,
            clearance,
            Content::text("."),
        )
    };
    assert_eq!(mk(false, None), mk(false, None));
    // Float diferente → diferente.
    assert_ne!(mk(false, None), mk(true, None));
    // Clearance diferente → diferente.
    assert_ne!(mk(false, None), mk(false, Some(Length::pt(5.0))));
}

#[test]
fn p224_grid_variant_aceita_5_fields_aditivos() {
    // Grid variant aceita gutter/align/inset/header/footer.
    use crate::entities::layout_types::{Align2D, HAlign, Length, TrackSizing, VAlign};
    use crate::entities::sides::Sides;
    let g =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![Content::text("A")],
            hlines: vec![],
            vlines: vec![],
            gutter: Some(Length::pt(5.0)),
            align: Some(Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) }),
            inset: Sides::uniform(Length::pt(2.0)),
            header: Some(Content::text("H")),
            footer: Some(Content::text("F")),
            stroke: None,
            fill: None,
        }));
    if let Content::Grid(e) = &g {
        assert_eq!(e.gutter, Some(Length::pt(5.0)));
        assert!(e.header.is_some());
        assert!(e.footer.is_some());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p224_grid_partial_eq_inclui_5_fields_novos() {
    use crate::entities::layout_types::{Length, TrackSizing};
    use crate::entities::sides::Sides;
    let mk = |gutter: Option<Length>| {
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![],
            hlines: vec![],
            vlines: vec![],
            gutter,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
        }))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(Length::pt(5.0))));
}

#[test]
fn p224_gridcell_variant_aceita_5_fields() {
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("cell"),
            x: Some(1),
            y: Some(2),
            colspan: Some(3),
            rowspan: Some(4),
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        },
    ));
    if let Content::GridCell(e) = &c {
        assert_eq!(e.x, Some(1));
        assert_eq!(e.y, Some(2));
        assert_eq!(e.colspan, Some(3));
        assert_eq!(e.rowspan, Some(4));
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p224_gridcell_partial_eq_5_fields() {
    // Eq compara 5 fields (paridade P157B TableCell literal).
    let mk = |x: Option<usize>| {
        Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("."),
                x,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(1)));
}

#[test]
fn p227_grid_variant_aceita_stroke() {
    // Grid variant aceita stroke (+1 field P227; total 9 fields).
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let g =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![Content::text("A")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(255, 0, 0)),
                thickness: 2.0,
                overhang: false,
                ..Stroke::default()
            }),
            fill: None,
        }));
    if let Content::Grid(e) = &g {
        assert!(e.stroke.is_some());
        let s = e.stroke.as_ref().unwrap();
        assert_eq!(s.thickness, 2.0);
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p227_table_variant_aceita_stroke() {
    // Paridade Grid para Table (refino paralelo).
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::{Color, TrackSizing};
    let t = Content::Table(std::sync::Arc::new(
        crate::entities::elements::table::TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: false,
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            children: vec![Content::text("X")],
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 255)),
                thickness: 1.5,
                overhang: false,
                ..Stroke::default()
            }),
            fill: None,
            caption: None,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        },
    ));
    if let Content::Table(e) = &t {
        assert!(e.stroke.is_some());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p227_grid_partial_eq_inclui_stroke() {
    // Eq compara 9 fields agora (P227 +1 stroke).
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let mk = |stroke: Option<Stroke>| {
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke,
            fill: None,
        }))
    };
    assert_eq!(mk(None), mk(None));
    let s = Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: 1.0,
        overhang: false,
        ..Stroke::default()
    };
    assert_ne!(mk(None), mk(Some(s)));
}

// ── Passo 228 (Fase 5 Layout Categoria A.2) — fill ──

#[test]
fn p228_grid_variant_aceita_fill() {
    // Grid variant aceita fill (+1 field P228; total 10 fields).
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let g =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![Content::text("A")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: None,
            fill: Some(Color::rgb(255, 255, 0)),
        }));
    if let Content::Grid(e) = &g {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p228_table_variant_aceita_fill() {
    use crate::entities::layout_types::{Color, TrackSizing};
    let t = Content::Table(std::sync::Arc::new(
        crate::entities::elements::table::TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: false,
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            children: vec![Content::text("X")],
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: Some(Color::rgb(0, 255, 0)),
            caption: None,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        },
    ));
    if let Content::Table(e) = &t {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p228_grid_partial_eq_inclui_fill() {
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let mk = |fill: Option<Color>| {
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: None,
            fill,
        }))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
}

#[test]
fn p230_gridcell_variant_aceita_stroke_fill() {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("cell"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 1.0,
                overhang: false,
                ..Stroke::default()
            }),
            fill: Some(Color::rgb(255, 255, 0)),
            align: None,
            inset: None,
            breakable: None,
        },
    ));
    if let Content::GridCell(e) = &c {
        assert!(e.stroke.is_some() && e.fill.is_some());
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p230_tablecell_variant_aceita_stroke_fill() {
    use crate::entities::layout_types::Color;
    let c = Content::TableCell(std::sync::Arc::new(
        crate::entities::elements::table_cell::TableCellElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            standalone: None,
            presence: crate::entities::elements::table_cell::TableCellPresence {
                fill: crate::entities::elements::table_cell::CellFieldPresence::Explicit,
                ..Default::default()
            },
            body: Content::text("cell"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: Some(Color::rgb(0, 255, 0)),
            align: None,
            inset: None,
            breakable: None,
            kind: crate::entities::elements::table_cell::TableCellKind::Auto,
        },
    ));
    if let Content::TableCell(e) = &c {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado TableCell");
    }
}

#[test]
fn p230_gridcell_partial_eq_inclui_stroke_fill() {
    use crate::entities::layout_types::Color;
    let mk = |fill: Option<Color>| {
        Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("."),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill,
                align: None,
                inset: None,
                breakable: None,
            },
        ))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
}

#[test]
fn p235_gridcell_variant_aceita_align_inset_breakable() {
    use crate::entities::layout_types::Align2D;
    use crate::entities::sides::Sides;
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("p235"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: Some(Align2D::from_string("center")),
            inset: Some(Sides::uniform(Length::pt(7.0))),
            breakable: Some(false),
        },
    ));
    if let Content::GridCell(e) = &c {
        assert!(e.align.is_some());
        assert!(e.inset.is_some());
        assert_eq!(e.breakable, Some(false));
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p235_tablecell_variant_aceita_align_inset_breakable() {
    use crate::entities::layout_types::Align2D;
    use crate::entities::sides::Sides;
    let c = Content::TableCell(std::sync::Arc::new(
        crate::entities::elements::table_cell::TableCellElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            standalone: None,
            presence: crate::entities::elements::table_cell::TableCellPresence {
                align: crate::entities::elements::table_cell::CellFieldPresence::Explicit,
                inset: crate::entities::elements::table_cell::CellFieldPresence::Explicit,
                breakable:
                    crate::entities::elements::table_cell::CellFieldPresence::Explicit,
                ..Default::default()
            },
            body: Content::text("p235t"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: Some(Align2D::from_string("right")),
            inset: Some(Sides::uniform(Length::pt(3.0))),
            breakable: Some(true),
            kind: crate::entities::elements::table_cell::TableCellKind::Auto,
        },
    ));
    if let Content::TableCell(e) = &c {
        assert!(e.align.is_some());
        assert!(e.inset.is_some());
        assert_eq!(e.breakable, Some(true));
    } else {
        panic!("esperado TableCell");
    }
}

#[test]
fn p235_gridcell_partial_eq_inclui_3_fields() {
    let mk = |breakable: Option<bool>| {
        Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("."),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable,
            },
        ))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(false)));
    assert_ne!(mk(Some(true)), mk(Some(false)));
}

#[test]
fn p231_block_variant_aceita_outset_radius_clip() {
    // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
    use crate::entities::corners::Corners;
    use crate::entities::sides::Sides;
    let b = Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body: Content::text("body"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(5.0)),
            radius: Corners::uniform(Length::pt(3.0)),
            clip: true,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        },
    ));
    if let Content::Block(e) = &b {
        assert_eq!(e.radius.top_left, Length::pt(3.0));
        assert_eq!(e.radius.top_right, Length::pt(3.0));
        assert_eq!(e.radius.bottom_right, Length::pt(3.0));
        assert_eq!(e.radius.bottom_left, Length::pt(3.0));
        assert!(e.clip);
        assert_eq!(e.outset.left, Length::pt(5.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p231_boxed_variant_aceita_outset_radius_clip() {
    // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
    use crate::entities::corners::Corners;
    use crate::entities::sides::Sides;
    let b = Content::Boxed(std::sync::Arc::new(
        crate::entities::elements::boxed::BoxedElem {
            body: Content::text("body"),
            width: None,
            width_fraction: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset: Sides::uniform(Length::pt(2.0)),
            radius: Corners::uniform(Length::pt(4.0)),
            clip: false,
            fill: None,
            stroke: None,
        },
    ));
    if let Content::Boxed(e) = &b {
        assert_eq!(e.radius.top_left, Length::pt(4.0));
        assert!(!e.clip);
        assert_eq!(e.outset.left, Length::pt(2.0));
    } else {
        panic!("esperado Boxed");
    }
}

#[test]
fn p231_block_partial_eq_inclui_3_fields() {
    // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
    use crate::entities::corners::Corners;
    use crate::entities::sides::Sides;
    let mk = |clip: bool| {
        Content::Block(std::sync::Arc::new(crate::entities::elements::block::BlockElem {
            body: Content::text("."),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    };
    assert_eq!(mk(false), mk(false));
    assert_ne!(mk(false), mk(true));
}

#[test]
fn p247_block_variant_aceita_fill_stroke() {
    use crate::entities::corners::Corners;
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::sides::Sides;
    let b = Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body: Content::text("p247"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: Some(Color::rgb(200, 0, 0)),
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 2.0,
                overhang: false,
                ..Stroke::default()
            }),
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        },
    ));
    if let Content::Block(e) = &b {
        assert_eq!(e.fill, Some(Color::rgb(200, 0, 0)));
        assert_eq!(e.stroke.as_ref().unwrap().thickness, 2.0);
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p247_boxed_variant_aceita_fill_stroke() {
    use crate::entities::corners::Corners;
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::sides::Sides;
    let b = Content::Boxed(std::sync::Arc::new(
        crate::entities::elements::boxed::BoxedElem {
            body: Content::text("p247"),
            width: None,
            width_fraction: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: Some(Color::rgb(0, 200, 0)),
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 1.5,
                overhang: false,
                ..Stroke::default()
            }),
        },
    ));
    if let Content::Boxed(e) = &b {
        assert_eq!(e.fill, Some(Color::rgb(0, 200, 0)));
        assert_eq!(e.stroke.as_ref().unwrap().thickness, 1.5);
    } else {
        panic!("esperado Boxed");
    }
}

#[test]
fn p247_block_partial_eq_inclui_fill_stroke() {
    use crate::entities::corners::Corners;
    use crate::entities::layout_types::Color;
    use crate::entities::sides::Sides;
    let mk = |fill: Option<Color>| {
        Content::Block(std::sync::Arc::new(crate::entities::elements::block::BlockElem {
            body: Content::text("p247eq"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    };
    assert_eq!(mk(None), mk(None));
    assert_ne!(mk(None), mk(Some(Color::rgb(255, 0, 0))));
    assert_eq!(mk(Some(Color::rgb(1, 2, 3))), mk(Some(Color::rgb(1, 2, 3))));
}

#[test]
fn p247_block_construtor_defaults_fill_stroke_none() {
    use crate::entities::sides::Sides;
    let b = Content::block(
        Content::text("p247def"),
        None,
        None,
        Sides::uniform(Length::pt(0.0)),
        true,
    );
    if let Content::Block(e) = &b {
        assert_eq!(e.fill, None, "construtor default fill = None");
        assert!(e.stroke.is_none(), "construtor default stroke = None");
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p247_boxed_construtor_defaults_fill_stroke_none() {
    use crate::entities::sides::Sides;
    let b = Content::boxed(
        Content::text("p247def"),
        None,
        None,
        Sides::uniform(Length::pt(0.0)),
        Length::pt(0.0),
    );
    if let Content::Boxed(e) = &b {
        assert_eq!(e.fill, None);
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Boxed");
    }
}

// ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — Table ─────────────────

#[test]
fn table_constructor_com_tracks_e_children() {
    use crate::entities::layout_types::TrackSizing;
    let t = Content::table(
        vec![TrackSizing::Auto, TrackSizing::Auto],
        vec![TrackSizing::Auto],
        vec![Content::text("a"), Content::text("b")],
    );
    if let Content::Table(e) = &t {
        assert_eq!(e.columns.len(), 2);
        assert_eq!(e.rows.len(), 1);
        assert_eq!(e.children.len(), 2);
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn table_partial_eq() {
    use crate::entities::layout_types::TrackSizing;
    let mk = || {
        Content::table(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("a")],
        )
    };
    assert_eq!(mk(), mk());
    // Children diferentes → diferente.
    let other_children = Content::table(
        vec![TrackSizing::Auto],
        vec![TrackSizing::Auto],
        vec![Content::text("b")],
    );
    assert_ne!(mk(), other_children);
    // Columns diferentes → diferente.
    let other_columns = Content::table(
        vec![TrackSizing::Auto, TrackSizing::Auto],
        vec![TrackSizing::Auto],
        vec![Content::text("a")],
    );
    assert_ne!(mk(), other_columns);
}

#[test]
fn table_cell_constructor_com_x_y() {
    // P157B: ADR-0064 Caso A — Some(n) ↔ posição explícita.
    let c = Content::table_cell(Content::text("x"), Some(2), Some(3), None, None);
    if let Content::TableCell(e) = &c {
        assert_eq!(e.x, Some(2));
        assert_eq!(e.y, Some(3));
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn table_cell_constructor_com_colspan_rowspan() {
    // P157B: ADR-0064 Caso C — Some(n) ↔ span explícito.
    let c = Content::table_cell(Content::text("x"), None, None, Some(2), Some(3));
    if let Content::TableCell(e) = &c {
        assert_eq!(e.colspan, Some(2));
        assert_eq!(e.rowspan, Some(3));
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn table_cell_partial_eq_cobre_todos_os_5_fields() {
    let mk =
        || Content::table_cell(Content::text("a"), Some(1), Some(2), Some(3), Some(4));
    assert_eq!(mk(), mk());
    // x diferente → diferente.
    let other_x =
        Content::table_cell(Content::text("a"), Some(99), Some(2), Some(3), Some(4));
    assert_ne!(mk(), other_x);
    // y diferente → diferente.
    let other_y =
        Content::table_cell(Content::text("a"), Some(1), Some(99), Some(3), Some(4));
    assert_ne!(mk(), other_y);
    // colspan diferente → diferente.
    let other_cs =
        Content::table_cell(Content::text("a"), Some(1), Some(2), Some(99), Some(4));
    assert_ne!(mk(), other_cs);
    // rowspan diferente → diferente.
    let other_rs =
        Content::table_cell(Content::text("a"), Some(1), Some(2), Some(3), Some(99));
    assert_ne!(mk(), other_rs);
    // body diferente → diferente.
    let other_body =
        Content::table_cell(Content::text("b"), Some(1), Some(2), Some(3), Some(4));
    assert_ne!(mk(), other_body);
}

#[test]
fn table_header_repeat_false_explicito() {
    let h = Content::table_header(Content::text("x"), false);
    if let Content::TableHeader(e) = h {
        assert!(!e.repeat);
    } else {
        panic!("esperado Content::TableHeader");
    }
}

#[test]
fn table_footer_repeat_false_explicito() {
    let f = Content::table_footer(Content::text("x"), false);
    if let Content::TableFooter(e) = f {
        assert!(!e.repeat);
    } else {
        panic!("esperado Content::TableFooter");
    }
}

#[test]
fn table_header_partial_eq() {
    let mk = || Content::table_header(Content::text("a"), true);
    assert_eq!(mk(), mk());
    // body diferente → diferente.
    let other_body = Content::table_header(Content::text("b"), true);
    assert_ne!(mk(), other_body);
    // repeat diferente → diferente.
    let other_repeat = Content::table_header(Content::text("a"), false);
    assert_ne!(mk(), other_repeat);
}

#[test]
fn table_footer_partial_eq() {
    let mk = || Content::table_footer(Content::text("a"), true);
    assert_eq!(mk(), mk());
    let other_body = Content::table_footer(Content::text("b"), true);
    assert_ne!(mk(), other_body);
    let other_repeat = Content::table_footer(Content::text("a"), false);
    assert_ne!(mk(), other_repeat);
}

#[test]
fn table_header_e_footer_sao_variants_distintos() {
    // Paridade interna absoluta no contrato; mas variants
    // distintos ao nível do enum (não confundir Header com
    // Footer mesmo com mesmos fields).
    let h = Content::table_header(Content::text("a"), true);
    let f = Content::table_footer(Content::text("a"), true);
    assert_ne!(h, f, "TableHeader e TableFooter são variants distintos");
}

#[test]
fn bibliography_partial_eq_cobre_2_fields() {
    use crate::entities::bib_entry::BibEntry;
    let mk = || {
        Content::bibliography(
            vec![BibEntry::new("k", "A", "T", 2024)],
            Some(Content::text("R")),
        )
    };
    assert_eq!(mk(), mk());
    // entries diferentes → diferente.
    let other_entries = Content::bibliography(
        vec![BibEntry::new("k", "A", "T", 2025)], // year diferente
        Some(Content::text("R")),
    );
    assert_ne!(mk(), other_entries);
    // title diferente → diferente.
    let other_title = Content::bibliography(
        vec![BibEntry::new("k", "A", "T", 2024)],
        Some(Content::text("Bibliografia")),
    );
    assert_ne!(mk(), other_title);
}

#[test]
fn bibliography_with_style_preserva_style_e_locale() {
    use crate::entities::bib_entry::BibEntry;
    let b = Content::bibliography_with_style(
        vec![BibEntry::new("k", "A", "T", 2024)],
        Some(Content::text("R")),
        Some("ieee".into()),
        Some("en-US".into()),
    );
    if let Content::Bibliography(e) = &b {
        assert_eq!(e.style.as_deref(), Some("ieee"));
        assert_eq!(e.locale.as_deref(), Some("en-US"));
    } else {
        panic!("esperado Bibliography");
    }
}

#[test]
fn bibliography_with_style_distingue_por_style() {
    use crate::entities::bib_entry::BibEntry;
    let a = Content::bibliography_with_style(
        vec![BibEntry::new("k", "A", "T", 2024)],
        None,
        Some("ieee".into()),
        None,
    );
    let b = Content::bibliography_with_style(
        vec![BibEntry::new("k", "A", "T", 2024)],
        None,
        Some("apa".into()),
        None,
    );
    assert_ne!(a, b);
}

#[test]
fn cite_constructor_so_key() {
    let c = Content::cite("smith2024", None, None);
    if let Content::Cite(e) = &c {
        assert_eq!(e.key, "smith2024");
        assert!(e.supplement.is_none());
        assert!(e.form.is_none());
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn cite_partial_eq_cobre_3_fields() {
    let mk = || Content::cite("k", Some(Content::text("p. 1")), None);
    assert_eq!(mk(), mk());
    // key diferente → diferente.
    let other_key = Content::cite("k2", Some(Content::text("p. 1")), None);
    assert_ne!(mk(), other_key);
    // supplement diferente → diferente.
    let other_sup = Content::cite("k", Some(Content::text("p. 99")), None);
    assert_ne!(mk(), other_sup);
    // supplement None vs Some → diferente.
    let other_none = Content::cite("k", None, None);
    assert_ne!(mk(), other_none);
}

// ── Passo 159C: Cite.form ──────────────────────────────────────────────
use crate::entities::citation_form::CitationForm;

#[test]
fn cite_constructor_com_form() {
    let c = Content::cite("smith2024", None, Some(CitationForm::Prose));
    if let Content::Cite(e) = &c {
        assert_eq!(e.key, "smith2024");
        assert!(e.supplement.is_none());
        assert_eq!(e.form, Some(CitationForm::Prose));
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn cite_partial_eq_cobre_form() {
    let mk = || Content::cite("k", None, Some(CitationForm::Prose));
    assert_eq!(mk(), mk());
    // form diferente → diferente.
    let other_form = Content::cite("k", None, Some(CitationForm::Author));
    assert_ne!(mk(), other_form);
    // form None vs Some → diferente.
    let other_none = Content::cite("k", None, None);
    assert_ne!(mk(), other_none);
}

// ── Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ) — Content::StateDisplay ──

#[test]
fn p240_content_statedisplay_partial_eq_sem_callback() {
    // Sem callback → comparação por key.
    let a = Content::state_display("k".to_string(), None);
    let b = Content::state_display("k".to_string(), None);
    assert_eq!(a, b);
    let c = Content::state_display("other".to_string(), None);
    assert_ne!(a, c);
}

#[test]
fn p240_content_statedisplay_partial_eq_com_callback_ptr_eq() {
    // Com callback → comparação via Func::PartialEq (identidade de Arc;
    // **P742**: nativas com mesmo nome são a mesma função — medido no
    // vanilla `color.rgb == rgb` → true — logo o caso "distinto" usa
    // nome diferente).
    use crate::entities::func::Func;
    let f1 = Func::native("identity", |_, args, _, _| {
        Ok(args
            .items
            .first()
            .cloned()
            .unwrap_or(crate::entities::value::Value::None))
    });
    let a = Content::state_display("k".to_string(), Some(f1.clone()));
    let b = Content::state_display("k".to_string(), Some(f1.clone()));
    // Mesmo Arc partilhado → equal.
    assert_eq!(a, b);
    // Func distinta (nome diferente → função diferente) → not equal.
    let f2 = Func::native("identity2", |_, args, _, _| {
        Ok(args
            .items
            .first()
            .cloned()
            .unwrap_or(crate::entities::value::Value::None))
    });
    let c = Content::state_display("k".to_string(), Some(f2));
    assert_ne!(a, c);
}

#[test]
fn p241_content_counter_display_callback_partial_eq_sem_callback() {
    // Sem callback → comparação por key.
    let a = Content::counter_display_callback("heading".to_string(), None);
    let b = Content::counter_display_callback("heading".to_string(), None);
    assert_eq!(a, b);
    let c = Content::counter_display_callback("figure".to_string(), None);
    assert_ne!(a, c);
}

#[test]
fn p241_content_counter_display_callback_partial_eq_com_callback_ptr_eq() {
    // Com callback → comparação via Func::PartialEq (identidade de Arc;
    // **P742**: nativas com mesmo nome são a mesma função — medido no
    // vanilla `color.rgb == rgb` → true — logo o caso "distinto" usa
    // nome diferente).
    use crate::entities::func::Func;
    let f1 = Func::native("identity", |_, args, _, _| {
        Ok(args
            .items
            .first()
            .cloned()
            .unwrap_or(crate::entities::value::Value::None))
    });
    let a = Content::counter_display_callback("k".to_string(), Some(f1.clone()));
    let b = Content::counter_display_callback("k".to_string(), Some(f1.clone()));
    assert_eq!(a, b);
    let f2 = Func::native("identity2", |_, args, _, _| {
        Ok(args
            .items
            .first()
            .cloned()
            .unwrap_or(crate::entities::value::Value::None))
    });
    let c = Content::counter_display_callback("k".to_string(), Some(f2));
    assert_ne!(a, c);
}

#[test]
fn p241_content_counter_display_callback_distinto_de_legacy_counter_display() {
    // P241 — variant nova paralela coexiste com Content::CounterDisplay
    // legacy { kind } sem conflito (Decisão 1 Opção α).
    let new = Content::counter_display_callback("heading".to_string(), None);
    let legacy = Content::counter_display("heading".to_string());
    assert_ne!(new, legacy);
}
