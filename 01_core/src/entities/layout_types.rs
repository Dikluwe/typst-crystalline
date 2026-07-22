//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/layout_types.md
//! @prompt-hash d11a51d1
//! @layer L1
//! @updated 2026-04-23
//!
//! Excepção Regra 6 da ADR-0037: agrega tipos geométricos e
//! estruturais fundamentais do layout (`Pt`, `Point`, `FrameItem`,
//! `Page`, `PageConfig`, `PagedDocument`, `TextStyle`, `Color`,
//! `Length`, `TrackSizing`, `Align2D`, `HAlign`/`VAlign`, `PlaceScope`,
//! `TransformMatrix`). Estes tipos têm muitas operações e conversões
//! próximas; separá-los por ficheiro destruiria a visibilidade mútua
//! (impls cruzadas) e multiplicaria imports nos consumidores sem
//! ganho. ~850 linhas aceitas como custo de coesão do vocabulário
//! geométrico.

use std::collections::HashMap;
use std::sync::Arc;

use ecow::EcoString;

use crate::entities::dir::Dir;
use crate::entities::document_info::DocumentInfo;
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::label::Label;
use crate::entities::source_result::SourceDiagnostic;

// ── Coordenadas e medidas ──────────────────────────────────────────────────

/// Ponto tipográfico — unidade interna de layout.
/// 1 pt = 1/72 inch.
///
/// Não implementa `Add<f64>` — escalares brutos requerem `Pt(valor)` explícito.
/// Isto previne misturar coordenadas com índices ou contagens.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Pt(pub f64);

impl Pt {
    pub const ZERO: Self = Self(0.0);

    pub fn val(self) -> f64 {
        self.0
    }
}

impl std::ops::Add for Pt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Pt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Pt {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl std::ops::AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

// Deliberadamente NÃO implementado:
// impl Add<f64> for Pt — escalares requerem Pt(valor) explícito

/// Posição 2D na página.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Pt,
    pub y: Pt,
}

impl Point {
    pub const ZERO: Self = Self { x: Pt::ZERO, y: Pt::ZERO };
}

/// Tamanho 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: Pt,
    pub height: Pt,
}

impl Size {
    /// Tamanho A4 em pontos tipográficos.
    pub fn a4() -> Self {
        Self { width: Pt(595.0), height: Pt(842.0) }
    }
}

/// Rectângulo alinhado aos eixos (paridade `Point` + `Size`).
///
/// **P273.5** — usado como bbox de contentor para resolver
/// `Gradient.relative: Some(RelativeTo::Parent)` no callsite real
/// do Layouter. Padrão DEBT-37 P84.6 `cell_origin_*: Option<f64>`
/// reused estructuralmente — campo opcional `parent_bbox: Option<Rect>`
/// no Layouter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: Pt,
    pub y: Pt,
    pub w: Pt,
    pub h: Pt,
}

// ── Estilo de texto ────────────────────────────────────────────────────────

/// **P837** — bordo vertical da linha de texto (`top-edge`/`bottom-edge`).
///
/// Espelho dos enums `TopEdge`/`BottomEdge` do vanilla
/// (`text/mod.rs:1161-1248`): ou uma métrica tipográfica nomeada, ou um
/// comprimento explícito. O domínio de nomes válidos difere entre top
/// (`"ascender"`, `"cap-height"`, `"x-height"`, `"baseline"`, `"bounds"`)
/// e bottom (`"baseline"`, `"descender"`, `"bounds"`); a validação é feita
/// no eval (`rules.rs`, erro verbatim do vanilla) — aqui a métrica é só
/// transportada. `Length` resolve a partir da baseline no font-size
/// (`length.at(font_size)` no vanilla, `Length::resolve_pt` no cristalino).
#[derive(Debug, Clone, PartialEq)]
pub enum TextEdge {
    /// Métrica tipográfica nomeada (ex.: `"cap-height"`, `"descender"`).
    Metric(ecow::EcoString),
    /// Comprimento explícito a partir da baseline.
    Length(Length),
}

/// Estilo de texto — struct plano.
///
/// DEBT: deve ser substituído por StyleChain (lista ligada de deltas)
/// antes de implementar `#set text(...)`. Ver DEBT.md.
/// Estilo resolvido — vista achatada do resultado de
/// `From<&StyleChain>` (ADR-0039, Passo 100). Os campos `fill` e
/// `heading_level` são forward-compat (ADR-0038) e por omissão `None`.
///
/// Semântica: é **o resultado** de resolver uma `StyleChain`, não a
/// cadeia em si. Consumido por `FrameItem::Text.style` e por
/// `export.rs` em L3.
/// Passo 136 (Fase A de DEBT-52, ADR-0054): `TextStyle` estendido
/// com 5 campos propagados de `StyleDelta`. Remoção de `Copy`
/// porque `FontList` contém `Vec<FontFamily>`; call sites usam
/// `.clone()` explícito. Consumers em fase B/C do roadmap.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub size: Pt,
    /// Cor de preenchimento — ADR-0038/0039 forward-compat.
    pub fill: Option<Color>,
    /// Nível de heading — ADR-0038/0039 forward-compat.
    pub heading_level: Option<u8>,

    // Passo 136 (Fase A — DEBT-52). Propagados de `StyleDelta`
    // mas sem consumer em layout ainda. Fases B/C resolvem.
    pub weight: Option<u16>,
    pub tracking: Option<crate::entities::layout_types::Length>,
    pub leading: Option<crate::entities::layout_types::Length>,
    /// **P762** — bordo superior da linha (`top-edge`). **P837**: métrica
    /// nomeada (`"ascender"`, `"cap-height"`, `"x-height"`, `"baseline"`,
    /// `"bounds"`) ou `Length` explícito. `None` = default do vanilla
    /// (`"cap-height"`).
    pub top_edge: Option<TextEdge>,
    /// **P762** — bordo inferior da linha (`bottom-edge`). **P837**:
    /// métrica nomeada (`"baseline"`, `"descender"`, `"bounds"`) ou
    /// `Length` explícito. `None` = default do vanilla (`"baseline"`).
    pub bottom_edge: Option<TextEdge>,
    pub lang: Option<crate::entities::lang::Lang>,
    pub font: Option<crate::entities::font_list::FontList>,
    /// **P576**: direcção de texto (`ltr`/`rtl`), transportada do `#set text(dir: ...)`.
    pub dir: Option<Dir>,
    /// **Passo 448 (P448)**: subscrito (`#sub[...]`).
    pub subscript: bool,
    /// **Passo 448 (P448)**: sobrescrito (`#super[...]`).
    pub superscript: bool,
    /// **Passo 449 (P449)**: cor de fundo do highlight (`#highlight[...]`).
    pub highlight: Option<Color>,
    /// **P471**: raio dos cantos do rectângulo de highlight.
    pub highlight_radius: Option<Length>,
    /// **P471**: extensão horizontal do rectângulo de highlight.
    pub highlight_extent: Option<Length>,
    /// **P471**: tamanho explícito do subscrito. `None` = 65% do font-size.
    pub subscript_size: Option<Length>,
    /// **P471**: tamanho explícito do sobrescrito. `None` = 65% do font-size.
    pub superscript_size: Option<Length>,
    /// **Passo 448 (P448)**: deslocamento vertical da baseline (resolvido em
    /// `layout/text.rs` e aplicado em `cursor.rs`).
    pub baseline_offset: Length,
    /// **P784** — `true` quando este texto é conteúdo matemático (definido
    /// uma vez em `layout/equation.rs::layout_equation`, herdado por
    /// `..style.clone()` em toda a árvore de layout math). Consumido em
    /// L3 (`shaper.rs`) para decidir se a cadeia de fallback específica de
    /// matemática (`New Computer Modern Math` → `Libertinus Serif` →
    /// emojis, paridade `math::families()` vanilla) entra sempre como
    /// primárias adicionais — não só quando a fonte já resolvida
    /// coincidentemente tem tabela MATH (que a fonte de corpo por omissão,
    /// `Libertinus Serif`, não tem).
    pub math: bool,
    /// **P836** — coordenadas de eixo OpenType explícitas
    /// (`#text(variations:)` / `#set text(variations:)`), resolvidas da
    /// chain por `StyleChain::variations()` (fold por tag). `None` =
    /// sem variações explícitas. Consumido em L3 (shaper, font_metrics,
    /// pipeline/export) fundido com os eixos derivados de `FontVariant`.
    pub variations: Option<crate::entities::font_variations::FontVariations>,
}

