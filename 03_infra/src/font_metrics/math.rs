//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/math.md
//! @prompt-hash 22408f23
//! @layer L3
//! @updated 2026-10-10

use std::collections::HashMap;
use ttf_parser::Face;
use typst_core::entities::glyph_variants::{
    GlyphAssembly, GlyphPart, GlyphVariant, GlyphVariants, MathGlyphKern, MathKernRecord,
    MathKernTable,
};
use typst_core::entities::layout_types::{MathSize, TextStyle};
use typst_core::entities::math_constants::MathConstants;

/// Extrai variantes verticais de um glifo directamente a partir da face.
pub(super) fn extract_variants(face: &Face<'_>, c: char) -> GlyphVariants {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphVariants::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphVariants::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphVariants::default(),
    };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphVariants::default(),
    };
    GlyphVariants {
        variants: construction
            .variants
            .into_iter()
            .map(|r| GlyphVariant {
                glyph_id: r.variant_glyph.0,
                advance: r.advance_measurement as f64,
                // **P917** — avanço horizontal NATIVO do glifo (hmtx),
                // nunca `advance_measurement` (eixo de esticamento, aqui
                // altura). Ver `entities/glyph_variants.md` §P917.
                hor_advance: face.glyph_hor_advance(r.variant_glyph).unwrap_or(0) as f64,
                top_accent_attach: None,
            })
            .collect(),
    }
}

/// Extrai a assembly vertical de um glifo directamente a partir da face.
pub(super) fn extract_assembly(face: &Face<'_>, c: char) -> GlyphAssembly {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphAssembly::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphAssembly::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphAssembly::default(),
    };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphAssembly::default(),
    };
    let ttf_assembly = match construction.assembly {
        Some(a) => a,
        None => return GlyphAssembly::default(),
    };
    GlyphAssembly {
        // **P945** — `minConnectorOverlap` da tabela MATH, lido do mesmo
        // sítio onde o vanilla o lê (`glyph.rs:542-548`). Ver
        // `infra/font_metrics.md` §P945.
        min_overlap: variants_table.min_connector_overlap,
        parts: ttf_assembly
            .parts
            .into_iter()
            .map(|p| GlyphPart {
                glyph_id: p.glyph_id.0,
                start_connector: p.start_connector_length,
                end_connector: p.end_connector_length,
                full_advance: p.full_advance,
                is_extender: p.part_flags.extender(),
                // **P917** — avanço horizontal nativo da peça (hmtx), nunca
                // `full_advance` (eixo de empilhamento). Ver `entities/
                // glyph_variants.md` §P917.
                hor_advance: face.glyph_hor_advance(p.glyph_id).unwrap_or(0) as f64,
            })
            .collect(),
    }
}

/// **P906** — extrai variantes horizontais de um glifo directamente a partir
/// da face. Espelha `extract_variants` acima — única diferença:
/// `variants_table.horizontal_constructions` em vez de `.vertical_
/// constructions` (campo simétrico da mesma struct `ttf_parser`) — ver
/// `infra/font_metrics.md` §P906.
pub(super) fn extract_variants_horizontal(face: &Face<'_>, c: char) -> GlyphVariants {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphVariants::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphVariants::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphVariants::default(),
    };
    let construction = match variants_table.horizontal_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphVariants::default(),
    };
    GlyphVariants {
        variants: construction
            .variants
            .into_iter()
            .map(|r| GlyphVariant {
                glyph_id: r.variant_glyph.0,
                advance: r.advance_measurement as f64,
                // **P917** — uniformidade com o extractor vertical: mesmo
                // para construções horizontais (onde `advance` e
                // `hor_advance` tendem a coincidir), a fonte de verdade
                // para posicionamento é sempre o hmtx nativo, nunca a
                // medida do eixo de esticamento.
                hor_advance: face.glyph_hor_advance(r.variant_glyph).unwrap_or(0) as f64,
                top_accent_attach: face
                    .tables()
                    .math
                    .and_then(|m| m.glyph_info)
                    .and_then(|gi| gi.top_accent_attachments)
                    .and_then(|ta| ta.get(r.variant_glyph))
                    .map(|v| v.value as f64)
                    .or_else(|| {
                        let advance = face.glyph_hor_advance(r.variant_glyph)? as f64;
                        let ic = face
                            .tables()
                            .math
                            .and_then(|m| m.glyph_info)
                            .and_then(|gi| gi.italic_corrections)
                            .and_then(|table| table.get(r.variant_glyph))
                            .map(|v| v.value as f64)
                            .unwrap_or(0.0);
                        Some((advance + ic) / 2.0)
                    }),
            })
            .collect(),
    }
}

