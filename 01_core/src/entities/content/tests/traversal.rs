//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/tests/traversal.md
//! @prompt-hash be4a6985
//! @layer L1
//! @updated 2026-09-26

use super::super::*;
use crate::entities::style::{Style, Styles};

#[test]
fn map_text_transforma_texto_simples() {
    let content = Content::text("hello");
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::text("HELLO"));
}

#[test]
fn map_text_desce_em_strong() {
    // F-5b fatia 1 (P371): `Content::strong(..)` produz `Content::Strong` (variante própria).
    let content = Content::strong(Content::text("hello"));
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::strong(Content::text("HELLO")));
}

#[test]
fn map_text_preserva_terminais_sem_texto() {
    let content = Content::Space;
    let result = content.map_text(&mut |s| s.to_uppercase());
    assert_eq!(result, Content::Space);
}

#[test]
fn map_text_closure_com_estado_entre_nos() {
    // Validar que o estado da closure (FnMut) persiste entre nós distintos.
    let content = Content::Sequence(
        vec![Content::text("a"), Content::strong(Content::text("a")), Content::text("a")]
            .into(),
    );
    let mut count = 0usize;
    content.map_text(&mut |s| {
        count += 1;
        s.to_string()
    });
    assert_eq!(count, 3, "A closure deve ser chamada uma vez por nó Text");
}

// ── map_content (Passo 69 — DEBT-19) ─────────────────────────────────────

#[test]
fn map_content_substitui_heading_em_sequence() {
    let content = Content::Sequence(Arc::from(vec![
        Content::text("Antes"),
        Content::heading(1, Content::text("Titulo")),
        Content::text("Depois"),
    ]));

    let result = content
        .map_content(&mut |node| {
            if matches!(node, Content::Heading(_)) {
                Ok(Some(Content::text("SUBSTITUIDO")))
            } else {
                Ok(None)
            }
        })
        .unwrap();

    assert_eq!(result.plain_text(), "AntesSUBSTITUIDODepois");
}

#[test]
fn map_content_bottom_up_pai_ve_filhos_transformados() {
    // F-5b fatia 1 (P371): `Content::strong` é variante própria `Content::Strong`
    // (o colapso P101 → `Styled[Bold]` foi superado). O bottom-up: o pai
    // (`Strong`) vê o filho já transformado.
    let content = Content::Sequence(Arc::from([
        Content::strong(Content::text("original")),
        Content::Space,
        Content::emph(Content::text("sibling")),
        Content::text("last"),
    ]));
    let mut strong_visits = 0;

    let result = content
        .map_content(&mut |node| match node {
            Content::Text(s) => Ok(Some(Content::text(s.to_uppercase()))),
            Content::Strong(e) => {
                strong_visits += 1;
                let text = e.body.plain_text();
                assert_eq!(
                    text, "ORIGINAL",
                    "Strong deve receber filho já transformado: {:?}",
                    text
                );
                Ok(None)
            }
            // No replacement retains already-transformed children and the node kind.
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => Ok(None),
        })
        .unwrap();

    assert_eq!(strong_visits, 1);
    assert_eq!(result.plain_text(), "ORIGINAL SIBLINGLAST");
    let Content::Sequence(items) = &result else { panic!("sequence must remain") };
    assert_eq!(items.len(), 4);
    let Content::Strong(strong) = &items[0] else { panic!("strong must remain") };
    assert_eq!(strong.body.plain_text(), "ORIGINAL");
    assert!(matches!(items[1], Content::Space));
    let Content::Emph(emph) = &items[2] else { panic!("emphasis must remain") };
    assert_eq!(emph.body.plain_text(), "SIBLING");
    assert_eq!(items[3].plain_text(), "LAST");
}

#[test]
fn map_content_nao_reavaliar_no_substituido() {
    let content = Content::heading(1, Content::text("X"));
    let mut call_count = 0usize;

    content
        .map_content(&mut |node| {
            if matches!(node, Content::Heading(_)) {
                call_count += 1;
                Ok(Some(Content::text("substituido")))
            } else {
                Ok(None)
            }
        })
        .unwrap();

    assert_eq!(call_count, 1, "Heading deve ser processado exactamente uma vez");
}

