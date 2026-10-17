use typst_core::{compiler::introspect::{introspect_with_introspector,convert_bib_refs_to_cites},entities::{content::Content,bib_entry::BibEntry,introspector::Introspector,layout_types::TransformMatrix}};
fn wrap(kind: usize, body: Content) -> Content {
        match kind {
            0 => Content::Par { body: Box::new(body) },
            1 => Content::heading(1, body),
            2 => Content::repeat(body, None, false),
            3 => Content::columns(body, 2, None),
            4 => Content::table_cell(body, None, None, None, None),
            5 => Content::table_header(body, true),
            6 => Content::table_footer(body, true),
            7 => Content::grid_header(body, true),
            8 => Content::grid_footer(body, true),
            9 => Content::label("i2-label", body),
            10 => Content::equation(body, false),
            11 => Content::terms(vec![body]),
            12 => Content::quote(Content::Empty, Some(body), false, true),
            13 => Content::figure_caption(Content::Empty, None, Some(body)),
            14 => Content::footnote_entry(Content::Empty, Some(body), None, None, None),
            15 => Content::bibliography(vec![], Some(body)),
            16 => Content::cite("k2", Some(body), None),
            17 => Content::transform(TransformMatrix::identity(), body),
            18 => Content::MathSequence(vec![body].into()), 19 => Content::smallcaps(body), _ => unreachable!("fixture kind"),
        }
    }
fn bib()->Content{Content::bibliography(vec![BibEntry::new("k1","A1","T1",2024),BibEntry::new("k2","A2","T2",2025)],None)}
fn refs(b:Content)->Content{Content::Sequence(vec![Content::reference("k2"),Content::reference("k1"),b].into())}
fn main(){let mut differences=0;for kind in 0..18{for position in 0..3{let input=match position{0=>wrap(kind,refs(bib())),1=>refs(wrap(kind,bib())),2=>Content::Sequence(vec![wrap(kind,Content::reference("k2")),wrap(kind,Content::reference("k1")),bib()].into()),_=>unreachable!()};let intr=introspect_with_introspector(&input);let order=intr.citation_order();let ok=order==["k2","k1"];println!("{kind}/{position}: {:?}; ok={ok}",order);if !ok{differences+=1;}}}for kind in 18..20{for position in 0..3{let input=match position{0=>wrap(kind,refs(bib())),1=>refs(wrap(kind,bib())),2=>Content::Sequence(vec![wrap(kind,Content::reference("k2")),wrap(kind,Content::reference("k1")),bib()].into()),_=>unreachable!()};let output=convert_bib_refs_to_cites(input);let mut keys=Vec::new();output.map_content(&mut |c|{if let Content::Cite(e)=c{keys.push(e.key.clone());}Ok(None)}).unwrap();let ok=keys==["k2","k1"];println!("static/{kind}/{position}: {:?}; ok={ok}",keys);if !ok{differences+=1;}}}println!("differences={differences}");}