/// **P906** — extrai a assembly horizontal de um glifo directamente a partir
/// da face. Espelha `extract_assembly` acima, lendo `.horizontal_
/// constructions` em vez de `.vertical_constructions` — ver
/// `infra/font_metrics.md` §P906.
pub(super) fn extract_assembly_horizontal(face: &Face<'_>, c: char) -> GlyphAssembly {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphAssembly::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphAssembly::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphAssembly::default(),
    };
    let construction = match variants_table.horizontal_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphAssembly::default(),
    };
    let ttf_assembly = match construction.assembly {
        Some(a) => a,
        None => return GlyphAssembly::default(),
    };
    GlyphAssembly {
        // **P945** — mesmo `minConnectorOverlap` do extractor vertical
        // (campo da tabela `variants`, partilhado pelos dois eixos).
        min_overlap: variants_table.min_connector_overlap,
        parts: ttf_assembly
            .parts
            .into_iter()
            .map(|p| GlyphPart {
                glyph_id: p.glyph_id.0,
                start_connector: p.start_connector_length,
                end_connector: p.end_connector_length,
                full_advance: p.full_advance,
                is_extender: p.part_flags.extender(),
                // **P917** — mesma uniformidade de `extract_variants_horizontal` acima.
                hor_advance: face.glyph_hor_advance(p.glyph_id).unwrap_or(0) as f64,
            })
            .collect(),
    }
}

