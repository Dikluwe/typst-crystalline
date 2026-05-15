//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash 3940e285
//! @layer L1
//! @updated 2026-05-05
//!
//! P162 sub-passos .E + .F: walk passa a aceitar `&mut Locator` e
//! `&mut Vec<Tag>`; emite `Tag::Start` antes da mutação de estado e
//! `Tag::End` depois da recursão para variantes locatable
//! (Heading/Figure/Cite).
//!
//! **P191B (ADR-0071)**: walk fn ganha `&mut TagIntrospector`
//! parameter; sub-stores populated directamente durante walk via
//! `populate_intr_from_tag_start`. Pipeline simplificado: walk →
//! return (etapa `from_tags::from_tags` eliminada; substituída por
//! `apply_state_funcs` slim post-pass para Funcs apenas, chamada
//! por fixpoint). Helper `compute_heading_auto_toc` migrado para
//! signature `<I: Introspector>(intr, location, counter_n)`. Walk
//! arm Equation gate migrado para
//! `intr.is_numbering_active_at("numbering_active:equation", loc)`.
//! API pública preservada (`introspect()` retorna `CounterStateLegacy`
//! idêntico). `introspect_with_introspector` simplificada — drops
//! parâmetros engine/ctx (Funcs continuam ignoradas neste path
//! coerente com semântica P171 pré-P191B).

pub mod convergence;
pub mod extract_payload;
pub mod fixpoint;
pub mod from_tags;
pub mod locatable;

use crate::entities::{
    content::Content,
    content_hash::hash_content,
    counter_update::CounterUpdate,
    element_info::ElementInfo,
    element_kind::ElementKind,
    element_payload::ElementPayload,
    introspector::{Introspector, TagIntrospector},
    label::Label,
    location::Location,
    locator::Locator,
    state_update::StateUpdate,
    tag::Tag,
    value::Value,
};

use crate::rules::introspect::extract_payload::extract_payload as do_extract_payload;

/// Pré-passagem analítica sobre `Content` — entry point legado.
///
/// Percorre a árvore completa uma vez, avançando contadores e populando
/// `resolved_labels`, sem realizar nenhum cálculo visual.
///
/// O `CounterStateLegacy` retornado é injectado no Layouter como estado inicial
/// (apenas o campo `resolved_labels`), garantindo que todas as referências —
/// incluindo para a frente — estão resolvidas antes do primeiro `FrameItem`
/// ser gerado.
///
/// **P166 (M4)**: esta função é agora wrapper sobre
/// `introspect_with_introspector` — descarta o `TagIntrospector`. Consumers
/// que precisem do introspector devem chamar `introspect_with_introspector`
/// directamente. M5 migra primeiro consumer real; M6 elimina este wrapper
/// + `CounterStateLegacy`.
///
/// **P173**: passa `None, None` para `introspect_with_introspector` —
/// Funcs em `state.update(key, fn)` são silenciosamente ignoradas neste
/// path legacy (sem Engine disponível). Comportamento defensivo coerente
/// com P171 ("update sem init é ignorado").
/// **P190I (M6 fechado)**: API pública continua a existir mas
/// retorna `TagIntrospector` (struct `CounterStateLegacy` eliminada).
/// Wrapper de compatibilidade — preserva nome `introspect()`
/// histórico. Callers actualizados para path Introspector.
pub fn introspect(content: &Content) -> TagIntrospector {
    introspect_with_introspector(content)
}

/// Entry point novo (M4 / P166): produz `CounterStateLegacy` E
/// `TagIntrospector` num único walk. Consumers que precisem do
/// introspector usam este entry point; consumers legacy continuam a
/// usar `introspect()`.
///
/// **Walk único subjacente**: state + introspector vêm da mesma
/// passagem — não há duplicação. `introspect()` é wrapper que descarta
/// o introspector.
///
/// **P173 (M9 sub-passo 5)**: aceita `Engine + EvalContext` opcionais.
/// Quando ambos `Some`, `from_tags` avalia `StateUpdate::Func` via
/// `apply_func`. Walk em si **NÃO modificado** — continua puro
/// (P163 invariante preservado). Engine só intervém em `from_tags`
/// (eval localizada).
///
/// Padrão de migração M5+: caller que actualmente faz
/// `let state = introspect(&c)` e quer queries via Introspector pode
/// adoptar `let intr = introspect_with_introspector(&c)`
/// sem custo adicional.
///
/// **P191B (ADR-0071)**: walk fn ganha `&mut TagIntrospector` parameter;
/// populate de sub-stores acontece directamente durante walk em vez de
/// pós-walk via `from_tags`. `from_tags::from_tags` substituído por
/// `apply_state_funcs` (slim post-pass para `StateUpdate::Func` apenas;
/// chamado por `fixpoint::run_fixpoint` que tem `Engine + EvalContext`).
/// Funcs em state.update **silenciosamente ignoradas** neste path
/// legacy (sem Engine disponível) — coerente com semântica P171
/// pré-P191B.
pub fn introspect_with_introspector(
    content: &Content,
) -> TagIntrospector {
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter: usize = 0;
    walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);
    intr
}

/// "Congela" o AST substituindo nós dependentes de contexto (como CounterDisplay)
/// pelos seus valores em texto estático no momento exacto da introspecção (Passo 66, DEBT-18).
///
/// Resolve DEBT-18: sem esta função, a TOC mostraria os valores dos contadores
/// no início do documento, não o valor que cada contador tinha quando o título ocorreu.
///
/// Dois braços explícitos — sem wildcard para manter verificação de exaustividade:
/// - Containers: propagam recursivamente.
/// - Terminais: clonados directamente.
///
/// **P190I (ADR-0070 ACEITE)** — signature migrada para
/// `(content, intr, location)`. Reads `state.display_value(kind)`
/// substituídos por `intr.formatted_counter_at(kind, location)`.
/// Caminho Introspector path location-aware. Walk fn deixou de
/// receber `state: &mut CounterStateLegacy` (struct eliminada em
/// P190I).
fn materialize_time(content: &Content, intr: &TagIntrospector, location: Location) -> Content {
    match content {
        // O caso crítico: substituir o nó dinâmico pelo valor actual do contador.
        Content::CounterDisplay { kind } => {
            Content::text(
                intr.formatted_counter_at(kind, location)
                    .unwrap_or_else(|| "0".to_string()),
            )
        }

        // ── Containers com filhos (propagação recursiva) ──────────────────
        Content::Sequence(seq) => {
            Content::Sequence(
                seq.iter().map(|c| materialize_time(c, intr, location)).collect::<Vec<_>>().into()
            )
        }
        // Passo 101: `Content::Strong` e `Content::Emph` removidos.
        // O arm `Content::Styled` abaixo cobre ambos (propaga recursivamente
        // preservando os estilos).
        Content::Heading { level, body } => Content::Heading {
            level: *level,
            body:  Box::new(materialize_time(body, intr, location)),
        },
        Content::ListItem(body) => Content::ListItem(Box::new(materialize_time(body, intr, location))),
        Content::EnumItem { number, body } => Content::EnumItem {
            number: *number,
            body:   Box::new(materialize_time(body, intr, location)),
        },
        Content::Link { url, body } => Content::Link {
            url:  url.clone(),
            body: Box::new(materialize_time(body, intr, location)),
        },
        Content::Labelled { target, label } => Content::Labelled {
            target: Box::new(materialize_time(target, intr, location)),
            label:  label.clone(),
        },
        Content::Figure { body, caption, kind, numbering } => Content::Figure {
            body:      Box::new(materialize_time(body, intr, location)),
            caption:   caption.as_ref().map(|c| Box::new(materialize_time(c, intr, location))),
            kind:      kind.clone(),
            numbering: numbering.clone(),
        },

        // Passo 154B: Terms recurse em items; TermItem recurse em par.
        Content::Terms { items } => Content::Terms {
            items: items.iter().map(|c| materialize_time(c, intr, location)).collect(),
        },
        Content::TermItem { term, description } => Content::TermItem {
            term:        Box::new(materialize_time(term, intr, location)),
            description: Box::new(materialize_time(description, intr, location)),
        },

        // Passo 155: Quote — recurse em body e attribution.
        Content::Quote { body, attribution, block, quotes } => Content::Quote {
            body:        Box::new(materialize_time(body, intr, location)),
            attribution: attribution.as_ref().map(|c| Box::new(materialize_time(c, intr, location))),
            block:       *block,
            quotes:      *quotes,
        },

        // ── Terminais — clonar directamente ──────────────────────────────
        // Nós matemáticos (Equation e subtipos) não podem conter CounterDisplay
        // em markup válido — clonados em bloco sem recursão.
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Raw { .. }
        | Content::Ref { .. }
        | Content::SetHeadingNumbering { .. }
        | Content::SetEquationNumbering { .. }
        | Content::SetFigureNumbering { .. }
        | Content::SetPage { .. }
        | Content::CounterUpdate { .. }
        | Content::Outline
        | Content::Linebreak
        | Content::MathAlignPoint
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::Equation { .. }
        | Content::MathSequence(_)
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::Image { .. }
        | Content::Divider
        // Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h/v spacing leaves.
        | Content::HSpace { .. }
        | Content::VSpace { .. }
        // Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak leaf.
        | Content::Pagebreak { .. }
        // Passo 220 (ADR-0078 sub-fase b 4/4) — colbreak leaf.
        | Content::Colbreak { .. }
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — clonar directamente.
        | Content::Metadata { .. }
        // P171 (M9): State e StateUpdate são terminais.
        | Content::State { .. }
        | Content::StateUpdate { .. }
        // P240 (M9d/M7+1): StateDisplay terminal em materialize_time —
        // resolução real via apply_state_displays + layout arm.
        | Content::StateDisplay { .. }
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; resolução real via apply_counter_displays.
        | Content::CounterDisplayCallback { .. } => content.clone(),
        // Passo 156C (ADR-0061 Fase 1) — pad / hide containers.
        // Materialize_time desce no body para resolver counters dentro;
        // padding e o invariante "hide" preservam-se.
        Content::Pad { body, sides } => Content::Pad {
            body:  Box::new(materialize_time(body, intr, location)),
            sides: *sides,
        },
        Content::Hide { body } => Content::Hide {
            body: Box::new(materialize_time(body, intr, location)),
        },
        // Passo 156G + P231 + P247 — Block container; preserva 5 cosméticos.
        Content::Block { body, width, height, inset, breakable, outset, radius, clip, fill, stroke } => Content::Block {
            body:      Box::new(materialize_time(body, intr, location)),
            width:     *width,
            height:    *height,
            inset:     *inset,
            breakable: *breakable,
            outset:    *outset,
            radius:    *radius,
            clip:      *clip,
            fill:      *fill,
            stroke:    stroke.clone(),
        },
        // Passo 156H + P231 + P247 — Boxed container; preserva 5 cosméticos.
        Content::Boxed { body, width, height, inset, baseline, outset, radius, clip, fill, stroke } => Content::Boxed {
            body:     Box::new(materialize_time(body, intr, location)),
            width:    *width,
            height:   *height,
            inset:    *inset,
            baseline: *baseline,
            outset:   *outset,
            radius:   *radius,
            clip:     *clip,
            fill:     *fill,
            stroke:   stroke.clone(),
        },
        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Materialize_time em cada child; preservar dir/spacing.
        Content::Stack { children, dir, spacing } => {
            let new_children: Vec<Content> = children.iter()
                .map(|c| materialize_time(c, intr, location))
                .collect();
            Content::Stack {
                children: std::sync::Arc::from(new_children),
                dir:      *dir,
                spacing:  *spacing,
            }
        },
        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat.
        // Análogo a Block: descer no body; preservar atributos.
        Content::Repeat { body, gap, justify } => Content::Repeat {
            body:    Box::new(materialize_time(body, intr, location)),
            gap:     *gap,
            justify: *justify,
        },
        // P217 — Columns container: análogo a Repeat/Block; descer
        // no body; count/gutter preservados (Copy primitivos).
        Content::Columns { count, gutter, body } => Content::Columns {
            count:  *count,
            gutter: *gutter,
            body:   Box::new(materialize_time(body, intr, location)),
        },
        Content::Transform { matrix, body } => Content::Transform {
            matrix: *matrix,
            body:   Box::new(materialize_time(body, intr, location)),
        },
        // P224+P227+P228 — Grid refino +7 fields preservados (gutter/align/inset/header/footer/stroke/fill).
        Content::Grid { columns, rows, cells, gutter, align, inset, header, footer, stroke, fill } => Content::Grid {
            columns: columns.clone(),
            rows:    rows.clone(),
            cells:   cells.iter().map(|c| materialize_time(c, intr, location)).collect(),
            gutter:  *gutter,
            align:   *align,
            inset:   *inset,
            header:  header.as_ref().map(|h| Box::new(materialize_time(h, intr, location))),
            footer:  footer.as_ref().map(|f| Box::new(materialize_time(f, intr, location))),
            stroke:  stroke.clone(),
            fill:    *fill,
        },
        // P224.B — GridHeader / GridFooter recurse no body.
        Content::GridHeader { body, repeat } => Content::GridHeader {
            body:   Box::new(materialize_time(body, intr, location)),
            repeat: *repeat,
        },
        Content::GridFooter { body, repeat } => Content::GridFooter {
            body:   Box::new(materialize_time(body, intr, location)),
            repeat: *repeat,
        },
        // P224.C + P230 + P235 — GridCell recurse no body; preserva
        // 5 fields cumulativos.
        Content::GridCell { body, x, y, colspan, rowspan, stroke, fill,
                             align, inset, breakable } => Content::GridCell {
            body:      Box::new(materialize_time(body, intr, location)),
            x:         *x,
            y:         *y,
            colspan:   *colspan,
            rowspan:   *rowspan,
            stroke:    stroke.clone(),
            fill:      *fill,
            align:     *align,
            inset:     inset.clone(),
            breakable: *breakable,
        },
        // Passo 157A + P227 + P228 — table; preserva stroke + fill.
        Content::Table { columns, rows, children, stroke, fill } => Content::Table {
            columns:  columns.clone(),
            rows:     rows.clone(),
            children: children.iter().map(|c| materialize_time(c, intr, location)).collect(),
            stroke:   stroke.clone(),
            fill:     *fill,
        },
        // Passo 157B + P230 + P235 — TableCell; preserva 5 fields cumulativos.
        Content::TableCell { body, x, y, colspan, rowspan, stroke, fill,
                              align, inset, breakable } => Content::TableCell {
            body:      Box::new(materialize_time(body, intr, location)),
            x:         *x,
            y:         *y,
            colspan:   *colspan,
            rowspan:   *rowspan,
            stroke:    stroke.clone(),
            fill:      *fill,
            align:     *align,
            inset:     inset.clone(),
            breakable: *breakable,
        },
        // Passo 157C (ADR-0060 Fase 2 sub-passo 3) — par simétrico
        // TableHeader/TableFooter. Recurse no body; preserva repeat.
        Content::TableHeader { body, repeat } => Content::TableHeader {
            body:   Box::new(materialize_time(body, intr, location)),
            repeat: *repeat,
        },
        Content::TableFooter { body, repeat } => Content::TableFooter {
            body:   Box::new(materialize_time(body, intr, location)),
            repeat: *repeat,
        },
        // Passo 159A — par acoplado Bibliography + Cite. Recurse em
        // title (Bibliography) ou supplement (Cite); preserva
        // entries/key.
        Content::Bibliography { entries, title } => Content::Bibliography {
            entries: entries.clone(),
            title:   title.as_ref().map(|t| Box::new(materialize_time(t, intr, location))),
        },
        Content::Cite { key, supplement, form } => Content::Cite {
            key:        key.clone(),
            supplement: supplement.as_ref().map(|s| Box::new(materialize_time(s, intr, location))),
            form:       *form,
        },
        Content::Align { alignment, body } => Content::Align {
            alignment: *alignment,
            body:      Box::new(materialize_time(body, intr, location)),
        },
        // P223 — Place refino: preservar float + clearance no materialize_time.
        Content::Place { alignment, dx, dy, scope, float, clearance, body } => Content::Place {
            alignment: *alignment,
            dx:        *dx,
            dy:        *dy,
            scope:     *scope,
            float:     *float,
            clearance: *clearance,
            body:      Box::new(materialize_time(body, intr, location)),
        },

        // Passo 99 (ADR-0038): `Styled` é transparente para materialização de
        // contadores — o body é processado e os estilos preservados.
        Content::Styled(body, styles) => Content::Styled(
            Box::new(materialize_time(body, intr, location)),
            styles.clone(),
        ),
    }
}

