//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/text.md
//! @prompt-hash 03b9a41d
//! @layer L3
//! @updated 2026-10-24

use super::PageContext;
use super::{
    escape_pdf_string, fill_rg_prefix, pdf_defaults, remap_glyph_id, spot_fill_operator,
    text_to_hex_string, FontScenario,
};
use crate::export::FontKey;
use crate::font_variant::text_style_to_font_variant;
use std::collections::HashMap;
mod runs;

pub(super) use runs::{emit_verbose_text_run, verbose_run_end};

pub(super) fn zero_width_space_alias(
    text: &str,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    font: Option<&typst_core::entities::world_types::Font>,
) -> bool {
    if !text.starts_with('\u{2060}')
        || text.chars().any(|ch| ch != '\u{2060}')
        || glyphs.is_empty()
    {
        return false;
    }
    let outline = match scenario {
        FontScenario::Cidfont { bitmap, .. } => bitmap.is_none(),
        FontScenario::Multifont { fonts, bindings, per_font_bitmap, .. } => {
            crate::export::font_binding::lookup(bindings.as_ref(), fonts, style, font)
                .and_then(|index| per_font_bitmap.get(index))
                .is_some_and(Option::is_none)
        }
        FontScenario::Type1 => false,
    };
    if !outline {
        return false;
    }
    let Some(font) = font else { return false };
    let Ok(face) = ttf_parser::Face::parse(font.as_slice(), 0) else { return false };
    let Some(space) = face.glyph_index(' ') else { return false };
    face.glyph_bounding_box(space).is_none()
        && glyphs.iter().all(|glyph| {
            glyph.glyph_id == space.0
                && glyph.x_advance == 0
                && glyph.x_offset == 0
                && glyph.y_offset == 0
        })
}

fn with_word_joiner_actual_text(
    ops: &mut String,
    text: &str,
    restore: bool,
    zero_alias: bool,
    emit: impl FnOnce(&mut String, bool) -> bool,
) -> bool {
    with_actual_text(
        ops,
        text.contains('\u{2060}').then_some(text),
        restore,
        zero_alias,
        emit,
    )
}

fn with_actual_text(
    ops: &mut String,
    actual: Option<&str>,
    restore: bool,
    zero_alias: bool,
    emit: impl FnOnce(&mut String, bool) -> bool,
) -> bool {
    let Some(text) = actual else {
        return emit(ops, restore);
    };
    if zero_alias {
        ops.push_str("q\n");
    }
    super::text_replacement::opening(ops, text);
    let deferred = emit(ops, false);
    super::text_replacement::closing(ops);
    if zero_alias {
        if deferred {
            ops.push_str("Q\n");
        }
        if restore {
            ops.push_str("Q\n");
            return false;
        }
        return true;
    }
    if restore && deferred {
        ops.push_str("Q\n");
        return false;
    }
    deferred
}

/// A certified original shaped leaf retains its paint and receives only an
/// occurrence replacement. An empty remainder still suppresses glyph aliases.
pub(super) fn emit_shaped_occurrence(
    ops: &mut String,
    origin: (f64, f64),
    item: &typst_core::entities::layout_types::FrameItem,
    ctx: &PageContext,
    actual: &str,
    restore: bool,
) -> bool {
    use typst_core::entities::layout_types::FrameItem;
    let FrameItem::TextShaped { glyphs, text, style, units_per_em, font, .. } = item
    else {
        unreachable!("a certified occurrence retains its original shaped leaf")
    };
    let zero_alias =
        zero_width_space_alias(text, glyphs, style, &ctx.font_scenario, font.as_ref());
    with_actual_text(ops, Some(actual), restore, zero_alias, |ops, restore| {
        let emit = match ctx.mode {
            super::StreamMode::Compact => emit_shaped_pdf_inner,
            super::StreamMode::Verbose => emit_shaped_pdf_verbose_inner,
        };
        emit(
            ops,
            origin.0,
            origin.1,
            glyphs,
            text,
            style,
            &ctx.font_scenario,
            *units_per_em,
            restore,
            font.as_ref(),
            zero_alias,
        )
    })
}

/// Preserve existing integral/decimal spelling without quantizing fractional sizes.
fn font_size(size: f64) -> String {
    let decimal = (size * 10.0).round() / 10.0;
    if (decimal - size).abs() <= 1e-10 {
        format!("{size:.1}")
    } else {
        format!("{size:.5}")
    }
}

/// P530 — devolve o índice da fonte embutida que corresponde ao
/// `(FontList, FontVariant)` derivado do `TextStyle`.
pub(crate) fn font_index_for_style(
    fonts: &[FontKey],
    style: &typst_core::entities::layout_types::TextStyle,
) -> usize {
    let variant = text_style_to_font_variant(style);
    // P836 — a chave inclui as variações explícitas (derivadas não
    // bastam: dois runs com o mesmo FontVariant podem pedir eixos
    // explícitos distintos).
    let variations = style.variations.clone().unwrap_or_default();
    style
        .font
        .as_ref()
        .and_then(|fl| {
            fonts
                .iter()
                .position(|((stored_fl, stored_variant, stored_vars), _)| {
                    stored_fl == fl
                        && stored_variant == &variant
                        && stored_vars == &variations
                })
        })
        .unwrap_or(0)
}

/// **P281** — emit Text PDF dispatched por `FontScenario`.
///
/// `base_y` é a coordenada Y final em PDF (top-level: `page_height -
/// pos.y`; local: `pos.y.0` directo após Group `cm`).
pub(in crate::export) fn emit_text_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    let _ = emit_text_pdf_inner(ops, pos_x, base_y, text, style, scenario, true);
}

pub(in crate::export) fn emit_text_pdf_deferred(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) -> bool {
    emit_text_pdf_inner(ops, pos_x, base_y, text, style, scenario, false)
}

fn emit_text_pdf_inner(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    restore: bool,
) -> bool {
    let mut deferred = false;
    let rg = fill_rg_prefix(&style.fill);
    match scenario {
        FontScenario::Type1 => {
            let safe = escape_pdf_string(text);
            if safe.is_empty() {
                return false;
            }
            let font_ref = match (style.bold, style.italic) {
                (true, _) => "F2",
                (false, true) => "F3",
                (false, false) => "F1",
            };
            let tracking_pt =
                style.tracking.map(|t| t.resolve_pt(style.size.val())).unwrap_or(0.0);
            let tc_op = if tracking_pt.abs() > f64::EPSILON {
                format!("{:.2} Tc\n", tracking_pt)
            } else {
                String::new()
            };
            let stroke_pt = style.faux_bold_stroke_pt(pdf_defaults::FAUX_BOLD_K);
            deferred = stroke_pt > f64::EPSILON && !restore;
            let (q_open, q_close, bold_ops) = if stroke_pt > f64::EPSILON {
                (
                    "q\n",
                    if restore { "Q\n" } else { "" },
                    format!("2 Tr\n{:.3} w\n", stroke_pt),
                )
            } else {
                ("", "", String::new())
            };
            ops.push_str(&format!(
                "{rg}{q_open}BT\n/{font_ref} {} Tf\n{tc_op}{bold_ops}{:.5} {:.5} Td\n({safe}) Tj\nET\n{q_close}",
                font_size(style.size.val()), pos_x, base_y
            ));
        }
        FontScenario::Cidfont { char_to_gid, .. } => {
            if text.is_empty() {
                return false;
            }
            let hex_str = text_to_hex_string(text, char_to_gid);
            ops.push_str(&format!(
                "{rg}BT\n/F1 {} Tf\n{:.5} {:.5} Td\n{hex_str} Tj\nET\n",
                font_size(style.size.val()),
                pos_x,
                base_y
            ));
        }
        FontScenario::Multifont { fonts, per_font_char_to_gid, .. } => {
            if text.is_empty() {
                return false;
            }
            let fi = font_index_for_style(fonts, style);
            let hex_str = text_to_hex_string(text, &per_font_char_to_gid[fi]);
            ops.push_str(&format!(
                "{rg}BT\n/F{} {} Tf\n{:.5} {:.5} Td\n{hex_str} Tj\nET\n",
                fi + 1,
                font_size(style.size.val()),
                pos_x,
                base_y
            ));
        }
    }

    deferred
}