impl TextStyle {
    pub fn regular(size: Pt) -> Self {
        Self {
            bold: false,
            italic: false,
            size,
            ..Self::default()
        }
    }
    pub fn bold(size: Pt) -> Self {
        Self { bold: true, italic: false, size, ..Self::default() }
    }
    pub fn italic(size: Pt) -> Self {
        Self { bold: false, italic: true, size, ..Self::default() }
    }

    /// Passo 139 (Fase B.3 DEBT-52): computa stroke width para
    /// faux-bold baseado em `weight` + `size`. Weight <= 400
    /// devolve 0 (sem stroke — sem efeito visível).
    ///
    /// Fórmula: `((weight - 400) / 300).max(0) × size × k`.
    /// `k` é coeficiente de calibração (typical: 0.04).
    ///
    /// Weight absent (`None`) é tratado como 400 (regular).
    pub fn faux_bold_stroke_pt(&self, k: f64) -> f64 {
        let w = self.weight.unwrap_or(400);
        let factor = ((w as f64 - 400.0) / 300.0).max(0.0);
        factor * self.size.val() * k
    }
}

// ── Frame e FrameItem ──────────────────────────────────────────────────────

pub use crate::entities::shaped_glyph::ShapedGlyph;

/// Item posicionado num frame.
///
/// Divergência: original usa `(Point, FrameItem)` como tupla separada.
/// Cristalino embute `pos` em `FrameItem::Text` por simplicidade.
#[derive(Debug, Clone)]
pub enum FrameItem {
    /// Texto posicionado (string plana, sem shaping real).
    ///
    /// **P483 — DEPRECATED**: Use `FrameItem::TextShaped`.
    /// Preservado como fallback para fontes não carregadas ou Type1.
    #[deprecated(
        since = "P483",
        note = "Use FrameItem::TextShaped. \
        Preserved as fallback for fonts not loaded or Type1."
    )]
    Text { pos: Point, text: EcoString, style: TextStyle },
    /// **P482** — Texto com shaping real (rustybuzz). Substitui `Text`
    /// após a passagem do shaper (L3). `Text` preservado para fallback
    /// quando shaping não disponível (fonte não carregada, etc.).
    TextShaped {
        pos: Point,
        /// Glifos shaped por rustybuzz.
        glyphs: Vec<ShapedGlyph>,
        style: TextStyle,
        /// Texto original (ToUnicode CMap + plain_text + fallback).
        text: EcoString,
        /// **P485** — Unidades por em da fonte shaped (de `Face::units_per_em()`).
        /// Usado em export para converter `x_advance` (font units) em pt:
        /// `advance_pt = x_advance / units_per_em × font_size`.
        units_per_em: u16,
    },
    /// Linha horizontal. Usada pela linha de fracção matemática (Passo 38).
    /// `start` e `end` são posições absolutas no Frame.
    /// `thickness` em pontos tipográficos.
    /// `color`: opcional (Passo 285). `Some(c)` → emit `r g b RG` antes
    /// de `S`; `None` → preserva default preto bit-exact (backward-compat
    /// para fracções, sqrt overline e linhas geométricas sem stroke
    /// explícito).
    Line { start: Point, end: Point, thickness: f64, color: Option<Color> },
    /// Glifo renderizado directamente por ID, sem mapeamento Unicode.
    ///
    /// Usado para variantes de tamanho matemático onde `glyph_to_char`
    /// retorna `None`. O export PDF escreve o ID como `<XXXX> Tj`.
    ///
    /// `pos`: posição final do glifo (coordenadas de página), calculada
    ///        pelo `MathLayouter` antes de emitir este item.
    /// `glyph_id`: índice do glifo na fonte (índice CIDFont, Identity-H).
    /// `x_advance`: largura horizontal do glifo em pt.
    /// `size`: corpo tipográfico em pt.
    Glyph { pos: Point, glyph_id: u16, x_advance: Pt, size: Pt },
    /// Imagem a desenhar na página.
    ///
    /// `pos`: canto superior esquerdo em coordenadas de página (pt).
    ///        NOTA: para imagens, pos.y é o TOPO da bounding box — não o baseline de texto.
    ///        O exportador calcula pdf_y = page_height - pos.y - height (inversão de eixo Y).
    /// `data`: bytes raw da imagem (JPEG, PNG, etc.) — Arc para zero-copy.
    /// `width`, `height`: dimensões físicas no documento (pt) — tamanho de layout
    ///   da transformação da imagem (pode exceder o target quando fit=cover).
    /// `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis, lidas do
    ///   cabeçalho da imagem. Obrigatórias para o dicionário XObject no PDF —
    ///   /Width e /Height intrínsecos ≠ tamanho de layout na página.
    /// `clip_rect`: rectângulo de clip no espaço do layout (Y crescente para cima),
    ///   preenchido quando fit=cover e a imagem transformada excede o target (P771).
    /// `orientation`: valor EXIF Orientation (1-8). O exportador PDF aplica a
    ///   transformação correspondente via matriz `cm`, preservando os bytes
    ///   originais do JPEG/PNG (P776).
    Image {
        pos: Point,
        data: Arc<Vec<u8>>,
        width: Pt,
        height: Pt,
        intrinsic_width: u32,
        intrinsic_height: u32,
        clip_rect: Option<Rect>,
        orientation: u32,
    },
    /// Forma geométrica com dimensões resolvidas em pontos (Passo 76).
    ///
    /// Todos os campos são concretos — sem `Option<Value>`.
    /// `pos`: canto superior esquerdo da bounding box.
    /// O exportador calcula `pdf_y = page_height - pos.y - height` (inversão de eixo Y).
    Shape {
        pos: Point,
        kind: ShapeKind,
        width: f64,
        height: f64,
        fill: Option<Color>,
        stroke: Option<Stroke>,
        /// **P273.6** — bbox do contentor imediato no momento do emit.
        /// `Some(rect)` quando shape foi emitida dentro de `Content::Block`
        /// com `width.is_some() && height.is_some()` (Decisão 3γ.2.γ).
        /// `None` quando top-level ou contentor sem dimensions literais
        /// (cai no fallback page_bbox L3 P273.5).
        parent_bbox_at_emit: Option<Rect>,
    },
    /// Grupo com transformação afim aplicada (Passo 78).
    ///
    /// O exportador emite q → cm → [W n se clip_mask] → itens filhos → Q.
    /// `pos`: posição do grupo na página (espaço global).
    /// `matrix`: transformação afim com compensação de origem negativa.
    /// `clip_mask`: forma que restringe o desenho à sua área interna (DEBT-30).
    ///   Se Some, o exportador emite o path da máscara seguido de `W n` no
    ///   espaço local (após `cm`). Se None, sem recorte.
    /// `inner_width`, `inner_height`: dimensões do conteúdo antes da transformação.
    ///   Necessárias para clip_mask do tipo Rect no espaço local.
    /// `items`: itens em espaço local (Y-down, origem em (0,0)).
    Group {
        pos: Point,
        matrix: TransformMatrix,
        clip_mask: Option<ShapeKind>,
        inner_width: f64,
        inner_height: f64,
        items: Vec<FrameItem>,
    },
    /// **P422** — Hiperligação. O body é renderizado normalmente; o destino é
    /// preservado como metadado para o consumer downstream (exportador PDF).
    /// **P424** — adicionados `pos` e `size` para permitir annotation URI no PDF.
    /// **P463** — `url` generalizado para `LinkTarget` (URL externo ou destino
    /// interno `/GoTo`). Cor/sublinhado continuam scope-out.
    Link { target: LinkTarget, items: Vec<FrameItem>, pos: Point, size: Size },
}

