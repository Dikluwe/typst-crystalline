from pathlib import Path
import json,hashlib
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/layout/paragraph.rs');checkpoint=json.load(open(out/'tests-before-code.json'));assert hashlib.sha256(p.read_bytes()).hexdigest()==checkpoint['sha256'];assert 'test result: FAILED.' in (out/'red.log').read_text() and 'panicked at' in (out/'red.log').read_text()
variants=json.load(open(out/'content-variants.json'));accepted={'Source','Empty','Space','SpaceFromNewline','Text','Sequence','Styled','Par'};pattern=lambda x:'Content::'+x['name']+({'(':'(..)','{':' { .. }',',':''}[x['shape']]);opaque='\n        | '.join(pattern(x) for x in variants if x['name'] not in accepted)
replacement='''enum ParagraphBody<'a> {
    Empty,
    Space,
    Text(&'a str),
    Sequence(&'a [Content]),
    Styled(&'a Content),
    Par,
    Opaque,
}

/// One exhaustive boundary owns the admission policy shared by these predicates.
fn paragraph_body(content: &Content) -> ParagraphBody<'_> {
    match content {
        Content::Source(e) => paragraph_body(&e.body),
        Content::Empty => ParagraphBody::Empty,
        Content::Space | Content::SpaceFromNewline => ParagraphBody::Space,
        Content::Text(text) => ParagraphBody::Text(text.as_str()),
        Content::Sequence(parts) => ParagraphBody::Sequence(parts),
        Content::Styled(styled) => ParagraphBody::Styled(&styled.body),
        Content::Par { .. } => ParagraphBody::Par,
        '''+opaque+''' => ParagraphBody::Opaque,
    }
}

pub(super) fn is_paragraph(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Par => true,
        ParagraphBody::Styled(body) => is_paragraph(body),
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Text(_)
        | ParagraphBody::Sequence(_)
        | ParagraphBody::Opaque => false,
    }
}

fn is_empty_body(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Empty | ParagraphBody::Space => true,
        ParagraphBody::Text(text) => text.is_empty(),
        ParagraphBody::Sequence(parts) => parts.iter().all(is_empty_body),
        ParagraphBody::Styled(body) => is_empty_body(body),
        ParagraphBody::Par | ParagraphBody::Opaque => false,
    }
}

fn contains_empty_text(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Text(text) => text.is_empty(),
        ParagraphBody::Sequence(parts) => parts.iter().any(contains_empty_text),
        ParagraphBody::Styled(body) => contains_empty_text(body),
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Par
        | ParagraphBody::Opaque => false,
    }
}
'''
s=p.read_text();start=s.index('pub(super) fn is_paragraph(');end=s.index('\npub(super) fn layout<',start);p.write_text(s[:start]+replacement+s[end:])
