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

fn watch_probe_tail(source: &Source) -> (&[SyntaxNode], &[SyntaxNode]) {
    let nodes = source.root().children().as_slice();
    let end = nodes.iter().rposition(|node| {
        !watch_probe_comment(node) && !matches!(node.kind(), SyntaxKind::Space | SyntaxKind::Parbreak)
    }).map_or(0, |index| index + 1);
    nodes.split_at(end)
}

// Preserve even the position of comment nodes in the stream: the evaluator's
// statement filter tests the previous node's kind before interpreting spaces.
fn watch_probe_tail_signature(node: &SyntaxNode) -> (SyntaxKind, bool) {
    (node.kind(), node.kind() == SyntaxKind::Space && node.text().as_str().contains('\n'))
}

fn watch_probe_tail_match(left: &Source, right: &Source) -> bool {
    if left.id() != right.id() || left.root().span() != right.root().span() {
        return false;
    }
    let (a, at) = watch_probe_tail(left);
    let (b, bt) = watch_probe_tail(right);
    at.iter().map(watch_probe_tail_signature).eq(bt.iter().map(watch_probe_tail_signature))
        && a.len() == b.len() && a.iter().zip(b).all(|(a,b)| watch_probe_nodes_match(a,b,0))
}

fn watch_probe_alternative(source: &Source) -> Option<Source> {
    let (prefix, tail) = watch_probe_tail(source);
    if prefix.is_empty() || !tail.iter().map(watch_probe_tail_signature)
        .eq([(SyntaxKind::Space, true)])
    {
        return None;
    }
    let end = prefix.iter().map(SyntaxNode::len).sum::<usize>();
    let alternative = source.reparse(format!("{}\n\n//\n", source.text().get(..end)?)).ok()?;
    if !watch_probe_source_eligible(&alternative)
        || alternative.root().span() != source.root().span()
    {
        return None;
    }
    let (other, tail) = watch_probe_tail(&alternative);
    (prefix.len() == other.len()
        && prefix.iter().zip(other).all(|(a,b)| watch_probe_nodes_match(a,b,0))
        && tail.iter().map(watch_probe_tail_signature).eq([
            (SyntaxKind::Parbreak,false), (SyntaxKind::LineComment,false), (SyntaxKind::Space,true)
        ])).then_some(alternative)
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
        visit(source, watch_probe_tail(source).0.iter(), &mut offset, &mut rows);
        rows
    }
    let before = inspect(previous);
    let after = inspect(current);
    eprintln!("{event} spans={} old={} new={} root_id_equal={} root_bytes={}->{}",
        before == after, before.len(), after.len(),
        previous.root().span() == current.root().span(), previous.len_bytes(), current.len_bytes());
}
fn inert_children(node: &SyntaxNode) -> impl Iterator<Item = &SyntaxNode> {
    node.children().filter(|node| node.kind() != SyntaxKind::Space)
}

fn inert_identifier(node: &SyntaxNode, name: &str) -> bool {
    node.kind() == SyntaxKind::Ident && node.text().as_str() == name
}

fn inert_plain(node: &SyntaxNode) -> bool {
    matches!(node.kind(), SyntaxKind::Text | SyntaxKind::Space | SyntaxKind::Parbreak)
        && node.children().len() == 0
        && node.text().as_str().is_ascii()
}

fn inert_named(node: &SyntaxNode) -> Option<(&SyntaxNode, &SyntaxNode)> {
    if node.kind() != SyntaxKind::Named {
        return None;
    }
    let mut parts = inert_children(node);
    let name = parts.next()?;
    let colon = parts.next()?;
    let value = parts.next()?;
    (colon.kind() == SyntaxKind::Colon && parts.next().is_none()).then_some((name, value))
}

fn inert_args(node: &SyntaxNode) -> Option<Vec<&SyntaxNode>> {
    if node.kind() != SyntaxKind::Args {
        return None;
    }
    let mut parts = inert_children(node);
    if parts.next()?.kind() != SyntaxKind::LeftParen {
        return None;
    }
    let mut args = Vec::new();
    loop {
        let next = parts.next()?;
        if next.kind() == SyntaxKind::RightParen {
            return parts.next().is_none().then_some(args);
        }
        args.push(next);
        match parts.next()?.kind() {
            SyntaxKind::RightParen => return parts.next().is_none().then_some(args),
            SyntaxKind::Comma => {}
            _ => return None,
        }
    }
}

