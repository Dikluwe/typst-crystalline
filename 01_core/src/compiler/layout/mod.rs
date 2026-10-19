//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/mod.md
//! @prompt-hash c1bdc984
//! @layer L1
//! @updated 2026-10-07

mod checkpoint;
mod convergence;
pub mod counters;
pub mod figure;
pub mod image;
pub mod outline;
mod position_marker;
use self::checkpoint::FlowSuffixCheckpoint;
#[cfg(test)]
use self::checkpoint::RegionTailCheckpoint;
mod finish;
mod page_canvas;
mod page_commit;
mod page_geometry;
mod page_run;
mod page_running;
pub mod references;
pub mod vanilla_defaults;

use crate::compiler::introspect::locatable::is_locatable;
use crate::entities::{
    content::Content,
    counter_format::count_numbering_tokens,
    geometry::ShapeKind,
    image_sizer::{ImageSizer, NullImageSizer},
    label::Label,
    layout_types::{
        FrameItem, HAlign, Length, Page, PageConfig, PagedDocument, Pt, TextStyle,
    },
    location::Location,
    locator::{Locator, LocatorSeed},
    source_result::{SourceDiagnostic, SourceResult},
    style_chain::StyleChain,
};
use ecow::EcoString;
use hayagriva::citationberg::IndependentStyle;
use std::sync::Arc;

/// P1133 — fronteira única `ShapeKind<Length> → ShapeKind<Pt>`.
/// O layout ainda possui o font-size que dá significado ao componente `em`;
/// Frame/export transportam somente geometria absoluta.
fn resolve_shape_kind(kind: &ShapeKind<Length>, em: Pt) -> ShapeKind<Pt> {
    match kind {
        ShapeKind::Rect => ShapeKind::Rect,
        ShapeKind::RoundedRect { radii } => ShapeKind::RoundedRect {
            radii: crate::entities::corners::Corners::new(
                Pt(radii.top_left.resolve_pt(em.val())),
                Pt(radii.top_right.resolve_pt(em.val())),
                Pt(radii.bottom_right.resolve_pt(em.val())),
                Pt(radii.bottom_left.resolve_pt(em.val())),
            ),
        },
        ShapeKind::Ellipse => ShapeKind::Ellipse,
        ShapeKind::Line { dx, dy } => ShapeKind::Line { dx: *dx, dy: *dy },
        ShapeKind::Path(items) => ShapeKind::Path(items.clone()),
    }
}

// FontMetrics / FixedMetrics extraídos para metrics.rs (Passo 96.7, ADR-0037).
mod metrics;
pub use crate::compiler::layout::metrics::{
    needs_shaped_width, FixedMetrics, FontMetrics,
};
mod measurement;
mod realized;
pub use measurement::measure_content_real;
pub(in crate::compiler) use measurement::measure_content_real_with_baseline;
mod par_line_callbacks;
pub use par_line_callbacks::{
    LayoutCallbackRequests, LayoutPassOutcome, ParLineCallbackPassState,
    ParLineCallbackRequest, ParLineCallbackResolution, SealedParLineCallbacks,
};

// Braços pesados do `layout_content` extraídos por cluster (Passo 96.7).
mod equation;
mod grid;
mod placement;
mod style_scope;

// Sub-layout isolado de conteúdo numa região (Passo 629).
mod sub_frame;
// **P994** — re-export interno para `math/layout/mod.rs::layout_external`
// (o módulo `sub_frame` fica privado; só o tipo da região sai).
pub(crate) use sub_frame::SubLayoutRegion;

// Atomização dos elementos-container (ADR-0109, P376): o layout de cada
// container vive no seu arquivo; o `match` delega numa linha.
mod block;
mod boxed;
mod pad;
mod stack;

// Atomização dos elementos visuais (ADR-0109, P377): mesma forma B.
mod columns;
mod curve;
mod heading;
mod html_frame;
pub use html_frame::HtmlFrameLayout;
mod shape;
mod tiling;
mod title;
mod transform;

// Atomização visuais/decorações (ADR-0109, P378): mesma forma B.
// (Image/Figure completam-se nos seus próprios arquivos image.rs/figure.rs.)
mod decorations;
mod flush;
mod place;

// Atomização Fatia 1/2 (ADR-0109, P380): fluxo de bloco e estrutura.
// (Grid completa-se no próprio grid.rs, junto de layout_grid.)
mod colbreak;
mod enum_item;
mod grid_cell;
mod grid_footer;
mod grid_header;
mod h_space;
mod list_item;
mod pagebreak;
mod repeat;
mod table;
mod table_cell;
mod table_footer;
mod table_header;
mod term_item;
mod terms;
mod v_space;

// Atomização Fatia Text (ADR-0109, P381): a folha de render, isolada.
mod text;

// Atomização Fatia 2 (ADR-0109, P381): refs/citações + avulsos.
mod align;
pub mod bib_csl;
mod bibliography;
mod bidi_origin;
mod cite;
mod context_block;
mod counter_display;
mod counter_display_callback;
mod divider;
mod dynamic;
mod footnote;
mod hide;
mod linebreak;
mod link;
mod link_capture;
mod outline_entry;
mod paragraph;
mod paragraph_emit;
mod paragraph_hyphen;
mod paragraph_indent;
mod paragraph_pagination;
mod paragraph_plan;
mod paragraph_prepare;
mod paragraph_select;
#[cfg(test)]
mod paragraph_select_bounded;
#[cfg(test)]
mod paragraph_select_forward;
mod parbreak;
mod pdf_artifact;
mod pdf_attach;
mod quote;
mod raw;
mod sequence;
mod set_page;
mod smartquote;
mod space;
mod state_display;

// Helpers livres usados pelo Layouter e pelos braços extraídos.
pub(crate) mod helpers;
// P224.C — Placement algorítmico Grid (fecha DEBT-34e colspan/rowspan).
pub(crate) mod grid_placement;
// P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial) — slice frame
// items por threshold em pos.y para row break TableCell cell-level.
mod slicing;
use crate::compiler::layout::helpers::{item_pos, resolve_pt, translate_frame_item};

// Cursor: palavras/wrap, progressão de regiões e floats.
mod break_selection;
mod cell_tail_commit;
mod cursor;
mod flow_suffix;
mod region_progress;
mod text_baseline;
mod text_measure;
mod text_tracking;
mod word_layout;
// Fechamento de linha: expansão, justificação e commit (P1503).
mod line_commit;
mod word_segment;

// Flush dos footnote bodies pendentes (P847 — extraído de `cursor.rs`).
mod footnote_flush;

mod format_options;
pub use format_options::collect_document_format_options;

// Hyphenation puro (Passo 144, ADR-0057): wrap sobre `hypher`.
mod hyphenation;

// ── Constantes de página ───────────────────────────────────────────────────

const DEFAULT_FONT_SIZE: f64 = 12.0;

/// **P219 (DEBT-56 sub-fase b 3/4)** — Default gutter para
/// `Content::Columns` quando `gutter: None` (vanilla paridade ~4%
/// width). Aplicado em `layout_content` arm `Content::Columns` e
/// `measure_content_constrained` arm.
///
/// Anti-inflação 14ª aplicação cumulativa pós-P205D — Opção β
/// constante named (vs Opção γ helper privado). Magic number
/// 0.04 explícito para auditoria.
const COLUMNS_DEFAULT_GUTTER_RATIO: f64 = 0.04;

/// P1416 — coordenação escalar de uma linha transversal entre regiões irmãs
/// de `#columns()` segmentado. As compensações são aplicadas antes da
/// translação normal da coluna para alcançar os bordos externos do contentor.
#[derive(Debug, Clone, Copy)]
pub(super) struct SegmentedParLineOverlay {
    pub suppress_through: usize,
    pub start_x_shift: f64,
    pub end_x_shift: f64,
}

/// P1417 — marcador marginal textual ainda não consolidado pelo owner da
/// região segmentada. A baseline já está estabilizada; nenhum contador foi
/// consumido enquanto a candidata permanece neste buffer.
#[derive(Debug, Clone)]
pub(super) struct SegmentedParLinePatternCandidate {
    pub baseline: Pt,
    pub page: usize,
    pub pattern: EcoString,
    pub styles: StyleChain,
}

// ── Layouter ──────────────────────────────────────────────────────────────

/// **P864** — grupo de item estrutural reconhecido para agrupamento por
/// `Content::Parbreak` em sequências.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ItemGroup {
    List,
    Enum,
    Terms,
}

