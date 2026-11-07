//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/svg_image.md
//! @prompt-hash b394bbc0
//! @layer L3
//! @updated 2026-11-01

use std::collections::HashMap;
use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use svg2pdf::usvg;
use typst_core::contracts::world::World;
use typst_core::entities::{
    font_list::FontList,
    image_sizer::ImageSizer,
    path::RootedPath,
    svg_image::{PreparedSvg, SvgOrigin, SvgSource},
};

const MAX_DATA: usize = 64 * 1024 * 1024;
const MAX_TOTAL: usize = 128 * 1024 * 1024;
const MAX_DEPTH: usize = 16;

#[derive(Clone)]
pub(crate) struct SvgImageSizer<'a> {
    world: &'a dyn World,
}
impl<'a> SvgImageSizer<'a> {
    pub(crate) fn new(world: &'a dyn World) -> Self {
        Self { world }
    }
    fn load_slot(
        &self,
        index: usize,
        db: &mut Arc<usvg::fontdb::Database>,
        loaded: &Mutex<HashMap<usize, Vec<usvg::fontdb::ID>>>,
    ) -> Vec<usvg::fontdb::ID> {
        let mut slots = loaded.lock().unwrap();
        if let Some(ids) = slots.get(&index) {
            return ids.clone();
        }
        let ids: Vec<usvg::fontdb::ID> = self
            .world
            .font(index)
            .map(|font| {
                Arc::make_mut(db)
                    .load_font_source(usvg::fontdb::Source::Binary(Arc::new(
                        font.as_slice().to_vec(),
                    )))
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default();
        slots.insert(index, ids.clone());
        ids
    }
}
fn domain_variant(plan: &FontPlan) -> typst_core::entities::font_book::FontVariant {
    use typst_core::entities::font_book::{
        FontStretch, FontStyle, FontVariant, FontWeight,
    };
    FontVariant {
        style: match plan.style {
            usvg::fontdb::Style::Normal => FontStyle::Normal,
            usvg::fontdb::Style::Italic => FontStyle::Italic,
            usvg::fontdb::Style::Oblique => FontStyle::Oblique,
        },
        weight: FontWeight(plan.weight.0),
        stretch: FontStretch::from_number(plan.stretch.to_number()),
    }
}
fn covers(db: &usvg::fontdb::Database, id: usvg::fontdb::ID, c: char) -> bool {
    db.with_face_data(id, |data, index| {
        ttf_parser::Face::parse(data, index)
            .ok()
            .and_then(|f| f.glyph_index(c))
            .is_some()
    })
    .unwrap_or(false)
}
impl ImageSizer for SvgImageSizer<'_> {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)> {
        crate::image_sizer::ImageSizeImageSizer.size(data)
    }
    fn dpi(&self, data: &[u8]) -> Option<f64> {
        crate::image_sizer::ImageSizeImageSizer.dpi(data)
    }
    fn orientation(&self, data: &[u8]) -> Option<u32> {
        crate::image_sizer::ImageSizeImageSizer.orientation(data)
    }
    fn prepare_svg(
        &self,
        data: &[u8],
        source: &SvgSource,
        families: Option<&FontList>,
        fallback: bool,
    ) -> Result<PreparedSvg, String> {
        let names: Vec<String> = families
            .map(|f| {
                f.as_slice()
                    .iter()
                    .flat_map(|f| {
                        self.world
                            .book()
                            .infos()
                            .iter()
                            .filter(move |info| f.name.is_match(&info.family))
                            .map(|info| info.family.clone())
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec!["Libertinus Serif".into()]);
        let names = Arc::new(names);
        let errors = Mutex::new(Vec::new());
        let warnings = Mutex::new(Vec::new());
        let total = AtomicUsize::new(0);
        let ctx = Preparation {
            adapter: self,
            names: &names,
            fallback,
            total: &total,
            errors: &errors,
            warnings: &warnings,
        };
        let tree = prepare_tree(&ctx, data, &source.origin, &[], 0)?;
        let errors = errors
            .into_inner()
            .map_err(|_| "SVG resource diagnostics unavailable")?;
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        let data = tree.to_string(&usvg::WriteOptions::default()).into_bytes();
        if data.len() > MAX_DATA {
            return Err("prepared SVG exceeds size limit".into());
        }
        Ok(PreparedSvg {
            data: Arc::new(data),
            width_px: f64::from(tree.size().width()),
            height_px: f64::from(tree.size().height()),
            warnings: warnings.into_inner().unwrap_or_default(),
        })
    }
}

fn bounded_xml(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() > MAX_DATA {
        return Err("SVG exceeds size limit".into());
    }
    let xml = if data.starts_with(&[0x1f, 0x8b]) {
        let mut bytes = Vec::new();
        flate2::read::MultiGzDecoder::new(data)
            .take((MAX_DATA + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("invalid SVGZ: {e}"))?;
        bytes
    } else {
        data.to_vec()
    };
    if xml.len() > MAX_DATA {
        return Err("decompressed SVG exceeds size limit".into());
    }
    Ok(xml)
}

#[derive(Clone)]
struct FontPlan {
    families: Vec<String>,
    weight: usvg::fontdb::Weight,
    style: usvg::fontdb::Style,
    stretch: usvg::fontdb::Stretch,
}
#[derive(Clone, Copy)]
struct Preparation<'a> {
    adapter: &'a SvgImageSizer<'a>,
    names: &'a Arc<Vec<String>>,
    fallback: bool,
    total: &'a AtomicUsize,
    errors: &'a Mutex<Vec<String>>,
    warnings: &'a Mutex<Vec<String>>,
}
fn prepare_tree(
    ctx: &Preparation<'_>,
    data: &[u8],
    origin: &SvgOrigin,
    stack: &[RootedPath],
    depth: usize,
) -> Result<usvg::Tree, String> {
    let Preparation { adapter, names, fallback, total, errors, warnings } = *ctx;
    if depth >= MAX_DEPTH {
        return Err("SVG resource recursion limit exceeded".into());
    }
    let xml = bounded_xml(data)?;
    if total
        .fetch_add(xml.len(), Ordering::Relaxed)
        .saturating_add(xml.len())
        > MAX_TOTAL
    {
        return Err("SVG aggregate resource size limit exceeded".into());
    }
    let text =
        std::str::from_utf8(&xml).map_err(|e| format!("invalid SVG UTF-8: {e}"))?;
    let parsed = usvg::roxmltree::Document::parse_with_options(
        text,
        usvg::roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() },
    )
    .map_err(|e| format!("invalid SVG XML: {e}"))?;
    if parsed.root_element().tag_name().name() != "svg"
        || parsed.root_element().tag_name().namespace()
            != Some("http://www.w3.org/2000/svg")
    {
        return Err("document root is not an SVG element".into());
    }
    if parsed.descendants().any(|n| n.has_tag_name("foreignObject")) {
        warnings
            .lock()
            .unwrap()
            .push("SVG foreignObject content may be unsupported".into());
    }
    let mut parents = stack.to_vec();
    if let SvgOrigin::File(path) = origin {
        if parents.last() != Some(path) {
            parents.push(path.clone());
        }
    }
    let mut opts = usvg::Options {
        dpi: 96.0,
        font_family: names.first().cloned().unwrap_or_else(|| "Libertinus Serif".into()),
        ..usvg::Options::default()
    };
    let loaded = Arc::new(Mutex::new(HashMap::new()));
    // usvg preselects all span fonts before shaping. Distinct font specs need
    // distinct IDs even when they share the same primary face, so that fallback
    // can recover the correct ordered secondary families for each span.
    let plans = Arc::new(Mutex::new(HashMap::<usvg::fontdb::ID, FontPlan>::new()));
    let selected_plans = Arc::clone(&plans);
    let selected_loaded = Arc::clone(&loaded);
    let families = Arc::clone(names);
    opts.font_resolver.select_font = Box::new(move |font, db| {
        let mut fs: Vec<usvg::fontdb::Family<'_>> = font
            .families()
            .iter()
            .map(|f| match f {
                usvg::FontFamily::Named(n) => usvg::fontdb::Family::Name(n),
                usvg::FontFamily::Serif => usvg::fontdb::Family::Serif,
                usvg::FontFamily::SansSerif => usvg::fontdb::Family::SansSerif,
                usvg::FontFamily::Monospace => usvg::fontdb::Family::Monospace,
                usvg::FontFamily::Cursive => usvg::fontdb::Family::Cursive,
                usvg::FontFamily::Fantasy => usvg::fontdb::Family::Fantasy,
            })
            .collect();
        fs.extend(families.iter().map(|n| usvg::fontdb::Family::Name(n)));
        let style = match font.style() {
            usvg::FontStyle::Normal => usvg::fontdb::Style::Normal,
            usvg::FontStyle::Italic => usvg::fontdb::Style::Italic,
            usvg::FontStyle::Oblique => usvg::fontdb::Style::Oblique,
        };
        let stretch = match font.stretch() {
            usvg::FontStretch::UltraCondensed => usvg::fontdb::Stretch::UltraCondensed,
            usvg::FontStretch::ExtraCondensed => usvg::fontdb::Stretch::ExtraCondensed,
            usvg::FontStretch::Condensed => usvg::fontdb::Stretch::Condensed,
            usvg::FontStretch::SemiCondensed => usvg::fontdb::Stretch::SemiCondensed,
            usvg::FontStretch::Normal => usvg::fontdb::Stretch::Normal,
            usvg::FontStretch::SemiExpanded => usvg::fontdb::Stretch::SemiExpanded,
            usvg::FontStretch::Expanded => usvg::fontdb::Stretch::Expanded,
            usvg::FontStretch::ExtraExpanded => usvg::fontdb::Stretch::ExtraExpanded,
            usvg::FontStretch::UltraExpanded => usvg::fontdb::Stretch::UltraExpanded,
        };
        let ordered: Vec<String> =
            fs.iter().map(|f| db.family_name(f).to_owned()).collect();
        let plan = FontPlan {
            families: ordered,
            weight: usvg::fontdb::Weight(font.weight()),
            style,
            stretch,
        };
        let variant = domain_variant(&plan);
        let mut chosen = None;
        for name in &plan.families {
            if let Some(index) = adapter.world.book().select(name, &variant) {
                adapter.load_slot(index, db, &selected_loaded);
                let names = [usvg::fontdb::Family::Name(name)];
                chosen = db.query(&usvg::fontdb::Query {
                    families: &names,
                    weight: plan.weight,
                    style: plan.style,
                    stretch: plan.stretch,
                });
                if chosen.is_some() {
                    break;
                }
            }
        }
        let id = chosen?;
        let face = db.face(id)?.clone();
        let unique = Arc::make_mut(db).push_face_info(face);
        selected_plans.lock().unwrap().insert(unique, plan);
        Some(unique)
    });
    let families = Arc::clone(names);
    opts.font_resolver.select_fallback = Box::new(move |c, excluded, db| {
        let plan = excluded
            .first()
            .and_then(|id| plans.lock().unwrap().get(id).cloned())
            .unwrap_or_else(|| FontPlan {
                families: families.as_ref().clone(),
                weight: usvg::fontdb::Weight::NORMAL,
                style: usvg::fontdb::Style::Normal,
                stretch: usvg::fontdb::Stretch::Normal,
            });
        let variant = domain_variant(&plan);
        for name in &plan.families {
            if let Some(index) = adapter.world.book().select(name, &variant) {
                if adapter.world.book().infos()[index].coverage.iter().next().is_some()
                    && !adapter.world.book().infos()[index].coverage.contains(c as u32)
                {
                    continue;
                }
                for id in adapter.load_slot(index, db, &loaded) {
                    if !excluded.contains(&id) && covers(db, id, c) {
                        return Some(id);
                    }
                }
            }
        }
        if fallback {
            let mut candidates: Vec<_> = adapter
                .world
                .book()
                .infos()
                .iter()
                .enumerate()
                .filter(|(_, info)| {
                    info.coverage.iter().next().is_none()
                        || info.coverage.contains(c as u32)
                })
                .collect();
            candidates.sort_by_key(|(index, info)| {
                (
                    info.variant.style.distance(variant.style),
                    info.variant.stretch.distance(variant.stretch),
                    info.variant.weight.distance(variant.weight),
                    *index,
                )
            });
            for (index, _) in candidates {
                for id in adapter.load_slot(index, db, &loaded) {
                    if !excluded.contains(&id) && covers(db, id, c) {
                        return Some(id);
                    }
                }
            }
        }
        None
    });
    opts.image_href_resolver.resolve_string = Box::new(|href, _| {
        let result = (|| {
            if href.contains("://") || href.starts_with("file:") {
                return Err("external SVG URL is not supported".into());
            }
            let path = match origin {
                SvgOrigin::File(path) => RootedPath::new(
                    path.root().clone(),
                    path.vpath()
                        .parent()
                        .ok_or("SVG resource has no parent")?
                        .join(href)
                        .map_err(|e| format!("invalid SVG resource path: {e:?}"))?,
                ),
                SvgOrigin::Inline(id) => adapter.world.resolve_path(*id, href)?,
            };
            if parents.contains(&path) {
                return Err("cyclic SVG resource".into());
            }
            let data = adapter.world.read_path(&path)?;
            resource_kind(ctx, &data, &SvgOrigin::File(path), &parents, depth + 1)
        })();
        match result {
            Ok(kind) => Some(kind),
            Err(e) => {
                errors.lock().unwrap().push(e);
                None
            }
        }
    });
    opts.image_href_resolver.resolve_data = Box::new(|_, data, _| {
        match resource_kind(ctx, &data, origin, &parents, depth + 1) {
            Ok(kind) => Some(kind),
            Err(e) => {
                errors.lock().unwrap().push(e);
                None
            }
        }
    });
    usvg::Tree::from_xmltree(&parsed, &opts).map_err(|e| format!("invalid SVG: {e}"))
}

fn resource_kind(
    ctx: &Preparation<'_>,
    data: &Arc<Vec<u8>>,
    origin: &SvgOrigin,
    parents: &[RootedPath],
    depth: usize,
) -> Result<usvg::ImageKind, String> {
    let total = ctx.total;
    use typst_core::entities::image_format::{detect_image_format, ImageFormat};
    if data.len() > MAX_DATA
        || total
            .fetch_add(data.len(), Ordering::Relaxed)
            .saturating_add(data.len())
            > MAX_TOTAL
    {
        return Err("SVG resource size limit exceeded".into());
    }
    Ok(match detect_image_format(data) {
        ImageFormat::Jpeg => usvg::ImageKind::JPEG(Arc::clone(data)),
        ImageFormat::Png => usvg::ImageKind::PNG(Arc::clone(data)),
        ImageFormat::Gif => usvg::ImageKind::GIF(Arc::clone(data)),
        ImageFormat::WebP => usvg::ImageKind::WEBP(Arc::clone(data)),
        ImageFormat::Svg => {
            usvg::ImageKind::SVG(prepare_tree(ctx, data, origin, parents, depth)?)
        }
        ImageFormat::Unknown => return Err("unsupported SVG image resource".into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use typst_core::entities::{
        file_id::FileId,
        font_book::FontBook,
        path::{VirtualPath, VirtualRoot},
        source::Source,
        span::Span,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct MemoryWorld {
        library: Library,
        book: FontBook,
        fonts: Vec<crate::fonts::FontSlot>,
        source: Source,
        files: HashMap<String, Arc<Vec<u8>>>,
        reads: AtomicUsize,
    }
    impl MemoryWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
            let sets = crate::embedded_fonts::load_embedded_fonts();
            let mut book = FontBook::new();
            for info in sets.text_book.infos().iter().chain(sets.math_code_book.infos()) {
                book.push(info.clone());
            }
            let mut fonts = sets.text_slots;
            fonts.extend(sets.math_code_slots);
            Self {
                library: Library::new(),
                book,
                fonts,
                source: Source::new(id, text.to_owned()).unwrap(),
                files: HashMap::new(),
                reads: AtomicUsize::new(0),
            }
        }
    }
    impl World for MemoryWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            panic!("byte SVG preparation must not read source files")
        }
        fn resolve_path(&self, _: FileId, path: &str) -> Result<RootedPath, String> {
            Ok(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new(path).map_err(|e| format!("{e:?}"))?,
            ))
        }
        fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
            self.reads.fetch_add(1, Ordering::Relaxed);
            self.files
                .get(path.vpath().get_with_slash())
                .cloned()
                .ok_or_else(|| "missing resource".into())
        }
        fn font(&self, index: usize) -> Option<Font> {
            self.fonts.get(index)?.get()
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    const RECT:&[u8]=b"<svg xmlns='http://www.w3.org/2000/svg' width='10.5' height='5.25'><rect width='10.5' height='5.25' fill='red'/></svg>";
    fn inline(world: &MemoryWorld) -> SvgSource {
        SvgSource {
            origin: SvgOrigin::Inline(world.main()),
            span: Span::detached(),
        }
    }
    #[test]
    fn svg_input_gzip_and_fractional_geometry_without_payload_io() {
        use std::io::Write;
        let world = MemoryWorld::new("");
        let adapter = SvgImageSizer::new(&world);
        let mut gzip =
            flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        gzip.write_all(RECT).unwrap();
        let compressed = gzip.finish().unwrap();
        for bytes in [RECT, compressed.as_slice()] {
            let out = adapter.prepare_svg(bytes, &inline(&world), None, true).unwrap();
            assert_eq!((out.width_px, out.height_px), (10.5, 5.25));
            let tree =
                usvg::Tree::from_data(&out.data, &usvg::Options::default()).unwrap();
            assert!(!tree.root().children().is_empty());
        }
        assert_eq!(world.reads.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn svg_input_invalid_root_utf8_gzip_size_and_zero_are_errors() {
        let world = MemoryWorld::new("");
        let adapter = SvgImageSizer::new(&world);
        for bytes in [
            b"<html/>".as_slice(),
            b"<svg xmlns='http://www.w3.org/2000/svg'>",
            b"\xff",
            b"\x1f\x8bBAD",
            b"<svg xmlns='http://www.w3.org/2000/svg' width='0' height='5'/>",
        ] {
            assert!(
                adapter.prepare_svg(bytes, &inline(&world), None, true).is_err(),
                "{bytes:?}"
            );
        }
        assert!(bounded_xml(&vec![0; MAX_DATA + 1]).is_err());
        let blank = adapter
            .prepare_svg(
                b"<svg xmlns='http://www.w3.org/2000/svg' width='5' height='5'/>",
                &inline(&world),
                None,
                true,
            )
            .unwrap();
        assert_eq!((blank.width_px, blank.height_px), (5.0, 5.0));
    }
    fn png(color: [u8; 4]) -> Arc<Vec<u8>> {
        let mut bytes = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(1, 1, image::Rgba(color))
            .write_to(&mut bytes, image::ImageOutputFormat::Png)
            .unwrap();
        Arc::new(bytes.into_inner())
    }
    #[test]
    fn svg_input_relative_resources_follow_origin_and_cycles_fail() {
        let mut world = MemoryWorld::new("");
        world.files.insert("/assets/tile.png".into(), png([255, 0, 0, 255]));
        world.files.insert("/tile.png".into(), png([0, 0, 255, 255]));
        let adapter = SvgImageSizer::new(&world);
        let svg=b"<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='5' height='5'><image xlink:href='tile.png' width='5' height='5'/></svg>";
        let file = SvgSource {
            origin: SvgOrigin::File(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("/assets/image.svg").unwrap(),
            )),
            span: Span::detached(),
        };
        let a = adapter.prepare_svg(svg, &file, None, true).unwrap();
        let b = adapter.prepare_svg(svg, &inline(&world), None, true).unwrap();
        assert_ne!(a.data, b.data, "different origins embed different pixels");
        for prepared in [&a, &b] {
            assert!(String::from_utf8_lossy(&prepared.data)
                .contains("data:image/png;base64,"));
            assert!(!String::from_utf8_lossy(&prepared.data).contains("tile.png"));
        }
        let cyclic=b"<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='5' height='5'><image xlink:href='image.svg' width='5' height='5'/></svg>";
        assert!(adapter
            .prepare_svg(cyclic, &file, None, true)
            .unwrap_err()
            .contains("cyclic"));
        let escape=b"<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='5' height='5'><image xlink:href='../../tile.png' width='5' height='5'/></svg>";
        assert!(adapter.prepare_svg(escape, &file, None, true).is_err());
    }
    #[test]
    fn svg_input_reprepares_fonts_for_effective_context() {
        let world = MemoryWorld::new("");
        let adapter = SvgImageSizer::new(&world);
        let bytes=b"<svg xmlns='http://www.w3.org/2000/svg' width='100' height='30'><text x='0' y='20'>Hello</text></svg>";
        let serif = FontList::single("Libertinus Serif".into());
        let mono = FontList::single("DejaVu Sans Mono".into());
        let first = adapter
            .prepare_svg(bytes, &inline(&world), Some(&serif), true)
            .unwrap();
        let second = adapter
            .prepare_svg(bytes, &inline(&world), Some(&mono), true)
            .unwrap();
        let paint_width = |data: &[u8]| {
            usvg::Tree::from_data(data, &usvg::Options::default())
                .unwrap()
                .root()
                .abs_bounding_box()
                .width()
        };
        assert!(
            (paint_width(&first.data) - paint_width(&second.data)).abs() > 1.0,
            "font changes must change glyph geometry"
        );
        for data in [&first.data, &second.data] {
            assert!(!String::from_utf8_lossy(data).contains("<text"));
            assert!(String::from_utf8_lossy(data).contains("<path"));
        }
    }
    #[test]
    fn svg_input_declared_svg_families_precede_context_fallback() {
        let world = MemoryWorld::new("");
        let adapter = SvgImageSizer::new(&world);
        let context = FontList::single("NewComputerModern10".into());
        let source = inline(&world);
        let mono=adapter.prepare_svg("<svg xmlns='http://www.w3.org/2000/svg' width='100' height='30'><text x='0' y='20' font-family='DejaVu Sans Mono'>Ա</text></svg>".as_bytes(),&source,Some(&context),true).unwrap();
        let declared=adapter.prepare_svg("<svg xmlns='http://www.w3.org/2000/svg' width='100' height='30'><text x='0' y='20' font-family='Libertinus Serif, DejaVu Sans Mono'>Ա</text></svg>".as_bytes(),&source,Some(&context),true).unwrap();
        let width = |data: &[u8]| {
            usvg::Tree::from_data(data, &usvg::Options::default())
                .unwrap()
                .root()
                .abs_bounding_box()
                .width()
        };
        assert!(width(&mono.data) > 0.0);
        assert!(
            (width(&mono.data) - width(&declared.data)).abs() < 0.001,
            "SVG declared secondary family must supply the glyph before ambient context"
        );
    }
    #[test]
    fn svg_input_does_not_acquire_unrelated_font_slots() {
        struct DemandWorld<'a>(&'a MemoryWorld);
        impl World for DemandWorld<'_> {
            fn library(&self) -> &Library {
                self.0.library()
            }
            fn book(&self) -> &FontBook {
                self.0.book()
            }
            fn main(&self) -> FileId {
                self.0.main()
            }
            fn source(&self, id: FileId) -> FileResult<Source> {
                self.0.source(id)
            }
            fn file(&self, id: FileId) -> FileResult<Bytes> {
                self.0.file(id)
            }
            fn font(&self, index: usize) -> Option<Font> {
                let info = &self.book().infos()[index];
                assert!(
                    info.family == "Libertinus Serif"
                        && info.variant
                            == typst_core::entities::font_book::FontVariant::default(),
                    "unrelated font IO: {info:?}"
                );
                self.0.font(index)
            }
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
                None
            }
        }
        let base = MemoryWorld::new("");
        let world = DemandWorld(&base);
        let adapter = SvgImageSizer::new(&world);
        let svg=b"<svg xmlns='http://www.w3.org/2000/svg' width='100' height='30'><text x='0' y='20'>Hello</text></svg>";
        let out = adapter
            .prepare_svg(svg, &inline(&base), None, true)
            .expect("only the requested regular serif face is available");
        assert!(String::from_utf8_lossy(&out.data).contains("<path"));
    }
    #[test]
    fn svg_input_external_doctype_keeps_paint_without_resource_io() {
        let world = MemoryWorld::new("");
        let adapter = SvgImageSizer::new(&world);
        let svg=b"<?xml version='1.0'?><!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'><svg xmlns='http://www.w3.org/2000/svg' width='10.5' height='5.25'><rect width='10.5' height='5.25' fill='red'/></svg>";
        let prepared = adapter
            .prepare_svg(svg, &inline(&world), None, true)
            .expect("SVG 1.1 DOCTYPE is a valid prolog");
        assert_eq!((prepared.width_px, prepared.height_px), (10.5, 5.25));
        assert!(String::from_utf8_lossy(&prepared.data).contains("<path"));
        assert_eq!(
            world.reads.load(Ordering::Relaxed),
            0,
            "DOCTYPE is not a resource request"
        );
    }
}
