//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style_chain.md
//! @prompt-hash 09cb9de2
//! @layer L1
//! @updated 2026-07-03
//!
//! Passo 99 (ADR-0038): `StyleChain` aceita uma colecção `Styles` via
//! `push_styles`. **Lote F-4 (P338): a dualidade de backing colapsou** —
//! `Styles` passou a ser uma **fachada sobre `StyleDelta`** (o backing único
//! desta chain; ver `style.rs`). Já não há duas representações: `push_styles`
//! empurra o `delta()` da fachada (sem reconversão), e `StyleDelta` é o backing
//! único lido pelos accessors **e** carregado por `Content::Styled`. A
//! coexistência "até migrar" terminou (o canal `custom` do F-2 fica disponível
//! no `Styled` para F-realização/F-5).
//!
//! **P554** — fonte por defeito do bridge `From<&StyleChain> for TextStyle`
//! alterada de `Helvetica` para `FreeSerif`, uma serif amplamente disponível
//! em sistemas Linux, para paridade visual e de paginação com o vanilla.
//! **P558** — corrigida para `Liberation Serif` para evitar acentos trocados
//! no PDF quando `FreeSerif` descompõe caracteres acentuados em base + mark.
//! **P753** — corrigida para `Libertinus Serif` para bater com o vanilla 0.15.0,
//! agora que o cristalino carrega as mesmas fontes embutidas via typst-assets.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::font_list::FontList;
use crate::entities::lang::Lang;
use crate::entities::layout_types::{Pt, TextStyle};
use crate::entities::style::{Style, Styles};
use crate::entities::value::Value;

/// Um delta de estilo — apenas as propriedades que este nó define explicitamente.
/// Propriedades ausentes são herdadas do nó pai na cadeia.
///
/// Passo 99 (ADR-0038): adicionados `fill` e `heading_level` para suportar
/// as variantes forward-compat do enum `Style`. Os accessors antigos
/// (`bold()`, `italic()`, `size()`) continuam a ignorar estes campos.
/// Passo 126 (ADR-0038 anotada): `weight` adicionado como primeira
/// propriedade numérica; capturado por `#set text(weight: N)` mas
/// ainda não consumido por layout (inerte).
#[derive(Debug, Clone, PartialEq)]
pub struct StyleDelta {
    pub bold: Option<bool>,
    /// Origem do negrito: `Some(true)` para `*bold*` / `Content::Strong`;
    /// `Some(false)` para `#set text(bold)` (DEBT-50).
    pub bold_from_strong: Option<bool>,
    pub italic: Option<bool>,
    /// Origem do itálico: `Some(true)` para `_italic_` / `Content::Emph`;
    /// `Some(false)` para `#set text(italic)`.
    pub italic_from_emph: Option<bool>,
    pub size: Option<f64>, // em pontos tipográficos
    /// Cor de preenchimento do texto (Passo 99, ADR-0038, forward-compat).
    pub fill: Option<crate::entities::layout_types::Color>,
    /// Nível de heading quando aplicado via `#set heading(level: N)` futuro
    /// (Passo 99, ADR-0038, forward-compat).
    pub heading_level: Option<u8>,
    /// Peso da fonte (Passo 126, ADR-0038). Valor raw `u16` (CSS/OpenType
    /// 0-1000). Capturado pelo eval mas ainda inerte em layout.
    pub weight: Option<u16>,
    /// Espaçamento adicional entre glyphs (Passo 127, ADR-0038). Preserva
    /// `Length` inteiro (`abs + em`); resolve para pt quando consumer
    /// conhecer font-size. Inerte em layout.
    pub tracking: Option<crate::entities::layout_types::Length>,
    /// Espaço entre linhas (Passo 128, DEBT-1 subset). Em vanilla é
    /// propriedade de `par` (não de `text`); capturado em `#set text`
    /// por conveniência temporária — migra para `eval_set_par` quando
    /// este for activado. Inerte em layout.
    pub leading: Option<crate::entities::layout_types::Length>,
    /// **P762** — bordo superior da linha (`top-edge`). **P837**: métrica
    /// nomeada ou `Length` explícito (`TextEdge`).
    pub top_edge: Option<crate::entities::layout_types::TextEdge>,
    /// **P762** — bordo inferior da linha (`bottom-edge`). **P837**:
    /// métrica nomeada ou `Length` explícito (`TextEdge`).
    pub bottom_edge: Option<crate::entities::layout_types::TextEdge>,
    /// Identificador de língua (ISO 639-1/2/3). `None` = herdado.
    /// Capturado inicialmente como `EcoString` raw no Passo 130;
    /// materializado como tipo semântico `Lang` no Passo 131B
    /// (ADR-0052) com validação e erro hard — paridade ADR-0033.
    /// Inerte em layout (consumer futuro: shaping, hyphenation).
    pub lang: Option<Lang>,
    /// Lista priorizada de famílias de fonte. `None` = herdado.
    /// Materializado no Passo 132B (ADR-0053) com paridade
    /// parcial: string e array capturadas, dict rejeitada
    /// (covers sem suporte até `regex` ser autorizado em L1).
    /// Inerte em layout (consumer futuro: shaping, lookup).
    pub font: Option<FontList>,
    /// **Passo 448 (P448)**: subscrito (`#sub[...]`).
    pub subscript: Option<bool>,
    /// **Passo 448 (P448)**: sobrescrito (`#super[...]`).
    pub superscript: Option<bool>,
    /// **Passo 449 (P449)**: cor de fundo do highlight (`#highlight[...]`).
    /// `None` no `Option` externo = não definido; `Some(None)` = desactivado;
    /// `Some(Some(Color))` = cor activa.
    pub highlight: Option<Option<crate::entities::layout_types::Color>>,
    /// **P471**: raio dos cantos do highlight. `None` = rect sem arredondamento.
    pub highlight_radius: Option<crate::entities::layout_types::Length>,
    /// **P471**: extensão horizontal do highlight. `None` = sem extensão.
    pub highlight_extent: Option<crate::entities::layout_types::Length>,
    /// **P471**: tamanho explícito do corpo de subscrito. `None` = 65% do font-size.
    pub subscript_size: Option<crate::entities::layout_types::Length>,
    /// **P471**: tamanho explícito do corpo de sobrescrito. `None` = 65% do font-size.
    pub superscript_size: Option<crate::entities::layout_types::Length>,
    /// **Canal aberto das `Set*` (Lote F-2, P335)** — propriedades não-texto
    /// dinâmicas resolvidas por chave (`PropKey → Value`), ao lado das 13
    /// nativas fechadas. Eixo do **DEBT 99.E**: as `Set*` (numbering de
    /// heading/equation/figure, dims de page) entram aqui em vez de canais
    /// dispersos (Introspector/`page_config`/baking), ganhando **escopo léxico**
    /// de graça (o `engine.styles` é escopado via `local_styles`). Chave é
    /// `EcoString` (clone O(1)); valor é o `Value` fechado (espelho da
    /// linguagem). Vazio na maioria dos nós (custo marginal só onde há `#set`).
    pub custom: Vec<(EcoString, Value)>,
}

