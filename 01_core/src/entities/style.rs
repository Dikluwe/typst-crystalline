//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style.md
//! @prompt-hash 779d20ca
//! @layer L1
//! @updated 2026-04-23
//!
//! Enum `Style` — propriedades individuais de um bloco estilizado.
//!
//! Colecção `Styles` — delta de propriedades aplicado a um nó de `Content`.
//! Fundação tipada para `#set` / `#show` (Passo 99, ADR-0038). **Lote F-4
//! (P338)**: `Styles` é uma **fachada sobre `StyleDelta`** (backing único —
//! colapso da dualidade); o enum `Style` é o vocabulário-construtor.
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
use crate::entities::style_chain::StyleDelta;

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

impl Style {
    /// **Lote F-4 (P338)** — dobra esta propriedade no `StyleDelta`
    /// (last-write-wins por campo). A projeção `Style→StyleDelta` (antes em
    /// `StyleChain::push_styles`) vive aqui: `Styles` é a fachada sobre o
    /// backing único. O `match` é exaustivo — uma nova variante de `Style`
    /// obriga a tratá-la aqui.
    fn fold_into(&self, delta: &mut StyleDelta) {
        match self {
            Style::Bold(b)         => delta.bold = Some(*b),
            Style::Italic(i)       => delta.italic = Some(*i),
            Style::Size(pt)        => delta.size = Some(pt.val()),
            Style::Fill(c)         => delta.fill = Some(*c),
            Style::HeadingLevel(l) => delta.heading_level = Some(*l),
            Style::Lang(l)         => delta.lang = Some(*l),
            Style::Weight(w)       => delta.weight = Some(*w),
            Style::Tracking(l)     => delta.tracking = Some(*l),
            Style::Leading(l)      => delta.leading = Some(*l),
            // `FontList: !Copy` (Vec<FontFamily>) — clone material (P292).
            Style::Font(f)         => delta.font = Some(f.clone()),
        }
    }
}

/// **Fachada tipada sobre `StyleDelta`** (Lote F-4, P338 — colapso da dualidade
/// de backing). `Styles` armazena **um único `StyleDelta`** (o backing que a
/// `StyleChain` também usa), de modo que `Content::Styled` e a chain carregam a
/// **mesma** representação — zero conversão dupla. O enum `Style` permanece como
/// vocabulário-construtor: `from_iter`/`push` dobram cada variante no delta na
/// borda (1×). O canal `custom` (F-2) fica disponível por construção (via o
/// `StyleDelta` embrulhado), abrindo a porta para `#set` viajar no `Styled`.
#[derive(Debug, Clone, PartialEq)]
pub struct Styles {
    delta: StyleDelta,
}

impl Default for Styles {
    fn default() -> Self {
        Self::new()
    }
}

impl Styles {
    /// Colecção vazia — nenhum estilo aplicado (delta todo `None`).
    pub const fn new() -> Self {
        Self { delta: StyleDelta::empty() }
    }

    /// Constrói dobrando um iterador de `Style` no backing único.
    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {
        let mut delta = StyleDelta::empty();
        for style in iter {
            style.fold_into(&mut delta);
        }
        Self { delta }
    }

    /// Dobra um estilo no backing (last-write-wins por campo).
    pub fn push(&mut self, style: Style) {
        style.fold_into(&mut self.delta);
    }

    /// `true` se nenhuma propriedade está definida.
    pub fn is_empty(&self) -> bool {
        self.delta.is_empty()
    }

    /// O backing único — os consumidores lêem campos tipados aqui (não mais
    /// uma lista de variantes). Lote F-4 (P338).
    pub fn delta(&self) -> &StyleDelta {
        &self.delta
    }

