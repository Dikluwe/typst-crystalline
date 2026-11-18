//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/shaping.md
//! @prompt-hash 1edecf5f
//! @layer L3
//! @updated 2026-11-07

use super::bidi::bidi_runs_with_direction;
use super::cache::{
    BidiParagraphs, CachedRun, FaceCache, PrimaryProofKey, RawShapeInput, ShapeCache,
};
use super::complete_session::{Product, Request};
use super::font_selection::{
    first_candidate, is_p1404_format_control, primary_candidates, split_run_by_font,
    CandidateSet, FontCandidate,
};
use crate::font_metrics::ssty_eligible_text;
use crate::font_variant::{axis_variations_for_text_style, text_style_to_font_variant};
use rustybuzz::{Direction, UnicodeBuffer};
use std::sync::Arc;
use typst_core::contracts::world::World;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{
    FrameItem, MathSize, Point, Pt, ShapedGlyph, SharedGlyphs, TextStyle,
};

struct CertifiedPrimary {
    primary: Vec<FontCandidate>,
    paragraphs: Option<BidiParagraphs>,
}

/// Retain only a completed ordinary proof; all source-selection guards still run.
fn certified_primary(
    world: &dyn World,
    text: &str,
    style: &TextStyle,
    face_cache: &mut FaceCache,
) -> Option<CertifiedPrimary> {
    if style.math
        || style.superscript
        || style.subscript
        || text.is_empty()
        || !matches!(
            style.dir,
            None | Some(typst_core::entities::dir::Dir::LTR)
                | Some(typst_core::entities::dir::Dir::RTL)
        )
    {
        return None;
    }
    let variant = text_style_to_font_variant(style);
    let first = first_candidate(world, style.font.as_ref()?, &variant, face_cache)?;
    // This exact lemma neither publishes a proof nor substitutes supplied bidi context.
    if text.chars().all(is_p1404_format_control) {
        return Some(CertifiedPrimary { primary: vec![first], paragraphs: None });
    }
    let face = face_cache.get(world, first.slot_idx)?;
    if !text
        .chars()
        .all(|c| face.face().glyph_index(c).is_some_and(|g| g.0 != 0))
    {
        return None;
    }
    let font = world.font(first.slot_idx)?;
    let axes = axis_variations_for_text_style(style);
    let key = PrimaryProofKey::new(
        text,
        first.slot_idx,
        &font,
        &axes,
        style.dir,
        style.lang.as_ref().map(|language| language.as_str()),
    );
    if let Some((_, paragraphs)) = face_cache.primary_proofs.get(&key) {
        return Some(CertifiedPrimary {
            primary: vec![first],
            paragraphs: Some(Arc::clone(paragraphs)),
        });
    }
    let raw = face_cache.raw_store();
    #[cfg(test)]
    super::tests::raw_reuse::record_primary_proof();
    let paragraphs = Arc::new(bidi_runs_with_direction(text, style.dir));
    if paragraphs.iter().all(Vec::is_empty) {
        return None;
    }
    {
        let mut candidates =
            CandidateSet::new(world, vec![first], face_cache, None, variant);
        for runs in paragraphs.iter() {
            for run in runs {
                for subrun in split_run_by_font(run, &mut candidates) {
                    let output = raw.lock().unwrap().shape(RawShapeInput {
                        text: &subrun.text,
                        slot: first.slot_idx,
                        font: &font,
                        axes: &axes,
                        rtl: run.rtl,
                        language: style.lang.as_ref().map(|language| language.as_str()),
                        script: None,
                        features: &[],
                    })?;
                    if output.glyph_infos().is_empty()
                        || output.glyph_infos().iter().any(|glyph| glyph.glyph_id == 0)
                    {
                        return None;
                    }
                }
            }
        }
    }
    face_cache.primary_proofs.insert(key, (font, Arc::clone(&paragraphs)));
    Some(CertifiedPrimary { primary: vec![first], paragraphs: Some(paragraphs) })
}

