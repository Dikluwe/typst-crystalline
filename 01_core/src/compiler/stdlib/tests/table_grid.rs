//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/table_grid.md
//! @prompt-hash af65acbd
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p224_native_grid_aceita_gutter() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("gutter".into(), Value::Length(Length::pt(5.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert_eq!(e.gutter, Some(Length::pt(5.0)));
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p224_native_grid_gutter_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("gutter".into(), Value::Length(Length::pt(-5.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "gutter negativo deve falhar");
}

#[test]
fn p224_native_grid_inset_uniforme_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert_eq!(e.inset.left, Length::pt(3.0));
        assert_eq!(e.inset.right, Length::pt(3.0));
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p772i_native_grid_header_footer_nomeados_rejeitados() {
    // P772i — substitui `p224_native_grid_header_footer_content_aceita`.
    // `header:`/`footer:` deixaram de ser argumentos nomeados válidos
    // (paridade vanilla — são elementos-filho `grid.header(...)`/
    // `grid.footer(...)`, nunca named args da função `grid()`). Ver
    // 00_nucleo/diagnosticos/paridade-producao-p772i.md.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named
        .insert("header".into(), Value::Content(Content::text("HDR")));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "header: nomeado deve ser rejeitado (paridade vanilla)");

    let mut args2 = p(vec![Value::Content(Content::text("body"))]);
    args2
        .named
        .insert("footer".into(), Value::Content(Content::text("FTR")));
    let r2 = native_grid(&mut ctx, &args2, &null_world(), test_file_id());
    assert!(r2.is_err(), "footer: nomeado deve ser rejeitado (paridade vanilla)");
}

#[test]
fn p772i_native_grid_header_footer_como_children_populam_grid_elem() {
    // P772i — header/footer agora vêm de `Content::GridHeader`/
    // `Content::GridFooter` como children posicionais (via
    // `native_grid_header`/`native_grid_footer`), não named args.
    null_ctx!(ctx);
    let hdr = native_grid_header(
        &mut ctx,
        &p(vec![Value::Content(Content::text("HDR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let ftr = native_grid_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("FTR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let args = p(vec![Value::Content(Content::text("body")), hdr, ftr]);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.header.is_some(), "header presente (via child)");
        assert!(e.footer.is_some(), "footer presente (via child)");
        assert!(
            matches!(e.header.as_ref().unwrap(), Content::GridHeader(_)),
            "header deve ser Content::GridHeader"
        );
        assert!(
            matches!(e.footer.as_ref().unwrap(), Content::GridFooter(_)),
            "footer deve ser Content::GridFooter"
        );
        // "body" (Content::text) continua como célula normal.
        assert_eq!(
            e.cells.len(),
            1,
            "header/footer não devem contar como células normais"
        );
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p822_native_grid_footer_fora_do_fim_rejeita() {
    null_ctx!(ctx);
    let ftr = native_grid_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("FTR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Footer seguido de célula do corpo → erro (vanilla: exit 1).
    let args = p(vec![ftr, Value::Content(Content::text("body"))]);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    let err = r.expect_err("footer fora do fim deve falhar");
    assert_eq!(
        err[0].message, "footer must end at the last row",
        "paridade verbatim vanilla resolve.rs:1889"
    );
}

#[test]
fn p822_native_table_footer_fora_do_fim_rejeita() {
    null_ctx!(ctx);
    let ftr = native_table_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("FTR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Footer seguido de Str (célula do corpo) → erro.
    let args = p(vec![ftr, Value::Str("body".into())]);
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id());
    let err = r.expect_err("footer fora do fim deve falhar");
    assert_eq!(
        err[0].message, "footer must end at the last row",
        "paridade verbatim vanilla resolve.rs:1889"
    );
}

#[test]
fn p822_native_grid_footer_no_fim_ok() {
    // Controlo: footer como último child continua a compilar.
    null_ctx!(ctx);
    let ftr = native_grid_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("FTR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let args = p(vec![Value::Content(Content::text("body")), ftr]);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "footer no fim é válido: {:?}", r.err());
}

#[test]
fn p822_native_grid_hline_depois_do_footer_ok() {
    // Medido no vanilla (sonda P822): hline depois do footer compila
    // (exit 0) — a validação só cobre células do corpo.
    null_ctx!(ctx);
    let ftr = native_grid_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("FTR"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let hline =
        native_grid_hline(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    let args = p(vec![ftr, hline]);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "hline depois do footer é válido: {:?}", r.err());
}

#[test]
fn p224_native_grid_named_arg_desconhecido_rejeita() {
    // stroke/fill cosméticos scope-out — desconhecidos rejeitados.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::Str("black".into()));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "stroke scope-out deve falhar");
}

#[test]
fn p224_native_grid_cell_body_obrigatorio() {
    null_ctx!(ctx);
    let r = native_grid_cell(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "body obrigatório");
}

#[test]
fn p224_native_grid_cell_x_y_colspan_rowspan_aceita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("cell"))]);
    args.named.insert("x".into(), Value::Int(1));
    args.named.insert("y".into(), Value::Int(0));
    args.named.insert("colspan".into(), Value::Int(2));
    args.named.insert("rowspan".into(), Value::Int(3));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert_eq!(e.x, Some(1));
        assert_eq!(e.y, Some(0));
        assert_eq!(e.colspan, Some(2));
        assert_eq!(e.rowspan, Some(3));
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p224_native_grid_cell_colspan_zero_rejeita() {
    // ADR-0064 Caso C — colspan >= 1.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("colspan".into(), Value::Int(0));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "colspan 0 deve falhar (paridade NonZeroUsize)");
}

#[test]
fn p224_native_grid_header_aceita_body() {
    null_ctx!(ctx);
    let r = native_grid_header(
        &mut ctx,
        &p(vec![Value::Content(Content::text("hdr"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::GridHeader(e)) = r {
        assert_eq!(e.body.plain_text(), "hdr");
        assert_eq!(e.repeat, true, "default repeat == true (paridade vanilla)");
    } else {
        panic!("esperado GridHeader");
    }
}

#[test]
fn p224_native_grid_footer_aceita_body_repeat_false() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("ftr"))]);
    args.named.insert("repeat".into(), Value::Bool(false));
    let r = native_grid_footer(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridFooter(e)) = r {
        assert_eq!(e.body.plain_text(), "ftr");
        assert_eq!(e.repeat, false);
    } else {
        panic!("esperado GridFooter");
    }
}

#[test]
fn p228_native_grid_fill_color_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named
        .insert("fill".into(), Value::Color(Color::rgb(255, 200, 0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p228_native_grid_fill_default_none() {
    null_ctx!(ctx);
    let r = native_grid(
        &mut ctx,
        &p(vec![Value::Content(Content::text("a"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.fill.is_none(), "default fill == None");
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p228_native_grid_fill_tipo_errado_rejeita() {
    // fill aceita só Color (não Length); rejeita explicitamente.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
}

#[test]
fn p228_native_table_fill_paridade_grid() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named
        .insert("fill".into(), Value::Color(Color::rgb(100, 100, 100)));
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p228_native_grid_fill_e_stroke_simultaneos_aceita() {
    // Ambos fill + stroke aceitos no mesmo grid() call.
    null_ctx!(ctx);
    use crate::entities::layout_types::{Color, Length};
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
    args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.fill.is_some() && e.stroke.is_some());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p726_block_fill_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::None);
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert!(e.fill.is_none(), "fill: none → sem preenchimento");
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p726_block_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("stroke".into(), Value::None);
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert!(e.stroke.is_none(), "stroke: none → sem traço");
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p726_box_fill_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::None);
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert!(e.fill.is_none());
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p726_box_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("stroke".into(), Value::None);
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p726_grid_fill_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("fill".into(), Value::None);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.fill.is_none());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p726_grid_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::None);
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p726_table_fill_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("fill".into(), Value::None);
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert!(e.fill.is_none());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p887_table_stroke_omitido_tem_default_1pt_preto() {
    // P887 (achado 3 de P885) — `stroke` omitido em table() deve ter
    // default `1pt + black` (paridade vanilla, confirmado no
    // código-fonte: `typst-library/src/model/table.rs:268-270` +
    // `visualize/stroke.rs:654-665`), distinto de `stroke: none`
    // explícito (que continua `None`, ver `p726_table_stroke_none_
    // aceite` logo abaixo — não pode regredir).
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("a"))]);
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        let s = e
            .stroke
            .clone()
            .expect("table sem stroke explícito deve ter default 1pt preto");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 0)),
            "stroke default deve ser preta"
        );
        assert_eq!(s.thickness, 1.0, "espessura default deve ser 1pt");
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p726_table_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::None);
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p726_table_cell_fill_e_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("fill".into(), Value::None);
    args.named.insert("stroke".into(), Value::None);
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
        assert!(e.fill.is_none());
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn p726_grid_cell_fill_e_stroke_none_aceite() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("fill".into(), Value::None);
    args.named.insert("stroke".into(), Value::None);
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert!(e.fill.is_none());
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::GridCell");
    }
}

#[test]
fn p726_none_nao_abre_porta_a_tipos_invalidos() {
    // Regressão do caminho de erro: tipos inválidos continuam rejeitados
    // (só `none` foi adicionado como válido).
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("fill".into(), Value::Bool(true));
    assert!(
        native_grid(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
        "fill Bool deve continuar erro em grid"
    );
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::Bool(true));
    assert!(
        native_table(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
        "stroke Bool deve continuar erro em table"
    );
}

#[test]
fn p230_native_grid_cell_stroke_aceita_length_shorthand() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert!(e.stroke.is_some());
        assert_eq!(e.stroke.clone().unwrap().thickness, 2.0);
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p230_native_grid_cell_stroke_aceita_color_shorthand() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert!(e.stroke.is_some());
        assert_eq!(e.stroke.clone().unwrap().thickness, 1.0);
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p230_native_grid_cell_fill_color_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert!(e.fill.is_some());
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn p230_native_grid_cell_fill_length_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
}

#[test]
fn p230_native_table_cell_paridade_gridcell() {
    // table_cell aceita stroke + fill (paridade native_grid_cell).
    null_ctx!(ctx);
    use crate::entities::layout_types::{Color, Length};
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
    args.named.insert("fill".into(), Value::Color(Color::rgb(0, 0, 255)));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
        assert!(e.stroke.is_some() && e.fill.is_some());
    } else {
        panic!("esperado TableCell");
    }
}

#[test]
fn p230_native_grid_cell_stroke_e_fill_simultaneos() {
    // Ambos aceitos no mesmo grid_cell call.
    null_ctx!(ctx);
    use crate::entities::layout_types::{Color, Length};
    let mut args = p(vec![Value::Content(Content::text("c"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
    args.named
        .insert("fill".into(), Value::Color(Color::rgb(100, 100, 100)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::GridCell(e)) = r {
        assert!(e.stroke.is_some() && e.fill.is_some());
    } else {
        panic!("esperado GridCell");
    }
}

#[test]
fn native_table_defaults_columns_rows_auto() {
    // P157A: defaults — columns/rows omitidos caem em [Auto]
    // (paridade com Grid em P83).
    null_ctx!(ctx);
    use crate::entities::layout_types::TrackSizing;
    let r = native_table(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert_eq!(e.columns, vec![TrackSizing::Auto]);
        assert_eq!(e.rows, vec![TrackSizing::Auto]);
        assert!(e.children.is_empty());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn native_table_columns_int() {
    // `#table(columns: 3)` → 3 tracks Auto (paridade Grid).
    null_ctx!(ctx);
    use crate::entities::layout_types::TrackSizing;
    let mut args = p(vec![]);
    args.named.insert("columns".into(), Value::Int(3));
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert_eq!(
            e.columns,
            vec![TrackSizing::Auto, TrackSizing::Auto, TrackSizing::Auto]
        );
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn native_table_columns_array_lengths() {
    // `#table(columns: (10pt, 20pt))` → 2 tracks Fixed.
    null_ctx!(ctx);
    use crate::entities::layout_types::{Length, TrackSizing};
    let mut args = p(vec![]);
    args.named.insert(
        "columns".into(),
        Value::Array(vec![
            Value::Length(Length::pt(10.0)),
            Value::Length(Length::pt(20.0)),
        ]),
    );
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert_eq!(e.columns.len(), 2);
        assert!(matches!(e.columns[0], TrackSizing::Fixed(v) if (v - 10.0).abs() < 1e-6));
        assert!(matches!(e.columns[1], TrackSizing::Fixed(v) if (v - 20.0).abs() < 1e-6));
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn native_table_children_variadicos() {
    // `#table(columns: 2)[a][b][c][d]` → 4 children.
    null_ctx!(ctx);
    let mut args = p(vec![
        Value::Content(Content::text("a")),
        Value::Content(Content::text("b")),
        Value::Content(Content::text("c")),
        Value::Content(Content::text("d")),
    ]);
    args.named.insert("columns".into(), Value::Int(2));
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert_eq!(e.children.len(), 4);
        assert_eq!(e.children[0].plain_text(), "a");
        assert_eq!(e.children[3].plain_text(), "d");
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn native_table_aceita_str_como_child() {
    null_ctx!(ctx);
    let r = native_table(
        &mut ctx,
        &p(vec![Value::Str("hello".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert_eq!(e.children.len(), 1);
        assert_eq!(e.children[0].plain_text(), "hello");
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn native_table_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![]);
    args.named
        .insert("unsupported_named_arg".into(), Value::Length(Length::pt(5.0)));
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em table() deve retornar Err (atributos avançados scope-out per ADR-0054 graded)");
}

#[test]
fn native_table_rejeita_child_int() {
    null_ctx!(ctx);
    let r =
        native_table(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "child Int em table() deve retornar Err");
}

#[test]
fn native_table_paridade_estrutural_com_grid() {
    // P157A: `table` e `grid` com mesmos args produzem variants
    // diferentes (semântica distinta) mas estrutura paralela
    // (mesmas tracks + mesmo número de cells/children).
    null_ctx!(ctx);
    let mut g_args =
        p(vec![Value::Content(Content::text("a")), Value::Content(Content::text("b"))]);
    g_args.named.insert("columns".into(), Value::Int(2));
    let g = native_grid(&mut ctx, &g_args, &null_world(), test_file_id()).unwrap();

    let mut t_args =
        p(vec![Value::Content(Content::text("a")), Value::Content(Content::text("b"))]);
    t_args.named.insert("columns".into(), Value::Int(2));
    let t = native_table(&mut ctx, &t_args, &null_world(), test_file_id()).unwrap();

    // Variants diferentes — não são iguais por PartialEq.
    assert_ne!(g, t);
    // Mas ambos têm 2 cells/children e 2 columns.
    if let (Value::Content(Content::Grid(ge)), Value::Content(Content::Table(te))) =
        (g, t)
    {
        assert_eq!(ge.cells.len(), te.children.len());
        assert_eq!(ge.columns.len(), te.columns.len());
    } else {
        panic!("esperado Grid + Table");
    }
}

#[test]
fn native_table_cell_defaults_todos_none() {
    // P157B: defaults — body required; outros fields None.
    null_ctx!(ctx);
    let r = native_table_cell(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
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
fn native_table_cell_x_y_explicitos() {
    // ADR-0064 Caso A: Some(n) ↔ posição explícita.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("x".into(), Value::Int(2));
    args.named.insert("y".into(), Value::Int(3));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
        assert_eq!(e.x, Some(2));
        assert_eq!(e.y, Some(3));
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn native_table_cell_x_auto_traduz_a_none() {
    // P157B ADR-0064 Caso A: Value::Auto → None.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("x".into(), Value::Auto);
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
        assert_eq!(e.x, None, "Value::Auto deve traduzir para None per ADR-0064 Caso A");
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn native_table_cell_colspan_rowspan_explicitos() {
    // ADR-0064 Caso C: Some(n) ↔ span explícito.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("colspan".into(), Value::Int(2));
    args.named.insert("rowspan".into(), Value::Int(3));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableCell(e)) = r {
        assert_eq!(e.colspan, Some(2));
        assert_eq!(e.rowspan, Some(3));
    } else {
        panic!("esperado Content::TableCell");
    }
}

#[test]
fn native_table_cell_colspan_zero_rejeitado() {
    // P157B: colspan = 0 rejeitado (paridade vanilla NonZeroUsize).
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("colspan".into(), Value::Int(0));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "colspan = 0 deve retornar Err (mínimo 1)");
}

#[test]
fn native_table_cell_colspan_negativo_rejeitado() {
    // P157B: int negativo rejeitado.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("colspan".into(), Value::Int(-1));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "colspan negativo deve retornar Err");
}

#[test]
fn native_table_cell_x_negativo_rejeitado() {
    // P157B: x negativo rejeitado (mínimo 0; sem ints negativos
    // mesmo para campos com min=0).
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("x".into(), Value::Int(-1));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "x negativo deve retornar Err");
}

#[test]
fn native_table_cell_named_arg_desconhecido_rejeitado() {
    null_ctx!(ctx);
    // P235 — `inset` agora conhecido (Categoria B.3); test usa
    // `outset` que continua scope-out per ADR-0054 graded.
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("outset".into(), Value::Int(5));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em table_cell() deve retornar Err");
}

#[test]
fn native_table_cell_sem_body_rejeitado() {
    null_ctx!(ctx);
    let r = native_table_cell(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "table_cell() sem body deve retornar Err");
}

#[test]
fn p235_native_grid_cell_align_aceita() {
    use crate::entities::layout_types::Align2D;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named
        .insert("align".into(), Value::Align(Align2D::from_string("center")));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    match r {
        Ok(Value::Content(Content::GridCell(e))) => assert!(e.align.is_some()),
        other => panic!("esperado GridCell align Some, recebeu {:?}", other),
    }
}

#[test]
fn p235_native_grid_cell_inset_length_uniforme_aceita() {
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(5.0)));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    match r {
        Ok(Value::Content(Content::GridCell(e))) => assert!(e.inset.is_some()),
        other => panic!("esperado GridCell inset Some, recebeu {:?}", other),
    }
}

#[test]
fn p235_native_grid_cell_breakable_bool_aceita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("breakable".into(), Value::Bool(false));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    match r {
        Ok(Value::Content(Content::GridCell(e))) => {
            assert_eq!(e.breakable, Some(false))
        }
        other => {
            panic!("esperado GridCell breakable Some(false), recebeu {:?}", other)
        }
    }
}

#[test]
fn p235_native_grid_cell_breakable_tipo_errado_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("breakable".into(), Value::Int(1));
    let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "breakable Int (não Bool) deve rejeitar pós-P235");
}

#[test]
fn p235_native_table_cell_align_paridade_gridcell() {
    use crate::entities::layout_types::Align2D;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named
        .insert("align".into(), Value::Align(Align2D::from_string("right")));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    match r {
        Ok(Value::Content(Content::TableCell(e))) => assert!(e.align.is_some()),
        other => panic!("esperado TableCell align Some, recebeu {:?}", other),
    }
}

#[test]
fn p235_native_table_cell_inset_paridade_gridcell() {
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
    let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
    match r {
        Ok(Value::Content(Content::TableCell(e))) => assert!(e.inset.is_some()),
        other => panic!("esperado TableCell inset Some, recebeu {:?}", other),
    }
}

#[test]
fn p236_state_final_introspector_vazio_retorna_none() {
    null_ctx!(ctx);
    let r = native_state_final(
        &mut ctx,
        &p(vec![Value::Str("counter_x".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Iter 0 fixpoint: introspector vazio → None.
    assert_eq!(r, Value::None);
}

#[test]
fn p236_state_final_apos_init_retorna_init_value() {
    use crate::entities::location::Location;
    null_ctx!(ctx);
    // Popular StateRegistry directamente via init.
    ctx.introspector
        .state
        .init("k".to_string(), Value::Int(42), Location::from_raw(1));
    let r = native_state_final(
        &mut ctx,
        &p(vec![Value::Str("k".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Sem updates → final = init.
    assert_eq!(r, Value::Int(42));
}

#[test]
fn p236_state_final_apos_updates_retorna_ultimo_valor() {
    use crate::entities::location::Location;
    null_ctx!(ctx);
    ctx.introspector
        .state
        .init("k".to_string(), Value::Int(1), Location::from_raw(1));
    ctx.introspector
        .state
        .update("k".to_string(), Value::Int(2), Location::from_raw(2));
    ctx.introspector
        .state
        .update("k".to_string(), Value::Int(99), Location::from_raw(3));
    let r = native_state_final(
        &mut ctx,
        &p(vec![Value::Str("k".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Último update vence — paridade vanilla state.final().
    assert_eq!(r, Value::Int(99));
}

#[test]
fn p236_state_final_key_inexistente_retorna_none() {
    use crate::entities::location::Location;
    null_ctx!(ctx);
    // Popular outra key (não a key consultada).
    ctx.introspector.state.init(
        "outra".to_string(),
        Value::Int(7),
        Location::from_raw(1),
    );
    let r = native_state_final(
        &mut ctx,
        &p(vec![Value::Str("inexistente".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::None);
}

#[test]
fn p236_state_final_arg_nao_string_retorna_err() {
    null_ctx!(ctx);
    let r = native_state_final(
        &mut ctx,
        &p(vec![Value::Int(42)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "arg não-string deve retornar Err");
}

#[test]
fn p236_state_final_zero_args_retorna_err() {
    null_ctx!(ctx);
    let r = native_state_final(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "zero args deve retornar Err");
}

#[test]
fn p237_state_at_label_inexistente_retorna_none() {
    null_ctx!(ctx);
    // Sem labels nem state populados; label_str não resolve.
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Paridade counter_at empty default: state_at retorna Value::None.
    assert_eq!(r, Value::None);
}

#[test]
fn p237_state_at_key_inexistente_retorna_none() {
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    // Popular label + outro state (não a key consultada).
    ctx.introspector
        .labels
        .add(Label("intro".to_string()), Location::from_raw(5));
    ctx.introspector.state.init(
        "outra".to_string(),
        Value::Int(7),
        Location::from_raw(5),
    );
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("inexistente".into()), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::None);
}

#[test]
fn p237_state_at_resolve_label_retorna_init() {
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    // Popular label "intro" → Location 5; state init em location 1.
    ctx.introspector
        .labels
        .add(Label("intro".to_string()), Location::from_raw(5));
    ctx.introspector
        .state
        .init("k".to_string(), Value::Int(42), Location::from_raw(1));
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Sem updates entre init e label → init wins.
    assert_eq!(r, Value::Int(42));
}

#[test]
fn p237_state_at_updates_antes_location_visivel() {
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    ctx.introspector
        .labels
        .add(Label("at_5".to_string()), Location::from_raw(5));
    ctx.introspector
        .state
        .init("k".to_string(), Value::Int(1), Location::from_raw(1));
    // 2 updates antes da location consultada (raw < 5).
    ctx.introspector
        .state
        .update("k".to_string(), Value::Int(2), Location::from_raw(2));
    ctx.introspector
        .state
        .update("k".to_string(), Value::Int(3), Location::from_raw(3));
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Str("at_5".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Valor em location=5 reflecte último update <= 5: Int(3).
    assert_eq!(r, Value::Int(3));
}

#[test]
fn p237_state_at_updates_depois_location_nao_visiveis() {
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    ctx.introspector
        .labels
        .add(Label("at_2".to_string()), Location::from_raw(2));
    ctx.introspector
        .state
        .init("k".to_string(), Value::Int(10), Location::from_raw(1));
    // Update em location 5 (depois da location consultada raw=2).
    ctx.introspector
        .state
        .update("k".to_string(), Value::Int(99), Location::from_raw(5));
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Str("at_2".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // Em location=2: só init (raw 1) visível; update raw 5 invisível.
    assert_eq!(r, Value::Int(10));
}

#[test]
fn p237_state_at_arg_nao_string_rejeita() {
    null_ctx!(ctx);
    // Key Int (não Str).
    let r = native_state_at(
        &mut ctx,
        &p(vec![Value::Int(42), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "key não-string deve retornar Err");
    // Label Int (não Str).
    let r2 = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Int(7)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r2.is_err(), "label não-string deve retornar Err");
}

#[test]
fn p237_state_at_arity_errada_rejeita() {
    null_ctx!(ctx);
    // 0 args.
    let r0 = native_state_at(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r0.is_err(), "0 args deve retornar Err");
    // 1 arg.
    let r1 = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r1.is_err(), "1 arg deve retornar Err");
    // 3 args.
    let r3 = native_state_at(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Str("l".into()), Value::Int(1)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r3.is_err(), "3 args deve retornar Err");
}

#[test]
fn p242_native_block_radius_length_uniforme() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Length(Length::pt(5.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.radius.top_left, Length::pt(5.0));
        assert_eq!(e.radius.top_right, Length::pt(5.0));
        assert_eq!(e.radius.bottom_right, Length::pt(5.0));
        assert_eq!(e.radius.bottom_left, Length::pt(5.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p242_native_block_radius_dict_por_canto() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("top-left".into(), Value::Length(Length::pt(1.0)));
    d.insert("top-right".into(), Value::Length(Length::pt(2.0)));
    d.insert("bottom-right".into(), Value::Length(Length::pt(3.0)));
    d.insert("bottom-left".into(), Value::Length(Length::pt(4.0)));
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.radius.top_left, Length::pt(1.0));
        assert_eq!(e.radius.top_right, Length::pt(2.0));
        assert_eq!(e.radius.bottom_right, Length::pt(3.0));
        assert_eq!(e.radius.bottom_left, Length::pt(4.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p242_native_block_radius_dict_precedencia_eixo_rest() {
    // Spec Decisão 4: precedência específico > eixo > rest.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    // top específico → ambos top-left/top-right.
    d.insert("top".into(), Value::Length(Length::pt(10.0)));
    // left específico → top-left/bottom-left mas top vence em top-left.
    d.insert("left".into(), Value::Length(Length::pt(20.0)));
    // rest fallback para o que sobra (bottom-right).
    d.insert("rest".into(), Value::Length(Length::pt(99.0)));
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        // top-left: top OR left (precedência top vence: top:10pt).
        assert_eq!(e.radius.top_left, Length::pt(10.0));
        // top-right: top.
        assert_eq!(e.radius.top_right, Length::pt(10.0));
        // bottom-left: left vence (bottom não-set; rest fallback).
        assert_eq!(e.radius.bottom_left, Length::pt(20.0));
        // bottom-right: rest fallback.
        assert_eq!(e.radius.bottom_right, Length::pt(99.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p242_native_block_radius_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "radius negativo deve falhar (preservado P231→P242)");
}

#[test]
fn p242_native_block_radius_chave_canto_invalida_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("northwest".into(), Value::Length(Length::pt(1.0)));
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "chave canto inválida deve falhar");
}

#[test]
fn p242_native_box_radius_paridade_block() {
    // native_box aceita radius paralelo native_block (Decisão 1+4).
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("top-left".into(), Value::Length(Length::pt(7.0)));
    d.insert("rest".into(), Value::Length(Length::pt(0.0)));
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Dict(d));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.radius.top_left, Length::pt(7.0));
        assert_eq!(e.radius.top_right, Length::pt(0.0));
        assert_eq!(e.radius.bottom_right, Length::pt(0.0));
        assert_eq!(e.radius.bottom_left, Length::pt(0.0));
    } else {
        panic!("esperado Boxed");
    }
}

#[test]
fn native_table_header_default_repeat_true() {
    // P157C ADR-0064 Caso D: default vanilla repeat=true.
    null_ctx!(ctx);
    let r = native_table_header(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::TableHeader(e)) = r {
        assert_eq!(e.body.plain_text(), "body");
        assert!(e.repeat, "default vanilla repeat=true (Caso D)");
    } else {
        panic!("esperado Content::TableHeader");
    }
}

#[test]
fn native_table_footer_default_repeat_true() {
    // Par simétrico — paridade absoluta com header.
    null_ctx!(ctx);
    let r = native_table_footer(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::TableFooter(e)) = r {
        assert_eq!(e.body.plain_text(), "body");
        assert!(e.repeat);
    } else {
        panic!("esperado Content::TableFooter");
    }
}

#[test]
fn native_table_header_repeat_false_explicito() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("repeat".into(), Value::Bool(false));
    let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableHeader(e)) = r {
        assert!(!e.repeat);
    } else {
        panic!("esperado Content::TableHeader");
    }
}

#[test]
fn native_table_footer_repeat_false_explicito() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("repeat".into(), Value::Bool(false));
    let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::TableFooter(e)) = r {
        assert!(!e.repeat);
    } else {
        panic!("esperado Content::TableFooter");
    }
}

#[test]
fn native_table_header_sem_body_rejeitado() {
    null_ctx!(ctx);
    let r = native_table_header(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "table_header() sem body deve retornar Err");
}

#[test]
fn native_table_footer_sem_body_rejeitado() {
    null_ctx!(ctx);
    let r = native_table_footer(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "table_footer() sem body deve retornar Err");
}

#[test]
fn native_table_header_repeat_int_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("repeat".into(), Value::Int(1));
    let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "repeat=Int em table_header() deve retornar Err");
}

#[test]
fn native_table_footer_repeat_int_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("repeat".into(), Value::Int(1));
    let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "repeat=Int em table_footer() deve retornar Err");
}

#[test]
fn native_table_header_named_arg_desconhecido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("repeat-rows".into(), Value::Int(2));
    let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "repeat-rows (scope-out) em table_header() deve retornar Err");
}

#[test]
fn native_table_footer_named_arg_desconhecido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("foo".into(), Value::Bool(true));
    let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em table_footer() deve retornar Err");
}