/// Máquina de estado de layout.
///
/// Consome `Content` e produz `PagedDocument`.
/// `font_size` é campo do Layouter — as métricas recebem-no por chamada
/// para suportar tamanhos mistos (rich text).
// Visibilidade `pub(super)` nos campos abaixo (Passo 96.7): os submódulos
// `grid.rs`, `placement.rs`, `equation.rs` recebem braços pesados do match
// `layout_content` e precisam de ler/escrever cursor, estado de célula,
// métricas e items acumulados. Criar getters/setters para cada acesso seria
// ruído sem ganho de invariante — a ADR-0037 Regra 3 autoriza `pub(super)`
// em campos quando métodos não agregam. A API externa (`pub fn layout`,
// `pub fn layout_content`) continua inalterada.
pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
    pub(super) metrics: M,
    sizer: S,

    /// Estilo activo resolvido — vista achatada de `self.chain` cacheada
    /// para evitar resolver em cada leitura de `.size` no hot path do layout.
    /// Mantido sincronizado com `self.chain` por cada push/pop (Passo 100,
    /// ADR-0039).
    pub(super) style: TextStyle,
    /// Primeiro idioma efetivo, inclusive sem glifos emitidos.
    document_language: Option<crate::entities::lang::Lang>,
    /// Cadeia de estilos activa — source-of-truth do estilo (Passo 100,
    /// ADR-0039). `Content::Styled` faz push; o save/restore de Strong/
    /// Emph/Heading/Text também passa por esta cadeia. `self.style` é a
    /// vista achatada (cache) que o layout consulta directamente.
    pub(super) chain: StyleChain,
    text_tracking_state: text_tracking::State,
    /// Configuração da página activa. Mutável via Content::SetPage (Passo 81).
    pub page_config: PageConfig,
    pub(super) pages: Vec<Page>,
    /// **P216A (DEBT-56 sub-fase a parte 1)** — Region agregando
    /// state geométrico previamente disperso em 5 fields escalares
    /// (`cursor_x`, `cursor_y`, `line_start_x`, `current_items`,
    /// `current_line`) + 2 dimensões (`width`/`height`).
    ///
    /// Caminho B1 fixado em P216A C4: `PageConfig.width/height`
    /// preservados; `region.width/height` é cópia derivada em
    /// `Layouter::new`. Redundância controlada por minimizar
    /// blast radius. P216B sub-fase (a) parte 2 introduz
    /// `Regions` wrapper; P219 consumer multi-column.
    ///
    /// Origem horizontal da linha actual (`region.line_start_x`):
    /// normalmente `Pt(page_config.margin)`. Em sub-layouts de
    /// células de Grid, toma o valor de `cell_x` para que
    /// `flush_line()` reinicie o cursor à origem da célula em
    /// vez da margem da página.
    /// P216B (DEBT-56 sub-fase a parte 2): agregação em `Regions`
    /// wrapper. Single-region em P216B (`current` = Region única);
    /// multi-region em P219 sub-fase (b) consumer (`Content::Columns`
    /// arm preenche `current` + futuros `backlog`/`last` quando
    /// emergir per anti-inflação 11ª aplicação cumulativa pós-P205D).
    pub(super) regions: crate::entities::region::Regions,
    /// P1140.11 — âncoras horizontais de espaços justificáveis na linha
    /// corrente. Geometria transitória privada do render, nunca contrato.
    pub(super) justify_opportunities: Vec<f64>,
    /// **P751** — indica que a baseline inicial da página/coluna ainda
    /// não foi fixada. Enquanto `true`, `cursor_y` representa o topo
    /// útil (margem) e `ensure_initial_baseline()` adiciona o
    /// `cap_height` do estilo realmente activo no momento do primeiro
    /// conteúdo real. Isto evita fixar o offset de baseline com o
    /// estilo por defeito em `Layouter::new` antes de processar
    /// `#set text(size: ...)` no início do documento.
    pub(super) initial_baseline_pending: bool,
    // P190I (M6 fechado): `counter: CounterStateLegacy` ELIMINADO —
    // struct eliminada. Layouter consumers usam Introspector path
    // puro via `self.introspector` (P184D / P190G/H/I migrations).
    /// P168 (M5 sub-passo 2): introspector populado paralelamente ao
    /// `counter` legacy. Consumer actual é `references.rs::layout_ref`
    /// (figure-ref); outros consumers migram em M9+.
    ///
    /// **P204C (M8)** — migrado de `TagIntrospector` por valor para
    /// `Tracked<'a, dyn Introspector + 'a>` per ADR-0073 (paridade
    /// vanilla literal). Caller constrói `TagIntrospector` + `.track()`
    /// e passa o handle a `Layouter::new`. Lifetime `'a` atado à
    /// fonte do tracked (introspector concreto deve outlive Layouter).
    pub(super) introspector:
        comemo::Tracked<'a, dyn crate::entities::introspector::Introspector + 'a>,
    /// Índice de progresso por kind para figuras (Passo 75, DEBT-14).
    /// kind → número de figuras já dispostas. Reiniciado por invocação de layout().
    figure_progress: std::collections::HashMap<String, usize>,
    /// Indica que o contexto de layout actual não tem altura delimitada
    /// (ex: célula de grid Auto, box sem height explícito). Passo 82.
    ///
    /// Quando true, `VAlign::Bottom` e `VAlign::Horizon` em `Content::Align`
    /// decaem para `VAlign::Top` — não existe "fundo" para ancorar.
    /// Definido como true por `layout_sub_frame` e restaurado
    /// ao regressar ao contexto pai.
    pub(super) is_height_unconstrained: bool,
    /// **P748** — true quando o Layouter está dentro de um sub-layout
    /// isolado (`layout_sub_frame`). Usado por `shape.rs` para saber se
    /// o cursor_y já foi compensado pelo ascender pelo sub-frame (caso
    /// em que não deve voltar a subtrair ascender ao posicionar formas).
    pub(super) is_sub_frame: bool,
    pub(super) relative_region_size: Option<(f64, f64)>,
    /// Vertical origin expected when recomposing this sub-frame into its parent.
    pub(super) sub_frame_origin_y: f64,
    /// Physical flow end and the prepared cursor that certifies it.
    pub(super) flow_placement_end: Option<(f64, f64)>,
    /// Logical origin and occupied width of the active flow scope.
    pub(super) flow_frame_extent: (f64, f64),
    /// Causal width returned by the most recent isolated subframe.
    pub(super) last_sub_frame_flow_width: f64,
    /// Inner Block region; independent of the page and cell references.
    pub(super) block_place_region: Option<crate::entities::layout_types::Rect>,
    /// Available width is a constraint; the current Block allocation is intrinsic.
    pub(super) block_auto_width: bool,
    /// Deferred Y entry index and its local decoration segment range.
    pub(super) block_place_decorations: Vec<(usize, usize, usize)>,
    // **P246 (cell layout migration)** — fields `cell_available_h` +
    // `cell_origin_w` migrados para `self.regions.cell: Option<Region>`
    // (entity-side; `Region.height` + `Region.width`). Reader pattern:
    // `self.regions.effective().height/width`. Activa A.4 breakable
    // per-cell arquiteturalmente (activação real diferida a passo
    // futuro não-reservado per política P158).
    //
    /// Coordenadas X/Y do canto superior esquerdo da célula activa.
    /// Passo 84.6 (encerra DEBT-37). **P246**: preservados como
    /// Layouter fields legacy — `Region` actual sem `origin: Point`
    /// (cell origin absoluto em pt na página exige fields paralelos);
    /// refactor futuro com `Region.origin` permitirá eliminar.
    ///
    /// Quando todos `Some` em conjunto com `regions.cell.is_some()`,
    /// `Content::Place { scope: Column, .. }` ancora à célula.
    /// Salvos e restaurados por célula no braço `Content::Grid`.
    pub(super) cell_origin_x: Option<f64>,
    pub(super) cell_origin_y: Option<f64>,
    /// **P273.5** — bbox do contentor imediato para resolver
    /// `Gradient.relative: Some(RelativeTo::Parent)` no callsite emit.
    /// Padrão DEBT-37 P84.6 (campo opcional Layouter para contexto pai)
    /// reused estructuralmente.
    ///
    /// **3γ.1 materializado P273.5**: `None` default; callsite L3 emit
    /// gradient usa `page_bbox` como fallback (cobre semântica vanilla
    /// "shape top-level com relative=Parent ancora à página").
    ///
    /// **3γ.2 materializada P273.6**: arm `Content::Block` save/restore
    /// real do campo (write quando dimensions literais Decisão 3γ.2.γ);
    /// emit shape sites populam `FrameItem::Shape.parent_bbox_at_emit:
    /// self.parent_bbox` (read).
    ///
    /// **3γ.2.γ Decisão Fase A P273.6**: popular apenas quando
    /// `width.is_some() && height.is_some()` no Block; caso ambíguo
    /// (auto/fr) cai no fallback page_bbox L3 P273.5 via LIFO restore.
    ///
    /// Boxed diferido P273.7 se necessário; Stack/Pad/Group/Grid cell
    /// scope-out per ADR-0054 graded.
    pub(super) parent_bbox: Option<crate::entities::layout_types::Rect>,
    /// **P232 (Fase 5 Layout Categoria A.5)** — alignment Grid-level
    /// disponível para `Content::Place` herdar via `.or()` per eixo
    /// quando dentro Grid context. Save/restore paridade cell_origin_*
    /// no braço `Content::Grid` em layout_grid. None fora Grid context
    /// (Place baseline P84.5 preservado).
    pub(super) cell_align: Option<crate::entities::layout_types::Align2D>,
    /// **P185C (mecanismo M3 da ADR-0068)** — gerador determinístico
    /// de `Location`s, sincronizado-por-construção com o `Locator`
    /// do walk de introspect (per P185A §3.3). Avança em cada chamada
    /// a `layout_content` cujo content satisfaz `is_locatable`.
    /// Nenhum consumer ainda — fica para P187/P188.
    pub(super) locator: Locator,
    locations_reserved: bool,
    /// **P185C** — `Location` do último content locatable processado.
    /// `None` antes de processar qualquer locatable. Consumers
    /// location-aware (`flat_counter_at`, P185B) consultam este campo
    /// em vez de snapshot final (cf. ADR-0068 PROPOSTO). (F-4 E0, P338:
    /// `is_numbering_active_at` saiu — gate vive no campo assado.)
    pub(super) current_location: Option<Location>,
    /// P1493 — emite carriers geométricos privados para ContextBlock nesta passagem.
    pub(super) capture_html_context_markers: bool,
    position_captures: Vec<position_marker::Target>,
    link_capture: link_capture::Capture,
    /// Callback state for this concrete layout attempt. It is borrowed from
    /// the entrypoint and never escapes L1.
    pub(super) math_callback_pass:
        Option<&'a crate::compiler::math::layout::callbacks::MathCallbackPassState>,
    /// Callback state de `par.line` para esta tentativa concreta.
    pub(super) par_line_callback_pass: Option<&'a ParLineCallbackPassState>,
    /// **P190C (M6 categoria Page tracking)** — state Layouter-runtime
    /// dedicado. Campos `label_pages` + `known_page_numbers` movidos
    /// de `CounterStateLegacy` para `LayouterRuntimeState` por não
    /// serem derivados de Content pre-pass (Layouter-runtime apenas).
    /// Pattern arquitectural "Layouter-runtime → struct dedicada"
    /// estabelecido em P190C; replicado em P190D para `is_readonly`
    /// + `lang`.
    pub runtime: crate::entities::layouter_runtime_state::LayouterRuntimeState,
    /// **P245 (M9d / M7+4)** — buffer de floats pendentes na página
    /// actual; flush em `new_page()` + `finish()`. Reset após flush
    /// (próxima página inicia com buffer vazio). Promoção real graded
    /// P223 `Content::Place.float` → semantic activa P245.
    pub(super) floats_pending: Vec<DeferredFloat>,
    /// **P245 (M9d / M7+4)** — espaço reservado no topo da página
    /// para floats top-aligned acumulados. Flow regular começa abaixo
    /// (cursor_y inicial += reserve). Reset em `new_page`.
    pub(super) cursor_y_top_reserve: f64,
    /// **P245 (M9d / M7+4)** — espaço reservado no fundo da página
    /// para floats bottom-aligned. Flow regular evita zona reservada
    /// no overflow check (`cursor_y > height - margin -
    /// cursor_y_bottom_reserve`). Reset em `new_page`.
    pub(super) cursor_y_bottom_reserve: f64,
    /// **P793** — Contador de itens de enum sequenciais.
    pub(super) enum_counter: Option<u32>,
    /// P1414 — estado privado da numeração das linhas físicas do fluxo raiz.
    /// Participa do rollback de cauda; subframes não o leem nem o alteram.
    pub(super) par_line_counter: usize,
    pub(super) par_line_page: Option<usize>,
    /// **P250 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa; cita
    /// ADR-0082 PROPOSTO N=1)** — below pendente do bloco anterior
    /// para CSS-style margin collapse `max(prev.below, curr.above)`
    /// entre blocks consecutivos. Reset por Sequence consumer +
    /// non-Block arms.
    pub(super) prev_block_below_pending: f64,
    /// P1374 — cota calculada pelo `Sequence` para o bloco fracionário
    /// directo em composição. `None` fora dessa ocorrência.
    pub(super) fractional_block_height: Option<f64>,
    /// Guarda do preflight puro P1374; impede distribuição aninhada/reentrada.
    pub(super) fractional_preflight_active: bool,
    /// A próxima linha fecha uma distribuição `fr`; o overflow considera a
    /// tinta confirmada, não a baseline hipotética seguinte.
    pub(super) fractional_flow_tail_pending: bool,
    /// **P250** — `true` se o elemento previamente laid out foi
    /// `Content::Block`. Used by `prev_block_below_pending` collapse
    /// logic + first-block-in-sequence above suppression. Reset por
    /// non-Block arms (via Sequence consumer ou directamente).
    pub(super) block_chain_active: bool,
    /// **P1061** — `true` se a margem pendente veio de `Content::Parbreak` (weakness 4)
    /// vs `false` se veio de `Content::Block` (weakness 3).
    pub(super) prev_margin_is_parbreak: bool,
    /// **P813** — avanço vertical aplicado pelo último `flush_line()`
    /// com items (`top + |bottom| + leading`). Permite a consumidores
    /// posteriores (equações de bloco) recuperar a baseline da linha
    /// anterior como `cursor_y - last_flush_advance`. Reset em
    /// `new_page`; save/restore em `layout_sub_frame`.
    /// **P1088** — baseline da última linha / cabeçalho / equação layoutada.
    /// Usada no protocolo genérico de colapso de margens de bloco.
    pub(super) prev_line_baseline: f64,
    pub(super) terminal_paragraph_line: bool,
    pub(super) empty_paragraph_weak_spacing: f64,
    pub(super) last_flush_advance: f64,
    /// **P952** — `descent_ink` da última equação de bloco layoutada
    /// (0.0 quando o conteúdo anterior não é equação de bloco). Somada à
    /// recuperação de baseline em `layout_equation` para que o espaçamento
    /// equação→equação fique aresta-a-aresta como o vanilla (fundo do frame
    /// anterior + 1.2em colapsado + topo do seguinte). Reset nos mesmos
    /// pontos de `last_flush_advance` (`cursor.rs`, `sub_frame.rs`).
    pub(super) prev_block_equation_descent: f64,
    pub(super) last_equation_descent: f64,
    pub(super) last_block_descent_y: Option<f64>,
    /// **P1120** — extensões verticais da linha em curso contribuídas por
    /// items inline com caixa própria (`#box` com `height`, `#rotate`/
    /// `#scale`/`#skew`), medidas a partir da baseline da linha.
    ///
    /// O vanilla alinha os items inline pela baseline e a linha fica com
    /// `altura = max(ascent_i) + max(descent_i)` — para uma caixa de altura
    /// explícita `h`, `ascent` é o ascent do próprio body (o body é colocado
    /// no topo da caixa e a baseline da caixa é a do body) e `descent` é
    /// `h − ascent`. Sem estes acumuladores a linha só conhecia as arestas do
    /// texto (`text_edges`), pelo que uma caixa mais alta que o texto não
    /// descia a baseline nem alargava a linha (medido em `.typ/sec_36.typ`:
    /// baseline 8,10 pt acima do vanilla e página 8,10 pt mais curta).
    ///
    /// Reset em `flush_line` (a linha fecha) e no construtor.
    pub(super) line_inline_ascent: f64,
    pub(super) line_inline_descent: f64,
    /// **P1120** — ascent assumido quando a baseline da linha em curso foi
    /// fixada (aresta superior do texto). `flush_line` compara-o com
    /// `line_inline_ascent` para saber quanto a linha tem de descer.
    /// `None` ainda não instala ascent; `Some(0.0)` instala baseline na origem.
    pub(super) line_assumed_ascent: Option<f64>,
    /// P1391 — arm one-shot para o flush automático de uma primeira palavra
    /// overlong certificada por Sequence.
    pub(super) wrapped_inline_prefix_line_armed: bool,
    /// **P1120** — fundo do último sub-frame layoutado (`last_block_descent_y`
    /// interno), em coordenadas do sub-frame. Permite ao chamador derivar o
    /// descent real do frame (`fundo − baseline`) — as arestas de fonte não
    /// servem para equações, cujo frame é medido pelas extensões da fórmula.
    pub(super) last_sub_frame_bottom: Option<f64>,
    pub(in crate::compiler) last_sub_frame_width: f64,
    pub(super) last_sub_frame_inline_extents: (f64, f64),

    /// **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita
    /// ADR-0082 PROPOSTO N=2 segunda aplicação citante)** — buffer
    /// de tails de cells que overflow a altura disponível. Flush em
    /// `new_page()` (após `flush_pending_floats` P245) + `finish()`.
    /// Paridade arquitectural ao P245 `floats_pending` (subpadrão
    /// "DeferredX buffer + flush em new_page" N=1 → 2 cumulativo).
    pub(super) pending_cell_tails: Vec<DeferredCellTail>,
    /// **P304 (P295.1; ADR-0079 Categoria C.2 paralela)** — buffer
    /// de footnote bodies pendentes na página actual. Populado pelo
    /// arm `Content::Footnote` quando o marker `[N]` é emitido; flush
    /// em `new_page()` (antes de saving a Page) + `finish()` (última
    /// página). Cada entry: `(número, body)`. Bodies são layoutados
    /// no rodapé via `layout_sub_frame` + posicionamento
    /// absoluto Y bottom.
    /// Sub-padrão "DeferredX buffer + flush em new_page" N=2 → 3
    /// cumulativo (P245 floats + P251 cell tails + P304 footnotes).
    pub(super) pending_footnote_bodies:
        Vec<(u32, Box<crate::entities::content::Content>)>,
    /// **P286 (frente `P-text-deco-multiline`; resolve P284 §5.3)** —
    /// collector opcional de segmentos `(start_x, end_x, baseline_y)`
    /// para decorações textuais wrap-aware (Underline/Strike/Overline).
    /// `flush_line` consulta este campo no início e, se `Some`, regista
    /// um segmento com o estado da linha que está a fechar. O consumer
    /// das decorações activa antes de `layout_content(body)`, drena
    /// no fim e emite 1 `FrameItem::Line` por segment + 1 para a linha
    /// "final não-flushed". `None` por default → flush_line ignora
    /// (zero overhead em todos os outros call-sites; backward-compat
    /// bit-exact preservado per regressão tests P285).
    pub(super) decoration_lines_collector: Option<Vec<DecoSegment>>,
    /// Contexto lógico das aspas após show, por fluxo, restaurado em checkpoint.
    pub(super) smartquote_context: crate::compiler::lang::quotes::SmartQuoteContext,
    pub(super) attachments:
        Vec<std::sync::Arc<crate::entities::elements::pdf_attach::PdfAttachElem>>,
    pub(super) artifact_marker_id: u64,
    /// **P418** — cache pré-renderizado CSL para citações/bibliografia.
    /// Populado em `layout_with_introspector` antes do layout principal.
    pub(super) bib_render_cache: Option<crate::compiler::layout::bib_csl::BibRenderCache>,
    bib_layout: Option<Arc<bib_csl::plan::BibliographyPlan>>,
    pending_bib_prefix: Option<bibliography::PendingPrefix>,
    paragraph_indent: paragraph_indent::State,
    pending_bib_escape: Option<paragraph_indent::Spec>,
    block_inline_region: Option<block::InlineRegion>,
    /// **P446** — smallcaps activo no corpo de um `#smallcaps[...]`.
    pub(super) smallcaps: bool,
    pub(super) paragraph_planning_suppressed: bool,
    pub(super) bidi_scope_active: bool,
    pub(super) cell_bidi_allowed: bool,
    bidi_cursor: Option<bidi_origin::Cursor>,
    pub(super) text_bidi: Option<crate::entities::bidi_context::BidiSpan>,
    pub(super) text_segment_chain: Option<StyleChain>,
    pub(super) geometric_text_lines: bool,
    pub(super) active_paragraph_dir: Option<crate::entities::dir::Dir>,
    pub(super) pending_line_dir: Option<crate::entities::dir::Dir>,
    pub(super) explicit_line_alignment: Option<HAlign>,
    /// **P472** — chave da última citação renderizada. Usada para ibid:
    /// quando a mesma key é citada consecutivamente, emite "ibid." em vez
    /// do número. `None` antes da primeira citação.
    pub(super) last_cited_key: Option<String>,
    /// **P473** — conjunto de todas as keys citadas antes da posição actual,
    /// excluindo a última (coberta por `last_cited_key`). Usada para op. cit.:
    /// key já citada mas não consecutivamente → "[N] Author, op. cit.".
    pub(super) previously_cited_keys: std::collections::HashSet<String>,
    /// **P505** — `true` se o item anterior numa sequência foi um
    /// `ListItem`/`EnumItem` com `tight: false`. Usado para adicionar
    /// espaçamento de parágrafo *entre* itens soltos sem duplicar o
    /// espaço após o último item. Resetado quando um elemento não-lista
    /// aparece na sequência.
    pub(super) last_was_loose_item: bool,
    /// **P864** — grupo do último item estrutural (`ListItem`,
    /// `EnumItem`, `TermItem`) visto numa sequência. Usado para detectar
    /// quando um `Content::Parbreak` separa itens do mesmo tipo.
    pub(super) last_seen_item_group: Option<ItemGroup>,
    /// **P864** — `true` se um `Content::Parbreak` ocorreu desde o último
    /// item estrutural, sem conteúdo que quebre a consecutividade no meio.
    pub(super) parbreak_since_last_item: bool,
    /// **P537** — modo coluna: quando `true`, `flush_pending_footnote_bodies`
    /// usa `column_origin_x`/`column_width` em vez da página inteira.
    pub(super) column_mode: bool,
    /// **P537** — origem horizontal absoluta da coluna actual na página.
    pub(super) column_origin_x: f64,
    /// **P537** — largura útil da coluna actual.
    pub(super) column_width: f64,
    /// **P538c** — número de colunas da página actual em fluxo contínuo.
    /// `None` significa layout de página normal (uma coluna).
    pub(super) page_columns: Option<usize>,
    /// **P538c** — índice da coluna actual dentro da página (0-based).
    pub(super) current_column: usize,
    /// Contexto exclusivo do modo segmentado, que não activa o mecanismo de
    /// overflow do fluxo contínuo mas ainda precisa identificar a última coluna.
    pub(super) segmented_par_line_column: Option<(usize, usize)>,
    /// P1416 — prefixo transversal já emitido e deslocamentos até os bordos
    /// externos de um contentor `#columns()` segmentado.
    pub(super) segmented_par_line_overlay: Option<SegmentedParLineOverlay>,
    /// P1417 — somente patterns em colunas segmentadas usam esta cauda.
    pub(super) defer_segmented_par_line_patterns: bool,
    pub(super) pending_segmented_par_line_patterns: Vec<SegmentedParLinePatternCandidate>,
    /// P1418 — configuração compartilhada e fronteira da coluna física ativa
    /// no fluxo contínuo de colunas de página.
    pub(super) page_par_line_pattern_style: Option<TextStyle>,
    pub(super) page_par_line_pattern_column_start: usize,
    /// **P538c** — items já fechados de cada coluna da página actual.
    /// Só preenchido quando `page_columns` é `Some`.
    pub(super) column_page_items: Vec<Vec<FrameItem>>,
    /// **P538c** — posições horizontais absolutas (x) de cada coluna na
    /// página actual. Usado para translação e avanço de coluna.
    pub(super) column_x_offsets: Vec<f64>,
    /// **P541** — numeração de página adiada que precisa do total de páginas.
    /// Cada entrada preserva também a configuração marginal da página.
    /// O FrameItem::Text é adicionado no final de `finish()` quando o total
    /// de páginas é conhecido.
    pub(super) pending_page_numbering: Vec<(
        usize,
        usize,
        ecow::EcoString,
        crate::entities::page_running::PageNumberAlign,
        crate::entities::layout_types::PageMargins,
        f64,
        f64,
    )>,
    /// P1414 — ajuste horizontal adiado dos números de linha da página
    /// corrente. A maior largura só é conhecida quando a região fecha.
    /// Entradas: índice do item, largura, margem e alinhamento pedidos.
    pub(super) pending_par_line_numbering: Vec<(usize, usize, f64, HAlign, HAlign)>,
    /// **P896** — centragem de equação de bloco adiada quando `width: auto`
    /// (a largura final da página só é conhecida em `finish()`/`new_page()`,
    /// depois de todo o conteúdo da página já estar posicionado — mesmo
    /// mecanismo de `pending_page_numbering` acima, aplicado por página em
    /// vez de por documento). Cada entrada: `(índice inicial em
    /// current_items, número de items da equação, largura própria da
    /// equação, offset x já aplicado nesses items)`. Limpa a cada fecho de
    /// página (`finish()`/`new_page()`).
    pub(super) pending_equation_centering: Vec<(usize, usize, f64, f64)>,
    /// **P896** — numeração de equação de bloco adiada quando `width: auto`
    /// (mesma razão do campo acima — a posição à direita depende da largura
    /// final da página). Cada entrada: `(y da baseline, texto formatado,
    /// estilo, largura do texto)`. **P987** — acrescentados `largura da
    /// equação` e `offset x aplicado` (fallback, como no pending de
    /// centragem): o número acompanha o fim do conteúdo da SUA equação
    /// (`content_end + gutter`, vanilla `typst-layout/src/math/mod.rs:209-330`),
    /// não a margem direita da largura computada da página.
    pub(super) pending_equation_numbering:
        Vec<(f64, ecow::EcoString, TextStyle, f64, f64, f64, HAlign)>,
    /// **P897** — mesmo mecanismo de `pending_equation_centering`,
    /// generalizado a `Content::Align`/`Content::Place` quando o eixo
    /// horizontal usado por `resolve_alignment` (`available_width()`) está
    /// infinito (`width: auto`, achado alargado de P896). Só cobre o eixo
    /// **horizontal** — o eixo vertical (`pending_align_v_centering`
    /// abaixo) foi corrigido em **P898**. Cada entrada:
    /// `(path, número de items, alinhamento pedido, largura própria do
    /// conteúdo, origin_x usado, x aplicado — fallback já usado nesses
    /// items)`. **P908** — `path: Vec<usize>` (era `start_idx: usize`):
    /// sequência de descida por `FrameItem::Group.items`/`Link.items`
    /// (todos os elementos excepto o último) terminando no índice
    /// efectivo na lista alcançada (último elemento) — ver
    /// `00_nucleo/prompts/core/compiler/layout/mod.md` §P908, `resolve_path_slice`.
    pub(super) pending_align_centering: Vec<PendingAlignXEntry>,
    /// **P898** — simétrico de `pending_align_centering`, eixo vertical.
    /// `VAlign::Horizon`/`VAlign::Bottom` em `layout_align`/`layout_place`
    /// (`placement.rs`) usam `available_height()`/`page_bottom_limit()`
    /// (`remaining_h`/`avail_h`) para resolver a posição Y; sob `height:
    /// auto` esses valores são `f64::INFINITY` até a altura final da
    /// página ser conhecida (só em `finish()`/`new_page()`, depois de todo
    /// o conteúdo da página já posicionado). Fallback aplicado no momento
    /// do posicionamento: usar a altura própria do conteúdo (`content_h`
    /// == `sub_h`) em vez do valor infinito — a fórmula degenera para
    /// "encostado a `origin_y`, sem deslocamento" (mesmo princípio de
    /// degenerescência do eixo horizontal). Corrigido depois por
    /// `apply_pending_align_v_fixups`, reaproveitando `resolve_alignment`
    /// com a altura restante final (`page_height - margin - origin_y`).
    ///
    /// **Correcção pós-implementação de Agente B** — `origin_y` gravado
    /// aqui é o valor **puro** (sem somar `dy`), ao contrário do truque de
    /// linearidade usado em `pending_align_centering` (eixo X, onde
    /// `origin_x + dx` é seguro porque `final_avail_w` é uma constante da
    /// página, independente de `origin_x`). No eixo Y, `final_avail_h`
    /// **depende** de `origin_y` (`page_height - margin - origin_y`,
    /// diferente por entrada, ao contrário do X); gravar `origin_y + dy`
    /// nesse campo faria `dy` contaminar o cálculo de `final_avail_h`,
    /// produzindo um resultado errado sempre que `dy != 0` — confirmado
    /// por um teste exploratório (`Content::Place(bottom)` com `dy=30` sob
    /// `height: auto` devolvia a posição sem `dy` nenhum aplicado, erro de
    /// 30pt). Por isso `dy` é gravado num campo próprio e só somado a
    /// `correct_base_y` depois de `resolve_alignment` já ter usado o
    /// `origin_y` puro. Cada entrada: `(path, número de items, alinhamento
    /// pedido, altura própria do conteúdo, origin_y puro (sem dy), dy —
    /// 0.0 para `layout_align`, que não tem dy —, y aplicado — fallback já
    /// usado nesses items)`. **P908** — `path: Vec<usize>` (era
    /// `start_idx: usize`), mesma semântica de `pending_align_centering`.
    pub(super) pending_align_v_centering: Vec<PendingAlignYEntry>,
    /// P1407 — peças de `repeat` inline aguardando a largura finita que só
    /// fica disponível ao resolver spacings fracionários/fechar a linha.
    pending_repeats: Vec<repeat::PendingRepeat>,
    /// P1408 — largura natural já ocupada por cada fronteira `pending_fr`,
    /// na mesma ordem. `h(Nfr)` registra zero; `box(width: Nfr)` registra a
    /// largura exterior que será substituída pela parcela fracionária.
    pub(super) pending_fr_natural_widths: Vec<f64>,
    /// **P595** — avisos produzidos durante o layout. L1 puro: strings
    /// simples, sem construção de `SourceDiagnostic` nem acesso a Sink.
    /// Exportado no `PagedDocument` e convertido a diagnósticos em L3.
    pub(super) layout_warnings: Vec<String>,
    /// **P644/P645** — erros produzidos durante o layout (ex: conversão de
    /// entrada bibliográfica). Guardados como `SourceDiagnostic` para
    /// preservar `span` e posição no ficheiro.
    pub(super) layout_errors: Vec<SourceDiagnostic>,
    /// P1140.19 — a página corrente nasceu da boundary final de um page-run.
    /// `finish` não a materializa enquanto continuar vazia.
    pub(super) page_run_boundary_empty: bool,
    /// P1292-D v11 — `place.flush()` arma exactamente a próxima unidade de
    /// flow para composição transaccional contra a região reduzida pelos
    /// floats que acabou de realizar. O Cursor consome esta sentinela.
    pub(super) flow_suffix_checkpoint_pending: bool,
    /// Verdade somente durante a primeira composição especulativa da unidade
    /// pós-marker. Permite ao overflow normal comunicar rejeição sem conhecer
    /// o variant de `Content` que está em curso.
    pub(super) flow_suffix_transaction_active: bool,
    /// Resultado causal da tentativa especulativa. O Cursor restaura o
    /// checkpoint e faz replay apenas quando o overflow ocorreu contra a
    /// geometria reduzida; pagebreaks explícitos não activam este estado.
    pub(super) flow_suffix_transaction_rejected: bool,
    /// Snapshot persistente enquanto a decisão de fitting do sufixo ainda
    /// não é observável. Não é exposto fora do owner Layouter.
    pub(super) flow_suffix_checkpoint: Option<FlowSuffixCheckpoint>,
    /// Ocorrências já compostas depois da fronteira. São clones lógicos do
    /// Content imutável (Arc-backed) e permitem replay sem reler o futuro.
    pub(super) flow_suffix_replay: Vec<Content>,
    /// Profundidade da ocorrência top-level actualmente especulada. Chamadas
    /// descendentes pertencem à mesma ocorrência e não entram duas vezes no
    /// replay-log.
    pub(super) flow_suffix_dispatch_depth: usize,
}