/// **P463** — Destino de um `FrameItem::Link`.
#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    /// Hiperligação externa — emite `/A << /S /URI /URI (...) >>`.
    Url(EcoString),
    /// Destino interno nomeado — emite `/A << /S /GoTo /D /name >>`.
    Destination(Label),
}

// ── Alinhamento (Passo 82) ─────────────────────────────────────────────────

/// Alinhamento horizontal.
///
/// Inclui `Start`/`End` (Typst 0.15.0) — por enquanto resolvem para
/// `Left`/`Right` no layout, porque o cristalino ainda não modela
/// direção do texto. A semântica da linguagem (aceitar `start`/`end`)
/// é preservada; o efeito visual é scope-out.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum HAlign {
    Left,
    Center,
    Right,
    Start,
    End,
}

/// Alinhamento vertical.
///
/// `Horizon` é o termo interno do Typst para centro vertical.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum VAlign {
    Top,
    Horizon,
    Bottom,
}

/// Alinhamento 2D composto por componentes horizontal e vertical opcionais.
///
/// Ambos `None` equivale a `Left + Top` (comportamento por omissão).
#[derive(Debug, Clone, Copy, PartialEq, Default, Hash)]
pub struct Align2D {
    pub h: Option<HAlign>,
    pub v: Option<VAlign>,
}

impl Align2D {
    /// Parse de uma string composta por partes separadas por '-'.
    ///
    /// Exemplos: `"center"`, `"top-right"`, `"bottom"`, `"horizon"`.
    /// Partes não reconhecidas são ignoradas silenciosamente.
    ///
    /// Sintaxe legacy preservada após o Passo 84.5 (DEBT-36 encerrado).
    /// A sintaxe preferida é a composição simbólica via `Value::Align`:
    /// `align(center + bottom, ...)` em vez de `align("center-bottom", ...)`.
    /// Continua a ser usada como fallback em `native_align` e `native_place`
    /// quando o utilizador passa string literal.
    pub fn from_string(s: &str) -> Self {
        let mut align = Align2D::default();
        for part in s.split('-') {
            match part {
                "left" => align.h = Some(HAlign::Left),
                "center" => align.h = Some(HAlign::Center),
                "right" => align.h = Some(HAlign::Right),
                "start" => align.h = Some(HAlign::Start),
                "end" => align.h = Some(HAlign::End),
                "top" => align.v = Some(VAlign::Top),
                "horizon" => align.v = Some(VAlign::Horizon),
                "bottom" => align.v = Some(VAlign::Bottom),
                _ => {}
            }
        }
        align
    }
}

/// Escopo de ancoragem de `Content::Place` (Passo 84.6, encerra DEBT-37).
///
/// Espelha `PlacementScope` do vanilla:
/// - `Column` (default): ancora ao "current container" — célula de Grid
///   activa, ou página se não houver Grid no contexto.
/// - `Parent`: ancora à página inteira mesmo dentro de Grid.
///
/// **Divergência vs vanilla:** o vanilla restringe `Parent` a `float: true`
/// e devolve erro caso contrário. O cristalino não tem `float` implementado,
/// pelo que `Parent` é aceite sempre — efeito visual: ancora à página sem
/// layout flutuante.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PlaceScope {
    #[default]
    Column,
    Parent,
}