/// Nível OpenType `ssty` solicitado para um sub-run matemático.
///
/// TextItem preserva `style.math` para família, fallback e demais eixos de
/// shaping, mas segue a rota textual do vanilla e não seleciona `.st`/`.sts`.
pub(super) fn ssty_level_for_subrun(style: &TextStyle, text: &str) -> Option<u8> {
    if !style.math || style.math_text_item || !ssty_eligible_text(text) {
        return None;
    }
    match style.math_size {
        MathSize::Script => Some(1),
        MathSize::ScriptScript => Some(2),
        MathSize::Display | MathSize::Text => None,
    }
}

/// Validate supplied policy and match the actual face's cmap, in policy order.
pub(crate) fn glyph_overhang_in_face(
    face: &ttf_parser::Face<'_>,
    glyph_id: u16,
    advance: Pt,
    ratios: &[(char, f64)],
) -> Option<Pt> {
    if !advance.0.is_finite()
        || advance.0 < 0.0
        || glyph_id >= face.number_of_glyphs()
        || ratios.iter().any(|(_, ratio)| !ratio.is_finite() || *ratio < 0.0)
    {
        return None;
    }
    let ratio = ratios
        .iter()
        .find_map(|(c, ratio)| {
            face.glyph_index(*c).filter(|gid| gid.0 == glyph_id).map(|_| *ratio)
        })
        .unwrap_or(0.0);
    let value = advance.0 * ratio;
    value.is_finite().then_some(Pt(value))
}

/// The measurement path's existing feature policy, without producing frame items.
pub(crate) fn terminal_in_face(
    face: &rustybuzz::Face<'_>,
    text: &str,
    style: &TextStyle,
    ratios: &[(char, f64)],
) -> Option<(Pt, usize)> {
    if text.is_empty()
        || !style.size.0.is_finite()
        || style.size.0 <= 0.0
        || style
            .dir
            .is_some_and(|dir| dir != typst_core::entities::dir::Dir::LTR)
        || text.chars().any(|c| c == '\n' || c == '\r')
    {
        return None;
    }
    let paragraphs = bidi_runs_with_direction(text, style.dir);
    if paragraphs.len() != 1 || paragraphs[0].iter().any(|run| run.rtl) {
        return None;
    }
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    super::text_identity::configure_authored_ignorables_buffer(&mut buffer, text);
    if let Some(lang) = style.lang {
        buffer.set_language(lang.as_str().parse().ok()?);
    }
    buffer.set_direction(Direction::LeftToRight);
    let feature = rustybuzz::Feature::new(ttf_parser::Tag::from_bytes(b"sups"), 1, ..);
    let native = style.superscript
        && style.superscript_native != Some(false)
        && !style.subscript
        && crate::font_metrics::superscript_in_face(face, text, style.size).3;
    let features = if native { std::slice::from_ref(&feature) } else { &[] };
    let output = rustybuzz::shape(face, features, buffer);
    let info = output.glyph_infos().last()?;
    let position = output.glyph_positions().last()?;
    let upem = face.units_per_em();
    if upem == 0 {
        return None;
    }
    let advance = Pt(f64::from(position.x_advance) * style.size.0 / f64::from(upem));
    let value = glyph_overhang_in_face(
        face,
        u16::try_from(info.glyph_id).ok()?,
        advance,
        ratios,
    )?;
    Some((value, output.glyph_infos().len()))
}

