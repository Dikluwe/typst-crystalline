//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/tests/projections.md
//! @prompt-hash 11007a2f
//! @layer L1
//! @updated 2026-09-26

use super::super::*;
use crate::entities::style::{Style, Styles};

#[test]
fn text_plain_text() {
    assert_eq!(Content::text("hello").plain_text(), "hello");
    assert_eq!(Content::text("").plain_text(), "");
}

#[test]
fn empty_is_empty() {
    assert!(Content::empty().is_empty());
    assert_eq!(Content::empty().plain_text(), "");
}

#[test]
fn space_nao_e_empty() {
    assert!(!Content::Space.is_empty());
    assert_eq!(Content::Space.plain_text(), " ");
}

#[test]
fn parbreak_e_marker_estrutural() {
    assert!(!Content::Parbreak.is_empty());
    assert_eq!(Content::Parbreak.plain_text(), "\n");
    assert_eq!(Content::Parbreak, Content::Parbreak);
    assert_ne!(Content::Parbreak, Content::Space);
}

#[test]
fn strong_plain_text_preservado() {
    assert_eq!(Content::strong(Content::text("bold")).plain_text(), "bold");
}

#[test]
fn emph_plain_text_preservado() {
    assert_eq!(Content::emph(Content::text("em")).plain_text(), "em");
}

#[test]
fn heading_plain_text() {
    let h = Content::heading(1, Content::text("Title"));
    assert_eq!(h.plain_text(), "Title");
}

#[test]
fn raw_plain_text() {
    assert_eq!(Content::raw("fn main() {}", None, false).plain_text(), "fn main() {}");
}

#[test]
fn list_item_tem_bullet_em_plain_text() {
    assert!(Content::list_item(Content::text("Apple"))
        .plain_text()
        .contains("Apple"));
}

#[test]
fn enum_item_com_numero() {
    let t = Content::enum_item(Some(1), Content::text("First")).plain_text();
    assert!(t.contains("1") && t.contains("First"));
}

#[test]
fn link_plain_text_e_o_corpo() {
    assert_eq!(
        Content::link("https://typst.app", Content::text("Typst")).plain_text(),
        "Typst",
    );
}

// ── Passo 34 — variantes matemáticas ─────────────────────────────────────

#[test]
fn content_equation_inline_plain_text() {
    let eq = Content::equation(Content::MathIdent("x".into()), false);
    assert_eq!(eq.plain_text(), "x");
}

#[test]
fn content_equation_block_plain_text() {
    let eq = Content::equation(Content::MathIdent("x".into()), true);
    assert_eq!(eq.plain_text(), "\nx\n");
}

#[test]
fn content_math_frac_plain_text() {
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    assert_eq!(frac.plain_text(), "(a)/(b)");
}

#[test]
fn content_math_attach_plain_text() {
    let attach = Content::math_attach_scripts(
        Content::MathIdent("x".into()),
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Present(Content::MathText("2".into())),
    );
    assert_eq!(attach.plain_text(), "x^2");
}

#[test]
fn content_math_root_quadrada() {
    let root = Content::math_root(None, Content::MathIdent("x".into()), false);
    assert_eq!(root.plain_text(), "sqrt(x)");
}

#[test]
fn content_math_root_cubica() {
    let root = Content::math_root(
        Some(Content::MathText("3".into())),
        Content::MathIdent("x".into()),
        true,
    );
    assert_eq!(root.plain_text(), "root(3, x)");
}

#[test]
fn styled_plain_text_transparente() {
    let inner = Content::text("hello");
    let styles = Styles::from_iter([
        Style::bold(true),
        Style::Size(crate::entities::layout_types::Length::pt(18.0)),
    ]);
    let styled = Content::styled(Box::new(inner), styles);
    assert_eq!(styled.plain_text(), "hello");
}

#[test]
fn divider_constructor_devolve_variant_correcto() {
    let c = Content::divider();
    assert!(matches!(c, Content::Divider(_)));
    // Divider é singleton estrutural: nunca empty.
    assert!(!c.is_empty());
}