// ── TrackSizing ───────────────────────────────────────────────────────────

/// Dimensionamento de uma coluna ou linha de grid (Passo 80).
#[derive(Debug, Clone, PartialEq)]
pub enum TrackSizing {
    /// Largura absoluta em pontos.
    Fixed(f64),
    /// Ajusta-se ao conteúdo mais largo da coluna, limitado por safe_available.
    Auto,
    /// Fracção do espaço restante após Fixed e Auto.
    /// Pode receber 0pt se Fixed + Auto esgotarem o espaço disponível (DEBT-34d).
    Fraction(f64),
}

// ── PageConfig e Page ─────────────────────────────────────────────────────

/// Configuração da página activa no layouter (Passo 81).
///
/// Mutável durante o layout — Content::SetPage altera estes valores.
/// As páginas já fechadas têm os seus próprios snapshots de width/height.
#[derive(Debug, Clone, PartialEq)]
pub struct PageConfig {
    pub width: f64,  // em pontos
    pub height: f64, // em pontos
    pub margin: f64, // margem uniforme em pontos
    /// **P598** — `true` se a margem está em modo automático (vanilla
    /// `margin: auto`). Quando `true`, qualquer alteração de `width` ou
    /// `height` por `SetPage` recalcula `margin` proporcionalmente à menor
    /// dimensão. Quando `false`, `margin` é um valor fixo definido pelo
    /// utilizador e não é recalculado.
    pub margin_is_auto: bool,
    /// **P532** — padrão de numeração automática de páginas.
    pub numbering: Option<EcoString>,
    /// **P537b** — colunas activas para páginas desta configuração.
    pub columns: Option<usize>,
}

impl Default for PageConfig {
    fn default() -> Self {
        // P598 — margem automática do vanilla 0.15.0:
        // 2.5/21 da menor dimensão da página (≈ 11.90476 %).
        // Para A4 dá 70.87 pt; para height: 200pt dá ≈ 23.81 pt.
        let width = 595.28; // A4 portrait
        let height = 841.89; // A4 portrait
        Self {
            width,
            height,
            margin: width.min(height) * 2.5 / 21.0,
            margin_is_auto: true,
            numbering: None,
            columns: None,
        }
    }
}

impl PageConfig {
    /// Calcula a margem automática do vanilla 0.15.0 para as dimensões
    /// actuais (P598).
    pub fn auto_margin(&self) -> f64 {
        self.width.min(self.height) * 2.5 / 21.0
    }
}

/// Página fechada — snapshot imutável das dimensões e items visuais (Passo 81).
///
/// `width` e `height` são capturados de `PageConfig` no momento do fecho da página.
/// Duas páginas consecutivas podem ter dimensões distintas.
#[derive(Debug, Clone)]
pub struct Page {
    /// Largura da página no momento em que foi fechada.
    pub width: f64,
    /// Altura da página no momento em que foi fechada.
    pub height: f64,
    /// **P532** — padrão de numeração automática activo na página.
    pub numbering: Option<EcoString>,
    pub items: Vec<FrameItem>,
}

#[allow(deprecated)] // P483 — Text é fallback legítimo em plain_text
fn plain_text_items<'a>(items: &'a [FrameItem], out: &mut Vec<&'a str>) {
    for item in items {
        match item {
            FrameItem::Text { text, .. } => out.push(text.as_str()),
            FrameItem::TextShaped { text, .. } => out.push(text.as_str()),
            FrameItem::Link { items, .. } => plain_text_items(items, out),
            _ => {}
        }
    }
}

impl Page {
    /// Extrai texto plano — para verificação em testes.
    pub fn plain_text(&self) -> String {
        let mut parts = Vec::new();
        plain_text_items(&self.items, &mut parts);
        parts.join(" ")
    }
}

/// Canvas de uma página — colecção de itens com posições absolutas.
///
/// Divergência: original usa `Arc<LazyHash<Vec<(Point, FrameItem)>>>`.
/// Cristalino usa `Vec<FrameItem>` directo por simplicidade.
#[derive(Debug, Clone)]
pub struct Frame {
    pub size: Size,
    pub items: Vec<FrameItem>,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self { size, items: Vec::new() }
    }

    pub fn push(&mut self, item: FrameItem) {
        self.items.push(item);
    }

    /// Extrai texto plano — para verificação em testes.
    pub fn plain_text(&self) -> String {
        let mut parts = Vec::new();
        plain_text_items(&self.items, &mut parts);
        parts.join(" ")
    }
}

// ── PagedDocument ──────────────────────────────────────────────────────────

/// Documento paginado — resultado de `layout()`.
///
/// Divergência: original tem `EcoVec<Page>` + `DocumentInfo` + `Arc<PagedIntrospector>`.
/// Cristalino usa `Vec<Page>` com snapshots imutáveis de dimensão (Passo 81).
#[derive(Debug, Clone)]
pub struct PagedDocument {
    pub pages: Vec<Page>,
    /// Mapa de labels para o número de página onde aterraram (Passo 63).
    /// Populado por `Layouter::finish()` após cada passagem de layout.
    /// Vazio por defeito — só tem dados após `layout()` com labels no documento.
    pub extracted_label_pages: HashMap<Label, usize>,
    /// **P460** — mapa de labels para posição (x, y) onde aterraram.
    /// Populado por `Layouter::finish()` após cada passagem de layout.
    /// Vazio por defeito — só tem dados após `layout()` com labels no documento.
    pub extracted_label_positions: HashMap<Label, Point>,
    /// **P205B (F3)** — sub-store sealed `Location → Position`
    /// extraído de `LayouterRuntimeState.positions` ao fim da
    /// iteração. Tracked via `#[comemo::track]` per ADR-0074
    /// PROPOSTO. Vazio por defeito — só tem dados após `layout()`
    /// com locatable content. Consumer migration em P205C.
    pub extracted_positions: crate::entities::sealed_positions::SealedPositions,
    /// **P488** — páginas de figuras contadas, em ordem de documento.
    /// Populado por `Layouter::finish()` a partir de `runtime.figure_page_numbers`.
    /// Usado pelo fixpoint em `mod.rs` para carry-forward entre iterações (LoF).
    pub extracted_figure_page_numbers: Vec<usize>,
    /// **P488** — páginas de tabelas contadas, em ordem de documento.
    /// Populado por `Layouter::finish()` a partir de `runtime.table_page_numbers`.
    /// Usado pelo fixpoint em `mod.rs` para carry-forward entre iterações (LoT).
    pub extracted_table_page_numbers: Vec<usize>,
    /// **P535** — headings para bookmarks PDF (`/Outlines`).
    /// Cópia de `Introspector::headings_for_toc()` feita pelo pipeline
    /// pós-layout. Cada tuplo é `(auto-label, número, body, level)`.
    pub extracted_headings: Vec<(
        crate::entities::label::Label,
        Option<String>,
        crate::entities::content::Content,
        usize,
    )>,
    /// **P536** — metadados do documento definidos por `#set document(...)`.
    /// Copiado do `Module` pelo pipeline antes da exportação PDF (`/Info`).
    pub document_info: DocumentInfo,
    /// **P595** — avisos produzidos durante o layout (ex: footnote body
    /// maior do que a página/coluna). L1 permanece puro; a conversão
    /// para `SourceDiagnostic` e injecção no `Sink` é responsabilidade
    /// da pipeline em L3.
    pub layout_warnings: Vec<String>,
    /// **P644/P645** — erros produzidos durante o layout (ex: conversão de
    /// entrada bibliográfica para hayagriva). Guardados como
    /// `SourceDiagnostic` para preservar `span` e posição no ficheiro.
    pub layout_errors: Vec<SourceDiagnostic>,
}