/// **P908** — entrada de `pending_align_centering` (eixo X):
/// `(path, count, align, content_w, origin_x, applied_x)`. `path`
/// substitui o antigo `start_idx: usize` — ver
/// `00_nucleo/prompts/core/compiler/layout/mod.md` §P908.
pub(super) type PendingAlignXEntry =
    (Vec<usize>, usize, crate::entities::layout_types::Align2D, f64, f64, f64);

/// **P908** — entrada de `pending_align_v_centering` (eixo Y):
/// `(path, count, align, content_h, origin_y, dy, applied_y)`. `path`
/// substitui o antigo `start_idx: usize` — ver
/// `00_nucleo/prompts/core/compiler/layout/mod.md` §P908.
pub(super) type PendingAlignYEntry =
    (Vec<usize>, usize, crate::entities::layout_types::Align2D, f64, f64, f64, f64);

/// **P286** — Segmento de linha visual capturado por `flush_line`
/// quando o `decoration_lines_collector` está activo. Cada segmento
/// corresponde a uma linha visual coberta por uma decoração textual
/// que faz wrap (Underline/Strike/Overline). Consumer P284 itera os
/// segments + acrescenta o segmento final não-flushed (porque o body
/// pode terminar antes do flush).
#[derive(Debug, Clone, Copy)]
pub(super) struct DecoSegment {
    pub start_x: Pt,
    pub end_x: Pt,
    pub baseline_y: Pt,
}

