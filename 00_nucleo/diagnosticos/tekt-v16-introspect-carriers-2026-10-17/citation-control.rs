
    use typst_core::entities::{
        content::{
            Content, LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
        },
        file_id::FileId,
        locator::Locator,
        span::Span,
        style_chain::StyleChain,
    };
    use std::sync::Arc;
    fn wrap(kind: usize, body: Content, locator: &mut Locator) -> Content {
        match kind {
            0 => Content::Source(Arc::new(SourceContent {
                body,
                span: Span::from_range(
                    FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
                    0..1,
                ),
            })),
            1 => Content::Located(Arc::new(LocatedContent {
                body,
                location: locator.next(),
            })),
            2 => Content::Realized(Arc::new(RealizedContent {
                original: Content::text("causal original"),
                visual: body,
            })),
            3 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::default_chain(),
                guards: vec![],
                file: FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
            })),
            _ => panic!("probe kind"),
        }
    }

    use typst_core::compiler::introspect::introspect_with_introspector;
    use typst_core::entities::{bib_entry::BibEntry, introspector::Introspector};
    fn bib() -> Content {
        Content::bibliography(
            vec![
                BibEntry::new("k1", "A1", "T1", 2024),
                BibEntry::new("k2", "A2", "T2", 2023),
            ],
            None,
        )
    }
    fn refs(b: Content) -> Content {
        Content::Sequence(
            vec![Content::reference("k2"), Content::reference("k1"), b].into(),
        )
    }


mod baseline { use typst_core::entities::content::Content;
pub fn convert_bib_refs_to_cites(content: Content) -> Content {
    let keys = collect_bib_keys(&content);
    if keys.is_empty() {
        return content;
    }
    convert_refs(content, &keys)
}

pub(super) fn collect_bib_keys(content: &Content) -> std::collections::HashSet<String> {
    use typst_core::entities::content::Content;
    let mut keys = std::collections::HashSet::new();
    fn walk(c: &Content, keys: &mut std::collections::HashSet<String>) {
        match c {
            Content::Bibliography(b) => {
                for e in &b.entries {
                    keys.insert(e.key.clone());
                }
            }
            Content::Sequence(seq) => seq.iter().for_each(|c| walk(c, keys)),
            Content::Styled(styled) => walk(&styled.body, keys),
            Content::Block(e) => walk(&e.body, keys),
            Content::Boxed(e) => walk(&e.body, keys),
            Content::Pad(e) => walk(&e.body, keys),
            Content::Align(e) => walk(&e.body, keys),
            Content::Hide(e) => walk(&e.body, keys),
            Content::Figure(e) => {
                walk(&e.body, keys);
                if let Some(cap) = &e.caption {
                    walk(cap, keys);
                }
            }
            Content::Table(e) => {
                if let Some(cap) = &e.caption {
                    walk(cap, keys);
                }
                for child in e.resolved.as_ref().map_or(&e.children, |r| &r.cells) {
                    walk(child, keys);
                }
            }
            Content::Grid(e) => e.cells.iter().for_each(|c| walk(c, keys)),
            Content::Stack(e) => e.children.iter().for_each(|c| walk(c, keys)),
            Content::ListItem(e) => walk(&e.body, keys),
            Content::EnumItem(e) => walk(&e.body, keys),
            Content::TermItem(e) => {
                walk(&e.term, keys);
                walk(&e.description, keys);
            }
            Content::Footnote(e) => walk(&e.body, keys),
            Content::Quote(e) => walk(&e.body, keys),
            Content::Overline(e) => walk(&e.body, keys),
            Content::Underline(e) => walk(&e.body, keys),
            Content::Strike(e) => walk(&e.body, keys),
            Content::Strong(e) => walk(&e.body, keys),
            Content::Emph(e) => walk(&e.body, keys),
            Content::Title(e) => walk(&e.body, keys),
            Content::SmallCaps { body, .. } => walk(body, keys),
            Content::Link(e) => walk(&e.body, keys),
            // ContextBlock e Dynamic não contêm content estático navegável.
            Content::ContextBlock(_) | Content::Dynamic(_) => {}
            _ => {} // neutro: N16[β] — variantes sem content estático navegável (terminais) não contribuem keys
        }
    }
    walk(content, &mut keys);
    keys
}

pub(super) fn convert_refs(
    content: Content,
    keys: &std::collections::HashSet<String>,
) -> Content {
    use typst_core::entities::content::Content;
    use std::sync::Arc;
    match content {
        Content::Ref(r) => {
            let key = r.name.to_string();
            if keys.contains(&key) {
                Content::cite(key, r.supplement.clone(), None)
            } else {
                Content::Ref(r)
            }
        }
        Content::Sequence(seq) => Content::Sequence(
            seq.iter()
                .map(|c| convert_refs(c.clone(), keys))
                .collect::<Vec<_>>()
                .into(),
        ),
        Content::Styled(styled) => {
            let styled = Arc::unwrap_or_clone(styled);
            Content::styled(convert_refs(styled.body, keys), styled.styles)
        }
        Content::Block(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Block(Arc::new(e))
        }
        Content::Boxed(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Boxed(Arc::new(e))
        }
        Content::Pad(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Pad(Arc::new(e))
        }
        Content::Align(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Align(Arc::new(e))
        }
        Content::Hide(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Hide(Arc::new(e))
        }
        Content::Figure(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            e.caption = e.caption.map(|c| convert_refs(c, keys));
            Content::Figure(Arc::new(e))
        }
        Content::TableCell(e) if e.resolved.is_some() => {
            let mut c = Arc::unwrap_or_clone(e);
            let mut r = Arc::unwrap_or_clone(c.resolved.take().unwrap());
            r.visual = convert_refs(r.visual, keys);
            c.resolved = Some(Arc::new(r));
            Content::TableCell(Arc::new(c))
        }
        Content::Table(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.caption = e.caption.map(|c| convert_refs(c, keys));
            if let Some(r) = e.resolved.take() {
                let mut r = Arc::unwrap_or_clone(r);
                r.cells = r.cells.into_iter().map(|c| convert_refs(c, keys)).collect();
                e.resolved = Some(Arc::new(r));
            } else {
                e.children =
                    e.children.into_iter().map(|c| convert_refs(c, keys)).collect();
            }
            Content::Table(Arc::new(e))
        }
        Content::Grid(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.cells = e.cells.into_iter().map(|c| convert_refs(c, keys)).collect();
            Content::Grid(Arc::new(e))
        }
        Content::Stack(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            let new_children: Vec<Content> =
                e.children.iter().map(|c| convert_refs(c.clone(), keys)).collect();
            e.children = Arc::from(new_children);
            Content::Stack(Arc::new(e))
        }
        Content::ListItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::ListItem(Arc::new(e))
        }
        Content::EnumItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::EnumItem(Arc::new(e))
        }
        Content::TermItem(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.term = convert_refs(e.term, keys);
            e.description = convert_refs(e.description, keys);
            Content::TermItem(Arc::new(e))
        }
        Content::Footnote(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Footnote(Arc::new(e))
        }
        Content::Quote(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Quote(Arc::new(e))
        }
        Content::Overline(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Overline(Arc::new(e))
        }
        Content::Underline(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Underline(Arc::new(e))
        }
        Content::Strike(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Strike(Arc::new(e))
        }
        Content::Strong(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Strong(Arc::new(e))
        }
        Content::Emph(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Emph(Arc::new(e))
        }
        Content::Title(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Title(Arc::new(e))
        }
        Content::SmallCaps { body, .. } => Content::smallcaps(convert_refs(*body, keys)),
        Content::Link(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.body = convert_refs(e.body, keys);
            Content::Link(Arc::new(e))
        }
        // ContextBlock não contém content aninhado (P506).
        Content::ContextBlock(e) => Content::ContextBlock(e),
        // Elementos dinâmicos: a transformação object-safe delega ao trait.
        // A transformação não pode falhar (não faz eval), por isso unwrap é
        // seguro — qualquer erro seria um bug interno.
        Content::Dynamic(e) => {
            let mut f = |c: &Content| -> typst_core::entities::source_result::SourceResult<
                Option<Content>,
            > { Ok(Some(convert_refs(c.clone(), keys))) };
            e.dyn_map_content(&mut f)
                .expect("convert_bib_refs_to_cites: dyn_map_content não deve falhar")
        }
        other => other,
    }
}

}