#[test]
fn divider_plain_text_devolve_vazio() {
    assert_eq!(Content::divider().plain_text(), "");
}

#[test]
fn terms_constructor_devolve_variant_correcto() {
    let t =
        Content::terms(vec![Content::term_item(Content::text("a"), Content::text("b"))]);
    assert!(matches!(t, Content::Terms(_)));
    assert!(!t.is_empty());
    // Terms vazio é considerado empty.
    assert!(Content::terms(vec![]).is_empty());
}

#[test]
fn terms_plain_text_concatena_pares() {
    let t = Content::terms(vec![
        Content::term_item(Content::text("Apple"), Content::text("fruit")),
        Content::term_item(Content::text("Banana"), Content::text("yellow")),
    ]);
    assert_eq!(t.plain_text(), "Apple: fruit\nBanana: yellow");
}

#[test]
fn term_item_plain_text() {
    let t = Content::term_item(Content::text("key"), Content::text("value"));
    assert_eq!(t.plain_text(), "key: value");
}

#[test]
fn quote_constructor_devolve_variant_correcto() {
    let q = Content::quote(Content::text("hello"), None, false, true);
    assert!(matches!(q, Content::Quote(_)));
    assert!(!q.is_empty());
}

#[test]
fn quote_plain_text_sem_attribution() {
    let q = Content::quote(Content::text("hello"), None, false, true);
    assert_eq!(q.plain_text(), "\"hello\"");
}

#[test]
fn quote_plain_text_com_attribution() {
    let q = Content::quote(
        Content::text("Errare humanum est"),
        Some(Content::text("Seneca")),
        true,
        true,
    );
    assert_eq!(q.plain_text(), "\"Errare humanum est\" — Seneca");
}

#[test]
fn quote_plain_text_quotes_false_omite_aspas() {
    let q = Content::quote(Content::text("texto"), None, false, false);
    assert_eq!(q.plain_text(), "texto");
}

#[test]
fn quote_is_empty_proxy_para_body() {
    let empty = Content::quote(Content::Empty, None, false, true);
    assert!(empty.is_empty());
    let nonempty = Content::quote(Content::text("x"), None, false, true);
    assert!(!nonempty.is_empty());
}

#[test]
fn decoration_plain_text_delega_no_body() {
    let mk = |body| Content::underline(body, None, None, None);
    assert_eq!(mk(Content::text("hello")).plain_text(), "hello");
    // strike/overline têm a mesma regra — paridade Quote/Link.
    let s = Content::strike(Content::text("x"), None, None, None);
    let o = Content::overline(Content::text("y"), None, None, None);
    assert_eq!(s.plain_text(), "x");
    assert_eq!(o.plain_text(), "y");
}

#[test]
fn decoration_is_empty_proxy_para_body() {
    let u_empty = Content::underline(Content::Empty, None, None, None);
    let u_full = Content::underline(Content::text("a"), None, None, None);
    assert!(u_empty.is_empty());
    assert!(!u_full.is_empty());
}

#[test]
fn smartquote_plain_text_ascii_paridade_vanilla() {
    // Paridade `PlainText for Packed<SmartQuoteElem>` — emite ASCII
    // fallback (Layouter resolve lang-aware via consumer; plain_text
    // é vista sem contexto).
    assert_eq!(Content::smartquote(true).plain_text(), "\"");
    assert_eq!(Content::smartquote(false).plain_text(), "'");
}

#[test]
fn smartquote_is_empty_nunca_vazio() {
    // SmartQuote sempre emite 1 glyph — nunca empty.
    assert!(!Content::smartquote(true).is_empty());
    assert!(!Content::smartquote(false).is_empty());
}

