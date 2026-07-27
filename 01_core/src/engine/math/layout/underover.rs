//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/underover.md
//! @prompt-hash 8375a61a
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_underover` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P297).

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
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
        let over_box = over.map(|c| self.layout_stretchy_or_node(c, min_width_du, style));
        let under_box = under.map(|c| self.layout_stretchy_or_node(c, min_width_du, style));

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