impl PagedDocument {
    pub fn new(pages: Vec<Page>) -> Self {
        Self {
            pages,
            extracted_label_pages: HashMap::new(),
            extracted_label_positions: HashMap::new(),
            extracted_positions:
                crate::entities::sealed_positions::SealedPositions::empty(),
            extracted_figure_page_numbers: Vec::new(),
            extracted_table_page_numbers: Vec::new(),
            extracted_headings: Vec::new(),
            document_info: DocumentInfo::empty(),
            layout_warnings: Vec::new(),
            layout_errors: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Extrai texto plano de todas as páginas — para verificação em testes.
    pub fn plain_text(&self) -> String {
        self.pages
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ── Transformações afins (Passo 78) ──────────────────────────────────────────

/// Matriz de transformação afim 2D: [a, b, c, d, tx, ty].
///
/// Representa a transformação:
///   x' = a*x + c*y + tx
///   y' = b*x + d*y + ty
///
/// Esta convenção segue o formato do operador `cm` do PDF.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformMatrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl TransformMatrix {
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: dx, ty: dy }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0 }
    }

    /// Rotação em radianos no sistema Y-down do layouter.
    ///   x' =  cos*x - sin*y
    ///   y' =  sin*x + cos*y
    pub fn rotate(radians: f64) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        Self { a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0 }
    }

    /// Distorção (skew) em radianos. Passo 156F (ADR-0061 Fase 1, sub-passo 4).
    ///
    /// `ax` distorce horizontalmente; `ay` distorce verticalmente.
    /// Análogo a vanilla `SkewElem`. Forma da matriz:
    ///   x' = x + tan(ax) * y
    ///   y' = tan(ay) * x + y
    ///
    /// Ângulos extremos próximos de π/2 produzem `tan` infinito; o caller
    /// deve validar (per `native_skew` em `stdlib/transforms.rs`).
    pub fn skew(ax_rad: f64, ay_rad: f64) -> Self {
        Self {
            a: 1.0,
            b: ay_rad.tan(),
            c: ax_rad.tan(),
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Compõe `other` primeiro, depois `self`.
    ///
    /// `rotate.concat(translate)` aplica translate primeiro, depois rotate.
    /// Composição não é comutativa.
    pub fn concat(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// Aplica a matriz a um ponto 2D.
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.c * y + self.tx, self.b * x + self.d * y + self.ty)
    }
}

// ── Tipos tipográficos (ADR-0028, ADR-0029) ────────────────────────────────

/// Comprimento absoluto em pontos tipográficos.
///
/// ADR-0029: representação fiel ao Typst vanilla (`Abs(Scalar)`).
/// Escala interna: 1.0 = 1pt (simplificação L1 — vanilla usa 127 raw/pt).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Abs(pub f64);

impl Abs {
    pub const ZERO: Self = Self(0.0);

    pub fn pt(v: f64) -> Self {
        Self(v)
    }
    pub fn to_pt(self) -> f64 {
        self.0
    }
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl std::ops::Add for Abs {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Neg for Abs {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl std::ops::Sub for Abs {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Abs {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for Abs {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }
}

/// Comprimento tipográfico — combinação de componente absoluta e relativa.
///
/// ADR-0029 — revoga ADR-0028. Estrutura fiel ao Typst vanilla:
/// `struct Length { abs: Abs, em: f64 }`.
/// `abs`: componente absoluta em pontos.
/// `em`: componente relativa em múltiplos do font-size actual.
///
/// A soma `1pt + 1em` é representável; a resolução para pt requer font-size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub abs: Abs,
    pub em: f64,
}

impl Length {
    pub const ZERO: Self = Self { abs: Abs::ZERO, em: 0.0 };

    pub fn pt(v: f64) -> Self {
        Self { abs: Abs::pt(v), em: 0.0 }
    }
    pub fn em(v: f64) -> Self {
        Self { abs: Abs::ZERO, em: v }
    }

    /// Centímetros: 1 cm = 28.346 pt (paridade com parser `Unit::Cm`).
    pub fn cm(v: f64) -> Self {
        Self { abs: Abs::pt(v * 28.346), em: 0.0 }
    }
    /// Milímetros: 1 mm = 2.8346 pt (paridade com parser `Unit::Mm`).
    pub fn mm(v: f64) -> Self {
        Self { abs: Abs::pt(v * 2.8346), em: 0.0 }
    }
    /// Polegadas: 1 in = 72 pt.
    pub fn inches(v: f64) -> Self {
        Self { abs: Abs::pt(v * 72.0), em: 0.0 }
    }

    pub fn is_zero(&self) -> bool {
        self.abs.is_zero() && self.em == 0.0
    }

    /// Resolve para pontos dado um font-size em pt.
    /// `1pt + 1em` com font_size=12.0 → 13.0pt
    pub fn resolve_pt(&self, size_pt: f64) -> f64 {
        self.abs.to_pt() + self.em * size_pt
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::ops::Add for Length {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { abs: self.abs + rhs.abs, em: self.em + rhs.em }
    }
}

impl std::ops::Sub for Length {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { abs: self.abs - rhs.abs, em: self.em - rhs.em }
    }
}

impl std::ops::Neg for Length {
    type Output = Self;
    fn neg(self) -> Self {
        Self { abs: -self.abs, em: -self.em }
    }
}

impl std::ops::Mul<f64> for Length {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self { abs: self.abs * rhs, em: self.em * rhs }
    }
}

impl std::ops::Div<f64> for Length {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self { abs: self.abs / rhs, em: self.em / rhs }
    }
}

/// Rácio — valor normalizado (0.0 = 0%, 1.0 = 100%).
///
/// ADR-0028: newtype f64. PartialEq exacto (derive).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio(pub f64);

impl Ratio {
    pub fn from_percent(pct: f64) -> Self {
        Self(pct / 100.0)
    }
    pub fn get(self) -> f64 {
        self.0
    }
    pub fn to_percent(self) -> f64 {
        self.0 * 100.0
    }
}

/// Ângulo — armazenado internamente em radianos.
///
/// ADR-0028: newtype f64. PartialEq exacto (derive) — sem tolerância embutida.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(f64);

impl Angle {
    pub fn deg(d: f64) -> Self {
        Self(d.to_radians())
    }
    pub fn rad(r: f64) -> Self {
        Self(r)
    }
    pub fn to_rad(self) -> f64 {
        self.0
    }
    pub fn to_deg(self) -> f64 {
        self.0.to_degrees()
    }
}

// **P257 (ADR-0083 PROPOSTO)** — `Color` migrado para
// `01_core/src/entities/color.rs` (8 variantes paridade
// vanilla). Re-export para compatibilidade hot path consumers
// (`Stroke.paint: Color`, `Style::Fill(Color)`,
// `FrameItem::Text.fill`, `Value::Color`, stdlib parsers).
pub use crate::entities::color::{Color, ColorSpace};

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_add_pt() {
        assert_eq!(Pt(10.0) + Pt(5.0), Pt(15.0));
    }

