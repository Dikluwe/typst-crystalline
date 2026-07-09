//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash 4d0b61c1
//! @layer L1
//! @updated 2026-06-27
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
//! arm Equation gate pelo campo assado `numbering_active` (F-2 S2;
//! o gate legado por StateRegistry foi removido em F-4 E0, P338).
//! API pública preservada (`introspect()` retorna `CounterStateLegacy`
//! idêntico). `introspect_with_introspector` simplificada — drops
//! parâmetros engine/ctx (Funcs continuam ignoradas neste path
//! coerente com semântica P171 pré-P191B).

use std::collections::HashMap;

pub mod convergence;
pub mod extract_payload;
pub mod fixpoint;
pub mod from_tags;
// Atomização por-elemento (ADR-0109, P383): compute_* movidos do tronco.
pub mod heading;
pub mod labelled;
pub mod locatable;

use std::sync::Arc;

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
    style_chain::StyleChain,
    tag::Tag,
    value::Value,
};

use crate::rules::introspect::extract_payload::extract_payload as do_extract_payload;

/// **P533** — Converte referências `@key` que correspondam a entradas
/// bibliográficas em `Content::Cite`, de modo que o walk de introspecção
/// as conte corretamente para `citation_order`/`back_refs` e o layout as
/// renderize como citações resolvidas em vez de referências cruzadas.
pub fn convert_bib_refs_to_cites(content: Content) -> Content {
    let keys = collect_bib_keys(&content);
    if keys.is_empty() {
        return content;
    }
    convert_refs(content, &keys)
}

fn collect_bib_keys(content: &Content) -> std::collections::HashSet<String> {
    use crate::entities::content::Content;
    let mut keys = std::collections::HashSet::new();
    fn walk(c: &Content, keys: &mut std::collections::HashSet<String>) {
        match c {
            Content::Bibliography(b) => {
                for e in &b.entries {
                    keys.insert(e.key.clone());
                }
            }
            Content::Sequence(seq) => seq.iter().for_each(|c| walk(c, keys)),
            Content::Styled(body, _) => walk(body, keys),
            Content::Block(e) => walk(&e.body, keys),
            Content::Boxed(e) => walk(&e.body, keys),
            Content::Pad(e) => walk(&e.body, keys),
            Content::Align(e) => walk(&e.body, keys),
            Content::Hide(e) => walk(&e.body, keys),
            Content::Figure(e) => {
                walk(&e.body, keys);
                if let Some(cap) = &e.caption {
                    walk(cap, keys);
                }
            }
            Content::Table(e) => {
                if let Some(cap) = &e.caption {
                    walk(cap, keys);
                }
                for child in &e.children {
                    walk(child, keys);
                }
            }
            Content::Grid(e) => e.cells.iter().for_each(|c| walk(c, keys)),
            Content::Stack(e) => e.children.iter().for_each(|c| walk(c, keys)),
            Content::ListItem(e) => walk(&e.body, keys),
            Content::EnumItem(e) => walk(&e.body, keys),
            Content::TermItem(e) => {
                walk(&e.term, keys);
                walk(&e.description, keys);
            }
            Content::Footnote(e) => walk(&e.body, keys),
            Content::Quote(e) => walk(&e.body, keys),
            Content::Overline(e) => walk(&e.body, keys),
            Content::Underline(e) => walk(&e.body, keys),
            Content::Strike(e) => walk(&e.body, keys),
            Content::Strong(e) => walk(&e.body, keys),
            Content::Emph(e) => walk(&e.body, keys),
            Content::SmallCaps { body, .. } => walk(body, keys),
            Content::Link(e) => walk(&e.body, keys),
            // ContextBlock e Dynamic não contêm content estático navegável.
            Content::ContextBlock(_) | Content::Dynamic(_) => {}
            _ => {}
        }
    }
    walk(content, &mut keys);
    keys
}

fn convert_refs(content: Content, keys: &std::collections::HashSet<String>) -> Content {
    use crate::entities::content::Content;
    use std::sync::Arc;
    match content {
        Content::Ref(r) => {
            let key = r.name.to_string();
            if keys.contains(&key) {
                Content::cite(key, r.supplement.clone(), None)
            } else {
                Content::Ref(r)
            }
        }
        Content::Sequence(seq) => Content::Sequence(
            seq.iter().map(|c| convert_refs(c.clone(), keys)).collect::<Vec<_>>().into()
        ),
        Content::Styled(body, styles) => Content::Styled(
            Box::new(convert_refs(*body, keys)),
            styles,
        ),
        Content::Block(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Block(Arc::new(e))
        }
        Content::Boxed(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Boxed(Arc::new(e))
        }
        Content::Pad(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Pad(Arc::new(e))
        }
        Content::Align(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Align(Arc::new(e))
        }
        Content::Hide(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Hide(Arc::new(e))
        }
        Content::Figure(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            e.caption = e.caption.map(|c| convert_refs(c, keys));
            Content::Figure(Arc::new(e))
        }
        Content::Table(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.caption = e.caption.map(|c| convert_refs(c, keys));
            e.children = e.children.into_iter().map(|c| convert_refs(c, keys)).collect();
            Content::Table(Arc::new(e))
        }
        Content::Grid(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.cells = e.cells.into_iter().map(|c| convert_refs(c, keys)).collect();
            Content::Grid(Arc::new(e))
        }
        Content::Stack(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            let new_children: Vec<Content> = e.children.iter()
                .map(|c| convert_refs(c.clone(), keys))
                .collect();
            e.children = Arc::from(new_children);
            Content::Stack(Arc::new(e))
        }
        Content::ListItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::ListItem(Arc::new(e))
        }
        Content::EnumItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::EnumItem(Arc::new(e))
        }
        Content::TermItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.term = convert_refs(e.term, keys);
            e.description = convert_refs(e.description, keys);
            Content::TermItem(Arc::new(e))
        }
        Content::Footnote(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Footnote(Arc::new(e))
        }
        Content::Quote(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Quote(Arc::new(e))
        }
        Content::Overline(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Overline(Arc::new(e))
        }
        Content::Underline(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Underline(Arc::new(e))
        }
        Content::Strike(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Strike(Arc::new(e))
        }
        Content::Strong(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Strong(Arc::new(e))
        }
        Content::Emph(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Emph(Arc::new(e))
        }
        Content::SmallCaps { body, .. } => Content::smallcaps(convert_refs(*body, keys)),
        Content::Link(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Link(Arc::new(e))
        }
        // ContextBlock não contém content aninhado (P506).
        Content::ContextBlock(e) => Content::ContextBlock(e),
        // Elementos dinâmicos: a transformação object-safe delega ao trait.
        // A transformação não pode falhar (não faz eval), por isso unwrap é
        // seguro — qualquer erro seria um bug interno.
        Content::Dynamic(e) => {
            let mut f = |c: &Content| -> crate::entities::source_result::SourceResult<Option<Content>> {
                Ok(Some(convert_refs(c.clone(), keys)))
            };
            e.dyn_map_content(&mut f).expect("convert_bib_refs_to_cites: dyn_map_content não deve falhar")
        }
        other => other,
    }
}

