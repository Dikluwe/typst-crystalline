//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/underover.md
//! @prompt-hash 1f0c4d20
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_underover` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P297).

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Pt, TextStyle},
};

use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P297** — Layout underover: empilha `over` (topo), `base`
    /// (meio), `under` (fundo). Cada Option é skipped se `None`.
    /// Width final = max das 3 partes; cada parte centrada
    /// horizontalmente. Agregação cristalina per ADR-0054 graded —
    /// vanilla typst fragmenta em 12 elementos (underbrace/overbrace/
    /// underbracket/etc.); cristalino unifica num único variant.
    pub(super) fn layout_underover(
        &self,
        base: &Content,
        under: Option<&Content>,
        over: Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        self.layout_underover_with_attach(base, under, over, style).0
    }

    fn layout_underover_with_attach(
        &self,
        base: &Content,
        under: Option<&Content>,
        over: Option<&Content>,
        style: &TextStyle,
    ) -> (MathBox, f64) {
        let (base_box, inherited_attach) = match base {
            Content::MathUnderover(e) => {
                let (box_, attach) = self.layout_underover_with_attach(
                    &e.base,
                    e.under.as_ref(),
                    e.over.as_ref(),
                    style,
                );
                (box_, Some(attach))
            }
            _ => (self.layout_node(base, style), None),
        };
        // **P906** — over/under de 1 carácter esticam para cobrir
        // `base_box.width` (ver `math/layout/underover.md` §P906). Anotação
        // multi-carácter (`underbrace`/`overbrace`) fica inalterada.
        let base_width_du =
            base_box.width * self.constants.upem / style.size.val().max(0.001);
        fn nested_spreader(c: &Content) -> Option<char> {
            match c {
                Content::MathUnderover(e) => e
                    .under
                    .as_ref()
                    .or(e.over.as_ref())
                    .and_then(|piece| match piece {
                        Content::MathText(s) if s.chars().count() == 1 => {
                            s.chars().next()
                        }
                        _ => nested_spreader(&e.base),
                    })
                    .or_else(|| nested_spreader(&e.base)),
                _ => None,
            }
        }
        let inherited_overflow_du = nested_spreader(base)
            .and_then(|ch| {
                self.metrics
                    .horizontal_glyph_variants(ch, style)
                    .select_variant(base_width_du)
                    .and_then(|v| {
                        v.top_accent_attach
                            .map(|attach| (attach - v.hor_advance / 2.0).abs())
                    })
            })
            .unwrap_or(0.0);
        let min_width_du = base_width_du + inherited_overflow_du;
        // **P945** — `math_size` mantido honesto: as anotações over/under
        // são scripts no vanilla (`resolve_underoverspreader`,
        // `resolve.rs:1441,1455`: under = `style_for_subscript`, over =
        // `style_for_superscript` — ambos descem um nível,
        // `style.rs:315-323`).
        // **P961** — a anotação passa a receber TAMBÉM o factor de tamanho
        // de script (a nota de P945 que adiava o factor fica revogada —
        // medido: o vanilla desenha a legenda a 0.7×; auditoria externa
        // 2026-08-04). `under` = subscrito → `cramped`; `over` =
        // superscrito → sem cramped (vanilla `style.rs:333`). A peça de 1
        // carácter (a chave que estica — acento largo no vanilla, não
        // script) é layoutada com o estilo AMBIENTE (sem redução). Ver
        // `underover.md` §P961.
        let script_math_size = match style.math_size {
            MathSize::Display | MathSize::Text => MathSize::Script,
            MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
        };
        let annotation_style = |cramped: bool| TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            math_script: true,
            math_size: script_math_size,
            cramped,
            ..style.clone()
        };
        // **P961** — discriminador (mesmo de P906): `MathText` de 1
        // carácter = peça esticável (estilo ambiente); o resto = anotação
        // (estilo de script).
        let is_single_char_piece =
            |c: &Content| matches!(c, Content::MathText(s) if s.chars().count() == 1);
        let over_box = over.map(|c| {
            if is_single_char_piece(c) {
                self.layout_stretchy_or_node(c, min_width_du, style, 0.0)
            } else {
                self.layout_node(c, &annotation_style(false))
            }
        });
        let under_box = under.map(|c| {
            if is_single_char_piece(c) {
                self.layout_stretchy_or_node(c, min_width_du, style, 0.0)
            } else {
                self.layout_node(c, &annotation_style(true))
            }
        });

        let over_w = over_box.as_ref().map(|b| b.width).unwrap_or(0.0);
        let under_w = under_box.as_ref().map(|b| b.width).unwrap_or(0.0);

        // **P1132n** — spreaders usam `exact_frame_width` no vanilla. A
        // largura e os deslocamentos horizontais são, portanto, derivados
        // dos `TopAccentAttachment` da fonte, não de centragem geométrica.
        // Isto também faz uma composição aninhada expor a sua largura real:
        // a chave exterior escolhe naturalmente a variante seguinte quando
        // o attachment da chave interior ultrapassa a caixa da base.
        fn extract_base_char(c: &Content) -> Option<char> {
            match c {
                Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
                Content::MathAccent(a) => extract_base_char(&a.base),
                _ => None,
            }
        }
        let piece_char = |c: &Content| match c {
            Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };
        let base_attach = inherited_attach.unwrap_or_else(|| {
            extract_base_char(base)
                .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
                .map(|a| a.val())
                .unwrap_or(base_box.width / 2.0)
        });

        let piece_attach =
            |content: Option<&Content>, box_: Option<&MathBox>, width: f64| {
                let glyphs: Vec<u16> = box_
                    .into_iter()
                    .flat_map(|b| &b.items)
                    .filter_map(|item| match item {
                        FrameItem::Glyph { glyph_id, .. } => Some(*glyph_id),
                        _ => None,
                    })
                    .collect();
                if glyphs.len() > 1 {
                    return width / 2.0;
                }
                let glyph_id = glyphs.first().copied();
                glyph_id
                    .and_then(|id| {
                        let ch = content.and_then(piece_char)?;
                        self.metrics
                            .horizontal_glyph_variants(ch, style)
                            .variants
                            .into_iter()
                            .find(|v| v.glyph_id == id)
                            .and_then(|v| v.top_accent_attach)
                            .map(|du| du * style.size.val() / self.constants.upem)
                    })
                    .or_else(|| {
                        content
                            .and_then(piece_char)
                            .and_then(|ch| {
                                self.metrics.top_accent_attach(ch, style.size, style)
                            })
                            .map(|a| a.val())
                    })
                    // rationale: fallback geométrico do attachment é o centro do frame, conforme P1132n/p.
                    .unwrap_or(width / 2.0)
            };
        let over_is_piece = over.map(is_single_char_piece).unwrap_or(false);
        let under_is_piece = under.map(is_single_char_piece).unwrap_or(false);
        let over_attach = piece_attach(over, over_box.as_ref(), over_w);
        let under_attach = piece_attach(under, under_box.as_ref(), under_w);

        let mut left_extension = 0.0_f64;
        let mut right_extension = 0.0_f64;
        if over_is_piece {
            let attach = over_attach;
            left_extension = left_extension.max(attach - base_attach);
            right_extension =
                right_extension.max((over_w - attach) - (base_box.width - base_attach));
        }
        if under_is_piece {
            let attach = under_attach;
            left_extension = left_extension.max(attach - base_attach);
            right_extension =
                right_extension.max((under_w - attach) - (base_box.width - base_attach));
        }
        left_extension = left_extension.max(0.0);
        right_extension = right_extension.max(0.0);

        let exact_w = left_extension + base_box.width + right_extension;
        let annotation_w = if over_is_piece { 0.0 } else { over_w }
            .max(if under_is_piece { 0.0 } else { under_w });
        let content_w = exact_w.max(annotation_w);
        let own_attachment_overflow =
            if over_is_piece { (over_attach - over_w / 2.0).abs() } else { 0.0 }.max(
                if under_is_piece { (under_attach - under_w / 2.0).abs() } else { 0.0 },
            );
        let w = content_w + 2.0 * own_attachment_overflow;
        let group_dx = (content_w - exact_w) / 2.0;
        let base_dx = group_dx + left_extension;
        let horizontal_dx = |is_over: bool, box_width: f64, is_piece: bool| {
            if is_piece {
                let attach = if is_over { over_attach } else { under_attach };
                group_dx + left_extension + base_attach - attach
            } else {
                (content_w - box_width) / 2.0
            }
        };

        // **P906** — convenção baseline-relativa (mesmo achado/correcção já
        // aplicado a `frac.rs` em P905 e `root.rs` em P901, nunca antes
        // auditado aqui): `local_y=0` é a BASELINE PRÓPRIA desta `MathBox`,
        // não o topo. A versão anterior posicionava `over`/`base`/`under`
        // por offsets crescentes a partir de `Pt(0.0)` (topo-relativo) —
        // funcionava por coincidência quando `MathUnderover` era o conteúdo
        // de TOPO da equação (P899 Parte A, `hat(a)`), porque `place()` no
        // topo cancela o termo `-ascent` independentemente da convenção
        // interna — mas quebrava assim que esta caixa fosse usada como
        // `base` de outra `MathUnderover` (caso de `underbrace`/`overbrace`
        // COM anotação, `eval.md` §P906: aninhamento de 2 níveis) ou
        // concatenada com irmãs via `hconcat_spaced`. `ascent`/`descent`
        // (valores escalares) já estavam correctos — só os offsets dos
        // items estavam errados. Ver `math/layout/underover.md` §P906.
        let mut items: Vec<FrameItem> = Vec::new();
        // Base: a sua própria baseline já é `local_y=0` — sem deslocamento.
        // rationale: P1064 Classe 1A — centragem da base sob under/over ((w - base_box.width) / 2.0)
        for item in base_box.items {
            items.push(offset_item(item, Pt(base_dx), Pt(0.0)));
        }
        // Over (topo). **P985** — peça de 1 carácter (⏞) usa a fórmula P922
        // do acento do vanilla (o spreader É um `AccentItem` no vanilla,
        // `resolve.rs:1416-1472`): `over_y = −base.ascent + min(base.ascent,
        // accent_base_height)` — a tinta do ⏞ flutua 539du acima da sua
        // baseline e o termo `min(...)` cancela essa parte oca; tight
        // stacking deixaria um gap de ~5.9pt. A legenda (multi-carácter) é
        // um anexo de LIMITE no vanilla (`ScriptsItem` top,
        // `compute_limit_shifts`, lab/typst-original/crates/typst-layout/src/math/scripts.rs:300).
        let mut over_y_opt = None;
        if let Some(ob) = over_box {
            // rationale: P1064 Classe 1A — centragem do termo superior over ((w - ob.width) / 2.0)
            let dx = horizontal_dx(true, ob.width, over_is_piece);
            let over_y = if over.map(is_single_char_piece).unwrap_or(false) {
                let abh_pt = self
                    .constants
                    .to_pt(self.constants.accent_base_height, style.size)
                    .val();
                -base_box.ascent + base_box.ascent.min(abh_pt)
            } else {
                // Legenda de cima: `base.ascent + max(upper_rise_min,
                // upper_gap_min + legenda.descent)` — tight stacking
                // deixava a legenda a tocar a chave (0.6pt vs 2.28pt).
                let rise_min = self
                    .constants
                    .to_pt(self.constants.upper_limit_baseline_rise_min, style.size)
                    .val();
                let gap_min = self
                    .constants
                    .to_pt(self.constants.upper_limit_gap_min, style.size)
                    .val();
                -(base_box.ascent + rise_min.max(gap_min + ob.descent))
            };
            over_y_opt = Some((over_y, ob.ascent));
            for item in ob.items {
                items.push(offset_item(item, Pt(dx), Pt(over_y)));
            }
        }
        // Under (fundo). Peça de 1 carácter (⏟): a sua baseline desce para
        // que o seu ascent pare no fundo da tinta da base — **P985**: com a
        // caixa medida pela tinta real (ascent de tinta do ⏟ = 0), equivale
        // ao `gap = −accent.ascent()` do vanilla. Legenda: anexo de LIMITE
        // (lab/typst-original/crates/typst-layout/src/math/scripts.rs:306).
        let mut under_y_opt = None;
        if let Some(ub) = under_box {
            // rationale: P1064 Classe 1A — centragem do termo inferior under ((w - ub.width) / 2.0)
            let dx = horizontal_dx(false, ub.width, under_is_piece);
            let under_y = if under.map(is_single_char_piece).unwrap_or(false) {
                // **P985** — `ink_up` vem COM SINAL da L3 (negativo para ⏟,
                // cuja tinta está toda abaixo da baseline): o `max(0, …)`
                // deixa a baseline da peça no fundo da tinta da base e o
                // gap conteúdo↔chave vem do bearing do próprio glifo
                // (−yMax), como no vanilla (`gap = −accent.ascent()`).
                base_box.descent + ub.ascent.max(0.0)
            } else {
                // Legenda de baixo: `base.descent + max(lower_drop_min,
                // lower_gap_min + legenda.ascent)` — tight stacking deixava
                // a legenda a tocar o ápice da chave (0pt vs 3.24pt).
                let drop_min = self
                    .constants
                    .to_pt(self.constants.lower_limit_baseline_drop_min, style.size)
                    .val();
                let gap_min = self
                    .constants
                    .to_pt(self.constants.lower_limit_gap_min, style.size)
                    .val();
                base_box.descent + drop_min.max(gap_min + ub.ascent)
            };
            under_y_opt = Some((under_y, ub.descent));
            for item in ub.items {
                items.push(offset_item(item, Pt(dx), Pt(under_y)));
            }
        }

        // **P985** — ascent/descent por `max`: a baseline da peça de cima
        // pode descer abaixo do topo da base (fórmula P922), logo
        // `base.ascent + over_h` já não é garantido.
        let ascent = match over_y_opt {
            Some((over_y, ob_ascent)) => base_box.ascent.max(-over_y + ob_ascent),
            None => base_box.ascent,
        };
        let descent = match under_y_opt {
            Some((under_y, ub_descent)) => base_box.descent.max(under_y + ub_descent),
            None => base_box.descent,
        };

        (MathBox { width: w, ascent, descent, items }, base_attach + base_dx)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
