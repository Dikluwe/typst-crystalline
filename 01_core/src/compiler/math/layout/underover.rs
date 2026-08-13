//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/underover.md
//! @prompt-hash db7920cf
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
        let base_box = self.layout_node(base, style);
        // **P906** — over/under de 1 carácter esticam para cobrir
        // `base_box.width` (ver `math/layout/underover.md` §P906). Anotação
        // multi-carácter (`underbrace`/`overbrace`) fica inalterada.
        let min_width_du =
            base_box.width * self.constants.upem / style.size.val().max(0.001);
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
        let w = base_box.width.max(over_w).max(under_w);

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
        let base_dx = (w - base_box.width) / 2.0;
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
        // `compute_limit_shifts`, `scripts.rs:300-304`).
        let mut over_y_opt = None;
        if let Some(ob) = over_box {
            let dx = (w - ob.width) / 2.0;
            let over_y = if over.map(is_single_char_piece).unwrap_or(false) {
                let abh_pt =
                    self.constants.to_pt(self.constants.accent_base_height, style.size).val();
                -base_box.ascent + base_box.ascent.min(abh_pt)
            } else {
                // Legenda de cima: `base.ascent + max(upper_rise_min,
                // upper_gap_min + legenda.descent)` — tight stacking
                // deixava a legenda a tocar a chave (0.6pt vs 2.28pt).
                let rise_min = self
                    .constants
                    .to_pt(self.constants.upper_limit_baseline_rise_min, style.size)
                    .val();
                let gap_min =
                    self.constants.to_pt(self.constants.upper_limit_gap_min, style.size).val();
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
        // (`scripts.rs:306-311`).
        let mut under_y_opt = None;
        if let Some(ub) = under_box {
            let dx = (w - ub.width) / 2.0;
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
                let gap_min =
                    self.constants.to_pt(self.constants.lower_limit_gap_min, style.size).val();
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

        MathBox {
            width: w,
            ascent,
            descent,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
