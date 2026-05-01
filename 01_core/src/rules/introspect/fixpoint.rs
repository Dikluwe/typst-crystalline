//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/fixpoint.md
//! @prompt-hash 455bbe61
//! @layer L1
//! @updated 2026-04-29
//!
//! `run_fixpoint` — orquestrador de loop de fixpoint para introspecção
//! runtime (P174 / M7 sub-passo 1).
//!
//! **Mecanismo sem clientes em P174.** Caller actual (`introspect()` +
//! Layouter) não usa fixpoint — adopção planeada para P175+ quando
//! features stdlib que dependem de `ctx.introspector` (`query`, `here`,
//! `counter.at`) materializarem.
//!
//! Loop linear, sem memoization. Detecção de convergência via hash de
//! `Vec<Tag>` produzido pelo walk. Hard cap em
//! `MAX_FIXPOINT_ITERATIONS` (5; paridade vanilla).
//!
//! Vanilla equivalente: `comemo::analyze::analyze` + memoization.

use crate::entities::content::Content;
use crate::entities::counter_state_legacy::CounterStateLegacy;
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::locator::Locator;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::tag::Tag;
use crate::rules::eval::EvalContext;
use crate::rules::introspect::convergence::compute_tags_hash;
use crate::rules::introspect::from_tags::from_tags;

/// Hard cap de iterações. Paridade com vanilla (5).
pub const MAX_FIXPOINT_ITERATIONS: usize = 5;

/// Erro do loop de fixpoint.
#[derive(Debug)]
pub enum FixpointError {
    /// Loop excedeu `MAX_FIXPOINT_ITERATIONS` sem convergir.
    NotConverged,
    /// Closure `eval_step` retornou erro.
    Eval(Vec<SourceDiagnostic>),
}

/// Orquestra um loop de fixpoint até convergência.
///
/// **Algoritmo**:
/// 1. Inicializa `prev_introspector = TagIntrospector::empty()`,
///    `prev_tags_hash = None`.
/// 2. A cada iter (até `MAX_FIXPOINT_ITERATIONS`):
///    - `ctx.introspector = prev_introspector.clone()`.
///    - `eval_step(engine, ctx)` produz `content`.
///    - Walk produz `(state, tags)`; `from_tags(_, Some(engine),
///      Some(ctx))` produz `introspector`.
///    - Se `hash(tags) == prev_tags_hash` → convergiu; retorna
///      `(state, introspector)`.
///    - Senão, actualiza `prev_*` e repete.
/// 3. Se loop sai sem convergir → `Err(NotConverged)`.
///
/// **Convergência requer dois ciclos consecutivos com mesmo hash.**
/// Doc trivial (sem queries) converge em **2 iter** — iter 0 produz
/// tags, iter 1 confirma.
///
/// **Erro de eval** (closure retorna `Err`) propaga imediatamente
/// como `Err(Eval(diagnostics))`.
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
    mut eval_step: F,
) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>,
{
    let mut prev_introspector = TagIntrospector::empty();
    let mut prev_tags_hash: Option<u64> = None;

    for _iteration in 0..MAX_FIXPOINT_ITERATIONS {
        ctx.introspector = prev_introspector.clone();

        let content = eval_step(engine, ctx).map_err(FixpointError::Eval)?;

        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        crate::rules::introspect::walk(
            &content, &mut state, &mut locator, &mut tags, None,
        );

        let curr_hash = compute_tags_hash(&tags);
        let introspector = from_tags(&tags, Some(engine), Some(ctx));

        if let Some(prev_hash) = prev_tags_hash {
            if prev_hash == curr_hash {
                return Ok((state, introspector));
            }
        }

        prev_tags_hash = Some(curr_hash);
        prev_introspector = introspector;
    }

    Err(FixpointError::NotConverged)
}

