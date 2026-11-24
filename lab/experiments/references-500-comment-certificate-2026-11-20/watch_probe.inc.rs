use typst_core::entities::{syntax_kind::SyntaxKind, syntax_node::SyntaxNode};

fn watch_probe_comment(node: &SyntaxNode) -> bool {
    matches!(node.kind(), SyntaxKind::LineComment | SyntaxKind::BlockComment)
        && node.children().len() == 0
}

fn watch_probe_source_eligible(source: &Source) -> bool {
    source.len_bytes() > 0
        && source.len_bytes() <= 512 * 1024
        && source.root().kind() == SyntaxKind::Markup
        && !source.root().erroneous()
}

fn watch_probe_nodes_match(left: &SyntaxNode, right: &SyntaxNode, depth: usize) -> bool {
    if depth == 128 || left.kind() != right.kind() || left.len() != right.len() {
        return false;
    }
    if watch_probe_comment(left) && watch_probe_comment(right) {
        let footprint = |c: char| {
            let line_break = matches!(c, '\r' | '\n' | '\u{2028}' | '\u{2029}').then_some(c);
            (c.len_utf8(), line_break)
        };
        return left.text().as_str().chars().map(footprint)
            .eq(right.text().as_str().chars().map(footprint));
    }
    if left.span() != right.span() || left.children().len() != right.children().len() {
        return false;
    }
    if left.children().len() == 0 {
        return left.text() == right.text();
    }
    left.children().zip(right.children())
        .all(|(a, b)| watch_probe_nodes_match(a, b, depth + 1))
}

fn watch_probe_root_prefix(root: &SyntaxNode) -> &[SyntaxNode] {
    let nodes = root.children().as_slice();
    match nodes.split_last() {
        Some((last, prefix)) if watch_probe_comment(last) => prefix,
        _ => nodes,
    }
}

fn watch_probe_comments_match(left: &Source, right: &Source) -> bool {
    if left.id() != right.id() || left.root().span() != right.root().span() {
        return false;
    }
    let a = watch_probe_root_prefix(left.root());
    let b = watch_probe_root_prefix(right.root());
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| watch_probe_nodes_match(a, b, 0))
}

// Trace the span identifiers, byte intervals and source coordinates of all
// compared syntax outside comments. The root is reported separately for EOF.
fn watch_probe_spans(event: &str, previous: &Source, current: &Source) {
    fn inspect(source: &Source) -> Vec<(typst_core::entities::span::Span, usize, usize, Option<(u32, u32)>)> {
        fn visit<'a>(
            source: &Source,
            nodes: impl Iterator<Item = &'a SyntaxNode>,
            offset: &mut usize,
            output: &mut Vec<(typst_core::entities::span::Span, usize, usize, Option<(u32, u32)>)>,
        ) {
            for node in nodes {
                let end = *offset + node.len();
                if !watch_probe_comment(node) {
                    output.push((node.span(), *offset, end, source.span_to_line_col(node.span())));
                    visit(source, node.children(), offset, output);
                }
                *offset = end;
            }
        }
        let mut rows = Vec::new();
        let mut offset = 0;
        visit(source, source.root().children(), &mut offset, &mut rows);
        rows
    }
    let before = inspect(previous);
    let after = inspect(current);
    eprintln!("{event} spans={} old={} new={} root_id_equal={} root_bytes={}->{}",
        before == after, before.len(), after.len(),
        previous.root().span() == current.root().span(), previous.len_bytes(), current.len_bytes());
}
struct WatchProbeEntry {
    source: Source,
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
    let Some(epoch) = world.font_query_epoch().filter(|_| {
        watch_probe_source_eligible(source) && world.main() == source.id()
            && world.library().global.is_empty()
    })
    else {
        *entry = None;
        return ordinary(timings);
    };
    let inputs = world.inputs();
    let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
    if let Some(previous) = entry.as_ref() {
        if previous.full_error == full_error
            && previous.features == features
            && previous.inputs == inputs
            && Arc::ptr_eq(&previous.font_epoch, &epoch)
            && watch_probe_comments_match(&previous.source, source)
        {
            if trace { watch_probe_spans("watch-retain hit", &previous.source, source); }
            return (Ok(previous.document.clone()), Vec::new());
        }
    }

    if trace {
        if let Some(previous) = entry.as_ref() {
            watch_probe_spans("watch-retain source-miss", &previous.source, source);
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
            if document.pages.len() > 128 || !document.attachments.is_empty() {
                return (result, warnings);
            }
            *entry = Some(WatchProbeEntry {
                source: source.clone(), full_error, features, inputs,
                font_epoch: epoch, document: document.clone(),
            });
        }
    }
    (result, warnings)
}