// ── Passo 99 (ADR-0038): Content::Styled ─────────────────────────────

#[test]
fn styled_preserva_estilos_em_map_text() {
    let inner = Content::text("abc");
    let styles = Styles::from_iter([Style::italic(true)]);
    let styled = Content::styled(Box::new(inner), styles.clone());
    let transformed = styled.map_text(&mut |s: &str| s.to_uppercase());
    match transformed {
        Content::Styled(styled) => {
            let body = &styled.body;
            let st = &styled.styles;
            assert_eq!(body.plain_text(), "ABC");
            assert_eq!(st, &styles);
        }
        other => panic!("esperado Content::Styled, obteve {:?}", other),
    }
}

// ── Passo 154B (ADR-0060 Fase 1) — terms + divider ────────────────────

#[test]
fn terms_map_text_recurse() {
    let t = Content::terms(vec![Content::term_item(
        Content::text("apple"),
        Content::text("fruit"),
    )]);
    let upper = t.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "APPLE: FRUIT");
}

#[test]
fn quote_map_text_recurse_em_body_e_attribution() {
    let q =
        Content::quote(Content::text("hello"), Some(Content::text("seneca")), true, true);
    let upper = q.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "\"HELLO\" — SENECA");
}

#[test]
fn decoration_map_text_recurse_no_body() {
    let u = Content::underline(Content::text("hello"), None, Some(Length::pt(3.0)), None);
    let upper = u.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    // Cosméticos preservados após map_text.
    if let Content::Underline(e) = upper {
        assert_eq!(e.offset, Some(Length::pt(3.0)));
    } else {
        panic!("map_text quebrou o variant kind");
    }
}

// ── Passo 287 — Content::SmartQuote leaf ────────────────────────────

#[test]
fn pad_e_hide_map_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let pad = Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(1.0))));
    let hide = Content::hide(Content::text("hello"));
    let pad_upper = pad.map_text(&mut |s| s.to_uppercase());
    let hide_upper = hide.map_text(&mut |s| s.to_uppercase());
    // Pad expõe via plain_text (recurse); Hide oculta plain_text mas
    // o body interno foi transformado — verificamos isso desembrulhando.
    assert_eq!(pad_upper.plain_text(), "HELLO");
    if let Content::Hide(e) = &hide_upper {
        assert_eq!(e.body.plain_text(), "HELLO");
    } else {
        panic!("esperado Content::Hide após map_text");
    }
}

// ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ─────────

#[test]
fn hspace_e_vspace_map_text_preserva() {
    use crate::entities::layout_types::Length;
    // Spacing primitives são leaves — map_text não muda nada.
    let h = Content::h_space(Length::pt(7.0), false);
    let v = Content::v_space(Length::pt(7.0), true);
    let h_mapped = h.map_text(&mut |s| s.to_uppercase());
    let v_mapped = v.map_text(&mut |s| s.to_uppercase());
    assert_eq!(h, h_mapped);
    assert_eq!(v, v_mapped);
}

// ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak ─────────────

#[test]
fn pagebreak_map_text_preserva() {
    use crate::entities::parity::Parity;
    let p = Content::pagebreak(true, Some(Parity::Even));
    // Pagebreak é leaf — map_text não tem effect.
    let p_mapped = p.map_text(&mut |s| s.to_uppercase());
    assert_eq!(p, p_mapped);
}

// ── Passo 156G (ADR-0061 Fase 2, sub-passo 1) — block container ───────

#[test]
fn block_map_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::block(
        Content::text("hello"),
        None,
        None,
        Sides::uniform(Length::pt(1.0)),
        true,
    );
    let upper = b.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    // Atributos preservados após map_text.
    if let Content::Block(e) = upper {
        assert_eq!(e.inset.left, Length::pt(1.0));
    } else {
        panic!("esperado Content::Block após map_text");
    }
}

