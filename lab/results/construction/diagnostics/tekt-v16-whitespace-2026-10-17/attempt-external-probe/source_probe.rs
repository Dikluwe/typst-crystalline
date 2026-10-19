pub use typst_core::entities;
mod measured {

use crate::entities::content::Content;

fn space_origin(content: &Content) -> Option<bool> {
    match content {
        Content::Realized(e) => space_origin(&e.visual),
        Content::Source(e) => space_origin(&e.body),
        Content::Located(e) => space_origin(&e.body),
        Content::Space => Some(false),
        Content::SpaceFromNewline => Some(true),
        Content::Styled(styled) => space_origin(&styled.body),
        _ => None,
    }
}

fn space_with_origin(original: &Content, had_newline: bool) -> Content {
    match original {
        Content::Source(e) => {
            space_with_origin(&e.body, had_newline).with_source_span(e.span)
        }
        Content::Styled(styled) => Content::styled(
            space_with_origin(&styled.body, had_newline),
            styled.styles.clone(),
        ),
        _ if had_newline => Content::SpaceFromNewline,
        _ => Content::Space,
    }
}

fn previous_visible(parts: &[Content], before: usize) -> Option<usize> {
    (0..before).rev().find(|&index| !parts[index].is_empty())
}

fn next_visible(parts: &[Content], from: usize) -> Option<usize> {
    (from..parts.len()).find(|&index| !parts[index].is_empty())
}

/// Texto de um wrapper de estilo pode informar o descarte do espaço exterior,
/// embora o wrapper continue a interromper matching literal/regex.
fn supportive_text(content: &Content) -> String {
    match content {
        Content::Realized(e) => supportive_text(&e.visual),
        Content::Located(e) => supportive_text(&e.body),
        Content::Source(e) => supportive_text(&e.body),
        Content::ShowRecipe(e) => supportive_text(&e.body),
        Content::Text(text) => text.to_string(),
        Content::Styled(..)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. } => content.plain_text(),
        _ => String::new(),
    }
}

// A canonical occurrence already has the representation produced by space_with_origin.
fn canonical_space(content: &Content, had_newline: bool) -> bool {
    match content {
        Content::Space => !had_newline,
        Content::SpaceFromNewline => had_newline,
        Content::Styled(e) => canonical_space(&e.body, had_newline),
        Content::Source(e) => {
            !e.span.is_detached()
                && e.body.source_span().is_detached()
                && canonical_space(&e.body, had_newline)
        }
        _ => false,
    }
}

struct SpaceRun {
    end: usize,
    had_newline: bool,
    discard: bool,
    repeated: bool,
}

// Both consumers inspect the current state, including earlier discarded groups.
fn next_space_run(parts: &[Content], mut start: usize) -> Option<(usize, SpaceRun)> {
    while start < parts.len() {
        let Some(mut had_newline) = space_origin(&parts[start]) else {
            start += 1;
            continue;
        };
        let mut end = start + 1;
        let mut repeated = false;
        while end < parts.len() {
            match space_origin(&parts[end]) {
                Some(origin) => {
                    had_newline |= origin;
                    repeated = true;
                }
                None if matches!(&parts[end], Content::Empty) => {}
                None => break,
            }
            end += 1;
        }
        let before = previous_visible(parts, start);
        let after = next_visible(parts, end);
        let destructive = before
            .is_some_and(|index| matches!(&parts[index], Content::Linebreak(_)))
            || after.is_some_and(|index| matches!(&parts[index], Content::Linebreak(_)));
        // An isolated newline must retain its origin until insertion.
        let discard = destructive
            || (had_newline
                && before.is_some()
                && after.is_some()
                && discard_space_between(
                    &supportive_text(&parts[before.unwrap()]),
                    &supportive_text(&parts[after.unwrap()]),
                ));
        return Some((start, SpaceRun { end, had_newline, discard, repeated }));
    }
    None
}

fn apply_space_run(parts: &mut [Content], start: usize, run: &SpaceRun) {
    parts[start] = if run.discard {
        Content::Empty
    } else {
        space_with_origin(&parts[start], run.had_newline)
    };
    for part in &mut parts[start + 1..run.end] {
        if space_origin(part).is_some() {
            *part = Content::Empty;
        }
    }
}

pub(crate) fn normalize_markup_spaces(parts: &mut [Content]) {
    let mut from = 0;
    while let Some((start, run)) = next_space_run(parts, from) {
        from = run.end;
        apply_space_run(parts, start, &run);
    }
}