/// Constrói o dicionário reverso preemptivo: glyph_id → char base.
///
/// Itera sobre os caracteres matemáticos extensíveis conhecidos, extrai
/// as variantes e as peças de assembly da tabela MATH, e guarda o
/// mapeamento inverso. Extensores mapeiam para `|` (barra vertical).
/// Usa `or_insert` para não sobrescrever se a fonte partilhar a peça
/// entre múltiplos caracteres base.
pub(crate) fn build_math_glyph_reverse_map(face: &Face<'_>) -> HashMap<u16, char> {
    // No construction can contribute glyphs without MATH variants.
    // Avoid building the intermediate reverse cmap that would be discarded.
    if face.tables().math.and_then(|math| math.variants).is_none() {
        return HashMap::new();
    }
    const STRETCHY_BASES: &[char] = &[
        '(', ')', '[', ']', '{', '}', '|', '‖', '∥', '⌊', '⌋', '⌈', '⌉', '⟨', '⟩', '〈',
        '〉', '√', '/', '\\', '↑', '↓', '↕', '⇑', '⇓', '⇕',
        // **P952** — operadores grandes (`MathClass::Large`) cujas variantes
        // de Display (`layout_large_operator_display`, `_comum.md` §P952)
        // precisam de entrar no subset/ToUnicode como qualquer outra
        // variante — sem isto, o `∑`/`∫` display emitidos como
        // `FrameItem::Glyph` caíam fora do subset embutido (glifo invisível
        // no PDF real, achado da revalidação end-to-end de P952).
        '∑', '∏', '∐', '⋃', '⋂', '⨄', '⨅', '⨆', '∫', '∬', '∭', '∮', '∯', '∰', '⨁', '⨂',
        '⨀', '⋀', '⋁',
        // Fraction slash variants retain their actual textual identity.
        '⁄',
    ];

    // **P906** — chars extensíveis no eixo horizontal (chaves/colchetes de
    // underbrace/overbrace/underbracket/overbracket + acentos largos
    // hat/tilde). Sem isto, os glyph_ids das suas variantes/assembly nunca
    // entravam neste mapa — nem no subsetting (P45/DEBT-9, `builder.rs:988`
    // consome exactamente este mapa) nem na selecção de `/Fn` em
    // `emit_glyph_pdf` (que passou a reaproveitá-lo neste mesmo passo) —
    // glifos de esticamento horizontal ficavam fora do subset embutido ou
    // referenciados pela fonte errada. Extensor mapeia para `_` (linha
    // horizontal), análogo ao `|` já usado para extensores verticais.
    const STRETCHY_BASES_HORIZONTAL: &[char] = &[
        '⏟', '⏞', '⎵', '⎴', '⏝', '⏜', '\u{0302}', '\u{0303}', '\u{0305}', '\u{0306}',
        '\u{030c}', '\u{20d6}', '\u{20d7}', '\u{20e1}', '\u{20d0}', '\u{20d1}',
    ];

    // **P946** — mapa inverso da cmap da face (glyph_id → char), um passe:
    // peças de assembly com codepoint próprio (ex.: `⎧`/`⎨`/`⎩`/`⎪`,
    // `⎛`/`⎜`/`⎝`) passam a mapear para esse codepoint real — extracção
    // semanticamente mais rica (decisão do dono, divergência deliberada:
    // o vanilla agrupa a assembly num único char base no ToUnicode —
    // medição registada em `infra/font_metrics.md` §P946). Glifos não
    // codificados (variantes `.vN`, peças sem codepoint próprio, ex.:
    // `braceleft.ex`) caem no fallback `base_char`/`|`/`_` (comportamento
    // anterior).
    let mut cmap_reverse: HashMap<u16, char> = HashMap::new();
    if let Some(cmap) = face.tables().cmap {
        for subtable in cmap.subtables {
            subtable.codepoints(|cp| {
                let Some(ch) = char::from_u32(cp) else { return };
                let Some(gid) = subtable.glyph_index(cp) else { return };
                cmap_reverse.entry(gid.0).or_insert(ch);
            });
        }
    }
    let real_or = |glyph_id: u16, fallback: char| -> char {
        cmap_reverse.get(&glyph_id).copied().unwrap_or(fallback)
    };

    let mut map = HashMap::new();
    for &base_char in STRETCHY_BASES {
        for v in extract_variants(face, base_char).variants {
            map.entry(v.glyph_id)
                .or_insert_with(|| real_or(v.glyph_id, base_char));
        }
        for part in extract_assembly(face, base_char).parts {
            let mapped = if part.is_extender { '|' } else { base_char };
            map.entry(part.glyph_id)
                .or_insert_with(|| real_or(part.glyph_id, mapped));
        }
    }
    for &base_char in STRETCHY_BASES_HORIZONTAL {
        for v in extract_variants_horizontal(face, base_char).variants {
            map.entry(v.glyph_id)
                .or_insert_with(|| real_or(v.glyph_id, base_char));
        }
        for part in extract_assembly_horizontal(face, base_char).parts {
            let mapped = if part.is_extender { '_' } else { base_char };
            map.entry(part.glyph_id)
                .or_insert_with(|| real_or(part.glyph_id, mapped));
        }
    }
    map
}

/// **P1129** — replica o predicado `num` de vanilla `resolve_text`
/// (`ir/resolve.rs:284-292`): texto puramente numérico (dígitos ASCII +
/// no máximo 1 ponto decimal, pelo menos 1 dígito) vira `NumberItem` —
/// CADA dígito é um `GlyphFragment` atómico (`layout_number` chama
/// `GlyphFragment::synthetic` por carácter, forçando o script `math`
/// individualmente), logo `ssty` aplica-se a cada um. Texto de 1
/// carácter é sempre atómico (`GlyphItem`/`MathIdent`/`MathText`).
/// Qualquer outro texto multi-carácter (`TextItem` — strings citadas,
/// ex. legendas de `underbrace`/`overbrace`) nunca é elegível: vanilla
/// shape-o via `layout_inline`, que não força o script `math`.
pub(crate) fn ssty_eligible_text(text: &str) -> bool {
    let count = text.chars().count();
    if count == 1 {
        return true;
    }
    let mut decimal_count = 0usize;
    let all_digit_or_dot = text.chars().all(|c| {
        if c == '.' {
            decimal_count += 1;
        }
        c.is_ascii_digit() || c == '.'
    });
    all_digit_or_dot && decimal_count != count && decimal_count <= 1
}

