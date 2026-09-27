use typst_core::entities::content::Content;
use typst_core::entities::dir::Dir;
use typst_core::entities::layout_types::Length;
use typst_core::entities::style::{Style, Styles};

pub fn short_text() -> Content {
    Content::text("crystalline layout")
}

pub fn long_text() -> Content {
    Content::text(
        "Layout measurement should expose scaling without including fixture construction. "
            .repeat(64),
    )
}

pub fn wide_sequence(children: usize) -> Content {
    Content::sequence(
        (0..children)
            .map(|index| Content::text(format!("item-{index}")))
            .collect(),
    )
}

pub fn nested_strong(depth: usize) -> Content {
    (0..depth).fold(short_text(), |body, _| Content::strong(body))
}

pub fn vertical_stack(children: usize) -> Content {
    Content::stack(
        (0..children).map(|_| short_text()).collect(),
        Dir::TTB,
        Some(Length::pt(2.0)),
    )
}

pub fn styled_once() -> Content {
    Content::styled(Box::new(short_text()), Styles::from_iter([Style::strong()]))
}

pub fn nested_styled(depth: usize) -> Content {
    (0..depth).fold(short_text(), |body, index| {
        let style = if index % 2 == 0 { Style::strong() } else { Style::emph() };
        Content::styled(Box::new(body), Styles::from_iter([style]))
    })
}