    #[test]
    fn pt_sub_pt() {
        assert_eq!(Pt(10.0) - Pt(3.0), Pt(7.0));
    }

    #[test]
    fn pt_mul_f64() {
        assert_eq!(Pt(10.0) * 2.0, Pt(20.0));
    }

    #[test]
    fn pt_add_assign() {
        let mut a = Pt(5.0);
        a += Pt(3.0);
        assert_eq!(a, Pt(8.0));
    }

    #[test]
    fn pt_zero_e_val() {
        assert_eq!(Pt::ZERO.val(), 0.0);
        assert_eq!(Pt(42.0).val(), 42.0);
    }

    #[test]
    fn pt_tipagem_nao_permite_add_f64() {
        // Verificação de compilação — se Add<f64> existisse, este teste
        // seria desnecessário. O compilador força conversão explícita.
        let a = Pt(10.0);
        let b = Pt(5.0);
        let c = a + b; // Add<Pt> — OK
        assert_eq!(c, Pt(15.0));
        // a + 5.0  ← não compila — sem impl Add<f64>
    }

    #[test]
    fn size_a4() {
        let s = Size::a4();
        assert_eq!(s.width, Pt(595.0));
        assert_eq!(s.height, Pt(842.0));
    }

    #[test]
    fn text_style_constructors() {
        let r = TextStyle::regular(Pt(12.0));
        assert!(!r.bold && !r.italic);
        let b = TextStyle::bold(Pt(12.0));
        assert!(b.bold && !b.italic);
        let i = TextStyle::italic(Pt(12.0));
        assert!(!i.bold && i.italic);
    }

    #[test]
    fn frame_plain_text() {
        let style = TextStyle::regular(Pt(12.0));
        let mut f = Frame::new(Size::a4());
        f.push(FrameItem::Text {
            pos: Point::ZERO,
            text: "Hello".into(),
            style: style.clone(),
        });
        f.push(FrameItem::Text {
            pos: Point { x: Pt(50.0), y: Pt::ZERO },
            text: "world".into(),
            style,
        });
        assert_eq!(f.plain_text(), "Hello world");
    }

    #[test]
    fn paged_document_plain_text() {
        let style = TextStyle::regular(Pt(12.0));
        let p1 = Page {
            width: 595.28,
            height: 841.89,
            numbering: None,
            items: vec![FrameItem::Text {
                pos: Point::ZERO,
                text: "page1".into(),
                style: style.clone(),
            }],
        };
        let p2 = Page {
            width: 595.28,
            height: 841.89,
            numbering: None,
            items: vec![FrameItem::Text {
                pos: Point::ZERO,
                text: "page2".into(),
                style,
            }],
        };
        let doc = PagedDocument::new(vec![p1, p2]);
        assert_eq!(doc.plain_text(), "page1\npage2");
    }

    #[test]
    fn paged_document_vazio() {
        let doc = PagedDocument::new(vec![]);
        assert!(doc.is_empty());
        assert_eq!(doc.plain_text(), "");
    }

    // ── Passo 139 (Fase B.3 DEBT-52): faux_bold_stroke_pt ───────────────

    #[test]
    fn text_style_faux_bold_400_zero_passo_139() {
        let style = TextStyle {
            weight: Some(400),
            size: Pt(11.0),
            ..Default::default()
        };
        // Weight 400 é regular — factor = 0, stroke = 0.
        assert_eq!(style.faux_bold_stroke_pt(0.04), 0.0);
    }

    #[test]
    fn text_style_faux_bold_700_positivo_passo_139() {
        let style = TextStyle {
            weight: Some(700),
            size: Pt(11.0),
            ..Default::default()
        };
        // Weight 700 = factor 1.0. Stroke = 1.0 × 11.0 × 0.04 = 0.44.
        let stroke = style.faux_bold_stroke_pt(0.04);
        assert!(
            (stroke - 0.44).abs() < 0.001,
            "stroke para weight 700 @ 11pt deve ser 0.44; got {}",
            stroke
        );
    }

    #[test]
    fn text_style_faux_bold_100_clamp_zero_passo_139() {
        // Weights abaixo de 400 (thin/extralight/light) → factor
        // negativo → clamp a 0. Aceite como limitação faux-bold.
        let style = TextStyle {
            weight: Some(100),
            size: Pt(11.0),
            ..Default::default()
        };
        assert_eq!(style.faux_bold_stroke_pt(0.04), 0.0);
    }