fn keys(c:&Content,out:&mut Vec<String>){match c {Content::Cite(e)=>out.push(e.key.clone()),Content::Sequence(s)=>for c in s.iter(){keys(c,out)},Content::Source(e)=>keys(&e.body,out),Content::Located(e)=>keys(&e.body,out),Content::ShowRecipe(e)=>keys(&e.body,out),Content::Realized(e)=>keys(&e.visual,out),Content::Bibliography(_)|Content::Ref(_)|Content::Empty|Content::Text(_)=>{},_=>panic!("unsupported fixture node")}}
fn main(){
 let before=std::env::args().nth(1).unwrap()=="before";
 let convert=|c| if before{baseline::convert_bib_refs_to_cites(c)}else{typst_core::compiler::introspect::convert_bib_refs_to_cites(c)};
 let expected=vec!["k2".to_string(),"k1".to_string()];let mut bare=Vec::new();keys(&convert(refs(bib())),&mut bare);assert_eq!(bare,expected);let mut differences=0;
 for kind in 0..4 {for position in 0..3 {
  let mut locator=Locator::new();for _ in 0..100{locator.next();}
  let input=match position{0=>wrap(kind,refs(bib()),&mut locator),1=>refs(wrap(kind,bib(),&mut locator)),2=>Content::Sequence(vec![wrap(kind,Content::reference("k2"),&mut locator),wrap(kind,Content::reference("k1"),&mut locator),bib()].into()),_=>unreachable!()};let output=convert(input);let mut got=Vec::new();keys(&output,&mut got);if !before{assert_eq!(introspect_with_introspector(&output).citation_order(),got.as_slice());}if got!=expected{differences+=1;}println!("{kind}/{position}: {got:?}");
 }}
 let hidden=Content::Realized(Arc::new(RealizedContent{original:refs(bib()),visual:Content::Empty}));let mut hidden_keys=Vec::new();keys(&convert(hidden),&mut hidden_keys);assert!(hidden_keys.is_empty());println!("differences={differences}");
}