/// **P482/P485** — emit TextShaped PDF via operador `TJ` com avanços explícitos.
///
/// **P485**: cada glifo é emitido individualmente no array `TJ` com o número de
/// deslocamento derivado de `x_advance / units_per_em × 1000` (unidades TJ).
/// Isto garante posicionamento correcto mesmo quando GPOS/kerning altera os avanços
/// relativamente ao `hmtx`. Type1: fallback para `emit_text_pdf` (sem glyph IDs).
/// **P941** — desenha um run de glifos bitmap (CBDT) como imagens XObject.
/// **P956** — inalterado nos dois modos (Verbose e Compact): é desenho de
/// imagem (`q … cm /ImN Do Q`), não emissão de texto — o envelope verbose
/// de ADR-0126 não se aplica.
///
/// Cada glifo é emitido como `q w 0 0 h x y cm /ImN Do Q` na posição calculada
/// a partir do cursor de texto; o cursor avança pelo `x_advance` do shaper,
/// pelo que o espaço na linha é idêntico ao de um glifo de fonte. A fórmula de
/// posicionamento (sinal de `y`) segue krilla `text/glyph/bitmap.rs` e foi
/// verificada visualmente com `mutool draw` (ver `builder.md` §P941).
fn emit_bitmap_type3_text(
    ops: &mut String,
    origin: (f64, f64),
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    refs: &HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>,
    font_index: usize,
    units_per_em: u16,
) -> bool {
    if !glyphs
        .iter()
        .all(|g| refs.get(&g.glyph_id).is_some_and(|r| r.type3_code.is_some()))
    {
        return false;
    }
    let upem = f64::from(units_per_em.max(1));
    let size = style.size.0;
    let mut x = origin.0;
    ops.push_str("q\n");
    for (index, g) in glyphs.iter().enumerate() {
        let code = refs[&g.glyph_id].type3_code.unwrap();
        let gx = x + f64::from(g.x_offset) * size / upem;
        let gy = origin.1 + f64::from(g.y_offset) * size / upem;
        ops.push_str(&format!("BT\n/F{font_index} {} Tf\n0 Tr\n0 Tc\n1 0 0 1 {gx:.9} {gy:.9} Tm\n<{code:02X}> Tj\nET\n",font_size(size)));
        x += f64::from(g.x_advance) * size / upem
            + crate::shaper::tracking::residual(glyphs, index, style, upem);
    }
    true
}

pub(super) fn emit_bitmap_glyph_draws(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    bitmap: &HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>,
    units_per_em: u16,
) {
    let upm = units_per_em as f64;
    let size = style.size.val();
    let mut cur_x = pos_x;
    for (i, g) in glyphs.iter().enumerate() {
        let x_off_pt = g.x_offset as f64 / upm * size;
        if let Some(bmp) = bitmap.get(&g.glyph_id) {
            let scale = size / bmp.pixels_per_em as f64;
            let w_pt = bmp.width as f64 * scale;
            let h_pt = bmp.height as f64 * scale;
            let x_pt = cur_x + x_off_pt + bmp.bearing_x as f64 * scale;
            // bearing_y (RasterGlyphImage.y) é o offset do topo do bitmap
            // acima da linha de base, em pixels; negativo em NotoColorEmoji
            // (-27) porque o bitmap desce ligeiramente abaixo da linha.
            let y_bottom = base_y + bmp.bearing_y as f64 * scale;
            ops.push_str(&format!(
                "q\n{w_pt:.3} 0 0 {h_pt:.3} {x_pt:.3} {y_bottom:.3} cm\n/{} Do\nQ\n",
                bmp.name
            ));
        }
        cur_x += g.x_advance as f64 / upm * size
            + crate::shaper::tracking::residual(glyphs, i, style, upm);
    }
}

/// **P956** — entradas do array `TJ` partilhadas pelos modos compact e
/// verbose: delta model P520/P548 (nominal − x_advance), `x_offset` P486 e
/// remap de subsetting P516. Conserva as frações em unidades de texto,
/// tanto sem tracking quanto com resíduo de tracking.
pub(super) fn push_tj_glyph_entries(
    ops: &mut String,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    glyph_mapping: &HashMap<u16, u16>,
    glyph_to_nominal: &HashMap<u16, i32>,
    upm: f64,
    style: Option<&typst_core::entities::layout_types::TextStyle>,
) {
    for (i, g) in glyphs.iter().enumerate() {
        // P486 — x_offset: deslocar glifo sem alterar o avanço do próximo.
        if g.x_offset != 0 {
            let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
            ops.push_str(&format!("{xoff_tu} "));
        }
        // P520/P548 — delta model: o CIDFont /W já fornece a largura
        // nominal (hmtx). O TJ deve conter apenas a diferença entre essa
        // largura declarada e o avanço real produzido pelo shaper. No
        // operador PDF TJ, o número é subtraído da coordenada horizontal,
        // logo `nominal - x_advance` é o sinal correcto: positivo para
        // kerning negativo (aproxima), negativo para kerning positivo
        // (afasta).
        let nominal = glyph_to_nominal.get(&g.glyph_id).copied().unwrap_or(g.x_advance);
        let advance_tu = (nominal - g.x_advance) as f64 / upm * 1000.0;
        let new_gid = if glyph_mapping.is_empty() {
            g.glyph_id
        } else {
            remap_glyph_id(g.glyph_id, glyph_mapping)
        };
        let residual_tu = style
            .map(|style| {
                let residual = crate::shaper::tracking::residual(glyphs, i, style, upm);
                if residual == 0.0 {
                    0.0
                } else {
                    residual / style.size.0 * 1000.0
                }
            })
            .unwrap_or(0.0);
        ops.push_str(&format!("<{:04X}> {} ", new_gid, advance_tu - residual_tu));
    }
}