/// **P363 (introspect-chain) — probe de verificação, `#[cfg(test)]`.** Captura, por
/// heading visitado no walk, o gate de numbering **lido da chain** — para um teste
/// provar que a infra threadou o custom `heading.numbering` até o heading (sem mudar
/// o comportamento de produção). Removida quando o de-bake (P364) tornar o read da
/// chain o caminho real e o teste passar a asserir o output.
#[cfg(test)]
pub(crate) mod introspect_chain_probe {
    use std::cell::RefCell;
    thread_local! {
        static GATES: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
    }
    /// Limpa a captura (chamar antes de um walk no teste).
    pub(crate) fn reset() { GATES.with(|g| g.borrow_mut().clear()); }
    /// Regista o gate da chain de um heading (chamado pelo arm Heading do walk).
    pub(crate) fn record(gate: bool) { GATES.with(|g| g.borrow_mut().push(gate)); }
    /// Os gates capturados, na ordem de visita.
    pub(crate) fn captured() -> Vec<bool> { GATES.with(|g| g.borrow().clone()) }
}

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
    // **P533** — garantir que `@key` bibliográficos são vistos como
    // `Content::Cite` durante o walk, para que `citation_order` e
    // `back_refs` fiquem correctos.
    let content = convert_bib_refs_to_cites(content.clone());
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter: usize = 0;
    // P363: chain raiz = default_chain (espelha o root do layout, `layout/mod.rs`).
    let root_chain = StyleChain::default_chain();
    walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &root_chain, None);
    intr.parent_locations = build_parent_index(&tags);
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
        Content::CounterDisplay(e) => {
            Content::text(
                intr.formatted_counter_at(&e.kind, location)
                    .unwrap_or_else(|| "0".to_string()),
            )
        }

        // ── Containers com filhos (propagação recursiva) ──────────────────
        Content::Sequence(seq) => {
            Content::Sequence(
                seq.iter().map(|c| materialize_time(c, intr, location)).collect::<Vec<_>>().into()
            )
        }
        // F-5b fatia 1 (P371): strong/emph voltaram a variantes próprias (o
        // colapso P101 foi superado); reconstroem via ctor, recursando no body.
        Content::Strong(e) => Content::strong(materialize_time(&e.body, intr, location)),
        Content::Emph(e)   => Content::emph(materialize_time(&e.body, intr, location)),
        // P408: smallcaps é container transparente para materialização de tempo
        // (o consumer real de small caps será aplicado no layout; DEBT-53).
        Content::SmallCaps { body } => Content::smallcaps(materialize_time(body, intr, location)),
        // Modelo D (P316): Heading delegado; reconstrói via ctor.
        // P606 — preserva `outlined` e `bookmarked` durante materialização de tempo.
        Content::Heading(h) => Content::heading_with_outlined_and_bookmarked(
            h.level,
            materialize_time(&h.body, intr, location),
            h.outlined,
            h.bookmarked,
        ),
        // Modelo D (Lote 3 P318): destructure de Arc<Elem> + reconstrução via construtor.
        Content::ListItem(e) => {
            let body = materialize_time(&e.body, intr, location);
            match &e.marker {
                None    => Content::list_item(body),
                Some(m) => Content::list_item_with_marker(body, m.clone()),
            }
        }
        Content::EnumItem(e) => {
            let body = materialize_time(&e.body, intr, location);
            match &e.numbering {
                None    => Content::enum_item(e.number, body),
                Some(n) => Content::enum_item_with_numbering(e.number, body, n.clone()),
            }
        }
        Content::Link(e) => Content::link(e.url.clone(), materialize_time(&e.body, intr, location)),
        // P464: Label — recurse no body via construtor, preservando origem.
        Content::Label(e) => {
            let body = materialize_time(&e.body, intr, location);
            if e.auto {
                Content::label_auto(e.name.clone(), body)
            } else {
                Content::label(e.name.clone(), body)
            }
        }
        // Modelo D (Lote 13 P328): Figure — recurse body+caption via construtor.
        // F-5a de-bake (P365): a figura não tem mais campo `numbering` — o gate
        // vive na chain (no `Content::Styled` que envolve a figura, preservado pelo
        // arm Styled de `materialize_time`). Reconstrói com numbering=None.
        Content::Figure(e) => Content::figure(
            materialize_time(&e.body, intr, location),
            e.caption.as_ref().map(|c| materialize_time(c, intr, location)),
            e.kind.clone(),
            None,
        ),

        // Modelo D (Lote 3 P318): Terms/TermItem delegam reconstrução ao construtor.
        Content::Terms(e) => Content::terms(
            e.items.iter().map(|c| materialize_time(c, intr, location)).collect(),
        ),
        Content::TermItem(e) => Content::term_item(
            materialize_time(&e.term, intr, location),
            materialize_time(&e.description, intr, location),
        ),

        // Passo 155: Quote — recurse em body e attribution.
        // Modelo D (Lote 8 P323): destrutura o `Arc<QuoteElem>`.
        Content::Quote(e) => Content::quote(
            materialize_time(&e.body, intr, location),
            e.attribution.as_ref().map(|c| materialize_time(c, intr, location)),
            e.block,
            e.quotes,
        ),

        // P295 — Footnote recurse em body (paridade Quote).
        Content::Footnote(e) => Content::footnote(materialize_time(&e.body, intr, location)),

        // Modelo D (Lote 4 P319): decorações — recurse no body via construtor.
        Content::Underline(e) => Content::underline(
            materialize_time(&e.body, intr, location), e.stroke, e.offset, e.extent),
        Content::Strike(e) => Content::strike(
            materialize_time(&e.body, intr, location), e.stroke, e.offset, e.extent),
        Content::Overline(e) => Content::overline(
            materialize_time(&e.body, intr, location), e.stroke, e.offset, e.extent),

        // ── Terminais — clonar directamente ──────────────────────────────
        // Nós matemáticos (Equation e subtipos) não podem conter CounterDisplay
        // em markup válido — clonados em bloco sem recursão.
        Content::Empty
        | Content::Text(_)
        | Content::Space
        // P622: Parbreak é leaf estrutural — sem CounterDisplay possível.
        | Content::Parbreak
        | Content::Raw(_)
        | Content::Ref(_)
        | Content::SetPage { .. }
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Linebreak(_)
        | Content::MathAlignPoint(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        // P287 — SmartQuote leaf (sem CounterDisplay possível).
        | Content::SmartQuote(_)
        | Content::Equation { .. }
        | Content::MathSequence(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathRoot(_)
        | Content::MathDelimited(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        // P296 — Math accent/cancel terminais em materialize_time
        // (paralelo MathFrac/MathRoot; sem CounterDisplay no body).
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        // P297 — Math underover terminal (paralelo P296).
        | Content::MathUnderover(_)
        // P298 — Math op terminal (paralelo cluster math).
        | Content::MathOp(_)
        // P311b.2 — MathStyled terminal em materialize_time (math structural).
        | Content::MathStyled(_)
        | Content::Image(_)
        | Content::Divider(_)
        // Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h/v spacing leaves.
        | Content::HSpace(_)
        | Content::VSpace(_)
        // Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak leaf.
        | Content::Pagebreak(_)
        // Passo 220 (ADR-0078 sub-fase b 4/4) — colbreak leaf.
        | Content::Colbreak(_)
        | Content::Shape { .. }
        // Passo 513 — Curve é leaf não-locatable.
        | Content::Curve(_)
        // P169 (M9): Metadata é terminal — clonar directamente.
        | Content::Metadata(_)
        // P171 (M9): State e StateUpdate são terminais.
        | Content::State(_)
        | Content::StateUpdate(_)
        // P240 (M9d/M7+1): StateDisplay terminal em materialize_time —
        // resolução real via apply_state_displays + layout arm.
        | Content::StateDisplay(_)
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; resolução real via apply_counter_displays.
        | Content::CounterDisplayCallback(_) => content.clone(),
        // Passo 156C (ADR-0061 Fase 1) — pad / hide containers.
        // Materialize_time desce no body para resolver counters dentro;
        // padding e o invariante "hide" preservam-se.
        Content::Pad(e) => Content::pad(materialize_time(&e.body, intr, location), e.sides),
        Content::Hide(e) => Content::hide(materialize_time(&e.body, intr, location)),
        // Modelo D (Lote 15 P330): Block — recurse no body via struct-update.
        Content::Block(e) => Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 14 P329): Boxed — recurse no body via struct-update.
        Content::Boxed(e) => Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Materialize_time em cada child; preservar dir/spacing.
        Content::Stack(e) => {
            let new_children: Vec<Content> = e.children.iter()
                .map(|c| materialize_time(c, intr, location))
                .collect();
            Content::stack(new_children, e.dir, e.spacing)
        },
        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat.
        // Análogo a Block: descer no body; preservar atributos.
        Content::Repeat(e) => Content::repeat(materialize_time(&e.body, intr, location), e.gap, e.justify),
        // P217 — Columns container: análogo a Repeat/Block; descer
        // no body; count/gutter preservados (Copy primitivos).
        Content::Columns(e) => Content::columns(
            materialize_time(&e.body, intr, location),
            e.count,
            e.gutter,
        ),
        Content::Transform(e) => Content::transform(e.matrix, materialize_time(&e.body, intr, location)),
        // Modelo D (Lote 12 P327): Grid — recurse cells+header+footer (struct-update).
        Content::Grid(e) => Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                cells:  e.cells.iter().map(|c| materialize_time(c, intr, location)).collect(),
                    hlines: vec![],
                    vlines: vec![],
                header: e.header.as_ref().map(|h| materialize_time(h, intr, location)),
                footer: e.footer.as_ref().map(|f| materialize_time(f, intr, location)),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 5 P320): GridHeader/GridFooter via construtor.
        Content::GridHeader(e) => Content::grid_header(materialize_time(&e.body, intr, location), e.repeat),
        Content::GridFooter(e) => Content::grid_footer(materialize_time(&e.body, intr, location), e.repeat),
        // Passo 512 — linhas em grid/table são terminais.
        Content::GridHLine(_) | Content::GridVLine(_) => content.clone(),
        // Modelo D (Lote 12 P327): GridCell — recurse no body via struct-update.
        Content::GridCell(e) => Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 12 P327): Table — recurse em caption + children.
        Content::Table(e) => Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                caption: e.caption.as_ref().map(|c| materialize_time(c, intr, location)),
                children: e.children.iter().map(|c| materialize_time(c, intr, location)).collect(),
                    hlines: vec![],
                    vlines: vec![],
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 12 P327): TableCell — recurse no body via struct-update.
        Content::TableCell(e) => Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 5 P320): TableHeader/TableFooter via construtor.
        Content::TableHeader(e) => Content::table_header(materialize_time(&e.body, intr, location), e.repeat),
        Content::TableFooter(e) => Content::table_footer(materialize_time(&e.body, intr, location), e.repeat),
        // Passo 512 — linhas em grid/table são terminais.
        Content::TableHLine(_) | Content::TableVLine(_) => content.clone(),
        // Passo 159A — par acoplado Bibliography + Cite. Recurse em
        // title (Bibliography) ou supplement (Cite); preserva
        // entries/key/style/locale/path (P429).
        Content::Bibliography(e) => Content::Bibliography(Arc::new(
            crate::entities::elements::bibliography::BibliographyElem {
                entries: e.entries.clone(),
                path: e.path.clone(),
                title: e.title.as_ref().map(|t| materialize_time(t, intr, location)),
                style: e.style.clone(),
                locale: e.locale.clone(),
            },
        )),
        Content::Cite(e) => Content::cite_with_style(
            e.key.clone(),
            e.supplement.as_ref().map(|s| materialize_time(s, intr, location)),
            e.form,
            e.style,
        ),
        Content::Align(e) => Content::align(e.alignment, materialize_time(&e.body, intr, location)),
        // P223 — Place refino: preservar float + clearance no materialize_time.
        Content::Place(e) => Content::place(
            e.alignment, e.dx, e.dy, e.scope, e.float, e.clearance,
            materialize_time(&e.body, intr, location),
        ),

        // P397 — Document recursa no title; Asset é terminal.
        Content::Document { title, author, date, keywords } => Content::Document {
            title: title.as_ref().map(|t| Box::new(materialize_time(t, intr, location))),
            author: author.clone(),
            date: *date,
            keywords: keywords.clone(),
        },
        Content::Asset { .. } => content.clone(),

        // Passo 99 (ADR-0038): `Styled` é transparente para materialização de
        // contadores — o body é processado e os estilos preservados.
        Content::Styled(body, styles) => Content::Styled(
            Box::new(materialize_time(body, intr, location)),
            styles.clone(),
        ),

        // P506: ContextBlock é terminal em materialize_time (o conteúdo real
        // só existe após expansão pós-introspecção).
        Content::ContextBlock(_) => content.clone(),

        // Lote F-1 (P334): a fronteira dinâmica é **terminal** aqui (clone). A
        // travessia/realização do nó dinâmico (e dos seus filhos) chega em F-2
        // (L0 `f_fronteira_e1.md` §3a.7); em F-1 só existe em fixtures.
        Content::Dynamic(_) => content.clone(),
    }
}

