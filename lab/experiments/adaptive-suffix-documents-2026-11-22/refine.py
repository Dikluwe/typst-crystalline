from pathlib import Path

root = Path(__file__).resolve().parent / 'source'
l0 = root / '00_nucleo/prompts/core/compiler/layout/page_running_requests.md'
with l0.open('a') as stream:
    stream.write(
        '\nThe initial census found that scanning every node after the packet ordinal refused '
        'all cuts, including the references-500 positive control. The packet ordinal is '
        'an identity, and the trace records the causal suffix. Compare exact Text values '
        'of old and current admitted packets, protected by the paired immutable syntax key '
        'along with the existing index coverage and read validation. Preserve position and '
        'label checks. Require visible edits in different sections and a negative syntax edit.\n'
    )
p = root / '01_core/src/compiler/layout/page_running_requests.rs'
s = p.read_text()
a = s.index('        if adaptive {\n            let first = current.boundary.packet.0;')
b = s.index('        let last_page =', a)
s = s[:a] + '''        fn literal_text(node: &PlanNode) -> Option<&ecow::EcoString> {
            if let Content::Text(text) = node.content.source_body() {
                Some(text)
            } else {
                None
            }
        }
''' + s[b:]
anchor = '                || std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)\n\n            {'
assert s.count(anchor) == 1
s = s.replace(anchor, anchor.replace('\n\n', '\n                || (adaptive && literal_text(a) != literal_text(b))\n'))
p.write_text(s)
