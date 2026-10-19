from pathlib import Path
p=Path('01_core/src/compiler/eval/operators/equality.rs');s=p.read_text();s=s.replace('use crate::entities::color::Color;','use crate::entities::color::Color;\nuse crate::entities::content::Content;')
s=s.replace('a.morph_canon() == b.morph_canon()', 'content_language_eq(&a, &b)').replace('a.morph_canon() != b.morph_canon()', '!content_language_eq(&a, &b)').replace('x.morph_canon() == y.morph_canon()', 'content_language_eq(x, y)')
# a/b are already references in the raw/None snapshot fallback.
s=s.replace('return content_language_eq(&a, &b);','return content_language_eq(a, b);')
i=s.index('/// Whether the selector')
s=s[:i]+'''/// Comparison-only projection: Styled identity is its child, not its map.
/// Keep the public morphology projection and Rust equality untouched.
fn content_language_eq(a: &Content, b: &Content) -> bool {
    fn project(content: &Content) -> Content {
        content
            .morph_canon()
            .map_content(&mut |node| {
                Ok(Some(match node {
                    Content::Styled(styled) => {
                        let mut body = styled.body.clone();
                        while let Content::Styled(inner) = &body {
                            body = inner.body.clone();
                        }
                        Content::styled(Box::new(body), crate::entities::style::Styles::new())
                    }
                    other => other.clone(),
                }))
            })
            .expect("content equality: comparison projection is total")
    }
    project(a) == project(b)
}

'''+s[i:];p.write_text(s)