pub(super) fn is_variation_selector(c: char) -> bool {
    matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
}

/// **P977** — nível ssty do estilo em contexto math: `Some(1)` para
/// `MathSize::Script`, `Some(2)` para `ScriptScript`, `None` nos restantes
/// casos (fora de math, Display, Text). Vanilla
/// `text/mod.rs:1457-1460`. Ver `infra/font_metrics.md` §P977.
///
/// **P1129** — só se `ssty_eligible` (ver `ssty_eligible_text`): vanilla
/// só força o script OpenType `math` (onde `ssty` está registada na
/// GSUB de NewCMMath) nos caminhos de glifo matemático atómico
/// (`GlyphItem`/`math/shaping.rs::shape_text`) e de número dígito-a-
/// dígito (`NumberItem`/`layout_number`); texto multi-carácter não-
/// numérico em math (`TextItem`/`resolve_text` — strings, legendas de
/// `underbrace`/`overbrace`) usa o shaper de parágrafo comum, sem forçar
/// o script, deixando o pedido genérico de `ssty` inerte. Ver correcção
/// de escopo em `infra/font_metrics.md` §P977.
pub(super) fn ssty_level_of(style: &TextStyle, ssty_eligible: bool) -> Option<u8> {
    if !style.math || style.math_text_item || !ssty_eligible {
        return None;
    }
    match style.math_size {
        MathSize::Script => Some(1),
        MathSize::ScriptScript => Some(2),
        MathSize::Display | MathSize::Text => None,
    }
}

/// **P977** — variante ssty (`.st`/`.sts`) de um glifo, lida da GSUB da
/// face (AlternateSubst da feature `ssty`: `alternates[level-1]`).
/// `None` se a face não tiver a feature/o glifo não for coberto — o
/// chamador fica com o glifo base nesse caso.
pub(super) fn ssty_substitute(
    face: &ttf_parser::Face,
    gid: ttf_parser::GlyphId,
    level: u8,
) -> Option<ttf_parser::GlyphId> {
    let gsub = face.tables().gsub?;
    let feature = gsub.features.find(ttf_parser::Tag::from_bytes(b"ssty"))?;
    for lookup_index in feature.lookup_indices {
        let Some(lookup) = gsub.lookups.get(lookup_index) else { continue };
        for subtable in
            lookup.subtables.into_iter::<ttf_parser::gsub::SubstitutionSubtable>()
        {
            let ttf_parser::gsub::SubstitutionSubtable::Alternate(alt) = subtable else {
                continue;
            };
            let Some(cov_idx) = alt.coverage.get(gid) else { continue };
            let Some(set) = alt.alternate_sets.get(cov_idx) else { continue };
            if let Some(sub) = set.alternates.get(u16::from(level) - 1) {
                return Some(sub);
            }
        }
    }
    None
}

