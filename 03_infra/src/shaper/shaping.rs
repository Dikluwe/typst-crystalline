//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/shaping.md
//! @prompt-hash 52842474
//! @layer L3
//! @updated 2026-09-29

use super::bidi::bidi_runs;
use super::cache::{CachedRun, FaceCache, ShapeCache};
use super::font_selection::{primary_candidates, split_run_by_font, CandidateSet};
use crate::font_metrics::ssty_eligible_text;
use crate::font_variant::{axis_variations_for_text_style, text_style_to_font_variant};
use rustybuzz::{Direction, UnicodeBuffer};
use typst_core::contracts::world::World;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, Point, Pt, ShapedGlyph, TextStyle};

/// Nível OpenType `ssty` solicitado para um sub-run matemático.
///
/// TextItem preserva `style.math` para família, fallback e demais eixos de
/// shaping, mas segue a rota textual do vanilla e não seleciona `.st`/`.sts`.
pub(super) fn ssty_level_for_subrun(style: &TextStyle, text: &str) -> Option<u8> {
    if !style.math || style.math_text_item || !ssty_eligible_text(text) {
        return None;
    }
    match style.math_size {
        typst_core::entities::layout_types::MathSize::Script => Some(1),
        typst_core::entities::layout_types::MathSize::ScriptScript => Some(2),
        _ => None,
    }
}

/// **P591** — mede a largura de `text` já com shaping aplicado, sem gerar
/// `FrameItem`s. Usado pelo `FallbackFontMetrics::advance_shaped` para que o
/// Layouter decida quebras de linha com a largura real de scripts contextuais
/// (árabe, síriaco, etc.).
///
/// Retorna `None` se não conseguir resolver fonte ou se o texto for vazio.
pub(crate) fn shaped_width(
    world: &dyn World,
    text: &str,
    style: &TextStyle,
    face_cache: &mut FaceCache,
) -> Option<Pt> {
    if text.is_empty() {
        return Some(Pt(0.0));
    }

    let font_list = style.font.as_ref()?;

    // P836 — eixos derivados + explícitos (`style.variations`), explícitos
    // vencem por tag.
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    let primary = primary_candidates(world, font_list, &variant, face_cache, style.math);

    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.book().infos().get(cand.slot_idx).cloned());
    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);

    // P845 — runs agrupados por parágrafo bidi (texto com `\n` interno).
    let paragraphs = bidi_runs(text);
    if paragraphs.iter().all(|p| p.is_empty()) {
        return Some(Pt(0.0));
    }

    // Largura de texto multilinha = max das larguras de linha (não a soma):
    // cada parágrafo é uma linha visível independente.
    let mut max_width = 0.0f64;

    for runs in &paragraphs {
        let mut line_width = 0.0f64;
        for run in runs {
            for subrun in split_run_by_font(run, &mut candidates) {
                let candidate = candidates.get(subrun.candidate_idx)?;
                let font = world.font(candidate.slot_idx)?;
                let mut rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;

                if !axis_vars.is_empty() {
                    rb_face.set_variations(&axis_vars);
                }

                let mut buffer = UnicodeBuffer::new();
                buffer.push_str(&subrun.text);
                super::text_identity::configure_authored_shy_buffer(&mut buffer, &subrun.text);
                if let Some(lang) = style.lang {
                    buffer.set_language(lang.as_str().parse().expect("Lang is nonempty"));
                }
                if run.rtl {
                    buffer.set_direction(Direction::RightToLeft);
                } else {
                    buffer.set_direction(Direction::LeftToRight);
                }
                let output = rustybuzz::shape(&rb_face, &[], buffer);
                let positions = output.glyph_positions();

                let run_width: i32 = positions.iter().map(|p| p.x_advance).sum();
                line_width +=
                    run_width as f64 * style.size.0 / candidate.units_per_em as f64;
            }
        }
        max_width = f64::max(max_width, line_width);
    }

    Some(Pt(max_width))
}