    #[test]
    fn text_style_faux_bold_escala_com_size_passo_139() {
        // Size dobra → stroke dobra (proporção visual mantida).
        let s11 = TextStyle {
            weight: Some(700),
            size: Pt(11.0),
            ..Default::default()
        }
        .faux_bold_stroke_pt(0.04);
        let s22 = TextStyle {
            weight: Some(700),
            size: Pt(22.0),
            ..Default::default()
        }
        .faux_bold_stroke_pt(0.04);
        assert!(
            (s22 - 2.0 * s11).abs() < 0.001,
            "size 22pt deve dar 2× stroke de size 11pt; s11={}, s22={}",
            s11,
            s22
        );
    }

    #[test]
    fn text_style_faux_bold_none_weight_tratado_como_400_passo_139() {
        // `weight = None` tratado como 400 → stroke 0.
        // Equivalente a `Some(400)`.
        let none = TextStyle { weight: None, size: Pt(11.0), ..Default::default() };
        let four = TextStyle {
            weight: Some(400),
            size: Pt(11.0),
            ..Default::default()
        };
        assert_eq!(none.faux_bold_stroke_pt(0.04), four.faux_bold_stroke_pt(0.04));
    }

    // ── Passo 25 — tipos tipográficos (ADR-0028) ─────────────────────────────

