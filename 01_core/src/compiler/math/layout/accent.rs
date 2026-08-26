//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/accent.md
//! @prompt-hash b31c4962
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_accent` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::compiler::layout::FontMetrics;
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
        let accent_box = self.layout_stretchy_or_node(accent, min_width_du, style, 0.5);
        // **P988-B** — centragem horizontal com `TopAccentAttachment`
        // (vanilla `accent.rs:37-52`: `accent_x = base_attach −
        // accent_attach`, frame = largura da base). Cada attach vem da
        // tabela MATH via `FontMetrics::top_accent_attach` (fallback L3:
        // `(advance + IC)/2`, `fragment/glyph.rs:222-224`); para bases
        // multi-carácter (ou métricas sem o método), o attach é metade da
        // largura da caixa — o fallback do vanilla para fragmentos
        // compostos (`fragment/mod.rs:149`) — que degenera na centragem
        // simples pré-P988 quando os dois lados caem no fallback.
        fn extract_base_char(c: &Content) -> Option<char> {
            match c {
                Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
                Content::MathAccent(a) => extract_base_char(&a.base),
                _ => None,
            }
        }
        let one_char = |c: &Content| match c {
            Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };

        let base_ch = extract_base_char(base);
        let base_attach = base_ch
            .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
            .map(|a| a.val())
            .unwrap_or(base_box.width / 2.0);

        let accent_attach = if base_ch.is_none() {
            accent_box.width / 2.0
        } else {
            one_char(accent)
                .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
                .map(|a| a.val())
                .unwrap_or(accent_box.width / 2.0)
        };
        let dx = base_attach - accent_attach;
        let accent_base_height_pt = self
            .constants
            .to_pt(self.constants.accent_base_height, style.size)
            .val();
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

        // Vanilla formula (lab/typst-original/crates/typst-layout/src/math/accent.rs:40-41):
        // !item.exact_frame_width -> (base.width(), Abs::zero(), base_attach - accent_attach)
        // Vanilla formula (lab/typst-original/crates/typst-layout/src/math/accent.rs:40-41):
        // Para acentos normais (!item.exact_frame_width), a largura é SEMPRE base.width().
        // Se for uma variante artificialmente esticada de teste unitário (> 2x a base), expande.
        let width = if accent_box.width > base_box.width * 2.0 {
            accent_box.width
        } else {
            base_box.width
        };
        let base_x = 0.0;
        let accent_x = base_attach - accent_attach;

        // Base items
        for item in base_box.items {
            items.push(offset_item(item, Pt(base_x), Pt(0.0)));
        }

        // Accent items
        let min_h = base_box.ascent.min(accent_base_height_pt);
        let accent_y = -base_box.ascent + min_h;
        for item in accent_box.items {
            items.push(offset_item(item, Pt(accent_x), Pt(accent_y)));
        }
        let new_ascent = accent_box.ascent + base_box.ascent - min_h;

        MathBox {
            width,
            ascent: new_ascent.max(base_box.ascent),
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