/// **P956** — prefixo do envelope verbose (ADR-0126): isola o bloco com
/// `q`, carrega posição + flip Y no `cm` (`Tm` fica sempre
/// `1 0 0 -1 0 0`), e declara a cor de preenchimento por bloco via
/// `/c0 cs … scn` (`/c0` é o colour space sRGB ICCBased dos recursos da
/// página — `builder.md` §P956). `base_y` é exactamente o valor que o
/// compacto passaria ao `Td` (top-level: `page_height − pos.y`; local em
/// Group: `pos.y` directo) — a derivação F·F=I de `stream.md` §P956
/// garante geometria idêntica nos dois modos. Fill `None` → preto `0 0 0`.
/// Precisão do `cm`: 5 casas decimais (convenção P777 das matrizes `cm` de
/// imagem; o vanilla usa 5-8 dígitos — medido em P956: `70.86614`).
pub(super) fn verbose_block_prefix(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    fill: &Option<typst_core::entities::layout_types::Color>,
) {
    let alpha = fill.as_ref().map(|c| c.to_rgba_f32().3).unwrap_or(1.0);
    let opacity = super::opacity_operator(alpha, alpha);
    if let Some(op) = fill.as_ref().and_then(spot_fill_operator) {
        ops.push_str(&format!(
            "q\n1 0 0 -1 {pos_x:.5} {base_y:.5} cm\n{opacity}{op}BT\n"
        ));
        return;
    }
    let (r, g, b) = match fill {
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            (r, g, b)
        }
        None => (0.0, 0.0, 0.0),
    };
    ops.push_str(&format!(
        "q\n1 0 0 -1 {pos_x:.5} {base_y:.5} cm\n{opacity}/c0 cs {r:.3} {g:.3} {b:.3} scn\nBT\n"
    ));
}

/// **P956** — operador `Tr` explícito do envelope verbose: `0 Tr` por
/// omissão; faux-bold (P139) → `2 Tr` + `{stroke:.3} w` no mesmo envelope.
pub(super) fn verbose_tr_ops(
    style: &typst_core::entities::layout_types::TextStyle,
) -> String {
    let stroke_pt = style.faux_bold_stroke_pt(pdf_defaults::FAUX_BOLD_K);
    if stroke_pt > f64::EPSILON {
        format!("2 Tr\n{stroke_pt:.3} w\n")
    } else {
        "0 Tr\n".to_string()
    }
}

/// A supplied face can realize the requested static weight or a real wght axis.
/// Keep semantic weight intact without painting an additional synthetic stroke.
pub(super) fn shaped_tr_ops(
    style: &typst_core::entities::layout_types::TextStyle,
    font: Option<&typst_core::entities::world_types::Font>,
) -> String {
    if style.faux_bold_stroke_pt(pdf_defaults::FAUX_BOLD_K) > f64::EPSILON {
        let realized_weight = font
            .and_then(|font| ttf_parser::Face::parse(font.as_slice(), 0).ok())
            .is_some_and(|face| {
                (face.tables().fvar.is_none()
                    && style.weight == Some(face.weight().to_number()))
                    || face.tables().fvar.is_some_and(|fvar| {
                        fvar.axes
                            .into_iter()
                            .any(|axis| axis.tag == ttf_parser::Tag::from_bytes(b"wght"))
                    })
            });
        if realized_weight {
            return "0 Tr\n".to_string();
        }
    }
    verbose_tr_ops(style)
}

/// **P956** — `Tc` do envelope verbose, apenas se tracking ≠ 0 (mesma
/// convenção do caminho Type1 compact).
pub(super) fn push_tc_if_tracking(
    ops: &mut String,
    style: &typst_core::entities::layout_types::TextStyle,
) {
    let tracking_pt =
        style.tracking.map(|t| t.resolve_pt(style.size.val())).unwrap_or(0.0);
    if tracking_pt.abs() > f64::EPSILON {
        ops.push_str(&format!("{tracking_pt:.2} Tc\n"));
    }
}

/// Shaped advances already include character spacing; clear inherited PDF Tc.
pub(super) fn push_shaped_spacing(ops: &mut String) {
    ops.push_str("0 Tc\n");
}

#[expect(
    clippy::too_many_arguments,
    reason = "C051: Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos"
)]
pub(in crate::export) fn emit_shaped_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    font: Option<&typst_core::entities::world_types::Font>,
) {
    let zero_alias = zero_width_space_alias(text, glyphs, style, scenario, font);
    let _ = with_word_joiner_actual_text(ops, text, true, zero_alias, |ops, restore| {
        emit_shaped_pdf_inner(
            ops,
            pos_x,
            base_y,
            glyphs,
            text,
            style,
            scenario,
            units_per_em,
            restore,
            font,
            zero_alias,
        )
    });
}

#[expect(
    clippy::too_many_arguments,
    reason = "C052: Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica"
)]
pub(in crate::export) fn emit_shaped_pdf_deferred(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    font: Option<&typst_core::entities::world_types::Font>,
) -> bool {
    let zero_alias = zero_width_space_alias(text, glyphs, style, scenario, font);
    with_word_joiner_actual_text(ops, text, false, zero_alias, |ops, restore| {
        emit_shaped_pdf_inner(
            ops,
            pos_x,
            base_y,
            glyphs,
            text,
            style,
            scenario,
            units_per_em,
            restore,
            font,
            zero_alias,
        )
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "C053: Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão"
)]
pub(super) fn emit_shaped_pdf_inner(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    restore: bool,
    font: Option<&typst_core::entities::world_types::Font>,
    zero_alias: bool,
) -> bool {
    let mut deferred = false;
    if glyphs.is_empty() {
        return false;
    }
    let upm = units_per_em as f64;
    match scenario {
        FontScenario::Type1 => {
            deferred =
                emit_text_pdf_inner(ops, pos_x, base_y, text, style, scenario, restore);
        }
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, bitmap, .. } => {
            if let Some(bmp) = bitmap {
                if emit_bitmap_type3_text(
                    ops,
                    (pos_x, base_y),
                    glyphs,
                    style,
                    bmp,
                    1,
                    units_per_em,
                ) {
                    if restore {
                        ops.push_str("Q\n");
                    }
                    return !restore;
                }
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return false;
            }
            let rg = fill_rg_prefix(&style.fill);
            ops.push_str(&format!("{rg}BT\n/F1 {} Tf\n", font_size(style.size.val())));
            if zero_alias {
                ops.push_str("0 Tz\n");
            }
            push_shaped_spacing(ops);
            ops.push_str(&format!("{pos_x:.5} {base_y:.5} Td\n[ "));
            push_tj_glyph_entries(
                ops,
                glyphs,
                glyph_mapping,
                glyph_to_nominal,
                upm,
                Some(style),
            );
            ops.push_str("] TJ\nET\n");
        }
        FontScenario::Multifont {
            fonts,
            bindings,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            per_font_bitmap,
            ..
        } => {
            let Some(fi) = crate::export::font_binding::lookup(
                bindings.as_ref(),
                fonts,
                style,
                font,
            ) else {
                return false;
            };
            if let Some(Some(bmp)) = per_font_bitmap.get(fi) {
                if emit_bitmap_type3_text(
                    ops,
                    (pos_x, base_y),
                    glyphs,
                    style,
                    bmp,
                    fi + 1,
                    units_per_em,
                ) {
                    if restore {
                        ops.push_str("Q\n");
                    }
                    return !restore;
                }
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return false;
            }
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let glyph_to_nominal = &per_font_glyph_to_nominal[fi];
            let rg = fill_rg_prefix(&style.fill);
            ops.push_str(&format!(
                "{rg}BT\n/F{} {} Tf\n",
                fi + 1,
                font_size(style.size.val())
            ));
            if zero_alias {
                ops.push_str("0 Tz\n");
            }
            push_shaped_spacing(ops);
            ops.push_str(&format!("{pos_x:.5} {base_y:.5} Td\n[ "));
            push_tj_glyph_entries(
                ops,
                glyphs,
                glyph_mapping,
                glyph_to_nominal,
                upm,
                Some(style),
            );
            ops.push_str("] TJ\nET\n");
        }
    }

    deferred
}