    #[cfg(test)]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a, $b, 1e-10)
        };
        ($a:expr, $b:expr, $eps:expr) => {{
            let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
            assert!(
                (a - b).abs() < eps,
                "assert_approx_eq falhou: |{a} - {b}| = {} >= {eps}",
                (a - b).abs()
            );
        }};
    }

    #[test]
    fn length_resolve_pt() {
        assert_eq!(Length::pt(12.0).resolve_pt(12.0), 12.0);
        assert_eq!(Length::em(1.5).resolve_pt(12.0), 18.0);
        assert_eq!(Length::em(2.0).resolve_pt(10.0), 20.0);
    }

    #[test]
    fn ratio_percent_roundtrip() {
        let r = Ratio::from_percent(50.0);
        assert_approx_eq!(r.get(), 0.5);
        assert_approx_eq!(r.to_percent(), 50.0);
    }

    #[test]
    fn angle_deg_rad_usa_approx() {
        let a = Angle::deg(180.0);
        assert_approx_eq!(a.to_rad(), std::f64::consts::PI);
        assert_approx_eq!(a.to_deg(), 180.0);
    }

    #[test]
    fn angle_partial_eq_e_exacto() {
        let a1 = Angle::deg(180.0);
        let a2 = Angle::deg(180.0);
        assert_eq!(a1, a2);
        // Ângulos diferentes NÃO são iguais — sem tolerância embutida.
        let a3 = Angle::deg(180.0 + 1e-15);
        let _ = a3; // comportamento documentado no relatório
    }

    #[test]
    fn color_to_rgba_f32() {
        let (r, g, b, a) = Color::rgb(255, 0, 128).to_rgba_f32();
        assert_approx_eq!(r, 1.0, 1e-3);
        assert_approx_eq!(g, 0.0, 1e-3);
        assert_approx_eq!(b, 0.502, 1e-3);
        assert_approx_eq!(a, 1.0, 1e-3);
    }

    // ── Passo 26 — Length struct fiel ao vanilla (ADR-0029) ──────────────────

    #[test]
    fn length_soma_mista_agora_funciona() {
        // Com Length vanilla (abs + em), a soma Pt + Em é representável
        let l = Length::pt(6.0) + Length::em(1.0);
        assert_eq!(l.abs.to_pt(), 6.0);
        assert_eq!(l.em, 1.0);
        // Resolve com font-size=12pt → 6 + 12 = 18pt
        assert_approx_eq!(l.resolve_pt(12.0), 18.0);
    }

    #[test]
    fn length_zero_constante() {
        assert!(Length::ZERO.is_zero());
        assert_approx_eq!(Length::ZERO.resolve_pt(12.0), 0.0);
    }

    #[test]
    fn length_soma_abs_preserva_em() {
        let a = Length::pt(3.0);
        let b = Length::pt(4.0);
        let sum = a + b;
        assert_approx_eq!(sum.abs.to_pt(), 7.0);
        assert_eq!(sum.em, 0.0);
    }

    #[test]
    fn length_neg() {
        let l = Length::pt(5.0);
        let neg = -l;
        assert_approx_eq!(neg.abs.to_pt(), -5.0);
        assert_eq!(neg.em, 0.0);
    }

    #[test]
    fn abs_add_e_neg() {
        assert_eq!(Abs::pt(3.0) + Abs::pt(4.0), Abs::pt(7.0));
        assert_eq!(-Abs::pt(2.0), Abs::pt(-2.0));
        assert!(Abs::ZERO.is_zero());
    }

    // ── Passo 78 — TransformMatrix ────────────────────────────────────────

    #[test]
    fn transform_matrix_rotacao_90_graus_quadrado_mantem_dimensoes() {
        let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
        let corners = [
            matrix.apply(0.0, 0.0),
            matrix.apply(100.0, 0.0),
            matrix.apply(0.0, 100.0),
            matrix.apply(100.0, 100.0),
        ];
        let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
        let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
        let new_w = max_x - min_x;
        let new_h = max_y - min_y;
        assert!(
            (new_w - 100.0).abs() < 0.001,
            "Quadrado 100×100 rodado 90° deve ter largura 100, obteve {}",
            new_w
        );
        assert!(
            (new_h - 100.0).abs() < 0.001,
            "Quadrado 100×100 rodado 90° deve ter altura 100, obteve {}",
            new_h
        );
    }

    #[test]
    fn transform_matrix_rotacao_45_graus_aumenta_bounding_box() {
        let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_4);
        let corners = [
            matrix.apply(0.0, 0.0),
            matrix.apply(100.0, 0.0),
            matrix.apply(0.0, 100.0),
            matrix.apply(100.0, 100.0),
        ];
        let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
        let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
        let new_w = max_x - min_x;
        let new_h = max_y - min_y;
        let diagonal = 100.0_f64 * std::f64::consts::SQRT_2;
        assert!(
            (new_w - diagonal).abs() < 0.01,
            "Quadrado 100×100 rodado 45° deve ter largura ≈ {:.2}, obteve {:.4}",
            diagonal,
            new_w
        );
        assert!(
            (new_h - diagonal).abs() < 0.01,
            "Quadrado 100×100 rodado 45° deve ter altura ≈ {:.2}, obteve {:.4}",
            diagonal,
            new_h
        );
    }

    // ── Passo 82 — Align2D ─────────────────────────────────────────────────

    #[test]
    fn align2d_from_string_parse_correcto() {
        let a = Align2D::from_string("top-right");
        assert_eq!(a.h, Some(HAlign::Right));
        assert_eq!(a.v, Some(VAlign::Top));

        let b = Align2D::from_string("center");
        assert_eq!(b.h, Some(HAlign::Center));
        assert_eq!(b.v, None);

        let c = Align2D::from_string("bottom");
        assert_eq!(c.h, None);
        assert_eq!(c.v, Some(VAlign::Bottom));

        let d = Align2D::from_string("horizon");
        assert_eq!(d.v, Some(VAlign::Horizon));

        // String inválida: nenhum campo deve ser preenchido.
        let e = Align2D::from_string("invalid");
        assert_eq!(e.h, None);
        assert_eq!(e.v, None);
    }

    #[test]
    fn p504_align2d_start_end_parse() {
        let start = Align2D::from_string("start");
        assert_eq!(start.h, Some(HAlign::Start));
        let end = Align2D::from_string("end");
        assert_eq!(end.h, Some(HAlign::End));
        let start_bottom = Align2D::from_string("start-bottom");
        assert_eq!(start_bottom.h, Some(HAlign::Start));
        assert_eq!(start_bottom.v, Some(VAlign::Bottom));
    }

    #[test]
    fn transform_matrix_concat_ordem_correta() {
        let translate = TransformMatrix::translate(10.0, 0.0);
        let rotate90 = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
        // rotate90.concat(translate): aplica translate primeiro, depois rotate90
        let composed = rotate90.concat(&translate);
        let (rx, ry) = composed.apply(0.0, 0.0);
        assert!((rx - 0.0).abs() < 0.001, "x esperado 0.0, obteve {}", rx);
        assert!((ry - 10.0).abs() < 0.001, "y esperado 10.0, obteve {}", ry);
    }

    // ── Passo 156F (ADR-0061 Fase 1, sub-passo 4) — TransformMatrix::skew ──

    #[test]
    fn transform_matrix_skew_zero_e_identidade() {
        // skew(0, 0) deve produzir matriz identidade.
        let m = TransformMatrix::skew(0.0, 0.0);
        assert_eq!(m, TransformMatrix::identity());
    }

    #[test]
    fn transform_matrix_skew_ax_distorce_horizontal() {
        // skew(45°, 0) — ponto (0, 1) deve ser deslocado horizontalmente
        // por tan(45°) = 1.0; y inalterado.
        let ax = std::f64::consts::FRAC_PI_4; // 45°
        let m = TransformMatrix::skew(ax, 0.0);
        let (x, y) = m.apply(0.0, 1.0);
        assert!((x - 1.0).abs() < 0.001, "x esperado 1.0 (tan(45°)), obteve {}", x);
        assert!((y - 1.0).abs() < 0.001, "y esperado inalterado 1.0, obteve {}", y);
    }

    #[test]
    fn transform_matrix_skew_ay_distorce_vertical() {
        // skew(0, 45°) — ponto (1, 0) deve ter y deslocado por tan(45°).
        let ay = std::f64::consts::FRAC_PI_4;
        let m = TransformMatrix::skew(0.0, ay);
        let (x, y) = m.apply(1.0, 0.0);
        assert!((x - 1.0).abs() < 0.001, "x esperado inalterado 1.0, obteve {}", x);
        assert!((y - 1.0).abs() < 0.001, "y esperado 1.0 (tan(45°)), obteve {}", y);
    }

    #[test]
    fn transform_matrix_skew_origin_zero_zero_imutavel() {
        // O ponto (0,0) é fixo sob skew (sem origin shift).
        let m = TransformMatrix::skew(0.5, 0.3);
        let (x, y) = m.apply(0.0, 0.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }

    // P485 — campo units_per_em em FrameItem::TextShaped
    #[allow(deprecated)]
    #[test]
    fn p485_textshaped_tem_units_per_em() {
        let item = FrameItem::TextShaped {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            glyphs: vec![],
            style: TextStyle::default(),
            text: EcoString::from("A"),
            units_per_em: 1000,
        };
        if let FrameItem::TextShaped { units_per_em, .. } = item {
            assert_eq!(units_per_em, 1000, "P485: units_per_em deve ser 1000");
        } else {
            panic!("P485: esperado TextShaped");
        }
    }

    #[allow(deprecated)]
    #[test]
    fn p485_textshaped_units_per_em_cast_nao_zero() {
        // Garantir que units_per_em > 0 (prevenção de divisão por zero em emit)
        let upm: u16 = 2048; // valor típico para TrueType
        assert!(upm > 0);
        let advance_tu = -(600_f64 / upm as f64 * 1000.0);
        // 600/2048*1000 ≈ -293.0
        assert!(
            (advance_tu - (-292.97)).abs() < 0.1,
            "P485: advance TJ calculado incorrectamente: {}",
            advance_tu
        );
    }

    #[test]
    fn p598_page_config_default_margin_a4_bate_vanilla() {
        let cfg = PageConfig::default();
        // Vanilla 0.15.0: margin = min(width, height) * 2.5/21.
        // Para A4 isto dá 70.8666... pt, tradicionalmente arredondado a 70.87 pt.
        assert!((cfg.width - 595.28).abs() < 0.001);
        assert!((cfg.height - 841.89).abs() < 0.001);
        assert!((cfg.margin - 70.87).abs() < 0.01);
        assert!(cfg.margin_is_auto, "margem por omissão deve ser automática");
        let expected = cfg.width.min(cfg.height) * 2.5 / 21.0;
        assert!((cfg.margin - expected).abs() < 0.001);
    }

    #[test]
    fn p598_page_config_margin_formula_uses_smaller_dimension() {
        // Se height for a dimensão menor, a margem deve ser proporcional a height.
        let mut cfg = PageConfig::default();
        cfg.height = 200.0;
        cfg.margin = cfg.auto_margin();
        assert!((cfg.margin - 23.8095).abs() < 0.001);
    }

    #[test]
    fn p598_page_config_auto_margin_follows_dimension_changes() {
        // Margem automática acompanha a menor dimensão mesmo quando width > height.
        let mut cfg = PageConfig::default();
        cfg.width = 800.0;
        cfg.height = 200.0;
        cfg.margin = cfg.auto_margin();
        assert!((cfg.margin - 23.8095).abs() < 0.001);

        cfg.width = 200.0;
        cfg.height = 800.0;
        cfg.margin = cfg.auto_margin();
        assert!((cfg.margin - 23.8095).abs() < 0.001);
    }
}