// ── Passo 156H (ADR-0061 Fase 2, sub-passo 2) — box inline container ──

#[test]
fn boxed_map_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::boxed(
        Content::text("hello"),
        None,
        None,
        Sides::uniform(Length::pt(1.0)),
        Length::pt(2.0),
    );
    let upper = b.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    // Atributos preservados.
    if let Content::Boxed(e) = upper {
        assert_eq!(e.baseline, Length::pt(2.0));
        assert_eq!(e.inset.left, Length::pt(1.0));
    } else {
        panic!("esperado Content::Boxed após map_text");
    }
}

// ── Passo 156I (ADR-0061 Fase 2, sub-passo 3) — stack compositivo ──────

#[test]
fn stack_map_text_recurse_em_cada_child() {
    use crate::entities::dir::Dir;
    let s = Content::stack(
        vec![Content::text("hello"), Content::text("world")],
        Dir::TTB,
        None,
    );
    let upper = s.map_text(&mut |t| t.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLOWORLD");
    // Atributos preservados.
    if let Content::Stack(e) = upper {
        assert_eq!(e.dir, Dir::TTB);
    } else {
        panic!("esperado Content::Stack após map_text");
    }
}

// ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — Repeat ────────────────

#[test]
fn repeat_map_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    let r = Content::repeat(Content::text("hello"), Some(Length::pt(2.0)), false);
    let upper = r.map_text(&mut |t| t.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    // Atributos preservados.
    if let Content::Repeat(e) = upper {
        assert_eq!(e.gap, Some(Length::pt(2.0)));
        assert!(!e.justify);
    } else {
        panic!("esperado Content::Repeat após map_text");
    }
}

// ── P217 (DEBT-56 sub-fase b primeiro sub-passo) — Columns ──────

#[test]
fn p217_columns_map_content_recurse() {
    // map_content recurse no body preservando count/gutter.
    use crate::entities::layout_types::Length;
    let c = Content::columns(Content::text("a"), 4, Some(Length::pt(5.0)));
    let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Columns(e) = &mapped {
        assert_eq!(e.count, 4);
        assert_eq!(e.gutter, Some(Length::pt(5.0)));
        assert_eq!(e.body.plain_text(), "a");
    } else {
        panic!("esperado Content::Columns após map_content");
    }
}

#[test]
fn p220_colbreak_map_content_terminal() {
    // Colbreak é leaf — map_content clone directo.
    let c = Content::colbreak(true);
    let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    assert_eq!(c, mapped);
}

// ── Passo 223 (ADR-0061 Fase 4 Layout candidata sub-passo 2) — Place refino ──

#[test]
fn p223_place_map_content_preserva_atributos() {
    // map_content recurse no body preservando float + clearance.
    use crate::entities::layout_types::{Align2D, HAlign, Length, PlaceScope, VAlign};
    let p = Content::place(
        Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
        0.0,
        0.0,
        PlaceScope::Parent,
        true,
        Some(Length::pt(8.0)),
        Content::text("X"),
    );
    let mapped = p.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Place(e) = &mapped {
        assert!(e.float, "map_content preserva float");
        assert_eq!(e.clearance, Some(Length::pt(8.0)), "map_content preserva clearance");
    } else {
        panic!("esperado Content::Place após map_content");
    }
}

// ── Passo 224 (ADR-0061 Fase 4 candidata sub-3) — Grid refino + 3 variants ──

#[test]
fn p224_gridcell_map_content_preserva_fields() {
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("x"),
            x: Some(0),
            y: Some(1),
            colspan: Some(2),
            rowspan: None,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        },
    ));
    let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::GridCell(e) = &mapped {
        assert_eq!(e.x, Some(0));
        assert_eq!(e.y, Some(1));
        assert_eq!(e.colspan, Some(2));
        assert!(e.rowspan.is_none());
    } else {
        panic!("esperado GridCell após map_content");
    }
}

