//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout.md
//! @prompt-hash 33e0e43e
//! @layer L1
//! @updated 2026-07-14

pub mod counters;
pub mod figure;
pub mod image;
pub mod outline;
pub mod references;

use crate::engine::introspect::locatable::is_locatable;
use crate::entities::{
    content::Content,
    counter_format::count_numbering_tokens,
    geometry::ShapeKind,
    image_sizer::{ImageSizer, NullImageSizer},
    label::Label,
    layout_types::{
        Align2D, FrameItem, HAlign, Page, PageConfig, PagedDocument, Point, Pt,
        TextStyle, VAlign,
    },
    location::Location,
    locator::Locator,
    source_result::SourceDiagnostic,
    style_chain::StyleChain,
};
use ecow::EcoString;
use hayagriva::citationberg::IndependentStyle;
use std::sync::Arc;

// FontMetrics / FixedMetrics extraídos para metrics.rs (Passo 96.7, ADR-0037).
mod metrics;
pub use crate::engine::layout::metrics::{needs_shaped_width, FixedMetrics, FontMetrics};

// Braços pesados do `layout_content` extraídos por cluster (Passo 96.7).
mod equation;
mod grid;
mod placement;

// Sub-layout isolado de conteúdo numa região (Passo 629).
mod sub_frame;

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
mod shape;
mod title;
mod transform;

// Atomização visuais/decorações (ADR-0109, P378): mesma forma B.
// (Image/Figure completam-se nos seus próprios arquivos image.rs/figure.rs.)
mod decorations;
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
pub mod bib_csl;
mod bibliography;
mod cite;
mod divider;
mod dynamic;
mod footnote;
mod hide;
mod link;
mod quote;
mod raw;
mod sequence;
mod set_page;
mod smartquote;

// Helpers livres usados pelo Layouter e pelos braços extraídos.
pub(crate) mod helpers;
// P224.C — Placement algorítmico Grid (fecha DEBT-34e colspan/rowspan).
pub(crate) mod grid_placement;
// P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial) — slice frame
// items por threshold em pos.y para row break TableCell cell-level.
mod slicing;
use crate::engine::layout::helpers::{
    item_pos, measure_content, resolve_pt, translate_frame_item,
};

// Gestão de cursor: word/space, layout_word, flush_line, new_page.
mod cursor;

// Flush dos footnote bodies pendentes (P847 — extraído de `cursor.rs`).
mod footnote_flush;

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

