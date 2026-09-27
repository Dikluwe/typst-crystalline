use typst_core::entities::content::Content;
use typst_core::entities::style::Styles;

pub fn representative() -> Vec<Content> {
    let text = || Content::text("fixture");

    vec![
        Content::Empty,
        text(),
        Content::Space,
        Content::SpaceFromNewline,
        Content::Parbreak,
        Content::sequence(vec![text(), Content::Space, text()]),
        Content::par(text()),
        Content::styled(Box::new(text()), Styles::new()),
        Content::heading(2, text()),
        Content::title(text()),
        Content::strong(text()),
        Content::emph(text()),
        Content::raw("raw", None, false),
        Content::list_item(text()),
        Content::enum_item(Some(3), text()),
        Content::link("https://example.invalid", text()),
        Content::equation(Content::MathIdent("x".into()), false),
        Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
        ),
        Content::math_root(None, Content::MathIdent("x".into()), false),
        Content::math_vec(vec![
            Content::MathIdent("x".into()),
            Content::MathText("2".into()),
        ]),
        Content::math_op(Content::MathText("lim".into()), true),
        Content::label("fixture", text()),
        Content::reference("fixture"),
        Content::divider(),
        Content::terms(vec![Content::term_item(text(), text())]),
        Content::smallcaps(text()),
        Content::hide(text()),
        Content::linebreak(),
        Content::pagebreak(false, None),
        Content::colbreak(false),
        Content::flush(),
        Content::smartquote(true),
        Content::outline(),
        Content::asset("fixture.bin".into(), vec![1, 2, 3, 4]),
    ]
}