// ── Passo 227 (ADR-0079 PROPOSTO Fase 5 Layout Categoria A.1) — stroke ──

#[test]
fn p228_grid_map_content_preserva_fill() {
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let fill_orig = Color::rgb(50, 50, 50);
    let g =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            source_order: None,
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![Content::text("a")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: None,
            fill: Some(fill_orig.clone()),
        }));
    let mapped = g.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Grid(e) = &mapped {
        assert_eq!(e.fill, Some(fill_orig));
    } else {
        panic!("esperado Content::Grid após map_content");
    }
}

#[test]
fn p227_grid_map_content_preserva_stroke() {
    // map_content preserva stroke (Option<Stroke> Clone).
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::{Color, Length, TrackSizing};
    use crate::entities::sides::Sides;
    let stroke_orig = Stroke {
        paint: Paint::Solid(Color::rgb(100, 100, 100)),
        thickness: 3.0,
        overhang: false,
        ..Stroke::default()
    };
    let g =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            source_order: None,
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: vec![Content::text("a")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: Some(stroke_orig.clone()),
            fill: None,
        }));
    let mapped = g.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Grid(e) = &mapped {
        assert_eq!(e.stroke, Some(stroke_orig));
    } else {
        panic!("esperado Content::Grid após map_content");
    }
}

// ── Passo 230 (Fase 5 Layout Categoria A.3) — stroke/fill per-cell ──

#[test]
fn p230_gridcell_map_content_preserva_stroke_fill() {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    let stroke_orig = Stroke {
        paint: Paint::Solid(Color::rgb(50, 50, 50)),
        thickness: 2.0,
        overhang: false,
        ..Stroke::default()
    };
    let fill_orig = Color::rgb(200, 200, 200);
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("c"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: Some(stroke_orig.clone()),
            fill: Some(fill_orig.clone()),
            align: None,
            inset: None,
            breakable: None,
        },
    ));
    let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::GridCell(e) = &mapped {
        assert_eq!(e.stroke, Some(stroke_orig));
        assert_eq!(e.fill, Some(fill_orig));
    } else {
        panic!("esperado GridCell após map_content");
    }
}

// ── Passo 235 (Fase 5 Layout Categoria B.3) — GridCell/TableCell algorítmicos ──

#[test]
fn p235_gridcell_map_content_preserva_3_fields() {
    use crate::entities::layout_types::Align2D;
    use crate::entities::sides::Sides;
    let c = Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body: Content::text("x"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: Some(Align2D::from_string("top")),
            inset: Some(Sides::uniform(Length::pt(2.0))),
            breakable: Some(true),
        },
    ));
    let mapped = c.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::GridCell(e) = &mapped {
        assert!(e.align.is_some());
        assert!(e.inset.is_some());
        assert_eq!(e.breakable, Some(true));
    } else {
        panic!("esperado GridCell após map_content");
    }
}

// ── Passo 231 (Fase 5 Layout Categoria A.4) — Block/Boxed outset/radius/clip ──

#[test]
fn p231_block_map_content_preserva_3_fields() {
    // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
    use crate::entities::corners::Corners;
    use crate::entities::sides::Sides;
    let outset_orig = Sides::uniform(Length::pt(7.0));
    let radius_orig = Corners::uniform(Length::pt(2.0));
    let b = Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body: Content::text("b"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: outset_orig,
            radius: radius_orig,
            clip: true,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        },
    ));
    let mapped = b.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Block(e) = &mapped {
        assert_eq!(e.outset, outset_orig);
        assert_eq!(e.radius, radius_orig);
        assert!(e.clip);
    } else {
        panic!("esperado Block após map_content");
    }
}

// ── Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4) ──────────────────
//     Block + Boxed +2 fields (fill, stroke); promoção real cumulativa
//     scope-outs ADR-0054 graded N=3 P247 (outset semantic real +
//     fill + stroke).