fn inert_call<'a>(node: &'a SyntaxNode, name: &str) -> Option<Vec<&'a SyntaxNode>> {
    let mut parts = inert_children(node);
    if node.kind() != SyntaxKind::FuncCall || !inert_identifier(parts.next()?, name) {
        return None;
    }
    let args = inert_args(parts.next()?)?;
    parts.next().is_none().then_some(args)
}

fn inert_set(node: &SyntaxNode) -> Option<bool> {
    let mut parts = inert_children(node);
    if node.kind() != SyntaxKind::SetRule || parts.next()?.kind() != SyntaxKind::Set {
        return None;
    }
    let target = parts.next()?;
    let args = inert_args(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    if inert_identifier(target, "page") {
        let mut width = false;
        let mut height = false;
        for arg in args {
            let (name, value) = inert_named(arg)?;
            let raw = value.text();
            let points = raw.as_str().strip_suffix("pt")?.parse::<f64>().ok()?;
            if value.kind() != SyntaxKind::Numeric || !(72.0..=2000.0).contains(&points) {
                return None;
            }
            if inert_identifier(name, "width") && !width {
                width = true;
            } else if inert_identifier(name, "height") && !height {
                height = true;
            } else {
                return None;
            }
        }
        return (width && height).then_some(true);
    }
    if inert_identifier(target, "heading") {
        let [arg] = args.as_slice() else { return None };
        let (name, value) = inert_named(arg)?;
        return (inert_identifier(name, "numbering") && value.kind() == SyntaxKind::Str
            && value.text().as_str().is_ascii()).then_some(false);
    }
    None
}

fn inert_loop(node: &SyntaxNode) -> Option<()> {
    let mut parts = inert_children(node);
    if node.kind() != SyntaxKind::ForLoop || parts.next()?.kind() != SyntaxKind::For {
        return None;
    }
    let binding = parts.next()?;
    if binding.kind() != SyntaxKind::Ident || parts.next()?.kind() != SyntaxKind::In {
        return None;
    }
    let args = inert_call(parts.next()?, "range")?;
    let [count] = args.as_slice() else { return None };
    let count_text = count.text();
    if count.kind() != SyntaxKind::Int
        || !(1..=512).contains(&count_text.as_str().parse::<usize>().ok()?)
    {
        return None;
    }
    let block = parts.next()?;
    if block.kind() != SyntaxKind::ContentBlock || parts.next().is_some() {
        return None;
    }
    let [left, markup, right] = block.children().as_slice() else { return None };
    if left.kind() != SyntaxKind::LeftBracket || markup.kind() != SyntaxKind::Markup
        || right.kind() != SyntaxKind::RightBracket
    {
        return None;
    }
    let mut body = markup.children();
    let last = body.next_back()?;
    if last.kind() != SyntaxKind::Parbreak || !inert_plain(last) {
        return None;
    }
    let mut material = false;
    while let Some(child) = body.next() {
        if child.kind() == SyntaxKind::Hash {
            let name = body.next()?;
            if !inert_identifier(name, binding.text().as_str()) {
                return None;
            }
            material = true;
        } else if inert_plain(child) {
            // A single material paragraph closes at the explicit final break.
            if child.kind() == SyntaxKind::Parbreak {
                return None;
            }
            material |= child.kind() == SyntaxKind::Text;
        } else {
            return None;
        }
    }
    material.then_some(())
}

fn inert_source(source: &Source) -> bool {
    let (prefix, tail) = watch_probe_tail(source);
    if !tail.iter().map(watch_probe_tail_signature).eq([(SyntaxKind::Space, true)])
        || prefix.last().is_none_or(|node| inert_loop(node).is_none())
    {
        return false;
    }
    let mut fixed_page = false;
    let mut in_body = false;
    let mut nodes = prefix.iter();
    while let Some(node) = nodes.next() {
        let safe = match node.kind() {
            SyntaxKind::Hash => match nodes.next() {
                Some(next) if next.kind() == SyntaxKind::SetRule && !in_body => {
                    inert_set(next).is_some_and(|page| {
                        if page && fixed_page {
                            return false;
                        }
                        fixed_page |= page;
                        true
                    })
                }
                Some(next) if next.kind() == SyntaxKind::ForLoop => {
                    in_body = true;
                    inert_loop(next).is_some()
                },
                Some(next) => inert_call(next, "outline").is_some_and(|args| {
                    args.is_empty() || matches!(args.as_slice(), [arg] if
                        inert_named(arg).is_some_and(|(name, value)|
                            inert_identifier(name, "title") && value.kind() == SyntaxKind::None))
                }),
                None => false,
            },
            SyntaxKind::Heading => node.children().all(|part| match part.kind() {
                SyntaxKind::HeadingMarker => part.text().as_str().is_ascii(),
                SyntaxKind::Markup => part.children().all(inert_plain),
                _ => inert_plain(part),
            }),
            SyntaxKind::Ref => matches!(node.children().as_slice(), [marker] if
                marker.kind() == SyntaxKind::RefMarker && marker.text().as_str().is_ascii()),
            SyntaxKind::Label => node.text().as_str().is_ascii(),
            _ => inert_plain(node) || watch_probe_comment(node),
        };
        if !safe {
            return false;
        }
        in_body |= matches!(node.kind(), SyntaxKind::Text | SyntaxKind::Heading | SyntaxKind::Ref | SyntaxKind::Label);
    }
    fixed_page
}

fn inert_matches(previous: &Source, current: &Source) -> bool {
    if previous.id() != current.id() || previous.root().span() != current.root().span() {
        return false;
    }
    let (prefix, _) = watch_probe_tail(previous);
    let (new_prefix, tail) = watch_probe_tail(current);
    tail.iter().map(watch_probe_tail_signature).eq([
        (SyntaxKind::Parbreak, false), (SyntaxKind::LineComment, false), (SyntaxKind::Space, true),
    ]) && prefix.len() == new_prefix.len()
        && prefix.iter().zip(new_prefix).all(|(a, b)| watch_probe_nodes_match(a, b, 0))
}

fn inert_terminal_break(document: &PagedDocument) -> bool {
    use typst_core::entities::layout_types::{FrameItem, SemanticKind};

    fn boundary(items: &[FrameItem]) -> Option<bool> {
        items.iter().rev().find_map(|item| match item {
            FrameItem::Semantic { kind: SemanticKind::ParbreakBoundary, .. } => Some(true),
            FrameItem::Semantic { items, .. } | FrameItem::Group { items, .. } => boundary(items),
            FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } if text.is_empty() => None,
            _ => Some(false),
        })
    }
    document.pages.last().and_then(|page| boundary(&page.items)) == Some(true)
}

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use typst_core::contracts::{plugin_host::PluginHost, world::SysInputs};
use typst_core::entities::{
    duration::Duration, file_id::FileId, font_book::{FontBook, FontInfo, FontVariant},
    font_list::FontNamePattern, package_spec::PackageSpec, path::RootedPath,
    world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
};

