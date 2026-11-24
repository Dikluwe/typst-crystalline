"""Replace the fixed fixture gate with a paired syntax certificate."""
from pathlib import Path
import sys

root = Path(__file__).resolve().parent / 'source'
core = root / '01_core/src/compiler/layout'
infra = root / '03_infra/src/pipeline'

def edit(path, before, after):
    data = path.read_text()
    assert data.count(before) == 1, (path, before)
    path.write_text(data.replace(before, after, 1))

if sys.argv[1] == 'spec':
    for owner, requirement in {
        'infra/pipeline/pdf': 'The fixed-fixture certificate admits only its historical template. For the isolated multi-document mode, derive an exact, bounded syntax skeleton within the validated inert outline subset. Mask only literal paragraph Text leaves immediately in the markup of a proven range loop. Preserve exact node kind, arity and other leaf bytes. Ignore only approved terminal trivia.',
        'infra/pipeline/context_stabilization/page_running': 'Select the syntax certificate in the explicit documents mode. Keep the existing outline, running, terminal and world guards. A refused certificate uses the prior non-reuse path.',
        'core/compiler/layout/physical_memo': 'The authoritative retained traversal owns the optional syntax certificate of its source. The multi-document arm must compare that certificate exactly before attempting a retained cut. Existing journal, epoch and PageStore checks all remain.',
        'core/compiler/layout/page_running_requests': 'In adaptive mode compare text payload first across every plan packet at and after the boundary, in addition to the existing admission and boundary checks. Require the exact sparse sequence of text packet ordinals and values. Keep the fixed comparison path for the prior control.',
    }.items():
        with (root / '00_nucleo/prompts' / f'{owner}.md').open('a') as stream:
            stream.write('\n\n## Multi-document adaptive suffix experiment\n\n' + requirement + '\n')
    sys.exit(0)

p = infra / 'pdf.rs'
edit(p, 'fn inert_matches(previous: &Source, current: &Source) -> bool {', '''/// Exact key for the part of the closed outline source that may not change.
pub(super) fn lab_suffix_syntax_key(source: &Source) -> Option<String> {
    use std::fmt::Write;

    fn encode(node: &SyntaxNode, in_loop: bool, mutable_text: bool, out: &mut String) {
        let kind = node.kind();
        let nested_loop = in_loop || kind == SyntaxKind::ForLoop;
        let mutable_children = nested_loop && kind == SyntaxKind::Markup;
        let children = node.children();
        write!(out, "{kind:?}/{}/", children.len()).expect("String write");
        if children.len() == 0 {
            let text = node.text();
            let bytes = if mutable_text && kind == SyntaxKind::Text {
                ""
            } else {
                text.as_str()
            };
            write!(out, "{}:", bytes.len()).expect("String write");
            out.push_str(bytes);
        } else {
            for child in children {
                encode(child, nested_loop, mutable_children, out);
            }
        }
    }

    if source.len_bytes() > 128 * 1024 || !lab_closed_outline_source(source) {
        return None;
    }
    let (prefix, _) = watch_probe_tail(source);
    let mut key = String::new();
    for node in prefix {
        encode(node, false, false, &mut key);
    }
    Some(key)
}

fn inert_matches(previous: &Source, current: &Source) -> bool {''')

p = infra / 'context_stabilization/page_running.rs'
edit(p, 'let adaptive = matches!(suffix_mode.as_deref(), Ok("adaptive"));',
     '''let documents = matches!(suffix_mode.as_deref(), Ok("documents"));
        let adaptive = documents || matches!(suffix_mode.as_deref(), Ok("adaptive"));
        let syntax = documents.then(|| super::super::pdf::lab_suffix_syntax_key(self.source)).flatten();
        let certify = if documents {
            syntax.is_some()
        } else {
            !matches!(suffix_mode.as_deref(), Ok("off"))
                && lab_page_three_fixture(self.source.text(), adaptive)
        };''')
edit(p, '''!matches!(suffix_mode.as_deref(), Ok("off"))
                && lab_page_three_fixture(self.source.text(), adaptive), adaptive).map_err''',
    'certify, adaptive, syntax).map_err')

p = core / 'physical_memo.rs'
edit(p,'sealed: &super::SealedPageRunning, certify_three: bool, adaptive: bool,',
     'sealed: &super::SealedPageRunning, certify_three: bool, adaptive: bool,\n    syntax: Option<String>,')
edit(p,'        let previous = prior.filter(|entry| {',
     '''        let previous = prior.filter(|entry| {
            if entry.syntax != syntax {
                refuse = 16;
                return false;
            }''')
edit(p,'LabThreeAttempt { previous, refuse, adaptive }','LabThreeAttempt { previous, refuse, adaptive, syntax }')
edit(p,'pages: witness.pages.clone(),\n            };',
     'pages: witness.pages.clone(),\n                syntax: witness.three.as_ref().and_then(|attempt| attempt.syntax.clone()),\n            };')
edit(p,'pub(super) struct LabThreeRetained {','pub(super) struct LabThreeRetained {\n    syntax: Option<String>,')
edit(p,'    pub(super) adaptive: bool,','    pub(super) adaptive: bool,\n    syntax: Option<String>,')

p = core / 'page_running_requests.rs'
edit(p,'''        let source = &self.source;
        let last_page = old.data.pages.total_pages().ok_or(8_u64)?;''',
     '''        let source = &self.source;
        if adaptive {
            let first = current.boundary.packet.0;
            let plain = |nodes: &[PlanNode]| {
                nodes.iter().enumerate().filter_map(|(ordinal, node)| {
                    if let Content::Text(text) = node.content.source_body() {
                        Some((ordinal, text.clone()))
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            };
            if plain(old.data.source.nodes.get(first..).ok_or(9_u64)?)
                != plain(source.nodes.get(first..).ok_or(9_u64)?)
            {
                return Err(9);
            }
        }
        let last_page = old.data.pages.total_pages().ok_or(8_u64)?;''')
edit(p,'''                || (adaptive && match (a.content.source_body(), b.content.source_body()) {
                    (Content::Text(old), Content::Text(new)) => old != new,
                    (Content::Text(_), _) | (_, Content::Text(_)) => true,
                    _ => false,
                })''','')
