//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style.md
//! @prompt-hash dfe68a89
//! @layer L1
//! @updated 2026-04-23
//!
//! Enum `Style` — propriedades individuais de um bloco estilizado.
//!
//! Colecção `Styles(Vec<Style>)` — delta de propriedades aplicado a um
//! nó de `Content`. Fundação tipada para `#set` / `#show` (Passo 99,
//! ADR-0038).
//!
//! Superconjunto (5 variantes no Passo 99): `Bold`, `Italic`, `Size`
//! (usadas hoje por `TextStyle`/StyleDelta), mais `Fill` e
//! `HeadingLevel` (forward-compat). Variantes adiadas (`text.font`,
//! `text.lang`, etc.) registadas em ADR-0038.
//!
//! Divergência do vanilla (ADR-0026 como precedente): enum linear
//! manual em vez de proc macros `#[elem]`.

use crate::entities::font_list::FontList;
use crate::entities::lang::Lang;
use crate::entities::layout_types::{Color, Length, Pt};

/// Uma propriedade individual de estilo. Usado em `Styles` como delta.
///
/// **P292**: `Copy` removido do derive porque `Style::Font(FontList)`
/// contém `Vec<FontFamily>` (não-Copy). `Clone + PartialEq` preserved.
/// Inventário literal P292 §A.2.0 confirma 0 call sites dependem de
/// `*style` desreferenciamento — perda de Copy é estructuralmente
/// inofensiva.
#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    /// Activa ou desactiva negrito.
    Bold(bool),
    /// Activa ou desactiva itálico.
    Italic(bool),
    /// Tamanho de fonte em pontos tipográficos.
    Size(Pt),
    /// Cor de preenchimento do texto. Forward-compat (Passo 99).
    Fill(Color),
    /// Nível de heading (1..=6). Forward-compat (Passo 99).
    HeadingLevel(u8),
    /// Lang ISO 639-1/2/3 (Passo 288 — fecha assimetria Tabela B.3 vs B.4).
    /// `StyleDelta.lang` existe desde P130/P131B/P144; até P288, escrito
    /// **apenas** por parse-driven `eval_set_rule` em
    /// `eval/rules.rs:377-394`. P288 adiciona **2ª fonte de entrada** via
    /// `Content::Styled(body, Styles::from_iter([Style::Lang(...)]))`.
    /// Caminho parse continua intacto — bit-exact preservado.
    Lang(Lang),
    /// Peso da fonte raw `u16` (Passo 289 — fecha 1/4 da assimetria
    /// residual P288 §7 risco terciário). `StyleDelta.weight: Option<u16>`
    /// existe desde P126/P129; até P289, escrito **apenas** por parse-driven
    /// `eval_set_rule` em `eval/rules.rs:350-368` (aceita `Int` ou nome
    /// simbólico via `FontWeight::from_name`). P289 adiciona **2ª fonte
    /// de entrada** via `Content::Styled(body, Styles::from_iter(
    /// [Style::Weight(700)]))`. Caminho parse continua intacto.
    /// Consumer faux-bold P139 (`TextStyle::faux_bold_stroke_pt`)
    /// reusado sem alteração — ADR-0098 aderência confirmada (hash
    /// `export.rs 66cb8ac3` preservado pelo 6º passo consecutivo).
    Weight(u16),
    /// Espaçamento adicional entre glyphs (Passo 290 — fecha 1/3 da
    /// assimetria residual P289 §5.6). `StyleDelta.tracking: Option<Length>`
    /// existe desde P127/P137; até P290, escrito **apenas** por parse-driven
    /// `eval_set_rule` em `eval/rules.rs:374`. P290 adiciona **2ª fonte de
    /// entrada** via `Content::Styled(body, Styles::from_iter(
    /// [Style::Tracking(Length::em(0.1))]))`. Caminho parse continua intacto.
    /// `Length` preserva `abs + em` (P127); consumer P137
    /// (`cursor.rs:30` tracking_extra + `export.rs:2139-2146` `Tc` operator)
    /// reusado sem alteração — ADR-0098 aderência confirmada **mesmo com
    /// emit consumer real** (paradigma `TextStyle` capture via
    /// `FrameItem::Text.style.tracking`; hash `export.rs 66cb8ac3`
    /// preservado pelo 7º passo consecutivo).
    Tracking(Length),
    /// Espaço entre linhas (Passo 291 — fecha 1/2 da assimetria residual
    /// P290 §5.6; resta apenas `font`). `StyleDelta.leading: Option<Length>`
    /// existe desde P128/P138; até P291, escrito **apenas** por parse-driven
    /// `eval_set_rule` em `eval/rules.rs:298`. P291 adiciona **2ª fonte de
    /// entrada** via `Content::Styled(body, Styles::from_iter(
    /// [Style::Leading(Length::em(0.65))]))`. Caminho parse continua intacto.
    ///
    /// **Paradigma consumer distinto vs Tracking** (per-line, não per-glyph):
    /// `cursor.rs:119-128` em `flush_line` faz peek do último
    /// `FrameItem::Text` da `current_line` (`iter().rev().find_map`) — lê
    /// `style.leading.resolve_pt(font_size)` e adiciona `line_height +
    /// leading_pt` a `cursor_y` antes do drain. `export.rs` zero hits para
    /// `leading` (paradigma ADR-0098 vigente; hash `66cb8ac3` preservado
    /// pelo 8º passo consecutivo).
    ///
    /// **Divergência arquitectural consciente**: vanilla typst tem `leading`
    /// em `par`; cristalino captura em `text` por conveniência temporária
    /// (Tabela A.3 linha 70 + 184; sem `Content::Par` propriamente).
    /// P291 preserva esta divergência (não-objectivo §5 spec P291).
    Leading(Length),
    /// Família/lista de fontes (Passo 292 — **fecha 1/1 final da
    /// assimetria residual P289 §5.6**; pós-P292 série cirúrgica P288-P292
    /// termina naturalmente — assimetria 5/5 fechada). `StyleDelta.font:
    /// Option<FontList>` existe desde P140B/P141/P146; até P292, escrito
    /// **apenas** por parse-driven `eval_set_rule` (`#set text(font: "Inter")`,
    /// `font: ("Inter", "Arial")`, ou `font: (name: ..., covers: ...)`
    /// scope-out ADR-0054bis). P292 adiciona **2ª fonte de entrada** via
    /// `Content::Styled(body, Styles::from_iter(
    /// [Style::Font(FontList::single("Inter"))]))`. Caminho parse continua
    /// intacto.
    ///
    /// **Distintivo arquitectural vs P288-P291**: `FontList` é
    /// `pub struct FontList(Vec<FontFamily>)` — **não é `Copy`**. Decisão
    /// A.2 P292 escolheu opção (a) `Font(FontList)` apesar disso — **`Style`
    /// enum perde `Copy` derive** (inventário A.2.0 confirma 0 call sites
    /// dependem; perda inofensiva). Cascade arm usa `f.clone()` em vez de
    /// `*f` por necessidade material (paralelo a `chain.font()` que
    /// clona em walk up-the-chain — comentário `style_chain.rs:264`).
    ///
    /// **Paradigma consumer "indirect resolution via FontBook"**:
    /// `cursor.rs` capture `style.font` em `FrameItem::Text`; emit
    /// (`export.rs:2169-2174` multifont path) consulta `style.font.as_ref()`
    /// + `fonts.iter().position(|f| /* match name */)` → `/F{i+1} Tf`
    /// (PDF font select). Paradigma adicional não presente em P288-P291:
    /// **2 layers** (resolução nome → index via lookup, depois emit).
    /// ADR-0098 vigente (paradigma `TextStyle` capture preservado;
    /// hash `export.rs 66cb8ac3` preservado pelo 9º passo consecutivo).
    Font(FontList),
}