// ── Layouter ──────────────────────────────────────────────────────────────

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
    /// Cadeia de estilos activa — source-of-truth do estilo (Passo 100,
    /// ADR-0039). `Content::Styled` faz push; o save/restore de Strong/
    /// Emph/Heading/Text também passa por esta cadeia. `self.style` é a
    /// vista achatada (cache) que o layout consulta directamente.
    pub(super) chain: StyleChain,
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
    /// **P295 (Footnote Fase 1)** — counter monotónico incrementado em
    /// cada `Content::Footnote` consumido. Marker `[N]` emitido como
    /// superscript inline. Walker counter simples (sem
    /// Counter/Introspector machinery) — magnitude reduzida para Fase 1.
    /// Sub-passos P295.1 (nota rodapé) + P295.2 (overflow) migrarão
    /// para Counter machinery se 2-pass layout for adoptado.
    pub(super) footnote_counter: u32,
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
    /// **P185C** — `Location` do último content locatable processado.
    /// `None` antes de processar qualquer locatable. Consumers
    /// location-aware (`flat_counter_at`, P185B) consultam este campo
    /// em vez de snapshot final (cf. ADR-0068 PROPOSTO). (F-4 E0, P338:
    /// `is_numbering_active_at` saiu — gate vive no campo assado.)
    pub(super) current_location: Option<Location>,
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
    /// **P250 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa; cita
    /// ADR-0082 PROPOSTO N=1)** — below pendente do bloco anterior
    /// para CSS-style margin collapse `max(prev.below, curr.above)`
    /// entre blocks consecutivos. Reset por Sequence consumer +
    /// non-Block arms.
    pub(super) prev_block_below_pending: f64,
    /// **P250** — `true` se o elemento previamente laid out foi
    /// `Content::Block`. Used by `prev_block_below_pending` collapse
    /// logic + first-block-in-sequence above suppression. Reset por
    /// non-Block arms (via Sequence consumer ou directamente).
    pub(super) block_chain_active: bool,
    /// **P813** — avanço vertical aplicado pelo último `flush_line()`
    /// com items (`top + |bottom| + leading`). Permite a consumidores
    /// posteriores (equações de bloco) recuperar a baseline da linha
    /// anterior como `cursor_y - last_flush_advance`. Reset em
    /// `new_page`; save/restore em `layout_sub_frame`.
    pub(super) last_flush_advance: f64,
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
    /// **P287 (frente `P-smartquote`)** — estado de alternância open/close
    /// para `Content::SmartQuote { double }` emitido pela função stdlib
    /// `#smartquote(...)`. **Independente** do estado do markup parser
    /// (`eval_markup` local var; P155) — diagnóstico P287 §A.3 opção
    /// (γ′): markup e função têm estados próprios; bit-exact do markup
    /// preservado. Per-document (reset em `Layouter::new`).
    pub(super) smartquote_double_open: bool,
    pub(super) smartquote_single_open: bool,
    /// **P418** — cache pré-renderizado CSL para citações/bibliografia.
    /// Populado em `layout_with_introspector` antes do layout principal.
    pub(super) bib_render_cache: Option<crate::engine::layout::bib_csl::BibRenderCache>,
    /// **P446** — smallcaps activo no corpo de um `#smallcaps[...]`.
    pub(super) smallcaps: bool,
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
    /// **P538c** — items já fechados de cada coluna da página actual.
    /// Só preenchido quando `page_columns` é `Some`.
    pub(super) column_page_items: Vec<Vec<FrameItem>>,
    /// **P538c** — posições horizontais absolutas (x) de cada coluna na
    /// página actual. Usado para translação e avanço de coluna.
    pub(super) column_x_offsets: Vec<f64>,
    /// **P541** — numeração de página adiada que precisa do total de páginas.
    /// Cada entrada: (índice da página no Vec, número da página, pattern).
    /// O FrameItem::Text é adicionado no final de `finish()` quando o total
    /// de páginas é conhecido.
    pub(super) pending_page_numbering: Vec<(usize, usize, ecow::EcoString)>,
    /// **P595** — avisos produzidos durante o layout. L1 puro: strings
    /// simples, sem construção de `SourceDiagnostic` nem acesso a Sink.
    /// Exportado no `PagedDocument` e convertido a diagnósticos em L3.
    pub(super) layout_warnings: Vec<String>,
    /// **P644/P645** — erros produzidos durante o layout (ex: conversão de
    /// entrada bibliográfica). Guardados como `SourceDiagnostic` para
    /// preservar `span` e posição no ficheiro.
    pub(super) layout_errors: Vec<SourceDiagnostic>,
}

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
    /// silenciosa — ver `00_nucleo/diagnosticos/paridade-producao-p772x.md`.
    pub deco_segments: Vec<DecoSegment>,
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
        font_size: f64,
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
            chain: StyleChain::default_chain(),
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
                rs.current.cursor_x = Pt(cfg.margin);
                rs.current.cursor_y = Pt(cfg.margin);
                rs.current.line_start_x = Pt(cfg.margin);
                rs
            },
            // P751 — baseline inicial ainda não fixada; será ajustada na
            // primeira emissão de conteúdo real com o estilo activo.
            initial_baseline_pending: true,
            // P190I: counter field eliminated.
            // P204C: field passa a ser Tracked, recebido por parameter.
            introspector,
            figure_progress: std::collections::HashMap::new(),
            footnote_counter: 0,
            is_height_unconstrained: false,
            is_sub_frame: false,
            // P246 — cell_available_h + cell_origin_w migrados a
            // regions.cell (entity-side). cell_origin_x/y preservados
            // como Layouter fields legacy.
            cell_origin_x: None,
            cell_origin_y: None,
            // P273.5 — fallback None; callsite L3 usa page_bbox.
            parent_bbox: None,
            cell_align: None, // P232
            locator: Locator::new(),
            current_location: None,
            runtime:
                crate::entities::layouter_runtime_state::LayouterRuntimeState::default(),
            // P245 (M9d / M7+4) — buffer floats + reservas inicializados vazios.
            floats_pending: Vec::new(),
            cursor_y_top_reserve: 0.0,
            cursor_y_bottom_reserve: 0.0,
            enum_counter: None,
            // P250 — spacing collapse state inicializado limpo.
            prev_block_below_pending: 0.0,
            block_chain_active: false,
            // P813 — sem flush prévio; a primeira equação de bloco usa o
            // caminho `initial_baseline_pending` (topo da página).
            last_flush_advance: 0.0,
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
            column_page_items: Vec::new(),
            column_x_offsets: Vec::new(),
            // P286 — collector inactivo por default; consumer P284 activa
            // localmente antes de layout_content do body decorado.
            decoration_lines_collector: None,
            // P287 — estado smartquote per-document (true = próximo é open).
            smartquote_double_open: true,
            smartquote_single_open: true,
            // P446 — smallcaps activo no corpo de um `#smallcaps[...]`.
            smallcaps: false,
            // P418 — cache pré-renderizado de citações/bibliografia CSL.
            bib_render_cache: None,
            // P472 — última key citada para ibid.
            last_cited_key: None,
            // P473 — keys citadas anteriormente para op. cit.
            previously_cited_keys: std::collections::HashSet::new(),
            // P505 — estado de espaçamento entre itens de lista soltos.
            last_was_loose_item: false,
            // P541 — numeração adiada para patterns compostos (ex: "1 / 1").
            pending_page_numbering: Vec::new(),
            // **P595** — avisos de layout inicializados vazios.
            layout_warnings: Vec::new(),
            // **P644** — erros de layout inicializados vazios.
            layout_errors: Vec::new(),
        }
    }

    /// Largura disponível para conteúdo (exclui margens dos dois lados).
    pub(super) fn available_width(&self) -> f64 {
        f64::max(0.0, self.regions.current.width - 2.0 * self.page_config.margin)
    }

    /// Altura disponível para conteúdo (exclui margens topo e base).
    #[allow(dead_code)]
    pub(super) fn available_height(&self) -> f64 {
        f64::max(0.0, self.regions.current.height - 2.0 * self.page_config.margin)
    }

    /// Limite inferior da página em pontos (`height - margin`). Passo 82.
    ///
    /// Usar este método em vez de `page_config.height - page_config.margin`
    /// inline — evita confundir com `available_height()` (que subtrai 2×margin).
    pub(super) fn page_bottom_limit(&self) -> f64 {
        self.regions.current.height - self.page_config.margin
    }

    /// Calcula a coordenada `(x, y)` do canto superior esquerdo de um item
    /// dado o alinhamento, as dimensões do conteúdo, e a área disponível.
    /// Passo 82.
    ///
    /// `origin_x` e `origin_y` definem o canto superior esquerdo da área
    /// de referência (`line_start_x` para Align; `line_start_x`/`margin` para Place).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_alignment(
        &self,
        align: Align2D,
        content_w: f64,
        content_h: f64,
        available_w: f64,
        available_h: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> (f64, f64) {
        let x = match align.h.unwrap_or(HAlign::Left) {
            HAlign::Left | HAlign::Start => origin_x,
            HAlign::Center => origin_x + (available_w - content_w) / 2.0,
            HAlign::Right | HAlign::End => origin_x + (available_w - content_w),
        };

        let y = match align.v.unwrap_or(VAlign::Top) {
            VAlign::Top => origin_y,
            VAlign::Horizon => origin_y + (available_h - content_h) / 2.0,
            VAlign::Bottom => origin_y + (available_h - content_h),
        };

        (x, y)
    }

    /// Fonte de verdade estrutural: a página actual não tem nenhum item visual.
    ///
    /// Verifica tanto `current_items` (linhas já fechadas) como `current_line`
    /// (items ainda pendentes de flush) — uma linha não fechada ainda constitui
    /// conteúdo visível na página.
    fn current_page_is_empty(&self) -> bool {
        self.regions.current.current_items.is_empty()
            && self.regions.current.current_line.is_empty()
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
            self.runtime
                .positions
                .insert(loc, crate::entities::position::Position { page, point });
        }
    }

    pub fn layout_content(&mut self, content: &Content) {
        // P185C: gating Locator atómico no topo, antes do match.
        // Avança em sincronia com walk de introspect; current_location
        // fica disponível para consumers location-aware (P187/P188).
        self.advance_locator_if_locatable(content);

        match content {
            Content::Empty => {}

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
            Content::StateDisplay(e) => {
                use crate::entities::introspector::Introspector;
                if let Some(loc) = self.current_location {
                    let pre_rendered_opt =
                        self.introspector.state_display_value(e.key.clone(), loc);
                    if let Some(pre_rendered) = pre_rendered_opt {
                        self.layout_content(&pre_rendered);
                    }
                    // Sem pre_rendered: defensive ignore (fixpoint pre-walk
                    // ainda não convergiu OR Func errored OR key inexistente).
                }
                // Sem current_location: defensive ignore (walk pre-Locator).
            }

            // P241 (M9d/M7+2): CounterDisplayCallback consome Content
            // pre-rendered pelo `apply_counter_displays` pós-fixpoint via
            // `Introspector::counter_display_value(key, loc)`. Layouter
            // permanece puro (paridade absoluta P240). Distinto de
            // `Content::CounterDisplay { kind }` legacy single-pass.
            Content::CounterDisplayCallback(e) => {
                use crate::entities::introspector::Introspector;
                if let Some(loc) = self.current_location {
                    let pre_rendered_opt =
                        self.introspector.counter_display_value(e.key.clone(), loc);
                    if let Some(pre_rendered) = pre_rendered_opt {
                        self.layout_content(&pre_rendered);
                    }
                }
            }

            // Atomizado (ADR-0109, P381) → layout/text.rs (folha de render).
            Content::Text(text) => text::layout(self, text),

            Content::Space => {
                // P588 — não renderizar espaço visual no início de uma linha
                // ou parágrafo. Espaços iniciais são gerados por newlines após
                // `#set`, headings, etc., e não devem avançar o cursor.
                if !self.regions.current.current_line.is_empty() {
                    self.regions.current.cursor_x += self.space_width();
                    if self.regions.current.cursor_x.0
                        > self.regions.current.width - self.page_config.margin
                    {
                        self.flush_line();
                    }
                }
            }

            // P622 — quebra de parágrafo semântica: drena a linha actual,
            // avançando verticalmente por line_height + leading.
            Content::Parbreak => {
                self.flush_line();
            }

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

            Content::CounterDisplay(e) => {
                // P190I (M6 fechado): Layouter consome via Introspector
                // path location-aware. `current_location` set por
                // walk-content para locatable contents (P185C). Para
                // CounterDisplay (não-locatable), usa última location
                // emitida (snapshot até este ponto).
                use crate::entities::introspector::Introspector;
                let kind = &e.kind;
                let text = self
                    .current_location
                    .and_then(|loc| self.introspector.formatted_counter_at(kind, loc))
                    .unwrap_or_else(|| {
                        self.introspector
                            .formatted_counter(kind)
                            .unwrap_or_else(|| "0".to_string())
                    });
                let display = Content::text(text);
                self.layout_content(&display);
            }

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
            | Content::MathRoot(_)
            | Content::MathDelimited(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathClassOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_) => self.layout_math_fallback(content),

            // Marcadores estruturais de equações — ignorados fora de contexto matemático.
            Content::MathAlignPoint(_) => {}

            // Quebra de linha explícita (`\\` e pré-renderizações como bibliografia CSL).
            Content::Linebreak(_) => self.flush_line(),

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

            // Passo 61/P457 — TOC: delegado a outline.rs.
            Content::Outline(e) => outline::layout_outline(self, e),

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

            // Atomizado (ADR-0109, P381) → layout/cite.rs.
            Content::Cite(e) => cite::layout(self, e),

            // Atomizado (ADR-0109, P425) → layout/set_page.rs.
            Content::SetPage { width, height, margin, numbering, columns } => {
                set_page::layout(self, width, height, margin, numbering, columns);
            }

            // Atomizado (ADR-0109, P378) → layout/image.rs.
            Content::Image(e) => image::layout(self, e),

            Content::Align(e) => {
                self.layout_align(e.alignment, &e.body);
            }

            // P223 — Place refino: float + clearance armazenados.
            // P245 (M9d / M7+4) — semantic real activa para `float: true`:
            // body layouted em sub-frame, capturado em buffer
            // `floats_pending`, emitido no flush da página (new_page/
            // finish). `float: false` preserva comportamento P84.5+P84.6
            // literal (body in-place via cursor).
            // Atomizado (ADR-0109, P378) → layout/place.rs.
            Content::Place(e) => place::layout(self, e),

            // Passo 100 (ADR-0039): `Content::Styled` activa push/pop na
            // `chain` interna. A vista achatada `self.style` é
            // re-sincronizada a partir da cadeia após push e depois do pop.
            // Save/restore via variável local — o `self.chain` do chamador
            // permanece íntegro se `layout_content` retornar via early
            // return (padrão Passo 98).
            Content::Styled(body, styles) => {
                let prev_chain = self.chain.clone(); // O(1) Arc::clone
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(styles);
                self.style = TextStyle::from(&self.chain);
                self.layout_content(body);
                self.chain = prev_chain;
                self.style = prev_style;
            }

            // F-5b fatia 1 (P371): strong/emph são variantes próprias (morfologia,
            // 0107). O render (bold/italic) **replica** o arm `Styled` acima — push
            // de `Bold(true)`/`Italic(true)` na chain, layout do body, restore →
            // output byte-idêntico ao `Styled[Bold/Italic]` de antes (paridade visual).
            Content::Strong(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain =
                    self.chain.push_styles(&Styles::from_iter([Style::strong()]));
                self.style = TextStyle::from(&self.chain);
                self.layout_content(&e.body);
                self.chain = prev_chain;
                self.style = prev_style;
            }
            Content::Emph(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(&Styles::from_iter([Style::emph()]));
                self.style = TextStyle::from(&self.chain);
                self.layout_content(&e.body);
                self.chain = prev_chain;
                self.style = prev_style;
            }

            // P446: smallcaps — fallback por scaling (0.8×) de maiúsculas.
            // Letras minúsculas são convertidas para maiúsculas e renderizadas
            // a 80% do tamanho actual; maiúsculas e não-letras mantêm o
            // tamanho. O flag `self.smallcaps` sinaliza ao layout de `Text`
            // para fazer a segmentação por runs.
            Content::SmallCaps { body } => {
                let prev = self.smallcaps;
                self.smallcaps = true;
                self.layout_content(body);
                self.smallcaps = prev;
            }

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

            // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ───────────
            // Atomizado (ADR-0109, P381) → layout/quote.rs.
            Content::Quote(e) => quote::layout(self, e),

            // ── P506 — ContextBlock (delayed evaluation). Deve ter sido
            // expandido antes do layout; se chegou aqui, é defensive no-op.
            Content::ContextBlock(_) => {}
        }
    }
    pub fn finish(mut self) -> PagedDocument {
        // P842 (#38) — a última linha também expande h(Nfr) pendentes
        // (paridade vanilla: fr consome o espaço restante mesmo na linha
        // final do documento — medido em `temp/p842/l7_h_1fr.typ`).
        self.expand_fr_spacings();
        // P576 — a última linha também pode ser RTL; alinhar antes de drenar.
        self.align_current_line_rtl();
        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        // P245 (M9d / M7+4) — flush floats pendentes da última página
        // antes de comitar a Page final.
        self.flush_pending_floats();
        // P304 (P295.1) — flush footnote bodies pendentes da última
        // página antes de comitar a Page final. Subpadrão DeferredX
        // N=3 paralelo a P245/P251.
        self.flush_pending_footnote_bodies(None);
        // P305 (P295.2) — overflow: se bodies sobraram no buffer,
        // criar páginas adicionais (`new_page()`) até buffer vazio.
        // Cada iteração: new_page() saves current page + flush
        // next batch. Iter limit defensivo (paralelo P251
        // forwarded_count limit) — assume cada iteração emite ≥1
        // body via defensive first-body fallback.
        let mut iter_limit = self.pending_footnote_bodies.len() + 1;
        while !self.pending_footnote_bodies.is_empty() && iter_limit > 0 {
            self.new_page();
            iter_limit -= 1;
        }
        if !self.regions.current.current_items.is_empty() || !self.pages.is_empty() {
            let page_numbering = self.page_config.numbering.clone();
            let page_number = self.pages.len() + 1;
            let mut items = self.regions.current.current_items;

            // **P532** — numeração automática na última página.
            // **P538d** — o texto de numeração deve usar o estilo activo da
            // página (StyleChain), não `TextStyle::regular`, para que `font`
            // esteja definida. Mesma correção de P483 para texto normal.
            // **P541** — padrões compostos (≥2 tokens de numeração) adiam-se
            // porque precisam do total de páginas, só conhecido no final.
            if let Some(pattern) = &page_numbering {
                if count_numbering_tokens(pattern) >= 2 {
                    self.pending_page_numbering.push((
                        self.pages.len(),
                        page_number,
                        pattern.clone(),
                    ));
                } else if let Some(text) = crate::entities::counter_format::format_counter(
                    &[page_number],
                    pattern.as_str(),
                ) {
                    let style = TextStyle::from(&self.chain);
                    let text_width = self.metrics.advance(&text, style.size, &style).0;
                    let x = (self.regions.current.width - text_width) / 2.0;
                    let y = self.regions.current.height - self.page_config.margin / 2.0;
                    items.push(FrameItem::Text {
                        pos: Point { x: Pt(x), y: Pt(y) },
                        text: text.into(),
                        style,
                    });
                }
            }

            let page = Page {
                width: self.regions.current.width,
                height: self.regions.current.height,
                numbering: page_numbering,
                items,
            };
            self.pages.push(page);
        }

        // **P541** — aplicar numerações de página adiadas agora que o total
        // de páginas é conhecido. Cada entrada pendente contém o índice da
        // página, o número da página e o pattern composto.
        let total_pages = self.pages.len();
        for (page_idx, page_number, pattern) in
            std::mem::take(&mut self.pending_page_numbering)
        {
            if let Some(text) = crate::entities::counter_format::format_counter(
                &[page_number, total_pages],
                pattern.as_str(),
            ) {
                if let Some(page) = self.pages.get_mut(page_idx) {
                    let style = TextStyle::from(&self.chain);
                    let text_width = self.metrics.advance(&text, style.size, &style).0;
                    let x = (page.width - text_width) / 2.0;
                    let y = page.height - self.page_config.margin / 2.0;
                    page.items.push(FrameItem::Text {
                        pos: Point { x: Pt(x), y: Pt(y) },
                        text: text.into(),
                        style,
                    });
                }
            }
        }

        let mut doc = PagedDocument::new(self.pages);
        // Expor o mapa de páginas sem mudar a assinatura de layout() (Passo 63).
        // P190C (M6 categoria Page tracking): label_pages movido para
        // LayouterRuntimeState.
        doc.extracted_label_pages = self.runtime.label_pages;
        // P460 — expor posições dos labels para /Dests no export PDF.
        doc.extracted_label_positions = self.runtime.label_positions;
        // P205B (F3): sealing point — extrai runtime.positions para
        // sub-store sealed `SealedPositions` per ADR-0074. Tracked
        // via comemo; consumer migration em P205C.
        doc.extracted_positions =
            crate::entities::sealed_positions::SealedPositions::from_runtime(
                self.runtime.positions,
            );
        // P488 — expor páginas de figuras/tabelas para fixpoint carry-forward (LoF/LoT).
        doc.extracted_figure_page_numbers = self.runtime.figure_page_numbers;
        doc.extracted_table_page_numbers = self.runtime.table_page_numbers;
        // **P595** — exportar avisos de layout acumulados no Layouter.
        doc.layout_warnings = self.layout_warnings;
        doc.layout_errors = self.layout_errors;
        doc
    }

    // ── Auxiliares de Grid (Passo 80) ─────────────────────────────────────

    /// P273.11 — Mede um Stack (children + dir + spacing) com `max_w`.
    /// Helper extraído da replicação inline P273.9 §2.2 (cleanup §9 P273.9).
    /// Decisão 1β Fase A: método em Layouter (reutiliza
    /// `measure_content_constrained` via `&self`).
    pub(super) fn measure_stack(
        &mut self,
        children: &[Content],
        dir: crate::entities::dir::Dir,
        spacing: Option<crate::entities::layout_types::Length>,
        max_w: f64,
    ) -> (f64, f64) {
        let n = children.len();
        if n == 0 {
            return (0.0, 0.0);
        }
        let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(self.style.size.val()));
        if dir.is_vertical() {
            let mut max_child_w = 0.0_f64;
            let mut sum_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                max_child_w = max_child_w.max(w);
                sum_h += h;
            }
            (max_child_w, sum_h + ((n - 1) as f64) * space_pt)
        } else {
            let mut sum_w = 0.0_f64;
            let mut max_child_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                sum_w += w;
                max_child_h = max_child_h.max(h);
            }
            (sum_w + ((n - 1) as f64) * space_pt, max_child_h)
        }
    }

    /// Mede conteúdo com restrição de largura máxima.
    ///
    /// Usado pelo algoritmo de grid para determinar a largura das colunas Auto.
    /// Retorna `(width, height)` em pontos.
    pub(super) fn measure_content_constrained(
        &mut self,
        content: &Content,
        max_width: f64,
    ) -> (f64, f64) {
        match content {
            Content::Text(text) => {
                let mut max_line_w = 0.0_f64;
                let mut current_w = 0.0_f64;
                let mut line_count = 1usize;
                // **P624** — usa `text_width` (P593) para herdar shaping em
                // scripts contextuais, em vez de `advance` directo.
                let space_w =
                    self.metrics.text_width(" ", self.style.size, &self.style).0;

                for word in text.split_whitespace() {
                    let word_w =
                        self.metrics.text_width(word, self.style.size, &self.style).0;
                    if current_w + word_w > max_width && current_w > 0.0 {
                        max_line_w = max_line_w.max(current_w);
                        line_count += 1;
                        current_w = word_w + space_w;
                    } else {
                        current_w += word_w + space_w;
                    }
                }
                max_line_w = max_line_w.max(current_w);
                let (_, line_height) =
                    self.metrics.vertical_metrics(self.style.size, &self.style);
                (max_line_w.min(max_width), line_height.0 * line_count as f64)
            }

            Content::Sequence(children) => {
                let mut total_h = 0.0_f64;
                let mut max_w = 0.0_f64;
                for child in children.iter() {
                    let (w, h) = self.measure_content_constrained(child, max_width);
                    total_h += h;
                    max_w = max_w.max(w);
                }
                (max_w, total_h)
            }

            Content::Shape(e) => {
                let (kind, width, height) = (&e.kind, &e.width, &e.height);
                match kind {
                    // P242 — RoundedRect partilha dimensões com Rect.
                    ShapeKind::Rect
                    | ShapeKind::RoundedRect { .. }
                    | ShapeKind::Ellipse
                    | ShapeKind::Path(_) => {
                        let w = resolve_pt(width.as_deref(), max_width).min(max_width);
                        let h = resolve_pt(height.as_deref(), 0.0);
                        (w, h)
                    }
                    ShapeKind::Line { dx, dy } => (dx.abs().min(max_width), dy.abs()),
                }
            }

            // Passo 513 — medição de `Content::Curve` via bbox do path.
            Content::Curve(e) => {
                let items =
                    curve::path_items_from_curve(&e.segments, self.style.size.val());
                let (min_x, min_y, max_x, max_y) =
                    crate::entities::geometry::path_bbox(&items);
                let w = (max_x - min_x).max(0.0).min(max_width);
                let h = (max_y - min_y).max(0.0);
                (w, h)
            }

            // Passo 156C / 156L: Pad / Hide para grid measurement.
            // P156L: cada side é Option<Length>; None ↔ zero.
            Content::Pad(e) => {
                let sides = &e.sides;
                let font = self.style.size.val();
                let left = sides.left.map_or(0.0, |l| l.resolve_pt(font));
                let right = sides.right.map_or(0.0, |l| l.resolve_pt(font));
                let top = sides.top.map_or(0.0, |l| l.resolve_pt(font));
                let bottom = sides.bottom.map_or(0.0, |l| l.resolve_pt(font));
                let constrained = (max_width - left - right).max(0.0);
                let (w, h) = self.measure_content_constrained(&e.body, constrained);
                (w + left + right, h + top + bottom)
            }
            Content::Hide(e) => self.measure_content_constrained(&e.body, max_width),

            // P408: smallcaps — stub transparente em medição (paridade layout).
            Content::SmallCaps { body } => {
                self.measure_content_constrained(body, max_width)
            }

            // Passo 156D: HSpace/VSpace dimensões para grid measurement.
            // **P842 (#38)** — `Fractional` mede 0 em contexto de medição
            // (o fr só expande contra o espaço restante de uma linha real;
            // numa medição isolada não há restante definido).
            Content::HSpace(e) => (
                match e.amount {
                    crate::entities::elements::h_space::Spacing::Absolute(l) => {
                        l.resolve_pt(self.style.size.val())
                    }
                    crate::entities::elements::h_space::Spacing::Fractional(_) => 0.0,
                },
                0.0,
            ),
            Content::VSpace(e) => (0.0, e.amount.resolve_pt(self.style.size.val())),

            // Passo 156E/220: Pagebreak/Colbreak — events sem dimensões em cell.
            Content::Pagebreak(_) => (0.0, 0.0),
            Content::Colbreak(_) => (0.0, 0.0),

            // Passo 156I: Stack dimensões para grid measurement.
            // TTB/BTT: max widths; sum heights + (n-1) * spacing.
            // LTR/RTL: sum widths + (n-1) * spacing; max heights.
            // P273.11 — delega ao helper Layouter::measure_stack (substitui
            // replicação inline ~25 LOC; bit-exact preserved).
            Content::Stack(e) => {
                self.measure_stack(&e.children, e.dir, e.spacing, max_width)
            }

            // Passo 156H: Boxed (Box inline) dimensões para grid
            // measurement. Análogo a Block (mesma lógica width/height/
            // inset; baseline ignorado em medição).
            Content::Boxed(e) => {
                let (body, width, height, inset) =
                    (&e.body, &e.width, &e.height, &e.inset);
                let font = self.style.size.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = match width {
                    Some(w) => w.resolve_pt(font).min(max_width - inset_l - inset_r),
                    None => (max_width - inset_l - inset_r).max(0.0),
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                let total_w = bw + inset_l + inset_r;
                let body_h_with_inset = bh + inset_t + inset_b;
                let total_h = match height {
                    Some(h) => h.resolve_pt(font).max(body_h_with_inset),
                    None => body_h_with_inset,
                };
                (total_w, total_h)
            }

            // Passo 156J: Repeat dimensões para grid measurement.
            // Single-render do body (consistente with layout_content
            // arm). Algoritmo dinâmico de quantidade defere per
            // ADR-0054 graded.
            Content::Repeat(e) => self.measure_content_constrained(&e.body, max_width),

            // P219 (DEBT-56 sub-fase b 3/4): Columns dimensões para grid
            // measurement. Consumer real graded — calcula column_width
            // (paralelo a layout_content arm); medir body com width
            // reduzida; retorna full_width (columns ocupa width inteira)
            // + body_h (single-render graded).
            Content::Columns(e) => {
                let count_f = if e.count == 0 { 1.0 } else { e.count as f64 };
                let gutter_pt = match e.gutter {
                    Some(g) => g.resolve_pt(self.style.size.0),
                    None => max_width * COLUMNS_DEFAULT_GUTTER_RATIO,
                };
                let column_width = if count_f >= 1.0 {
                    (max_width - (count_f - 1.0) * gutter_pt) / count_f
                } else {
                    max_width
                };
                let (_body_w, body_h) =
                    self.measure_content_constrained(&e.body, column_width);
                (max_width, body_h)
            }

            // Passo 156G: Block dimensões para grid measurement.
            // Inset adiciona aos lados; height: Some(h) força mínimo;
            // width: Some(w) prefere essa largura mas constrained por max.
            Content::Block(e) => {
                let (body, width, height, inset) =
                    (&e.body, &e.width, &e.height, &e.inset);
                let font = self.style.size.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = match width {
                    Some(w) => w.resolve_pt(font).min(max_width - inset_l - inset_r),
                    None => (max_width - inset_l - inset_r).max(0.0),
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                // P772f — largura explícita do bloco é a largura reportada
                // (o corpo mede-se dentro dela, mas não a substitui). Antes
                // usava sempre `bw`, colapsando a 0 quando o corpo era um
                // `Content::Align`/`Content::Place` sem braço próprio (ex:
                // `block(width: 3cm, align(top, place(..)))` numa coluna
                // auto de grid) — a coluna auto-dimensionava para 0pt.
                let total_w = match width {
                    Some(w) => w.resolve_pt(font).min(max_width),
                    None => bw + inset_l + inset_r,
                };
                let body_h_with_inset = bh + inset_t + inset_b;
                let total_h = match height {
                    Some(h) => h.resolve_pt(font).max(body_h_with_inset),
                    None => body_h_with_inset,
                };
                (total_w, total_h)
            }

            // P772f — `align` não tem tamanho intrínseco próprio: reporta o
            // tamanho do corpo (o wrapper só reposiciona dentro do espaço
            // disponível, não o redimensiona). Braço em falta colapsava a
            // 0 e fazia colunas auto de grid com células `align(...)`
            // colidirem na mesma posição x (ambas medidas a largura 0).
            Content::Align(e) => self.measure_content_constrained(&e.body, max_width),

            Content::Styled(body, styles) => {
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(styles);
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            Content::Strong(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain =
                    self.chain.push_styles(&Styles::from_iter([Style::strong()]));
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(&e.body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            Content::Emph(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(&Styles::from_iter([Style::emph()]));
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(&e.body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            _ => (0.0, 0.0),
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
/// Helper privado P159D + P159E + P159G — formata `BibEntry`
/// para render Bibliography. Concatenação condicional dos
/// fields opcionais quando presentes; backwards compat preserva
/// formato P159E quando todos os 6 fields P159G são `None`.
///
/// Ordem APA-like extendida (decisões diagnósticos P159D §10 +
/// P159E §8.2 + P159G §8.2):
/// `[key] author. title (Ed. editor) (series) journal vol. volume,`
/// `pp. pages. location: publisher (year). isbn:XXX url, doi:YYY [note].`
///
/// **P159G**: editor/series após title; location antes de
/// publisher; organization substitutivo a publisher quando
/// publisher ausente; isbn antes de url/doi; note ao final.
/// **P468** — corpo da entry sem prefixo `[key]`. Usado por `bibliography.rs`
/// para adicionar `[N]` numeração por ordem de citação.
pub(super) fn format_bib_entry_body(e: &crate::entities::bib_entry::BibEntry) -> String {
    let mut out = format!("{}. {}", e.author, e.title);
    format_bib_entry_body_fields(e, &mut out);
    out
}

fn format_bib_entry(e: &crate::entities::bib_entry::BibEntry) -> String {
    let mut out = format!("[{}] {}. {}", e.key, e.author, e.title);
    format_bib_entry_body_fields(e, &mut out);
    out
}

fn format_bib_entry_body_fields(
    e: &crate::entities::bib_entry::BibEntry,
    out: &mut String,
) {
    // P159G — editor/series após title.
    if let Some(ed) = &e.editor {
        out.push_str(&format!(" (Ed. {})", ed));
    }
    if let Some(se) = &e.series {
        out.push_str(&format!(" ({})", se));
    }
    // P159D — journal/volume/pages.
    if let Some(j) = &e.journal {
        out.push_str(&format!(" {}", j));
    }
    if let Some(v) = &e.volume {
        out.push_str(&format!(" vol. {}", v));
    }
    if let Some(p) = &e.pages {
        out.push_str(&format!(", pp. {}", p));
    }
    // P159G — location antes de publisher; organization substitutivo
    // a publisher quando publisher ausente.
    let pub_slot: Option<String> = match (&e.publisher, &e.organization) {
        (Some(pb), _) => Some(pb.clone()),
        (None, Some(o)) => Some(o.clone()),
        (None, None) => None,
    };
    match (&e.location, &pub_slot) {
        (Some(l), Some(pb)) => out.push_str(&format!(". {}: {}", l, pb)),
        (Some(l), None) => out.push_str(&format!(". {}", l)),
        (None, Some(pb)) => out.push_str(&format!(". {}", pb)),
        (None, None) => {}
    }
    out.push_str(&format!(" ({}).", e.year));
    // P159G — isbn antes de url/doi.
    if let Some(i) = &e.isbn {
        out.push_str(&format!(" isbn:{}", i));
    }
    // P159E — par natural url/doi após (year). per Opção C.
    match (&e.url, &e.doi) {
        (Some(u), Some(d)) => out.push_str(&format!(" {}, doi:{}.", u, d)),
        (Some(u), None) => out.push_str(&format!(" {}.", u)),
        (None, Some(d)) => out.push_str(&format!(" doi:{}.", d)),
        (None, None) => {
            // Fechar com `.` se isbn presente sem url/doi.
            if e.isbn.is_some() {
                out.push('.');
            }
        }
    }
    // P159G — note ao final.
    if let Some(n) = &e.note {
        out.push_str(&format!(" [{}]", n));
    }
}

pub fn layout(content: &Content) -> PagedDocument {
    // P190I (M6 fechado): `initial_state: CounterStateLegacy` parameter
    // ELIMINADO — struct eliminada. layout() corre
    // `introspect_with_introspector` internamente para obter
    // `TagIntrospector` populated. API breaking change comparada
    // com versões anteriores; callers externos adaptados.
    let intr = crate::engine::introspect::introspect_with_introspector(content);
    layout_with_introspector(content, intr)
}

/// Entry point P168 (M5 sub-passo 2): aceita `TagIntrospector` adicional
/// para que consumers como `references.rs::layout_ref` (figure-ref) possam
/// usar `query_by_label` em vez de `state.figure_label_numbers` legacy.
///
/// Caller típico:
/// ```ignore
/// let intr = introspect_with_introspector(&content);
/// let doc = layout_with_introspector(&content, intr);
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
}

/// **P712** — layout real e isolado de conteúdo para `measure()` (stdlib).
///
/// Constrói um `Layouter` isolado com `chain` = `chain` do chamador (o
/// tamanho medido depende do `#set text(size:)` activo, paridade com o
/// vanilla `measure()`, que usa `context.styles()`) e corre
/// `layout_sub_frame` (Passo 629 — o mesmo mecanismo já reutilizado por
/// `Content::Place`/`Content::Transform` e por `grid.rs` para medir
/// células) numa região efectivamente sem limites (`width:
/// f64::INFINITY`, `height: None`) — paridade com o vanilla
/// `Region::new(.., Abs::inf())` para a `measure()` sem `width`/`height`
/// explícitos. Devolve `(width, height)` em pontos, a partir dos itens
/// realmente emitidos pelo layout — não uma aproximação manual por tipo
/// de `Content` (ao contrário do antigo `measure_content` em
/// `layout/helpers.rs`, que continua a servir os seus próprios
/// consumers internos — `Content::Transform`/`Content::Place` —,
/// inalterado por este passo).
///
/// **Divergência mecânica documentada, não de língua (ADR-0107):**
/// `FixedMetrics` (monoespaçado, 0.6×size por codepoint) — L1 não tem
/// acesso a métricas de fonte reais (`FallbackFontMetrics` é L3). A
/// largura devolvida é real e proporcional ao conteúdo dado o motor de
/// layout usado, mas não byte-exacta ao vanilla (que usa shaping real
/// via `rustybuzz`).
pub fn measure_content_real(content: &Content, chain: &StyleChain) -> (f64, f64) {
    use comemo::Track;

    let font_size = chain.size();
    let intr = crate::entities::introspector::TagIntrospector::empty();
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, font_size, intr_dyn.track());
    layouter.chain = chain.clone();
    layouter.style = TextStyle::from(chain);

    // P772x — `layouter` é uma instância isolada e efémera (só para medir);
    // não há collector ambiente possível aqui, `_deco` é sempre vazio.
    let (height, items, _deco) = layouter.layout_sub_frame(
        content,
        sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: f64::INFINITY,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );

    let width = if items.is_empty() {
        0.0
    } else {
        let refs: Vec<&FrameItem> = items.iter().collect();
        FixedMetrics.line_content_right(&refs)
    };
    (width, height)
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
) -> PagedDocument {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use std::collections::HashMap;

    // ── Short-circuit: sem TOC, não há necessidade de fixpoint ──────────────
    // A condição correcta é "tem Content::Outline?", não
    // `headings_for_toc.is_empty()`. Um documento com títulos mas sem
    // `#outline()` não precisa do ciclo.
    //
    // P189B (M5): walk puro — flag obtida via Introspector
    // (`kind_index[Outline]` populado por `from_tags` P178) em vez de
    // `state.has_outline` (mutação removida em `introspect.rs:610`).
    // Field `CounterStateLegacy::has_outline` fica morto; cleanup em M6.
    use crate::entities::element_kind::ElementKind;
    let has_outline = introspector.kind_index.contains_key(&ElementKind::Outline);

    // P204C (M8): construir Tracked uma vez. introspector é binding
    // local (owned por valor desde signature) e outlive todos os
    // Layouters criados abaixo (single-pass ou fixpoint loop).
    use comemo::Track;
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &introspector;
    let intr_tracked = intr_dyn.track();

    // P418/P420/P429 — Pré-renderização CSL: descobre style/locale do primeiro
    // BibliographyElem e constrói cache para citações/bibliografia.
    // Só activa CSL quando style é explicitamente fornecido; caso contrário
    // preserva o fallback local `format_bib_entry` (compatibilidade P159A-G).
    // P429: style custom `.csl` já foi resolvido em eval time e injectado no
    // `BibStore` do TagIntrospector pelo pipeline; built-ins continuam
    // resolvidos aqui por nome quando não houver entrada na tabela lateral.
    let bib_style = find_first_bibliography_style(content, &introspector);
    let citation_order = introspector.citation_order();
    let mut layout_errors: Vec<SourceDiagnostic> = Vec::new();
    let bib_render_cache = bib_style.and_then(|style| {
        let result = if let Some(resolved) = style.resolved_style {
            crate::engine::layout::bib_csl::build_cache_with_style(
                introspector.bib_store.entries(),
                &resolved,
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else if let Some(name) = style.style.as_deref() {
            crate::engine::layout::bib_csl::build_cache(
                introspector.bib_store.entries(),
                Some(name),
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else {
            return None;
        };
        match result {
            Ok(cache) => Some(cache),
            Err(diagnostics) => {
                layout_errors.extend(diagnostics);
                None
            }
        }
    });

    if !has_outline {
        let mut l =
            Layouter::new(metrics.clone(), sizer.clone(), font_size, intr_tracked);
        l.bib_render_cache = bib_render_cache;
        l.layout_errors = layout_errors;
        // P204C (M8): introspector já fornecido a Layouter::new via
        // tracked. Mutações pós-construção (`l.introspector =
        // introspector`) eliminadas porque Tracked é borrow.
        // P190G (M6 categoria Labels & TOC eliminada) + restantes
        // limpezas mantidas — sem trabalho aqui.
        l.layout_content(content);
        return l.finish();
    }

    // ── Fixpoint: documentos com TOC ────────────────────────────────────────
    const MAX_ITERATIONS: usize = 5;

    // Mapa de páginas da iteração anterior — lido por `outline.rs`.
    // NÃO é o mesmo campo onde `references.rs` escreve durante o layout.
    // Separação leitura/escrita: Layouter lê de `known_page_numbers` e
    // escreve em `label_pages` (que começa vazio em cada iteração via Layouter::new()).
    let mut known_page_numbers: HashMap<Label, usize> = HashMap::new();
    // P488 — carry-forward páginas de figuras/tabelas entre iterações (LoF/LoT).
    let mut known_figure_page_numbers: Vec<usize> = Vec::new();
    let mut known_table_page_numbers: Vec<usize> = Vec::new();
    let mut final_doc: Option<PagedDocument> = None;

    for _ in 0..MAX_ITERATIONS {
        let mut l =
            Layouter::new(metrics.clone(), sizer.clone(), font_size, intr_tracked);
        l.bib_render_cache = bib_render_cache.clone();
        l.layout_errors = layout_errors.clone();

        // P204C (M8): assignment `l.introspector = introspector.clone()`
        // eliminado — Tracked partilhado entre iterações via construtor.
        // Tracked é Copy; cada iteração reusa o mesmo handle.
        // P190G (M6 categoria Labels & TOC eliminada): assignments
        // `resolved_labels`/`headings_for_toc` removidos — fields já
        // não existem. Layouter consumers (`references.rs:64`,
        // `outline.rs:38`) migrados para Introspector path puro.
        // P190E/P190G: assignment `numbering_active` removido — field
        // eliminado em P190G Caso 1.
        // P190B: assignments bib_* removidos — fields já não existem.

        // Injectar páginas da iteração anterior para leitura pelo outline.rs.
        // label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        // P190C (M6 categoria Page tracking): known_page_numbers movido
        // para LayouterRuntimeState.
        l.runtime.known_page_numbers = known_page_numbers.clone();
        // P488 — injectar páginas de figuras/tabelas da iteração anterior.
        l.runtime.known_figure_page_numbers = known_figure_page_numbers.clone();
        l.runtime.known_table_page_numbers = known_table_page_numbers.clone();

        l.layout_content(content);
        let doc = l.finish();

        // Convergência: mapa de páginas gerado == mapa da iteração anterior?
        // P488: estendido para incluir páginas de figuras/tabelas (LoF/LoT).
        if doc.extracted_label_pages == known_page_numbers
            && doc.extracted_figure_page_numbers == known_figure_page_numbers
            && doc.extracted_table_page_numbers == known_table_page_numbers
        {
            return doc;
        }

        // Actualizar para a próxima iteração.
        known_page_numbers = doc.extracted_label_pages.clone();
        // P488 — actualizar carry-forward de figuras/tabelas.
        known_figure_page_numbers = doc.extracted_figure_page_numbers.clone();
        known_table_page_numbers = doc.extracted_table_page_numbers.clone();
        final_doc = Some(doc);
    }

    // Limite atingido sem convergência (DEBT-17: caso patológico).
    // Retornar o documento da última iteração — melhor esforço.
    // Sem `log::` em L1 — não existe ADR que o autorize.
    final_doc.expect("layout: deve produzir pelo menos um documento")
}

// ── Helpers P418 ───────────────────────────────────────────────────────────

/// Dados do primeiro `Content::Bibliography` encontrado no documento.
///
/// P429: `resolved_style` vem do `BibStore` do `TagIntrospector` (tabela
/// lateral indexada pela chave do `BibliographyElem`), não do próprio elemento.
struct FirstBibliographyStyle {
    resolved_style: Option<Arc<IndependentStyle>>,
    style: Option<EcoString>,
    locale: Option<EcoString>,
}

/// Procura o primeiro `Content::Bibliography` no documento (DFS simples) e
/// devolve os dados necessários à pré-renderização CSL. Usado para construir o
/// cache antes do layout principal, permitindo que `cite` antes do
/// `bibliography` use o mesmo style.
///
/// P429: o style resolvido é consultado no `BibStore` via
/// `BibliographyElem::style_key()`.
fn find_first_bibliography_style(
    content: &Content,
    introspector: &crate::entities::introspector::TagIntrospector,
) -> Option<FirstBibliographyStyle> {
    fn walk(
        c: &Content,
        introspector: &crate::entities::introspector::TagIntrospector,
    ) -> Option<FirstBibliographyStyle> {
        match c {
            Content::Bibliography(e) => Some(FirstBibliographyStyle {
                resolved_style: introspector
                    .bib_store
                    .style_for_key(e.style_key())
                    .cloned(),
                style: e.style.clone(),
                locale: e.locale.clone(),
            }),
            Content::Sequence(seq) => seq.iter().find_map(|c| walk(c, introspector)),
            Content::Styled(body, _) => walk(body, introspector),
            Content::Block(e) => walk(&e.body, introspector),
            Content::Boxed(e) => walk(&e.body, introspector),
            Content::Pad(e) => walk(&e.body, introspector),
            Content::Align(e) => walk(&e.body, introspector),
            Content::Hide(e) => walk(&e.body, introspector),
            Content::Figure(e) => walk(&e.body, introspector)
                .or_else(|| e.caption.as_ref().and_then(|c| walk(c, introspector))),
            Content::Table(e) => e
                .caption
                .as_ref()
                .and_then(|c| walk(c, introspector))
                .or_else(|| e.children.iter().find_map(|c| walk(c, introspector))),
            Content::Grid(e) => e.cells.iter().find_map(|c| walk(c, introspector)),
            Content::Stack(e) => e.children.iter().find_map(|c| walk(c, introspector)),
            Content::ListItem(e) => walk(&e.body, introspector),
            Content::EnumItem(e) => walk(&e.body, introspector),
            Content::TermItem(e) => {
                walk(&e.term, introspector).or_else(|| walk(&e.description, introspector))
            }
            Content::Footnote(e) => walk(&e.body, introspector),
            Content::Overline(e) => walk(&e.body, introspector),
            Content::Strike(e) => walk(&e.body, introspector),
            Content::Underline(e) => walk(&e.body, introspector),
            Content::Strong(e) => walk(&e.body, introspector),
            Content::Emph(e) => walk(&e.body, introspector),
            Content::Link(e) => walk(&e.body, introspector),
            Content::SmallCaps { body } => walk(body, introspector),
            Content::Heading(e) => walk(&e.body, introspector),
            Content::Title(e) => walk(&e.body, introspector),
            _ => None,
        }
    }
    walk(content, introspector)
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests;