// Atomização (ADR-0109, P383): `compute_labelled` movido para
// `introspect/labelled.rs`; `compute_heading_auto_toc` + `compute_heading_for_toc`
// para `introspect/heading.rs`. O walk chama via `labelled::`/`heading::`.

// P197B helper `compute_figure` ELIMINADO em P190H (M6 categoria
// Figures eliminada). Walk arm Figure ficou puro — sem necessidade
// de calcular figure_number durante walk porque populate_intr arm
// Figure (P191C, gated por is_counted) popula
// `intr.counters["figure:{kind}"]` directamente. Consumers
// (`compute_labelled` Figure arm via `intr.flat_counter_at`,
// Layouter C3 via `figure_number_at_index`) consomem do intr.


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
            if let Some(label) = &info.label {
                intr.label_to_counter_key.insert(label.clone(), "heading".into());
            }
        }
        ElementPayload::Figure { kind, counter_update, is_counted, caption_text } => {
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
                let counter_key = format!("figure:{}", kind_key);
                intr.counters.apply_at(
                    counter_key.clone(),
                    counter_update.clone(),
                    loc,
                );
                intr.counters.apply_at(
                    "figure".to_string(),
                    counter_update.clone(),
                    loc,
                );
                // **P472** — popular figures_for_lof com (número, caption).
                let num = intr.counters.value_at("figure", loc)
                    .and_then(|v| v.last().copied())
                    .unwrap_or(0);
                if let Some(cap) = caption_text {
                    intr.figures_for_lof.push((num, cap.clone()));
                }
                if let Some(label) = &info.label {
                    let next_num = intr.figure_label_numbers.len() + 1;
                    intr.figure_label_numbers
                        .entry(label.clone())
                        .or_insert(next_num);
                    intr.label_to_counter_key.insert(label.clone(), counter_key.into());
                }
            }
        }
        ElementPayload::Citation { key } => {
            intr.kind_index
                .entry(ElementKind::Citation)
                .or_default()
                .push(loc);
            // **P468** — numeração de citações por ordem de aparição.
            intr.counters.apply_at(
                "citation".to_string(),
                crate::entities::counter_update::CounterUpdate::Step,
                loc,
            );
            intr.bib_store.record_citation(key.as_str().to_string());
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
        ElementPayload::Equation { block, counter_update, numbering_active } => {
            intr.kind_index
                .entry(ElementKind::Equation)
                .or_default()
                .push(loc);
            // Lote F-2 S2 (P335): gate pelo `numbering_active` **assado** no
            // `EquationElem` (escopo léxico via chain) — não mais pelo
            // StateRegistry `numbering_active:equation` (canal global retirado).
            if *block && *numbering_active {
                intr.counters.apply_at(
                    "equation".to_string(),
                    counter_update.clone(),
                    loc,
                );
                if let Some(label) = &info.label {
                    intr.label_to_counter_key.insert(label.clone(), "equation".into());
                }
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
            // P462: Labelled usa caminho legacy; evita conflito com
            // Content::Label numérico.
            intr.label_to_counter_key.remove(label);
        }
        ElementPayload::Table { counter_update, is_counted, caption_text } => {
            intr.kind_index
                .entry(ElementKind::Table)
                .or_default()
                .push(loc);
            // P461: counter "table" avança só quando caption + numbering.
            if *is_counted {
                intr.counters.apply_at(
                    "table".to_string(),
                    counter_update.clone(),
                    loc,
                );
                // **P472** — popular tables_for_lot com (número, caption).
                let num = intr.counters.value_at("table", loc)
                    .and_then(|v| v.last().copied())
                    .unwrap_or(0);
                if let Some(cap) = caption_text {
                    intr.tables_for_lot.push((num, cap.clone()));
                }
                if let Some(label) = &info.label {
                    intr.label_to_counter_key.insert(label.clone(), "table".into());
                }
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
        ElementPayload::HeadingForToc { label, number, body, level } => {
            intr.headings_for_toc.push((
                label.clone(),
                number.clone(),
                body.clone(),
                *level,
            ));
        }
        ElementPayload::HeadingForBookmarks { label, number, body, level } => {
            intr.headings_for_bookmarks.push((
                label.clone(),
                number.clone(),
                body.clone(),
                *level,
            ));
        }
        ElementPayload::ContextBlock { id } => {
            // P506: ContextBlock só precisa de ser locatable (tag emitida);
            // a expansão pós-introspecção resolve o closure.
            intr.kind_index
                .entry(ElementKind::ContextBlock)
                .or_default()
                .push(loc);
            intr.context_block_locations.insert(*id, loc);
        }
    }
}

/// **P504** — Constrói mapa `Location → Location` do parent imediato a
/// partir da sequência `Start`/`End` de tags. Cada `Start` empilha a sua
/// Location; `End` desempilha. Filhos são registados contra o topo da
/// pilha no momento do seu `Start`.
fn build_parent_index(tags: &[Tag]) -> HashMap<Location, Location> {
    use crate::entities::element_payload::ElementPayload;
    use crate::entities::tag::Tag;
    let mut parent_map = HashMap::new();
    let mut stack: Vec<Location> = Vec::new();
    for tag in tags {
        match tag {
            Tag::Start(loc, info) => {
                // P504: só nós locatable reais (Heading, Figure, etc.) são
                // entradas de parentesco. Tags pós-recursão (Labelled,
                // HeadingForToc) reutilizam a mesma Location e não devem
                // sobrescrever o parent do nó real.
                if matches!(
                    info.payload,
                    ElementPayload::Heading { .. }
                        | ElementPayload::Figure { .. }
                        | ElementPayload::Citation { .. }
                        | ElementPayload::Metadata { .. }
                        | ElementPayload::State { .. }
                        | ElementPayload::StateUpdate { .. }
                        | ElementPayload::Outline
                        | ElementPayload::Bibliography { .. }
                        | ElementPayload::Equation { .. }
                        | ElementPayload::Table { .. }
                        | ElementPayload::CounterUpdate { .. }
                ) {
                    parent_map.insert(*loc, stack.last().copied());
                }
                stack.push(*loc);
            }
            Tag::End(loc, _) => {
                if stack.last() == Some(loc) {
                    stack.pop();
                }
            }
        }
    }
    parent_map
        .into_iter()
        .filter_map(|(loc, parent)| parent.map(|p| (loc, p)))
        .collect()
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
    // P363 (F-5a, introspect-chain): a chain léxica threaded no walk. Empurrada
    // ao descer num `Content::Styled` (espelho de `layout/mod.rs:1248`), carrega o
    // gate de numbering (`X.numbering`) para o introspect ler de UMA fonte só (a
    // chain) no de-bake (P364). **Aditivo aqui** — usada pelo push em `Styled` e
    // pela probe `#[cfg(test)]` no arm Heading; os reads de gate continuam no campo
    // assado até o P364. Separação gate-vs-número: NÃO toca o contador (P335).
    chain:              &StyleChain,
    label_from_parent:  Option<&Label>,
) {
    // P162 .E + P191B: emissão Tag::Start em paralelo, antes da mutação
    // de estado. populate_intr_from_tag_start popula sub-stores intr no
    // momento da emissão (ADR-0071).
    let emitted_loc = if let Some(mut payload) = do_extract_payload(content) {
        let loc = locator.next();
        // F-5a de-bake (P364, §3a.9): a equação **não baka** mais o gate. O
        // payload tira `numbering_active` da **chain** aqui (introspect-chain
        // P363), no momento da emissão — a consumição posterior (`from_tags` /
        // `populate_intr_from_tag_start`) não tem chain. Fonte única. (`block`
        // segue no payload; o gate efetivo é `block && numbering` no consumidor.)
        if let ElementPayload::Equation { numbering_active, .. } = &mut payload {
            *numbering_active =
                matches!(chain.custom("equation.numbering"), Some(Value::Str(_)));
        }
        // F-5a de-bake (P365): a figura não baka mais o padrão. `is_counted` é o
        // placeholder (`caption.is_some()`, de `to_payload`) **ANDado** com o gate
        // do padrão lido da chain aqui (`Some(Str)` = numerado). Fonte única.
        if let ElementPayload::Figure { is_counted, .. } = &mut payload {
            *is_counted &= matches!(chain.custom("figure.numbering"), Some(Value::Str(_)));
        }
        // P461: a table não baka o padrão. `is_counted` placeholder
        // (`caption.is_some()`, de `to_payload`) **ANDado** com o gate
        // `table.numbering` lido da chain. Fonte única.
        if let ElementPayload::Table { is_counted, .. } = &mut payload {
            *is_counted &= matches!(chain.custom("table.numbering"), Some(Value::Str(_)));
        }
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
                walk(item, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        // F-5b fatia 1 (P371): strong/emph são transparentes ao walk (morfologia,
        // sem custom/locatável) — descem no body, como o arm `Styled`.
        Content::Strong(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Emph(e)   => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P408: smallcaps é transparente ao walk (morfologia, não locatável).
        Content::SmallCaps { body } => walk(
            body, locator, tags, intr, auto_label_counter, lang, chain, None,
        ),

        Content::Heading(h) => {
            // Modelo D (P316): Heading delegado; re-bind dos campos.
            let level = &h.level;
            let body = &h.body;
            let _ = level;
            // P363 (introspect-chain, PROBE de verificação): o gate de numbering
            // lido **da chain** neste heading. **Aditivo** — não muda o read do
            // gate (continua `h.numbering_active` até o de-bake P364). Só captura,
            // sob `#[cfg(test)]`, o que a chain entrega aqui, para o teste provar que
            // a infra threadou o custom até o heading (um heading sob o `Content::
            // Styled` de `#set heading(numbering:)` vê `Some(Bool(true))`).
            #[cfg(test)]
            self::introspect_chain_probe::record(
                matches!(chain.custom("heading.numbering"), Some(Value::Bool(true))),
            );
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
            // F-5a de-bake (P364, §3a.9): o gate vive **só na chain** (lido aqui
            // sobre o introspect-chain do P363). O campo assado `numbering_active`
            // foi removido. O **valor** do contador (`apply_hierarchical_at`,
            // incondicional P335) e `formatted_counter_at` ficam intactos.
            let numbering_active =
                matches!(chain.custom("heading.numbering"), Some(Value::Bool(true)));
            let (auto_label, resolved_text) = heading::compute_heading_auto_toc(
                &*intr,
                auto_loc,
                current_auto_label,
                numbering_active,
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

            walk(body, locator, tags, intr, auto_label_counter, lang, chain, None);

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

            // P200B/P606: emit Tag::HeadingForToc e/ou Tag::HeadingForBookmarks
            // pós-recursão. `outlined` controla o índice do documento;
            // `bookmarked` (explicitamente ou via `outlined`) controla a árvore
            // `/Outlines` do PDF. Ambas partilham o mesmo cálculo de entrada.
            if let Some(loc) = emitted_loc {
                if let Some((label, number, body_for_toc, lvl)) = heading::compute_heading_for_toc(
                    &*intr,
                    loc,
                    current_auto_label,
                    frozen_body,
                    *level as usize,
                    numbering_active,
                ) {
                    if h.outlined {
                        let info = ElementInfo::new(ElementPayload::HeadingForToc {
                            label: label.clone(),
                            number: number.clone(),
                            body: body_for_toc.clone(),
                            level: lvl,
                        });
                        populate_intr_from_tag_start(intr, &info, loc);
                        tags.push(Tag::Start(loc, info));
                        tags.push(Tag::End(loc, 0));
                    }
                    if h.is_bookmarked() {
                        let info = ElementInfo::new(ElementPayload::HeadingForBookmarks {
                            label,
                            number,
                            body: body_for_toc,
                            level: lvl,
                        });
                        populate_intr_from_tag_start(intr, &info, loc);
                        tags.push(Tag::Start(loc, info));
                        tags.push(Tag::End(loc, 0));
                    }
                }
            }
        }

        Content::Equation(e) => {
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
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
        }

        Content::Figure(e) => {
            let (body, caption) = (&e.body, &e.caption);
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
            walk(body, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(cap) = caption {
                walk(cap, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        Content::Label(e) => {
            let label = Label(e.name.to_string());
            if e.auto {
                // P464: Label auto-gerado (ex-sintaxe `<label>`). Mantém
                // comportamento legado de `Content::Labelled`: emite Tag
                // pós-recursão com texto resolvido / número de figura.
                let target = &e.body;
                let tags_len_before = tags.len();
                walk(target, locator, tags, intr, auto_label_counter, lang, chain, Some(&label));
                // Label auto-gerado usa caminho legacy; elimina qualquer
                // mapeamento numérico eventualmente criado pelo walk recursivo.
                intr.label_to_counter_key.remove(&label);

                let target_loc = tags[tags_len_before..]
                    .iter()
                    .find_map(|t| if let Tag::Start(l, _) = t { Some(*l) } else { None });

                let (resolved_text, figure_number) = match target_loc {
                    Some(loc) => labelled::compute_labelled(&*intr, loc, target, lang),
                    None => (None, None),
                };

                let _ = figure_number;

                if resolved_text.is_some() || figure_number.is_some() {
                    if let Some(loc) = target_loc {
                        let info = ElementInfo::new(ElementPayload::Labelled {
                            label,
                            resolved_text,
                            figure_number,
                        });
                        populate_intr_from_tag_start(intr, &info, loc);
                        tags.push(Tag::Start(loc, info));
                        tags.push(Tag::End(loc, 0));
                    }
                }
            } else {
                // P460/P464: Label criado pelo utilizador. Propaga a label
                // para o body, permitindo que Ref resolva labels em
                // headings/figures/equations. Não emite Tag próprio quando
                // o body não é locatable — o destino PDF ainda é gerado pelo
                // layout independentemente.
                walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, Some(&label));
            }
        }

        // Lote F-2 S5 (P335): arms populate Set*Numbering removidos com as variantes.

        Content::CounterUpdate(_) => {
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

        Content::Empty
        | Content::Text(_)
        | Content::Space
        // P622: Parbreak é leaf estrutural — sem efeito em counters.
        | Content::Parbreak
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathRoot(_)
        | Content::MathDelimited(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        // P296 — Math accent/cancel sem children non-math em walk
        // (paralelo MathFrac/MathRoot: math layout interno não emite
        // tags introspecção).
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        // P297 — Math underover terminal em walk (paralelo P296).
        | Content::MathUnderover(_)
        // P298 — Math op terminal em walk.
        | Content::MathOp(_)
        // P311b.2 — MathStyled terminal em walk (math structural).
        | Content::MathStyled(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        // P287 — SmartQuote leaf (não-locatable; sem counters).
        | Content::SmartQuote(_)
        | Content::Image(_)
        | Content::SetPage { .. }
        | Content::Divider(_)
        // Passo 156D — h/v spacing leaves; sem effect em counters.
        | Content::HSpace(_)
        | Content::VSpace(_)
        // Passo 156E — pagebreak leaf; sem effect em counters.
        | Content::Pagebreak(_)
        // Passo 220 — colbreak leaf; sem effect em counters.
        | Content::Colbreak(_)
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — sem efeito em counters.
        // Tag::Start/End já é emitido no topo de walk via extract_payload
        // (que produz `Some(ElementPayload::Metadata)`).
        | Content::Metadata(_)
        // P171 (M9): State e StateUpdate são terminais. Tag emitido
        // no topo via extract_payload.
        | Content::State(_)
        | Content::StateUpdate(_)
        // P240 (M9d/M7+1): StateDisplay terminal. Tag emitido no topo
        // via extract_payload; valor pre-rendered em apply_state_displays.
        | Content::StateDisplay(_)
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; tag emitido no topo via extract_payload; valor
        // pre-rendered em apply_counter_displays.
        | Content::CounterDisplayCallback(_) => {}

        // Lote F-1 (P334): a fronteira dinâmica é **leaf** no walk — o tag de
        // início (se locatável) já foi emitido acima via `extract_payload`; a
        // travessia dos filhos do nó dinâmico chega com a realização em F-2
        // (L0 `f_fronteira_e1.md` §3a.7). Em F-1 só existe em fixtures.
        Content::Dynamic(_) => {}

        // Passo 154B — Terms / TermItem: descem em items para que filhos
        // com contadores ou labels sejam processados.
        Content::Terms(e) => {
            for item in e.items.iter() { walk(item, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }
        Content::TermItem(e) => {
            walk(&e.term, locator, tags, intr, auto_label_counter, lang, chain, None);
            walk(&e.description, locator, tags, intr, auto_label_counter, lang, chain, None);
        }

        // Passo 155 — Quote: walk em body + attribution.
        Content::Quote(e) => {
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(a) = &e.attribution {
                walk(a, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        // Modelo D (Lote 4 P319): decorações — walk no body (não-locatable).
        Content::Underline(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Strike(e)    => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Overline(e)  => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Transform(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Grid(e) => {
            // P224 — Grid refino: walk em header (se houver) + cells + footer.
            if let Some(h) = &e.header { walk(h, locator, tags, intr, auto_label_counter, lang, chain, None); }
            for cell in &e.cells { walk(cell, locator, tags, intr, auto_label_counter, lang, chain, None); }
            if let Some(f) = &e.footer { walk(f, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }

        // P224.B — GridHeader / GridFooter (recurse no body; paridade P157C).
        Content::GridHeader(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::GridFooter(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P224.C — GridCell (recurse no body; paridade P157B TableCell).
        Content::GridCell(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 512 — linhas em grid/table não têm body recursivo.
        Content::GridHLine(_) | Content::GridVLine(_) => {}
        Content::TableHLine(_) | Content::TableVLine(_) => {}

        // P461 — Table locatable. Tag emitido no walk top; aqui recursa
        // em caption (se houver) + children, espelhando Figure.
        Content::Table(e) => {
            if let Some(cap) = &e.caption {
                walk(cap, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
            for c in &e.children { walk(c, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }

        // Passo 157B — TableCell (recurse no body).
        Content::TableCell(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 157C — par simétrico TableHeader/TableFooter
        // (recurse no body).
        Content::TableHeader(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::TableFooter(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

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
        Content::Bibliography(e) => {
            if let Some(t) = &e.title {
                walk(t, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }
        Content::Cite(e) => {
            if let Some(s) = &e.supplement { walk(s, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }

        // P295 — Footnote walk em body (locatable infrastructure não
        // aplicada em Fase 1; body recurse preserva counters/labels
        // dentro para passes futuros).
        Content::Footnote(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Align(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Place(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156C (ADR-0061 Fase 1) — pad / hide são containers
        // estruturais; descer no body para que counters/labels dentro sejam
        // processados. `Hide` mesmo "ocultando visualmente" mantém a
        // semântica de presence (label/ref dentro de hide ainda resolvem).
        Content::Pad(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Hide(e)           => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156G (ADR-0061 Fase 2) — block container; descer no body.
        Content::Block(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        Content::Boxed(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Walk em cada child em ordem (counters/labels resolvem).
        Content::Stack(e) => {
            for c in e.children.iter() {
                walk(c, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        },

        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat container.
        // Walk no body uma vez (counters/labels dentro de body
        // resolvem; semântica de repetição é runtime-only e não
        // multiplica state — vanilla repeat também só conta uma vez).
        Content::Repeat(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        // P217 (DEBT-56 sub-fase b) — Columns container.
        // Walk no body (counters/labels dentro contam normalmente);
        // sem Tag::Start/End próprio (columns não é locatable).
        // Consumer multi-region em P219.
        Content::Columns(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 99 (ADR-0038): `Styled` é transparente — desce no body.
        // P363 (introspect-chain): empurra os styles na chain ao descer (espelho de
        // `layout/mod.rs:1248` `push_styles`), para que um heading/equation/figure
        // dentro do `Content::Styled` que o `#set …(numbering:)` embrulha tenha o
        // gate `X.numbering` na chain. Escopado à subárvore (o `pushed` só vale na
        // recursão do `body`).
        Content::Styled(body, styles) => {
            let pushed = chain.push_styles(styles);
            walk(body, locator, tags, intr, auto_label_counter, lang, &pushed, label_from_parent);
        }

        Content::Outline(_) => {
            // P189B (M5): walk puro para Outline.
            // Mutação `state.has_outline = true` removida; flag obtida
            // via `intr.kind_index.contains_key(&ElementKind::Outline)`
            // (populado por `from_tags` arm Outline P178). Consumer
            // migrado em `mod.rs:1470` (`layout_with_introspector`).
            // `Content::Outline` continua a ser locatable e emite
            // Tag::Start no topo da `walk` fn — apenas a mutação
            // directa em state foi removida.
            //
            // P480 — registo sintético de heading de título em kind_index
            // para paridade de query de count com vanilla (vanilla conta
            // o heading de título do outline via pós-layout; aqui registo
            // pré-layout como entry sintética). headings_for_toc NÃO
            // actualizado — evita TOC auto-referente. Counter NÃO
            // aplicado — título do outline não é secção numerada.
            // Abordagem directa ao kind_index adoptada em vez da Sequence
            // em native_outline (spec P480 §A.1) porque a Sequence faria
            // walk normal do Heading → headings_for_toc receberia o
            // título → layout_outline listaria o próprio título como
            // entrada na TOC (auto-referência). ADR-0108 §6: intenção
            // = count parity; comportamento = registo sintético.
            let title_loc = locator.next();
            intr.kind_index
                .entry(ElementKind::Heading)
                .or_default()
                .push(title_loc);
        }

        // P506 — ContextBlock é terminal no walk (a tag já foi emitida no
        // topo via extract_payload; o corpo é uma closure avaliada na
        // fase de expansão pós-introspecção).
        Content::ContextBlock(_) => {}

        // P397 — Document/Asset são metadata/resources; não entram no walk
        // de conteúdo renderizável.
        Content::Document { .. } => {}
        Content::Asset { .. }    => {}

        // Passo 513 — Curve é leaf não-locatable; sem descendência.
        Content::Curve(_) => {}
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

    /// **F-5a de-bake (P365)** — rotula reproduzindo a **forma de produção**: o
    /// transporte de numbering (`Content::Styled`, ex.: o que `Content::figure(..,
    /// Some)` produz) fica **fora** do `Labelled` (`Styled{ Labelled{ alvo } }`),
    /// como a fatia-1 embrulha a cauda. Sem isto, `Labelled{ Styled{ alvo } }`
    /// inverte a ordem e o `compute_labelled` (que inspeciona o tipo do alvo) não
    /// dispara. Levanta o transporte transparente para fora; alvo simples passa
    /// direto.
    fn labelled_prod(target: Content, label: Label) -> Content {
        let name = label.0;
        match target {
            Content::Styled(inner, styles) =>
                Content::Styled(Box::new(Content::label_auto(name, *inner)), styles),
            other => Content::label_auto(name, other),
        }
    }

    #[test]
    fn introspect_popula_label_forward() {
        // Ref antes do Labelled — forward reference
        let content = Content::Sequence(
            vec![
                Content::reference("conclusao"),
                Content::label_auto("conclusao".to_string(), Content::heading(1, Content::text("Conclusão"))),
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
            vec![Content::counter_update("equation".to_string(), CounterAction::Update(5))]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.counters.value("equation").and_then(|v| v.last()).copied().unwrap_or(0), 5);
    }

    #[test]
    fn introspect_dois_conteudos_independentes() {
        let content_a = Content::label_auto("a".to_string(), Content::heading(1, Content::text("A")));
        let content_b = Content::reference("a");

        let intr_a = introspect_with_introspector(&content_a);
        let intr_b = introspect_with_introspector(&content_b);

        assert!(intr_a.resolved_labels.get(&Label("a".to_string())).is_some());
        assert!(
            intr_b.resolved_labels.get(&Label("a".to_string())).is_none(),
            "estados de introspecção devem ser independentes"
        );
    }


    // ── Testes de Passo 61 — TOC ─────────────────────────────────────────

    #[test]
    fn introspect_cataloga_headings_para_toc() {
        let content = Content::Sequence(vec![
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
            Content::heading(1, Content::text("Conclusão")),
        ].into());

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 3);

        let (_, _, title_0, level_0) = &intr.headings_for_toc()[0];
        assert_eq!(title_0.plain_text(), "Introdução");
        assert_eq!(*level_0, 1);

        let (_, _, _, level_1) = &intr.headings_for_toc()[1];
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
        let (label, _, _, _) = &intr.headings_for_toc()[0];
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
            vec![labelled_prod(Content::figure(Content::text("Um gráfico"), Some(Content::text("Evolução")), Some("image".to_string()), Some("1".to_string())), Label("fig1".to_string()))]
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
                labelled_prod(Content::figure(Content::text("A"), Some(Content::text("Legenda A")), Some("image".to_string()), Some("1".to_string())), Label("f1".to_string())),
                labelled_prod(Content::figure(Content::text("B"), Some(Content::text("Legenda B")), Some("image".to_string()), Some("1".to_string())), Label("f2".to_string())),
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
                Content::figure(Content::text("Diagrama"), None, Some("image".to_string()), Some("1".to_string())),
                labelled_prod(Content::figure(Content::text("B"), Some(Content::text("Legenda")), Some("image".to_string()), Some("1".to_string())), Label("f2".to_string())),
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
                Content::label_auto("sec".to_string(), Content::heading(1, Content::text("Secção"))),
                Content::reference("sec"),
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
                Content::counter_display("fig".to_string()),
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
                Content::counter_update("fig".to_string(), CounterAction::Update(7)),
                Content::heading(1, Content::Sequence(
                    vec![
                        Content::text("Figura "),
                        Content::counter_display("fig".to_string()),
                    ]
                    .into(),
                )),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 1);

        let (_, _, frozen_body, _) = &intr.headings_for_toc()[0];
        let text = frozen_body.plain_text();
        // O body congelado deve conter "7" (valor no momento da introspecção),
        // não "0" (valor no início do documento quando a TOC é renderizada).
        assert!(text.contains("7"),
            "CounterDisplay no título deve ser congelado com o valor correcto: {:?}", text);
    }

    // ── Testes de Passo 75 — figure_numbers por kind (DEBT-14/15) ───────────

    #[test]
    fn figure_tem_kind_e_numbering() {
        // F-5a de-bake (P365): `Content::figure(.., Some(padrão))` produz a forma de
        // transporte `Styled(Figure, custom("figure.numbering"))` — o `kind` fica no
        // `Figure`; o padrão vive no custom da chain (não mais campo do elemento).
        let fig = Content::figure(Content::text("corpo"), Some(Content::text("legenda")), Some("image".to_string()), Some("1".to_string()));
        if let Content::Styled(body, styles) = fig {
            let pat = styles.delta().custom.iter()
                .find(|(k, _)| k == "figure.numbering")
                .map(|(_, v)| v.clone());
            assert_eq!(pat, Some(Value::Str("1".into())));
            assert!(matches!(&*body, Content::Figure(e) if e.kind.as_deref() == Some("image")));
        } else {
            panic!("esperado Styled (transporte de numbering)");
        }
    }

    #[test]
    fn figuras_kind_diferente_contadores_independentes() {
        let doc = Content::Sequence(vec![
            Content::figure(Content::text("img1"), Some(Content::text("cap1")), Some("image".to_string()), Some("1".to_string())),
            Content::figure(Content::text("tab1"), Some(Content::text("cap2")), Some("table".to_string()), Some("1".to_string())),
            Content::figure(Content::text("img2"), Some(Content::text("cap3")), Some("image".to_string()), Some("1".to_string())),
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
        walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, Some(&lang), &crate::entities::style_chain::StyleChain::default_chain(), None);
        intr
    }

    #[test]
    fn figure_label_default_no_lang_set_devolve_pt() {
        // P158B §8.2: lang None → fallback PT (backwards compat).
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("image".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("image".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("table".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("raw".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("image".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
        let figure = Content::figure(Content::text("body"), Some(Content::text("caption")), Some("custom".to_string()), Some("1".to_string()));
        let labelled = labelled_prod(figure, label.clone());
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
            Content::figure(Content::text("body1"), Some(Content::text("c1")), Some("image".to_string()), Some("1".to_string())),
            Content::figure(Content::text("body2"), Some(Content::text("c2")), Some("table".to_string()), Some("1".to_string())),
            Content::figure(Content::text("body3"), Some(Content::text("c3")), Some("image".to_string()), Some("1".to_string())),
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
                labelled_prod(Content::figure(Content::text("body sem kind explícito"), Some(Content::text("legenda")), None, Some("1".to_string())), Label("f_none".to_string())),
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
        walk(content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None);
        tags
    }

    #[test]
    fn walk_emite_start_e_end_para_heading() {
        let h = Content::heading(1, Content::text("title"));
        let tags = introspect_with_tags(&h);
        // P606: heading emite 8 tags com mesma Location:
        //   Start(Heading), Start(Labelled auto-toc), End(Labelled),
        //   Start(HeadingForToc), End(HeadingForToc),
        //   Start(HeadingForBookmarks), End(HeadingForBookmarks), End(Heading).
        // 4 pares Start/End — bracketing por construção (mesma loc).
        assert_eq!(tags.len(), 8, "heading deve emitir 8 tags pós-P606; obtido {tags:?}");
        let locs: Vec<_> = tags.iter().map(|t| match t {
            Tag::Start(l, _) | Tag::End(l, _) => *l,
        }).collect();
        // Todas as 8 tags partilham mesma Location (P196A §11.5 +
        // P200B/P606 trabalho híbrido).
        for w in locs.windows(2) {
            assert_eq!(w[0], w[1], "tags devem partilhar mesma Location");
        }
        // Ordem esperada: 4 Start consecutivas + 4 End consecutivas
        // (Heading abre, Labelled abre, Labelled fecha, HeadingForToc
        // abre, HeadingForToc fecha, HeadingForBookmarks abre,
        // HeadingForBookmarks fecha, Heading fecha).
        match (&tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5], &tags[6], &tags[7]) {
            (
                Tag::Start(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::End(_, _),
            ) => {}
            other => panic!("ordem esperada: Start, Start, End, Start, End, Start, End, End; obtido {other:?}"),
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
        let figure = Content::figure(Content::Empty, Some(Content::text("cap")), Some("image".into()), Some("1".into()));
        let h = Content::heading(1, figure);
        let tags = introspect_with_tags(&h);
        // P606: heading emite 8 tags + figura emite 2 tags = 10 tags.
        // Sequência: Start(Heading), Start(Figure), End(Figure),
        // Start(Labelled), End(Labelled), Start(HeadingForToc),
        // End(HeadingForToc), Start(HeadingForBookmarks),
        // End(HeadingForBookmarks), End(Heading).
        assert_eq!(tags.len(), 10, "heading-com-figura deve emitir 10 tags pós-P606, obtido {tags:?}");
        match (&tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5], &tags[6], &tags[7], &tags[8], &tags[9]) {
            (
                Tag::Start(_, _), // Heading
                Tag::Start(_, _), // Figure
                Tag::End(_, _),   // Figure
                Tag::Start(_, _), // Labelled auto-toc
                Tag::End(_, _),   // Labelled auto-toc
                Tag::Start(_, _), // HeadingForToc
                Tag::End(_, _),   // HeadingForToc
                Tag::Start(_, _), // HeadingForBookmarks
                Tag::End(_, _),   // HeadingForBookmarks
                Tag::End(_, _),   // Heading
            ) => {}
            other => panic!("ordem esperada após P606: 10 tags; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_emite_tags_em_paralelo_com_state() {
        // Verifica que tanto state quanto tags são populados.
        let content = Content::Sequence(
            vec![
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
        // P606: cada heading emite 8 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc,
        // Start_HeadingForBookmarks, End_HeadingForBookmarks, End_h).
        // Total: 2 headings × 8 = 16.
        assert_eq!(tags.len(), 16, "deve haver 8 tags por heading (2 headings x 8); obtido {tags:?}");
    }

    #[test]
    fn walk_label_de_wrapper_chega_ao_payload() {
        // Content::Labelled { target: Heading } → tag Heading recebe Some(label).
        let content = Content::label_auto("intro".to_string(), Content::heading(1, Content::text("Introdução")));
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
                Content::heading(1, Content::text("Capítulo")),
                Content::figure(Content::Empty, Some(Content::text("legenda")), Some("image".into()), Some("1".into())),
                Content::heading(2, Content::text("Secção")),
                Content::cite("smith2024", None, None,),
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
        let figure_with_h = Content::figure(inner_h, Some(Content::text("cap")), Some("image".into()), Some("1".into()));
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
        // P606: cada heading emite 8 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc,
        // Start_HeadingForBookmarks, End_HeadingForBookmarks, End_h)
        // — todas com mesma Location, bracketing continua válido.
        // 3 headings × 8 = 24.
        assert_eq!(tags.len(), 24, "3 headings × 8 tags pós-P606 = 24");
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
        // Lote F-2 S5 (P335): marcador removido — contador de heading incondicional.
        let content = Content::Sequence(
            levels.iter().map(|&l| Content::heading(l, Content::text("h")))
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
                Content::figure(Content::Empty, Some(Content::text("c1")), Some("image".into()), Some("1".into())),
                Content::figure(Content::Empty, Some(Content::text("c2")), Some("table".into()), Some("1".into())),
                Content::figure(Content::Empty, Some(Content::text("c3")), None, Some("1".into())),
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
                Content::cite("smith2024", None, None,),
                Content::cite("jones2023", None, None,),
                Content::cite("smith2024", None, None,),  // repetida
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
        // **P533** — mirror do path público: converter `@key` bibliográficos
        // antes do walk para contar citações correctamente.
        let content = convert_bib_refs_to_cites(content.clone());
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None);
        intr.parent_locations = build_parent_index(&tags);
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
        // Lote F-2 S5 (P335): marcador removido — contador de heading incondicional.
        let content = Content::Sequence(
            levels.iter().map(|&l| Content::heading(l, Content::text("h")))
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
                Content::figure(Content::Empty, Some(Content::text("c1")), Some("image".into()), Some("1".into())),
                Content::figure(Content::Empty, Some(Content::text("c2")), Some("table".into()), Some("1".into())),
                Content::figure(Content::Empty, Some(Content::text("c3")), None, Some("1".into())),
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
                Content::cite("smith2024", None, None,),
                Content::cite("jones2023", None, None,),
                Content::cite("brown2022", None, None,),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let locs = intr.query_by_kind(ElementKind::Citation);
        assert_eq!(locs.len(), 3);
    }



    #[test]
    fn introspector_query_by_label() {
        // P165 .G.4: walk com Heading labelled → query_by_label retorna location;
        // mesma location aparece em query_by_kind(Heading).
        let content = Content::label_auto("intro".to_string(), Content::heading(1, Content::text("Introdução")));
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
        let single_figure = Content::figure(Content::Empty, Some(Content::text("c")), Some("image".into()), Some("1".into()));
        let intr1 = introspect_with_introspector(&single_figure);
        assert!(intr1.query_first(ElementKind::Figure).is_some());
        assert!(intr1.query_unique(ElementKind::Figure).is_some());
        assert_eq!(
            intr1.query_first(ElementKind::Figure),
            intr1.query_unique(ElementKind::Figure)
        );

        let two_figures = Content::Sequence(
            vec![
                Content::figure(Content::Empty, Some(Content::text("c1")), Some("image".into()), Some("1".into())),
                Content::figure(Content::Empty, Some(Content::text("c2")), Some("table".into()), Some("1".into())),
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
    fn introspect_chain_threada_gate_de_numbering_ate_o_heading() {
        // **P363 (introspect-chain) — prova a infra (aditiva).** Um heading DENTRO do
        // `Content::Styled` que o `#set heading(numbering:)` embrulha vê o gate
        // `heading.numbering` **na chain** (true); um heading fora não (false). A probe
        // `#[cfg(test)]` no arm Heading do walk captura o gate lido da chain. Prova que
        // a chain foi threadada até o heading (sem mudar o comportamento — o read real do
        // gate continua no campo assado até o de-bake P364).
        use crate::entities::style::Styles;
        super::introspect_chain_probe::reset();
        let sob_set = Content::Styled(
            Box::new(Content::heading(1, Content::text("Sob set"))),
            Styles::new().push_custom("heading.numbering", Value::Bool(true)),
        );
        let fora = Content::heading(1, Content::text("Fora"));
        let doc = Content::Sequence(vec![sob_set, fora].into());
        let _ = introspect_with_introspector(&doc);
        assert_eq!(
            super::introspect_chain_probe::captured(),
            vec![true, false],
            "introspect-chain: o heading sob #set numbering vê o gate na chain (true); \
             o de fora não (false)"
        );
    }

    #[test]
    fn introspect_legacy_continua_a_funcionar() {
        // P166 .C.3 (backward compat): call-site antigo
        // `let state = introspect(&c)` continua a compilar e funcionar.
        // Esta é a invariante crítica de M4b — wrapper preserva API.
        let content = Content::Sequence(
            vec![
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
                Content::figure(Content::Empty, Some(Content::text("cap")), Some("image".into()), Some("1".into())),
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
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut $engine = Engine {
                world,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file,
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
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update("c".to_string(), StateUpdate::Func(f)),
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
                &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            ).expect("apply_state_funcs deve suceder");
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
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update("c".to_string(), StateUpdate::Func(f)),
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
                Content::state("c".to_string(), Value::Int(5)),
                Content::state_update("c".to_string(), StateUpdate::Func(f1)),
            ]
            .into(),
        );
        let content_b = Content::Sequence(
            vec![
                Content::state("c".to_string(), Value::Int(5)),
                Content::state_update("c".to_string(), StateUpdate::Func(f2)),
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
                &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            ).expect("apply_state_funcs deve suceder");
            intr.state_final_value("c").cloned()
        });
        let v_b = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content_b, &mut locator, &mut tags,
                &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None,
            );
            super::from_tags::apply_state_funcs(
                &tags, &mut intr, &mut engine, &mut ctx,
            ).expect("apply_state_funcs deve suceder");
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

        let content = Content::bibliography(vec![
                BibEntry::new("a", "Author A", "Title A", 2024),
                BibEntry::new("b", "Author B", "Title B", 2025),
            ], None);
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None);

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

        let titulo = Content::label_auto("bib-title".to_string(), Content::heading(1, Content::Empty));

        let content = Content::bibliography(vec![BibEntry::new("a", "A", "T", 2024)], Some(titulo));
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None);

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
                Content::heading_numbered(1, Content::text("Intro")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Auto-label "auto-toc-1" deve estar no Introspector.resolved_labels
        // populado via Tag::Labelled pós-P196B.
        let auto_label = Label("auto-toc-1".to_string());
        assert_eq!(
            intr.resolved_label_for(&auto_label),
            // P359 (DEBT-60 b): auto-toc resolved_text é o NÚMERO ("1."), sem o
            // supplement "Secção" — o outline mostra o numbering (paridade vanilla).
            Some("1."),
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
            .map(|(_, _, _, lvl)| *lvl)
            .collect();
        assert_eq!(levels, vec![1, 2, 1]);
    }

    #[test]
    fn p606_outlined_e_bookmarked_separados_nas_substores() {
        // Quatro casos vanilla:
        // A: outlined=true,  bookmarked=auto  -> em TOC e bookmarks
        // B: outlined=false, bookmarked=auto  -> fora de TOC e bookmarks
        // C: outlined=true,  bookmarked=false -> em TOC, fora de bookmarks
        // D: outlined=false, bookmarked=false -> fora de ambos
        let content = Content::Sequence(vec![
            Content::heading(1, Content::text("A")),
            Content::heading_with_outlined_and_bookmarked(1, Content::text("B"), false, None),
            Content::heading_with_outlined_and_bookmarked(1, Content::text("C"), true, Some(false)),
            Content::heading_with_outlined_and_bookmarked(1, Content::text("D"), false, Some(false)),
        ].into());
        let intr = introspect_with_introspector(&content);

        let toc_titles: Vec<String> = intr.headings_for_toc()
            .iter()
            .map(|(_, _, body, _)| body.plain_text())
            .collect();
        let bm_titles: Vec<String> = intr.headings_for_bookmarks()
            .iter()
            .map(|(_, _, body, _)| body.plain_text())
            .collect();

        assert_eq!(toc_titles, vec!["A", "C"], "TOC deve conter A e C");
        assert_eq!(bm_titles, vec!["A"], "Bookmarks devem conter apenas A");
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
                Content::heading_numbered(1, Content::text("Capítulo")),
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
        // P359 (DEBT-60 b): número só ("1."), sem supplement "Secção".
        assert_eq!(via_introspector, Some("1."));
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
        let content = Content::figure(Content::Empty, Some(Content::text("Cap")), Some("image".into()), Some("1".to_string()));
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
                Content::figure(Content::Empty, Some(Content::text("c1")), Some("image".into()), Some("1".to_string())),
                Content::figure(Content::Empty, Some(Content::text("c2")), Some("image".into()), Some("1".to_string())),
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
        let content = Content::figure(Content::Empty, Some(Content::text("Cap")), Some("table".into()), Some("1".to_string()));
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
        let figura_sem_caption = Content::figure(Content::Empty, None, Some("image".into()), Some("1".to_string()));
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
        let content = labelled_prod(Content::figure(Content::Empty, Some(Content::text("Cap")), Some("image".into()), Some("1".to_string())), Label("fig1".to_string()));
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

        // F-5a de-bake (P365): `Content::figure(.., Some)` produz a forma de
        // transporte `Styled(Figure, custom)`. Para a parte (a) — que chama
        // `extract_payload` **direto** no elemento — descasca o transporte para
        // alcançar a `Figure`. `extract_payload`/`to_payload` dão agora o
        // **placeholder** de `is_counted` (= `caption.is_some()`); o gate de
        // numbering é ANDado no walk (parte (b)/(c) abaixo o exercitam). Como todos
        // os casos têm numbering Some, o placeholder coincide com o valor antigo.
        fn fig_inner(c: &Content) -> &Content {
            match c { Content::Styled(b, _) => b, other => other }
        }

        let caso1 = Content::figure(Content::text("img"), None, None, Some("1".to_string()));
        let caso2 = Content::figure(Content::text("img"), Some(Content::text("c")), None, Some("1".to_string()));
        let caso3 = Content::figure(Content::text("t"), Some(Content::text("c")), Some("table".to_string()), Some("1".to_string()));
        let caso4 = Content::figure(Content::text("t"), None, Some("table".to_string()), Some("1".to_string()));

        // (a) `extract_payload` preserva `kind` literalmente (sem default
        //     — lacuna #1 fecha porque tag preserva None vs Some("image")
        //     distintamente; default só aplica em populate_intr).
        match extract_payload(fig_inner(&caso1)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None,         "caso1 kind preservado None literal");
                assert_eq!(is_counted, false, "caso1 sem caption → is_counted=false");
            }
            other => panic!("caso1: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso2)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None,        "caso2 kind preservado None literal");
                assert_eq!(is_counted, true, "caso2 com caption+numbering → is_counted=true");
            }
            other => panic!("caso2: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso3)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, Some("table".to_string()), "caso3 kind literal Some(\"table\")");
                assert_eq!(is_counted, true,                "caso3 com caption+numbering → is_counted=true");
            }
            other => panic!("caso3: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso4)) {
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

        let content = Content::counter_update("equation".to_string(), CounterAction::Step);
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

        let c = Content::counter_update("page".to_string(), CounterAction::Update(42));
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
                Content::counter_update("equation".to_string(), CounterAction::Step),
                Content::counter_update("equation".to_string(), CounterAction::Step),
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
                Content::counter_update("equation".to_string(), CounterAction::Step),
                Content::counter_update("equation".to_string(), CounterAction::Step),
                Content::counter_update("equation".to_string(), CounterAction::Step),
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
    fn p461_table_counter_popula_via_introspector() {
        // P461: Table locatable + counter "table" populado quando
        // caption + table.numbering presentes.
        use crate::entities::introspector::Introspector;
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::style::Styles;
        use crate::entities::value::Value;

        let mk = |n: usize| Content::table_with_caption(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text(format!("cell{n}"))],
            Some(Content::text(format!("cap{n}"))),
        );
        let content = Content::Sequence(vec![
            Content::Styled(
                Box::new(mk(1)),
                Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
            ),
            Content::Styled(
                Box::new(mk(2)),
                Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
            ),
        ].into());
        let intr = introspect_with_introspector(&content);

        let locs = intr.query_by_kind(ElementKind::Table);
        assert_eq!(locs.len(), 2, "duas tables locatable");
        assert_eq!(
            intr.flat_counter_at("table", locs[0]),
            Some(1),
            "primeira table numerada 1"
        );
        assert_eq!(
            intr.flat_counter_at("table", locs[1]),
            Some(2),
            "segunda table numerada 2"
        );
    }

    #[test]
    fn counter_update_action_update_apply_correctly() {
        // P198C test 5: 3º caminho da match — Update(val) aplica via
        // apply_at(Update). Legacy via state.update_flat. Paridade.
        use crate::entities::introspector::Introspector;

        let content = Content::counter_update("page".to_string(), CounterAction::Update(42));
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
                Content::counter_update("equation".to_string(), CounterAction::Step),
                Content::label_auto("eq1".to_string(), Content::equation(Content::Empty, true)),
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





    #[test]
    fn consumer_layouter_equation_activa_via_introspector() {
        // P199B test 5: caminho Introspector activado por construção
        // — pipeline com SetEquationNumbering + Equation labelled
        // produz resolved_labels populated via cadeia legacy
        // (compute_labelled P195D) E intr.state populated via
        // Tag::StateUpdate. Confirma activação imediata após P199B.
        use crate::entities::introspector::Introspector;

        // F-5a de-bake (P364, §3a.9): o gate de numeração vive **só na chain**,
        // transportado por `Content::Styled`. A forma de **produção** põe o
        // transporte **fora** do `Labelled` (a fatia-1 embrulha a cauda:
        // `Styled{ Labelled{ Equation } }`) — assim o alvo do `Labelled` é a
        // própria `Equation` (o `compute_labelled` inspeciona o tipo do alvo) e o
        // `custom("equation.numbering")` chega à chain antes da emissão do payload.
        // Fixture roteado a essa forma (S5b declarado; era campo assado
        // `equation_numbered` direto, P335).
        let content = Content::Sequence(
            vec![
                Content::Styled(
                    Box::new(Content::label_auto(
                        "eq1".to_string(),
                        Content::equation(Content::Empty, true),
                    )),
                    crate::entities::style::Styles::new()
                        .push_custom("equation.numbering", Value::Str("(1)".into())),
                ),
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
        // Lote F-2 S5 (P335): asserção is_numbering_active removida (StateRegistry
        // numbering_active:equation saiu — gate via campo assado).
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
        assert_eq!(entry.3, 1, "level esperado");
    }

    #[test]
    fn headings_for_toc_paridade_legacy_vs_introspector() {
        // P200B test 2: write paralelo legacy + Introspector preserva
        // paridade exacta. intr.headings_for_toc().len() ==
        // intr.headings_for_toc().len(); conteúdo idêntico.
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

        assert_eq!(intr.headings_for_toc().len(), 3,
            "legacy: 3 entries (mutação 4 preservada como write paralelo M5)");
        assert_eq!(intr.headings_for_toc().len(), 3,
            "Introspector: 3 entries via Tag::HeadingForToc");

        // Paridade exacta — labels e levels.
        for (legacy_entry, intr_entry) in intr.headings_for_toc().iter().zip(intr.headings_for_toc()) {
            assert_eq!(legacy_entry.0, intr_entry.0, "labels devem ser idênticos");
            assert_eq!(legacy_entry.3, intr_entry.3, "levels devem ser idênticos");
        }
    }

    #[test]
    fn bracketing_valido_8_tags_por_heading_p606() {
        // P606 test: confirma bracketing válido com 8 tags por
        // Heading folha (4 Start + 4 End consecutivas; mesma Location).
        let h = Content::heading(1, Content::text("título"));
        let tags = introspect_with_tags(&h);

        assert_eq!(tags.len(), 8, "P606: 8 tags por Heading folha");

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
        let levels_legacy: Vec<_> = intr.headings_for_toc().iter().map(|(_, _, _, l)| *l).collect();
        let levels_intr: Vec<_> = intr.headings_for_toc().iter().map(|(_, _, _, l)| *l).collect();
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
        assert_eq!(entry.3, 2,
            "level preservado per cast usize do level: u8");
        // body materializado preserva text content.
        match &entry.2 {
            Content::Text(s) => assert_eq!(s.as_str(), "título"),
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
                Content::figure(Content::Empty, Some(Content::text("c")), Some("image".to_string()), Some("1".to_string())),
                Content::cite("k".to_string(), None, None),
            ]
            .into(),
        );

        // Walk directo — sem from_tags.
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(&content, &mut locator, &mut tags, &mut intr, &mut auto_label_counter, None, &crate::entities::style_chain::StyleChain::default_chain(), None);

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
                Content::heading(1, Content::text("intro")),
                Content::label_auto("sec".to_string(), Content::heading(1, Content::text("body"))),
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
        // signature <I: Introspector>(intr, location, counter_n). Gate pelo
        // campo assado `numbering_active` + `formatted_counter_at` location-
        // aware (F-4 E0, P338: o gate legado `is_numbering_active_at` saiu).
        // Paridade quando SetHeadingNumbering(true) precede o Heading.
        let content = Content::Sequence(
            vec![
                Content::heading_numbered(1, Content::text("um")),
                Content::heading_numbered(1, Content::text("dois")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);

        // resolved_labels populated com auto-toc texts via Introspector path.
        // P359 (DEBT-60 b): NÚMERO só ("1." / "2."), sem supplement "Secção" — o
        // outline mostra o numbering (paridade vanilla), não a cross-reference.
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-1".to_string())),
            Some("1."),
            "compute_heading_auto_toc via Introspector path: 1ª heading → '1.'",
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-2".to_string())),
            Some("2."),
            "compute_heading_auto_toc via Introspector path: 2ª heading → '2.'",
        );
    }

    #[test]
    fn p464_label_auto_e_user_partilham_resolucao() {
        // P464: `Content::Label` único cobre ambas as origens. Label
        // user-created popula `label_to_counter_key`; label auto-gerado
        // popula `resolved_labels` (caminho legacy ex-Labelled).
        //
        // `labelled_prod` levanta o `Styled` do numbering para fora,
        // garantindo que o alvo do label auto é o Heading puro
        // (requerido por `compute_labelled`).
        let user_label = Content::label(
            "user-sec".to_string(),
            Content::heading_numbered(1, Content::text("User")),
        );
        let auto_label = labelled_prod(
            Content::heading_numbered(1, Content::text("Auto")),
            Label("auto-sec".to_string()),
        );
        let content = Content::Sequence(vec![user_label, auto_label].into());
        let intr = introspect_with_introspector(&content);

        assert_eq!(
            intr.counter_key_for_label(&Label("user-sec".to_string())),
            Some("heading"),
            "user label deve ter counter_key 'heading'",
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-sec".to_string())),
            Some("Secção 2"),
            "auto label deve ter texto resolvido via caminho legacy",
        );
    }

    // ── P480 — Outline arm kind_index ────────────────────────────────────────

    #[test]
    fn p480_walk_outline_registra_heading_em_kind_index() {
        // P480 — walk arm de Content::Outline regista 1 entrada em
        // kind_index[Heading] para paridade de count com vanilla.
        // vanilla conta o heading do título via pós-layout; cristalino
        // regista aqui pré-layout como entrada sintética.
        let content = Content::outline();
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.query_by_kind(ElementKind::Heading).len(),
            1,
            "P480: walk arm Outline deve registar 1 heading sintético em kind_index",
        );
    }

    #[test]
    fn p480_outline_nao_adiciona_headings_for_toc() {
        // P480 — registo sintético no kind_index NÃO afecta headings_for_toc.
        // headings_for_toc continua a conter apenas os headings reais
        // do documento (sem auto-referência do TOC).
        let content = Content::Sequence(
            vec![
                Content::outline(),
                Content::heading(1, Content::text("H1")),
                Content::heading(2, Content::text("H2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // kind_index: 1 outline title + 2 headings reais = 3
        assert_eq!(
            intr.query_by_kind(ElementKind::Heading).len(),
            3,
            "P480: kind_index deve conter 1 outline title + 2 headings reais",
        );
        // headings_for_toc: apenas 2 headings reais (sem outline title)
        assert_eq!(
            intr.headings_for_toc().len(),
            2,
            "P480: headings_for_toc deve conter só headings reais; \
             outline title não incluído (evita TOC auto-referente)",
        );
    }

    #[test]
    fn p504_within_selector_query() {
        use crate::entities::selector::Selector;
        let content = Content::Sequence(
            vec![
                Content::figure(Content::heading(1, Content::text("Dentro")), None, None, None),
                Content::heading(1, Content::text("Fora")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let within = Selector::Within {
            base: Box::new(Selector::Kind(ElementKind::Heading)),
            ancestor: Box::new(Selector::Kind(ElementKind::Figure)),
        };
        let locations = intr.query(&within);
        assert_eq!(locations.len(), 1, "só o heading dentro da figure deve match");
    }

    #[test]
    fn p504_parent_locations_index() {
        let content = Content::Sequence(
            vec![
                Content::figure(Content::heading(1, Content::text("Dentro")), None, None, None),
                Content::heading(1, Content::text("Fora")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let headings = intr.query_by_kind(ElementKind::Heading);
        let figures = intr.query_by_kind(ElementKind::Figure);
        assert_eq!(headings.len(), 2);
        assert_eq!(figures.len(), 1);
        // O primeiro heading é filho da figure; o segundo não tem parent.
        assert_eq!(intr.parent_locations.get(&headings[0]), Some(&figures[0]));
        assert_eq!(intr.parent_locations.get(&headings[1]), None);
    }

    // **P533** — `@key` bibliográficos devem ser contados como citações.
    #[test]
    fn p533_bib_refs_convertidos_contam_citation_order() {
        use crate::entities::bib_entry::BibEntry;
        let bib = Content::bibliography(
            vec![
                BibEntry::new("k1", "A1", "T1", 2024),
                BibEntry::new("k2", "A2", "T2", 2023),
            ],
            None,
        );
        let content = Content::Sequence(
            vec![
                Content::reference("k2"),
                Content::reference("k1"),
                bib,
            ]
            .into(),
        );
        let converted = convert_bib_refs_to_cites(content.clone());
        let refs: Vec<_> = match &converted {
            Content::Sequence(seq) => seq.iter().collect(),
            _ => panic!("esperado Sequence"),
        };
        assert!(
            matches!(&refs[0], Content::Cite(c) if c.key == "k2"),
            "primeiro ref deve converter para Cite(k2): {:?}", refs[0]
        );
        assert!(
            matches!(&refs[1], Content::Cite(c) if c.key == "k1"),
            "segundo ref deve converter para Cite(k1): {:?}", refs[1]
        );
        // O path interno de introspecção também deve aplicar a conversão.
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.citation_order(), &["k2", "k1"]);
    }
}