/// Colecção de `Style` — delta de propriedades aplicado a um nó.
///
/// Usado em `Content::Styled` e para construir `StyleChain` via `push_styles`.
/// Invariante: a ordem dos estilos preserva a de inserção; a resolução
/// num `StyleChain` privilegia o valor mais recente em cada variante.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Styles {
    inner: Vec<Style>,
}

impl Styles {
    /// Colecção vazia — sem estilos aplicados.
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Constrói a partir de um iterador de `Style`.
    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {
        Self { inner: iter.into_iter().collect() }
    }

    /// Adiciona um estilo à colecção.
    pub fn push(&mut self, style: Style) {
        self.inner.push(style);
    }

    /// Iterador sobre os estilos, pela ordem de inserção.
    pub fn iter(&self) -> std::slice::Iter<'_, Style> {
        self.inner.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styles_new_vazio() {
        let s = Styles::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn styles_push_e_iter() {
        let mut s = Styles::new();
        s.push(Style::Bold(true));
        s.push(Style::Italic(false));
        assert_eq!(s.len(), 2);
        let collected: Vec<_> = s.iter().collect();
        assert_eq!(collected.len(), 2);
        assert!(matches!(collected[0], Style::Bold(true)));
        assert!(matches!(collected[1], Style::Italic(false)));
    }

    #[test]
    fn styles_from_iter() {
        let s = Styles::from_iter([
            Style::Bold(true),
            Style::Size(Pt(18.0)),
        ]);
        assert_eq!(s.len(), 2);
        assert!(s.iter().any(|st| matches!(st, Style::Bold(true))));
        assert!(s.iter().any(|st| matches!(st, Style::Size(p) if p.val() == 18.0)));
    }

    #[test]
    fn styles_eq() {
        let a = Styles::from_iter([Style::Bold(true)]);
        let b = Styles::from_iter([Style::Bold(true)]);
        let c = Styles::from_iter([Style::Bold(false)]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn style_variantes_cobrem_catalog_99a_e_p288_a_p292() {
        // Passo 99.A inaugurou 5 variantes. P288 adicionou `Lang(Lang)` → 6.
        // P289 adicionou `Weight(u16)` → 7. P290 adicionou `Tracking(Length)`
        // → 8. P291 adicionou `Leading(Length)` → 9. P292 adiciona
        // `Font(FontList)` → 10 (**fecha série cumulativa P288-P292**).
        // Este teste falha se alguém tentar remover uma.
        use crate::entities::font_list::FontList;
        use crate::entities::lang::Lang;
        use ecow::EcoString;
        let variants = [
            Style::Bold(true),
            Style::Italic(false),
            Style::Size(Pt(12.0)),
            Style::Fill(Color::rgb(0, 0, 0)),
            Style::HeadingLevel(1),
            Style::Lang(Lang::ENGLISH),  // P288
            Style::Weight(700),           // P289
            Style::Tracking(Length::pt(0.5)),  // P290
            Style::Leading(Length::em(0.65)),   // P291
            Style::Font(FontList::single(EcoString::from("Inter"))),  // P292
        ];
        assert_eq!(variants.len(), 10);
    }
}