pub(super) fn try_shape(
    world: &dyn World,
    pos: &Point,
    text: &ecow::EcoString,
    style: &TextStyle,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Option<Vec<FrameItem>> {
    // P568 — espaços entre palavras são emitidos como FrameItem::Text para
    // que o PDF contenha o caractere de espaço. Não os shapear, para que
    // o export primário (emit_text_pdf) os escreva directamente na stream
    // de texto em vez de os perder no shaping de glyphs.
    if text.trim().is_empty() {
        return None;
    }

    let font_list = style.font.as_ref()?;

    // P525 — derivar a variante real do TextStyle para VF e selecção de fonte.
    // P836 — eixos derivados + explícitos (`style.variations`).
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    // P515 — resolver todas as fontes candidatas da FontList.
    // **P538e/P555** — se a fonte declarada (incluindo a default "Libertinus Serif")
    // não existe no FontBook, tentar fontes padrão de fallback da mesma
    // classe (serif/sans) antes de recair no fallback global carácter-a-carácter.
    let primary = primary_candidates(world, font_list, &variant, face_cache, style.math);

    // P534 — candidatos de fallback: todo o FontBook, carregados lazy.
    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.book().infos().get(cand.slot_idx).cloned());

    // P484 — dividir em runs bidirectionais antes de shape.
    // **P845** — iterar TODOS os parágrafos bidi (antes só `paragraphs[0]`,
    // truncando texto com `\n` interno na primeira linha — achado #55 de
    // P831). Cada parágrafo é uma linha visível: x reinicia e y avança por
    // `line_advance`.
    let paragraphs = bidi_runs(text.as_str());
    if paragraphs.iter().all(|p| p.is_empty()) {
        return None;
    }

    // **P845** — avanço vertical por linha: top-edge + |bottom-edge| +
    // leading (default 0,65em), a MESMA fórmula do avanço de linha do
    // Layouter em L1 (P762) e os mesmos edges por omissão
    // (`cap-height`/`baseline`, via `edge_offset_pt`) — assim as linhas de
    // um texto com `\n` interno ficam espaçadas como as linhas normais do
    // documento. Sem face resolvida, cai para size + leading. Calculado
    // ANTES de mover `primary`/`face_cache` para o `CandidateSet`.
    let line_advance = {
        let (top, bottom) = primary
            .first()
            .and_then(|cand| face_cache.get(world, cand.slot_idx))
            .map(|cached| {
                let face = cached.face();
                let upem = face.units_per_em().max(1) as f64;
                (
                    crate::font_metrics::edge_offset_pt(
                        face,
                        upem,
                        style.size,
                        style.top_edge.as_ref(),
                        true,
                    ),
                    crate::font_metrics::edge_offset_pt(
                        face,
                        upem,
                        style.size,
                        style.bottom_edge.as_ref(),
                        false,
                    ),
                )
            })
            .unwrap_or((style.size, Pt(0.0)));
        let leading = style
            .leading
            .map(|l| l.resolve_pt(style.size.val()))
            // ref: lab/typst-original/crates/typst-library/src/model/par.rs:210
            .unwrap_or_else(|| style.size.0 * 0.65);
        top.0 - bottom.0 + leading
    };

    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);

    let mut items = Vec::new();

    for (para_idx, runs) in paragraphs.iter().enumerate() {
        // Linha vazia (`\n\n`): sem runs, mas o índice do parágrafo já faz o
        // y avançar para as linhas seguintes.
        let line_y = Pt(pos.y.0 + line_advance * para_idx as f64);
        let mut x_offset = Pt(0.0);

        for run in runs {
            for subrun in split_run_by_font(run, &mut candidates) {
                let candidate = candidates.get(subrun.candidate_idx)?;

                // P657 — cache de shaping: reaproveita o resultado bruto do shaper
                // quando o mesmo sub-run (texto + face + direção + variações +
                // tracking) já foi processado neste documento.
                // **P977** — nível ssty do estilo math (vanilla
                // `text/mod.rs:1457-1460`): Script → ssty=1,
                // ScriptScript → ssty=2. `shaper.md` §P977.
                // **P1129** — só para sub-run ssty-elegível
                // (`ssty_eligible_text`: 1 carácter ou número puro):
                // vanilla só força o script OpenType `math` (onde `ssty`
                // está registada na GSUB) nos caminhos de glifo singular
                // (`math/shaping.rs`) e de número dígito-a-dígito
                // (`layout_number`); texto multi-carácter não-numérico em
                // math (legendas de `underbrace`/`overbrace`, strings)
                // usa o shaper de parágrafo comum, que nunca pede `ssty`
                // com efeito nesta fonte.
                let ssty_level = ssty_level_for_subrun(style, &subrun.text);
                let tracking_fu = style
                    .tracking
                    .map(|t| {
                        let pt = t.resolve_pt(style.size.val());
                        (pt * candidate.units_per_em as f64 / style.size.val()).round()
                            as i32
                    })
                    .unwrap_or(0);
                let cache_key = ShapeCache::key(
                    &subrun.text,
                    candidate.slot_idx,
                    run.rtl,
                    &axis_vars,
                    tracking_fu,
                    ssty_level,
                    style.lang.as_ref().map(|lang| lang.as_str()),
                );
                let cached = cache.map.get(&cache_key).cloned();
                let (run_glyphs, _run_width) = if let Some(cached) = cached {
                    (cached.glyphs, cached.width)
                } else {
                    let font = world.font(candidate.slot_idx)?;
                    let mut rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;

                    // P525 — aplicar coordenadas de eixo OpenType (weight/italic) antes de shape.
                    if !axis_vars.is_empty() {
                        rb_face.set_variations(&axis_vars);
                    }

                    let mut buffer = UnicodeBuffer::new();
                    buffer.push_str(&subrun.text);
                    super::text_identity::configure_authored_shy_buffer(&mut buffer, &subrun.text);
                    if let Some(lang) = style.lang {
                        buffer.set_language(lang.as_str().parse().expect("Lang is nonempty"));
                    }
                    if run.rtl {
                        buffer.set_direction(Direction::RightToLeft);
                    } else {
                        buffer.set_direction(Direction::LeftToRight);
                    }
                    // **P977** — o vanilla shape texto math com o script
                    // OpenType `math` (`typst-layout/src/math/shaping.rs:200`)
                    // — a feature ssty (e outras de matemática) só está
                    // registada nesse script em NewCMMath.
                    if style.math {
                        if let Some(math_script) = rustybuzz::Script::from_iso15924_tag(
                            rustybuzz::ttf_parser::Tag::from_bytes(b"math"),
                        ) {
                            buffer.set_script(math_script);
                        }
                    }
                    // **P977** — feature ssty para texto math de script:
                    // os glifos shaped passam a ser as variantes
                    // `.st`/`.sts` (como o vanilla).
                    let ssty_feature;
                    let features: &[rustybuzz::Feature] = match ssty_level {
                        Some(level) => {
                            ssty_feature = [rustybuzz::Feature::new(
                                rustybuzz::ttf_parser::Tag::from_bytes(b"ssty"),
                                level as u32,
                                ..,
                            )];
                            &ssty_feature
                        }
                        None => &[],
                    };
                    let output = rustybuzz::shape(&rb_face, features, buffer);
                    let infos = output.glyph_infos();
                    let positions = output.glyph_positions();

                    let mut run_glyphs: Vec<ShapedGlyph> =
                        Vec::with_capacity(infos.len());
                    let mut run_width = 0i32;
                    // **P543** — o campo `text` do TextShaped reflecte apenas o sub-run,
                    // e os clusters são relativos a esse texto. Isto evita que o export
                    // ToUnicode ou `pdftotext` dupliquem conteúdo quando o texto original
                    // era partido em sub-runs pequenos.
                    let subrun_text = subrun.text.as_str();
                    // **P621** — tracking: adicionar espaçamento extra entre clusters
                    // de caracteres, convertido de pontos para unidades da fonte.
                    // O tracking é aplicado ao avanço de um glifo apenas quando o glifo
                    // seguinte pertence a um cluster diferente, evitando partir ligaduras
                    // e conjuntos (ex.: devanágari) onde vários glifos compõem um único
                    // caractere visual. O último glifo do sub-run nunca recebe tracking.
                    let n_glyphs = infos.len();
                    let clusters: Vec<usize> =
                        infos.iter().map(|info| info.cluster as usize).collect();
                    for (idx, (info, pos_g)) in
                        infos.iter().zip(positions.iter()).enumerate()
                    {
                        let cluster = clusters[idx];
                        let char_code =
                            byte_idx_to_char(subrun_text, cluster).unwrap_or('\u{FFFD}');
                        let is_last = idx + 1 == n_glyphs;
                        let next_cluster_differs =
                            !is_last && clusters[idx + 1] != cluster;
                        let extra = if next_cluster_differs { tracking_fu } else { 0 };
                        run_glyphs.push(ShapedGlyph {
                            glyph_id: info.glyph_id as u16,
                            x_advance: pos_g.x_advance + extra,
                            x_offset: pos_g.x_offset,
                            y_offset: pos_g.y_offset,
                            cluster: cluster as u32,
                            char_code,
                        });
                        run_width += pos_g.x_advance + extra;
                    }

                    cache.map.insert(
                        cache_key,
                        CachedRun { glyphs: run_glyphs.clone(), width: run_width },
                    );

                    (run_glyphs, run_width)
                };

                if !run_glyphs.is_empty() {
                    let subrun_width_pt =
                        super::tracking::width(&run_glyphs, style, candidate.units_per_em as f64);
                    let item_pos = Point { x: Pt(pos.x.0 + x_offset.0), y: line_y };
                    x_offset.0 += subrun_width_pt;

                    // P534 — cada sub-run reflecte a família real usada, para que o
                    // export multi-font embuta a face correcta e a associe via
                    // `font_index_for_style`.
                    let mut segment_style = style.clone();
                    let real_family =
                        world.book().infos().get(candidate.slot_idx)?.family.clone();
                    segment_style.font =
                        Some(FontList::single(ecow::EcoString::from(real_family)));

                    items.push(FrameItem::TextShaped {
                        pos: item_pos,
                        glyphs: run_glyphs,
                        style: segment_style,
                        text: subrun.text.clone().into(),
                        units_per_em: candidate.units_per_em,
                    });
                }
            }
        }
    }

    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

pub(super) fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s.get(byte_idx..)?.chars().next()
}