pub(crate) fn normalize_markup_spaces_if_changed(
    parts: &[Content],
) -> Option<Vec<Content>> {
    let mut output: Option<Vec<Content>> = None;
    let mut from = 0;
    while let Some((start, run)) =
        next_space_run(output.as_deref().unwrap_or(parts), from)
    {
        from = run.end;
        if run.discard
            || run.repeated
            || !canonical_space(
                &output.as_deref().unwrap_or(parts)[start],
                run.had_newline,
            )
        {
            apply_space_run(output.get_or_insert_with(|| parts.to_vec()), start, &run);
        }
    }
    output
}

/// Predicado Unicode puro equivalente à superfície do pin vanilla:
/// Han/Bopomofo/Hiragana/Katakana/Yi e pontuação ideográfica não recebem
/// espaços inferidos; coreano, aspas ambíguas e emoji conservam-nos.
fn discard_space_between(left: &str, right: &str) -> bool {
    use icu_properties::props::{EastAsianWidth, Emoji, Script};
    use icu_properties::{CodePointMapDataBorrowed, CodePointSetDataBorrowed};

    fn spacingless(c: char) -> bool {
        const SCRIPTS: CodePointMapDataBorrowed<Script> = CodePointMapDataBorrowed::new();
        const WIDTHS: CodePointMapDataBorrowed<EastAsianWidth> =
            CodePointMapDataBorrowed::new();
        const EMOJI: CodePointSetDataBorrowed = CodePointSetDataBorrowed::new::<Emoji>();

        match SCRIPTS.get(c) {
            Script::Han
            | Script::Bopomofo
            | Script::Hiragana
            | Script::Katakana
            | Script::Yi => true,
            Script::Inherited if matches!(c, '\u{302A}'..='\u{302D}' | '\u{3099}'..='\u{309A}') => {
                true
            }
            Script::Common => match WIDTHS.get(c) {
                EastAsianWidth::Fullwidth => true,
                EastAsianWidth::Halfwidth if c != '₩' => true,
                EastAsianWidth::Wide if !EMOJI.contains(c) => true,
                EastAsianWidth::Ambiguous if matches!(c, '㉈'..='㉏') => true,
                _ => false,
            },
            _ => false,
        }
    }

    left.chars().next_back().is_some_and(spacingless)
        || right.chars().next().is_some_and(spacingless)
}

#[cfg(test)]
mod p1494_space_tests {
    use super::{discard_space_between, normalize_markup_spaces};
    use crate::entities::content::Content;
    use crate::entities::style::Styles;

    #[test]
    fn pin_unicode_spacing_controls() {
        for (left, right, discard) in [
            ("水", "果", true),
            ("换行", "newline", true),
            ("newline", "换行", true),
            ("쉼표，", "줄", true),
            ("줄", "바꿈", false),
            ("new", "line", false),
            ("“你好”", "“你好”", false),
            ("🎃", "🎃", false),
            ("A", "（B", true),
            ("A", "₩", false),
        ] {
            assert_eq!(discard_space_between(left, right), discard, "{left:?}/{right:?}");
        }
    }

    #[test]
    fn styled_space_preserva_origem_ate_observar_vizinhos() {
        let styled =
            Content::styled(Box::new(Content::SpaceFromNewline), Styles::default());
        let mut isolated = [styled.clone()];
        normalize_markup_spaces(&mut isolated);
        assert!(matches!(
            &isolated[0],
            Content::Styled(styled) if matches!(&styled.body, Content::SpaceFromNewline)
        ));

        let mut between_han = [Content::text("水"), styled, Content::text("果")];
        normalize_markup_spaces(&mut between_han);
        assert!(matches!(&between_han[1], Content::Empty));
    }
}

