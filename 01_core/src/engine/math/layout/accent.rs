//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/accent.md
//! @prompt-hash bf7cde11
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_accent` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
};

use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P296** — Posiciona `accent` glyph centrado horizontalmente
    /// acima de `base`. Heurística minimal per ADR-0054 graded:
    /// - Sem `dotless` (i/j) handling — base mantém glyph original.
    /// - Sem `size` ratio — accent é width natural.
    pub(super) fn layout_accent(
        &self,
        base: &Content,
        accent: &Content,
        style: &TextStyle,
    ) -> MathBox {
        // **P915** — base é cramped, incondicional (o cristalino só
        // implementa accent "acima" — `MathAccentElem` sem campo de
        // posição — logo a condição do vanilla "só se accent.is_bottom()
        // == false" é trivialmente sempre verdadeira aqui). Achado do
        // vanilla: "the base is resolved in cramped style if the accent is
        // above" (`resolve_accent`, `resolve.rs:362-373`). Ver `accent.md`
        // §P915.
        let base_style = TextStyle { cramped: true, ..style.clone() };
        let base_box = self.layout_node(base, &base_style);
        // **P906** — accent de 1 carácter estica para cobrir `base_box.width`
        // (ver `math/layout/accent.md` §P906). Multi-carácter: inalterado.
        let min_width_du =
            base_box.width * self.constants.upem / style.size.val().max(0.001);
        let accent_box = self.layout_stretchy_or_node(accent, min_width_du, style);
        // Centrar accent horizontalmente. dx é deslocamento do accent
        // para alinhar centro do accent com centro da base.
        let dx = (base_box.width - accent_box.width) / 2.0;
        let accent_h = accent_box.height();
        let new_ascent = base_box.ascent + accent_h;
        // **P906** — convenção baseline-relativa (mesmo achado/correcção já
        // aplicado a `frac.rs` P905, `root.rs` P901 e `layout_underover`,
        // ver `math/layout/accent.md` §P906): `local_y=0` é a BASELINE
        // PRÓPRIA desta `MathBox`. A versão anterior posicionava accent/base
        // por offsets topo-relativos — funcionava por coincidência no topo
        // da equação (P899 Parte A), quebrava se esta caixa fosse usada
        // como sub-caixa de outra (`hconcat_spaced`, ou aninhada como base
        // de outro `MathUnderover`/`MathAccent`). `ascent`/`descent` já
        // correctos — só os offsets dos items mudam.
        let mut items: Vec<FrameItem> = Vec::new();
        // Base: a sua própria baseline já é `local_y=0` — sem deslocamento.
        for item in base_box.items {
            items.push(offset_item(item, Pt(0.0), Pt(0.0)));
        }
        // Accent: a sua baseline própria sobe o suficiente para que o seu
        // descent pare exactamente no topo da tinta da base.
        let accent_y = -(base_box.ascent + accent_box.descent);
        for item in accent_box.items {
            items.push(offset_item(item, Pt(dx), Pt(accent_y)));
        }
        MathBox {
            width: base_box.width.max(accent_box.width),
            ascent: new_ascent,
            descent: base_box.descent,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
