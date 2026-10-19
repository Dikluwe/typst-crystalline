//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/runtime.md
//! @prompt-hash d1ba106b
//! @layer L3
//! @updated 2026-10-17

use super::*;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::introspector::TagIntrospector;

pub(super) fn with_introspection_runtime<R>(
    world: &dyn World,
    source: &Source,
    target: EvalTarget,
    features: Features,
    operation: impl FnOnce(&mut Engine<'_>, &mut EvalContext) -> R,
) -> (R, Vec<SourceDiagnostic>) {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    ctx.target = target;
    ctx.features = features;
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        operation(&mut engine, &mut ctx)
    };
    (result, sink.into_diagnostics())
}

pub(super) fn prepare_heading_supplements(
    intr: &mut TagIntrospector,
    world: &dyn World,
    source: &Source,
    target: EvalTarget,
    features: Features,
) -> (SourceResult<()>, Vec<SourceDiagnostic>) {
    with_introspection_runtime(world, source, target, features, |engine, ctx| {
        typst_core::compiler::introspect::realize_heading_supplements(intr, engine, ctx)
    })
}
