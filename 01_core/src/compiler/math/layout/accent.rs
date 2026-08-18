//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/accent.md
//! @prompt-hash 8351cea7
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
        let one_char = |c: &Content| match c {
            Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };
        let base_attach = one_char(base)
            .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
            .map(|a| a.val())
            // rationale: P1064 Classe 1A — ponto médio da base para acento (base_box.width / 2.0)
            .unwrap_or(base_box.width / 2.0);
        let accent_attach = one_char(accent)
            .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
            .map(|a| a.val())
            // rationale: P1064 Classe 1A — ponto médio do glifo de acento (accent_box.width / 2.0)
            .unwrap_or(accent_box.width / 2.0);
        let dx = base_attach - accent_attach;
        let accent_base_height_pt =
            self.constants.to_pt(self.constants.accent_base_height, style.size).val();
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
        // descent pare `gap` acima do topo da tinta da base.
        // Derivado de: accent_y + signed_descent = -base_box.ascent - gap
        // => accent_y = -base_box.ascent - gap - signed_descent
        // Substituindo gap: accent_y = -base_box.ascent + base_box.ascent.min(accent_base_height_pt).
        let accent_y = -base_box.ascent + base_box.ascent.min(accent_base_height_pt);
        for item in accent_box.items {
            items.push(offset_item(item, Pt(dx), Pt(accent_y)));
        }
        // **P989** — `new_ascent` = topo da tinta acima da baseline:
        // `max(base.ascent, −accent_y + accent.ascent)` — algebricamente
        // idêntico à fórmula aditiva do vanilla
        // (`base.ascent + accent.height + gap`) quando `accent.height` é a
        // altura de tinta com sinal, mas dispensa o `signed_descent` (que
        // a L3 devolvia com o sinal invertido até P989 — achado §8.2, o
        // segundo ponto de `dot(dot(x))` caía em cima do primeiro porque a
        // caixa interna não carregava o topo da tinta do acento). Ver
        // `accent.md` §P989.
        let new_ascent = base_box.ascent.max(-accent_y + accent_box.ascent);
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