#[cfg(test)]
mod normalization_change_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use crate::entities::style::Styles;
    use std::sync::Arc;

    #[test]
    fn canonical_spaces_and_empty_slots_need_no_output() {
        for space in [
            Content::Space,
            Content::SpaceFromNewline,
            Content::styled(Box::new(Content::Space), Styles::default()),
        ] {
            assert!(normalize_markup_spaces_if_changed(&[space.clone(), Content::Empty])
                .is_none());
            assert!(normalize_markup_spaces_if_changed(&[
                Content::text("left"),
                space,
                Content::text("right")
            ])
            .is_none());
        }
        assert!(normalize_markup_spaces_if_changed(&[
            Content::text("水"),
            Content::Space,
            Content::text("果")
        ])
        .is_none());
        assert!(normalize_markup_spaces_if_changed(&[
            Content::text("줄"),
            Content::SpaceFromNewline,
            Content::text("바꿈")
        ])
        .is_none());
    }

    #[test]
    fn changed_groups_keep_unicode_origin_and_style() {
        let input = [
            Content::text("left"),
            Content::styled(Box::new(Content::Space), Styles::default()),
            Content::Empty,
            Content::SpaceFromNewline,
            Content::text("right"),
        ];
        let output = normalize_markup_spaces_if_changed(&input).unwrap();
        assert!(
            matches!(&output[1], Content::Styled(e) if matches!(e.body, Content::SpaceFromNewline))
        );
        assert!(matches!(&output[2], Content::Empty));
        assert!(matches!(&output[3], Content::Empty));
        let han = normalize_markup_spaces_if_changed(&[
            Content::text("水"),
            Content::SpaceFromNewline,
            Content::text("果"),
        ])
        .unwrap();
        assert!(matches!(&han[1], Content::Empty));
    }

    #[test]
    fn space_carriers_keep_previous_normalization() {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let span = Span::from_range(file, 1..2);
        let sourced = Content::Space.with_source_span(span);
        assert!(
            normalize_markup_spaces_if_changed(std::slice::from_ref(&sourced)).is_none()
        );
        for carrier in [
            Content::Source(Arc::new(SourceContent {
                body: Content::Space,
                span: Span::detached(),
            })),
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(9),
                body: Content::Space,
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("origin"),
                visual: Content::Space,
            })),
        ] {
            assert!(matches!(
                &normalize_markup_spaces_if_changed(&[carrier]).unwrap()[0],
                Content::Space
            ));
        }
        let nested = Content::Source(Arc::new(SourceContent {
            body: sourced,
            span: Span::from_range(file, 3..4),
        }));
        let output = normalize_markup_spaces_if_changed(&[nested]).unwrap();
        assert_eq!(output[0].source_span(), span);
        assert!(
            matches!(&output[0], Content::Source(e) if matches!(e.body, Content::Space))
        );
    }

    #[test]
    fn destructive_groups_observe_previously_normalized_neighbors() {
        let input = [
            Content::text("left"),
            Content::Space,
            Content::linebreak(),
            Content::SpaceFromNewline,
            Content::text("right"),
        ];
        let output = normalize_markup_spaces_if_changed(&input).unwrap();
        assert!(matches!(&output[1], Content::Empty));
        assert!(matches!(&output[2], Content::Linebreak(_)));
        assert!(matches!(&output[3], Content::Empty));
        let mut inplace = input.clone();
        normalize_markup_spaces(&mut inplace);
        assert_eq!(inplace.as_slice(), output.as_slice());
        assert!(normalize_markup_spaces_if_changed(&output).is_none());
    }
}

 fn run(parts: &[Content])->Vec<Content> {let mut out=parts.to_vec();normalize_markup_spaces(&mut out);out}
 pub fn measure(){
  use crate::entities::{span::Span,file_id::FileId};
  let span=Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),1..2);
  fn view(c:&Content)->String {match c.source_body(){Content::Empty=>"_".into(),Content::Space=>"S".into(),Content::SpaceFromNewline=>"N".into(),Content::Linebreak(_)=>"L".into(),Content::Parbreak=>"P".into(),Content::Text(t)=>format!("T:{t}"),other=>format!("{other:?}")}}
  for name in ["break-right","break-left","empty-run","empty-run-reverse","explicit-run","paragraph","text"] {for source in [false,true]{
   let mut middle=match name {"break-right"|"break-left"=>Content::linebreak(),"paragraph"=>Content::Parbreak,"text"=>Content::text("x"),_=>Content::Empty};if source{middle=middle.with_source_span(span);}
   let input=match name {"break-right"=>vec![Content::text("a"),Content::Space,middle,Content::text("b")],"break-left"=>vec![Content::text("a"),middle,Content::SpaceFromNewline,Content::text("b")],"empty-run-reverse"=>vec![Content::text("a"),Content::SpaceFromNewline,middle,Content::Space,Content::text("b")],"explicit-run"=>vec![Content::text("a"),Content::Space,middle,Content::Space,Content::text("b")],_=>vec![Content::text("a"),Content::Space,middle,Content::SpaceFromNewline,Content::text("b")]};
   let output=run(&input);let lazy=normalize_markup_spaces_if_changed(&input);println!("{name} source={source} {:?} lazy_changed={}",output.iter().map(view).collect::<Vec<_>>(),lazy.is_some());
  }}
 }

}
fn main(){measured::measure();}