/// **P245 (M9d / M7+4)** — entry do buffer `floats_pending` no
/// Layouter. Captura body já-layouted + alignment para emit deferred
/// no flush da página actual.
#[derive(Debug, Clone)]
pub(super) struct DeferredFloat {
    /// Alignment per eixo. `alignment.y == Top` → topo da página;
    /// `Bottom`/`Horizon` → fundo (paridade vanilla default bottom).
    /// `alignment.x` aplica-se à largura útil da página para
    /// posicionamento horizontal.
    pub alignment: crate::entities::layout_types::Align2D,
    /// Items do body já layouted via `layout_sub_frame`;
    /// posições locais (origem 0,0); flush translada para destino
    /// final.
    pub body_items: Vec<crate::entities::layout_types::FrameItem>,
    /// Altura ocupada pelo body (sub_h retornado por
    /// `layout_sub_frame`).
    pub body_height: f64,
    /// Largura ocupada pelo body (`content_w` via
    /// `measure_content`).
    pub body_width: f64,
    /// Clearance vertical entre flow regular e área float (resolvido
    /// a Pt; 0.0 se `clearance: None`).
    pub clearance: f64,
    /// **P772x** — segmentos de decoração (Underline/Strike/Overline) do
    /// body, em coordenadas locais (mesmo referencial de `body_items`).
    /// Best-effort: só produzem `FrameItem::Line` em `emit_deferred_float`
    /// se `decoration_lines_collector` ainda estiver activo nesse ponto —
    /// verdade quando o flush ocorre antes do consumer `Underline`/`Strike`/
    /// `Overline` (`decorations.rs`) restaurar o collector (ex.: quebra de
    /// página a meio do body decorado); **não** garantido para floats que só
    /// flusham num ponto muito posterior (ex.: `finish()` no fim do
    /// documento, já fora do `layout_content(body)` do consumer) — nesse
    /// caso o segmento é descartado silenciosamente, mesma disciplina de
    /// fallback do resto do mecanismo P284/P286. Limitação registada, não
    /// silenciosa — ver `lab/results/construction/diagnostics/paridade-producao-p772x.md`.
    pub deco_segments: Vec<DecoSegment>,
    /// **P908** — entradas órfãs de `pending_align_centering` devolvidas
    /// pelo `layout_sub_frame` do body deste float (`Content::Align`/
    /// `Content::Place` aninhado dentro do body, sob `width: auto`), já
    /// relativas a `body_items`. `emit_deferred_float` rebaseia-as com a
    /// MESMA translação aplicada aos `body_items` reais e empurra-as para
    /// `pending_align_centering` — como `flush_pending_floats` corre
    /// sempre antes de `apply_pending_align_fixups` (mesma `new_page()`/
    /// `finish()`), a entrada resolve-se correctamente sem sequenciamento
    /// novo. Ver `00_nucleo/prompts/core/compiler/layout/mod.md` §P908.
    pub orphaned_align_x: Vec<PendingAlignXEntry>,
    /// **P908** — simétrico de `orphaned_align_x`, eixo vertical.
    pub orphaned_align_y: Vec<PendingAlignYEntry>,
}

