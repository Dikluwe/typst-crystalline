//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/underover.md
//! @prompt-hash 4af75b04
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_underover` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P297).

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Pt, TextStyle},
};

use super::{offset_item, stack_tight_above, MathBox};

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

        let over_h = over_box.as_ref().map(|b| b.height()).unwrap_or(0.0);
        let under_h = under_box.as_ref().map(|b| b.height()).unwrap_or(0.0);

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
        // Over (topo): a sua baseline própria sobe o suficiente para que o
        // seu descent pare exactamente no topo da tinta da base.
        if let Some(ob) = over_box {
            let dx = (w - ob.width) / 2.0;
            let over_y = stack_tight_above(base_box.ascent, ob.descent);
            for item in ob.items {
                items.push(offset_item(item, Pt(dx), Pt(over_y)));
            }
        }
        // Under (fundo): a sua baseline própria desce o suficiente para que
        // o seu ascent pare exactamente no fundo da tinta da base.
        if let Some(ub) = under_box {
            let dx = (w - ub.width) / 2.0;
            let under_y = base_box.descent + ub.ascent;
            for item in ub.items {
                items.push(offset_item(item, Pt(dx), Pt(under_y)));
            }
        }

        MathBox {
            width: w,
            ascent: base_box.ascent + over_h,
            descent: base_box.descent + under_h,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
