//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style.md
//! @prompt-hash 87396557
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

use crate::entities::lang::Lang;
use crate::entities::layout_types::{Color, Pt};

/// Uma propriedade individual de estilo. Usado em `Styles` como delta.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn style_variantes_cobrem_catalog_99a_e_p288_e_p289() {
        // Passo 99.A inaugurou 5 variantes. P288 adicionou `Lang(Lang)` → 6.
        // P289 adiciona `Weight(u16)` → 7. Este teste falha se alguém
        // tentar remover uma.
        use crate::entities::lang::Lang;
        let variants = [
            Style::Bold(true),
            Style::Italic(false),
            Style::Size(Pt(12.0)),
            Style::Fill(Color::rgb(0, 0, 0)),
            Style::HeadingLevel(1),
            Style::Lang(Lang::ENGLISH),  // P288
            Style::Weight(700),           // P289
        ];
        assert_eq!(variants.len(), 7);
    }
}