/// Lê as constantes da tabela MATH de `face`, ou `MathConstants::fallback()`
/// se a face não tiver tabela MATH/constants.
///
/// **P893** — extraída de `FontBookMetrics::math_constants` (mesmo padrão de
/// `math_kern_from_face`, P891) para ser partilhada com
/// `FallbackFontMetrics::math_constants`, que resolve a face candidata (via
/// `resolve_primary_with_math_fallback`, primeira com tabela MATH) antes de
/// chamar esta função, em vez de ter uma única face fixa.
pub(super) fn math_constants_from_face(face: &Face<'_>, upem: f64) -> MathConstants {
    match face.tables().math {
        Some(math_table) => match math_table.constants {
            Some(c) => MathConstants {
                upem,
                fraction_rule_thickness: c.fraction_rule_thickness().value as f64,
                fraction_num_gap: c.fraction_numerator_gap_min().value as f64,
                fraction_denom_gap: c.fraction_denominator_gap_min().value as f64,
                // P920 — mesmo mecanismo dos outros campos; métodos já
                // existentes em ttf_parser 0.25 (confirmado antes de propor
                // o campo, `ttf-parser-0.25.1/src/tables/math.rs:378,390`).
                fraction_numerator_shift_up: c.fraction_numerator_shift_up().value as f64,
                fraction_denominator_shift_down: c.fraction_denominator_shift_down().value
                    as f64,
                skewed_fraction_vertical_gap: c.skewed_fraction_vertical_gap().value
                    as f64,
                skewed_fraction_horizontal_gap: c.skewed_fraction_horizontal_gap().value
                    as f64,
                // P990 — variantes Display (mesmo mecanismo; métodos de
                // ttf_parser 0.25, `tables/math.rs`).
                fraction_numerator_display_style_shift_up: c
                    .fraction_numerator_display_style_shift_up()
                    .value
                    as f64,
                fraction_denominator_display_style_shift_down: c
                    .fraction_denominator_display_style_shift_down()
                    .value
                    as f64,
                fraction_num_display_style_gap_min: c
                    .fraction_num_display_style_gap_min()
                    .value as f64,
                fraction_denom_display_style_gap_min: c
                    .fraction_denom_display_style_gap_min()
                    .value as f64,
                // P1132f — constantes próprias de FractionItem sem barra.
                stack_top_shift_up: c.stack_top_shift_up().value as f64,
                stack_top_display_style_shift_up: c
                    .stack_top_display_style_shift_up()
                    .value as f64,
                stack_bottom_shift_down: c.stack_bottom_shift_down().value as f64,
                stack_bottom_display_style_shift_down: c
                    .stack_bottom_display_style_shift_down()
                    .value as f64,
                stack_gap_min: c.stack_gap_min().value as f64,
                stack_display_style_gap_min: c.stack_display_style_gap_min().value as f64,
                superscript_shift_up: c.superscript_shift_up().value as f64,
                // P915 — mesmo mecanismo dos outros 14 campos; método já
                // existia em ttf_parser 0.25, só nunca tinha sido lido.
                superscript_shift_up_cramped: c.superscript_shift_up_cramped().value
                    as f64,
                subscript_shift_down: c.subscript_shift_down().value as f64,
                superscript_bottom_min: c.superscript_bottom_min().value as f64,
                superscript_bottom_max_with_subscript: c
                    .superscript_bottom_max_with_subscript()
                    .value as f64,
                superscript_baseline_drop_max: c.superscript_baseline_drop_max().value
                    as f64,
                sub_superscript_gap_min: c.sub_superscript_gap_min().value as f64,
                subscript_top_max: c.subscript_top_max().value as f64,
                subscript_baseline_drop_min: c.subscript_baseline_drop_min().value as f64,
                overbar_vertical_gap: c.overbar_vertical_gap().value as f64,
                overbar_rule_thickness: c.overbar_rule_thickness().value as f64,
                overbar_extra_ascender: c.overbar_extra_ascender().value as f64,
                underbar_vertical_gap: c.underbar_vertical_gap().value as f64,
                underbar_rule_thickness: c.underbar_rule_thickness().value as f64,
                underbar_extra_descender: c.underbar_extra_descender().value as f64,
                radical_vertical_gap: c.radical_vertical_gap().value as f64,
                // P974 — MathValueRecord, mesmo padrão dos vizinhos
                // (`infra/font_metrics.md` §P974).
                radical_display_style_vertical_gap: c
                    .radical_display_style_vertical_gap()
                    .value as f64,
                radical_rule_thickness: c.radical_rule_thickness().value as f64,
                // P970 — os três primeiros são `MathValueRecord` (`.value`),
                // mesmo padrão dos vizinhos; o percentual é `i16` cru em
                // ttf-parser 0.25 (`radical_degree_bottom_raise_percent()`),
                // lido ÷100 como `script_percent_scale_down`.
                // (`infra/font_metrics.md` §P970.)
                radical_kern_before_degree: c.radical_kern_before_degree().value as f64,
                radical_kern_after_degree: c.radical_kern_after_degree().value as f64,
                radical_degree_bottom_raise_percent: c
                    .radical_degree_bottom_raise_percent()
                    as f64
                    / 100.0,
                radical_extra_ascender: c.radical_extra_ascender().value as f64,
                axis_height: c.axis_height().value as f64,
                script_percent_scale_down: c.script_percent_scale_down() as f64 / 100.0,
                script_script_percent_scale_down: c.script_script_percent_scale_down()
                    as f64
                    / 100.0,
                upper_limit_gap_min: c.upper_limit_gap_min().value as f64,
                lower_limit_gap_min: c.lower_limit_gap_min().value as f64,
                // P959 — `MathValueRecord.value` (i16), mesmo padrão dos
                // vizinhos (`infra/font_metrics.md` §P959).
                upper_limit_baseline_rise_min: c.upper_limit_baseline_rise_min().value
                    as f64,
                lower_limit_baseline_drop_min: c.lower_limit_baseline_drop_min().value
                    as f64,
                math_leading: c.math_leading().value as f64,
                // P922 — métodos já existem em ttf_parser 0.25.
                accent_base_height: c.accent_base_height().value as f64,
                flattened_accent_base_height: c.flattened_accent_base_height().value
                    as f64,
                space_after_script: c.space_after_script().value as f64,
                // P952 — DisplayOperatorMinHeight. **Desvio do L0** (forma,
                // não substância): o L0 diz `c.display_operator_min_height()
                // .value`, mas em ttf_parser 0.25 este método devolve `u16`
                // directamente (não `MathLeadingValue` — é um dos poucos
                // campos u16 crus da tabela, `ttf-parser-0.25.1/src/tables/
                // math.rs:196`; o vanilla lê-o da mesma forma,
                // `typst-library/src/text/font/metrics.rs:228`).
                display_operator_min_height: c.display_operator_min_height() as f64,
            },
            None => MathConstants::fallback(),
        },
        None => MathConstants::fallback(),
    }
}