impl StyleDelta {
    pub const fn empty() -> Self {
        Self {
            bold: None,
            bold_from_strong: None,
            italic: None,
            italic_from_emph: None,
            size: None,
            fill: None,
            heading_level: None,
            weight: None,
            tracking: None,
            leading: None,
            top_edge: None,
            bottom_edge: None,
            lang: None,
            font: None,
            subscript: None,
            superscript: None,
            highlight: None,
            highlight_radius: None,
            highlight_extent: None,
            subscript_size: None,
            superscript_size: None,
            custom: Vec::new(),
        }
    }

    /// `true` se nenhuma propriedade está definida (todos os campos `None` e
    /// `custom` vazio). Lote F-4 (P338): backing de `Styles::is_empty` — `Styles`
    /// é fachada sobre `StyleDelta` (colapso da dualidade).
    pub fn is_empty(&self) -> bool {
        self.bold.is_none()
            && self.bold_from_strong.is_none()
            && self.italic.is_none()
            && self.italic_from_emph.is_none()
            && self.size.is_none()
            && self.fill.is_none()
            && self.heading_level.is_none()
            && self.weight.is_none()
            && self.tracking.is_none()
            && self.leading.is_none()
            && self.top_edge.is_none()
            && self.bottom_edge.is_none()
            && self.lang.is_none()
            && self.font.is_none()
            && self.subscript.is_none()
            && self.superscript.is_none()
            && self.highlight.is_none()
            && self.highlight_radius.is_none()
            && self.highlight_extent.is_none()
            && self.subscript_size.is_none()
            && self.superscript_size.is_none()
            && self.custom.is_empty()
    }

    /// Constroi uma `Styles` com apenas os campos que diferem de `other`.
    /// Usado em `eval_markup` (P431) para embrulhar a cauda de um `#set` no
    /// delta exacto que esse `#set` introduziu, preservando a origem
    /// (`from_strong`/`from_emph`) quando aplicável.
    pub fn diff_styles(&self, other: &StyleDelta) -> Styles {
        let mut styles = Styles::new();
        if self.bold != other.bold || self.bold_from_strong != other.bold_from_strong {
            styles.push(Style::Bold {
                value: self.bold.unwrap_or(false),
                from_strong: self.bold_from_strong.unwrap_or(false),
            });
        }
        if self.italic != other.italic || self.italic_from_emph != other.italic_from_emph
        {
            styles.push(Style::Italic {
                value: self.italic.unwrap_or(false),
                from_emph: self.italic_from_emph.unwrap_or(false),
            });
        }
        if self.size != other.size {
            styles.push(Style::Size(Pt(self.size.unwrap_or(0.0))));
        }
        if self.fill != other.fill {
            if let Some(c) = self.fill {
                styles.push(Style::Fill(c));
            }
        }
        if self.heading_level != other.heading_level {
            if let Some(l) = self.heading_level {
                styles.push(Style::HeadingLevel(l));
            }
        }
        if self.weight != other.weight {
            if let Some(w) = self.weight {
                styles.push(Style::Weight(w));
            }
        }
        if self.tracking != other.tracking {
            if let Some(l) = self.tracking.clone() {
                styles.push(Style::Tracking(l));
            }
        }
        if self.leading != other.leading {
            if let Some(l) = self.leading.clone() {
                styles.push(Style::Leading(l));
            }
        }
        if self.lang != other.lang {
            if let Some(l) = self.lang.clone() {
                styles.push(Style::Lang(l));
            }
        }
        if self.font != other.font {
            if let Some(f) = self.font.clone() {
                styles.push(Style::Font(f));
            }
        }
        if self.subscript != other.subscript {
            styles.push(Style::Subscript(self.subscript.unwrap_or(false)));
        }
        if self.superscript != other.superscript {
            styles.push(Style::Superscript(self.superscript.unwrap_or(false)));
        }
        if self.highlight != other.highlight {
            styles.push(Style::highlight(self.highlight.unwrap_or(None)));
        }
        if self.highlight_radius != other.highlight_radius {
            if let Some(r) = self.highlight_radius {
                styles.push(Style::highlight_radius(r));
            }
        }
        if self.highlight_extent != other.highlight_extent {
            if let Some(e) = self.highlight_extent {
                styles.push(Style::highlight_extent(e));
            }
        }
        if self.subscript_size != other.subscript_size {
            if let Some(s) = self.subscript_size {
                styles.push(Style::subscript_size(s));
            }
        }
        if self.superscript_size != other.superscript_size {
            if let Some(s) = self.superscript_size {
                styles.push(Style::superscript_size(s));
            }
        }
        for (k, v) in &self.custom {
            let changed = other
                .custom
                .iter()
                .find(|(ok, _)| ok == k)
                .map(|(_, ov)| ov != v)
                .unwrap_or(true);
            if changed {
                styles = styles.push_custom(k.clone(), v.clone());
            }
        }
        styles
    }
}