/// **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita ADR-0082
/// PROPOSTO N=2)** — entry do buffer `pending_cell_tails` no
/// Layouter. Captura items de cell que ultrapassaram o limite
/// vertical (row break real cell-level) + bounds para re-emit fill/
/// stroke na nova página.
///
/// Limitações conscientes (per ADR-0054 graded):
/// - Items são rebased (`pos.y -= threshold` no slice).
/// - Fill/stroke re-emit na nova página com bounds = tail extent
///   (não bounds originais do cell — visualmente "duas células
///   separadas"; paridade vanilla "split block draws two borders").
/// - Recursive overflow (tail que ela própria overflow na nova
///   página) limita 3 iterações (mitigação loop infinito).
#[derive(Debug, Clone)]
pub(super) struct DeferredCellTail {
    /// Items do cell tail (`pos.y` já rebased pelo slice).
    pub items: Vec<crate::entities::layout_types::FrameItem>,
    /// `cell_x` (origem horizontal preservada column-aligned).
    pub origin_x: f64,
    /// `body_w` (largura útil da cell preservada).
    pub width: f64,
    /// Fill efectivo do cell (paridade Z-order step 1; re-emit
    /// atrás dos items na nova página).
    pub fill: Option<crate::entities::layout_types::Color>,
    /// Stroke efectivo do cell (paridade Z-order step 3; re-emit
    /// à frente dos items na nova página).
    pub stroke: Option<crate::entities::geometry::Stroke>,
    /// **P251** — contador de forwardings consecutivos (cada nova
    /// página que ainda gera tail). Incrementa em
    /// `flush_pending_cell_tails`; tail descartado quando atinge 3
    /// (paridade vanilla heurística max-iter).
    pub forwarded_count: u32,
    /// **P908** — entradas órfãs de `pending_align_centering` cujo
    /// intervalo `[path[0], path[0]+count)` caiu inteiramente no tail
    /// (célula com `Content::Align`/`Content::Place` aninhado sob
    /// `width: auto`, cujos items foram partidos por
    /// `slice_frame_items_at_height`), já relativas a `items` (rebase
    /// `path[0]` pela mesma partição head/tail; `origin`/`applied`
    /// deslocados por `-threshold`, mesmo `rebase_item_y` aplicado aos
    /// items reais). `flush_pending_cell_tails` rebaseia-as outra vez por
    /// `+cursor_top` no flush. Entradas cujo intervalo atravessa a
    /// fronteira head/tail são descartadas (degradação graciosa — mesmo
    /// princípio de `resolve_path_slice`, caso raro: exige overflow de
    /// linha `Auto`/`Fraction` E um Align/Place aninhado cujos items
    /// fiquem divididos pelo próprio corte). Ver
    /// `00_nucleo/prompts/core/compiler/layout/mod.md` §P908.
    pub orphaned_align_x: Vec<PendingAlignXEntry>,
    /// **P908** — simétrico de `orphaned_align_x`, eixo vertical.
    pub orphaned_align_y: Vec<PendingAlignYEntry>,
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// **P204C (M8)** — `introspector` parameter agora obrigatório
    /// (migrado de `TagIntrospector` field assignment para
    /// `Tracked<'a, dyn Introspector + 'a>` aceite no construtor).
    /// Caller constrói `TagIntrospector` (provavelmente via
    /// `introspect_with_introspector`) + `.track()` e passa o handle.
    pub fn new(
        metrics: M,
        sizer: S,
        _font_size: f64,
        introspector: comemo::Tracked<
            'a,
            dyn crate::entities::introspector::Introspector + 'a,
        >,
    ) -> Self {
        let cfg = PageConfig::default();
        Self {
            metrics,
            sizer,

            // **F-5b fatia 2 (P373)**: o `self.style` inicial deriva da chain
            // (ADR-0039: a chain é a fonte da verdade do estilo de texto), não de
            // `font_size` (geometria). Antes: `regular(size=12)` → `style.size=12`
            // (usado por `space_width` quando não-embrulhado), mas o texto resolve
            // a 11 (default da chain) e qualquer descida de `Content::Styled`
            // recompõe `style` da chain (11). A inconsistência 12-vs-11 ficava só no
            // espaço-líder de docs non-embrulhados. Derivar da chain unifica em 11.
            style: TextStyle::from(&StyleChain::default_chain()),
            document_language: None,
            chain: StyleChain::default_chain(),
            text_tracking_state: text_tracking::State::default(),
            page_config: cfg.clone(),
            pages: Vec::new(),
            // P216A: 5 fields escalares + 2 dimensões agregados em
            // Region. Cursor + line_start_x inicializados a margin;
            // cursor_y inicializado a margin (sem cap-height — o offset
            // da baseline é adiado por P751 até o primeiro conteúdo real,
            // usando o estilo activo nesse momento).
            // P216B: agregação adicional em Regions wrapper (single-region
            // por anti-inflação 11ª; multi-region em P219).
            regions: {
                let mut rs =
                    crate::entities::region::Regions::single(cfg.width, cfg.height);
                rs.current.cursor_x = Pt(cfg.margin.left);
                rs.current.cursor_y = Pt(cfg.margin.top);
                rs.current.line_start_x = Pt(cfg.margin.left);
                rs
            },
            justify_opportunities: Vec::new(),
            // P751 — baseline inicial ainda não fixada; será ajustada na
            // primeira emissão de conteúdo real com o estilo activo.
            initial_baseline_pending: true,
            // P190I: counter field eliminated.
            // P204C: field passa a ser Tracked, recebido por parameter.
            introspector,
            figure_progress: std::collections::HashMap::new(),
            is_height_unconstrained: false,
            is_sub_frame: false,
            relative_region_size: None,
            sub_frame_origin_y: 0.0,
            flow_placement_end: None,
            flow_frame_extent: (0.0, 0.0),
            last_sub_frame_flow_width: 0.0,
            block_place_region: None,
            block_auto_width: false,
            block_place_decorations: Vec::new(),
            // P246 — cell_available_h + cell_origin_w migrados a
            // regions.cell (entity-side). cell_origin_x/y preservados
            // como Layouter fields legacy.
            cell_origin_x: None,
            cell_origin_y: None,
            // P273.5 — fallback None; callsite L3 usa page_bbox.
            parent_bbox: None,
            cell_align: None, // P232
            capture_html_context_markers: false,
            position_captures: Vec::new(),
            link_capture: link_capture::Capture::default(),
            locator: Locator::new(),
            locations_reserved: false,
            current_location: None,
            math_callback_pass: None,
            par_line_callback_pass: None,
            runtime:
                crate::entities::layouter_runtime_state::LayouterRuntimeState::default(),
            // P245 (M9d / M7+4) — buffer floats + reservas inicializados vazios.
            floats_pending: Vec::new(),
            cursor_y_top_reserve: 0.0,
            cursor_y_bottom_reserve: 0.0,
            enum_counter: None,
            par_line_counter: 0,
            par_line_page: None,
            // P250 — spacing collapse state inicializado limpo.
            prev_block_below_pending: 0.0,
            fractional_block_height: None,
            fractional_preflight_active: false,
            fractional_flow_tail_pending: false,
            block_chain_active: false,
            prev_margin_is_parbreak: false,
            // P813 — sem flush prévio; a primeira equação de bloco usa o
            // caminho `initial_baseline_pending` (topo da página).
            prev_line_baseline: 0.0,
            terminal_paragraph_line: false,
            empty_paragraph_weak_spacing: 0.0,
            last_flush_advance: 0.0,
            prev_block_equation_descent: 0.0,
            last_equation_descent: 0.0,
            last_block_descent_y: None,
            // P1120 — linha ainda vazia: sem contribuições inline.
            line_inline_ascent: 0.0,
            line_inline_descent: 0.0,
            line_assumed_ascent: None,
            wrapped_inline_prefix_line_armed: false,
            last_sub_frame_bottom: None,
            last_sub_frame_width: 0.0,
            last_sub_frame_inline_extents: (0.0, 0.0),

            // P251 — buffer cell tails inicializado vazio.
            pending_cell_tails: Vec::new(),
            // P304 — buffer footnote bodies inicializado vazio.
            pending_footnote_bodies: Vec::new(),
            // P537 — modo coluna inactivo por default.
            column_mode: false,
            column_origin_x: 0.0,
            column_width: 0.0,
            // P538c — fluxo contínuo multi-coluna inactivo por default.
            page_columns: None,
            current_column: 0,
            segmented_par_line_column: None,
            segmented_par_line_overlay: None,
            defer_segmented_par_line_patterns: false,
            pending_segmented_par_line_patterns: Vec::new(),
            page_par_line_pattern_style: None,
            page_par_line_pattern_column_start: 0,
            column_page_items: Vec::new(),
            column_x_offsets: Vec::new(),
            // P286 — collector inactivo por default; consumer P284 activa
            // localmente antes de layout_content do body decorado.
            decoration_lines_collector: None,
            // P287 — estado smartquote per-document (true = próximo é open).
            smartquote_context: Default::default(),
            attachments: Vec::new(),
            artifact_marker_id: 0,
            // P446 — smallcaps activo no corpo de um `#smallcaps[...]`.
            smallcaps: false,
            paragraph_planning_suppressed: false,
            bidi_scope_active: false,
            cell_bidi_allowed: false,
            bidi_cursor: None,
            text_bidi: None,
            text_segment_chain: None,
            geometric_text_lines: false,
            active_paragraph_dir: None,
            pending_line_dir: None,
            explicit_line_alignment: None,
            // P418 — cache pré-renderizado de citações/bibliografia CSL.
            bib_render_cache: None,
            bib_layout: None,
            pending_bib_prefix: None,
            paragraph_indent: paragraph_indent::State::default(),
            pending_bib_escape: None,
            block_inline_region: None,
            // P472 — última key citada para ibid.
            last_cited_key: None,
            // P473 — keys citadas anteriormente para op. cit.
            previously_cited_keys: std::collections::HashSet::new(),
            // P505 — estado de espaçamento entre itens de lista soltos.
            last_was_loose_item: false,
            // P864 — agrupamento por Parbreak em sequências.
            last_seen_item_group: None,
            parbreak_since_last_item: false,
            // P541 — numeração adiada para patterns compostos (ex: "1 / 1").
            pending_page_numbering: Vec::new(),
            pending_par_line_numbering: Vec::new(),
            pending_equation_centering: Vec::new(),
            pending_equation_numbering: Vec::new(),
            pending_align_centering: Vec::new(),
            pending_align_v_centering: Vec::new(),
            pending_repeats: Vec::new(),
            pending_fr_natural_widths: Vec::new(),
            // **P595** — avisos de layout inicializados vazios.
            layout_warnings: Vec::new(),
            // **P644** — erros de layout inicializados vazios.
            layout_errors: Vec::new(),
            page_run_boundary_empty: false,
            flow_suffix_checkpoint_pending: false,
            flow_suffix_transaction_active: false,
            flow_suffix_transaction_rejected: false,
            flow_suffix_checkpoint: None,
            flow_suffix_replay: Vec::new(),
            flow_suffix_dispatch_depth: 0,
        }
    }

    /// Largura disponível para conteúdo (exclui margens dos dois lados).
    /// **P867** — quando `width: auto`, o espaço é ilimitado.
    pub(super) fn available_width(&self) -> f64 {
        if self.is_sub_frame {
            return (self.regions.current.width
                - self.regions.current.line_start_x.0
                - self.page_config.margin.right)
                .max(0.0);
        }
        if self.page_config.width.is_infinite() {
            f64::INFINITY
        } else {
            f64::max(
                0.0,
                self.regions.current.width - self.page_config.margin.horizontal(),
            )
        }
    }

