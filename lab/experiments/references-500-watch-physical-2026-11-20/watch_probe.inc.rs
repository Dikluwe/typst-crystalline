// Lab-only certification for these fixtures and comment tails. Whole-source
// match keeps edits in code, spans preceding the tail, and arbitrary show
// rules out of the experiment's positive domain.
fn watch_probe_source(text: &str) -> Option<bool> {
    const FIXTURE: &str = include_str!("watch_probe_fixture.typ");
    let (include, body) = match text.strip_prefix("#include \"middle.typ\"\n") {
        Some(body) => (true, body),
        None => (false, text),
    };
    let tail = body.strip_prefix(FIXTURE)?;
    matches!(tail, "" | "\n// integration comparison\n" | "\n// next comment; same visible source\n")
        .then_some(include)
}

struct WatchProbeEntry {
    source: typst_core::entities::file_id::FileId,
    include: bool,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    inputs: typst_core::contracts::world::SysInputs,
    font_epoch: std::sync::Arc<()>,
    document: std::sync::Arc<PagedDocument>,
}

fn watch_probe_paged(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
    session: Option<&PdfCompileSession>,
) -> (SourceResult<std::sync::Arc<PagedDocument>>, Vec<SourceDiagnostic>) {
    use std::sync::Arc;
    let ordinary = |timings: &mut Timings| {
        let (result, warnings) = super::paged::compile_paged_in_session(
            world, source, full_error, features, timings, session,
        );
        (result.map(|compiled| Arc::new(compiled.document)), warnings)
    };
    let Some(session) = session.filter(|_| std::env::var_os("TEKT_LAB_WATCH_RETAIN").is_some()) else {
        return ordinary(timings);
    };
    let mut entry = session.watch_probe.lock().unwrap();
    let Some((include, epoch)) = watch_probe_source(source.text())
        .filter(|_| world.main() == source.id() && world.library().global.is_empty())
        .zip(world.font_query_epoch())
    else {
        *entry = None;
        return ordinary(timings);
    };
    let inputs = world.inputs();
    let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
    if let Some(previous) = entry.as_ref() {
        if previous.source == source.id()
            && previous.include == include
            && previous.full_error == full_error
            && previous.features == features
            && previous.inputs == inputs
            && Arc::ptr_eq(&previous.font_epoch, &epoch)
        {
            if trace { eprintln!("watch-retain hit"); }
            return (Ok(previous.document.clone()), Vec::new());
        }
    }

    *entry = None;
    let guard = super::context_stabilization::LegacyWorldGuard::for_watch_probe(world, source);
    let (result, warnings) = super::paged::compile_paged_in_session(
        &guard, source, full_error, features, timings, Some(session),
    );
    let result = result.map(|compiled| Arc::new(compiled.document));
    let sealed = guard.sealed();
    let stable_fonts = world.font_query_epoch().is_some_and(|current| Arc::ptr_eq(&epoch, &current));
    if trace {
        eprintln!("watch-retain miss sealed={sealed} fonts={stable_fonts} warnings={}", warnings.len());
    }
    if sealed && stable_fonts && warnings.is_empty() {
        if let Ok(document) = &result {
            *entry = Some(WatchProbeEntry {
                source: source.id(), include, full_error, features, inputs,
                font_epoch: epoch, document: document.clone(),
            });
        }
    }
    (result, warnings)
}