/// Nó interno da lista ligada.
#[derive(Debug, Clone)]
struct StyleNode {
    delta: StyleDelta,
    parent: Option<Arc<StyleNode>>,
}

/// Lista ligada imutável de deltas de estilo.
///
/// Clone é O(1) — apenas o `Arc` do nó de topo é clonado.
/// Leitura é O(N) percorrendo a cadeia até encontrar o primeiro delta
/// que define a propriedade (N tipicamente < 10).
///
/// Equivalente simplificado de `StyleChain` do Typst original.
/// Suporta `#set text(bold: true)` e herança em blocos aninhados.
///
/// Pureza L1: não usa I/O de sistema — apenas `Arc` e `Vec` em memória.
#[derive(Debug, Clone)]
pub struct StyleChain(Option<Arc<StyleNode>>);

impl StyleChain {
    /// Cadeia vazia — resolve para os defaults codificados em cada accessor.
    pub const fn empty() -> Self {
        StyleChain(None)
    }

    /// Cadeia com os valores por defeito do motor Typst.
    /// bold: false, italic: false, size: 11.0pt.
    /// **P1034** — `lang` e `figure.numbering` têm defaults de linguagem
    /// aplicados pelos **consumidores**, não bakados na cadeia: bakar aqui
    /// quebraria a detecção de fronteiras de `#set` via `collapse()`.
    pub fn default_chain() -> Self {
        let root = StyleNode {
            delta: StyleDelta {
                bold: Some(false),
                italic: Some(false),
                size: Some(11.0),
                ..StyleDelta::empty()
            },
            parent: None,
        };
        StyleChain(Some(Arc::new(root)))
    }

    /// Cria uma nova cadeia que herda desta e aplica `delta` por cima.
    /// Custo: O(1) — cria um novo `Arc`.
    pub fn push(&self, delta: StyleDelta) -> Self {
        let node = StyleNode { delta, parent: self.0.clone() };
        StyleChain(Some(Arc::new(node)))
    }

    /// Cria uma nova cadeia aplicando `styles` como delta. Passo 99 (ADR-0038)
    /// — entrada tipada para `Content::Styled`.
    ///
    /// **Lote F-4 (P338): `Styles` é fachada sobre `StyleDelta`** (backing
    /// único — colapso da dualidade). A projeção `Style→StyleDelta` (o `match`
    /// exaustivo das 10 variantes) mudou-se para `Styles::from_iter`/`push`;
    /// aqui só empurramos o `delta` já dobrado — **zero conversão dupla**.
    pub fn push_styles(&self, styles: &Styles) -> Self {
        self.push(styles.delta().clone())
    }

    /// **Canal aberto (Lote F-2, P335)** — empurra uma propriedade `Set*`
    /// `(key → value)` como novo delta. Escopo léxico de graça (o `engine.styles`
    /// é escopado por `local_styles`). Convenção de chave: `"heading.numbering"`,
    /// `"equation.numbering"`, `"figure.numbering"`, `"page.width/height/margin"`.
    pub fn push_custom(&self, key: impl Into<EcoString>, value: Value) -> Self {
        self.push(StyleDelta {
            custom: vec![(key.into(), value)],
            ..StyleDelta::empty()
        })
    }