    /// Altura disponível para conteúdo (exclui margens topo e base).
    #[allow(dead_code)]
    pub(super) fn available_height(&self) -> f64 {
        if self.is_sub_frame {
            return self.regions.current.height.max(0.0);
        }
        if self.page_config.height.is_infinite() {
            f64::INFINITY
        } else {
            f64::max(
                0.0,
                self.regions.current.height - self.page_config.margin.vertical(),
            )
        }
    }

    /// Limite inferior da página em pontos (`height - margin`). Passo 82.
    ///
    /// Usar este método em vez de `page_config.height - page_config.margin`
    /// inline — evita confundir com `available_height()` (que subtrai 2×margin).
    /// **P867** — quando `height: auto`, não há limite inferior.
    pub(super) fn page_bottom_limit(&self) -> f64 {
        if self.is_sub_frame {
            return self.regions.current.height;
        }
        if self.page_config.height.is_infinite() {
            f64::INFINITY
        } else {
            self.regions.current.height - self.page_config.margin.bottom
        }
    }

    /// Fonte de verdade estrutural: a página actual não tem nenhum item visual.
    ///
    /// Verifica tanto `current_items` (linhas já fechadas) como `current_line`
    /// (items ainda pendentes de flush) — uma linha não fechada ainda constitui
    /// conteúdo visível na página.
    fn current_page_is_empty(&self) -> bool {
        !html_frame::items_have_material(&self.regions.current.current_items)
            && !html_frame::items_have_material(&self.regions.current.current_line)
    }

    /// Instala uma configuração numa página vazia e sincroniza a região.
    /// P1140.19: usado pelo page-run sem aplicar a semântica progressiva de
    /// `Content::SetPage` nem provocar uma quebra adicional.
    pub(super) fn install_page_config(&mut self, config: PageConfig) {
        self.page_config = config;
        self.regions.current.width = self.page_config.width;
        self.regions.current.height = self.page_config.height;
        self.regions.current.cursor_x = Pt(self.page_config.margin.left);
        self.regions.current.line_start_x = Pt(self.page_config.margin.left);
        self.regions.current.cursor_y = Pt(self.page_config.margin.top);
        self.initial_baseline_pending = true;
        self.flow_placement_end = None;
        self.block_place_region = None;
        self.block_auto_width = false;
        self.block_place_decorations.clear();
    }

    /// **P185C (mecanismo M3 da ADR-0068)** — avança `self.locator` e
    /// actualiza `self.current_location` se `content` for locatable.
    /// Mirror exacto do gating do walk de introspect
    /// (`introspect.rs:329` — `do_extract_payload(content).is_some()`):
    /// invariante `is_locatable ↔ extract_payload.is_some()` (provada
    /// em `locatable.rs:11`) garante sincronização-por-construção das
    /// duas sequências de `Location`s.
    ///
    /// **P204D (M8)** — emit Position single-pass per ADR-0073.
    /// Para cada locatable, popular `runtime.positions` com
    /// `Position { page: pages.len() + 1, point: (cursor_x,
    /// cursor_y) }`. Single canonical site — mirror do gating
    /// que set `current_location`. Idempotência via `insert`.
    fn advance_locator_if_locatable(&mut self, content: &Content) {
        if is_locatable(content) {
            let loc = self.locator.next();
            self.current_location = Some(loc);
            // P204D: emit Position concrete single-pass.
            let page = std::num::NonZeroUsize::new(self.pages.len() + 1)
                .expect("pages.len() + 1 >= 1");
            let point = crate::entities::layout_types::Point {
                x: self.regions.current.cursor_x,
                y: self.regions.current.cursor_y,
            };
            self.record_location_position(
                loc,
                crate::entities::position::Position { page, point },
                false,
            );
        }
    }

    pub fn layout_content(&mut self, content: &Content) {
        let content = content.source_body();
        if self.flow_suffix_checkpoint_pending {
            self.flow_suffix_checkpoint_pending = false;
            self.flow_suffix_transaction_active = true;
            self.flow_suffix_transaction_rejected = false;
            self.flow_suffix_checkpoint = Some(self.capture_flow_suffix_checkpoint());
            self.flow_suffix_replay.clear();
        }

        if self.flow_suffix_transaction_active && self.flow_suffix_dispatch_depth == 0 {
            self.layout_flow_suffix_transaction(content);
        } else {
            self.layout_content_once(content);
        }
    }

    /// Executa uma ocorrência sem reabrir a sentinela transaccional. O
    /// Cursor usa esta entrada no primeiro ensaio e no replay da mesma
    /// ocorrência; descendants continuam a chamar `layout_content` e, como a
    /// sentinela já foi consumida, integram a mesma unidade atómica.
    pub(super) fn layout_content_once(&mut self, content: &Content) {
        if !self.locations_reserved {
            crate::compiler::introspect::walk::reserve_locations(
                content,
                &mut self.locator,
            );
            self.locations_reserved = true;
        }
        match content {
            Content::Source(e) => {
                self.layout_content_once(&e.body);
                return;
            }
            Content::Located(e) => {
                self.locator.observe(e.location);
                self.current_location = Some(e.location);
                let page = std::num::NonZeroUsize::new(self.pages.len() + 1).unwrap();
                let point = crate::entities::layout_types::Point {
                    x: self.regions.current.cursor_x,
                    y: self.regions.current.cursor_y,
                };
                self.record_location_position(
                    e.location,
                    crate::entities::position::Position { page, point },
                    false,
                );
                self.layout_content_body(&e.body);
                return;
            }
            Content::Realized(e) => {
                realized::layout(self, &e.original, &e.visual);
                return;
            }
            Content::ShowRecipe(e) => {
                self.layout_content(&e.body);
                return;
            }
            _ => {}
        }
        // P185C: gating Locator atómico no topo, antes do match.
        // Avança em sincronia com walk de introspect; current_location
        // fica disponível para consumers location-aware (P187/P188).
        self.advance_locator_if_locatable(content);
        self.layout_content_body(content);
    }