/// Type1 silently ignored (sem TrueType embebida); CIDFont/Multifont
/// emitem `<{:04X}>` Identity-H via `/F1` (math fonts).
///
/// **P906** — `FontScenario::Multifont` tem várias fontes embutidas
/// (`/F1`, `/F2`, ...); antes deste passo, esta função hardcodava `/F1`
/// incondicionalmente, correcto por coincidência só quando o glifo vinha
/// mesmo da primeira fonte embutida. Achado: glifos de esticamento
/// (`FrameItem::Glyph`, usados por `layout_stretchy_delimiter`/
/// `layout_assembly`, eixo vertical e — desde este passo — horizontal)
/// resolvidos numa fonte companion MATH (índice != 0) desenhavam-se com o
/// `glyph_id` certo interpretado pela fonte errada — invisível ou lixo
/// visual. Corrigido reaproveitando `per_font_glyph_reverse` (mesmo `build_
/// math_glyph_reverse_map` já usado no subsetting DEBT-9/P45,
/// `builder.rs:988`) — não precisa de `style`: procura em qual fonte o
/// `glyph_id` está efectivamente presente (glifos de esticamento só
/// existem na fonte de onde vieram, por construção do subsetting).
/// `FontScenario::Cidfont` continua a usar sempre `/F1` (só tem uma fonte
/// candidata — mesmo comportamento de antes, preservado).
///
/// **P906 (2º achado nesta função)** — nem o ramo `Cidfont` nem o
/// `Multifont` aplicavam `remap_glyph_id`/`per_font_glyph_mapping` (P516,
/// já aplicado a `emit_text_pdf`/`emit_shaped_pdf`) ao `glyph_id` recebido
/// — desenhava sempre o índice ORIGINAL (pré-subsetting) da fonte
/// completa, mas a fonte efectivamente embutida no PDF está subsetada
/// (renumerada) sempre que houver `glyph_mapping` não vazio. Resultado:
/// `<XXXX> Tj` referenciava um slot de glifo errado/inexistente na fonte
/// embutida — glifo invisível ou lixo, mesmo depois da selecção de `/Fn`
/// já estar correcta. Corrigido aplicando o mesmo remap já usado no
/// caminho de texto. Ver `export/stream.md` §P906.
pub(in crate::export) fn emit_glyph_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    emit_glyph_pdf_marked(
        ops, pos_x, base_y, glyph_id, size, style, scenario, None, false,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "C054: Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente"
)]
pub(in crate::export) fn emit_glyph_pdf_marked(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    actualtext_open: Option<&str>,
    actualtext_close: bool,
) {
    if let Some(c) = &style.fill {
        if let Some(op) = spot_fill_operator(c) {
            ops.push_str(&op);
        } else {
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{r:.3} {g:.3} {b:.3} rg\n"));
        }
    }
    match scenario {
        FontScenario::Type1 => {
            // Sem fonte TrueType → glyph_id sem significado. Ignored.
        }
        FontScenario::Cidfont { glyph_mapping, .. } => {
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            ops.push_str("BT\n");
            if let Some(text) = actualtext_open {
                super::text_replacement::opening(ops, text);
            }
            ops.push_str(&format!(
                "/F1 {} Tf\n{:.5} {:.5} Td\n<{:04X}> Tj\n",
                font_size(size.val()),
                pos_x,
                base_y,
                new_gid
            ));
            if actualtext_close {
                super::text_replacement::closing(ops);
            }
            ops.push_str("ET\n");
        }
        FontScenario::Multifont {
            per_font_glyph_reverse,
            per_font_glyph_mapping,
            ..
        } => {
            let fi = per_font_glyph_reverse
                .iter()
                .position(|m| m.contains_key(&glyph_id))
                .unwrap_or(0);
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            ops.push_str("BT\n");
            if let Some(text) = actualtext_open {
                super::text_replacement::opening(ops, text);
            }
            ops.push_str(&format!(
                "/F{} {} Tf\n{:.5} {:.5} Td\n<{:04X}> Tj\n",
                fi + 1,
                font_size(size.val()),
                pos_x,
                base_y,
                new_gid
            ));
            if actualtext_close {
                super::text_replacement::closing(ops);
            }
            ops.push_str("ET\n");
        }
    }
}

// ── P956 — variantes verbose (envelope vanilla-espelhado, ADR-0126) ────────
//
// Por bloco de texto: `q` + `cm 1 0 0 -1 x y` (posição + flip Y) +
// `/c0 cs r g b scn` (cor por bloco) + `BT 0 Tr /F{n} {size} Tf [Tc]
// 1 0 0 -1 0 0 Tm … ET` + `Q`. O `base_y` recebido é o mesmo valor que o
// compacto passa ao `Td` (derivação F·F=I em stream.md §P956) — o que muda
// é o envelope, não a coordenada.

/// **P956** — `emit_text_pdf` verbose: mesmo envelope com `(…) Tj` (Type1)
/// ou `<hex> Tj` (CIDFont/Multifont).
pub(in crate::export) fn emit_text_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    let _ = emit_text_pdf_verbose_inner(ops, pos_x, base_y, text, style, scenario, true);
}

pub(in crate::export) fn emit_text_pdf_verbose_deferred(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) -> bool {
    emit_text_pdf_verbose_inner(ops, pos_x, base_y, text, style, scenario, false)
}

fn emit_text_pdf_verbose_inner(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    restore: bool,
) -> bool {
    let mut deferred = false;
    match scenario {
        FontScenario::Type1 => {
            let safe = escape_pdf_string(text);
            if safe.is_empty() {
                return false;
            }
            let font_ref = match (style.bold, style.italic) {
                (true, _) => "F2",
                (false, true) => "F3",
                (false, false) => "F1",
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/{font_ref} {} Tf\n", font_size(style.size.val())));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n({safe}) Tj\nET\n"));
            if restore {
                ops.push_str("Q\n");
            } else {
                deferred = true;
            }
        }
        FontScenario::Cidfont { char_to_gid, .. } => {
            if text.is_empty() {
                return false;
            }
            let hex_str = text_to_hex_string(text, char_to_gid);
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F1 {} Tf\n", font_size(style.size.val())));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n{hex_str} Tj\nET\n"));
            if restore {
                ops.push_str("Q\n");
            } else {
                deferred = true;
            }
        }
        FontScenario::Multifont { fonts, per_font_char_to_gid, .. } => {
            if text.is_empty() {
                return false;
            }
            let fi = font_index_for_style(fonts, style);
            let hex_str = text_to_hex_string(text, &per_font_char_to_gid[fi]);
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F{} {} Tf\n", fi + 1, font_size(style.size.val())));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n{hex_str} Tj\nET\n"));
            if restore {
                ops.push_str("Q\n");
            } else {
                deferred = true;
            }
        }
    }

    deferred
}