    /// **Show-set (P352)** — dobra **todos** os nós da cadeia num único
    /// `StyleDelta`, preservando a semântica `Option` (top-wins por campo; o
    /// `custom` mantém a primeira ocorrência por chave). Read-only.
    ///
    /// Usado pela captura do show-set (`#show k: set …`): a cadeia é construída
    /// sobre `StyleChain::empty()` (não `default_chain()`), logo o resultado é
    /// **apenas** o que o `#set` definiu — sem os defaults bold/italic/size. Os
    /// consumidores (`Content::Styled`) recebem o efeito exato do `set`.
    pub fn collapse(&self) -> StyleDelta {
        let mut out = StyleDelta::empty();
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            let d = &n.delta;
            if out.bold.is_none() {
                out.bold = d.bold;
            }
            if out.bold_from_strong.is_none() {
                out.bold_from_strong = d.bold_from_strong;
            }
            if out.italic.is_none() {
                out.italic = d.italic;
            }
            if out.italic_from_emph.is_none() {
                out.italic_from_emph = d.italic_from_emph;
            }
            if out.size.is_none() {
                out.size = d.size;
            }
            if out.fill.is_none() {
                out.fill = d.fill;
            }
            if out.heading_level.is_none() {
                out.heading_level = d.heading_level;
            }
            if out.weight.is_none() {
                out.weight = d.weight;
            }
            if out.tracking.is_none() {
                out.tracking = d.tracking;
            }
            if out.leading.is_none() {
                out.leading = d.leading;
            }
            if out.lang.is_none() {
                out.lang = d.lang.clone();
            }
            if out.font.is_none() {
                out.font = d.font.clone();
            }
            for (k, v) in &d.custom {
                if !out.custom.iter().any(|(ek, _)| ek == k) {
                    out.custom.push((k.clone(), v.clone()));
                }
            }
            node = n.parent.as_deref();
        }
        out
    }

    /// **Canal aberto (Lote F-2, P335)** — resolve uma propriedade `Set*` por
    /// chave, percorrendo a cadeia até ao primeiro nó que a define (top-wins,
    /// fallback léxico). `None` = não definida em nenhum nível.
    pub fn custom(&self, key: &str) -> Option<&Value> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some((_, v)) = n.delta.custom.iter().find(|(k, _)| k == key) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `fill` (cor de texto) percorrendo a cadeia até ao primeiro
    /// delta que o define. Forward-compat — ainda não consumido pelo
    /// Layouter/export (ver DEBT-Style sucessor em DEBT.md).
    pub fn fill(&self) -> Option<crate::entities::layout_types::Color> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.fill {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `heading_level` percorrendo a cadeia até ao primeiro delta
    /// que o define. Forward-compat — o AST ainda representa headings via
    /// `Content::Heading{level, ..}` directamente.
    pub fn heading_level(&self) -> Option<u8> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.heading_level {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text`/`#set par`
    // deixou de ser assado no `Content::Text` — viaja no canal `custom`
    // (`"text.<campo>"` / `"par.leading"`). Para que `TextStyle::from(&chain)` —
    // e portanto o `self.style` que o layout lê em TODO lugar (margem, medição,
    // render), não só no merge arm — reflita o `#set text` como o antigo delta
    // tipado refletia, cada resolver consulta, **no mesmo nó e antes de subir**, o
    // campo tipado E o `custom` (top-wins exato por profundidade). O `custom` é
    // ignorado por `is_semantically_empty` → permanece transparente à morfologia.

    /// Resolve `bold` (typed > custom `"text.bold"`, top-wins por profundidade).
    pub fn bold(&self) -> bool {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.bold {
                return v;
            }
            if let Some(Value::Bool(b)) = delta_custom(&n.delta, "text.bold") {
                return *b;
            }
            node = n.parent.as_deref();
        }
        false
    }

    /// Resolve `italic`.
    pub fn italic(&self) -> bool {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.italic {
                return v;
            }
            if let Some(Value::Bool(b)) = delta_custom(&n.delta, "text.italic") {
                return *b;
            }
            node = n.parent.as_deref();
        }
        false
    }

    /// Resolve `size` em pontos tipográficos.
    pub fn size(&self) -> f64 {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.size {
                return v;
            }
            if let Some(Value::Length(l)) = delta_custom(&n.delta, "text.size") {
                return l.abs.to_pt();
            }
            node = n.parent.as_deref();
        }
        11.0
    }

    /// Resolve `weight` percorrendo a cadeia.
    pub fn weight(&self) -> Option<u16> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.weight {
                return Some(v);
            }
            if let Some(Value::Int(i)) = delta_custom(&n.delta, "text.weight") {
                return u16::try_from(*i).ok();
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `tracking` (Length inteiro, preservando abs+em).
    pub fn tracking(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.tracking {
                return Some(v);
            }
            if let Some(Value::Length(l)) = delta_custom(&n.delta, "text.tracking") {
                return Some(*l);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `leading` (Length inteiro). Custom: `"par.leading"`.
    pub fn leading(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.leading {
                return Some(v);
            }
            if let Some(Value::Length(l)) = delta_custom(&n.delta, "par.leading") {
                return Some(*l);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `top_edge`. Custom: `"text.top-edge"` (`Value::Str` →
    /// métrica nomeada, `Value::Length` → comprimento explícito — P837).
    pub fn top_edge(&self) -> Option<crate::entities::layout_types::TextEdge> {
        use crate::entities::layout_types::TextEdge;
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(ref v) = n.delta.top_edge {
                return Some(v.clone());
            }
            match delta_custom(&n.delta, "text.top-edge") {
                Some(Value::Str(s)) => return Some(TextEdge::Metric(s.clone())),
                Some(Value::Length(l)) => return Some(TextEdge::Length(*l)),
                _ => {}
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `bottom_edge`. Custom: `"text.bottom-edge"` (`Value::Str` →
    /// métrica nomeada, `Value::Length` → comprimento explícito — P837).
    pub fn bottom_edge(&self) -> Option<crate::entities::layout_types::TextEdge> {
        use crate::entities::layout_types::TextEdge;
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(ref v) = n.delta.bottom_edge {
                return Some(v.clone());
            }
            match delta_custom(&n.delta, "text.bottom-edge") {
                Some(Value::Str(s)) => return Some(TextEdge::Metric(s.clone())),
                Some(Value::Length(l)) => return Some(TextEdge::Length(*l)),
                _ => {}
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `lang` (código BCP 47 validado).
    ///
    /// **P1034** — devolve `None` quando não há `#set text(lang:)`, de
    /// propósito. O default da linguagem Typst **é** `en`, mas aplicá-lo aqui
    /// liga a hifenização em todos os documentos: `cursor.rs:137` decide
    /// hifenizar só por `style.lang` ser `Some`, sem porta de `justify`. Medido:
    /// o vanilla **não** hifeniza um parágrafo não-justificado (`The
    /// extraordinary characteristics…` numa coluna de 100pt sai sem hífenes nos
    /// dois, mas o cristalino produzia 3 com o default aqui). O default de
    /// língua vive, por isso, nos sítios que **geram texto** —
    /// `lang::figure_supplement` e `lang::outline_title`, ambos com fallback
    /// `en`. Mover o default para cá exige primeiro corrigir a porta de
    /// hifenização (passo próprio).
    pub fn lang(&self) -> Option<crate::entities::lang::Lang> {
        use std::str::FromStr;
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.lang {
                return Some(v);
            }
            if let Some(Value::Str(s)) = delta_custom(&n.delta, "text.lang") {
                return crate::entities::lang::Lang::from_str(s).ok();
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `font` (FontList — não-Copy, clona). Top-wins.
    pub fn font(&self) -> Option<crate::entities::font_list::FontList> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = &n.delta.font {
                return Some(v.clone());
            }
            if let Some(Value::Array(arr)) = delta_custom(&n.delta, "text.font") {
                let fams: Vec<_> = arr
                    .iter()
                    .filter_map(|v| {
                        if let Value::Str(s) = v {
                            Some(crate::entities::font_list::FontFamily::new(s.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();
                return crate::entities::font_list::FontList::new(fams);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve subscrito (`#sub[...]`).
    pub fn subscript(&self) -> bool {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.subscript {
                return v;
            }
            node = n.parent.as_deref();
        }
        false
    }

    /// Resolve sobrescrito (`#super[...]`).
    pub fn superscript(&self) -> bool {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.superscript {
                return v;
            }
            node = n.parent.as_deref();
        }
        false
    }

    /// Resolve highlight (`#highlight[...]`).
    pub fn highlight(&self) -> Option<crate::entities::layout_types::Color> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.highlight {
                return v;
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve raio dos cantos do highlight (P471).
    pub fn highlight_radius(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.highlight_radius {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve extensão horizontal do highlight (P471).
    pub fn highlight_extent(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.highlight_extent {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve tamanho do corpo de subscrito (P471). `None` = usar escala padrão.
    pub fn subscript_size(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.subscript_size {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve tamanho do corpo de sobrescrito (P471). `None` = usar escala padrão.
    pub fn superscript_size(&self) -> Option<crate::entities::layout_types::Length> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.superscript_size {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// **P836** — resolve `variations` dobrando todos os níveis da cadeia
    /// (canal custom `"text.variations"`, escrito pelo constructor
    /// `text(variations:)` e pela set rule `#set text(variations:)`).
    /// Paridade do `#[fold]` do campo ghost `TextElem::variations` no
    /// vanilla: o nível interno vence por tag; tags de níveis externos
    /// ausentes no interno sobrevivem. `None` = nenhum nível define.
    pub fn variations(&self) -> Option<crate::entities::font_variations::FontVariations> {
        let mut acc: Option<crate::entities::font_variations::FontVariations> = None;
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(Value::Dict(d)) = delta_custom(&n.delta, "text.variations") {
                let level = crate::entities::font_variations::FontVariations::from_validated_dict(d);
                acc = Some(match acc {
                    Some(inner) => inner.fold(&level),
                    None => level,
                });
            }
            node = n.parent.as_deref();
        }
        acc
    }
}

/// **F-5b fatia 2 (P373)** — lê o valor do canal `custom` para `key` no delta
/// deste nó (sem subir a cadeia; o walk de cada resolver trata a herança).
fn delta_custom<'a>(delta: &'a StyleDelta, key: &str) -> Option<&'a Value> {
    delta.custom.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

/// Conversão para `TextStyle` plano — **ponto único de resolução**
/// (ADR-0039, Passo 100). O Layouter mantém `StyleChain` como
/// source-of-truth; `From<&StyleChain>` achata em `TextStyle` quando
/// emite um `FrameItem::Text`.
impl From<&StyleChain> for TextStyle {
    fn from(chain: &StyleChain) -> Self {
        TextStyle {
            bold: chain.bold(),
            italic: chain.italic(),
            size: Pt(chain.size()),
            fill: chain.fill(),
            heading_level: chain.heading_level(),
            // Passo 136 (Fase A — DEBT-52): propagação via
            // `StyleChain::resolve_*`. Inertes em layout até
            // Fase B/C adicionar consumers.
            weight: chain.weight(),
            tracking: chain.tracking(),
            leading: chain.leading(),
            top_edge: chain.top_edge(),
            bottom_edge: chain.bottom_edge(),
            lang: chain.lang(),
            // P753 — fonte padrão Libertinus Serif bate com o vanilla 0.15.0.
            // O cristalino carrega as mesmas fontes embutidas via typst-assets,
            // pelo que Libertinus Serif está sempre disponível. Se o FontBook
            // estiver vazio (configuração especial), o shaper faz fallback pelas
            // serif em DEFAULT_FALLBACK_FONTS_SERIF (salvaguarda de P558).
            font: Some(chain.font().unwrap_or_else(|| {
                FontList::single(EcoString::from("Libertinus Serif"))
            })),
            dir: None,
            subscript: chain.subscript(),
            superscript: chain.superscript(),
            highlight: chain.highlight(),
            highlight_radius: chain.highlight_radius(),
            highlight_extent: chain.highlight_extent(),
            subscript_size: chain.subscript_size(),
            superscript_size: chain.superscript_size(),
            baseline_offset: crate::entities::layout_types::Length::ZERO,
            // P784 — `StyleChain` não carrega contexto math; `false` aqui é
            // sempre correcto (`layout_equation` põe `true` explicitamente
            // no `TextStyle` que passa para o motor de layout matemático,
            // por cima deste valor base).
            math: false,
            // P891 — mesmo motivo de P784: `StyleChain` não carrega contexto
            // de script; `attach.rs` põe `true` explicitamente no
            // `script_style` que constrói para sub/super-índices.
            math_script: false,
            // P915 — mesmo motivo de P891: `StyleChain` não carrega contexto
            // cramped; `attach.rs`/`frac.rs`/`root.rs`/`accent.rs` põem
            // `true` explicitamente nos estilos que constroem para os
            // pontos de propagação mapeados (subscrito, denominador,
            // radicando+índice, base de accent).
            cramped: false,
            // P945 — mesmo motivo de P784/P891/P915: `StyleChain` não carrega
            // o nível MathSize; o valor neutro `Text` é o default do vanilla
            // para contexto não-math, e `layout_equation` fixa
            // `Display`/`Text` explicitamente no `math_style` por cima deste
            // valor base.
            math_size: crate::entities::layout_types::MathSize::Text,
            // P836 — eixos explícitos (`#text(variations:)`), fold por tag
            // entre níveis da chain.
            variations: chain.variations(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::style::Style;

    // ── Lote F-2 S5 (P335) — trava do canal aberto (teste-varre-tabela) ──────
    // A trava da ADR-0105 cláusula 3, lado estilo: onde o canal aberto perde a
    // exaustividade do compilador (chave dinâmica → `Value`), este teste enumera
    // as `PropKey` das `Set*` migradas e assere que resolvem com o **tipo
    // esperado pelos consumidores**. Adicionar uma `Set*` nova ao canal obriga a
    // estender esta tabela (caso contrário a chave fica sem cobertura).
    #[test]
    fn f2_canal_aberto_varre_tabela_de_chaves() {
        // (chave, valor esperado pelo consumidor)
        let chain = StyleChain::default_chain()
            .push_custom("heading.numbering", Value::Bool(true)) // S1
            .push_custom("equation.numbering", Value::Bool(true)) // S2
            .push_custom("figure.numbering", Value::Str("1".into())); // S3

        // Cada chave resolve com o tipo que o consumidor casa (Bool/Bool/Str).
        assert!(matches!(chain.custom("heading.numbering"), Some(Value::Bool(true))));
        assert!(matches!(chain.custom("equation.numbering"), Some(Value::Bool(true))));
        assert!(matches!(chain.custom("figure.numbering"), Some(Value::Str(_))));

        // Chave ausente → `None` (fallback léxico explícito; o consumidor trata
        // como "não definido", não como erro de tipo silencioso).
        assert!(chain.custom("chave.inexistente").is_none());

        // Top-wins (escopo léxico): um push mais interno sobrepõe o externo —
        // a base do `#set` dentro de bloco não vazar (DEBT 99.E).
        let inner = chain.push_custom("heading.numbering", Value::Bool(false));
        assert!(matches!(inner.custom("heading.numbering"), Some(Value::Bool(false))));
        // O externo permanece intacto (clone O(1), imutável).
        assert!(matches!(chain.custom("heading.numbering"), Some(Value::Bool(true))));
    }

    #[test]
    fn style_chain_defaults() {
        use crate::entities::lang::Lang;
        let chain = StyleChain::default_chain();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(), 11.0);
        // **P1034** — `lang` fica `None` sem `#set text(lang:)`; o default `en`
        // da linguagem é aplicado por quem gera texto, não aqui (ver `lang()`).
        assert_eq!(chain.lang(), None);
    }

    #[test]
    fn style_chain_push_herda() {
        let base = StyleChain::default_chain();
        let child = base.push(StyleDelta {
            bold: Some(true),
            italic: None,
            size: None,
            ..StyleDelta::empty()
        });
        assert!(child.bold());
        assert!(!child.italic()); // herdado
        assert_eq!(child.size(), 11.0); // herdado
    }

    #[test]
    fn style_chain_push_multiplos_niveis() {
        let base = StyleChain::default_chain();
        let mid = base.push(StyleDelta {
            bold: Some(true),
            italic: None,
            size: None,
            ..StyleDelta::empty()
        });
        let child = mid.push(StyleDelta {
            bold: None,
            italic: None,
            size: Some(14.0),
            ..StyleDelta::empty()
        });
        // bold herdado de mid, size de child, italic do root
        assert!(child.bold());
        assert!(!child.italic());
        assert_eq!(child.size(), 14.0);
    }

    #[test]
    fn style_chain_clone_e_o1() {
        let base = StyleChain::default_chain();
        let chain = base.push(StyleDelta {
            bold: Some(true),
            italic: None,
            size: None,
            ..StyleDelta::empty()
        });
        let clone = chain.clone();
        assert!(clone.bold());
        // Clone correcto — mesmo que O(1) não seja verificável directamente
    }

    #[test]
    fn text_style_from_style_chain() {
        let chain = StyleChain::default_chain().push(StyleDelta {
            bold: Some(true),
            italic: None,
            size: Some(14.0),
            ..StyleDelta::empty()
        });
        let ts = TextStyle::from(&chain);
        assert!(ts.bold);
        assert!(!ts.italic);
        assert_eq!(ts.size.val(), 14.0);
    }

    // ── Passo 136 (Fase A — DEBT-52): propagação dos 5 campos novos.
    // Testa via `TextStyle::from(&chain)` construindo `StyleChain`
    // directamente — mais preciso que pipeline completo e mais rápido.

    #[test]
    fn text_style_from_chain_propaga_weight_passo_136() {
        let chain = StyleChain::default_chain()
            .push(StyleDelta { weight: Some(700), ..StyleDelta::empty() });
        let ts = TextStyle::from(&chain);
        assert_eq!(ts.weight, Some(700));
    }

    #[test]
    fn text_style_from_chain_propaga_tracking_passo_136() {
        use crate::entities::layout_types::Length;
        let chain = StyleChain::default_chain().push(StyleDelta {
            tracking: Some(Length::pt(0.5)),
            ..StyleDelta::empty()
        });
        let ts = TextStyle::from(&chain);
        assert_eq!(ts.tracking, Some(Length::pt(0.5)));
    }

    #[test]
    fn text_style_from_chain_propaga_leading_passo_136() {
        use crate::entities::layout_types::Length;
        let chain = StyleChain::default_chain().push(StyleDelta {
            leading: Some(Length::em(0.65)),
            ..StyleDelta::empty()
        });
        let ts = TextStyle::from(&chain);
        assert_eq!(ts.leading, Some(Length::em(0.65)));
    }

    #[test]
    fn text_style_from_chain_propaga_lang_passo_136() {
        use crate::entities::lang::Lang;
        let chain = StyleChain::default_chain()
            .push(StyleDelta { lang: Some(Lang::ENGLISH), ..StyleDelta::empty() });
        let ts = TextStyle::from(&chain);
        assert_eq!(ts.lang, Some(Lang::ENGLISH));
    }

    #[test]
    fn text_style_from_chain_propaga_font_passo_136() {
        use crate::entities::font_list::FontList;
        use ecow::EcoString;
        let fl = FontList::single(EcoString::from("Arial"));
        let chain = StyleChain::default_chain()
            .push(StyleDelta { font: Some(fl.clone()), ..StyleDelta::empty() });
        let ts = TextStyle::from(&chain);
        assert_eq!(ts.font, Some(fl));
    }

    // ── P836 — resolver `variations()` (fold por tag entre níveis) ─────────

    fn p836_dict(entries: &[(&str, i64)]) -> Value {
        let mut d = indexmap::IndexMap::with_hasher(rustc_hash::FxBuildHasher::default());
        for (k, v) in entries {
            d.insert(ecow::EcoString::from(*k), Value::Int(*v));
        }
        Value::Dict(d)
    }

    #[test]
    fn p836_variations_ausente_e_none() {
        let chain = StyleChain::default_chain();
        assert_eq!(chain.variations(), None);
        assert_eq!(TextStyle::from(&chain).variations, None);
    }

    #[test]
    fn p836_variations_nivel_unico() {
        let chain = StyleChain::default_chain()
            .push_custom("text.variations", p836_dict(&[("wght", 250)]));
        let fv = chain.variations().expect("variations resolvido");
        assert_eq!(fv.0, vec![(*b"wght", 250.0)]);
        assert_eq!(TextStyle::from(&chain).variations, Some(fv));
    }

    #[test]
    fn p836_variations_fold_interno_vence_por_tag() {
        // Paridade `#[fold]` do vanilla: `#set text(variations: (wght: 700,
        // ital: 1))` externo + `#set text(variations: (wght: 250))` interno
        // → wght 250 (interno vence), ital 1 (externo sobrevive).
        let chain = StyleChain::default_chain()
            .push_custom("text.variations", p836_dict(&[("wght", 700), ("ital", 1)]))
            .push_custom("text.variations", p836_dict(&[("wght", 250)]));
        let fv = chain.variations().expect("variations resolvido");
        assert_eq!(fv.0, vec![(*b"ital", 1.0), (*b"wght", 250.0)]);
    }

    #[test]
    fn empty_chain_usa_defaults() {
        use crate::entities::lang::Lang;
        let chain = StyleChain::empty();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(), 11.0);
        assert_eq!(chain.lang(), None);
    }

    // ── P483 ────────────────────────────────────────────────────────────────

    #[test]
    fn p483_textstyle_from_chain_font_nunca_none() {
        // P753 — From<&StyleChain> para TextStyle com chain vazia deve
        // retornar font = Some(Libertinus Serif) (fallback padrão do vanilla).
        use crate::entities::layout_types::TextStyle;
        let chain = StyleChain::default_chain(); // sem #set text(font:...)
        let style = TextStyle::from(&chain);
        assert!(
            style.font.is_some(),
            "P753: TextStyle de chain default deve ter font = Some(Libertinus Serif)"
        );
        let families = style.font.as_ref().unwrap().as_slice();
        assert_eq!(families.len(), 1);
        assert_eq!(
            families[0].name.as_str(),
            Some("libertinus serif"),
            "P753: fonte padrão deve ser 'libertinus serif' (lowercase)"
        );
    }

    #[test]
    fn p483_textstyle_from_chain_com_font_explicity_preserva() {
        // Se #set text(font: "Arial") → font = Some(Arial), não Helvetica.
        use crate::entities::font_list::FontList;
        use crate::entities::layout_types::TextStyle;
        let fl = FontList::single(EcoString::from("Arial"));
        let chain = StyleChain::default_chain()
            .push(StyleDelta { font: Some(fl.clone()), ..StyleDelta::empty() });
        let style = TextStyle::from(&chain);
        let families = style.font.as_ref().unwrap().as_slice();
        assert_eq!(
            families[0].name.as_str(),
            Some("arial"),
            "P483: font explícito do documento deve ser preservado"
        );
    }

    // ── Passo 99 (ADR-0038): Styles/Style integração ──────────────────────

    use crate::entities::layout_types::Color;

    #[test]
    fn push_styles_projecta_bold_italic_size() {
        let base = StyleChain::default_chain();
        let styles = Styles::from_iter([
            Style::bold(true),
            Style::italic(true),
            Style::Size(Pt(18.0)),
        ]);
        let child = base.push_styles(&styles);
        assert!(child.bold());
        assert!(child.italic());
        assert_eq!(child.size(), 18.0);
    }

    #[test]
    fn push_styles_herda_propriedade_nao_definida() {
        let base = StyleChain::default_chain();
        // Só define bold — italic e size devem cair no default.
        let styles = Styles::from_iter([Style::bold(true)]);
        let child = base.push_styles(&styles);
        assert!(child.bold());
        assert!(!child.italic());
        assert_eq!(child.size(), 11.0);
    }

    #[test]
    fn push_styles_topo_ganha_sobre_base() {
        let base = StyleChain::default_chain()
            .push_styles(&Styles::from_iter([Style::bold(true)]));
        let child = base.push_styles(&Styles::from_iter([Style::bold(false)]));
        assert!(!child.bold(), "o delta mais próximo do texto ganha");
    }

    #[test]
    fn fill_forward_compat() {
        let base = StyleChain::default_chain();
        assert_eq!(base.fill(), None, "sem Fill, None");
        let red = Color::rgb(255, 0, 0);
        let child = base.push_styles(&Styles::from_iter([Style::Fill(red)]));
        assert_eq!(child.fill(), Some(red));
    }

    #[test]
    fn heading_level_forward_compat() {
        let base = StyleChain::default_chain();
        assert_eq!(base.heading_level(), None);
        let child = base.push_styles(&Styles::from_iter([Style::HeadingLevel(3)]));
        assert_eq!(child.heading_level(), Some(3));
    }

    #[test]
    fn chain_aninhada_fill_heading_level_top_wins() {
        let base = StyleChain::default_chain().push_styles(&Styles::from_iter([
            Style::Fill(Color::rgb(0, 0, 255)),
            Style::HeadingLevel(1),
        ]));
        let child =
            base.push_styles(&Styles::from_iter([Style::Fill(Color::rgb(255, 0, 0))]));
        // Fill: o topo define — ganha.
        assert_eq!(child.fill(), Some(Color::rgb(255, 0, 0)));
        // HeadingLevel: o topo não define — herda do pai.
        assert_eq!(child.heading_level(), Some(1));
    }

    // ── Passo 99.D: Teste de integração conceptual ───────────────────────

    use crate::entities::content::Content;

    /// Integração: um `Content::Styled` constrói-se com `Styles`; a
    /// resolução via `StyleChain` devolve os mesmos valores que foram
    /// aplicados como delta. Isto valida que a fundação é usável sem
    /// activar `#set` no eval (Passo 99, ADR-0038).
    #[test]
    fn integracao_content_styled_resolve_via_style_chain() {
        let body = Content::text("hello");
        let styles = Styles::from_iter([Style::bold(true), Style::Size(Pt(18.0))]);
        let styled = Content::Styled(Box::new(body), styles);

        // O consumidor futuro (`eval_markup`) faria:
        //   1. ler os estilos do Content::Styled;
        //   2. push na StyleChain;
        //   3. passar a StyleChain à avaliação do body.
        // Aqui simulamos o passo 2+3 manualmente.
        let chain = match &styled {
            Content::Styled(_body, ss) => StyleChain::default_chain().push_styles(ss),
            _ => panic!("esperado Content::Styled"),
        };

        assert!(chain.bold());
        assert!(!chain.italic()); // default
        assert_eq!(chain.size(), 18.0);
    }

    /// Integração: `Styled` aninhado — o delta mais próximo do texto ganha
    /// (top-wins), consistente com o vanilla (ADR-0033).
    #[test]
    fn integracao_styled_aninhado_top_wins() {
        let inner_body = Content::text("hi");
        let inner = Content::Styled(
            Box::new(inner_body),
            Styles::from_iter([Style::italic(true)]),
        );
        let outer = Content::Styled(
            Box::new(inner),
            Styles::from_iter([Style::bold(true), Style::italic(false)]),
        );

        // Simular o caminho que o eval tomaria: outer primeiro (mais
        // longe do texto), depois inner (mais perto).
        let chain = match &outer {
            Content::Styled(body, ss_outer) => {
                let chain_outer = StyleChain::default_chain().push_styles(ss_outer);
                match body.as_ref() {
                    Content::Styled(_, ss_inner) => chain_outer.push_styles(ss_inner),
                    _ => panic!("esperado Content::Styled aninhado"),
                }
            }
            _ => panic!("esperado Content::Styled"),
        };

        // bold=true (só outer define).
        assert!(chain.bold());
        // italic=true — inner está mais perto do texto e define Italic(true),
        // sobrepondo o Italic(false) do outer. Top-wins (paridade vanilla).
        assert!(chain.italic());
    }
}