/// Resolve precisely the same candidates/axes as shaped_width; certify LTR only.
pub(crate) fn shaped_trailing_overhang(
    world: &dyn World,
    text: &str,
    style: &TextStyle,
    ratios: &[(char, f64)],
    face_cache: &mut FaceCache,
) -> Option<(Pt, usize)> {
    if style
        .dir
        .is_some_and(|dir| dir != typst_core::entities::dir::Dir::LTR)
        || text.chars().any(|c| c == '\n' || c == '\r')
    {
        return None;
    }
    let paragraphs = bidi_runs_with_direction(text, style.dir);
    if paragraphs.len() != 1 || paragraphs[0].iter().any(|run| run.rtl) {
        return None;
    }
    let font_list = style.font.as_ref()?;
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);
    let primary = certified_primary(world, text, style, face_cache)
        .map(|proof| proof.primary)
        .unwrap_or_else(|| {
            primary_candidates(world, font_list, &variant, face_cache, style.math)
        });
    let like = primary
        .first()
        .and_then(|cand| world.font_info(cand.slot_idx).cloned());
    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);
    let mut terminal = None;
    let mut count = 0usize;
    for run in &paragraphs[0] {
        for subrun in split_run_by_font(run, &mut candidates) {
            let candidate = candidates.get(subrun.candidate_idx)?;
            let font = world.font(candidate.slot_idx)?;
            let mut face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;
            face.set_variations(&axis_vars);
            let mut substyle = style.clone();
            if !crate::font_metrics::superscript_in_face(&face, text, style.size).3 {
                substyle.superscript_native = Some(false);
            }
            let (amount, glyphs) =
                terminal_in_face(&face, &subrun.text, &substyle, ratios)?;
            count = count.checked_add(glyphs)?;
            terminal = Some(amount);
        }
    }
    Some((terminal?, count))
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

    let request = Request::width(face_cache.products.as_ref(), world, text, style);
    if let Some(product) = request.as_ref().and_then(|r| r.lookup(world)) {
        if let Product::Width(width) = product.as_ref() {
            return Some(*width);
        }
    }
    let font_list = style.font.as_ref()?;
    let raw_cache = face_cache.raw_store();

    // P836 — eixos derivados + explícitos (`style.variations`), explícitos
    // vencem por tag.
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    let (primary, prepared) = certified_primary(world, text, style, face_cache)
        .map(|proof| (proof.primary, proof.paragraphs))
        .unwrap_or_else(|| {
            (primary_candidates(world, font_list, &variant, face_cache, style.math), None)
        });

    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.font_info(cand.slot_idx).cloned());
    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);

    // P845 — runs agrupados por parágrafo bidi (texto com `\n` interno).
    let paragraphs =
        prepared.unwrap_or_else(|| Arc::new(bidi_runs_with_direction(text, style.dir)));
    if paragraphs.iter().all(|p| p.is_empty()) {
        return Some(Pt(0.0));
    }

    // Largura de texto multilinha = max das larguras de linha (não a soma):
    // cada parágrafo é uma linha visível independente.
    let mut max_width = 0.0f64;

    for runs in paragraphs.iter() {
        let mut line_width = 0.0f64;
        for run in runs {
            for subrun in split_run_by_font(run, &mut candidates) {
                let candidate = candidates.get(subrun.candidate_idx)?;
                let font = world.font(candidate.slot_idx)?;
                let native = if style.superscript
                    && style.superscript_native != Some(false)
                    && !style.subscript
                {
                    let mut face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;
                    face.set_variations(&axis_vars);
                    crate::font_metrics::superscript_in_face(&face, text, style.size).3
                } else {
                    false
                };
                let sups = [rustybuzz::Feature::new(
                    rustybuzz::ttf_parser::Tag::from_bytes(b"sups"),
                    1,
                    ..,
                )];
                let features = if native { &sups[..] } else { &[] };
                let output = raw_cache.lock().unwrap().shape(RawShapeInput {
                    text: &subrun.text,
                    slot: candidate.slot_idx,
                    font: &font,
                    axes: &axis_vars,
                    rtl: run.rtl,
                    language: style.lang.as_ref().map(|l| l.as_str()),
                    script: None,
                    features,
                })?;
                let positions = output.glyph_positions();

                let run_width: i32 = positions.iter().map(|p| p.x_advance).sum();
                line_width +=
                    run_width as f64 * style.size.0 / candidate.units_per_em as f64;
            }
        }
        max_width = f64::max(max_width, line_width);
    }

    let width = Pt(max_width);
    if let Some(request) = request {
        request.publish(world, Product::Width(width));
    }
    Some(width)
}