/// During speculative preparation, every unsupported input invalidates
/// the product. The only file that can be read is the synthetic main CST.
struct WatchProbeShadow<'a> {
    source: &'a Source,
    library: &'a Library,
    inputs: &'a SysInputs,
    epoch: &'a Arc<()>,
    slots: Vec<crate::fonts::FontSlot>,
    book: FontBook,
    rejected: AtomicBool,
}

impl WatchProbeShadow<'_> {
    fn reject<T>(&self, fallback: T) -> T {
        self.rejected.store(true, Ordering::Relaxed);
        fallback
    }
}

impl World for WatchProbeShadow<'_> {
    fn font_query_epoch(&self) -> Option<Arc<()>> {
        (!self.rejected.load(Ordering::Relaxed)).then(|| self.epoch.clone())
    }
    fn library(&self) -> &Library { self.library }
    fn inputs(&self) -> SysInputs { self.inputs.clone() }
    fn main(&self) -> FileId { self.source.id() }
    fn book(&self) -> &FontBook { self.reject(&self.book) }
    fn select_font(&self, pattern: &FontNamePattern, variant: &FontVariant) -> Option<usize> {
        if matches!(pattern, FontNamePattern::Literal(_)) {
            if let Some(index) = self.book.select_pattern(pattern, variant) {
                let info = self.book.infos()[index].variant;
                if info.weight.distance(variant.weight) == 0
                    && info.style.distance(variant.style) == 0
                {
                    return Some(index);
                }
            }
        }
        self.reject(None)
    }
    fn font_info(&self, index: usize) -> Option<&FontInfo> {
        self.book.infos().get(index).or_else(|| self.reject(None))
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.slots.get(index).and_then(crate::fonts::FontSlot::get)
            .or_else(|| self.reject(None))
    }
    fn candidates_for_char(&self, _: char) -> Vec<usize> { self.reject(Vec::new()) }
    fn source(&self, id: FileId) -> FileResult<Source> {
        self.immutable_source(id).ok_or(FileError::Other("external source".into()))
    }
    fn immutable_source(&self, id: FileId) -> Option<Source> {
        if id == self.main() { Some(self.source.clone()) } else { self.reject(None) }
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        self.reject(Err(FileError::Other("external file".into())))
    }
    fn resolve_path(&self, _: FileId, _: &str) -> Result<RootedPath, String> {
        self.reject(Err("external path".into()))
    }
    fn read_path(&self, _: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        self.reject(Err("external bytes".into()))
    }
    fn include_path(&self, _: &RootedPath) -> Result<Source, String> {
        self.reject(Err("external include".into()))
    }
    fn read_bytes(&self, _: FileId, _: &str) -> Result<Arc<Vec<u8>>, String> {
        self.reject(Err("external bytes".into()))
    }
    fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> {
        self.reject(Err("external include".into()))
    }
    fn resolve_package(&self, _: &PackageSpec) -> Result<Source, String> {
        self.reject(Err("external package".into()))
    }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> { self.reject(None) }
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>> { self.reject(None) }
}