#[test]
fn pad_constructor_envolve_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    // P156L: cada side é Option<Length>; Some(...) ↔ lado declarado.
    let p = Content::pad(Content::text("x"), Sides::uniform(Some(Length::pt(10.0))));
    if let Content::Pad(e) = &p {
        assert_eq!(e.body.plain_text(), "x");
        assert_eq!(e.sides.left, Some(Length::pt(10.0)));
        assert_eq!(e.sides.right, Some(Length::pt(10.0)));
        assert_eq!(e.sides.top, Some(Length::pt(10.0)));
        assert_eq!(e.sides.bottom, Some(Length::pt(10.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn hide_constructor_envolve_body() {
    let h = Content::hide(Content::text("placeholder"));
    if let Content::Hide(e) = &h {
        assert_eq!(e.body.plain_text(), "placeholder");
    } else {
        panic!("esperado Content::Hide");
    }
}

#[test]
fn pad_e_hide_is_empty_proxy_para_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    // Pad/Hide com body Empty são considerados vazios.
    let pad_empty = Content::pad(Content::Empty, Sides::uniform(Some(Length::pt(5.0))));
    let hide_empty = Content::hide(Content::Empty);
    assert!(pad_empty.is_empty());
    assert!(hide_empty.is_empty());
    // Com body com texto, não vazios.
    let pad_text = Content::pad(Content::text("a"), Sides::uniform(None));
    let hide_text = Content::hide(Content::text("a"));
    assert!(!pad_text.is_empty());
    assert!(!hide_text.is_empty());
}

#[test]
fn pad_plain_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let p = Content::pad(Content::text("hello"), Sides::uniform(Some(Length::pt(2.0))));
    assert_eq!(p.plain_text(), "hello");
}

#[test]
fn hide_plain_text_e_string_vazia() {
    // Hide é layout-aware mas não rende — plain_text vazio.
    let h = Content::hide(Content::text("invisivel"));
    assert_eq!(h.plain_text(), "");
}

#[test]
fn hspace_e_vspace_is_empty_se_amount_zero() {
    use crate::entities::layout_types::Length;
    // Amount zero → considerado vazio (consistente com Sequence vazia).
    let h_zero = Content::h_space(Length::ZERO, false);
    let v_zero = Content::v_space(Length::ZERO, false);
    assert!(h_zero.is_empty());
    assert!(v_zero.is_empty());
    // Amount não-zero → não vazio.
    let h_nonzero = Content::h_space(Length::pt(1.0), false);
    let v_nonzero = Content::v_space(Length::pt(1.0), false);
    assert!(!h_nonzero.is_empty());
    assert!(!v_nonzero.is_empty());
}

#[test]
fn hspace_e_vspace_plain_text_vazio() {
    use crate::entities::layout_types::Length;
    let h = Content::h_space(Length::pt(5.0), false);
    let v = Content::v_space(Length::pt(5.0), false);
    // Spacing primitives não rendem texto.
    assert_eq!(h.plain_text(), "");
    assert_eq!(v.plain_text(), "");
}

#[test]
fn pagebreak_is_empty_returns_false() {
    // Pagebreak é event observável mesmo "vazio" — análogo a Divider.
    let p = Content::pagebreak(false, None);
    assert!(
        !p.is_empty(),
        "Content::Pagebreak nunca é considerado vazio (event com efeito)"
    );
}

#[test]
fn pagebreak_plain_text_vazio() {
    let p = Content::pagebreak(false, None);
    assert_eq!(p.plain_text(), "");
}

#[test]
fn block_constructor_default_field_values() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::block(
        Content::text("body"),
        None,
        None,
        Sides::uniform(Length::ZERO),
        true,
    );
    if let Content::Block(e) = &b {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.width, None);
        assert_eq!(e.height, None);
        assert_eq!(e.inset.left, Length::ZERO);
        assert!(e.breakable);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn block_is_empty_proxy_para_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    // Block com body Empty é vazio (atributos não-nulos não mudam isso).
    let b_empty = Content::block(
        Content::Empty,
        Some(Length::pt(100.0)),
        None,
        Sides::uniform(Length::pt(5.0)),
        true,
    );
    assert!(b_empty.is_empty());
    // Com texto, não vazio.
    let b_text = Content::block(
        Content::text("a"),
        None,
        None,
        Sides::uniform(Length::ZERO),
        true,
    );
    assert!(!b_text.is_empty());
}

#[test]
fn styled_is_empty_proxy_para_body_b2() {
    // Lote F-4 S1 (P338) — fecha **B2**: um nó estilizado delega `is_empty`
    // ao body (o estilo não cria observable). Antes caía em `_ => false`
    // (styled-de-vazio reportava não-vazio).
    let styled_vazio = Content::strong(Content::Empty);
    assert!(styled_vazio.is_empty(), "strong(Empty) deve ser vazio (B2)");
    let styled_emph_vazio = Content::emph(Content::Empty);
    assert!(styled_emph_vazio.is_empty(), "emph(Empty) deve ser vazio (B2)");
    // Com conteúdo, não vazio.
    let styled_texto = Content::strong(Content::text("a"));
    assert!(!styled_texto.is_empty(), "strong(texto) não é vazio");
}

#[test]
fn block_plain_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::block(
        Content::text("hello"),
        None,
        None,
        Sides::uniform(Length::pt(2.0)),
        true,
    );
    assert_eq!(b.plain_text(), "hello");
}