#[cfg(test)]
pub(super) fn try_shape(
    world: &dyn World,
    pos: &Point,
    text: &ecow::EcoString,
    style: &TextStyle,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Option<Vec<FrameItem>> {
    try_shape_context(world, pos, text, style, cache, face_cache, None, None)
}
#[expect(
    clippy::too_many_arguments,
    reason = "C117: Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada"
)]
pub(super) fn try_shape_context(
    world: &dyn World,
    pos: &Point,
    text: &ecow::EcoString,
    style: &TextStyle,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
    bidi: Option<&typst_core::entities::bidi_context::BidiSpan>,
    resolved: Option<&crate::bidi_context::Resolved<'_>>,
) -> Option<Vec<FrameItem>> {
    // P568 — espaços entre palavras são emitidos como FrameItem::Text para
    // que o PDF contenha o caractere de espaço. Não os shapear, para que
    // o export primário (emit_text_pdf) os escreva directamente na stream
    // de texto em vez de os perder no shaping de glyphs.
    if text.trim().is_empty() {
        return None;
    }

    let request = Request::emission(
        face_cache.products.as_ref(),
        world,
        text,
        style,
        pos,
        bidi,
        resolved,
    );
    if let Some(product) = request.as_ref().and_then(|r| r.lookup(world)) {
        if let Some(items) = product.materialize(style, bidi) {
            return Some(items);
        }
    }
    let font_list = style.font.as_ref()?;
    let raw_cache = face_cache.raw_store();

    // P525 — derivar a variante real do TextStyle para VF e selecção de fonte.
    // P836 — eixos derivados + explícitos (`style.variations`).
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    // P515 — resolver todas as fontes candidatas da FontList.
    // **P538e/P555** — se a fonte declarada (incluindo a default "Libertinus Serif")
    // não existe no FontBook, tentar fontes padrão de fallback da mesma
    // classe (serif/sans) antes de recair no fallback global carácter-a-carácter.
    let (primary, prepared) =
        if bidi.is_none() || text.chars().all(is_p1404_format_control) {
            certified_primary(world, text, style, face_cache)
        } else {
            None
        }
        .map(|proof| (proof.primary, proof.paragraphs))
        .unwrap_or_else(|| {
            (primary_candidates(world, font_list, &variant, face_cache, style.math), None)
        });

    let bidi_only = text.chars().all(|c| {
        matches!(
            c, '\u{61c}' | '\u{200e}' | '\u{200f}'
                | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
        )
    });

    // P534 — candidatos de fallback: todo o FontBook, carregados lazy.
    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.font_info(cand.slot_idx).cloned());

    // P484 — dividir em runs bidirectionais antes de shape.
    // **P845** — iterar TODOS os parágrafos bidi (antes só `paragraphs[0]`,
    // truncando texto com `\n` interno na primeira linha — achado #55 de
    // P831). Cada parágrafo é uma linha visível: x reinicia e y avança por
    // `line_advance`.
    let paragraphs = if let (Some(span), Some(resolved)) = (bidi, resolved) {
        Arc::new(vec![resolved
            .runs(span, text)?
            .into_iter()
            .map(|(r, rtl)| super::bidi::BidiRun {
                text: text[r.clone()].to_string(),
                rtl,
                byte_start: r.start,
            })
            .collect()])
    } else {
        prepared.unwrap_or_else(|| {
            Arc::new(bidi_runs_with_direction(text.as_str(), style.dir))
        })
    };
    if paragraphs.iter().all(|p| p.is_empty()) {
        return if bidi_only && !primary.is_empty() { Some(Vec::new()) } else { None };
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
            let mut subrun_start = run.byte_start;
            for subrun in split_run_by_font(run, &mut candidates) {
                let subrun_range = subrun_start..subrun_start + subrun.text.len();
                subrun_start = subrun_range.end;
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
                let native_sups = if style.superscript
                    && !style.subscript
                    && style.superscript_native != Some(false)
                {
                    let mut face = candidates.selected_face(candidate.slot_idx)?.clone();
                    for variation in &axis_vars {
                        face.set_variation(variation.tag, variation.value);
                    }
                    crate::font_metrics::superscript_in_face(&face, text, style.size).3
                } else {
                    false
                };
                let mut cache_key = ShapeCache::key(
                    &subrun.text,
                    candidate.slot_idx,
                    run.rtl,
                    &axis_vars,
                    tracking_fu,
                    ssty_level,
                    style.lang.as_ref().map(|lang| lang.as_str()),
                );
                if style.superscript
                    && !style.subscript
                    && style.superscript_native != Some(false)
                {
                    cache_key.set_native_sups(native_sups);
                }
                let cached = cache.map.get(&cache_key).cloned();
                let (run_glyphs, _run_width, source_font) = if let Some(cached) = cached {
                    (cached.glyphs, cached.width, cached.font)
                } else {
                    let font = world.font(candidate.slot_idx)?;
                    let script = if style.math {
                        rustybuzz::Script::from_iso15924_tag(
                            rustybuzz::ttf_parser::Tag::from_bytes(b"math"),
                        )
                    } else {
                        None
                    };
                    let sups_feature = [rustybuzz::Feature::new(
                        rustybuzz::ttf_parser::Tag::from_bytes(b"sups"),
                        1,
                        ..,
                    )];
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
                    let features = if native_sups { &sups_feature[..] } else { features };
                    let output = raw_cache.lock().unwrap().shape(RawShapeInput {
                        text: &subrun.text,
                        slot: candidate.slot_idx,
                        font: &font,
                        axes: &axis_vars,
                        rtl: run.rtl,
                        language: style.lang.as_ref().map(|l| l.as_str()),
                        script,
                        features,
                    })?;
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

                    let run_glyphs = SharedGlyphs::from(run_glyphs);
                    cache.map.insert(
                        cache_key,
                        CachedRun {
                            glyphs: run_glyphs.clone(),
                            width: run_width,
                            font: font.clone(),
                        },
                    );

                    (run_glyphs, run_width, font)
                };

                if !run_glyphs.is_empty() {
                    let subrun_width_pt = super::tracking::width(
                        &run_glyphs,
                        style,
                        candidate.units_per_em as f64,
                    );
                    let item_pos = Point { x: Pt(pos.x.0 + x_offset.0), y: line_y };
                    x_offset.0 += subrun_width_pt;

                    // P534 — cada sub-run reflecte a família real usada, para que o
                    // export multi-font embuta a face correcta e a associe via
                    // `font_index_for_style`.
                    let mut segment_style = style.clone();
                    let real_family = world.font_info(candidate.slot_idx)?.family.clone();
                    segment_style.font =
                        Some(FontList::single(ecow::EcoString::from(real_family)));

                    items.push(FrameItem::TextShaped {
                        pos: item_pos,
                        glyphs: run_glyphs,
                        style: segment_style,
                        text: subrun.text.clone().into(),
                        units_per_em: candidate.units_per_em,
                        bidi: bidi.and_then(|span| span.subspan(subrun_range)),
                        font: Some(source_font),
                    });
                }
            }
        }
    }

    // Successful removal of a bidi-only run must not re-enter plain fallback.
    if items.is_empty() && !bidi_only {
        None
    } else {
        if let Some(request) = request {
            if let Some(product) = Product::emitted(&items, style, bidi) {
                request.publish(world, product);
            }
        }
        Some(items)
    }
}

