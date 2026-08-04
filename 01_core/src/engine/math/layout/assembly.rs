//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/assembly.md
//! @prompt-hash 995a1b6d
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_assembly` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::layout_types::{FrameItem, Point, Pt, TextStyle};

use super::MathBox;
use crate::entities::glyph_variants::{GlyphAssembly, GlyphPart};

/// **P918** — Resolve `repeat` (nº de repetições das peças extensoras) e
/// `ratio` (razão de espalhamento) para cobrir `target_pt`, devolvendo
/// `parts_vec` já expandido com `repeat` cópias de cada peça extensora.
/// Algoritmo do vanilla (P913). Partilhado por `layout_assembly` e
/// `layout_assembly_horizontal` — laço confirmado idêntico byte-a-byte nos
/// dois (ver `assembly.md` §P918).
/// **P945** — o laço passa a descontar `min_overlap` (`minConnectorOverlap`
/// da tabela MATH) do acumulador `growable` (vanilla `glyph.rs:610`:
/// `growable += max(0, max_overlap − min_overlap)`); antes acumulava o
/// `max_overlap` inteiro. Ver `assembly.md` §P945.
fn resolve_assembly_repeat<'p>(
    assembly: &'p GlyphAssembly,
    scale: f64,
    target_pt: f64,
) -> (Vec<&'p GlyphPart>, f64) {
    const MAX_REPEATS: usize = 1024;
    // **P945** — `minConnectorOverlap` convertido para pt (design units ×
    // scale) — mesmo factor de escala dos conectores das peças.
    let min_overlap_pt = assembly.min_overlap as f64 * scale;
    let mut full_pt;
    let mut ratio = 0.0_f64;
    let mut repeat = 0_usize;

    loop {
        full_pt = 0.0;
        ratio = 0.0;

        let mut parts_vec: Vec<&GlyphPart> = Vec::new();
        for part in &assembly.parts {
            let count = if part.is_extender { repeat } else { 1 };
            for _ in 0..count {
                parts_vec.push(part);
            }
        }

        let mut growable_pt = 0.0_f64;
        let n = parts_vec.len();
        for i in 0..n {
            let part = parts_vec[i];
            let advance_pt = part.full_advance as f64 * scale;
            let mut advance = advance_pt;
            if i + 1 < n {
                let next = parts_vec[i + 1];
                let max_overlap =
                    (part.end_connector as f64).min(next.start_connector as f64) * scale;
                advance -= max_overlap;
                // **P945** — só o overlap ACIMA do mínimo da fonte é
                // growable (vanilla `glyph.rs:610`).
                growable_pt += (max_overlap - min_overlap_pt).max(0.0);
            }
            full_pt += advance;
        }

        if full_pt < target_pt && growable_pt > 0.0 {
            let delta = target_pt - full_pt;
            ratio = (delta / growable_pt).min(1.0);
            full_pt += ratio * growable_pt;
        }

        if target_pt <= full_pt || repeat >= MAX_REPEATS {
            break;
        }

        repeat += 1;
    }

    let mut parts_vec: Vec<&GlyphPart> = Vec::new();
    for part in &assembly.parts {
        let count = if part.is_extender { repeat } else { 1 };
        for _ in 0..count {
            parts_vec.push(part);
        }
    }

    (parts_vec, ratio)
}

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_assembly(
        &self,
        c: char,
        assembly: GlyphAssembly,
        target_advance_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let target_pt = target_advance_du * scale;

        // **P918** — determinação de `repeat`/`ratio` (algoritmo do vanilla,
        // P913) extraída para `resolve_assembly_repeat` (ver `assembly.md`
        // §P918) — laço confirmado idêntico byte-a-byte entre este método e
        // `layout_assembly_horizontal`.
        let (parts_vec, ratio) = resolve_assembly_repeat(&assembly, scale, target_pt);
        // **P945** — `minConnectorOverlap` em pt, usado na fórmula de
        // overlap do posicionamento (vanilla `glyph.rs:639-640`), replicada
        // byte-a-byte em `layout_assembly_horizontal` (ver `assembly.md`
        // §P945).
        let min_overlap_pt = assembly.min_overlap as f64 * scale;

        let mut items = Vec::new();
        let mut max_advance = 0.0_f64;
        // **P917** — `advance_pt` (eixo de empilhamento, para `y_in_box`) e
        // `hor_advance_pt` (avanço nativo do glifo, para `x_advance`/largura
        // da caixa) são medidas distintas — ver `assembly.md` §P917.
        let mut piece_positions: Vec<(f64, u16, f64, f64)> = Vec::new();

        let mut y_cursor = 0.0_f64;
        let n = parts_vec.len();
        for i in 0..n {
            let part = parts_vec[i];
            let advance_pt = part.full_advance as f64 * scale;
            let hor_advance_pt = part.hor_advance * scale;

            let overlap = if i + 1 < n {
                let next = parts_vec[i + 1];
                let max_overlap =
                    (part.end_connector as f64).min(next.start_connector as f64) * scale;
                // **P945** — vanilla `glyph.rs:639-640`: o espalhamento
                // (ratio) actua só sobre o overlap ACIMA do mínimo da fonte
                // — as juntas nunca abrem menos que `min_overlap`.
                max_overlap - ratio * (max_overlap - min_overlap_pt)
            } else {
                0.0
            };

            piece_positions.push((y_cursor, part.glyph_id, advance_pt, hor_advance_pt));
            max_advance = max_advance.max(hor_advance_pt);
            y_cursor += advance_pt - overlap;
        }
        let total_height = y_cursor;
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let half_h = total_height / 2.0;
        let ascent = axis_pt + half_h;
        let descent = (half_h - axis_pt).max(0.0);
        // **P957** — a baseline de cada peça fica no FUNDO do seu slot
        // (convenção do vanilla, `glyph.rs:631-671`: cada glifo é desenhado
        // na origem acumulada; as peças de NewCMMath têm `yMin = 0`, logo o
        // `y_offset = descent` por peça do vanilla é zero aqui). Antes de
        // P957, `y_in_box` subtraía `advance_pt` — baseline no TOPO do slot
        // — e o passo entre baselines consecutivas saía `advance_{i+1} −
        // overlap_i` (sequência rodada de uma posição face ao vanilla,
        // medido com valores reais de NewCMMath-Book: parêntese
        // [429, 376, 1426]du vs vanilla [1426, 376, 429]du): cada peça
        // ficava `advance_i` acima do sítio certo e a peça inferior era
        // coberta pelo extensor (visual: "canto reto" no fundo do
        // delimitador — achado do dono). Com baselines no fundo dos slots,
        // a tinta ocupa exactamente `[0, total_height]`, logo a centragem
        // no eixo é `shift_y = −axis − total/2` — a tinta cobre exactamente
        // a caixa declarada (`axis ± half`, P914). Isto SUBSTITUI a fórmula
        // de P952b (`−axis − (total − adv_topo − adv_fundo)/2`), que era a
        // compensação necessária enquanto as baselines estavam no topo dos
        // slots. Ver `assembly.md` §P957.
        let shift_y = -axis_pt - half_h;

        for (y_from_bottom, glyph_id, _advance_pt, hor_advance_pt) in piece_positions {
            // Posição vertical: eixo de empilhamento (`full_advance`),
            // inalterado por P917; baseline no fundo do slot (P957).
            let y_in_box = total_height - y_from_bottom;
            items.push(FrameItem::Glyph {
                pos: Point {
                    x: Pt(0.0),
                    y: Pt(y_in_box + shift_y),
                },
                glyph_id,
                // **P917** — avanço nativo do glifo, nunca a medida do eixo
                // de empilhamento.
                x_advance: Pt(hor_advance_pt),
                size: style.size,
                style: style.clone(),
                base_char: c,
            });
        }

        MathBox {
            width: max_advance,
            ascent,
            descent,
            items,
        }
    }

    pub(super) fn layout_assembly_horizontal(
        &self,
        c: char,
        assembly: GlyphAssembly,
        target_advance_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let target_pt = target_advance_du * scale;

        // **P918** — determinação de `repeat`/`ratio` extraída para
        // `resolve_assembly_repeat` (ver `assembly.md` §P918) — laço
        // confirmado idêntico byte-a-byte a `layout_assembly`.
        let (parts_vec, ratio) = resolve_assembly_repeat(&assembly, scale, target_pt);
        // **P945** — mesma fórmula de overlap com `min_overlap` de
        // `layout_assembly` (replicada byte-a-byte, ver `assembly.md`
        // §P945).
        let min_overlap_pt = assembly.min_overlap as f64 * scale;

        let mut items = Vec::new();
        let mut x_cursor = 0.0_f64;

        let n = parts_vec.len();
        for i in 0..n {
            let part = parts_vec[i];
            let advance_pt = part.full_advance as f64 * scale;
            // **P917** — `x_cursor` (posição cumulativa, determina onde a
            // peça seguinte começa) continua a usar `full_advance` (eixo de
            // empilhamento, que aqui coincide com X); só o `x_advance`
            // reportado no FrameItem passa a usar o avanço nativo do glifo
            // — ver `assembly.md` §P917.
            let x_advance = Pt(part.hor_advance * scale);

            items.push(FrameItem::Glyph {
                pos: Point {
                    x: Pt(x_cursor),
                    y: Pt(0.0),
                },
                glyph_id: part.glyph_id,
                x_advance,
                size: style.size,
                style: style.clone(),
                base_char: c,
            });

            let overlap = if i + 1 < n {
                let next = parts_vec[i + 1];
                let max_overlap =
                    (part.end_connector as f64).min(next.start_connector as f64) * scale;
                // **P945** — fórmula idêntica à de `layout_assembly`
                // (vanilla `glyph.rs:639-640`).
                max_overlap - ratio * (max_overlap - min_overlap_pt)
            } else {
                0.0
            };
            x_cursor += advance_pt - overlap;
        }

        let (ascent, _) = self.metrics.vertical_metrics(style.size, style);

        MathBox {
            width: x_cursor,
            ascent: ascent.val(),
            descent: 0.0,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