#[test]
fn p247_block_map_content_preserva_fill_stroke() {
    use crate::entities::corners::Corners;
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::sides::Sides;
    let fill_orig = Some(Color::rgb(50, 100, 150));
    let stroke_orig = Some(Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: 3.0,
        overhang: false,
        ..Stroke::default()
    });
    let b = Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body: Content::text("p247map"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: fill_orig.clone(),
            stroke: stroke_orig.clone(),
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        },
    ));
    let mapped = b.map_content(&mut |x| Ok(Some(x.clone()))).unwrap();
    if let Content::Block(e) = &mapped {
        assert_eq!(e.fill, fill_orig);
        assert_eq!(e.stroke, stroke_orig);
    } else {
        panic!("esperado Block após map_content");
    }
}

#[test]
fn table_map_text_recurse_em_children() {
    use crate::entities::layout_types::TrackSizing;
    let t = Content::table(
        vec![TrackSizing::Auto],
        vec![TrackSizing::Auto],
        vec![Content::text("hello"), Content::text("world")],
    );
    let upper = t.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO WORLD");
}

// ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — TableCell ─────────────

#[test]
fn table_cell_map_text_recurse_no_body_preserva_fields() {
    let c =
        Content::table_cell(Content::text("hello"), Some(2), Some(3), Some(4), Some(5));
    let upper = c.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    // Atributos preservados após map_text.
    if let Content::TableCell(e) = upper {
        assert_eq!(e.x, Some(2));
        assert_eq!(e.y, Some(3));
        assert_eq!(e.colspan, Some(4));
        assert_eq!(e.rowspan, Some(5));
    } else {
        panic!("esperado Content::TableCell após map_text");
    }
}

// ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
// Par simétrico TableHeader/TableFooter — tests imediatamente
// adjacentes para tornar paridade visualmente óbvia.

#[test]
fn table_header_map_text_recurse_e_preserva_repeat() {
    let h = Content::table_header(Content::text("hello"), false);
    let upper = h.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    if let Content::TableHeader(e) = upper {
        assert!(!e.repeat, "repeat preservado após map_text");
    } else {
        panic!("esperado Content::TableHeader");
    }
}

#[test]
fn table_footer_map_text_recurse_e_preserva_repeat() {
    let f = Content::table_footer(Content::text("hello"), false);
    let upper = f.map_text(&mut |s| s.to_uppercase());
    assert_eq!(upper.plain_text(), "HELLO");
    if let Content::TableFooter(e) = upper {
        assert!(!e.repeat);
    } else {
        panic!("esperado Content::TableFooter");
    }
}

// ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──

#[test]
fn figure_v16_preorder_map_preserves_prepared_supplement_origin_and_presence() {
    use crate::entities::elements::figure::{FigureElem, FigureKind, FigureSupplement};
    let element = FigureElem {
        body: Content::text("body"),
        caption: Some(Content::text("caption")),
        kind: Some(None),
        resolved_kind: Some(FigureKind::Name("custom".into())),
        supplement: Some(FigureSupplement::Content(Content::text("lexical-origin"))),
        resolved_supplement: Some(Some(Content::text("resolved"))),
    };
    let content = Content::Figure(std::sync::Arc::new(element.clone()));
    let mut seen = Vec::new();
    let result = content
        .map_content_pre(&mut |content| {
            if let Content::Text(text) = content {
                seen.push(text.to_string());
                return Ok(Some(Content::text(text.to_uppercase())));
            }
            Ok(None)
        })
        .unwrap();
    let Content::Figure(mapped) = result else { panic!("missing Figure") };
    assert_eq!(seen, ["body", "caption", "resolved"]);
    assert_eq!(mapped.body.plain_text(), "BODY");
    assert_eq!(mapped.caption.as_ref().unwrap().plain_text(), "CAPTION");
    assert_eq!(mapped.kind, element.kind);
    assert_eq!(mapped.resolved_kind, element.resolved_kind);
    assert_eq!(mapped.supplement, element.supplement);
    assert_eq!(mapped.resolved_supplement, Some(Some(Content::text("RESOLVED"))));
}