pub(super) fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s.get(byte_idx..)?.chars().next()
}

#[cfg(test)]
mod r8_script_cache_tests {
    use super::*;
    use typst_core::entities::{
        duration::Duration,
        file_id::FileId,
        font_book::FontBook,
        font_list::{FontFamily, FontList},
        layout_types::ShapedGlyph,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct FontWorld {
        library: Library,
        book: FontBook,
        fonts: Vec<Font>,
    }
    impl World for FontWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, i: usize) -> Option<Font> {
            self.fonts.get(i).cloned()
        }
        fn today(&self, _: Option<Duration>) -> Option<Datetime> {
            None
        }
        fn candidates_for_char(&self, _: char) -> Vec<usize> {
            (0..self.fonts.len()).collect()
        }
    }
    fn trace(items: Vec<FrameItem>) -> Vec<Vec<ShapedGlyph>> {
        items
            .into_iter()
            .filter_map(|item| match item {
                FrameItem::TextShaped { glyphs, .. } => Some(glyphs.to_vec()),
                _ => None,
            })
            .collect()
    }
    #[test]
    fn r8_native_feature_cache_is_independent_of_previous_script_runs() {
        let data = [
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").as_slice(),
            include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").as_slice(),
        ];
        assert!(ttf_parser::Face::parse(data[0], 0)
            .unwrap()
            .glyph_index('☃')
            .is_none());
        assert!(ttf_parser::Face::parse(data[1], 0)
            .unwrap()
            .glyph_index('☃')
            .is_some());
        let mut book = FontBook::new();
        for bytes in data {
            let mut info = crate::fonts::font_info_from_bytes(bytes, 0).unwrap();
            info.coverage = crate::fonts::extract_coverage(
                &ttf_parser::Face::parse(bytes, 0).unwrap(),
            );
            book.push(info);
        }
        let world = FontWorld {
            library: Library::new(),
            book,
            fonts: data.into_iter().map(|b| Font::from_data(b.to_vec())).collect(),
        };
        let normal = TextStyle {
            size: Pt(11.0),
            superscript: true,
            font: FontList::new(vec![
                FontFamily::new("Nimbus Sans".into()),
                FontFamily::new("DejaVu Sans".into()),
            ]),
            ..TextStyle::default()
        };
        let mixed = TextStyle { size: Pt(7.15), ..normal.clone() };
        let pos = Point { x: Pt(0.0), y: Pt(0.0) };
        for (text, expected_runs) in [("123☃", 1), ("123\nZ", 2)] {
            let mut cache = ShapeCache::new();
            let mut faces = FaceCache::new();
            try_shape(&world, &pos, &"123".into(), &normal, &mut cache, &mut faces)
                .unwrap();
            let warmed = trace(
                try_shape(&world, &pos, &text.into(), &mixed, &mut cache, &mut faces)
                    .unwrap(),
            );
            let cold = trace(
                try_shape(
                    &world,
                    &pos,
                    &text.into(),
                    &mixed,
                    &mut ShapeCache::new(),
                    &mut FaceCache::new(),
                )
                .unwrap(),
            );
            assert_eq!(cold.len(), expected_runs, "actual run selection for {text:?}");
            assert_eq!(warmed,cold,"warming another superscript must not change emitted glyph forms or advances for {text:?}");
        }
    }
    #[test]
    fn selective_primary_does_not_poison_later_longest_fallback() {
        let data = [
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").as_slice(),
            include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").as_slice(),
        ];
        assert!(ttf_parser::Face::parse(data[0], 0)
            .unwrap()
            .glyph_index('☃')
            .is_none());
        assert!(ttf_parser::Face::parse(data[1], 0)
            .unwrap()
            .glyph_index('☃')
            .is_some());
        let mut book = FontBook::new();
        for bytes in data {
            let mut info = crate::fonts::font_info_from_bytes(bytes, 0).unwrap();
            info.coverage = crate::fonts::extract_coverage(
                &ttf_parser::Face::parse(bytes, 0).unwrap(),
            );
            book.push(info);
        }
        let world = FontWorld {
            library: Library::new(),
            book,
            fonts: data.into_iter().map(|b| Font::from_data(b.to_vec())).collect(),
        };
        let normal = TextStyle {
            size: Pt(11.0),
            superscript: false,
            font: FontList::new(vec![
                FontFamily::new("Nimbus Sans".into()),
                FontFamily::new("DejaVu Sans".into()),
            ]),
            ..TextStyle::default()
        };
        let mixed = TextStyle { size: Pt(7.15), ..normal.clone() };
        let pos = Point { x: Pt(0.0), y: Pt(0.0) };
        for (text, expected_runs) in [("123☃", 1), ("123\nZ", 2)] {
            let mut cache = ShapeCache::new();
            let mut faces = FaceCache::new();
            try_shape(&world, &pos, &"123".into(), &normal, &mut cache, &mut faces)
                .unwrap();
            let warmed = trace(
                try_shape(&world, &pos, &text.into(), &mixed, &mut cache, &mut faces)
                    .unwrap(),
            );
            let cold = trace(
                try_shape(
                    &world,
                    &pos,
                    &text.into(),
                    &mixed,
                    &mut ShapeCache::new(),
                    &mut FaceCache::new(),
                )
                .unwrap(),
            );
            assert_eq!(cold.len(), expected_runs, "actual run selection for {text:?}");
            assert_eq!(warmed,cold,"warming the selective primary must not change emitted glyph forms or advances for {text:?}");
        }
    }
}