    fn layout_content_body(&mut self, content: &Content) {
        paragraph_indent::before_content(self, content);
        // Observe the logical input once; leaf arms remain static adapters.
        match content {
            Content::Text(text) => self.smartquote_context.observe_text(text),
            Content::Space | Content::SpaceFromNewline => {
                self.smartquote_context.observe_text(" ")
            }
            _ => {}
        }
        match content {
            Content::Source(_)
            | Content::Located(_)
            | Content::Realized(_)
            | Content::ShowRecipe(_) => self.layout_content_once(content),
            Content::Empty => {}

            // A unidade do parágrafo fecha suas fronteiras sob o estilo ativo.
            Content::Par { body } => paragraph::layout(self, body),

            Content::HtmlFrame(e) => html_frame::layout(self, e),

            // Lote F-3 (DEBT C2 fechado): layout **default** do elemento de
            // utilizador (fronteira E1). Renderiza o campo `body` (se o elemento
            // o expõe via `get_field`) ou, em fallback, o `plain_text`. Um
            // `#show <kind>: …` na linguagem (recipe → conteúdo nativo) transforma
            // o nó ANTES de chegar aqui (caminho eager existente; o casamento por
            // kind dinâmico chega no incremento seguinte do F-3). A divergência
            // S2–S6 (realização multi-passe/guards do vanilla) fica registada
            // (L0 §3b.6) — o eager do cristalino com `active_guards`+depth basta
            // até `#show` na linguagem exigir paridade medida (gatilho registado).
            // Atomizado (ADR-0109, P425) → layout/dynamic.rs.
            Content::Dynamic(e) => dynamic::layout(self, e),

            // P169 (M9): Metadata é zero-size em layout — sem caixa,
            // sem texto, sem efeito visual. O `value` permanece
            // disponível via `Introspector::query_metadata` para
            // querying do utilizador.
            Content::Metadata(_) => {}

            // P171 (M9): State e StateUpdate são zero-size em layout.
            // Disponíveis via `Introspector::state_value` /
            // `state_final_value`.
            Content::State(_) => {}
            Content::StateUpdate(_) => {}

            // P397 — Document/Asset são metadata/resources; não emitem frames.
            Content::Document { .. } => {}
            Content::Asset { .. } => {}

            // P240 (M9d/M7+1): StateDisplay consome Content pre-rendered
            // pelo `apply_state_displays` pós-fixpoint via
            // `Introspector::state_display_value(key, loc)`. Layouter
            // permanece puro (sem Engine+ctx em signature) — paridade
            // arquitectural estrita preservada (Opção γ P239 audit).
            Content::StateDisplay(e) => state_display::layout(self, e),

            // P241 (M9d/M7+2): CounterDisplayCallback consome Content
            // pre-rendered pelo `apply_counter_displays` pós-fixpoint via
            // `Introspector::counter_display_value(key, loc)`. Layouter
            // permanece puro (paridade absoluta P240). Distinto de
            // `Content::CounterDisplay { kind }` legacy single-pass.
            Content::CounterDisplayCallback(e) => {
                counter_display_callback::layout(self, e)
            }

            // Atomizado (ADR-0109, P381) → layout/text.rs (folha de render).
            Content::Text(text) => text::layout(self, text),

            Content::Space => space::layout(
                self,
                crate::entities::layout_types::TextSeparatorOrigin::Markup,
            ),
            Content::SpaceFromNewline => space::layout(
                self,
                crate::entities::layout_types::TextSeparatorOrigin::Newline,
            ),

            // **P1057/P1061** — paridade vanilla `par.spacing` e margin collapsing parágrafo↔bloco:
            // O flush_line drena a linha e avança (top + bottom + leading); adicionamos
            // o delta de (spacing - leading) para atingir par.spacing total.
            // P1061: Se houver um bloco anterior pendente (prev_block_below_pending), colapsa
            // contra ele; e regista spacing_pt como prev_block_below_pending para que um
            // bloco subsequente possa colapsar contra este parágrafo.
            Content::Parbreak => parbreak::layout(self),

            // Atomizado (ADR-0109, P425) → layout/sequence.rs.
            Content::Sequence(parts) => sequence::layout(self, parts),

            // Passo 101: `Content::Strong` e `Content::Emph` removidos do enum.
            // `*bold*` e `_italic_` produzem `Content::Styled([Bold(true)/Italic(true)], body)`
            // no `eval_markup` (ou via `Content::strong/emph` construtores
            // redefinidos). O arm `Content::Styled` (introduzido no Passo 100)
            // cobre ambos os casos via push/pop na `chain`.

            // Atomizado (ADR-0109, P377) → layout/heading.rs.
            Content::Heading(h) => heading::layout(self, h),
            Content::Title(t) => title::layout(self, t),

            // Lote F-2 S5 (P335): marcadores Set*Numbering removidos — numeração assada nos elementos.
            Content::CounterUpdate(_) => {
                // P190I (M6 fechado): mutação Layouter do counter
                // ELIMINADA — `self.counter` field eliminado. Caminho
                // Introspector activo via populate_intr arm
                // CounterUpdate (P198C); intr.counters é única fonte
                // da verdade. Layouter no-op.
            }

            Content::CounterDisplay(e) => counter_display::layout(self, e),

            // Atomizado (ADR-0109, P381) → layout/raw.rs.
            Content::Raw(e) => raw::layout(self, e),

            // Modelo D (Lote 3 P318): destructure de Arc<Elem> — mesma lógica.
            // Atomizado (ADR-0109, P380) → layout/list_item.rs, enum_item.rs.
            Content::ListItem(e) => list_item::layout(self, e),
            Content::EnumItem(e) => enum_item::layout(self, e),

            // Atomizado (ADR-0109, P381) → layout/link.rs.
            Content::Link(e) => link::layout(self, e),

            // ── Matemática (Passo 37) — delegação ao MathLayouter ───────────
            // Atomizado (ADR-0109, P382) → equation.rs (cola math; o layout real
            // dos nós vive em rules/math/layout/). Convenção impl Layouter.
            Content::Equation(e) => self.layout_equation_arm(e),

            Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_) => self.layout_math_fallback(content),

            // Marcadores estruturais de equações — ignorados fora de contexto matemático.
            Content::MathAlignPoint(_) => {}

            // Quebra de linha explícita (`\\` e pré-renderizações como bibliografia CSL).
            Content::Linebreak(e) => linebreak::layout(self, e),

            // P460/P464 — Label: destino nomeado. Layout transparente do body
            // com registo de página + posição para /Dests no PDF.
            Content::Label(e) => {
                references::layout_label(self, &e.body, Label(e.name.to_string()));
            }

            Content::Ref(e) => {
                references::layout_ref(self, e);
            }

            // Passo 62/75 — Figure: delegado a figure.rs com kind/numbering (DEBT-14/15).
            // Passo 158C: kind é Option<String>; resolver default "image"
            // em uso (paridade introspect.rs walk arm).
            // Atomizado (ADR-0109, P378) → layout/figure.rs.
            Content::Figure(e) => figure::layout(self, e),

            // P1473 — figure.caption(...), escopo reduzido: sem síntese nem
            // layout consumidor nesta fatia (owner: `compiler/stdlib/
            // figure_image.md` §P1473). Zero-size fora do pipeline de
            // `figure()`, paridade com Metadata/State (dado sem render
            // próprio até integração futura).
            Content::FigureCaption(_) => {}

            // Passo 61/P457 — TOC: delegado a outline.rs.
            Content::Outline(e) => outline::layout_outline(self, e),

            Content::OutlineEntry(e) => outline_entry::layout(self, e),

            // Atomizado (ADR-0109, P377) → layout/shape.rs.
            Content::Shape(e) => shape::layout(self, e),

            // Passo 513 — Atomizado (ADR-0109) → layout/curve.rs.
            Content::Curve(e) => curve::layout(self, e),

            // Atomizado (ADR-0109, P377) → layout/transform.rs.
            Content::Transform(e) => transform::layout(self, e),

            // P224+P227+P228 — Grid refino +7 fields. gutter/align/inset/header/footer/stroke/fill
            // são consumidos por layout_grid (signature expandida).
            // Atomizado (ADR-0109, P380) → grid.rs (cluster layout_grid) + table.rs
            // e os arquivos próprios das cells/headers/footers.
            Content::Grid(e) => grid::layout(self, e),
            Content::GridHeader(e) => grid_header::layout(self, e),
            Content::GridFooter(e) => grid_footer::layout(self, e),
            Content::GridCell(e) => grid_cell::layout(self, e),
            // Passo 512 — linhas em grid/table são renderizadas pelo
            // layout_grid interno, não como content independente.
            Content::GridHLine(_) | Content::GridVLine(_) => {}
            Content::Table(e) => table::layout(self, e),
            Content::TableCell(e) => table_cell::layout(self, e),
            Content::TableHeader(e) => table_header::layout(self, e),
            Content::TableFooter(e) => table_footer::layout(self, e),
            Content::TableHLine(_) | Content::TableVLine(_) => {}

            // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──
            // Render placeholder per ADR-0033 + ADR-0054 graded:
            // Bibliography renderiza title (se Some) + lista de entries
            // formatadas como `"[{key}] {author}. {title} ({year})."`;
            // Cite renderiza placeholder `"[{key}]"` + supplement.
            // Refinos futuros (CSL styles, form variants, hayagriva)
            // NÃO reservados per política P158.
            // Atomizado (ADR-0109, P381) → layout/bibliography.rs.
            Content::Bibliography(b) => bibliography::layout(self, b),
            // P295 — Footnote Fase 1 marker emitido inline; P304
            // (P295.1) — body diferido para o rodapé via buffer
            // `pending_footnote_bodies` (subpadrão DeferredX N=3
            // cumulativo: P245 floats + P251 cell tails + P304
            // footnotes). Flush em `new_page()` (antes de saving
            // Page) + `finish()` (última página) emite os bodies
            // no rodapé com posicionamento Y absoluto bottom-up.
            // P295.2 (overflow multi-página) permanece scope-out.
            // Atomizado (ADR-0109, P381) → layout/footnote.rs.
            Content::Footnote(e) => footnote::layout(self, e),

            // P1473 — footnote.entry(...), escopo reduzido: sem realização
            // nem layout consumidor nesta fatia (owner: `compiler/stdlib/
            // structural/footnote.md` §P1473). `compiler/layout/footnote.rs`
            // não muda — continua a gerar a lista de notas como hoje, sem
            // consultar este constructor. Zero-size fora desse pipeline.
            Content::FootnoteEntry(_) => {}

            // Atomizado (ADR-0109, P381) → layout/cite.rs.
            Content::Cite(e) => cite::layout(self, e),

            // Atomizado (ADR-0109, P425) → layout/set_page.rs.
            Content::SetPage(delta) => set_page::layout(self, delta),

            Content::PageRun(e) => page_run::layout(self, e),

            // Atomizado (ADR-0109, P378) → layout/image.rs.
            Content::Image(e) => image::layout(self, e),

            Content::Align(e) => align::layout(self, e),

            // P223 — Place refino: float + clearance armazenados.
            // P245 (M9d / M7+4) — semantic real activa para `float: true`:
            // body layouted em sub-frame, capturado em buffer
            // `floats_pending`, emitido no flush da página (new_page/
            // finish). `float: false` preserva comportamento P84.5+P84.6
            // literal (body in-place via cursor).
            // Atomizado (ADR-0109, P378) → layout/place.rs.
            Content::Place(e) => place::layout(self, e),
            Content::Flush(e) => flush::layout(self, e),

            // Passo 100 (ADR-0039): `Content::Styled` activa push/pop na
            // `chain` interna. A vista achatada `self.style` é
            // re-sincronizada a partir da cadeia após push e depois do pop.
            // Save/restore via variável local — o `self.chain` do chamador
            // permanece íntegro se `layout_content` retornar via early
            // return (padrão Passo 98).
            Content::Styled(styled) => {
                style_scope::layout_styled(self, &styled.body, &styled.styles)
            }

            // F-5b fatia 1 (P371): strong/emph são variantes próprias (morfologia,
            // 0107). O render (bold/italic) **replica** o arm `Styled` acima — push
            // de `Bold(true)`/`Italic(true)` na chain, layout do body, restore →
            // output byte-idêntico ao `Styled[Bold/Italic]` de antes (paridade visual).
            Content::Strong(e) => style_scope::layout_strong(self, &e.body),
            Content::Emph(e) => style_scope::layout_emph(self, &e.body),

            // P446: smallcaps — fallback por scaling (0.8×) de maiúsculas.
            // Letras minúsculas são convertidas para maiúsculas e renderizadas
            // a 80% do tamanho actual; maiúsculas e não-letras mantêm o
            // tamanho. O flag `self.smallcaps` sinaliza ao layout de `Text`
            // para fazer a segmentação por runs.
            Content::SmallCaps { body } => style_scope::layout_smallcaps(self, body),

            // ── Passo 154B (ADR-0060 Fase 1) — terms + divider ──────────────
            // Atomizado (ADR-0109, P381) → layout/divider.rs.
            Content::Divider(e) => divider::layout(self, e),

            // Atomizado (ADR-0109, P380) → layout/terms.rs, term_item.rs.
            Content::Terms(e) => terms::layout(self, e),
            Content::TermItem(e) => term_item::layout(self, e),

            // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──
            // P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
            // — promoção real `Pad.right`: `regions.current.width` save/restore
            // permite width-aware wrap em `layout_word` consumir largura útil
            // reduzida pelo `right` durante body layout.
            // Atomizado (ADR-0109, P376) → layout/pad.rs.
            Content::Pad(e) => pad::layout(self, e),

            // Atomizado (ADR-0109, P381) → layout/hide.rs.
            Content::Hide(e) => hide::layout(self, e),

            // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ──
            // `weak` armazenado mas comportamento de collapse adiado
            // (perfil ADR-0054 graded). Refino futuro se necessário.
            // Atomizado (ADR-0109, P380) → layout/h_space.rs, v_space.rs.
            Content::HSpace(e) => h_space::layout(self, e),
            Content::VSpace(e) => v_space::layout(self, e),

            // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──
            // **Último sub-passo Fase 2 (atinge target 72% Layout)**.
            // Container compositivo: itera children + spacing + dir.
            // Structural (força flush_line antes; cada child em "linha"
            // própria para TTB/BTT; inline para LTR/RTL).
            //
            // Implementação simples per ADR-0054 graded: BTT/RTL
            // implementadas como reverse iteration (children[len-1..0])
            // — geometricamente similar a TTB/LTR mas com order
            // visualmente invertido. Refino futuro pode aplicar
            // posicionamento absoluto reverso real (sob forma de
            // FrameItem positioning).
            // Atomizado (ADR-0109, P376) → layout/stack.rs.
            Content::Stack(e) => stack::layout(self, e),

            // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ──
            // Container INLINE: NÃO força flush_line. Aplica inset.left
            // + body + inset.right como avanço de cursor.x na linha
            // actual. width/height/baseline armazenados mas semantic real
            // adiada per ADR-0054 graded (consistente com Block):
            //   - `width`: limitar largura útil exigiria refactor
            //     multi-region (DEBT-56).
            //   - `height` em contexto inline alteraria line_height —
            //     refino futuro.
            //   - `baseline` exige offset vertical mid-linha — não
            //     suportado por cursor.rs actual.
            // `inset.top`/`inset.bottom` em contexto inline são complexos;
            // armazenados mas não aplicados (refino futuro).
            // P231 — Boxed +3 cosméticos cosméticos armazenados mas semantic real
            // adiada (outset visual ainda não aplicado; radius/clip primitivos
            // baseline ausentes — pattern N=5 → 7 cumulativo).
            // Atomizado (ADR-0109, P376) → layout/boxed.rs.
            Content::Boxed(e) => boxed::layout(self, e),

            // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ──
            // Container que ocupa nova "linha lógica" (força flush_line se
            // houver conteúdo pendente), aplica inset (análogo a Pad),
            // reserva altura mínima se `height: Some(h)`, e respeita a
            // largura disponível através do mecanismo `line_start_x`/
            // `flush_line` existente.
            //
            // `breakable` armazenado mas semantic real (impedir quebra
            // mid-block) defere — exigiria refactor multi-region (per
            // DEBT-56). Per ADR-0054 graded.
            //
            // `width` actualmente reduz a largura útil temporariamente
            // (cursor.x começa em line_start_x + offset). `width: None`
            // == auto (largura completa).
            // Atomizado (ADR-0109, P376) → layout/block.rs.
            Content::Block(e) => block::layout(self, e),

            // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ──
            // **Primeira aplicação Fase 3**. Variant + paridade
            // estrutural (single-render do body no contexto actual).
            // Algoritmo dinâmico de quantidade-para-encher (vanilla
            // calcula floor(available / (body_width + gap))) está
            // diferido per ADR-0054 graded — exige refactor inline-
            // region não disponível no Layouter actual (mesma razão
            // que `Block.width`/`Boxed.width` em P156G/H).
            //
            // `gap` armazenado mas não emite spacing entre cópias
            // (só uma cópia neste passo). `justify` armazenado mas
            // sem distribuição de espaço residual (idem).
            // Atomizado (ADR-0109, P380) → layout/repeat.rs.
            Content::Repeat(e) => repeat::layout(self, e),

            // ── P219 (DEBT-56 sub-fase b 3/4) — columns consumer REAL graded
            //
            // **Opção B fixada (paridade ADR-0054 graded)**: reduz width
            // temporariamente para column_width; body single-render na
            // primeira "coluna virtual"; width restaurada após body.
            //
            // **Multi-region flow real é SCOPE-OUT** — body overflow salta
            // para next page (não next column). Refino candidato a
            // P-Layout-Fase4 (Opção A multi-region completa); decisão
            // P216B `Regions { current: Region }` minimal preservada.
            //
            // Fórmula: column_width = (full_width - (count-1)*gutter) / count.
            // Default gutter ~4% full_width (constante COLUMNS_DEFAULT_GUTTER_RATIO).
            //
            // count=0 (caso degenerate construtor Rust; stdlib P218 valida >=1):
            // tratar como passthrough (count=1 equivalente; column_width=full_width).
            // Atomizado (ADR-0109, P377) → layout/columns.rs.
            Content::Columns(e) => columns::layout(self, e),

            // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak ──
            // `weak` armazenado mas collapse defere (consistente P156D).
            // Layouter reusa `new_page` (cursor.rs:128) que commits items
            // actuais a Page e reseta cursor.
            // Atomizado (ADR-0109, P380) → layout/pagebreak.rs.
            Content::Pagebreak(e) => pagebreak::layout(self, e),

            // ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak ──
            // Opção β graded: downgrade a pagebreak literal (paridade
            // vanilla quando fora de columns context). Reusa
            // `Layouter::new_page` (paridade P156E literal). Refino
            // multi-region salto entre colunas reais é P-Layout-Fase4
            // candidato (não-reservado per política P158).
            // `weak` armazenado mas semantic adiada (paridade P156D/E).
            // Atomizado (ADR-0109, P380) → layout/colbreak.rs.
            Content::Colbreak(e) => colbreak::layout(self, e),

            // ── Passo 284 + P285 + P286 (ADR-0054 graded) — text decoration ─
            //
            // Cluster decorações **COMPLETO** após P286:
            // - P284: 3 variants ricos + body recurse + cálculo offset.
            // - P285: `stroke` funcional + herança `style.fill` + emit RG.
            // - P286: **wrap-aware** — N linhas visuais → N `FrameItem::Line`.
            //
            // Consumer arquitectural: reusa `FrameItem::Line` para o emit PDF
            // (precedente Passo 38 frac); offset Y é a única dimensão que
            // varia entre underline/strike/overline (diagnóstico P284 §A.2).
            //
            // Algoritmo P286 (diagnóstico §A.2 opção b minimalista):
            //   1. Captura snapshot inicial `(start_x, baseline_y)`.
            //   2. Activa `decoration_lines_collector = Some(Vec::new())`.
            //   3. `layout_content(body)` — flush_line colecciona segments.
            //   4. Desactiva collector; drena vec.
            //   5. Acrescenta segment "final não-flushed"
            //      `(line_start_x | start_x, cursor_x_final, cursor_y_final)`.
            //   6. Emite 1 `FrameItem::Line` por segment com `line_y =
            //      seg.baseline_y + offset_pt`; aplica `extent` simétrico
            //      (P286 §A.3 opção α — vanilla painter parity).
            //
            // Fallback single-line bit-exact: se o collector ficar vazio
            // após `layout_content(body)` e o body permaneceu na mesma
            // linha, emite **exactamente** o algoritmo P284 (1 Line).
            // Validado por regression P285 + dedicated P286 test.
            // Atomizado (ADR-0109, P378) → layout/decorations.rs (arm agrupado).
            Content::Underline(_) | Content::Strike(_) | Content::Overline(_) => {
                decorations::layout(self, content)
            }

            // ── Passo 287 (frente `P-smartquote`) — função stdlib ──────────
            //
            // Consumer alterna estado open/close per-document do Layouter
            // (`smartquote_*_open`). Resolução lang-aware via reuso de
            // `rules/lang/quotes.rs::localize_quotes` — single source of
            // truth partilhada com markup `eval_markup` P155 (padrão "Win
            // arquitectural" §8.2 P286 N=3 → **N=4 cumulativo P287**).
            //
            // Glyph resolvido emitido como `Content::Text(glyph)` —
            // reutiliza o caminho `Content::Text` standard (passa por
            // hyphenation/wrap/font scenarios pré-existentes). Sem touch
            // points em `export.rs` (hash `66cb8ac3` preserved).
            //
            // Divergência aceite vs vanilla per ADR-0054 graded: markup
            // e função têm estados independentes; mistura programática +
            // markup literal (caso edge raro) pode produzir "2 opens
            // consecutivos" — registado em diagnóstico §A.3.2.
            // Atomizado (ADR-0109, P381) → layout/smartquote.rs.
            Content::SmartQuote(e) => smartquote::layout(self, e),
            Content::PdfAttach(e) => pdf_attach::layout(self, e),
            Content::PdfArtifact(e) => pdf_artifact::layout(self, e),

            // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ───────────
            // Atomizado (ADR-0109, P381) → layout/quote.rs.
            Content::Quote(e) => quote::layout(self, e),

            // P1493: ContextBlock continua no-op salvo na passagem intrínseca
            // detalhada, onde um carrier privado acompanha a geometria final.
            Content::ContextBlock(elem) => context_block::layout(self, elem),
            Content::HtmlElem(_) => {}
        }
        paragraph_indent::after_content(self, content);
        if matches!(content, Content::Equation(_) | Content::Image(_) | Content::Boxed(_))
        {
            self.smartquote_context.observe_object();
        }
    }
}