#[derive(Default)]
struct WatchProbeState {
    entry: Option<WatchProbeEntry>,
    attempted: bool,
}

struct WatchProbeEntry {
    source: Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    inputs: typst_core::contracts::world::SysInputs,
    font_epoch: std::sync::Arc<()>,
    document: Arc<PagedDocument>,
    alternative: Option<(Source, Arc<PagedDocument>)>,
    inert_ready: bool,
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
    let mut state = session.watch_probe.lock().unwrap();
    let Some(epoch) = world.font_query_epoch().filter(|_| {
        watch_probe_source_eligible(source) && world.main() == source.id()
            && world.library().global.is_empty()
    })
    else {
        state.entry = None;
        return ordinary(timings);
    };
    let inputs = world.inputs();
    let inert_mode = std::env::var_os("TEKT_LAB_WATCH_INERT").is_some();
    let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
    if let Some(previous) = state.entry.as_ref() {
        if previous.full_error == full_error
            && previous.features == features
            && previous.inputs == inputs
            && Arc::ptr_eq(&previous.font_epoch, &epoch)
        {
            if watch_probe_comments_match(&previous.source, source)
                || watch_probe_tail_match(&previous.source, source)
            {
                if trace { watch_probe_spans("watch-retain hit", &previous.source, source); }
                return (Ok(previous.document.clone()), Vec::new());
            }
            if inert_mode && previous.inert_ready && inert_matches(&previous.source, source) {
                if trace { watch_probe_spans("watch-retain hit inert", &previous.source, source); }
                return (Ok(previous.document.clone()), Vec::new());
            }
            if let Some((alternative, document)) = &previous.alternative {
                if watch_probe_tail_match(alternative, source) {
                    if trace { watch_probe_spans("watch-retain hit alternative", alternative, source); }
                    return (Ok(document.clone()), Vec::new());
                }
            }
        }
        if trace {
            watch_probe_spans("watch-retain source-miss", &previous.source, source);
        }
    }
    state.entry = None;
    let warm = !std::mem::replace(&mut state.attempted, true);
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
            let alternative = if warm && !inert_mode && std::env::var_os("TEKT_LAB_WATCH_LAZY").is_none() {
                watch_probe_alternative(source).and_then(|alternative| {
                    let embedded = crate::embedded_fonts::load_embedded_fonts();
                    let shadow = WatchProbeShadow {
                        source: &alternative, library: world.library(), inputs: &inputs,
                        epoch: &epoch, slots: embedded.text_slots, book: embedded.text_book,
                        rejected: AtomicBool::new(false),
                    };
                    if trace { eprintln!("watch-retain alternative-start"); }
                    let (compiled, warnings) = super::paged::compile_paged_in_session(
                        &shadow, &alternative, full_error, features, &mut Timings::default(), None,
                    );
                    let sealed = shadow.font_query_epoch().is_some();
                    let document = compiled.ok().map(|compiled| compiled.document);
                    let admitted = sealed && warnings.is_empty() && document.as_ref().is_some_and(|doc|
                        doc.pages.len() <= 128 && doc.attachments.is_empty());
                    if trace { eprintln!("watch-retain alternative-end admitted={admitted}"); }
                    if admitted { document.map(|doc| (alternative, Arc::new(doc))) } else { None }
                })
            } else {
                None
            };
            let inert_ready = inert_mode && inert_source(source) && inert_terminal_break(document);
            if trace && inert_mode {
                eprintln!("watch-retain inert-ready={inert_ready} grammar={} terminal={}",
                    inert_source(source), inert_terminal_break(document));
            }
            state.entry = Some(WatchProbeEntry {
                source: source.clone(), full_error, features, inputs,
                font_epoch: epoch, document: document.clone(), alternative, inert_ready,
            });
        }
    }
    (result, warnings)
}