/// Lê o kern matemático por quadrante para `c` na tabela MATH de `face`.
///
/// **P891** — extraída de `FontBookMetrics::math_kern` para ser partilhada
/// com `FallbackFontMetrics::math_kern`, que resolve a face candidata (via
/// `resolve_primary_with_math_fallback` + `covering`) antes de chamar esta
/// função, em vez de ter uma única face fixa.
pub(super) fn math_kern_from_face(face: &Face<'_>, c: char) -> MathGlyphKern {
    let Some(glyph_id) = face.glyph_index(c) else { return MathGlyphKern::default() };
    let Some(math) = face.tables().math else { return MathGlyphKern::default() };
    let Some(glyph_info) = math.glyph_info else { return MathGlyphKern::default() };
    let Some(kern_infos) = glyph_info.kern_infos else { return MathGlyphKern::default() };
    let Some(kern_record) = kern_infos.get(glyph_id) else {
        return MathGlyphKern::default();
    };

    // Lê uma tabela Kern do ttf-parser em MathKernTable de L1.
    // A tabela tem `count` alturas e `count+1` valores de kern:
    //   kern[0] aplica-se até height[0], …, kern[count] aplica-se
    //   a todas as alturas acima de height[count-1].
    fn read_kern(kern: Option<ttf_parser::math::Kern>) -> MathKernTable {
        let kern = match kern {
            Some(k) => k,
            None => return MathKernTable::default(),
        };
        let count = kern.count() as usize;
        let mut records = Vec::with_capacity(count + 1);
        for i in 0..count {
            let height = kern.height(i as u16).map(|v| v.value as f64);
            let kv = kern.kern(i as u16).map(|v| v.value as f64).unwrap_or(0.0);
            records.push(MathKernRecord { correction_height: height, kern_value: kv });
        }
        // Último valor de kern (sem correction_height associado)
        if let Some(kv) = kern.kern(count as u16).map(|v| v.value as f64) {
            records.push(MathKernRecord { correction_height: None, kern_value: kv });
        }
        MathKernTable { records }
    }

    MathGlyphKern {
        top_right: read_kern(kern_record.top_right),
        top_left: read_kern(kern_record.top_left),
        bottom_right: read_kern(kern_record.bottom_right),
        bottom_left: read_kern(kern_record.bottom_left),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{Pt, TextStyle};

    #[test]
    fn reverse_map_without_math_is_empty() {
        let mut checked = 0;
        for data in typst_assets::fonts() {
            let face = Face::parse(data, 0).unwrap();
            if face.tables().math.is_none() {
                assert!(build_math_glyph_reverse_map(&face).is_empty());
                checked += 1;
            }
        }
        assert!(checked > 0, "embedded non-MATH coverage is required");
    }

    #[test]
    fn reverse_map_without_math_variants_is_empty() {
        let data = typst_assets::fonts()
            .find(|data| {
                Face::parse(data, 0)
                    .unwrap()
                    .tables()
                    .math
                    .and_then(|m| m.variants)
                    .is_some()
            })
            .expect("embedded MATH variants font is required");
        let face = Face::parse(data, 0).unwrap();
        let math = face.raw_face().table(ttf_parser::Tag::from_bytes(b"MATH")).unwrap();
        let offset = math.as_ptr() as usize - data.as_ptr() as usize;
        let mut bytes = data.to_vec();
        // OpenType MATH header: variants Offset16 follows constants and glyph info.
        bytes[offset + 8..offset + 10].copy_from_slice(&[0, 0]);
        let face = Face::parse(&bytes, 0).unwrap();
        assert!(face.tables().math.is_some());
        assert!(face.tables().math.unwrap().variants.is_none());
        assert!(build_math_glyph_reverse_map(&face).is_empty());
    }

    /// **P946** — `build_math_glyph_reverse_map` prefere o **codepoint real**
    /// da cmap da face para peças de assembly (ex.: `⎩`/`⎪`/`⎨`/`⎧` para a
    /// chave) — decisão do dono: extracção semanticamente mais rica, como
    /// divergência deliberada do vanilla (que agrupa a assembly num único
    /// char base no ToUnicode — medição em `infra/font_metrics.md` §P946).
    /// Glifos **não codificados** (variantes `.vN`, peças sem codepoint
    /// próprio como `braceleft.ex`) mantêm o fallback anterior
    /// (`base_char`/`|`/`_`).
    ///
    /// Usa a fonte `NewCMMath` real embutida (mesmo padrão de P891:
    /// localizada por cobertura, não por nome) — a cmap dela cobre
    /// U+23A7–U+23AA/U+239B–U+23A0 (verificado via fontTools em P946).
    #[test]
    fn p946_reverse_map_assembly_usa_codepoints_reais_da_cmap() {
        let data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                face.tables().math.and_then(|m| m.variants).is_some()
                    && face.glyph_index('\u{23A9}').is_some()
                    && face.glyph_index('\u{239C}').is_some()
            })
            .expect(
                "fonte embutida NewCMMath com variants + U+23A9/U+239C tem de existir",
            );
        let face = ttf_parser::Face::parse(data, 0).unwrap();
        let map = build_math_glyph_reverse_map(&face);

        // Assembly de '{' (ordem da fonte: fundo → topo):
        // [⎩ U+23A9, braceleft.ex (sem codepoint próprio → fallback '|'),
        //  ⎨ U+23A8, braceleft.ex ('|'), ⎧ U+23A7].
        let assembly = extract_assembly(&face, '{');
        assert!(!assembly.is_empty(), "NewCMMath tem assembly para '{{'");
        let chars: Vec<char> = assembly.parts.iter().map(|p| map[&p.glyph_id]).collect();
        assert_eq!(
            chars,
            vec!['\u{23A9}', '|', '\u{23A8}', '|', '\u{23A7}'],
            "peças com codepoint na cmap mapeiam para o codepoint real; \
             extensores sem codepoint próprio (braceleft.ex) caem no fallback '|'"
        );

        // Assembly de '(': [⎝ U+239D, ⎜ U+239C, ⎛ U+239B].
        let paren = extract_assembly(&face, '(');
        assert!(!paren.is_empty(), "NewCMMath tem assembly para '('");
        let paren_chars: Vec<char> =
            paren.parts.iter().map(|p| map[&p.glyph_id]).collect();
        assert_eq!(paren_chars, vec!['\u{239D}', '\u{239C}', '\u{239B}']);

        // Variantes (.vN) não são codificadas na cmap → fallback base_char.
        let variants = extract_variants(&face, '(');
        assert!(!variants.is_empty(), "NewCMMath tem variantes para '('");
        for v in &variants.variants {
            assert_eq!(
                map[&v.glyph_id], '(',
                "variante sem codepoint na cmap deve cair no base_char"
            );
        }

        // **P952** — variantes de operadores grandes (Display) também entram
        // no mapa (subset/ToUnicode), com fallback ao base_char.
        let sum_variants = extract_variants(&face, '∑');
        assert!(!sum_variants.is_empty(), "NewCMMath tem variantes para '∑'");
        for v in &sum_variants.variants {
            assert_eq!(
                map[&v.glyph_id], '∑',
                "variante de operador grande sem codepoint deve cair no base_char"
            );
        }
    }

    #[test]
    fn p1293_textitem_ssty_fallback_exclui_somente_proveniencia_textual() {
        let script_glyph = TextStyle {
            math: true,
            math_text_item: false,
            math_size: MathSize::Script,
            ..TextStyle::regular(Pt(11.0))
        };
        let script_textitem = TextStyle { math_text_item: true, ..script_glyph.clone() };
        let scriptscript_glyph = TextStyle {
            math_size: MathSize::ScriptScript,
            ..script_glyph.clone()
        };
        let scriptscript_textitem =
            TextStyle { math_text_item: true, ..scriptscript_glyph.clone() };

        assert!(script_textitem.math, "TextItem deve preservar o contexto math");
        assert_eq!(ssty_level_of(&script_textitem, true), None);
        assert_eq!(ssty_level_of(&scriptscript_textitem, true), None);
        assert_eq!(ssty_level_of(&script_glyph, true), Some(1));
        assert_eq!(ssty_level_of(&scriptscript_glyph, true), Some(2));
        assert_eq!(ssty_level_of(&script_glyph, false), None);
    }
}