/// **P956** — `emit_shaped_pdf` verbose: mesmo envelope com o array `TJ`
/// partilhado (`push_tj_glyph_entries` — delta model P520/P548, x_offset
/// P486, remap P516). Glifos bitmap (CBDT, P941) ficam **inalterados nos
/// dois modos** — são desenho de imagem (`/ImN Do`), não texto.
#[expect(
    clippy::too_many_arguments,
    reason = "C055: Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico"
)]
pub(in crate::export) fn emit_shaped_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    font: Option<&typst_core::entities::world_types::Font>,
) {
    let zero_alias = zero_width_space_alias(text, glyphs, style, scenario, font);
    let _ = with_word_joiner_actual_text(ops, text, true, zero_alias, |ops, restore| {
        emit_shaped_pdf_verbose_inner(
            ops,
            pos_x,
            base_y,
            glyphs,
            text,
            style,
            scenario,
            units_per_em,
            restore,
            font,
            zero_alias,
        )
    });
}

#[expect(
    clippy::too_many_arguments,
    reason = "C056: Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente"
)]
pub(in crate::export) fn emit_shaped_pdf_verbose_deferred(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    font: Option<&typst_core::entities::world_types::Font>,
) -> bool {
    let zero_alias = zero_width_space_alias(text, glyphs, style, scenario, font);
    with_word_joiner_actual_text(ops, text, false, zero_alias, |ops, restore| {
        emit_shaped_pdf_verbose_inner(
            ops,
            pos_x,
            base_y,
            glyphs,
            text,
            style,
            scenario,
            units_per_em,
            restore,
            font,
            zero_alias,
        )
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "C057: Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes"
)]
pub(super) fn emit_shaped_pdf_verbose_inner(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
    restore: bool,
    font: Option<&typst_core::entities::world_types::Font>,
    zero_alias: bool,
) -> bool {
    let mut deferred = false;
    if glyphs.is_empty() {
        return false;
    }
    let upm = units_per_em as f64;
    match scenario {
        FontScenario::Type1 => {
            deferred = emit_text_pdf_verbose_inner(
                ops, pos_x, base_y, text, style, scenario, restore,
            );
        }
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, bitmap, .. } => {
            if let Some(bmp) = bitmap {
                if emit_bitmap_type3_text(
                    ops,
                    (pos_x, base_y),
                    glyphs,
                    style,
                    bmp,
                    1,
                    units_per_em,
                ) {
                    if restore {
                        ops.push_str("Q\n");
                    }
                    return !restore;
                }
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return false;
            }
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&shaped_tr_ops(style, font));
            ops.push_str(&format!("/F1 {} Tf\n", font_size(style.size.val())));
            if zero_alias {
                ops.push_str("0 Tz\n");
            }
            push_shaped_spacing(ops);
            ops.push_str("1 0 0 -1 0 0 Tm\n[ ");
            push_tj_glyph_entries(
                ops,
                glyphs,
                glyph_mapping,
                glyph_to_nominal,
                upm,
                Some(style),
            );
            ops.push_str("] TJ\nET\n");
            if restore {
                ops.push_str("Q\n");
            } else {
                deferred = true;
            }
        }
        FontScenario::Multifont {
            fonts,
            bindings,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            per_font_bitmap,
            ..
        } => {
            let Some(fi) = crate::export::font_binding::lookup(
                bindings.as_ref(),
                fonts,
                style,
                font,
            ) else {
                return false;
            };
            if let Some(Some(bmp)) = per_font_bitmap.get(fi) {
                if emit_bitmap_type3_text(
                    ops,
                    (pos_x, base_y),
                    glyphs,
                    style,
                    bmp,
                    fi + 1,
                    units_per_em,
                ) {
                    if restore {
                        ops.push_str("Q\n");
                    }
                    return !restore;
                }
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return false;
            }
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&shaped_tr_ops(style, font));
            ops.push_str(&format!("/F{} {} Tf\n", fi + 1, font_size(style.size.val())));
            if zero_alias {
                ops.push_str("0 Tz\n");
            }
            push_shaped_spacing(ops);
            ops.push_str("1 0 0 -1 0 0 Tm\n[ ");
            push_tj_glyph_entries(
                ops,
                glyphs,
                &per_font_glyph_mapping[fi],
                &per_font_glyph_to_nominal[fi],
                upm,
                Some(style),
            );
            ops.push_str("] TJ\nET\n");
            if restore {
                ops.push_str("Q\n");
            } else {
                deferred = true;
            }
        }
    }

    deferred
}