#[test]
fn boxed_constructor_default() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::boxed(
        Content::text("body"),
        None,
        None,
        Sides::uniform(Length::ZERO),
        Length::ZERO,
    );
    if let Content::Boxed(e) = &b {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.width, None);
        assert_eq!(e.height, None);
        assert_eq!(e.inset.left, Length::ZERO);
        assert_eq!(e.baseline, Length::ZERO);
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn boxed_is_empty_proxy_para_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let empty = Content::boxed(
        Content::Empty,
        Some(Length::pt(50.0)),
        None,
        Sides::uniform(Length::pt(5.0)),
        Length::ZERO,
    );
    assert!(empty.is_empty());
    let nonempty = Content::boxed(
        Content::text("a"),
        None,
        None,
        Sides::uniform(Length::ZERO),
        Length::ZERO,
    );
    assert!(!nonempty.is_empty());
}

#[test]
fn boxed_plain_text_recurse_no_body() {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    let b = Content::boxed(
        Content::text("hello"),
        None,
        None,
        Sides::uniform(Length::pt(1.0)),
        Length::ZERO,
    );
    assert_eq!(b.plain_text(), "hello");
}

#[test]
fn stack_is_empty_se_todos_children_vazios() {
    use crate::entities::dir::Dir;
    // Children todos Empty → stack é vazio.
    let s_empty = Content::stack(vec![Content::Empty, Content::Empty], Dir::TTB, None);
    assert!(s_empty.is_empty());
    // Stack sem children → também vazio.
    let s_zero = Content::stack(vec![], Dir::TTB, None);
    assert!(s_zero.is_empty());
    // Algum child com texto → não vazio.
    let s_nonempty =
        Content::stack(vec![Content::Empty, Content::text("a")], Dir::TTB, None);
    assert!(!s_nonempty.is_empty());
}

#[test]
fn stack_plain_text_concatena_children() {
    use crate::entities::dir::Dir;
    let s = Content::stack(
        vec![Content::text("Hello "), Content::text("world")],
        Dir::TTB,
        None,
    );
    // Plain text concatena (consistente com Sequence).
    assert_eq!(s.plain_text(), "Hello world");
}