#[cfg(test)]
mod fraction_slash_tests {
    use super::*;
    #[test]
    fn readiness_fraction_slash_maps_each_real_font_variant() {
        let data = include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf");
        let face = Face::parse(data, 0).expect("valid real MATH fixture");
        let variants = extract_variants(&face, '⁄');
        assert!(variants.variants.len() > 1, "real stretch constructions required");
        let map = build_math_glyph_reverse_map(&face);
        for glyph in variants.variants {
            assert_eq!(
                map.get(&glyph.glyph_id),
                Some(&'⁄'),
                "fraction slash glyph{}",
                glyph.glyph_id
            );
        }
    }
}

#[cfg(test)]
mod readiness_accent_reverse_tests {
    use super::*;
    #[test]
    fn readiness_accent_reverse_each_real_horizontal_variant_has_unicode() {
        let data = include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf");
        let face = Face::parse(data, 0).expect("valid real MATH fixture");
        let map = build_math_glyph_reverse_map(&face);
        for ch in [
            '\u{0305}', '\u{0306}', '\u{030c}', '\u{20d6}', '\u{20d7}', '\u{20e1}',
            '\u{20d0}', '\u{20d1}',
        ] {
            let variants = extract_variants_horizontal(&face, ch);
            assert!(variants.variants.len() > 1, "real horizontal variants for {ch:?}");
            for glyph in variants.variants {
                assert_eq!(
                    map.get(&glyph.glyph_id),
                    Some(&ch),
                    "{ch:?} glyph{}",
                    glyph.glyph_id
                );
            }
        }
    }
}