/// **P956** — `emit_glyph_pdf` verbose (stretchy): mesmo envelope com
/// `<gid> Tj`. Sem `style` → fill preto e `0 Tr` (como o compact, que
/// também não emite cor neste caminho).
pub(in crate::export) fn emit_glyph_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    emit_glyph_pdf_verbose_marked(
        ops, pos_x, base_y, glyph_id, size, style, scenario, None, false,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "C058: Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado"
)]
pub(in crate::export) fn emit_glyph_pdf_verbose_marked(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    actualtext_open: Option<&str>,
    actualtext_close: bool,
) {
    match scenario {
        FontScenario::Type1 => {
            // Sem fonte TrueType → glyph_id sem significado. Ignored.
        }
        FontScenario::Cidfont { glyph_mapping, .. } => {
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            if let Some(text) = actualtext_open {
                super::text_replacement::opening(ops, text);
            }
            ops.push_str("0 Tr\n");
            ops.push_str(&format!("/F1 {} Tf\n", font_size(size.val())));
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n<{new_gid:04X}> Tj\n"));
            if actualtext_close {
                super::text_replacement::closing(ops);
            }
            ops.push_str("ET\nQ\n");
        }
        FontScenario::Multifont {
            per_font_glyph_reverse,
            per_font_glyph_mapping,
            ..
        } => {
            let fi = per_font_glyph_reverse
                .iter()
                .position(|m| m.contains_key(&glyph_id))
                .unwrap_or(0);
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            if let Some(text) = actualtext_open {
                super::text_replacement::opening(ops, text);
            }
            ops.push_str("0 Tr\n");
            ops.push_str(&format!("/F{} {} Tf\n", fi + 1, font_size(size.val())));
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n<{new_gid:04X}> Tj\n"));
            if actualtext_close {
                super::text_replacement::closing(ops);
            }
            ops.push_str("ET\nQ\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::ShapedGlyph;

    #[test]
    fn readiness_static_face_realized_weight_is_not_stroked_again() {
        use typst_core::entities::layout_types::{
            FrameItem, Point, Pt, ShapedGlyph, TextStyle,
        };
        use typst_core::entities::world_types::Font;
        let font = typst_assets::fonts()
            .find_map(|data| {
                let face = ttf_parser::Face::parse(data, 0).ok()?;
                (face.weight().to_number() == 700 && face.tables().fvar.is_none())
                    .then(|| Font::from_data(data.to_vec()))
            })
            .expect("bundled actual static 700 face");
        let style = TextStyle {
            size: Pt(20.0),
            weight: Some(700),
            ..Default::default()
        };
        let glyphs = vec![ShapedGlyph {
            glyph_id: 1,
            x_advance: 689,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'H',
        }];
        let map = HashMap::new();
        let chars = HashMap::new();
        let nominal = HashMap::from([(1, 689)]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &map,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        let mut ops = String::new();
        emit_shaped_pdf_verbose(
            &mut ops,
            10.0,
            20.0,
            &glyphs,
            "H",
            &style,
            &scenario,
            1000,
            Some(&font),
        );
        assert!(
            !ops.contains("2 Tr"),
            "actual static weight must not acquire a synthetic stroke: {ops}"
        );
        ops.clear();
        emit_shaped_pdf_verbose(
            &mut ops, 10.0, 20.0, &glyphs, "H", &style, &scenario, 1000, None,
        );
        assert!(ops.contains("2 Tr"), "legacy None preserves its policy");
        let regular = typst_assets::fonts()
            .find_map(|data| {
                let face = ttf_parser::Face::parse(data, 0).ok()?;
                (face.weight().to_number() == 400 && face.tables().fvar.is_none())
                    .then(|| Font::from_data(data.to_vec()))
            })
            .expect("bundled actual static regular face");
        ops.clear();
        emit_shaped_pdf_verbose(
            &mut ops,
            10.0,
            20.0,
            &glyphs,
            "H",
            &style,
            &scenario,
            1000,
            Some(&regular),
        );
        assert!(ops.contains("2 Tr"), "unmatched actual face preserves its old policy");
        let ptrs = HashMap::new();
        let pats = HashMap::new();
        let mut ctx = PageContext::type1(
            &ptrs,
            &[],
            &pats,
            &[],
            crate::export::StreamMode::Verbose,
        );
        ctx.font_scenario = scenario;
        let run: Vec<_> = [10.0, 20.0]
            .into_iter()
            .map(|x| FrameItem::TextShaped {
                pos: Point { x: Pt(x), y: Pt(20.0) },
                glyphs: glyphs.clone(),
                style: style.clone(),
                text: "H".into(),
                units_per_em: 1000,
                bidi: None,
                font: Some(font.clone()),
            })
            .collect();
        ops.clear();
        super::runs::emit_verbose_text_run(&mut ops, &run, 200.0, &ctx);
        assert!(
            !ops.contains("2 Tr"),
            "coalesced static runs must not acquire a synthetic stroke: {ops}"
        );
    }

    #[test]
    fn certified_variable_weight_is_not_synthetically_stroked() {
        use typst_core::entities::layout_types::{
            FrameItem, Point, Pt, ShapedGlyph, TextStyle,
        };
        use typst_core::entities::world_types::Font;
        let font = Font::from_data(
            std::fs::read(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/fixtures/fonts/UbuntuSans-Variable.ttf"
            ))
            .unwrap(),
        );
        let style = TextStyle {
            size: Pt(20.0),
            weight: Some(700),
            variations: Some(typst_core::entities::font_variations::FontVariations(
                vec![(*b"wght", 250.0)],
            )),
            ..Default::default()
        };
        let glyphs = vec![ShapedGlyph {
            glyph_id: 1,
            x_advance: 689,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'H',
        }];
        let map = HashMap::new();
        let chars = HashMap::new();
        let nominal = HashMap::from([(1, 689)]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &map,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        let mut ops = String::new();
        emit_shaped_pdf_verbose(
            &mut ops,
            10.0,
            20.0,
            &glyphs,
            "H",
            &style,
            &scenario,
            1000,
            Some(&font),
        );
        assert!(
            !ops.contains("2 Tr"),
            "real variable weight must not acquire a synthetic stroke: {ops}"
        );
        ops.clear();
        emit_shaped_pdf_verbose(
            &mut ops, 10.0, 20.0, &glyphs, "H", &style, &scenario, 1000, None,
        );
        assert!(ops.contains("2 Tr"), "legacy None preserves its policy");
        let ptrs = HashMap::new();
        let pats = HashMap::new();
        let mut ctx = PageContext::type1(
            &ptrs,
            &[],
            &pats,
            &[],
            crate::export::StreamMode::Verbose,
        );
        ctx.font_scenario = scenario;
        let run: Vec<_> = [10.0, 20.0]
            .into_iter()
            .map(|x| FrameItem::TextShaped {
                pos: Point { x: Pt(x), y: Pt(20.0) },
                glyphs: glyphs.clone(),
                style: style.clone(),
                text: "H".into(),
                units_per_em: 1000,
                bidi: None,
                font: Some(font.clone()),
            })
            .collect();
        ops.clear();
        super::runs::emit_verbose_text_run(&mut ops, &run, 200.0, &ctx);
        assert!(
            !ops.contains("2 Tr"),
            "coalesced variable runs must not acquire a synthetic stroke: {ops}"
        );
    }

    #[test]
    fn fractional_pdf_compact_absolute_origins() {
        let style = typst_core::entities::layout_types::TextStyle::regular(
            typst_core::entities::layout_types::Pt(11.0),
        );
        let empty = HashMap::new();
        let chars = HashMap::from([('A', 1)]);
        let nominal = HashMap::from([(1, 500)]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &empty,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        let mut plain = String::new();
        emit_text_pdf(&mut plain, 12.34567, 23.45678, "A", &style, &scenario);
        let mut type1 = String::new();
        emit_text_pdf(&mut type1, 12.34567, 23.45678, "A", &style, &FontScenario::Type1);
        let mut glyph = String::new();
        emit_glyph_pdf(&mut glyph, 12.34567, 23.45678, 1, style.size, &style, &scenario);
        let mut shaped = String::new();
        let glyphs = [ShapedGlyph {
            glyph_id: 1,
            x_advance: 500,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }];
        emit_shaped_pdf(
            &mut shaped,
            12.34567,
            23.45678,
            &glyphs,
            "A",
            &style,
            &scenario,
            1000,
            None,
        );
        for ops in [plain, type1, glyph, shaped] {
            let td = ops.lines().find_map(|line| line.strip_suffix(" Td")).unwrap();
            let xy: Vec<f64> =
                td.split_whitespace().map(|v| v.parse().unwrap()).collect();
            assert!(
                (xy[0] - 12.34567).abs() < 1e-7 && (xy[1] - 23.45678).abs() < 1e-7,
                "{ops}"
            );
        }
    }

    #[test]
    fn fractional_pdf_single_glyph_advances_and_offsets() {
        let glyphs = [ShapedGlyph {
            glyph_id: 65,
            x_advance: 1280,
            x_offset: 13,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }];
        let mut ops = String::new();
        push_tj_glyph_entries(
            &mut ops,
            &glyphs,
            &HashMap::from([(65, 3)]),
            &HashMap::from([(65, 1400)]),
            2048.0,
            None,
        );
        let mut cursor = 0.0;
        let mut painted = None;
        for token in ops.split_whitespace() {
            if token.starts_with('<') {
                assert_eq!(token, "<0003>");
                painted = Some(cursor);
                cursor += 1400.0 / 2048.0 * 11.0;
            } else {
                cursor -= token.parse::<f64>().unwrap() / 1000.0 * 11.0;
            }
        }
        assert!((painted.unwrap() - 13.0 / 2048.0 * 11.0).abs() < 1e-7, "{ops}");
        assert!((cursor - (1280.0 + 13.0) / 2048.0 * 11.0).abs() < 1e-7, "{ops}");
    }

    #[test]
    fn tj_remap_preserves_nominal_delta() {
        let glyphs = [ShapedGlyph {
            glyph_id: 65,
            x_advance: 599,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }];
        let mut ops = String::new();
        push_tj_glyph_entries(
            &mut ops,
            &glyphs,
            &HashMap::from([(65, 3)]),
            &HashMap::from([(65, 639)]),
            1000.0,
            None,
        );
        assert_eq!(ops, "<0003> 40 ");
    }
    // Interpret PDF text advance independently from the formatter's delta model.
    fn rendered_positions(ops: &str, size: f64) -> Vec<f64> {
        let tc = ops
            .split("[ ")
            .next()
            .unwrap()
            .lines()
            .filter_map(|line| line.strip_suffix(" Tc"))
            .next_back()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap();
        let array = ops.split("[ ").nth(1).unwrap().split("] TJ").next().unwrap();
        let mut cursor = 0.0;
        let mut positions = Vec::new();
        for token in array.split_whitespace() {
            if token.starts_with('<') {
                positions.push(cursor);
                cursor += 500.0 / 1000.0 * size + tc;
            } else {
                cursor -= token.parse::<f64>().unwrap() / 1000.0 * size;
            }
        }
        positions
    }

    fn tracking_controls(
        mut test: impl FnMut(
            &FontScenario,
            &typst_core::entities::layout_types::TextStyle,
            Vec<ShapedGlyph>,
        ),
    ) {
        use typst_core::entities::layout_types::{Length, Pt, TextStyle};
        let char_map = HashMap::from([('A', 1)]);
        let nominal = HashMap::from([(1, 500)]);
        let empty = HashMap::new();
        let chars = [char_map.clone()];
        let maps = [empty.clone()];
        let widths = [nominal.clone()];
        let reverse = [HashMap::new()];
        let bitmaps = [None];
        let scenarios = [
            FontScenario::Cidfont {
                char_to_gid: &char_map,
                glyph_mapping: &empty,
                glyph_to_nominal: &nominal,
                bitmap: None,
            },
            FontScenario::Multifont {
                fonts: &[],
                per_font_char_to_gid: &chars,
                per_font_glyph_mapping: &maps,
                per_font_glyph_to_nominal: &widths,
                per_font_glyph_reverse: &reverse,
                per_font_bitmap: &bitmaps,
                bindings: None,
            },
        ];
        for scenario in &scenarios {
            for tracking in [Some(1.2), Some(-1.2), None, Some(0.0)] {
                let mut style = TextStyle::regular(Pt(12.0));
                style.tracking = tracking.map(Length::pt);
                let extra = (tracking.unwrap_or(0.0) / 12.0 * 1000.0).round() as i32;
                let glyphs = vec![
                    ShapedGlyph {
                        glyph_id: 1,
                        x_advance: 500 + extra,
                        x_offset: 0,
                        y_offset: 0,
                        cluster: 0,
                        char_code: 'A',
                    },
                    ShapedGlyph {
                        glyph_id: 1,
                        x_advance: 500,
                        x_offset: 0,
                        y_offset: 0,
                        cluster: 1,
                        char_code: 'A',
                    },
                ];
                test(scenario, &style, glyphs);
            }
        }
    }

    #[test]
    fn shaped_tracking_is_applied_once_and_clears_inherited_tc() {
        tracking_controls(|scenario, style, glyphs| {
            for verbose in [true, false] {
                for defer in [false, true] {
                    let mut ops = String::from("7 Tc\n");
                    if verbose {
                        emit_shaped_pdf_verbose_inner(
                            &mut ops, 10.0, 20.0, &glyphs, "AA", style, scenario, 1000,
                            !defer, None, false,
                        );
                    } else {
                        emit_shaped_pdf_inner(
                            &mut ops, 10.0, 20.0, &glyphs, "AA", style, scenario, 1000,
                            !defer, None, false,
                        );
                    }
                    let positions = rendered_positions(&ops, style.size.0);
                    assert_eq!(positions.len(), 2);
                    assert!(positions[0].abs() < 1e-8);
                    assert!(
                        (positions[1]
                            - glyphs[0].x_advance as f64 / 1000.0 * style.size.0)
                            .abs()
                            < 1e-8,
                        "{ops}"
                    );
                }
            }
        });
    }

    #[test]
    fn shaped_tracking_coalesced_run_preserves_absolute_boundary() {
        use crate::export::StreamMode;
        use typst_core::entities::layout_types::{FrameItem, Point, Pt};
        tracking_controls(|scenario, style, glyphs| {
            // Reconstruct the enum with borrowed fields; no font resources needed.
            let copied = match scenario {
                FontScenario::Cidfont {
                    char_to_gid,
                    glyph_mapping,
                    glyph_to_nominal,
                    bitmap,
                } => FontScenario::Cidfont {
                    char_to_gid,
                    glyph_mapping,
                    glyph_to_nominal,
                    bitmap: *bitmap,
                },
                FontScenario::Multifont {
                    fonts,
                    per_font_char_to_gid,
                    per_font_glyph_mapping,
                    per_font_glyph_to_nominal,
                    per_font_glyph_reverse,
                    per_font_bitmap,
                    ..
                } => FontScenario::Multifont {
                    fonts,
                    per_font_char_to_gid,
                    per_font_glyph_mapping,
                    per_font_glyph_to_nominal,
                    per_font_glyph_reverse,
                    per_font_bitmap,
                    bindings: None,
                },
                FontScenario::Type1 => unreachable!(),
            };
            let ptrs = HashMap::new();
            let pats = HashMap::new();
            let mut ctx = PageContext::type1(&ptrs, &[], &pats, &[], StreamMode::Verbose);
            ctx.font_scenario = copied;
            let run: Vec<_> = [10.0, 46.0]
                .into_iter()
                .map(|x| FrameItem::TextShaped {
                    pos: Point { x: Pt(x), y: Pt(20.0) },
                    text: "AA".into(),
                    style: style.clone(),
                    glyphs: glyphs.clone(),
                    units_per_em: 1000,
                    bidi: None,
                    font: None,
                })
                .collect();
            let mut ops = String::from("7 Tc\n");
            emit_verbose_text_run(&mut ops, &run, 100.0, &ctx);
            let positions = rendered_positions(&ops, style.size.0);
            let step = glyphs[0].x_advance as f64 / 1000.0 * style.size.0;
            assert_eq!(positions.len(), 4);
            for (actual, expected) in positions.iter().zip([0.0, step, 36.0, 36.0 + step])
            {
                assert!((*actual - expected).abs() < 1e-8, "{ops}");
            }
        });
    }

    #[test]
    fn shaped_tracking_fix_retains_plain_text_and_type1_spacing() {
        tracking_controls(|scenario, style, glyphs| {
            let Some(tracking) = style.tracking else { return };
            let expected = tracking.resolve_pt(style.size.0);
            if expected == 0.0 {
                return;
            }
            for scenario in [scenario, &FontScenario::Type1] {
                let mut ops = String::new();
                emit_text_pdf_verbose(&mut ops, 10.0, 20.0, "AA", style, scenario);
                assert!(ops.contains(&format!("{expected:.2} Tc")), "{ops}");
            }
            for verbose in [false, true] {
                let mut ops = String::new();
                if verbose {
                    emit_shaped_pdf_verbose(
                        &mut ops,
                        10.0,
                        20.0,
                        &glyphs,
                        "AA",
                        style,
                        &FontScenario::Type1,
                        1000,
                        None,
                    );
                } else {
                    emit_shaped_pdf(
                        &mut ops,
                        10.0,
                        20.0,
                        &glyphs,
                        "AA",
                        style,
                        &FontScenario::Type1,
                        1000,
                        None,
                    );
                }
                assert!(ops.contains(&format!("{expected:.2} Tc")), "{ops}");
            }
        });
    }

    #[test]
    fn fractional_tracking_pdf_keeps_exact_cluster_positions() {
        use typst_core::entities::layout_types::{Length, Pt, TextStyle};
        let map = HashMap::new();
        let nominal = HashMap::from([(1, 500)]);
        let chars = HashMap::from([('A', 1)]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &map,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        for t in [0.2, -0.2, 0.333] {
            let mut style = TextStyle::regular(Pt(12.0));
            style.tracking = Some(Length::pt(t));
            let q = (t / 12.0 * 1000.0).round() as i32;
            let glyphs: Vec<_> = [0, 0, 1]
                .into_iter()
                .enumerate()
                .map(|(i, cluster)| ShapedGlyph {
                    glyph_id: 1,
                    x_advance: 500 + if i == 1 { q } else { 0 },
                    x_offset: 0,
                    y_offset: 0,
                    cluster,
                    char_code: 'A',
                })
                .collect();
            for verbose in [false, true] {
                let mut ops = String::new();
                if verbose {
                    emit_shaped_pdf_verbose(
                        &mut ops, 0.0, 0.0, &glyphs, "AAA", &style, &scenario, 1000, None,
                    );
                } else {
                    emit_shaped_pdf(
                        &mut ops, 0.0, 0.0, &glyphs, "AAA", &style, &scenario, 1000, None,
                    );
                }
                let xs = rendered_positions(&ops, 12.0);
                for (x, want) in xs.into_iter().zip([0.0, 6.0, 12.0 + t]) {
                    assert!((x - want).abs() < 1e-6, "{ops}");
                }
            }
            use crate::export::StreamMode;
            use typst_core::entities::layout_types::{FrameItem, Point};
            let ptrs = HashMap::new();
            let pats = HashMap::new();
            let mut ctx = PageContext::type1(&ptrs, &[], &pats, &[], StreamMode::Verbose);
            ctx.font_scenario = FontScenario::Cidfont {
                char_to_gid: &chars,
                glyph_mapping: &map,
                glyph_to_nominal: &nominal,
                bitmap: None,
            };
            let run: Vec<_> = [0.0, 30.123]
                .into_iter()
                .map(|x| FrameItem::TextShaped {
                    pos: Point { x: Pt(x), y: Pt(0.0) },
                    glyphs: glyphs.clone(),
                    style: style.clone(),
                    text: "AAA".into(),
                    units_per_em: 1000,
                    bidi: None,
                    font: None,
                })
                .collect();
            let mut ops = String::new();
            emit_verbose_text_run(&mut ops, &run, 100.0, &ctx);
            for (x, want) in rendered_positions(&ops, 12.0).into_iter().zip([
                0.0,
                6.0,
                12.0 + t,
                30.123,
                36.123,
                42.123 + t,
            ]) {
                assert!((x - want).abs() < 1e-6, "{ops}");
            }
        }
    }
}

#[cfg(test)]
mod r8_font_size_tests {
    use super::*;
    use typst_core::entities::layout_types::{Pt, TextStyle};
    #[test]
    fn r8_emitted_font_size_retains_fractional_note_and_script_metrics() {
        for size in [7.69677734375, 9.35, 6.5422607421875, 11.0] {
            let style = TextStyle::regular(Pt(size));
            let mut ops = String::new();
            emit_text_pdf(&mut ops, 0.0, 0.0, "1", &style, &FontScenario::Type1);
            let prefix = ops.split(" Tf").next().unwrap();
            let actual: f64 = prefix.split_whitespace().last().unwrap().parse().unwrap();
            assert!((actual - size).abs() <= 0.000005, "{size}: {ops}");
            if size == 11.0 {
                assert!(ops.contains("/F1 11.0 Tf"));
            }
        }
    }
}

#[cfg(test)]
mod readiness_unicode_tests {
    use super::*;
    use typst_core::entities::layout_types::{Pt, ShapedGlyph, TextStyle};

    #[test]
    fn readiness_unicode_zero_width_scaling_requires_empty_space_face() {
        let bytes = include_bytes!("../../../fixtures/fonts/DejaVuSans.ttf");
        let font = typst_core::entities::world_types::Font::from_data(bytes.to_vec());
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let space = face.glyph_index(' ').unwrap();
        let ink = face.glyph_index('A').unwrap();
        assert!(face.glyph_bounding_box(space).is_none());
        let chars = HashMap::new();
        let mapping = HashMap::new();
        let nominal = HashMap::from([
            (space.0, i32::from(face.glyph_hor_advance(space).unwrap())),
            (ink.0, i32::from(face.glyph_hor_advance(ink).unwrap())),
        ]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &mapping,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        let style = TextStyle::regular(Pt(12.0));
        for (gid, advance, supplied, expected) in [
            (space.0, 0, true, true),
            (ink.0, 0, true, false),
            (space.0, 10, true, false),
            (space.0, 0, false, false),
        ] {
            let glyphs = [ShapedGlyph {
                glyph_id: gid,
                x_advance: advance,
                x_offset: 0,
                y_offset: 0,
                cluster: 0,
                char_code: '\u{2060}',
            }];
            let mut ops = String::new();
            emit_shaped_pdf_verbose(
                &mut ops,
                10.0,
                20.0,
                &glyphs,
                "\u{2060}",
                &style,
                &scenario,
                face.units_per_em(),
                supplied.then_some(&font),
            );
            assert_eq!(
                ops.contains("0 Tz\n"),
                expected,
                "only a certified zero-advance empty space alias is scaled: {ops}"
            );
        }
    }

    #[test]
    fn readiness_unicode_joiner_occurrence_has_balanced_actualtext() {
        let chars = HashMap::new();
        let mapping = HashMap::new();
        let nominal = HashMap::from([(3, 250)]);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &mapping,
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        let glyphs = vec![ShapedGlyph {
            glyph_id: 3,
            x_advance: 0,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: '\u{2060}',
        }];
        let style = TextStyle::regular(Pt(11.0));
        for verbose in [false, true] {
            for deferred in [false, true] {
                let mut ops = String::new();
                let pending = match (verbose, deferred) {
                    (false, true) => emit_shaped_pdf_deferred(
                        &mut ops, 10.0, 20.0, &glyphs, "\u{2060}", &style, &scenario,
                        1000, None,
                    ),
                    (true, true) => emit_shaped_pdf_verbose_deferred(
                        &mut ops, 10.0, 20.0, &glyphs, "\u{2060}", &style, &scenario,
                        1000, None,
                    ),
                    (false, false) => {
                        emit_shaped_pdf(
                            &mut ops, 10.0, 20.0, &glyphs, "\u{2060}", &style, &scenario,
                            1000, None,
                        );
                        false
                    }
                    (true, false) => {
                        emit_shaped_pdf_verbose(
                            &mut ops, 10.0, 20.0, &glyphs, "\u{2060}", &style, &scenario,
                            1000, None,
                        );
                        false
                    }
                };
                if pending {
                    ops.push_str("Q\n");
                }
                assert_eq!(ops.matches("/ActualText <FEFF2060>").count(), 1, "{ops}");
                if let Some(restore) = ops.rfind("Q\n") {
                    assert!(ops.find("EMC\n").unwrap() < restore, "{ops}");
                }
                assert_eq!(
                    ops.lines().filter(|line| *line == "q").count(),
                    ops.lines().filter(|line| *line == "Q").count()
                );
            }
        }
    }
}