#[test]
fn repeat_constructor_default_gap_justify() {
    let r = Content::repeat(Content::text("."), None, true);
    if let Content::Repeat(e) = &r {
        assert_eq!(e.body.plain_text(), ".");
        assert_eq!(e.gap, None);
        assert!(e.justify);
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn repeat_is_empty_proxy_via_body() {
    // Body Empty → repeat é vazio (atributos não tornam não-vazio).
    let r_empty = Content::repeat(Content::Empty, None, true);
    assert!(r_empty.is_empty());
    // Body com texto → não vazio.
    let r_dot = Content::repeat(Content::text("."), None, true);
    assert!(!r_dot.is_empty());
}

#[test]
fn repeat_plain_text_recurse_no_body() {
    // Plain text recurse sem multiplicar — paridade não visível em
    // texto plano (semântica de repetição é runtime-only).
    let r = Content::repeat(Content::text("xy"), None, true);
    assert_eq!(r.plain_text(), "xy");
}

#[test]
fn p217_columns_variant_existe() {
    use crate::entities::layout_types::Length;
    let c = Content::columns(Content::text("hello"), 2, Some(Length::pt(10.0)));
    if let Content::Columns(e) = &c {
        assert_eq!(e.count, 2);
        assert_eq!(e.gutter, Some(Length::pt(10.0)));
        assert_eq!(e.body.plain_text(), "hello");
    } else {
        panic!("esperado Content::Columns");
    }
}

#[test]
fn p217_columns_plain_text_recurse() {
    // Stub transparente — plain_text recurse no body.
    let c = Content::columns(Content::text("multi"), 3, None);
    assert_eq!(c.plain_text(), "multi");
}

#[test]
fn p217_columns_is_empty_proxy() {
    // Body Empty → columns vazio (atributos não tornam não-vazio).
    let c_empty = Content::columns(Content::Empty, 2, None);
    assert!(c_empty.is_empty());
    // Body com texto → não vazio.
    let c_text = Content::columns(Content::text("x"), 2, None);
    assert!(!c_text.is_empty());
}

#[test]
fn p220_colbreak_is_empty_sempre_false() {
    // Colbreak é event observável (downgrade graded a pagebreak)
    // mesmo "vazio" — paridade Pagebreak/Divider.
    assert!(
        !Content::colbreak(false).is_empty(),
        "Content::Colbreak nunca é considerado vazio (event com efeito)"
    );
    assert!(!Content::colbreak(true).is_empty());
}

#[test]
fn p220_colbreak_plain_text_vazio() {
    // Colbreak é leaf — sem texto plain.
    assert_eq!(Content::colbreak(false).plain_text(), "");
    assert_eq!(Content::colbreak(true).plain_text(), "");
}

#[test]
fn p224_gridheader_variant_aceita() {
    let h = Content::grid_header(Content::text("hdr"), true);
    if let Content::GridHeader(e) = &h {
        assert_eq!(e.body.plain_text(), "hdr");
        assert!(e.repeat);
    } else {
        panic!("esperado GridHeader");
    }
}

#[test]
fn p224_gridfooter_variant_aceita() {
    let f = Content::grid_footer(Content::text("ftr"), false);
    if let Content::GridFooter(e) = &f {
        assert_eq!(e.body.plain_text(), "ftr");
        assert!(!e.repeat);
    } else {
        panic!("esperado GridFooter");
    }
}

#[test]
fn p224_gridheader_is_empty_proxy_body() {
    assert!(Content::grid_header(Content::Empty, true).is_empty());
    assert!(!Content::grid_header(Content::text("x"), true).is_empty());
}

#[test]
fn table_constructor_default() {
    let t = Content::table(vec![], vec![], vec![]);
    if let Content::Table(e) = &t {
        assert!(e.columns.is_empty());
        assert!(e.rows.is_empty());
        assert!(e.children.is_empty());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn table_is_empty_proxy_via_children() {
    use crate::entities::layout_types::TrackSizing;
    // Children vazios → table vazio (mesmo com tracks declaradas).
    let t_empty =
        Content::table(vec![TrackSizing::Auto], vec![TrackSizing::Auto], vec![]);
    assert!(t_empty.is_empty());
    // Children com texto → não vazio.
    let t_full = Content::table(
        vec![TrackSizing::Auto],
        vec![TrackSizing::Auto],
        vec![Content::text("x")],
    );
    assert!(!t_full.is_empty());
}

#[test]
fn table_plain_text_concatena_children_com_space() {
    use crate::entities::layout_types::TrackSizing;
    let t = Content::table(
        vec![TrackSizing::Auto, TrackSizing::Auto],
        vec![],
        vec![Content::text("hello"), Content::text("world")],
    );
    // Paridade Grid: join(" ").
    assert_eq!(t.plain_text(), "hello world");
}

#[test]
fn table_cell_constructor_default_todos_none() {
    // P157B: defaults — todos os fields x/y/colspan/rowspan None.
    let c = Content::table_cell(Content::text("body"), None, None, None, None);
    if let Content::TableCell(e) = &c {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.x, None);
        assert_eq!(e.y, None);
        assert_eq!(e.colspan, None);
        assert_eq!(e.rowspan, None);
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn table_cell_is_empty_proxy_via_body() {
    // Body Empty → cell vazio (atributos não tornam não-vazio).
    let c_empty = Content::table_cell(Content::Empty, Some(2), Some(3), None, None);
    assert!(c_empty.is_empty());
    // Body com texto → não vazio.
    let c_full = Content::table_cell(Content::text("a"), None, None, None, None);
    assert!(!c_full.is_empty());
}

#[test]
fn table_cell_plain_text_recurse_no_body() {
    // Plain text recurse sem multiplicar por colspan/rowspan
    // (paridade não visível em texto plano; spans são runtime
    // diferidos em DEBT-34e).
    let c = Content::table_cell(Content::text("xy"), None, None, Some(3), Some(2));
    assert_eq!(c.plain_text(), "xy");
}

#[test]
fn table_header_constructor_default_repeat_true() {
    // P157C ADR-0064 Caso D: default vanilla `repeat=true`.
    let h = Content::table_header(Content::text("body"), true);
    if let Content::TableHeader(e) = &h {
        assert_eq!(e.body.plain_text(), "body");
        assert!(e.repeat, "default vanilla repeat=true (Caso D)");
    } else {
        panic!("esperado Content::TableHeader");
    }
}

#[test]
fn table_footer_constructor_default_repeat_true() {
    // Par simétrico — paridade absoluta com TableHeader.
    let f = Content::table_footer(Content::text("body"), true);
    if let Content::TableFooter(e) = &f {
        assert_eq!(e.body.plain_text(), "body");
        assert!(e.repeat);
    } else {
        panic!("esperado Content::TableFooter");
    }
}

#[test]
fn table_header_is_empty_proxy_via_body() {
    let h_empty = Content::table_header(Content::Empty, true);
    let h_full = Content::table_header(Content::text("a"), true);
    assert!(h_empty.is_empty());
    assert!(!h_full.is_empty());
}

#[test]
fn table_footer_is_empty_proxy_via_body() {
    let f_empty = Content::table_footer(Content::Empty, true);
    let f_full = Content::table_footer(Content::text("a"), true);
    assert!(f_empty.is_empty());
    assert!(!f_full.is_empty());
}

#[test]
fn table_header_plain_text_recurse_no_body() {
    let h = Content::table_header(Content::text("hi"), true);
    assert_eq!(h.plain_text(), "hi");
}

#[test]
fn table_footer_plain_text_recurse_no_body() {
    let f = Content::table_footer(Content::text("hi"), true);
    assert_eq!(f.plain_text(), "hi");
}

#[test]
fn bibliography_constructor_default_vazia() {
    let b = Content::bibliography(vec![], None);
    if let Content::Bibliography(e) = &b {
        assert!(e.entries.is_empty());
        assert!(e.title.is_none());
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn bibliography_constructor_com_entries_e_title() {
    use crate::entities::bib_entry::BibEntry;
    let b = Content::bibliography(
        vec![BibEntry::new("k1", "A1", "T1", 2024)],
        Some(Content::text("Referências")),
    );
    if let Content::Bibliography(e) = &b {
        assert_eq!(e.entries.len(), 1);
        assert_eq!(e.entries[0].key, "k1");
        assert_eq!(
            e.title.as_ref().map(|t| t.plain_text()).as_deref(),
            Some("Referências")
        );
    } else {
        panic!("esperado Content::Bibliography");
    }
}

#[test]
fn bibliography_is_empty_proxy_via_entries_e_title() {
    use crate::entities::bib_entry::BibEntry;
    // Vazia (sem entries, sem title) → empty.
    let b_empty = Content::bibliography(vec![], None);
    assert!(b_empty.is_empty());
    // Só com title → não empty.
    let b_title = Content::bibliography(vec![], Some(Content::text("R")));
    assert!(!b_title.is_empty());
    // Só com entries → não empty.
    let b_entries = Content::bibliography(vec![BibEntry::new("k", "A", "T", 2024)], None);
    assert!(!b_entries.is_empty());
}

#[test]
fn bibliography_plain_text_concatena_title_e_entries() {
    use crate::entities::bib_entry::BibEntry;
    let b = Content::bibliography(
        vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
        Some(Content::text("Referências")),
    );
    let txt = b.plain_text();
    assert!(txt.contains("Referências"), "title presente no plain_text");
    assert!(txt.contains("[smith2024]"), "key formatada como [key]");
    assert!(txt.contains("Smith, J."), "author presente");
    assert!(txt.contains("On Crystal Math"), "title presente");
    assert!(txt.contains("2024"), "year presente");
}

#[test]
fn bibliography_from_path_preserva_path_e_style() {
    let b = Content::bibliography_from_path(
        "refs.bib",
        Some(Content::text("R")),
        Some("ieee".into()),
        Some("en-US".into()),
    );
    if let Content::Bibliography(e) = &b {
        assert_eq!(e.path.as_deref(), Some("refs.bib"));
        assert!(e.entries.is_empty());
        assert_eq!(e.style.as_deref(), Some("ieee"));
        assert_eq!(e.locale.as_deref(), Some("en-US"));
    } else {
        panic!("esperado Bibliography");
    }
}

#[test]
fn cite_constructor_com_supplement() {
    let c = Content::cite("smith2024", Some(Content::text("p. 42")), None);
    if let Content::Cite(e) = &c {
        assert_eq!(e.key, "smith2024");
        assert_eq!(
            e.supplement.as_ref().map(|s| s.plain_text()).as_deref(),
            Some("p. 42")
        );
        assert!(e.form.is_none());
    } else {
        panic!("esperado Content::Cite");
    }
}

#[test]
fn cite_is_empty_sempre_false() {
    // Cite nunca vazio — placeholder [key] sempre observable.
    let c1 = Content::cite("k", None, None);
    let c2 = Content::cite("k", Some(Content::text("p. 1")), None);
    assert!(!c1.is_empty());
    assert!(!c2.is_empty());
}

#[test]
fn cite_plain_text_emite_placeholder_com_key() {
    // Sem supplement.
    let c1 = Content::cite("smith2024", None, None);
    assert_eq!(c1.plain_text(), "[smith2024]");
    // Com supplement.
    let c2 = Content::cite("smith2024", Some(Content::text("p. 42")), None);
    assert_eq!(c2.plain_text(), "[smith2024]p. 42");
}

#[test]
fn p240_content_statedisplay_plain_text_vazio() {
    // P240 — StateDisplay sem texto direct (resolução pós-fixpoint).
    let c = Content::state_display("k".to_string(), None);
    assert_eq!(c.plain_text(), "");
}

// ── Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo
//     absoluto P240) — Content::CounterDisplayCallback ──

#[test]
fn p241_content_counter_display_callback_plain_text_vazio() {
    // P241 — CounterDisplayCallback sem texto direct (resolução
    // pós-fixpoint via apply_counter_displays).
    let c = Content::counter_display_callback("heading".to_string(), None);
    assert_eq!(c.plain_text(), "");
}