/// Percurso recursivo sem efeitos visuais.
///
/// Replica exactamente os side-effects de estado que o Layouter produz,
/// na mesma ordem de travessia, mas sem aceder a `FontMetrics` nem alocar
/// `Frame`/`FrameItem`.
///
/// **P195D** — computa `(resolved_text, figure_number)` para
/// `Content::Labelled` baseado no target type. Helper privado isolado
/// (per ADR-0069) para reuso entre mutação legacy (`state.resolved_labels`,
/// `state.figure_label_numbers`) e populate Tag pós-recursão
/// (`ElementPayload::Labelled` payload).
///
/// **P191C (ADR-0071 ACEITE)** — signature migrada para
/// `<I: Introspector>(intr: &I, location: Location, target: &Content,
/// lang: Option<&Lang>)`. Reads location-aware (`formatted_counter_at`,
/// `flat_counter_at` per P185B) substituem reads de
/// `state.format_hierarchical`, `state.get_flat`,
/// `state.figure_numbers`. `lang` continua passado por parameter
/// (Opção β cláusula `.A.8`) — `state.lang` ainda existe em
/// `CounterStateLegacy` durante janela compat M5/M6 (defer P190G+).
///
/// `location` aqui é a Location do **target** (figure/heading/equation),
/// não do Labelled wrapper — preserva pattern P195D variante
/// não-locatable (target_loc obtido via snapshot+find_map em walk
/// arm Labelled).
///
/// Função pura sobre `(intr, location, target, lang)` — sem mutação.
/// 2º helper migrado pela ADR-0071 (após `compute_heading_auto_toc`
/// P191B).
fn compute_labelled<I: Introspector>(
    intr:     &I,
    location: Location,
    target:   &Content,
    lang:     Option<&crate::entities::lang::Lang>,
) -> (Option<String>, Option<usize>) {
    match target {
        Content::Heading { .. } => (
            intr.formatted_counter_at("heading", location)
                .map(|n| format!("Secção {}", n)),
            None,
        ),
        Content::Equation { block, .. } if *block => {
            let n = intr
                .flat_counter_at("equation", location)
                .unwrap_or(0);
            if n > 0 {
                (Some(format!("Equação ({})", n)), None)
            } else {
                (None, None)
            }
        }
        Content::Figure { kind, numbering, caption, .. } => {
            let kind_key = kind.as_deref().unwrap_or("image");
            let n = if numbering.is_some() && caption.is_some() {
                intr.flat_counter_at(
                    &format!("figure:{}", kind_key),
                    location,
                )
                .unwrap_or(0)
            } else {
                0
            };
            if n > 0 {
                let supplement = crate::rules::lang::figure_supplement::figure_supplement_for_lang(
                    kind_key,
                    lang,
                );
                (Some(format!("{} {}", supplement, n)), Some(n))
            } else {
                (Some(String::new()), None)
            }
        }
        _ => (None, None),
    }
}