/// Entry point semanticamente claro para introspecção com fixpoint
/// (P175 / M9 sub-passo 5). Wrapper directo sobre `run_fixpoint` —
/// adopta o loop quando feature stdlib `query()` (ou similar P176+)
/// requer introspector populado de iter anterior.
///
/// **Opt-in**: callers existentes (`introspect()`, Layouter) não
/// migram. Adopção pontual quando feature explicitamente depende
/// de fixpoint.
pub fn introspect_to_fixpoint<F>(
    engine:    &mut Engine<'_>,
    ctx:       &mut EvalContext,
    eval_step: F,
) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>,
{
    run_fixpoint(engine, ctx, eval_step)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        main_id: FileId,
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId { self.main_id }
        fn source(&self, _: FileId)
            -> FileResult<crate::entities::source::Source>
        { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes>
        { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    fn make_world() -> MockWorld {
        MockWorld {
            library: Library::new(),
            book:    FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }

    /// Macro para construir Engine local — evita lifetime stuff.
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
    fn fixpoint_converge_em_doc_estavel() {
        // Closure retorna Content fixo → converge em 2 iter (iter 0
        // produz tags; iter 1 confirma com mesmo hash).
        let world = make_world();
        let result = with_engine!(&world, |engine, ctx| {
            run_fixpoint(&mut engine, &mut ctx, |_eng, _ctx| {
                Ok(Content::heading(1, Content::text("title")))
            })
        });
        assert!(matches!(result, Ok(_)));
        let (_, intr) = result.unwrap();
        // Heading indexado.
        use crate::entities::introspector::Introspector;
        use crate::entities::element_kind::ElementKind;
        assert_eq!(intr.query_by_kind(ElementKind::Heading).len(), 1);
    }

    #[test]
    fn fixpoint_excede_cap_oscilatorio() {
        // Closure oscila entre dois Contents distintos → nunca converge
        // (hash da iter N nunca igual ao hash da iter N-1).
        let world = make_world();
        let mut counter = 0usize;
        let result = with_engine!(&world, |engine, ctx| {
            run_fixpoint(&mut engine, &mut ctx, |_eng, _ctx| {
                counter += 1;
                if counter % 2 == 1 {
                    Ok(Content::heading(1, Content::text("a")))
                } else {
                    Ok(Content::heading(2, Content::text("b")))
                }
            })
        });
        assert!(matches!(result, Err(FixpointError::NotConverged)));
        // 5 iter consumidas.
        assert_eq!(counter, MAX_FIXPOINT_ITERATIONS);
    }

    #[test]
    fn fixpoint_propaga_erro_eval() {
        // Closure retorna Err logo no primeiro tick → run_fixpoint
        // devolve Eval(diagnostics).
        let world = make_world();
        let result = with_engine!(&world, |engine, ctx| {
            run_fixpoint(&mut engine, &mut ctx, |_eng, _ctx| {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "test error".to_string(),
                )])
            })
        });
        match result {
            Err(FixpointError::Eval(diags)) => {
                assert_eq!(diags.len(), 1);
            }
            _ => panic!("esperado Err(Eval(_)), recebido {:?}", result),
        }
    }

    #[test]
    fn fixpoint_introspector_actualiza_entre_iters() {
        // Closure regista `ctx.introspector.len_kinds()` em cada iter.
        // Iter 0 deve ver introspector vazio; iter 1 deve ver
        // populado pela iter 0 (1 heading).
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;

        let world = make_world();
        let mut observations: Vec<usize> = Vec::new();

        // Native func usada por StateUpdate::Func — não precisamos
        // dela, mas precisamos de algo que vire side-effect via Funcs.
        // Em vez disso, usamos directamente o closure de eval_step.
        let _ = Args::positional(vec![Value::Int(0)]); // suprime warning de unused import

        let result = with_engine!(&world, |engine, ctx| {
            run_fixpoint(&mut engine, &mut ctx, |_eng, c| {
                // Registar quantos headings o introspector da iter
                // anterior viu.
                observations.push(
                    c.introspector.query_by_kind(ElementKind::Heading).len(),
                );
                Ok(Content::heading(1, Content::text("title")))
            })
        });
        assert!(matches!(result, Ok(_)));
        // Doc estável → 2 iter. Iter 0: prev empty (0 headings).
        // Iter 1: prev populado (1 heading).
        assert!(
            observations.len() >= 2,
            "esperado >=2 observações, obtido {}",
            observations.len()
        );
        assert_eq!(observations[0], 0, "iter 0: ctx.introspector vazio");
        assert_eq!(observations[1], 1, "iter 1: introspector populado por iter 0");
    }

    // ── P175 (M9 sub-passo 5) — query + introspect_to_fixpoint ────────

    #[test]
    fn p175_query_em_doc_estavel_converge() {
        // E2E: introspect_to_fixpoint sobre Content fixo com 2 headings.
        // Convergência em 2 iter; introspector final tem 2 Heading
        // locations indexadas.
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::selector::Selector;

        let world = make_world();
        let result = with_engine!(&world, |engine, ctx| {
            introspect_to_fixpoint(&mut engine, &mut ctx, |_eng, _ctx| {
                Ok(Content::Sequence(
                    vec![
                        Content::heading(1, Content::text("um")),
                        Content::heading(1, Content::text("dois")),
                    ]
                    .into(),
                ))
            })
        });
        assert!(matches!(result, Ok(_)));
        let (_, intr) = result.unwrap();
        // Selector::Kind(Heading) retorna ambas locations.
        let locs = intr.query(&Selector::Kind(ElementKind::Heading));
        assert_eq!(locs.len(), 2);
    }

    #[test]
    fn p175_query_evolui_entre_iters_e_converge() {
        // Closure observa `ctx.introspector.query(Heading)` mas devolve
        // Content fixo. Dependência apenas via observação, sem mudança
        // de Content em função do introspector → converge na 2ª iter
        // confirmando hash igual.
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::selector::Selector;

        let world = make_world();
        let mut counts_seen: Vec<usize> = Vec::new();
        let result = with_engine!(&world, |engine, ctx| {
            introspect_to_fixpoint(&mut engine, &mut ctx, |_eng, c| {
                // Regista quantos headings o introspector anterior tem.
                let n = c.introspector
                    .query(&Selector::Kind(ElementKind::Heading)).len();
                counts_seen.push(n);
                Ok(Content::heading(1, Content::text("h")))
            })
        });
        assert!(matches!(result, Ok(_)));
        // Iter 0: introspector vazio → 0.
        // Iter 1: introspector populado por iter 0 → 1.
        // Convergência detectada (hash de tags igual entre iters).
        assert_eq!(counts_seen.len(), 2);
        assert_eq!(counts_seen[0], 0);
        assert_eq!(counts_seen[1], 1);
    }

    #[test]
    fn p175_lacuna_7_outline_kind_ausente() {
        // Lacuna #7 (`has_outline`): P175 NÃO fecha completamente.
        // `ElementKind::Outline` não existe; Content::Outline não emite
        // tag (não é payload-yielder). `query("outline")` via stdlib
        // retornaria erro de kind não reconhecido.
        //
        // Test documenta o estado: `ElementKind::from_name("outline")`
        // retorna None; lacuna fica parcial até passo dedicado adicionar
        // ElementKind::Outline + arm em extract_payload.
        use crate::entities::element_kind::ElementKind;
        assert!(
            ElementKind::from_name("outline").is_none(),
            "P175 documenta: Outline não está em ElementKind ainda"
        );
        // Mas queries para kinds existentes funcionam:
        assert!(ElementKind::from_name("heading").is_some());
        assert!(ElementKind::from_name("figure").is_some());
    }
}