// ── Auxiliares ────────────────────────────────────────────────────────────

/// Layout com convergência de fixpoint (Passo 65).
///
/// Recebe o `CounterStateLegacy` produzido por `introspect::introspect`.
/// Se o documento não contiver `Content::Outline` (`has_outline = false`),
/// corre uma única passagem — o fixpoint de páginas só serve a TOC.
/// Caso contrário, itera até convergência (máximo 5 vezes).
///
/// Para métricas de fonte reais: `03_infra::layout::layout_with_font()`.
pub fn layout(content: &Content) -> PagedDocument {
    // P190I (M6 fechado): `initial_state: CounterStateLegacy` parameter
    // ELIMINADO — struct eliminada. layout() corre
    // `introspect_with_introspector` internamente para obter
    // `TagIntrospector` populated. API breaking change comparada
    // com versões anteriores; callers externos adaptados.
    let intr = crate::compiler::introspect::introspect_with_introspector(content);
    layout_with_introspector(content, intr)
}

/// Entry point P168 (M5 sub-passo 2): aceita `TagIntrospector` adicional
/// para que consumers como `references.rs::layout_ref` (figure-ref) possam
/// usar `query_by_label` em vez de `state.figure_label_numbers` legacy.
///
/// Caller típico:
/// ```rust
/// use typst_core::compiler::introspect::introspect_with_introspector;
/// use typst_core::compiler::layout::layout_with_introspector;
/// use typst_core::entities::content::Content;
///
/// let content = Content::text("Hello");
/// let intr = introspect_with_introspector(&content);
/// let doc = layout_with_introspector(&content, intr);
/// assert!(doc.plain_text().contains("Hello"));
/// ```
///
/// **P190I (M6 fechado)**: signature drop `initial_state:
/// CounterStateLegacy` parameter — struct eliminada.
///
/// **P544**: este entry point mantém o comportamento legacy com
/// `FixedMetrics`. A pipeline de produção usa
/// `layout_with_introspector_and_metrics` para métricas reais de fonte.
pub fn layout_with_introspector(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
) -> PagedDocument {
    layout_with_introspector_and_metrics(
        content,
        introspector,
        FixedMetrics,
        NullImageSizer,
        DEFAULT_FONT_SIZE,
    )
    .expect("legacy layout entrypoint cannot resolve math callbacks")
}

/// Layouta uma ocorrência de `html.frame` como fragmento paginado intrínseco.
pub fn layout_html_frame<M: FontMetrics, S: ImageSizer>(
    elem: &crate::entities::html::HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
) -> Page {
    html_frame::layout_intrinsic(elem, chain, metrics, sizer)
}
/// P1493 — layout intrínseco com posições locais dos ContextBlocks.
pub fn layout_html_frame_detailed<M: FontMetrics, S: ImageSizer>(
    elem: &crate::entities::html::HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
) -> HtmlFrameLayout {
    html_frame::layout_intrinsic_detailed(elem, chain, metrics, sizer)
}

/// **P544** — entry point genérico que permite injectar métricas de fonte
/// reais (com fallback multi-script) no layout. Usado pela pipeline de
/// produção em L3.
pub fn layout_with_introspector_and_metrics<
    M: FontMetrics + Clone,
    S: ImageSizer + Clone,
>(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
) -> SourceResult<PagedDocument> {
    match layout_with_introspector_and_metrics_and_math_callbacks(
        content,
        introspector,
        metrics,
        sizer,
        font_size,
        None,
    ) {
        crate::compiler::math::layout::callbacks::MathLayoutPassOutcome::Complete(
            doc,
        ) => Ok(doc),
        crate::compiler::math::layout::callbacks::MathLayoutPassOutcome::Pending(_) => {
            Err(vec![SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "math callbacks require the callback-aware layout entrypoint",
            )])
        }
    }
}

pub fn layout_with_introspector_and_metrics_and_math_callbacks<
    M: FontMetrics + Clone,
    S: ImageSizer + Clone,
>(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    callback_store: Option<
        &crate::compiler::math::layout::callbacks::SealedMathCallbacks,
    >,
) -> crate::compiler::math::layout::callbacks::MathLayoutPassOutcome {
    match layout_with_introspector_and_metrics_and_callbacks(
        content,
        introspector,
        metrics,
        sizer,
        font_size,
        callback_store,
        None,
    ) {
        LayoutPassOutcome::Complete(document) => {
            crate::compiler::math::layout::callbacks::MathLayoutPassOutcome::Complete(
                document,
            )
        }
        LayoutPassOutcome::Pending(requests) => {
            crate::compiler::math::layout::callbacks::MathLayoutPassOutcome::Pending(
                requests.math,
            )
        }
    }
}

pub fn layout_with_introspector_and_metrics_and_callbacks<
    M: FontMetrics + Clone,
    S: ImageSizer + Clone,
>(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    math_callback_store: Option<
        &crate::compiler::math::layout::callbacks::SealedMathCallbacks,
    >,
    par_line_callback_store: Option<&SealedParLineCallbacks>,
) -> LayoutPassOutcome {
    layout_with_introspector_and_metrics_and_callbacks_from(
        content,
        introspector,
        metrics,
        sizer,
        font_size,
        math_callback_store,
        par_line_callback_store,
        Locator::new().checkpoint(),
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "C017: Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos"
)]
pub fn layout_with_introspector_and_metrics_and_callbacks_from<
    M: FontMetrics + Clone,
    S: ImageSizer + Clone,
>(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    math_callback_store: Option<
        &crate::compiler::math::layout::callbacks::SealedMathCallbacks,
    >,
    par_line_callback_store: Option<&SealedParLineCallbacks>,
    location_start: LocatorSeed,
) -> LayoutPassOutcome {
    convergence::run(
        content,
        introspector,
        metrics,
        sizer,
        font_size,
        math_callback_store,
        par_line_callback_store,
        location_start,
    )
}

// ── Helpers P418 ───────────────────────────────────────────────────────────

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests;
