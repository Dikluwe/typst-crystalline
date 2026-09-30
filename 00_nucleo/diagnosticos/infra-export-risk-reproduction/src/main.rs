//! Diagnostic-only public API probes. No tests or edits in productive layers.
#![allow(deprecated)]
use std::{path::Path, sync::Arc};
use typst_core::{compiler::{eval::bibtex::parse_bibtex, layout::image::calculate_dimensions},
    contracts::package_downloader::PackageDownloader,
    entities::{content::Content, geometry::{LineCap, ShapeKind}, image_sizer::ImageSizer,
        layout_types::{FrameItem, Page, PagedDocument, Point, Pt, Rect,
            SemanticKind, SemanticPlacement, TextStyle, TransformMatrix}, paint::Paint,
        document_format_options::{PdfStandard, PdfStandards}, value::Value}};
use typst_infra::{export::*, image_sizer::ImageSizeImageSizer,
    package_downloader::HttpPackageDownloader};
use serde_json::json;

fn page(items: Vec<FrameItem>) -> Page {
    Page { width: 100., height: 100., logical_number: 1, numbering: None,
        supplement: Content::Empty, bleed: Default::default(), fill: Default::default(),
        background: vec![], foreground: vec![], items }
}
fn point(x: f64,y: f64)->Point { Point { x: Pt(x), y: Pt(y) } }
fn shape()->FrameItem {
    FrameItem::Shape { pos: point(0.,0.), kind: ShapeKind::Rect, width: 40., height: 40.,
        fill: Some(Paint::Solid(typst_core::entities::color::Color::rgba(255,0,0,255))),
        stroke: None, fill_rule: Default::default(), parent_bbox_at_emit: None }
}
fn text(s: &str,y:f64)->FrameItem {
    FrameItem::Text { pos: point(10.,y), text:s.into(), style: TextStyle::regular(Pt(10.)) }
}
fn label(s:&str,y:f64)->FrameItem {
    FrameItem::Semantic { kind: SemanticKind::Label, placement: SemanticPlacement::Inline,
        alt: None, items:vec![text(s,y)] }
}
fn save(out:&Path,name:&str,bytes:impl AsRef<[u8]>) { std::fs::write(out.join(name),bytes).unwrap(); }

fn main() {
    let args:Vec<String>=std::env::args().collect();
    if args[1]=="download" {
        let dl=HttpPackageDownloader::with_url(args[2].clone().into(), &args[3]);
        let spec="@preview/risk-probe:1.0.0".parse().unwrap();
        println!("{}", match dl.download(&spec) { Ok(p)=>json!({"ok":p}),
            Err(e)=>json!({"error":format!("{e:?}")}) });
        return;
    }
    let out=Path::new(&args[1]);
    for (name,src) in [
        ("quoted-control", r#"@book{k,title="Plain",author={Doe, Jane},year=2024}"#),
        ("escaped-quote", r#"@book{k,title="G\"{o}del",author={Doe, Jane},year=2024}"#),
        ("braced-quote", r#"@book{k,title={G\"{o}del},author={Doe, Jane},year=2024}"#),
        ("negative-volume", "@book{k,title={T},author={Doe, Jane},year=2024,volume=-1}"),
        ("positive-sign-volume", "@book{k,title={T},author={Doe, Jane},year=2024,volume=+1}"),
        ("braced-negative-volume", "@book{k,title={T},author={Doe, Jane},year=2024,volume={-1}}"),
        ("negative-year", "@book{k,title={T},author={Doe, Jane},year=-1}"),
        ("braced-negative-year", "@book{k,title={T},author={Doe, Jane},year={-1}}") ] {
        save(out,&format!("{name}.bib"),src);
        let result=match parse_bibtex(src) {Ok(v)=>json!({"entries":format!("{v:?}")}),
            Err(e)=>json!({"error":e.message})};
        println!("{}",json!({"case":name,"result":result}));
    }
    for name in ["no-density.png","zero-phys.png","positive-phys.png","zero-exif.png",
        "zero-jfif.jpg","positive-jfif.jpg","zero-exif.jpg"] {
        let bytes=std::fs::read(out.join(name)).unwrap(); let s=ImageSizeImageSizer;
        let w=Value::Float(20.); let h=Value::Float(10.);
        for (fit,uw,uh) in [("natural",None,None),("width",Some(&w),None),
            ("stretch",Some(&w),Some(&h)),("cover",Some(&w),Some(&h)),("contain",Some(&w),Some(&h))] {
            let d=calculate_dimensions(&bytes,uw,uh,fit,&s);
            println!("{}",json!({"case":name,"fit":fit,"dpi":s.dpi(&bytes),"size":s.size(&bytes),
                "width":format!("{}",d.width_pt),"height":format!("{}",d.height_pt)}));
        }
    }
    for (layer,y) in [("body",30.),("background",20.),("foreground",40.)] {
        let mut p=page(vec![]); let v=vec![label(layer,y)];
        match layer { "body"=>p.items=v,"background"=>p.background=v,_=>p.foreground=v }
        for ua in [false,true] {
            let standards=PdfStandards::new(if ua {vec![PdfStandard::Ua1]} else {vec![]}).unwrap();
            let bytes=export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
                &PagedDocument::new(vec![p.clone()]),Some([0;16]),StreamMode::Verbose,
                PdfTags::Enabled,PdfSerializationMode::Pretty,None,standards);
            save(out,&format!("tag-{layer}-{}.pdf",if ua {"ua1"}else{"ordinary"}),bytes);
        }
    }
    for (name,clip) in [("group-clip",true),("group-no-clip",false)] {
        let p=page(vec![FrameItem::Group {pos:point(10.,10.),matrix:TransformMatrix::identity(),
            clip_mask:if clip {Some(ShapeKind::Rect)}else{None}, inner_width:10.,inner_height:10.,
            items:vec![shape()]}]);
        save(out,&format!("{name}.png"),render_page_to_png(&p,&RenderOptions::default()));
        save(out,&format!("{name}.svg"),export_svg(&p,&SvgOptions::default()));
    }
    for (name,clip) in [("image-clip",true),("image-no-clip",false)] {
        let p=page(vec![FrameItem::Image {pos:point(10.,10.),data:Arc::new(std::fs::read(out.join("red.png")).unwrap()),
            alt:None,width:Pt(40.),height:Pt(40.),intrinsic_width:2,intrinsic_height:2,
            clip_rect:if clip{Some(Rect{x:Pt(10.),y:Pt(10.),w:Pt(10.),h:Pt(10.)})}else{None},orientation:1}]);
        save(out,&format!("{name}.png"),render_page_to_png(&p,&RenderOptions::default()));
        save(out,&format!("{name}.svg"),export_svg(&p,&SvgOptions::default()));
    }
    for (name,cap) in [("butt",LineCap::Butt),("round",LineCap::Round),("square",LineCap::Square)] {
        let p=page(vec![FrameItem::Line {start:point(20.,30.),end:point(40.,30.),
            thickness:10.,color:None,cap}]);
        save(out,&format!("cap-{name}.svg"),export_svg(&p,&SvgOptions::default()));
        save(out,&format!("cap-{name}.png"),render_page_to_png(&p,&RenderOptions::default()));
    }
}