    /// **F-realização fatia 1 (P339, L0 §3a.8)** — dobra uma entrada
    /// `(key, value)` no canal `custom` do backing. **Espelho** de
    /// `StyleChain::push_custom`: é o mecanismo que deixa `#set …(numbering:)`
    /// embrulhar o escopo num `Content::Styled` carregando o custom (transporte
    /// aditivo β1). **Não** toca o enum `Style` — o custom é `(key, value)`, por
    /// decisão da §3a.8 (resolve o fork "forma de partida" do §3b.1).
    pub fn push_custom(
        mut self,
        key: impl Into<ecow::EcoString>,
        value: crate::entities::value::Value,
    ) -> Self {
        self.delta.custom.push((key.into(), value));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styles_new_vazio() {
        // Lote F-4 (P338): `Styles` é fachada sobre `StyleDelta`; vazio = delta
        // vazio (`len()` removido — não há mais lista de variantes).
        let s = Styles::new();
        assert!(s.is_empty());
        assert!(s.delta().is_empty());
    }

    #[test]
    fn styles_push_dobra_no_delta() {
        // F-4: `push` dobra a variante no campo correspondente do delta.
        let mut s = Styles::new();
        s.push(Style::Bold(true));
        s.push(Style::Italic(false));
        assert!(!s.is_empty());
        assert_eq!(s.delta().bold, Some(true));
        assert_eq!(s.delta().italic, Some(false));
    }

    #[test]
    fn styles_from_iter_dobra_no_delta() {
        // F-4: `from_iter` dobra cada variante no backing único.
        let s = Styles::from_iter([
            Style::Bold(true),
            Style::Size(Pt(18.0)),
        ]);
        assert_eq!(s.delta().bold, Some(true));
        assert_eq!(s.delta().size, Some(18.0));
    }

    #[test]
    fn styles_from_iter_last_write_wins_por_campo() {
        // F-4: dobra é last-write-wins por campo (semântica idêntica à projeção
        // antiga via LIFO da chain).
        let s = Styles::from_iter([Style::Bold(true), Style::Bold(false)]);
        assert_eq!(s.delta().bold, Some(false), "última escrita do campo vence");
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
    fn f4_s3_trava_backing_unico_varre_10_campos() {
        // **Lote F-4 S3 (P338) — TRAVA contra o renascimento da dualidade de
        // backing.** Duas asserções pinam o backing único de ponta a ponta:
        //
        // (a) a fachada `Styles` **É** o backing: `from_iter` das 10 variantes
        //     dobra TODAS num único `StyleDelta` (não numa lista de variantes).
        // (b) `push_styles` é **definicionalmente** `push(styles.delta())` — não
        //     uma 2ª conversão. Compara, accessor a accessor, a chain via
        //     `push_styles(&s)` com a chain via `push(s.delta().clone())`.
        //
        // Se alguém reviver um `Vec<Style>` em `Styles`, ou reintroduzir um
        // `match` (conversão) divergente/parcial em `push_styles`, (a) perde um
        // campo ou (b) diverge — e a varredura quebra. É a S5a do F-4.
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::style_chain::StyleChain;
        use ecow::EcoString;

        let red = Color::rgb(255, 0, 0);
        let font = FontList::single(EcoString::from("Inter"));
        let s = Styles::from_iter([
            Style::Bold(true),
            Style::Italic(true),
            Style::Size(Pt(18.0)),
            Style::Fill(red),
            Style::HeadingLevel(3),
            Style::Lang(Lang::ENGLISH),
            Style::Weight(700),
            Style::Tracking(Length::pt(0.5)),
            Style::Leading(Length::em(0.65)),
            Style::Font(font.clone()),
        ]);

        // (a) backing único: a fachada dobrou os 10 campos no delta.
        let d = s.delta();
        assert_eq!(d.bold, Some(true));
        assert_eq!(d.italic, Some(true));
        assert_eq!(d.size, Some(18.0));
        assert_eq!(d.fill, Some(red));
        assert_eq!(d.heading_level, Some(3));
        assert_eq!(d.lang, Some(Lang::ENGLISH));
        assert_eq!(d.weight, Some(700));
        assert_eq!(d.tracking, Some(Length::pt(0.5)));
        assert_eq!(d.leading, Some(Length::em(0.65)));
        assert_eq!(d.font, Some(font.clone()));

        // (b) push_styles ≡ push(delta) — sem 2ª conversão. Idêntico em todos
        // os accessors da chain.
        let via_styles = StyleChain::default_chain().push_styles(&s);
        let via_delta = StyleChain::default_chain().push(s.delta().clone());
        assert_eq!(via_styles.bold(), via_delta.bold());
        assert_eq!(via_styles.italic(), via_delta.italic());
        assert_eq!(via_styles.size(), via_delta.size());
        assert_eq!(via_styles.fill(), via_delta.fill());
        assert_eq!(via_styles.heading_level(), via_delta.heading_level());
        assert_eq!(via_styles.lang(), via_delta.lang());
        assert_eq!(via_styles.weight(), via_delta.weight());
        assert_eq!(via_styles.tracking(), via_delta.tracking());
        assert_eq!(via_styles.leading(), via_delta.leading());
        assert_eq!(via_styles.font(), via_delta.font());
        // e o valor real propagou (não só "iguais por ambos vazios"):
        assert!(via_styles.bold() && via_styles.italic());
        assert_eq!(via_styles.size(), 18.0);
        assert_eq!(via_styles.font(), Some(font));
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