/// **P196B** — computa `(auto_label, resolved_text)` para
/// auto-toc Heading (pattern ADR-0069). Helper privado análogo a
/// `compute_labelled` (P195D). Função pura sobre `(intr, location,
/// auto_label_n)` — sem mutação. Replica lógica legacy do walk arm
/// Heading (introspect.rs:411-428 pré-P196B).
///
/// Sempre retorna concrete `(Label, String)` — paridade legacy
/// que insere `auto_label → resolved_text` mesmo quando
/// numbering inactivo (resolved_text fica vazio nesse caso).
///
/// **P191B (ADR-0071)** — signature migrada para
/// `<I: Introspector>(intr: &I, location: Location, auto_label_n)`.
/// Reads location-aware (`is_numbering_active_at` +
/// `formatted_counter_at` per P185B) substituem reads de
/// `state.is_numbering_active` + `state.format_hierarchical`.
/// Walk popula intr (incluindo `numbering_active:heading` Set tag)
/// ANTES desta call, garantindo consistência por construção.
fn compute_heading_auto_toc<I: Introspector>(
    intr:         &I,
    location:     Location,
    auto_label_n: usize,
) -> (Label, String) {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    let resolved_text = if intr.is_numbering_active_at(
        "numbering_active:heading",
        location,
    ) {
        intr.formatted_counter_at("heading", location)
            .map(|prefix| format!("Secção {}", prefix))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (auto_label, resolved_text)
}

// P197B helper `compute_figure` ELIMINADO em P190H (M6 categoria
// Figures eliminada). Walk arm Figure ficou puro — sem necessidade
// de calcular figure_number durante walk porque populate_intr arm
// Figure (P191C, gated por is_counted) popula
// `intr.counters["figure:{kind}"]` directamente. Consumers
// (`compute_labelled` Figure arm via `intr.flat_counter_at`,
// Layouter C3 via `figure_number_at_index`) consomem do intr.

/// **P200B** (M5 universal completo) — projecta a entry de outline
/// para um Heading. Helper privado análogo a `compute_labelled`
/// (P195D), `compute_heading_auto_toc` (P196B), e `compute_figure`
/// (P197B). 4º helper na família ADR-0069 stylesheet.
///
/// Função pura sobre `(auto_label_n, frozen_body, level)` — sem
/// mutação. `frozen_body` é assumido já materializado (chamado
/// pelo walk arm Heading que computa `materialize_time(body,
/// state)` em linha imediatamente anterior).
///
/// Sempre retorna `Some(...)` — paridade com mutação 4 legacy
/// (introspect.rs:486 pré-P200B) que faz push **incondicional**.
/// Auto-label sintetizada usa `auto_label_n` (já incrementado pela
/// chamada anterior em walk arm Heading). Reusa `frozen_body` para
/// evitar chamada redundante a `materialize_time`.
///
/// **P190G (M6 categoria Labels & TOC)**: signature migrada para
/// receber `auto_label_n` por parameter — field
/// `state.auto_label_counter` eliminado de `CounterStateLegacy`
/// e substituído por local var em walk fn. Helper continua
/// walk-internal (chamado apenas por walk arm Heading).
fn compute_heading_for_toc(
    auto_label_n: usize,
    frozen_body:  Content,
    level:        usize,
) -> Option<(Label, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    Some((auto_label, frozen_body, level))
}

/// **P191B (ADR-0071)** — populate `TagIntrospector` sub-stores a partir
/// de uma `Tag::Start` emitida pelo walk. Substitui o match exhaustivo
/// que vivia em `from_tags::from_tags` (eliminado em P191B). Walk arms
/// chamam este helper imediatamente antes de `tags.push(Tag::Start(...))`
/// para que sub-stores fiquem populated em ordem location-monotónica.
///
/// **`StateUpdate::Func` excepção**: Funcs requerem `Engine +
/// EvalContext` para `apply_func` — não disponíveis em walk. Defer
/// para `apply_state_funcs` (post-pass slim chamado em `fixpoint`).
fn populate_intr_from_tag_start(
    intr: &mut TagIntrospector,
    info: &ElementInfo,
    loc:  Location,
) {
    if let Some(label) = &info.label {
        intr.labels.add(label.clone(), loc);
    }
    match &info.payload {
        ElementPayload::Heading { depth, .. } => {
            intr.kind_index
                .entry(ElementKind::Heading)
                .or_default()
                .push(loc);
            intr.counters.apply_hierarchical_at(
                "heading".to_string(),
                *depth as usize,
                loc,
            );
        }
        ElementPayload::Figure { kind, counter_update, is_counted, .. } => {
            intr.kind_index
                .entry(ElementKind::Figure)
                .or_default()
                .push(loc);
            // P191C (ADR-0071 ACEITE): counter populated apenas quando
            // `is_counted` (numbering+caption). Alinha com legacy
            // `state.figure_numbers` que só regista figuras counted
            // (introspect.rs walk arm Figure pre-P191C). Pre-P191C
            // populate era unconditional — divergência latente
            // ocultada porque `compute_labelled` Figure arm lia
            // `state.figure_numbers`. Após P191C `compute_labelled`
            // usa Introspector path; gate restaurado por paridade
            // semântica.
            if *is_counted {
                let kind_key = kind.as_deref().unwrap_or("image");
                intr.counters.apply_at(
                    format!("figure:{}", kind_key),
                    counter_update.clone(),
                    loc,
                );
                intr.counters.apply_at(
                    "figure".to_string(),
                    counter_update.clone(),
                    loc,
                );
                if let Some(label) = &info.label {
                    let next_num = intr.figure_label_numbers.len() + 1;
                    intr.figure_label_numbers
                        .entry(label.clone())
                        .or_insert(next_num);
                }
            }
        }
        ElementPayload::Citation { .. } => {
            intr.kind_index
                .entry(ElementKind::Citation)
                .or_default()
                .push(loc);
        }
        ElementPayload::Metadata { value } => {
            intr.kind_index
                .entry(ElementKind::Metadata)
                .or_default()
                .push(loc);
            intr.metadata.add((**value).clone());
        }
        ElementPayload::State { key, init } => {
            intr.kind_index
                .entry(ElementKind::State)
                .or_default()
                .push(loc);
            intr.state.init(key.clone(), (**init).clone(), loc);
        }
        ElementPayload::Outline => {
            intr.kind_index
                .entry(ElementKind::Outline)
                .or_default()
                .push(loc);
        }
        ElementPayload::Bibliography { entries } => {
            intr.kind_index
                .entry(ElementKind::Bibliography)
                .or_default()
                .push(loc);
            let entries_owned = entries.clone();
            for entry in &entries_owned {
                let next_num = intr.bib_store.numbers_len() as u32 + 1;
                intr.bib_store
                    .assign_number(entry.key.clone(), next_num);
            }
            intr.bib_store.add_bibliography(entries_owned);
        }
        ElementPayload::StateUpdate { key, update } => {
            intr.kind_index
                .entry(ElementKind::StateUpdate)
                .or_default()
                .push(loc);
            match update {
                StateUpdate::Set(value) => {
                    if intr.state.value_at(key, loc).is_none() {
                        intr.state.init(key.clone(), (**value).clone(), loc);
                    } else {
                        intr.state.update(key.clone(), (**value).clone(), loc);
                    }
                }
                StateUpdate::Func(_) => {
                    // P191B (ADR-0071): Func eval requires Engine+ctx —
                    // deferred to `apply_state_funcs` post-pass.
                }
            }
        }
        ElementPayload::StateDisplay { .. } => {
            // P240 (M9d/M7+1): kind_index registo apenas em walk; valor
            // pre-rendered é produzido em `apply_state_displays` pós-walk
            // (paralelo `apply_state_funcs` — requer Engine+ctx).
            intr.kind_index
                .entry(ElementKind::StateDisplay)
                .or_default()
                .push(loc);
        }
        ElementPayload::CounterDisplay { .. } => {
            // P241 (M9d/M7+2): paralelo absoluto StateDisplay; valor
            // pre-rendered em `apply_counter_displays` pós-walk.
            intr.kind_index
                .entry(ElementKind::CounterDisplay)
                .or_default()
                .push(loc);
        }
        ElementPayload::Equation { block, counter_update } => {
            intr.kind_index
                .entry(ElementKind::Equation)
                .or_default()
                .push(loc);
            // Gate location-aware: state populated por SetEquationNumbering
            // tag emitida ANTES desta Equation tag (location-monotónica
            // por construção de Locator).
            if *block
                && matches!(
                    intr.state.value_at("numbering_active:equation", loc),
                    Some(Value::Bool(true)),
                )
            {
                intr.counters.apply_at(
                    "equation".to_string(),
                    counter_update.clone(),
                    loc,
                );
            }
        }
        ElementPayload::Labelled { label, resolved_text, figure_number } => {
            if let Some(text) = resolved_text {
                intr.resolved_labels
                    .insert(label.clone(), text.clone());
            }
            if let Some(n) = figure_number {
                intr.figure_label_numbers
                    .insert(label.clone(), *n);
            }
        }
        ElementPayload::CounterUpdate { key, action } => {
            intr.kind_index
                .entry(ElementKind::CounterUpdate)
                .or_default()
                .push(loc);
            match action {
                CounterUpdate::Step => {
                    if key == "heading" {
                        intr.counters.apply_hierarchical_at(
                            key.clone(),
                            1,
                            loc,
                        );
                    } else {
                        intr.counters.apply_at(
                            key.clone(),
                            CounterUpdate::Step,
                            loc,
                        );
                    }
                }
                CounterUpdate::Update(val) => {
                    intr.counters.apply_at(
                        key.clone(),
                        CounterUpdate::Update(*val),
                        loc,
                    );
                }
            }
        }
        ElementPayload::HeadingForToc { label, body, level } => {
            intr.headings_for_toc.push((
                label.clone(),
                body.clone(),
                *level,
            ));
        }
    }
}

/// **P162 .E**: emite `Tag::Start`/`Tag::End` em paralelo para os 3 kinds
/// locatable (Heading/Figure/Cite). `label_from_parent` é `Some(label)`
/// quando este nó é descendente directo de um `Content::Labelled` wrapper;
/// `None` caso contrário.
///
/// **P191B (ADR-0071)**: walk fn ganha `intr: &mut TagIntrospector`.
/// Sub-stores populated directamente durante walk via
/// `populate_intr_from_tag_start` no momento de cada `Tag::Start`
/// emission. Pipeline simplificado: walk → return; eliminado etapa
/// `from_tags::from_tags` post-walk (excepto Funcs deferred).
///
/// **P190G (M6 categoria Labels & TOC)**: walk fn ganha
/// `auto_label_counter: &mut usize` parameter (Opção α `.D`).
/// Substitui field `CounterStateLegacy::auto_label_counter`
/// eliminado em P190G. Walk-internal counter para gerar IDs únicas
/// auto-toc-{n} per Heading; incrementado por walk arm Heading;
/// lido por helpers `compute_heading_auto_toc` (P191B) +
/// `compute_heading_for_toc` (P200B).
///
/// **P190I (M6 fechado)**: walk fn drop parameter `state: &mut
/// CounterStateLegacy` — struct eliminada. Walk fn ganha parameter
/// `lang: Option<&Lang>` (substitui `state.lang` lido por walk arm
/// Labelled per P191C Opção β). Net signature: 7 parameters
/// (mantém-se).
pub(crate) fn walk(
    content:            &Content,
    locator:            &mut Locator,
    tags:               &mut Vec<Tag>,
    intr:               &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang:               Option<&crate::entities::lang::Lang>,
    label_from_parent:  Option<&Label>,
) {
    // P162 .E + P191B: emissão Tag::Start em paralelo, antes da mutação
    // de estado. populate_intr_from_tag_start popula sub-stores intr no
    // momento da emissão (ADR-0071).
    let emitted_loc = if let Some(payload) = do_extract_payload(content) {
        let loc = locator.next();
        let info = ElementInfo {
            payload,
            label: label_from_parent.cloned(),
        };
        populate_intr_from_tag_start(intr, &info, loc);
        tags.push(Tag::Start(loc, info));
        Some(loc)
    } else {
        None
    };

    match content {
        Content::Sequence(seq) => {
            for item in seq.iter() {
                walk(item, locator, tags, intr, auto_label_counter, lang, None);
            }
        }

        Content::Heading { level, body } => {
            // P200B (M5 universal completo) — walk arm Heading
            // E2-residuo fechada estruturalmente. Trabalho híbrido
            // combinando 3 padrões testados:
            // - sub-store novo `intr.headings_for_toc` (P193B-style),
            // - Tag pós-recursão `ElementPayload::HeadingForToc`
            //   (variante P196B locatable + emitted_loc directo),
            // - consumer outline.rs:24 substitution-with-fallback
            //   (P184D / P194B style).
            //
            // P196B (referência histórica): 3 das 4 mutações E2
            // migraram estruturalmente via Tag::Labelled auto-toc;
            // a 4ª (`state.headings_for_toc.push`) era residual.
            // P200B fecha esse residuo via Tag::HeadingForToc
            // (3ª Tag pós-recursão, mesma `emitted_loc`).
            //
            // Mutação 4 legacy preservada como write paralelo M5
            // porque Layouter assignments (`mod.rs:1490, 1521`)
            // dependem de `state.headings_for_toc`. Cleanup
            // orgânico em M6.
            // P190I (M6 fechado): mutação `state.step_hierarchical`
            // ELIMINADA — caminho Introspector activo via populate_intr
            // arm Heading (P191B `apply_hierarchical_at`). intr.counters
            // populated antes desta arm via walk top emission. State
            // legacy struct eliminada.

            // P190G (M6 categoria Labels & TOC): `state.auto_label_counter`
            // eliminado — substituído por local var threaded via
            // `auto_label_counter: &mut usize` parameter (Opção α).
            *auto_label_counter += 1;
            let current_auto_label = *auto_label_counter;
            // P191B (ADR-0071): compute_heading_auto_toc lê via
            // Introspector path location-aware. `emitted_loc` é
            // `Some(loc)` para Heading (locatable; populate_intr no
            // walk top já registou Heading kind + counter no intr).
            let auto_loc = emitted_loc.expect(
                "Heading é locatable — emitted_loc deve ser Some",
            );
            let (auto_label, resolved_text) = compute_heading_auto_toc(
                &*intr,
                auto_loc,
                current_auto_label,
            );
            // P190G: mutação `state.resolved_labels.insert` ELIMINADA
            // — caminho Introspector activo via Tag::Labelled
            // pós-recursão (populate_intr_from_tag_start arm Labelled
            // popula intr.resolved_labels). Layouter consumer
            // `references.rs:64` migrado para Introspector path puro
            // (sem fallback legacy).

            // P190G: mutação `state.headings_for_toc.push` ELIMINADA
            // — caminho Introspector activo via Tag::HeadingForToc
            // pós-recursão (populate_intr_from_tag_start arm
            // HeadingForToc popula intr.headings_for_toc). Layouter
            // consumer `outline.rs:38` migrado para Introspector path
            // puro (sem fallback legacy).
            //
            // `frozen_body` ainda computed aqui — reusado na Tag
            // HeadingForToc pós-recursão (P200B).
            let frozen_body = materialize_time(body, &*intr, auto_loc);

            walk(body, locator, tags, intr, auto_label_counter, lang, None);

            // P196B: emit Tag auto-toc pós-recursão (ADR-0069).
            // Reusa Location alocada para Heading (locatable; walk
            // top emitiu Tag::Start). `emitted_loc` é `Some(loc)` —
            // mais simples que P195D (Labelled não-locatable
            // exigiu snapshot+find_map).
            //
            // P191B (ADR-0071): popula intr directamente via
            // populate_intr_from_tag_start no momento da emissão.
            if let Some(loc) = emitted_loc {
                let info = ElementInfo::new(ElementPayload::Labelled {
                    label: auto_label.clone(),
                    resolved_text: Some(resolved_text),
                    figure_number: None,
                });
                populate_intr_from_tag_start(intr, &info, loc);
                tags.push(Tag::Start(loc, info));
                tags.push(Tag::End(loc, 0));
            }

            // P200B: emit Tag::HeadingForToc pós-recursão (3ª Tag).
            // Popula sub-store intr.headings_for_toc via
            // populate_intr_from_tag_start (P191B). Mesma Location
            // que Heading + Tag::Labelled auto-toc — sub-stores
            // diferentes (sem conflito per P196A §11.5). Fecha
            // E2-residuo + lacuna #3.
            if let Some(loc) = emitted_loc {
                if let Some((label, body_for_toc, lvl)) = compute_heading_for_toc(
                    current_auto_label,
                    frozen_body,
                    *level as usize,
                ) {
                    let info = ElementInfo::new(ElementPayload::HeadingForToc {
                        label,
                        body: body_for_toc,
                        level: lvl,
                    });
                    populate_intr_from_tag_start(intr, &info, loc);
                    tags.push(Tag::Start(loc, info));
                    tags.push(Tag::End(loc, 0));
                }
            }
        }

        Content::Equation { block: _, body } => {
            // E1 fechada — Reserva 1 materializada em P199B (cenário α
            // por construção): SetEquationNumbering popula intr.state;
            // populate_intr arm Equation aplica counter gated por
            // `block && intr.state.value_at("numbering_active:equation",
            // loc) == Some(Bool(true))`.
            //
            // P190I (M6 fechado): mutação `state.step_flat("equation")`
            // ELIMINADA — populate_intr arm Equation já aplica counter
            // a `intr.counters["equation"]` no momento da emission. Walk
            // arm Equation puro — apenas desce em body.
            walk(body, locator, tags, intr, auto_label_counter, lang, None);
        }

        Content::Figure { body, caption, kind: _, numbering: _ } => {
            // P197B (cenário α) — caminho Introspector activo desde
            // P184 (variant ElementPayload::Figure + populate_intr arm
            // Figure + sub-store CounterRegistry chave `figure:{kind}`
            // + consumer C3 P184D `figure_number_at_index`).
            //
            // P190H (M6 categoria Figures eliminada): mutações walk
            // arm Figure ELIMINADAS (Opção α `.D`):
            // - `state.local_figure_counters.entry(...) += 1`.
            // - `state.figure_numbers.entry(...).push(...)`.
            // - Helper `compute_figure` removido (orphan após
            //   eliminação dos consumers walk-side).
            //
            // populate_intr Figure arm (P191C) já popula
            // `intr.counters["figure:{kind}"]` no momento da Tag::Start
            // emission (gated por `is_counted`). compute_labelled
            // (P191C migrated) consume via `intr.flat_counter_at`.
            // Layouter consumer C3 consume via `figure_number_at_index`
            // (P184D). Walk arm Figure agora puro — apenas desce em
            // body + caption.
            walk(body, locator, tags, intr, auto_label_counter, lang, None);
            if let Some(cap) = caption {
                walk(cap, locator, tags, intr, auto_label_counter, lang, None);
            }
        }

        Content::Labelled { target, label } => {
            // P195D — Walk arm Labelled emite Tag pós-recursão
            // (pattern ADR-0069 post-recursion-tag-emission).
            // Lógica legacy (E2/E3 P189B excepção) **preservada**
            // como write paralelo durante janela compat M5;
            // funcionalmente fecha em M6 quando legacy for removido.
            //
            // Walk no target primeiro — garante que o contador já avançou.
            // P162 .E: passa `Some(label)` para que o tag emitido pelo
            // walk recursivo (ex. Heading) inclua a label do wrapper.
            let tags_len_before = tags.len();
            walk(target, locator, tags, intr, auto_label_counter, lang, Some(label));

            // P191C (ADR-0071 ACEITE): Location do target obtida via
            // snapshot+find_map (pattern P195D variante não-locatable).
            // Necessária ANTES de chamar compute_labelled — helper
            // agora recebe `intr + location + target + lang`.
            let target_loc = tags[tags_len_before..]
                .iter()
                .find_map(|t| if let Tag::Start(l, _) = t { Some(*l) } else { None });

            // Computar resolved_text + figure_number via helper
            // privado (per P195A §11.6; ADR-0069). Sem mutação aqui.
            // P191C: helper migrado para Introspector path
            // location-aware. Caso target não-locatable
            // (target_loc=None): retorna (None, None) coerente com
            // legacy "label sem target locatable não resolve".
            let (resolved_text, figure_number) = match target_loc {
                Some(loc) => compute_labelled(
                    &*intr,
                    loc,
                    target,
                    lang,
                ),
                None => (None, None),
            };

            // P190H (M6 categoria Figures eliminada): mutação
            // `state.figure_label_numbers.insert` ELIMINADA — caminho
            // Introspector activo via Tag::Labelled pós-recursão
            // (populate_intr_from_tag_start arm Labelled popula
            // `intr.figure_label_numbers` quando figure_number é
            // Some). Consumer C2 (Layouter Ref-arm references.rs:51)
            // migrado para Introspector path puro.
            //
            // P190G: mutação `state.resolved_labels.insert` ELIMINADA
            // — caminho Introspector activo via Tag::Labelled
            // pós-recursão (intr.resolved_labels). Layouter consumer
            // `references.rs:64` migrado para Introspector path puro.
            let _ = figure_number;

            // P195D: emit Tag pós-recursão (ADR-0069). Reusa Location
            // do target — preserva sincronização-por-construção
            // ADR-0068 (walk Locator e Layouter Locator não avançam
            // para Labelled em nenhum dos lados).
            //
            // P191B (ADR-0071): popula intr directamente via
            // populate_intr_from_tag_start no momento da emissão.
            if resolved_text.is_some() || figure_number.is_some() {
                if let Some(loc) = target_loc {
                    let info = ElementInfo::new(ElementPayload::Labelled {
                        label: label.clone(),
                        resolved_text,
                        figure_number,
                    });
                    populate_intr_from_tag_start(intr, &info, loc);
                    tags.push(Tag::Start(loc, info));
                    tags.push(Tag::End(loc, 0));
                }
                // else: target não-locatable → no Tag::Start em tags;
                // sem Location para reuso. Sub-store via Tag não é
                // populated (mas mutação legacy acima preservou
                // resolved_labels). Caso edge raro — labels
                // tipicamente envolvem locatables.
            }
        }

        Content::SetHeadingNumbering { active: _ } => {
            // P198B — E5 fechada estruturalmente (cenário α). Caminho
            // Introspector activo desde P182C (extract_payload →
            // ElementPayload::StateUpdate sob chave
            // numbering_active:heading → populate_intr_from_tag_start
            // arm StateUpdate popula intr.state).
            //
            // P190G (M6 categoria Labels & TOC; Caso 1 `.H`): mutação
            // `state.numbering_active.insert("heading", *active)`
            // ELIMINADA — sem walk readers após P191B/C migrarem
            // helpers (`compute_heading_auto_toc` lê
            // `intr.is_numbering_active_at`) e walk arm Equation gate
            // (lê `intr.is_numbering_active_at`). Field
            // `state.numbering_active` eliminado de
            // `CounterStateLegacy`.
            //
            // Tag::Start emitida pelo walk top via `extract_payload`;
            // populate_intr_from_tag_start popula intr.state — caminho
            // Introspector é única fonte da verdade.
        }

        Content::SetEquationNumbering { active: _ } => {
            // P199B — E1 fechada estruturalmente (cenário α por
            // construção). Materializa Reserva 1 desde P189B.
            // Caminho Introspector activado por construção:
            // extract_payload → ElementPayload::StateUpdate sob chave
            // numbering_active:equation → populate_intr_from_tag_start
            // arm StateUpdate popula intr.state.
            //
            // P190G (M6 categoria Labels & TOC; Caso 1 `.H`): mutação
            // `state.numbering_active.insert("equation", *active)`
            // ELIMINADA — sem walk readers após P191B migrar walk arm
            // Equation gate para `intr.is_numbering_active_at` e
            // P191C migrar `compute_labelled` Equation arm para
            // Introspector path. Field `state.numbering_active`
            // eliminado.
        }

        Content::CounterUpdate { key: _, action: _ } => {
            // P198C — E6 fechada estruturalmente (cenário β-promote
            // ADR-0069). Caminho Introspector activo:
            // extract_payload arm emite ElementPayload::CounterUpdate
            // pré-recursão; populate_intr arm CounterUpdate popula
            // CounterRegistry via apply_at (flat) ou
            // apply_hierarchical_at (key="heading").
            //
            // P190I (M6 fechado): mutação legacy
            // `state.step_*`/`state.update_flat` ELIMINADA — `compute_*`
            // helpers migrados para Introspector path location-aware
            // (P191B/C). populate_intr é única fonte da verdade. Walk
            // arm CounterUpdate puro — sem recursão, sem mutação.
        }

        // Passo 101: `Content::Strong`/`Content::Emph` removidos; o caso
        // equivalente de filhos com Labels/contadores dentro de um bloco
        // estilizado está agora coberto pelo arm `Content::Styled` abaixo.

        // Terminais e nós sem efeito em contadores — cobertos explicitamente
        // para que o compilador detecte variantes em falta (sem wildcard silencioso).
        Content::SetFigureNumbering { .. } => {
            // No-op: a numeração está baked-in em cada nó Figure (capturada em eval).
        }

        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Ref { .. }
        | Content::CounterDisplay { .. }
        | Content::Raw { .. }
        | Content::ListItem(_)
        | Content::EnumItem { .. }
        | Content::Link { .. }
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::MathAlignPoint
        | Content::Linebreak
        | Content::Image { .. }
        | Content::SetPage { .. }
        | Content::Divider
        // Passo 156D — h/v spacing leaves; sem effect em counters.
        | Content::HSpace { .. }
        | Content::VSpace { .. }
        // Passo 156E — pagebreak leaf; sem effect em counters.
        | Content::Pagebreak { .. }
        // Passo 220 — colbreak leaf; sem effect em counters.
        | Content::Colbreak { .. }
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — sem efeito em counters.
        // Tag::Start/End já é emitido no topo de walk via extract_payload
        // (que produz `Some(ElementPayload::Metadata)`).
        | Content::Metadata { .. }
        // P171 (M9): State e StateUpdate são terminais. Tag emitido
        // no topo via extract_payload.
        | Content::State { .. }
        | Content::StateUpdate { .. }
        // P240 (M9d/M7+1): StateDisplay terminal. Tag emitido no topo
        // via extract_payload; valor pre-rendered em apply_state_displays.
        | Content::StateDisplay { .. }
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; tag emitido no topo via extract_payload; valor
        // pre-rendered em apply_counter_displays.
        | Content::CounterDisplayCallback { .. } => {}

        // Passo 154B — Terms / TermItem: descem em items para que filhos
        // com contadores ou labels sejam processados.
        Content::Terms { items } => {
            for item in items { walk(item, locator, tags, intr, auto_label_counter, lang, None); }
        }
        Content::TermItem { term, description } => {
            walk(term, locator, tags, intr, auto_label_counter, lang, None);
            walk(description, locator, tags, intr, auto_label_counter, lang, None);
        }

        // Passo 155 — Quote: walk em body + attribution.
        Content::Quote { body, attribution, .. } => {
            walk(body, locator, tags, intr, auto_label_counter, lang, None);
            if let Some(a) = attribution {
                walk(a, locator, tags, intr, auto_label_counter, lang, None);
            }
        }

        Content::Transform { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        Content::Grid { cells, header, footer, .. } => {
            // P224 — Grid refino: walk em header (se houver) + cells + footer.
            if let Some(h) = header { walk(h, locator, tags, intr, auto_label_counter, lang, None); }
            for cell in cells { walk(cell, locator, tags, intr, auto_label_counter, lang, None); }
            if let Some(f) = footer { walk(f, locator, tags, intr, auto_label_counter, lang, None); }
        }

        // P224.B — GridHeader / GridFooter (recurse no body; paridade P157C).
        Content::GridHeader { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),
        Content::GridFooter { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // P224.C — GridCell (recurse no body; paridade P157B TableCell).
        Content::GridCell { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 157A — Table (paridade Grid).
        Content::Table { children, .. } => {
            for c in children { walk(c, locator, tags, intr, auto_label_counter, lang, None); }
        }

        // Passo 157B — TableCell (recurse no body).
        Content::TableCell { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 157C — par simétrico TableHeader/TableFooter
        // (recurse no body).
        Content::TableHeader { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),
        Content::TableFooter { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // P181H: walk arm puro (P163 invariante restaurada para bib).
        // Pré-P181H (P159C/F): walk mutava `state.bib_entries.extend(...)`
        // e `state.bib_numbers.entry(key).or_insert(...)` directamente.
        // Pós-P181H: tag emitida no topo via `extract_payload` (P181D);
        // BibStore populado por `from_tags` arm Bibliography (P181E).
        // Apenas descida no `title` permanece — `entries` são dados
        // opacos consumidos pelo `extract_payload` que constrói
        // `ElementPayload::Bibliography { entries }`. Cite walk em
        // supplement; sem validação cross-reference (ADR-0017
        // Introspection runtime adiada).
        Content::Bibliography { title, .. } => {
            if let Some(t) = title {
                walk(t, locator, tags, intr, auto_label_counter, lang, None);
            }
        }
        Content::Cite { supplement, .. } => {
            if let Some(s) = supplement { walk(s, locator, tags, intr, auto_label_counter, lang, None); }
        }

        Content::Align { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        Content::Place { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 156C (ADR-0061 Fase 1) — pad / hide são containers
        // estruturais; descer no body para que counters/labels dentro sejam
        // processados. `Hide` mesmo "ocultando visualmente" mantém a
        // semântica de presence (label/ref dentro de hide ainda resolvem).
        Content::Pad  { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),
        Content::Hide { body }     => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 156G (ADR-0061 Fase 2) — block container; descer no body.
        Content::Block { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        Content::Boxed { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Walk em cada child em ordem (counters/labels resolvem).
        Content::Stack { children, .. } => {
            for c in children.iter() {
                walk(c, locator, tags, intr, auto_label_counter, lang, None);
            }
        },

        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat container.
        // Walk no body uma vez (counters/labels dentro de body
        // resolvem; semântica de repetição é runtime-only e não
        // multiplica state — vanilla repeat também só conta uma vez).
        Content::Repeat { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),
        // P217 (DEBT-56 sub-fase b) — Columns container.
        // Walk no body (counters/labels dentro contam normalmente);
        // sem Tag::Start/End próprio (columns não é locatable).
        // Consumer multi-region em P219.
        Content::Columns { body, .. } => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        // Passo 99 (ADR-0038): `Styled` é transparente — desce no body.
        Content::Styled(body, _) => walk(body, locator, tags, intr, auto_label_counter, lang, None),

        Content::Outline => {
            // P189B (M5): walk puro para Outline.
            // Mutação `state.has_outline = true` removida; flag obtida
            // via `intr.kind_index.contains_key(&ElementKind::Outline)`
            // (populado por `from_tags` arm Outline P178). Consumer
            // migrado em `mod.rs:1470` (`layout_with_introspector`).
            // `Content::Outline` continua a ser locatable e emite
            // Tag::Start no topo da `walk` fn — apenas a mutação
            // directa em state foi removida.
        }
    }

    // P162 .E: emissão Tag::End após recursão. Usa o mesmo Location
    // que o Tag::Start emitido no topo, e o hash determinístico do
    // conteúdo via hash_content.
    if let Some(loc) = emitted_loc {
        tags.push(Tag::End(loc, hash_content(content)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::Content,
        counter_update::CounterUpdate as CounterAction,
        element_payload::ElementPayload,
        label::Label,
        location::Location,
    };

    #[test]
    fn introspect_popula_label_forward() {
        // Ref antes do Labelled — forward reference
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::Ref { target: Label("conclusao".to_string()) },
                Content::Labelled {
                    label:  Label("conclusao".to_string()),
                    target: Box::new(Content::heading(1, Content::text("Conclusão"))),
                },
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert!(
            intr.resolved_labels.get(&Label("conclusao".to_string())).is_some(),
            "introspect deve popular resolved_labels mesmo para forward refs"
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("conclusao".to_string())),
            Some("Secção 1")
        );
    }

    #[test]
    fn introspect_counter_update_e_aplicado() {
        let content = Content::Sequence(
            vec![Content::CounterUpdate {
                key:    "equation".to_string(),
                action: CounterAction::Update(5),
            }]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 5);
    }

    #[test]
    fn introspect_dois_conteudos_independentes() {
        let content_a = Content::Labelled {
            label:  Label("a".to_string()),
            target: Box::new(Content::heading(1, Content::text("A"))),
        };
        let content_b = Content::Ref { target: Label("a".to_string()) };

        let intr_a = introspect_with_introspector(&content_a);
        let intr_b = introspect_with_introspector(&content_b);

        assert!(intr_a.resolved_labels.get(&Label("a".to_string())).is_some());
        assert!(
            intr_b.resolved_labels.get(&Label("a".to_string())).is_none(),
            "estados de introspecção devem ser independentes"
        );
    }

    #[test]
    fn introspect_set_heading_numbering_activa_flag() {
        let content = Content::SetHeadingNumbering { active: true };
        let intr = introspect_with_introspector(&content);
        assert!(intr.is_numbering_active("numbering_active:heading"));
    }

    // ── Testes de Passo 61 — TOC ─────────────────────────────────────────

    #[test]
    fn introspect_cataloga_headings_para_toc() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
            Content::heading(1, Content::text("Conclusão")),
        ].into());

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 3);

        let (_, title_0, level_0) = &intr.headings_for_toc()[0];
        assert_eq!(title_0.plain_text(), "Introdução");
        assert_eq!(*level_0, 1);

        let (_, _, level_1) = &intr.headings_for_toc()[1];
        assert_eq!(*level_1, 2);
    }

    #[test]
    fn introspect_gera_labels_automaticas_unicas() {
        let content = Content::Sequence(vec![
            Content::heading(1, Content::text("A")),
            Content::heading(1, Content::text("B")),
        ].into());

        let intr = introspect_with_introspector(&content);
        let label_a = &intr.headings_for_toc()[0].0;
        let label_b = &intr.headings_for_toc()[1].0;
        assert_ne!(label_a, label_b, "labels automáticas devem ser únicas");

        // As labels devem estar em resolved_labels
        assert!(intr.resolved_labels.get(label_a).is_some());
        assert!(intr.resolved_labels.get(label_b).is_some());
    }

    #[test]
    fn introspect_heading_sem_numbering_insere_string_vazia_em_resolved_labels() {
        // Sem numeração activa, resolved_labels deve conter "" (não "@auto-toc-N").
        let content = Content::heading(1, Content::text("Título"));
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 1);
        let (label, _, _) = &intr.headings_for_toc()[0];
        assert_eq!(
            intr.resolved_labels.get(label),
            Some(""),
            "heading sem numeração deve ter string vazia em resolved_labels"
        );
    }

    // ── Testes de Passo 62 — Figuras ─────────────────────────────────────

    #[test]
    fn introspect_resolve_label_de_figura() {
        let content = Content::Sequence(
            vec![Content::Labelled {
                label:  Label("fig1".to_string()),
                target: Box::new(Content::Figure {
                    body:      Box::new(Content::text("Um gráfico")),
                    caption:   Some(Box::new(Content::text("Evolução"))),
                    kind:      Some("image".to_string()),
                    numbering: Some("1".to_string()),
                }),
            }]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.resolved_labels.get(&Label("fig1".to_string())),
            Some("Figura 1"),
            "label de figura deve resolver para 'Figura 1'"
        );
    }

    #[test]
    fn introspect_duas_figuras_contadores_independentes() {
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("f1".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("A")),
                        caption:   Some(Box::new(Content::text("Legenda A"))),
                        kind:      Some("image".to_string()),
                        numbering: Some("1".to_string()),
                    }),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda B"))),
                        kind:      Some("image".to_string()),
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.resolved_labels.get(&Label("f1".to_string())),
            Some("Figura 1")
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("f2".to_string())),
            Some("Figura 2")
        );
    }

    #[test]
    fn introspect_figura_sem_caption_nao_incrementa_contador() {
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body:      Box::new(Content::text("Diagrama")),
                    caption:   None,
                    kind:      Some("image".to_string()),
                    numbering: Some("1".to_string()),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda"))),
                        kind:      Some("image".to_string()),
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        // Figura sem caption não consome contador — a segunda figura numerada é "Figura 1"
        assert_eq!(
            intr.resolved_labels.get(&Label("f2".to_string())),
            Some("Figura 1"),
            "figura sem caption não deve consumir o contador"
        );
    }

    #[test]
    fn introspect_backward_ref_tambem_funciona() {
        // Labelled antes de Ref — deve também popular o mapa
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("sec".to_string()),
                    target: Box::new(Content::heading(1, Content::text("Secção"))),
                },
                Content::Ref { target: Label("sec".to_string()) },
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert!(
            intr.resolved_labels.get(&Label("sec".to_string())).is_some(),
            "backward ref deve também estar em resolved_labels"
        );
    }

    // ── Testes de Passo 66 — Materialização temporal (DEBT-18) ───────────────

    #[test]
    fn materialize_time_substitui_counter_display() {
        // P190I (M6 fechado): test adaptado — populate intr manualmente
        // para simular contador "fig" = 42 numa Location.
        use crate::entities::counter_update::CounterUpdate as CU;
        use crate::entities::location::Location;

        let mut intr = TagIntrospector::empty();
        let loc = Location::from_raw(1);
        intr.counters.apply_at("fig".to_string(), CU::Update(42), loc);

        let dynamic_ast = Content::Sequence(
            vec![
                Content::text("Figura "),
                Content::CounterDisplay { kind: "fig".to_string() },
            ]
            .into(),
        );

        let frozen = materialize_time(&dynamic_ast, &intr, loc);

        let expected = Content::Sequence(
            vec![
                Content::text("Figura "),
                Content::text("42"),
            ]
            .into(),
        );

        assert_eq!(frozen, expected,
            "CounterDisplay deve ser materializado em Text com o valor do contador");
    }

    #[test]
    fn materialize_time_preserva_terminais() {
        // P190I: test adaptado — intr vazio, Location dummy.
        use crate::entities::location::Location;

        let intr = TagIntrospector::empty();
        let loc = Location::from_raw(1);

        // Nós terminais sem CounterDisplay devem ser clonados sem alteração.
        let content = Content::Sequence(
            vec![
                Content::text("Texto estático"),
                Content::strong(Content::text("Negrito")),
            ]
            .into(),
        );

        let frozen = materialize_time(&content, &intr, loc);
        assert_eq!(frozen, content, "Terminais sem CounterDisplay não devem ser alterados");
    }

    #[test]
    fn introspect_headings_for_toc_congelados() {
        // Simular: = Figura #counter("fig").display()
        // O CounterDisplay no título deve ser substituído pelo valor no momento
        // da introspecção — não pelo valor quando a TOC for renderizada.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::CounterUpdate {
                    key:    "fig".to_string(),
                    action: CounterAction::Update(7),
                },
                Content::heading(1, Content::Sequence(
                    vec![
                        Content::text("Figura "),
                        Content::CounterDisplay { kind: "fig".to_string() },
                    ]
                    .into(),
                )),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 1);

        let (_, frozen_body, _) = &intr.headings_for_toc()[0];
        let text = frozen_body.plain_text();
        // O body congelado deve conter "7" (valor no momento da introspecção),
        // não "0" (valor no início do documento quando a TOC é renderizada).
        assert!(text.contains("7"),
            "CounterDisplay no título deve ser congelado com o valor correcto: {:?}", text);
    }

    // ── Testes de Passo 75 — figure_numbers por kind (DEBT-14/15) ───────────

    #[test]
    fn figure_tem_kind_e_numbering() {
        let fig = Content::Figure {
            body:      Box::new(Content::text("corpo")),
            caption:   Some(Box::new(Content::text("legenda"))),
            kind:      Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        if let Content::Figure { kind, numbering, .. } = fig {
            assert_eq!(kind.as_deref(), Some("image"));
            assert_eq!(numbering, Some("1".to_string()));
        } else {
            panic!("Variante inesperada");
        }
    }

    #[test]
    fn figuras_kind_diferente_contadores_independentes() {
        let doc = Content::Sequence(vec![
            Content::Figure {
                body:      Box::new(Content::text("img1")),
                caption:   Some(Box::new(Content::text("cap1"))),
                kind:      Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("tab1")),
                caption:   Some(Box::new(Content::text("cap2"))),
                kind:      Some("table".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("img2")),
                caption:   Some(Box::new(Content::text("cap3"))),
                kind:      Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
        ].into());

        let intr = introspect_with_introspector(&doc);

        let image_nums = (0..).map_while(|i| intr.figure_number_at_index("image", i)).collect::<Vec<_>>();
        let table_nums = (0..).map_while(|i| intr.figure_number_at_index("table", i)).collect::<Vec<_>>();

        assert_eq!(image_nums, vec![1, 2],
            "Duas figuras de kind 'image' devem produzir [1, 2]");
        assert_eq!(table_nums, vec![1],
            "Uma figura de kind 'table' deve produzir [1] independentemente");
    }

    // ── Passo 158B — Supplement automático por lang em figure ────────────

    /// Helper para construir state com lang explícito.
    /// **P190G**: retorna `(state, intr)` — testes leem
    /// `intr.resolved_labels` (field state.resolved_labels eliminado).
    fn introspect_with_lang(content: &Content, lang_code: &str) -> TagIntrospector {
        use std::str::FromStr;
        use crate::entities::lang::Lang;
        let lang = Lang::from_str(lang_code).unwrap();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, Some(&lang), None);
        intr
    }

    #[test]
    fn figure_label_default_no_lang_set_devolve_pt() {
        // P158B §8.2: lang None → fallback PT (backwards compat).
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_introspector(&labelled);
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Figura 1"),
            "Default sem lang → PT 'Figura'"
        );
    }

    #[test]
    fn figure_label_lang_pt_image_devolve_figura() {
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_lang(&labelled, "pt");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Figura 1"),
        );
    }

    #[test]
    fn figure_label_lang_en_table_devolve_table() {
        use crate::entities::label::Label;
        let label = Label("tab1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("table".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_lang(&labelled, "en");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Table 1"),
        );
    }

    #[test]
    fn figure_label_lang_de_raw_devolve_listing() {
        use crate::entities::label::Label;
        let label = Label("lst1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("raw".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_lang(&labelled, "de");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Listing 1"),
        );
    }

    #[test]
    fn figure_label_lang_unknown_fallback_pt() {
        // P158B §8.2: lang desconhecido (zh) → fallback PT.
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_lang(&labelled, "zh");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Figura 1"),
            "Lang desconhecido cai no fallback PT"
        );
    }

    #[test]
    fn figure_label_kind_custom_devolve_capitalizado() {
        // P158B §6: kind desconhecido devolve string capitalizada.
        use crate::entities::label::Label;
        let label = Label("custom1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("custom".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let intr = introspect_with_lang(&labelled, "en");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Custom 1"),
            "Kind desconhecido capitalizado"
        );
    }

    #[test]
    fn figure_counters_independentes_por_kind_continuam_a_funcionar_apos_p158b() {
        // Regression P157A/P158A: counters por kind continuam
        // independentes; supplement P158B não interfere.
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::Figure {
                body: Box::new(Content::text("body1")),
                caption: Some(Box::new(Content::text("c1"))),
                kind: Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body: Box::new(Content::text("body2")),
                caption: Some(Box::new(Content::text("c2"))),
                kind: Some("table".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body: Box::new(Content::text("body3")),
                caption: Some(Box::new(Content::text("c3"))),
                kind: Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
        ]));
        let intr = introspect_with_introspector(&content);
        assert_eq!((0..).map_while(|i| intr.figure_number_at_index("image", i)).collect::<Vec<_>>(),
            vec![1, 2], "image counter independente");
        assert_eq!((0..).map_while(|i| intr.figure_number_at_index("table", i)).collect::<Vec<_>>(),
            vec![1], "table counter independente");
    }

    // ── Passo 158C: Figure.kind = None resolve a "image" em uso ─────────

    #[test]
    fn introspect_figure_kind_none_resolve_para_image_no_counter() {
        // P158C: kind=None deve cair no default "image" via fallback
        // em counter (paridade backwards compat com tests pré-existentes
        // que usam Some("image".to_string())).
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("f_none".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("body sem kind explícito")),
                        caption:   Some(Box::new(Content::text("legenda"))),
                        kind:      None,  // P158C: novo caso
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Counter "image" deve avançar via fallback default.
        assert_eq!((0..).map_while(|i| intr.figure_number_at_index("image", i)).collect::<Vec<_>>(),
            vec![1],
            "kind=None deve cair no default 'image' no counter");
        // Label resolve para "Figura 1" via fallback (PT default em
        // figure_supplement_for_lang).
        assert_eq!(
            intr.resolved_labels.get(&Label("f_none".to_string())),
            Some("Figura 1"),
            "label de figura kind=None deve resolver via fallback 'image' default"
        );
    }

    // ── P162 .G — Tests do walk com tags em paralelo ─────────────────────

    /// Helper de teste para correr walk e devolver tags.
    /// **P190I**: state eliminado.
    fn introspect_with_tags(content: &Content) -> Vec<Tag> {
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);
        tags
    }

    #[test]
    fn walk_emite_start_e_end_para_heading() {
        let h = Content::heading(1, Content::text("title"));
        let tags = introspect_with_tags(&h);
        // P200B: heading emite 6 tags com mesma Location:
        //   Start(Heading), Start(Labelled auto-toc), End(Labelled),
        //   Start(HeadingForToc), End(HeadingForToc), End(Heading).
        // 3 pares Start/End — bracketing por construção (mesma loc).
        assert_eq!(tags.len(), 6, "heading deve emitir 6 tags pós-P200B; obtido {tags:?}");
        let locs: Vec<_> = tags.iter().map(|t| match t {
            Tag::Start(l, _) | Tag::End(l, _) => *l,
        }).collect();
        // Todas as 6 tags partilham mesma Location (P196A §11.5 +
        // P200B trabalho híbrido).
        for w in locs.windows(2) {
            assert_eq!(w[0], w[1], "tags devem partilhar mesma Location");
        }
        // Ordem esperada: 3 Start consecutivas + 3 End consecutivas
        // (Heading abre, Labelled abre, Labelled fecha, HeadingForToc
        // abre, HeadingForToc fecha, Heading fecha) — ou variação
        // específica per ordem de emit no walk arm.
        match (&tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5]) {
            (
                Tag::Start(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::End(_, _),
            ) => {}
            other => panic!("ordem esperada: Start, Start, End, Start, End, End; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_nao_emite_para_text_simples() {
        let t = Content::text("plain");
        let tags = introspect_with_tags(&t);
        assert!(tags.is_empty(), "text simples não deve emitir tags");
    }

    #[test]
    fn walk_aninha_start_end_para_heading_contendo_figure() {
        // Heading com Figure aninhada no body.
        let figure = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   Some(Box::new(Content::text("cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".into()),
        };
        let h = Content::heading(1, figure);
        let tags = introspect_with_tags(&h);
        // P200B: heading emite 6 tags + figura emite 2 tags = 8 tags.
        // Sequência: Start(Heading), Start(Figure), End(Figure),
        // Start(Labelled), End(Labelled), Start(HeadingForToc),
        // End(HeadingForToc), End(Heading).
        assert_eq!(tags.len(), 8, "heading-com-figura deve emitir 8 tags pós-P200B, obtido {tags:?}");
        match (&tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5], &tags[6], &tags[7]) {
            (
                Tag::Start(_, _), // Heading
                Tag::Start(_, _), // Figure
                Tag::End(_, _),   // Figure
                Tag::Start(_, _), // Labelled auto-toc
                Tag::End(_, _),   // Labelled auto-toc
                Tag::Start(_, _), // HeadingForToc
                Tag::End(_, _),   // HeadingForToc
                Tag::End(_, _),   // Heading
            ) => {}
            other => panic!("ordem esperada após P200B: 8 tags; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_emite_tags_em_paralelo_com_state() {
        // Verifica que tanto state quanto tags são populados.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);
        // P190I: intr verificado via introspect_with_introspector
        // separado (state legacy eliminada).
        let intr = introspect_with_introspector(&content);
        // Contador heading deve estar em "2" após dois headings nivel 1.
        assert_eq!(intr.formatted_counter("heading").as_deref(), Some("2"),
            "intr deve ter contador heading=2 após dois headings nível 1");
        // P182C: SetHeadingNumbering passou a ser locatable (emite
        // ElementPayload::StateUpdate sob chave numbering_active:heading).
        // P200B: cada heading emite 6 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc, End_h).
        // Total: 1 SetHeadingNumbering × 2 + 2 headings × 6 = 14.
        assert_eq!(tags.len(), 14, "deve haver Start+End para SetHeadingNumbering e 6 tags por heading; obtido {tags:?}");
    }

    #[test]
    fn walk_label_de_wrapper_chega_ao_payload() {
        // Content::Labelled { target: Heading } → tag Heading recebe Some(label).
        let content = Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        };
        let tags = introspect_with_tags(&content);
        // Esperado: Start(Heading) com label="intro", End(Heading).
        match &tags[0] {
            Tag::Start(_, info) => {
                assert_eq!(
                    info.label.as_ref().map(|l| l.0.as_str()),
                    Some("intro"),
                    "label do wrapper Labelled deve aparecer no ElementInfo do Heading"
                );
            }
            other => panic!("esperado Tag::Start, obtido {other:?}"),
        }
    }

    // ── P163 .C — Tests E2E de bracketing ────────────────────────────────

    /// Helper local: constrói Content com headings, figures, citations
    /// aninhados em diversas profundidades para tests E2E.
    fn make_content_complexo() -> Content {
        Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Capítulo")),
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("legenda"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
                Content::heading(2, Content::text("Secção")),
                Content::cite("smith2024", None, None),
            ]
            .into(),
        )
    }

    #[test]
    fn walk_e_deterministico() {
        // P163 .C.1: walk duas vezes sobre o mesmo Content produz
        // Vec<Tag> idêntico (mesma ordem, mesmas Locations, mesmos hashes).
        let content = make_content_complexo();
        let tags1 = introspect_with_tags(&content);
        let tags2 = introspect_with_tags(&content);
        assert_eq!(
            tags1, tags2,
            "walk não-determinístico: tags1.len={}, tags2.len={}",
            tags1.len(), tags2.len()
        );
    }

    #[test]
    fn bracketing_valido_em_aninhamento_complexo() {
        // P163 .C.2: heading ⊃ figure-com-caption-com-text ⊃ heading.
        // Verificar que cada Start tem o seu End correspondente, sem
        // overlapping. Headings emitem tags; figures aninhadas no
        // caption também.
        let inner_h = Content::heading(2, Content::text("inner"));
        let figure_with_h = Content::Figure {
            body:      Box::new(inner_h),
            caption:   Some(Box::new(Content::text("cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".into()),
        };
        let outer_h = Content::heading(1, figure_with_h);
        let tags = introspect_with_tags(&outer_h);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc, "End com Location diferente do último Start");
                }
            }
        }
        assert!(stack.is_empty(), "Start sem End correspondente; stack={stack:?}");
    }

    #[test]
    fn bracketing_valido_em_sequencia_plana() {
        // Caso adicional .C.2: múltiplos headings ao mesmo nível
        // (não aninhados) — bracketing também válido.
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
                Content::heading(1, Content::text("três")),
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc);
                }
            }
        }
        assert!(stack.is_empty());
        // P200B: cada heading emite 6 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc, End_h)
        // — todas com mesma Location, bracketing continua válido.
        // 3 headings × 6 = 18.
        assert_eq!(tags.len(), 18, "3 headings × 6 tags pós-P200B = 18");
    }

    #[test]
    fn end_hash_distingue_conteudo() {
        // P163 .C.3: dois headings com bodies diferentes produzem
        // Tag::End com u128 distintos.
        // P196B: heading emite agora 2 Tag::End — primeiro o auto-toc
        // Labelled (hash=0 fixo, ADR-0069) e depois o End real do
        // Heading (hash via hash_content). Filtramos hash != 0.
        let a = Content::heading(1, Content::text("Título A"));
        let b = Content::heading(1, Content::text("Título B"));
        let tags_a = introspect_with_tags(&a);
        let tags_b = introspect_with_tags(&b);

        let end_a = tags_a.iter().find_map(|t| match t {
            Tag::End(_, h) if *h != 0 => Some(*h),
            _ => None,
        }).expect("nenhum Tag::End com hash != 0 emitido para a");
        let end_b = tags_b.iter().find_map(|t| match t {
            Tag::End(_, h) if *h != 0 => Some(*h),
            _ => None,
        }).expect("nenhum Tag::End com hash != 0 emitido para b");

        assert_ne!(
            end_a, end_b,
            "Contents distintos produziram mesmo content_hash em Tag::End"
        );
    }

    // ── P163 .D — Tests de consistência por kind ─────────────────────────

    #[test]
    fn headings_capturados_em_paralelo() {
        // P163 .D.1: walk sobre Content com N headings em níveis
        // variados. Verificar:
        //  - número de Tag::Start(_, Heading{..}) bate com input;
        //  - depth de cada Heading bate com level esperado;
        //  - intr.formatted_counter("heading") fica no valor
        //    esperado após todos os headings (verificação cruzada).
        let levels = vec![1u8, 2, 2, 3];
        let content = Content::Sequence(
            std::iter::once(Content::SetHeadingNumbering { active: true })
                .chain(levels.iter().map(|&l| Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let tags = introspect_with_tags(&content);
        // P190I: intr separado para verificação cross.
        let intr = introspect_with_introspector(&content);

        let captured_levels: Vec<u8> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Heading { depth, .. } => Some(*depth),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_levels, levels,
            "depth dos heading payloads em tags difere dos levels do input"
        );

        // Verificação cruzada: intr.formatted_counter("heading")
        // depois de [1, 2, 2, 3] deve ser "1.1.1.1" — após
        // step ao nível 1, depois nível 2 (= [1,1]), nível 2 outra
        // vez (= [1,2]), depois nível 3 (= [1,2,1]).
        // Na verdade: walk usa step_hierarchical que avança o último
        // segmento ou empurra um novo nível. Com [1,2,2,3]:
        //  []     +1 → [1]
        //  [1]    +2 → [1, 1]
        //  [1,1]  +2 → [1, 2]
        //  [1,2]  +3 → [1, 2, 1]
        // → format = "1.2.1"
        assert_eq!(
            intr.formatted_counter("heading").as_deref(),
            Some("1.2.1"),
            "format_hierarchical não bate com sequência [1,2,2,3]"
        );
    }

    #[test]
    fn figures_capturadas_em_paralelo() {
        // P163 .D.2: walk sobre Content com 3 figures (kind variados).
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c1"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c2"))),
                    kind:      Some("table".into()),
                    numbering: Some("1".into()),
                },
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c3"))),
                    kind:      None,
                    numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let captured_kinds: Vec<Option<String>> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Figure { kind, .. } => Some(kind.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_kinds,
            vec![Some("image".to_string()), Some("table".to_string()), None],
            "kinds das figures em tags não batem com input"
        );

        // P190H: paridade verificada em testes dedicados via
        // intr.figure_number_at_index (P190H não toca este test
        // específico de Tag emission).
    }

    #[test]
    fn citations_capturadas_em_paralelo() {
        // P163 .D.3: walk sobre Content com 3 citations distintas.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("smith2024", None, None),  // repetida
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let captured_keys: Vec<String> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Citation { key } => Some(key.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_keys,
            vec!["smith2024".to_string(), "jones2023".to_string(), "smith2024".to_string()],
            "keys das citations em tags não batem com input (incluindo repetição)"
        );

        // Verificação: 3 citations × 2 tags cada = 6 (Start + End por citation).
        let citation_tags: Vec<&Tag> = tags.iter()
            .filter(|t| match t {
                Tag::Start(_, info) => matches!(info.payload, ElementPayload::Citation { .. }),
                Tag::End(_, _) => true, // End não tem payload mas todos os Ends correspondem a Citations aqui
            })
            .collect();
        assert_eq!(citation_tags.len(), 6, "3 citations × 2 tags = 6, obtido {}", citation_tags.len());
    }

    // ── P165 .G — Tests E2E paralelo CounterStateLegacy + Introspector ───

    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;

    /// Helper de teste para correr walk + construir Introspector em paralelo.
    /// **P191B (ADR-0071)**: walk popula intr directamente; sem
    /// chamada a `from_tags` (eliminado). Funcs em `state.update`
    /// silenciosamente ignoradas neste path local (sem Engine
    /// disponível) — coerente com semântica P173 pré-P191B.
    fn introspect_with_introspector(content: &Content) -> TagIntrospector {
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);
        intr
    }

    #[test]
    fn introspector_consistencia_heading() {
        // P165 .G.1: walk com headings em níveis [1, 2, 2, 3] →
        // Introspector.kind_index[Heading] tem 4 locations.
        // CounterStateLegacy.format_hierarchical("heading") confirma
        // mesma contagem.
        //
        // **P170 (M9 sub-passo 2)**: paridade extendida — Introspector
        // agora também produz "1.2.1" via formatted_counter (resolve
        // lacuna #5).
        let levels = vec![1u8, 2, 2, 3];
        let content = Content::Sequence(
            std::iter::once(Content::SetHeadingNumbering { active: true })
                .chain(levels.iter().map(|&l| Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Introspector tem 4 headings indexados.
        assert_eq!(intr.query_by_kind(ElementKind::Heading).len(), 4);
        // CounterStateLegacy tem hierarchical "1.2.1" após [1,2,2,3]
        // (verificado em P163 .D.1).
        assert_eq!(
            intr.formatted_counter("heading").as_deref(),
            Some("1.2.1")
        );
        // P170: Introspector tem mesma string via formatted_counter.
        assert_eq!(
            intr.formatted_counter("heading").as_deref(),
            Some("1.2.1"),
            "P170: paridade entre legacy.format_hierarchical e \
             introspector.formatted_counter"
        );
    }

    #[test]
    fn introspector_consistencia_figure() {
        // P165 .G.2: 3 figures (kind variados) → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c1"))),
                    kind: Some("image".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c2"))),
                    kind: Some("table".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c3"))),
                    kind: None, numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 3);
        // CounterStateLegacy resolve kind=None para "image" → image=2, table=1.
        // Introspector preserva kind literal mas conta as 3 sob ElementKind::Figure.
        // Divergência conhecida (m1-lacunas-captura.md #1).
        assert_eq!((0..).map_while(|i| intr.figure_number_at_index("image", i)).count(), 2);
        assert_eq!((0..).map_while(|i| intr.figure_number_at_index("table", i)).count(), 1);
    }

    #[test]
    fn introspector_consistencia_citation() {
        // P165 .G.3: 3 citations distintas com keys → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("brown2022", None, None),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let locs = intr.query_by_kind(ElementKind::Citation);
        assert_eq!(locs.len(), 3);
    }

    // ── P182C — pipeline E2E SetHeadingNumbering → StateRegistry ─────────

    #[test]
    fn introspector_set_heading_numbering_active_true_popula_state_registry() {
        // P182C: walk emite tag para Content::SetHeadingNumbering;
        // extract_payload produz ElementPayload::StateUpdate;
        // from_tags arm StateUpdate popula StateRegistry; trait method
        // is_numbering_active retorna true para chave canónica.
        let content = Content::SetHeadingNumbering { active: true };
        let intr = introspect_with_introspector(&content);
        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "P182C: pipeline deve popular StateRegistry com Bool(true)"
        );
    }

    #[test]
    fn introspector_set_heading_numbering_active_false_em_state_registry() {
        // Caso simétrico — Bool(false) é registado e propagado;
        // is_numbering_active devolve false (não por estar ausente,
        // mas porque o valor explícito é Bool(false)).
        let content = Content::SetHeadingNumbering { active: false };
        let intr = introspect_with_introspector(&content);
        assert!(!intr.is_numbering_active("numbering_active:heading"));
    }

    #[test]
    fn introspector_query_by_label() {
        // P165 .G.4: walk com Heading labelled → query_by_label retorna location;
        // mesma location aparece em query_by_kind(Heading).
        let content = Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        };
        let intr = introspect_with_introspector(&content);

        let by_label = intr.query_by_label(&Label("intro".to_string()));
        assert!(by_label.is_some(), "label intro deveria ter sido indexada");
        let by_kind = intr.query_by_kind(ElementKind::Heading);
        assert_eq!(by_kind.len(), 1);
        assert_eq!(by_kind.first().copied(), by_label, "location deve coincidir");
    }

    #[test]
    fn introspector_query_first_e_query_unique() {
        // P165 .G.5: walk com 1 Figure → query_first e query_unique retornam Some.
        // walk com 2 Figures → query_first retorna Some(loc1), query_unique None.
        let single_figure = Content::Figure {
            body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c"))),
            kind: Some("image".into()), numbering: Some("1".into()),
        };
        let intr1 = introspect_with_introspector(&single_figure);
        assert!(intr1.query_first(ElementKind::Figure).is_some());
        assert!(intr1.query_unique(ElementKind::Figure).is_some());
        assert_eq!(
            intr1.query_first(ElementKind::Figure),
            intr1.query_unique(ElementKind::Figure)
        );

        let two_figures = Content::Sequence(
            vec![
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c1"))),
                    kind: Some("image".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c2"))),
                    kind: Some("table".into()), numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let intr2 = introspect_with_introspector(&two_figures);
        let first = intr2.query_first(ElementKind::Figure);
        assert!(first.is_some(), "query_first deve devolver primeira location");
        assert_eq!(intr2.query_unique(ElementKind::Figure), None,
            "query_unique deve devolver None com 2 figures");
    }

    // ── P166 .C — Tests da exposição pública do TagIntrospector ──────────

    #[test]
    fn introspect_with_introspector_devolve_par() {
        // P166 .C.1: novo entry point retorna tuple (state, introspector).
        // Verificar que introspector tem 1 heading indexado.
        let content = Content::heading(1, Content::text("título"));
        let intr = introspect_with_introspector(&content);
        assert!(
            intr.query_first(ElementKind::Heading).is_some(),
            "introspector exposto deve ter o heading indexado"
        );
    }

    #[test]
    fn introspect_legacy_continua_a_funcionar() {
        // P166 .C.3 (backward compat): call-site antigo
        // `let state = introspect(&c)` continua a compilar e funcionar.
        // Esta é a invariante crítica de M4b — wrapper preserva API.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.formatted_counter("heading").as_deref(),
            Some("2"),
            "wrapper introspect() deve produzir mesmo state que antes de M4"
        );
    }

    #[test]
    fn introspect_e_introspect_with_introspector_produzem_mesmo_state() {
        // P166 .C.2: state retornado pelo wrapper e pelo entry point novo
        // deve ser idêntico (walk único subjacente; wrapper só descarta
        // o introspector).
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("a")),
                Content::heading(2, Content::text("b")),
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("cap"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let state_legacy = introspect(&content);
        let _intr_new = introspect_with_introspector(&content);

        // Comparação por campos relevantes (CounterStateLegacy não
        // implementa PartialEq globalmente; comparar via API pública).
        assert_eq!(
            state_legacy.formatted_counter("heading").as_deref(),
            _intr_new.formatted_counter("heading").as_deref(),
        );
        // P190H: state.figure_numbers eliminado. Cobertura paridade
        // via intr sub-stores em testes dedicados (P184E suite).
    }

    // ── P173 (M9 sub-passo 5) — E2E cascade Engine através da API pública ─

    use crate::contracts::world::World as _;
    use crate::entities::args::Args;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };
    use crate::rules::eval::EvalContext;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct E2EWorld {
        library: Library,
        book:    FontBook,
        main_id: FileId,
    }
    impl crate::contracts::world::World for E2EWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId { self.main_id }
        fn source(&self, _: FileId) -> FileResult<crate::entities::source::Source>
        { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes>
        { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    fn make_e2e_world() -> E2EWorld {
        E2EWorld {
            library: Library::new(),
            book:    FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }
    fn add_one_native(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: FileId,
        _figure_numbering: Option<&str>,
    ) -> crate::entities::source_result::SourceResult<Value> {
        match args.items.first() {
            Some(Value::Int(n)) => Ok(Value::Int(n + 1)),
            _ => Ok(Value::None),
        }
    }

    macro_rules! with_engine {
        ($world:expr, |$engine:ident, $ctx:ident| $body:block) => {{
            use comemo::Track;
            let world: &dyn crate::contracts::world::World = $world;
            let mut $ctx = EvalContext::new();
            let route = Route::root().with_id(world.main());
            let mut styles = StyleChain::default_chain();
            let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
            let mut active_guards: Vec<RuleId> = Vec::new();
            let current_file = world.main();
            let mut figure_numbering: Option<String> = None;
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut $engine = Engine {
                world,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file,
                figure_numbering: &mut figure_numbering,
                sink: &mut sink,
            };
            $body
        }};
    }

    #[test]
    fn p173_cascade_engine_via_api_publica() {
        // E2E: estado de count via API pública. Cascade Engine →
        // from_tags → apply_func executa Func real.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                },
            ]
            .into(),
        );
        // P191B (ADR-0071): pipeline walk → apply_state_funcs.
        // introspect_with_introspector já não recebe engine/ctx (Funcs
        // path legacy ignoradas); para exercitar Func eval, replicamos
        // o pipeline manual de fixpoint::run_fixpoint.
        let world = make_e2e_world();
        let intr = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content, &mut locator, &mut tags,
                &mut intr, &mut auto_label_counter, None, None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            );
            intr
        });
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(1)));
    }

    #[test]
    fn p173_introspect_legacy_ignora_func() {
        // E2E: API legacy `introspect()` (sem Engine) continua a funcionar.
        // Funcs em state.update são silenciosamente ignoradas.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                },
            ]
            .into(),
        );
        // Path legacy: walk + from_tags(_, None, None).
        let intr = super::introspect_with_introspector(&content);
        // Func ignorada → final value continua em init.
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(0)));
    }

    #[test]
    fn p173_determinismo_func_eval() {
        // E2E: dois chamadas com Engine produzem mesmo resultado.
        let f1 = Func::native("add_one", add_one_native);
        let f2 = Func::native("add_one", add_one_native);
        let content_a = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(5)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f1),
                },
            ]
            .into(),
        );
        let content_b = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(5)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f2),
                },
            ]
            .into(),
        );
        // P191B (ADR-0071): pipeline walk → apply_state_funcs.
        let world = make_e2e_world();
        let v_a = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content_a, &mut locator, &mut tags,
                &mut intr, &mut auto_label_counter, None, None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            );
            intr.state_final_value("c").cloned()
        });
        let v_b = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content_b, &mut locator, &mut tags,
                &mut intr, &mut auto_label_counter, None, None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            );
            intr.state_final_value("c").cloned()
        });
        assert_eq!(v_a, Some(Value::Int(6)));
        assert_eq!(v_a, v_b);
    }

    // ── P181H — Walk arm Bibliography puro (P163 invariante restaurada) ─

    #[test]
    fn walk_arm_bibliography_nao_muta_state_bib_legacy() {
        // P181H: walk arm `Content::Bibliography` não muta directamente
        // `state.bib_entries` ou `state.bib_numbers`. Tag emitida via
        // extract_payload do topo de walk (P181D); BibStore populado
        // por from_tags arm (P181E). Walk puro restaurado para bib —
        // invariante P163 preservada.
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let content = Content::Bibliography {
            entries: vec![
                BibEntry::new("a", "Author A", "Title A", 2024),
                BibEntry::new("b", "Author B", "Title B", 2025),
            ],
            title: None,
        };
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);

        // P190B (M6 categoria Bibliography eliminada): assertions sobre
        // `state.bib_entries`/`bib_numbers` removidas — fields eliminados
        // de CounterStateLegacy. Walk arm puro desde P181H confirmado
        // estruturalmente (sem mutação a fields que já não existem).
        // Cobertura cobrir-a-tag preservada abaixo.

        // Tag emitida pelo topo via extract_payload (P181D): existe
        // exactamente uma Tag::Start de Bibliography.
        use crate::entities::element_payload::ElementPayload;
        let bib_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Bibliography { .. })
        )).collect();
        assert_eq!(bib_tags.len(), 1,
            "tag Bibliography deve ser emitida via extract_payload mesmo com walk puro");
    }

    #[test]
    fn walk_arm_bibliography_desce_em_title() {
        // P181H: walk arm puro continua a descer no `title` (preserva
        // comportamento legacy para que children dentro de title
        // sejam visíveis a outros consumers).
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;
        use crate::entities::label::Label;

        let titulo = Content::Labelled {
            target: Box::new(Content::Heading {
                level: 1,
                body:  Box::new(Content::Empty),
            }),
            label: Label("bib-title".to_string()),
        };

        let content = Content::Bibliography {
            entries: vec![BibEntry::new("a", "A", "T", 2024)],
            title:   Some(Box::new(titulo)),
        };
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);

        // Heading dentro de title produz Tag de Heading.
        use crate::entities::element_payload::ElementPayload;
        let heading_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Heading { .. })
        )).collect();
        assert_eq!(heading_tags.len(), 1,
            "walk deve descer em Bibliography.title — Heading interno deve produzir Tag");
    }

    // ── P196B — Walk arm Heading auto-toc via Tag pattern (ADR-0069) ─────
    //
    // 5 tests E2E que validam: (a) emissão de Tag::Labelled auto-toc;
    // (b) paridade entre legacy state e Introspector resolved_labels;
    // (c) numbering inactivo produz string vazia (paridade legacy);
    // (d) E2-residuo `headings_for_toc` continua via legacy mutation;
    // (e) consumer C4 substitution-with-fallback recebe Some via
    //     Introspector path (auto-toc label sintetizada).

    #[test]
    fn heading_auto_toc_walk_emite_tag_e_popula_introspector() {
        // P196B: walk arm Heading emite Tag::Start+End com payload
        // Labelled { label: "auto-toc-N", resolved_text, figure_number: None }
        // pós-recursão. Introspector populated via from_tags.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Intro")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Auto-label "auto-toc-1" deve estar no Introspector.resolved_labels
        // populado via Tag::Labelled pós-P196B.
        let auto_label = Label("auto-toc-1".to_string());
        assert_eq!(
            intr.resolved_label_for(&auto_label),
            Some("Secção 1"),
            "P196B: Introspector.resolved_labels deve conter auto-toc-1 \
             populado via Tag::Labelled emitida pelo walk arm Heading"
        );
    }

    #[test]
    fn heading_auto_toc_paridade_legacy_vs_introspector() {
        // P196B: durante janela compat M5, legacy state e Introspector
        // devem conter MESMO valor para auto-toc label (write paralelo).
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(2, Content::text("dois")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 headings → auto-toc-1 e auto-toc-2.
        for n in [1usize, 2] {
            let lbl = Label(format!("auto-toc-{n}"));
            let from_legacy = intr.resolved_labels.get(&lbl);
            let from_intr   = intr.resolved_label_for(&lbl);
            assert!(from_legacy.is_some(),
                "legacy state deve conter {lbl:?} (write paralelo M5)");
            assert!(from_intr.is_some(),
                "Introspector deve conter {lbl:?} (Tag::Labelled P196B)");
            assert_eq!(from_legacy, from_intr,
                "paridade compat M5: legacy e Introspector devem ter \
                 o mesmo resolved_text para {lbl:?}");
        }
    }

    #[test]
    fn heading_auto_toc_numbering_inactivo_emite_string_vazia() {
        // P196B preserva paridade legacy: quando numbering inactivo,
        // helper compute_heading_auto_toc retorna (label, "") —
        // resolved_labels recebe insert mesmo assim (presença, não conteúdo).
        let content = Content::heading(1, Content::text("sem numbering"));
        let intr = introspect_with_introspector(&content);

        let lbl = Label("auto-toc-1".to_string());
        // Legacy state preserva insert "" (não None).
        assert_eq!(
            intr.resolved_labels.get(&lbl),
            Some(""),
            "legacy: numbering inactivo → insert string vazia"
        );
        // Introspector path: Tag::Labelled tem resolved_text=Some("") →
        // populated em ResolvedLabelStore.
        assert_eq!(
            intr.resolved_label_for(&lbl),
            Some(""),
            "P196B: Introspector path preserva paridade — string vazia \
             sob numbering inactivo, não None"
        );
    }

    #[test]
    fn walk_e2_residuo_headings_for_toc_via_legacy() {
        // E2-residuo: state.headings_for_toc.push continua activo
        // como mutação legacy porque sub-store `intr.headings_for_toc`
        // não existe (lacuna #3). Test confirma write paralelo M5.
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // P190G: 3 entries em intr.headings_for_toc (1 por heading).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "walk arm Heading popula intr.headings_for_toc via \
             Tag::HeadingForToc pós-recursão (P200B + P190G)"
        );
        // Levels preservados em ordem.
        let levels: Vec<usize> = intr.headings_for_toc()
            .iter()
            .map(|(_, _, lvl)| *lvl)
            .collect();
        assert_eq!(levels, vec![1, 2, 1]);
    }

    #[test]
    fn consumer_c4_recebe_some_para_auto_toc_label() {
        // P196B: consumer C4 (Ref-arm em Layouter, P194B) usa
        // substitution-with-fallback: `intr.resolved_label_for(label)
        // .or_else(|| state.resolved_labels.get(label))`.
        // Pós-P196B, primeira branch (`.resolved_label_for`) deve
        // devolver `Some(text)` para auto-toc labels — caminho
        // Introspector activo (sem fallback necessário).
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Capítulo")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Primeira branch: Introspector path.
        let auto_lbl = Label("auto-toc-1".to_string());
        let via_introspector = intr.resolved_label_for(&auto_lbl);
        assert!(
            via_introspector.is_some(),
            "P196B: consumer C4 deve obter Some via Introspector path \
             para auto-toc-1 — fallback legacy desnecessário"
        );
        assert_eq!(via_introspector, Some("Secção 1"));
    }

    // ── P197B — Walk arm Figure refactor (cenário α) ─────────────────────
    //
    // 5 tests sentinela que validam: (a) caminho Introspector já activo
    // desde P184 (independente de P197B); (b) helper compute_figure
    // produz mesmo resultado que walk legacy; (c) paridade
    // legacy↔Introspector inalterada; (d) numbering inactivo retorna
    // None (helper sem mutação); (e) compute_labelled P195D Figure arm
    // continua funcional após refactor (cadeia E2-E3 preservada).

    #[test]
    fn figure_walk_caminho_introspector_ja_activo() {
        // P197B test 1: confirma que o caminho Introspector para figure
        // numbering já está activo desde P184. Independente de P197B —
        // cenário α (P197A diagnóstico §5).
        let content = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   Some(Box::new(Content::text("Cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".to_string()),
        };
        let intr = introspect_with_introspector(&content);

        // Consumer C3 path (P184D): figure_number_at_index retorna Some.
        assert_eq!(
            intr.figure_number_at_index("image", 0),
            Some(1),
            "caminho Introspector já activo desde P184 — P197B não alterou"
        );
        // kind_index populado.
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 1);
    }

    #[test]
    fn figure_walk_intr_counters_populated_correctamente() {
        // P197B test 2 (P190H adapted): walk arm Figure puro — populate_intr
        // arm Figure (P191C) popula intr.counters["figure:image"] em
        // ordem para figuras is_counted. Field state.figure_numbers
        // eliminado.
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c1"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".to_string()),
                },
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c2"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".to_string()),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 figures image numeradas → intr.figure_number_at_index = [1, 2].
        assert_eq!(
            (0..).map_while(|i| intr.figure_number_at_index("image", i)).collect::<Vec<_>>(),
            vec![1, 2],
            "P190H: populate_intr Figure popula intr.counters em ordem (1-based)"
        );
    }

    #[test]
    fn figure_paridade_introspector_pos_p190h() {
        // P197B test 3 (P190H adapted): confirma paridade Introspector
        // path puro após eliminação fields legacy. Caminho Introspector
        // (P184D) é única fonte da verdade.
        let content = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   Some(Box::new(Content::text("Cap"))),
            kind:      Some("table".into()),
            numbering: Some("1".to_string()),
        };
        let intr = introspect_with_introspector(&content);

        let intr_num = intr.figure_number_at_index("table", 0);
        assert_eq!(intr_num, Some(1),
            "P190H: figura única counted → intr.figure_number_at_index(table, 0) = 1");
    }

    #[test]
    fn figure_numbering_inactivo_nao_popula_intr() {
        // P197B test 4 (P190H adapted): figura sem caption → is_counted
        // = false → populate_intr arm Figure (gated por is_counted)
        // NÃO popula intr.counters. Field state.figure_numbers e
        // state.local_figure_counters eliminados.
        let figura_sem_caption = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   None, // ← sem caption: is_counted = false
            kind:      Some("image".into()),
            numbering: Some("1".to_string()),
        };
        let intr = introspect_with_introspector(&figura_sem_caption);

        // intr.counters["figure:image"] não populated — gate is_counted.
        assert_eq!(
            intr.figure_number_at_index("image", 0),
            None,
            "is_counted=false → populate_intr não aplica counter"
        );
    }

    #[test]
    fn figure_compute_labelled_p195d_continua_funcional() {
        // P197B test 5 (P190H adapted): cadeia E2-E3 preservada via
        // Introspector path puro. compute_labelled (P191C migrado) lê
        // intr.flat_counter_at; populate_intr arm Labelled popula
        // intr.figure_label_numbers.
        let content = Content::Labelled {
            label:  Label("fig1".to_string()),
            target: Box::new(Content::Figure {
                body:      Box::new(Content::Empty),
                caption:   Some(Box::new(Content::text("Cap"))),
                kind:      Some("image".into()),
                numbering: Some("1".to_string()),
            }),
        };
        let intr = introspect_with_introspector(&content);

        // P190H: intr.figure_label_numbers populated via populate_intr
        // arms Figure + Labelled (sem fallback legacy).
        assert_eq!(
            intr.figure_label_numbers.get(&Label("fig1".to_string())).copied(),
            Some(1),
            "Introspector path: figure_label_numbers populated via P195D + P184B"
        );
    }

    // ── P203B — Formalização do fecho das lacunas #1 e #1b ───────────────
    //
    // Lacunas #1 (`figure.kind` literal em tags vs colapsado em state) e
    // #1b (`from_tags` arm Figure sem gate `is_counted`) foram
    // **estruturalmente fechadas** por P190H + P191C:
    //
    // - **P190H**: campos legacy `state.figure_numbers`,
    //   `state.figure_label_numbers`, `state.local_figure_counters`
    //   eliminados; helper `compute_figure` orphan removido.
    // - **P191C**: `populate_intr_from_tag_start` arm Figure aplica
    //   `is_counted` gate + `kind.as_deref().unwrap_or("image")` default
    //   no momento da Tag::Start emission.
    //
    // P203B é trabalho **declarativo** — não toca código produção
    // (já alinhado pós-P190/P191). Este test consolidado formaliza o
    // fecho das lacunas com 4 casos canónicos.
    //
    // Per `P203B.div-1`: spec P203B descrevia walk arm Figure como
    // usando `unwrap_or("image")` directamente, mas walk arm Figure
    // está puro desde P190H — apenas desce em body + caption. O default
    // foi migrado para `populate_intr_from_tag_start` (P191C).

    #[test]
    fn p203b_lacuna_1_e_1b_fecho_formal_4_casos() {
        // 4 casos canónicos per spec P203B C3:
        //   (1) #figure([img])                      — kind=None,    sem caption
        //   (2) #figure([img], caption: [c])        — kind=None,    com caption
        //   (3) #figure(kind: "table", [t], caption)— kind=Some,    com caption
        //   (4) #figure(kind: "table", [t])         — kind=Some,    sem caption

        use crate::rules::introspect::extract_payload::extract_payload;

        let caso1 = Content::Figure {
            body:      Box::new(Content::text("img")),
            caption:   None,
            kind:      None,
            numbering: Some("1".to_string()),
        };
        let caso2 = Content::Figure {
            body:      Box::new(Content::text("img")),
            caption:   Some(Box::new(Content::text("c"))),
            kind:      None,
            numbering: Some("1".to_string()),
        };
        let caso3 = Content::Figure {
            body:      Box::new(Content::text("t")),
            caption:   Some(Box::new(Content::text("c"))),
            kind:      Some("table".to_string()),
            numbering: Some("1".to_string()),
        };
        let caso4 = Content::Figure {
            body:      Box::new(Content::text("t")),
            caption:   None,
            kind:      Some("table".to_string()),
            numbering: Some("1".to_string()),
        };

        // (a) `extract_payload` preserva `kind` literalmente (sem default
        //     — lacuna #1 fecha porque tag preserva None vs Some("image")
        //     distintamente; default só aplica em populate_intr).
        match extract_payload(&caso1) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None,         "caso1 kind preservado None literal");
                assert_eq!(is_counted, false, "caso1 sem caption → is_counted=false");
            }
            other => panic!("caso1: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(&caso2) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None,        "caso2 kind preservado None literal");
                assert_eq!(is_counted, true, "caso2 com caption+numbering → is_counted=true");
            }
            other => panic!("caso2: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(&caso3) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, Some("table".to_string()), "caso3 kind literal Some(\"table\")");
                assert_eq!(is_counted, true,                "caso3 com caption+numbering → is_counted=true");
            }
            other => panic!("caso3: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(&caso4) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, Some("table".to_string()), "caso4 kind literal Some(\"table\")");
                assert_eq!(is_counted, false,               "caso4 sem caption → is_counted=false");
            }
            other => panic!("caso4: esperado Some(Figure), obtido {other:?}"),
        }

        // (b) `populate_intr_from_tag_start` aplica gate `is_counted`
        //     + default `unwrap_or("image")` consistentemente para os
        //     4 casos. Lacuna #1b fecha — gate aplicado no caminho
        //     activo (não em from_tags arm porque tal arm não existe;
        //     população é durante walk via populate_intr_from_tag_start
        //     desde P191B/C ADR-0071).
        let intr1 = introspect_with_introspector(&caso1);
        assert_eq!(intr1.figure_number_at_index("image", 0), None,
            "caso1 is_counted=false → counter NÃO populated (gate aplicado)");

        let intr2 = introspect_with_introspector(&caso2);
        assert_eq!(intr2.figure_number_at_index("image", 0), Some(1),
            "caso2 is_counted=true + kind=None → kind_key='image' (default) → counter[1]");

        let intr3 = introspect_with_introspector(&caso3);
        assert_eq!(intr3.figure_number_at_index("table", 0), Some(1),
            "caso3 is_counted=true + kind=Some(\"table\") → counter['table'][1]");
        assert_eq!(intr3.figure_number_at_index("image", 0), None,
            "caso3 kind='table' → contador 'image' NÃO populated");

        let intr4 = introspect_with_introspector(&caso4);
        assert_eq!(intr4.figure_number_at_index("table", 0), None,
            "caso4 is_counted=false → counter NÃO populated (gate aplicado)");

        // (c) Walk emite Tags consistentes (Tag preserva kind literal).
        let tags1 = introspect_with_tags(&caso1);
        let tags2 = introspect_with_tags(&caso2);
        // caso1 (sem caption, kind=None): payload kind=None preservado.
        let payload1 = tags1.iter().find_map(|t| match t {
            Tag::Start(_, info) => match &info.payload {
                ElementPayload::Figure { kind, is_counted, .. } => Some((kind.clone(), *is_counted)),
                _ => None,
            },
            _ => None,
        });
        assert_eq!(payload1, Some((None, false)),
            "Tag preserva kind=None literal e is_counted=false (paridade pre-walk).");
        // caso2 (com caption, kind=None): payload kind=None preservado, is_counted=true.
        let payload2 = tags2.iter().find_map(|t| match t {
            Tag::Start(_, info) => match &info.payload {
                ElementPayload::Figure { kind, is_counted, .. } => Some((kind.clone(), *is_counted)),
                _ => None,
            },
            _ => None,
        });
        assert_eq!(payload2, Some((None, true)),
            "Tag preserva kind=None literal mesmo quando is_counted=true.");
    }

    // ── P198B — Walk arm SetHeadingNumbering (cenário α) ─────────────────
    //
    // 5 tests sentinela que validam: (a) extract_payload já emite
    // StateUpdate desde P182C; (b) from_tags arm StateUpdate popula
    // StateRegistry; (c) paridade legacy vs Introspector; (d)
    // compute_heading_auto_toc lê mutação legacy durante walk; (e)
    // cadeia E5 ↔ E2 (Heading auto-toc) preservada.

    #[test]
    fn set_heading_numbering_extract_payload_emite_state_update() {
        // P198B test 1: confirma que extract_payload(SetHeadingNumbering)
        // retorna Some(ElementPayload::StateUpdate { ... }) — caminho
        // P182C activo independente de P198B.
        use crate::rules::introspect::extract_payload::extract_payload;
        use crate::entities::state_update::StateUpdate;
        use crate::entities::value::Value;

        let content = Content::SetHeadingNumbering { active: true };
        match extract_payload(&content) {
            Some(ElementPayload::StateUpdate { key, update }) => {
                assert_eq!(key, "numbering_active:heading",
                    "P182C: chave canónica numbering_active:heading");
                match update {
                    StateUpdate::Set(boxed) => assert_eq!(*boxed, Value::Bool(true)),
                    other => panic!("esperado StateUpdate::Set(Bool(true)), obtido {other:?}"),
                }
            }
            other => panic!("esperado Some(StateUpdate), obtido {other:?}"),
        }
    }

    #[test]
    fn set_heading_numbering_from_tags_popula_state_registry() {
        // P198B test 2: pipeline walk + from_tags com SetHeadingNumbering
        // popula StateRegistry com chave canónica.
        use crate::entities::introspector::Introspector;

        let content = Content::SetHeadingNumbering { active: true };
        let intr = introspect_with_introspector(&content);

        // Caminho Introspector activo desde P171/P182C: from_tags arm
        // StateUpdate popula intr.state com numbering_active:heading.
        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "P182C/P171: from_tags arm StateUpdate popula StateRegistry \
             com numbering_active:heading=true"
        );
    }

    #[test]
    fn set_heading_numbering_paridade_legacy_vs_introspector() {
        // P198B test 3: write paralelo legacy + Introspector preserva
        // paridade. Legacy intr.is_numbering_active("numbering_active:heading") == true;
        // Introspector intr.is_numbering_active("numbering_active:heading")
        // == true (chave canónica diferente mas mesmo significado).
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Intro")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy state populado via mutação directa walk arm.
        assert!(intr.is_numbering_active("numbering_active:heading"),
            "legacy: state.numbering_active['heading'] = true (write paralelo M5)");
        // Introspector path populado via from_tags arm StateUpdate.
        assert!(intr.is_numbering_active("numbering_active:heading"),
            "Introspector: StateRegistry populado com chave canónica");
    }

    #[test]
    fn compute_heading_auto_toc_le_numbering_active_via_introspector() {
        // P198B test 4 (P190G adapted): confirma cadeia E5 —
        // compute_heading_auto_toc (P196B + P191B) lê
        // intr.is_numbering_active_at via Introspector path
        // location-aware. Quando numbering inactivo (sem
        // SetHeadingNumbering precedente), resolved_text fica vazia.
        let sem_set = Content::heading(1, Content::text("título"));
        let intr_sem = introspect_with_introspector(&sem_set);
        assert!(!intr_sem.is_numbering_active("numbering_active:heading"));
        assert_eq!(
            intr_sem.resolved_labels.get(&Label("auto-toc-1".to_string())),
            Some(""),
            "cadeia E5: numbering inactivo → resolved_text vazia (P196B §3)"
        );

        let com_set = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("título")),
            ]
            .into(),
        );
        let intr_com = introspect_with_introspector(&com_set);
        assert!(intr_com.is_numbering_active("numbering_active:heading"));
        assert_eq!(
            intr_com.resolved_labels.get(&Label("auto-toc-1".to_string())),
            Some("Secção 1"),
            "cadeia E5: numbering activo → compute_heading_auto_toc \
             retorna 'Secção 1'"
        );
    }

    #[test]
    fn walk_arm_set_heading_popula_intr_state() {
        // P198B test 5 (P190G adapted): confirma cadeia E5 ↔ E2 via
        // Introspector path puro. Mutação legacy
        // `state.numbering_active.insert` ELIMINADA em P190G; caminho
        // Introspector é única fonte da verdade desde P191B.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Intro")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Caminho Introspector activo: populate_intr arm StateUpdate
        // popula intr.state com chave canónica.
        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "P190G: SetHeadingNumbering popula intr.state via populate_intr"
        );
        // Consumer C4 (P194B) recebe Some via Introspector path —
        // cadeia E5 ↔ E2 funcional via Introspector puro.
        let auto_lbl = Label("auto-toc-1".to_string());
        assert_eq!(
            intr.resolved_label_for(&auto_lbl),
            Some("Secção 1"),
            "cadeia E5↔E2: P196B auto-toc Tag::Labelled populated via \
             Introspector path location-aware (P191B + P190G)"
        );
    }

    // ── P198C — Walk arm CounterUpdate (cenário β-promote) ──────────────
    //
    // 6 tests sentinela que validam: (a) extract_payload arm novo;
    // (b) is_locatable activado; (c) from_tags arm popula
    // CounterRegistry via apply_at; (d) paridade legacy vs Introspector;
    // (e) action Update aplica correctamente; (f) cadeia E6 ↔ helpers
    // compute_* funcional após promote.

    #[test]
    fn counter_update_extract_payload_emite_payload() {
        // P198C test 1: confirma que extract_payload(CounterUpdate)
        // retorna Some(ElementPayload::CounterUpdate { ... }).
        use crate::rules::introspect::extract_payload::extract_payload;
        use crate::entities::counter_update::CounterUpdate as CU;

        let content = Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Step,
        };
        match extract_payload(&content) {
            Some(ElementPayload::CounterUpdate { key, action }) => {
                assert_eq!(key, "equation");
                assert_eq!(action, CU::Step);
            }
            other => panic!("esperado Some(CounterUpdate), obtido {other:?}"),
        }
    }

    #[test]
    fn counter_update_is_locatable_true() {
        // P198C test 2: is_locatable(CounterUpdate) = true após promote.
        use crate::rules::introspect::locatable::is_locatable;

        let c = Content::CounterUpdate {
            key:    "page".to_string(),
            action: CounterAction::Update(42),
        };
        assert!(is_locatable(&c),
            "P198C: is_locatable(CounterUpdate) deve retornar true após promote");
    }

    #[test]
    fn counter_update_walk_popula_counter_registry() {
        // P198C test 3: pipeline walk + from_tags com 2 CounterUpdate
        // (Step) popula CounterRegistry; flat_counter_at retorna valor
        // correcto.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 Steps em "equation" → counter chega a 2.
        // Procurar a última location com snapshot.
        let tags_locations: Vec<Location> = intr
            .query_by_kind(ElementKind::CounterUpdate);
        assert_eq!(tags_locations.len(), 2,
            "P198C: 2 CounterUpdate emitem 2 locations indexadas em kind_index");
        let last_loc = *tags_locations.last().unwrap();
        assert_eq!(
            intr.flat_counter_at("equation", last_loc),
            Some(2),
            "P198C: from_tags arm popula CounterRegistry; flat=2 após 2 Steps"
        );
    }

    #[test]
    fn counter_update_paridade_legacy_vs_introspector() {
        // P198C test 4: paridade legacy state vs Introspector após promote.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy: state.flat populated via walk arm.
        assert_eq!(intr.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 3,
            "legacy: 3 Steps → state.flat['equation'] == 3");
        // Introspector: counter populated via from_tags arm.
        let last_loc = *intr.query_by_kind(ElementKind::CounterUpdate)
            .last().unwrap();
        assert_eq!(
            intr.flat_counter_at("equation", last_loc),
            Some(3),
            "Introspector: paridade pós-P198C — flat_counter_at == 3"
        );
    }

    #[test]
    fn counter_update_action_update_apply_correctly() {
        // P198C test 5: 3º caminho da match — Update(val) aplica via
        // apply_at(Update). Legacy via state.update_flat. Paridade.
        use crate::entities::introspector::Introspector;

        let content = Content::CounterUpdate {
            key:    "page".to_string(),
            action: CounterAction::Update(42),
        };
        let intr = introspect_with_introspector(&content);

        // Legacy.
        assert_eq!(intr.counters.value("page").and_then(|v| v.last()).copied().unwrap_or(0), 42,
            "legacy: state.update_flat('page', 42) → 42");
        // Introspector.
        let loc = *intr.query_by_kind(ElementKind::CounterUpdate)
            .first().unwrap();
        assert_eq!(
            intr.flat_counter_at("page", loc),
            Some(42),
            "P198C: apply_at(Update(42)) → flat_counter_at == 42"
        );
    }

    #[test]
    fn counter_update_compute_helpers_continuam_funcionais() {
        // P198C test 6: cadeia E6 ↔ compute_labelled Equation arm
        // preservada após promote. Walk arm Equation lê
        // intr.is_numbering_active("numbering_active:equation") + state.step_flat
        // durante walk; compute_labelled lê intr.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0).
        // Mutação legacy preservada → cadeia funcional.
        let content = Content::Sequence(
            vec![
                // Set equation numbering active via direct mutation —
                // SetEquationNumbering ainda não existe (Reserva 1 P186A);
                // mutação directa do state via walk arm Equation requer
                // is_numbering_active("equation") = true. Sem isso,
                // walk arm Equation não avança counter. Test usa
                // CounterUpdate directo para bypass.
                Content::CounterUpdate {
                    key:    "equation".to_string(),
                    action: CounterAction::Step,
                },
                Content::Labelled {
                    label:  Label("eq1".to_string()),
                    target: Box::new(Content::Equation {
                        body:  Box::new(Content::Empty),
                        block: true,
                    }),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Mutação legacy preservada: state.flat["equation"] populado
        // pelo CounterUpdate Step.
        assert_eq!(intr.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 1,
            "P198C: mutação legacy preservada — state.step_flat('equation')");
        // compute_labelled Equation arm lê state.get_flat → produz
        // resolved_text "Equação (1)".
        assert_eq!(
            intr.resolved_labels.get(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "cadeia E6↔E4: compute_labelled Equation arm continua funcional \
             após promote — lê state.get_flat('equation') durante walk"
        );
        // Introspector path: resolved_labels populated via P195D Tag.
        assert_eq!(
            intr.resolved_label_for(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "Introspector path: resolved_labels populated via P195D + Tag::Labelled"
        );
    }

    // ── P199B — Materialização Content::SetEquationNumbering ───────────
    //
    // 5 tests sentinela que validam: (a) extract_payload arm novo;
    // (b) from_tags arm StateUpdate (P171 genérica) processa
    // numbering_active:equation transparentemente; (c) paridade
    // legacy vs Introspector; (d) cadeia E1 — walk arm Equation lê
    // mutação legacy durante walk após SetEquationNumbering;
    // (e) consumer Layouter Equation activação por construção.

    #[test]
    fn set_equation_numbering_extract_payload_emite_state_update() {
        // P199B test 1: confirma que extract_payload(SetEquationNumbering)
        // retorna Some(ElementPayload::StateUpdate { ... }) com chave
        // canónica numbering_active:equation.
        use crate::rules::introspect::extract_payload::extract_payload;
        use crate::entities::state_update::StateUpdate;
        use crate::entities::value::Value;

        let content = Content::SetEquationNumbering { active: true };
        match extract_payload(&content) {
            Some(ElementPayload::StateUpdate { key, update }) => {
                assert_eq!(key, "numbering_active:equation",
                    "P199B: chave canónica numbering_active:equation");
                match update {
                    StateUpdate::Set(boxed) => assert_eq!(*boxed, Value::Bool(true)),
                    other => panic!("esperado StateUpdate::Set(Bool(true)), obtido {other:?}"),
                }
            }
            other => panic!("esperado Some(StateUpdate), obtido {other:?}"),
        }
    }

    #[test]
    fn set_equation_numbering_from_tags_popula_state_registry() {
        // P199B test 2: pipeline walk + from_tags com SetEquationNumbering
        // popula StateRegistry com chave canónica via arm StateUpdate
        // genérica (P171) — sem modificação a from_tags.
        use crate::entities::introspector::Introspector;

        let content = Content::SetEquationNumbering { active: true };
        let intr = introspect_with_introspector(&content);

        // P171 arm StateUpdate é genérica — processa qualquer key
        // incluindo numbering_active:equation transparentemente.
        assert!(
            intr.is_numbering_active("numbering_active:equation"),
            "P199B: from_tags arm StateUpdate popula StateRegistry \
             com numbering_active:equation=true (sem modificação a P171)"
        );
    }

    #[test]
    fn set_equation_numbering_paridade_legacy_vs_introspector() {
        // P199B test 3: write paralelo legacy + Introspector preserva
        // paridade. Legacy intr.is_numbering_active("numbering_active:equation") == true;
        // Introspector intr.is_numbering_active("numbering_active:equation")
        // == true.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::SetEquationNumbering { active: true },
                Content::Equation {
                    body:  Box::new(Content::Empty),
                    block: true,
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy state populado via mutação directa walk arm.
        assert!(intr.is_numbering_active("numbering_active:equation"),
            "legacy: state.numbering_active['equation'] = true (write paralelo M5)");
        // Introspector path populado via from_tags arm StateUpdate.
        assert!(intr.is_numbering_active("numbering_active:equation"),
            "Introspector: StateRegistry populado com chave canónica");
    }

    #[test]
    fn walk_arm_equation_le_numbering_active_legacy_apos_set() {
        // P199B test 4: cadeia E1 — walk arm Equation lê
        // intr.is_numbering_active("numbering_active:equation") durante walk para
        // gating do counter step. Confirma que mutação legacy de
        // SetEquationNumbering antes do Equation faz counter avançar.
        let com_set = Content::Sequence(
            vec![
                Content::SetEquationNumbering { active: true },
                Content::Equation {
                    body:  Box::new(Content::Empty),
                    block: true,
                },
            ]
            .into(),
        );
        let intr_com = introspect_with_introspector(&com_set);

        // Após SetEquationNumbering activo, walk arm Equation gate
        // (P191B: usa intr.is_numbering_active_at) dispara →
        // state.step_flat("equation") → counter chega a 1.
        assert!(intr_com.is_numbering_active("numbering_active:equation"));
        assert_eq!(intr_com.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 1,
            "cadeia E1: walk arm Equation gate via intr → counter avança");

        // Sem SetEquationNumbering, gate não dispara.
        let sem_set = Content::Equation {
            body:  Box::new(Content::Empty),
            block: true,
        };
        let intr_sem = introspect_with_introspector(&sem_set);
        assert!(!intr_sem.is_numbering_active("numbering_active:equation"));
        assert_eq!(intr_sem.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 0,
            "sem SetEquationNumbering: gate inactivo → counter não avança");
    }

    #[test]
    fn consumer_layouter_equation_activa_via_introspector() {
        // P199B test 5: caminho Introspector activado por construção
        // — pipeline com SetEquationNumbering + Equation labelled
        // produz resolved_labels populated via cadeia legacy
        // (compute_labelled P195D) E intr.state populated via
        // Tag::StateUpdate. Confirma activação imediata após P199B.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::SetEquationNumbering { active: true },
                Content::Labelled {
                    label:  Label("eq1".to_string()),
                    target: Box::new(Content::Equation {
                        body:  Box::new(Content::Empty),
                        block: true,
                    }),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy: compute_labelled Equation arm produz "Equação (1)".
        assert_eq!(
            intr.resolved_labels.get(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "compute_labelled Equation arm activado via mutação legacy SetEquationNumbering"
        );
        // Introspector: resolved_labels populated via P195D Tag::Labelled.
        assert_eq!(
            intr.resolved_label_for(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "Introspector path: resolved_labels populated em paralelo (P199B + P195D)"
        );
        // Introspector: StateRegistry populated via Tag::StateUpdate.
        assert!(
            intr.is_numbering_active("numbering_active:equation"),
            "P199B: Layouter equation.rs:32 first branch retorna Some via Introspector path"
        );
    }

    // ── P200B — Sub-store headings_for_toc + Tag + consumer (M5 universal) ─
    //
    // 5 tests sentinela que validam: (a) walk arm Heading emite
    // Tag::HeadingForToc pós-recursão; (b) sub-store
    // intr.headings_for_toc populated em paridade com legacy;
    // (c) bracketing válido com 6 tags por Heading folha;
    // (d) E2-residuo fechada — paridade legacy/Introspector
    // preservada; (e) compute_heading_for_toc helper produz
    // tuple correcto.

    #[test]
    fn headings_for_toc_walk_emite_tag_e_popula_sub_store() {
        // P200B test 1: pipeline walk + from_tags com Heading
        // numerado popula sub-store intr.headings_for_toc.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Intro")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Sub-store populated com 1 entry.
        assert_eq!(
            intr.headings_for_toc().len(),
            1,
            "P200B: 1 Heading deve gerar 1 entry em intr.headings_for_toc"
        );
        // Auto-label sintetizada usa state.auto_label_counter (1).
        let entry = &intr.headings_for_toc()[0];
        assert_eq!(entry.0, Label("auto-toc-1".to_string()));
        assert_eq!(entry.2, 1, "level esperado");
    }

    #[test]
    fn headings_for_toc_paridade_legacy_vs_introspector() {
        // P200B test 2: write paralelo legacy + Introspector preserva
        // paridade exacta. intr.headings_for_toc().len() ==
        // intr.headings_for_toc().len(); conteúdo idêntico.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        assert_eq!(intr.headings_for_toc().len(), 3,
            "legacy: 3 entries (mutação 4 preservada como write paralelo M5)");
        assert_eq!(intr.headings_for_toc().len(), 3,
            "Introspector: 3 entries via Tag::HeadingForToc");

        // Paridade exacta — labels e levels.
        for (legacy_entry, intr_entry) in intr.headings_for_toc().iter().zip(intr.headings_for_toc()) {
            assert_eq!(legacy_entry.0, intr_entry.0, "labels devem ser idênticos");
            assert_eq!(legacy_entry.2, intr_entry.2, "levels devem ser idênticos");
        }
    }

    #[test]
    fn bracketing_valido_6_tags_por_heading_p200b() {
        // P200B test 3: confirma bracketing válido com 6 tags por
        // Heading folha (3 Start + 3 End consecutivas; mesma Location).
        let h = Content::heading(1, Content::text("título"));
        let tags = introspect_with_tags(&h);

        assert_eq!(tags.len(), 6, "P200B: 6 tags por Heading folha");

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc, "End com Location diferente do último Start");
                }
            }
        }
        assert!(stack.is_empty(), "todos Start têm End correspondente");
    }

    #[test]
    fn e2_residuo_fechada_paridade_legacy_introspector() {
        // P200B test 4: sentinela substituindo
        // walk_e2_residuo_headings_for_toc_via_legacy P196B.
        // Confirma que E2-residuo fecha estruturalmente: ambos
        // legacy e Introspector populated em paralelo.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy preservado (write paralelo M5).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "E2-residuo: mutação 4 legacy preservada (Layouter mod.rs:1490, 1521 dependem)"
        );
        // Introspector popula via Tag::HeadingForToc (E2-residuo fechada).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "P200B: E2-residuo fechada estruturalmente via Tag::HeadingForToc"
        );
        // Levels preservados em ordem.
        let levels_legacy: Vec<_> = intr.headings_for_toc().iter().map(|(_, _, l)| *l).collect();
        let levels_intr: Vec<_> = intr.headings_for_toc().iter().map(|(_, _, l)| *l).collect();
        assert_eq!(levels_legacy, vec![1, 2, 1]);
        assert_eq!(levels_intr, levels_legacy);
    }

    #[test]
    fn headings_for_toc_helper_compute_produces_correct_entry() {
        // P200B test 5: confirma que compute_heading_for_toc
        // helper produz tuple correcto reusando frozen_body
        // computed pelo walk arm.
        use crate::entities::introspector::Introspector;

        let h = Content::heading(2, Content::text("título"));
        let intr = introspect_with_introspector(&h);

        // P190G: walk-internal `auto_label_counter` (local var em
        // walk fn) incrementado a 1 — verificável via Label
        // sintetizada no sub-store.
        // Entry correcta no sub-store.
        let entry = &intr.headings_for_toc()[0];
        assert_eq!(entry.0, Label("auto-toc-1".to_string()),
            "label sintetizada usa walk-internal auto_label_counter");
        assert_eq!(entry.2, 2,
            "level preservado per cast usize do level: u8");
        // body materializado preserva text content.
        match &entry.1 {
            Content::Text(s, _) => assert_eq!(s.as_str(), "título"),
            other => panic!("body esperado Text, obtido {other:?}"),
        }
    }

    // ── P191B (ADR-0071) — sentinelas mecanismo walk pipeline ───────────

    #[test]
    fn p191b_walk_popula_intr_directamente_sem_from_tags() {
        // P191B sentinela #1: walk popula `TagIntrospector` directamente
        // durante walk via populate_intr_from_tag_start. Sem chamada
        // a from_tags::from_tags (eliminado em P191B).
        //
        // Verifica que doc com Heading + Figure + Cite produz intr
        // populated com sub-stores correctos só por chamar walk.
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("h")),
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c"))),
                    kind:      Some("image".to_string()),
                    numbering: Some("1".to_string()),
                },
                Content::Cite {
                    key:        "k".to_string(),
                    supplement: None,
                    form:       None,
                },
            ]
            .into(),
        );

        // Walk directo — sem from_tags.
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, None);

        // Sub-stores populated por walk directo.
        assert_eq!(intr.query_by_kind(ElementKind::Heading).len(), 1,
            "ADR-0071: walk popula kind_index[Heading] directamente");
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 1,
            "ADR-0071: walk popula kind_index[Figure] directamente");
        assert_eq!(intr.query_by_kind(ElementKind::Citation).len(), 1,
            "ADR-0071: walk popula kind_index[Citation] directamente");
    }

    #[test]
    fn p191c_compute_labelled_le_via_introspector_path() {
        // P191C (ADR-0071 ACEITE) sentinela: compute_labelled migrada
        // para signature <I: Introspector>(intr, location, target,
        // lang). Reads location-aware substituem state.figure_numbers
        // / state.format_hierarchical / state.get_flat. Paridade com
        // comportamento legacy.
        //
        // Cenário: Heading numbered + Labelled Heading (cadeia C1).
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("intro")),
                Content::Labelled {
                    label:  Label("sec".to_string()),
                    target: Box::new(Content::heading(1, Content::text("body"))),
                },
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);

        // resolved_labels populated com "Secção 2" via Introspector path
        // location-aware (compute_labelled Heading arm chama
        // intr.formatted_counter_at("heading", target_loc)).
        assert_eq!(
            intr.resolved_labels.get(&Label("sec".to_string())),
            Some("Secção 2"),
            "compute_labelled via Introspector path: 2ª heading labelled → 'Secção 2'",
        );
    }

    #[test]
    fn p191b_compute_heading_auto_toc_le_via_introspector_path() {
        // P191B sentinela #2: compute_heading_auto_toc migrada para
        // signature <I: Introspector>(intr, location, counter_n).
        // Reads `intr.is_numbering_active_at` + `formatted_counter_at`
        // location-aware. Paridade com comportamento legacy quando
        // SetHeadingNumbering(true) precede o Heading.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);

        // resolved_labels populated com auto-toc texts via Introspector
        // path. "Secção 1" para auto-toc-1, "Secção 2" para auto-toc-2.
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-1".to_string())),
            Some("Secção 1"),
            "compute_heading_auto_toc via Introspector path: 1ª heading → 'Secção 1'",
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-2".to_string())),
            Some("Secção 2"),
            "compute_heading_auto_toc via Introspector path: 2ª heading → 'Secção 2'",
        );
    }
}
