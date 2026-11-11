//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/resources.md
//! @prompt-hash e399205f
//! @layer L3
//! @updated 2026-11-09
//!
//! Possui coleta ordenada e resolução das fontes do documento, fallback por família, variações explícitas e instanciação de fontes variáveis. Recursa pelos containers e pelas camadas marginais existentes, preservando a chave completa e os diagnósticos vigentes.

use super::*;

/// Match the dependency guard to the instancing owner's filtered axis requests.
pub(super) fn requires_font_instancing(
    bytes: &[u8],
    variant: &FontVariant,
    variations: &FontVariations,
) -> bool {
    let Ok(face) = ttf_parser::Face::parse(bytes, 0) else { return false };
    let merged =
        merge_explicit_variations(axis_variations_for_font_variant(variant), variations);
    face.variation_axes()
        .into_iter()
        .any(|axis| merged.iter().any(|request| request.tag == axis.tag))
}

/// Shared early dependency gate for every resolved-font export path.
/// The availability probe is lazy: unknown-axis and static-only requests never
/// consult Python. The first matching resolved font supplies the diagnostic.
pub(super) fn missing_font_instancer_diagnostic(
    resolved: &[FontKey],
    is_available: impl FnOnce() -> bool,
) -> Option<SourceDiagnostic> {
    let ((font_list, _, _), _) =
        resolved.iter().find(|((_, variant, variations), bytes)| {
            requires_font_instancing(bytes, variant, variations)
        })?;
    if is_available() {
        return None;
    }
    let name = font_list
        .as_slice()
        .first()
        .and_then(|font| font.name.as_str())
        .unwrap_or("fonte variável");
    Some(SourceDiagnostic::error(
        Span::detached(),
        format!(
            "fonte variável '{}' requer instanciação, mas Python/fontTools não está disponível",
            name
        ),
    ))
}

/// Resolve os bindings originais e valida a dependência de instanciação.
/// PNG/SVG preparam bytes da instância no exportador, mantendo a identidade
/// original distinta dos bytes preparados na mesma fase de exportação.
pub(super) fn resolve_export_fonts(
    doc: &PagedDocument,
    world: &dyn World,
) -> Result<Vec<FontKey>, SourceDiagnostic> {
    let combos = collect_fonts_from_doc(doc, world);
    let resolved = bind_owned_export_fonts(doc, resolve_fonts(&combos, world));

    if let Some(diag) =
        missing_font_instancer_diagnostic(&resolved, variable_font_instancer_available)
    {
        return Err(diag);
    }

    Ok(resolved)
}

/// Bind an owned export resource table without changing document binding policy.
fn bind_owned_export_fonts(doc: &PagedDocument, supplied: Vec<FontKey>) -> Vec<FontKey> {
    match crate::export::font_binding::bind_document(doc, &supplied) {
        std::borrow::Cow::Borrowed(_) => supplied,
        std::borrow::Cow::Owned(bound) => bound,
    }
}

/// Itera `doc.pages → items` recursivamente (atravessa `Group`)
/// e devolve **todas** as combinações `(FontList, FontVariant)` distintas
/// em ordem de primeira ocorrência (Passo 146, ADR-0055 decisão 5;
/// P530 — chave expandida para incluir weight/style).
///
/// Deduplicação por igualdade estrutural via `Vec::contains`.
/// Complexidade O(N²) em N = combinações distintas; aceite porque N é
/// tipicamente pequeno (<10) em documentos reais.
fn record_text_font(
    seen: &mut Vec<(FontList, FontVariant, FontVariations)>,
    list: &FontList,
    variant: FontVariant,
    axes: Option<&FontVariations>,
    build: impl FnOnce() -> (FontList, FontVariant, FontVariations),
) {
    let present = seen.iter().any(|(known_list, known_variant, known_axes)| {
        known_list == list
            && *known_variant == variant
            && match axes {
                Some(axes) => known_axes == axes,
                None => known_axes.0.is_empty(),
            }
    });
    if !present {
        seen.push(build());
    }
}

/// **P906** — `world` adicionado: `FrameItem::Glyph` precisa de resolver a
/// fonte que efectivamente o cobre (`FallbackFontMetrics::resolve_font_
/// combo`, via `base_char`), à semelhança do que `style.font` já faz
/// directamente para `Text`/`TextShaped`. Ver nota em `collect_fonts_in_
/// items` abaixo.
pub(super) fn collect_fonts_from_doc(
    doc: &PagedDocument,
    world: &dyn World,
) -> Vec<(FontList, FontVariant, FontVariations)> {
    let metrics = FallbackFontMetrics::new(world);
    let mut seen: Vec<(FontList, FontVariant, FontVariations)> = Vec::new();
    for page in &doc.pages {
        collect_fonts_in_items(&page.background, &metrics, &mut seen);
        collect_fonts_in_items(&page.items, &metrics, &mut seen);
        collect_fonts_in_items(&page.foreground, &metrics, &mut seen);
    }
    seen
}

/// **P906** — braço `FrameItem::Glyph` adicionado. Antes deste passo, era
/// ignorado (`FrameItem::Glyph { .. } => {}`) — todo glifo de esticamento
/// matemático (`layout_stretchy_delimiter`/`layout_assembly`, eixo
/// vertical e horizontal) era invisível para a selecção Cidfont-vs-
/// Multifont, que decide cedo no pipeline QUAIS fontes embutir a partir
/// só dos caracteres vistos em `Text`/`TextShaped`. Achado: uma equação
/// cujo ÚNICO conteúdo a precisar da fonte companion MATH fosse um glifo
/// de esticamento (ex: `underbracket(a+b+c)` sem mais nenhum texto
/// itálico matemático à volta) escolhia só a fonte de corpo como
/// candidata Cidfont única — o glyph_id do esticamento, correcto na fonte
/// MATH, caía fora do subset embutido (glifo `.notdef` no PDF real).
/// Corrigido resolvendo `base_char`+`style` via `resolve_font_combo`
/// (mesmo mecanismo `covering`/`resolve_primary_with_math_fallback` já
/// usado internamente por `FallbackFontMetrics` para o LAYOUT) — Glyph
/// entra na mesma lista `seen`, com a mesma deduplicação, como se fosse
/// mais um span de texto. Ver `pipeline.md` §P906.
fn collect_fonts_in_items(
    items: &[FrameItem],
    metrics: &FallbackFontMetrics,
    seen: &mut Vec<(FontList, FontVariant, FontVariations)>,
) {
    for item in items {
        match item {
            FrameItem::TextSeparator { style, .. }
            | FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    let variant = text_style_to_font_variant(style);
                    // P836 — variações explícitas (`#text(variations:)`)
                    // entram na chave: runs com o mesmo FontVariant mas
                    // eixos explícitos distintos embutem fontes distintas.
                    record_text_font(
                        seen,
                        fl,
                        variant,
                        style.variations.as_ref(),
                        || {
                            (
                                fl.clone(),
                                variant,
                                style.variations.clone().unwrap_or_default(),
                            )
                        },
                    );
                }
            }
            FrameItem::Glyph { style, base_char, .. } => {
                if let Some(key) = metrics.resolve_font_combo(*base_char, style) {
                    if !seen.contains(&key) {
                        seen.push(key);
                    }
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => {
                collect_fonts_in_items(items, metrics, seen);
            }
            FrameItem::Shape { fill, stroke, .. } => {
                for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                    if let typst_core::entities::frame_paint::FramePaint::Tiling(cell) =
                        paint
                    {
                        collect_fonts_in_items(&cell.frame.items, metrics, seen);
                    }
                }
            }
            FrameItem::Line { .. } | FrameItem::Image { .. } => {}
        }
    }
}

/// Map-filter de `resolve_font` (Passo 141) sobre uma lista de
/// combinações `(FontList, FontVariant)`. Devolve
/// `((FontList, FontVariant), bytes)` para preservar a associação entre
/// input style e output embed (Passo 146; P530 — chave expandida).
///
/// Silent drop quando `resolve_font` devolve `None` — consistente
/// com a política de fallback de fonts (140B/141).
pub(super) fn resolve_fonts(
    font_combos: &[(FontList, FontVariant, FontVariations)],
    world: &dyn World,
) -> Vec<((FontList, FontVariant, FontVariations), Vec<u8>)> {
    if font_combos.is_empty() {
        return Vec::new();
    }
    let font_book = world.book();
    font_combos
        .iter()
        .filter_map(|(fl, variant, variations)| {
            resolve_font(fl, variant, font_book, world)
                .map(|bytes| ((fl.clone(), *variant, variations.clone()), bytes))
        })
        .collect()
}

/// Itera `doc.pages → items` recursivamente (atravessa `Group`)
/// e devolve o primeiro `TextStyle.font` com `Some(FontList)`.
/// Preservada do Passo 140B (MVP single-font: primeira vence —
/// ADR-0055 decisão 3) para tests directos do helper. O dispatch
/// principal usa `collect_fonts_from_doc` (Passo 146).
#[allow(dead_code)]
pub(super) fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList> {
    for page in &doc.pages {
        if let Some(fl) = first_font_in_items(&page.items) {
            return Some(fl);
        }
    }
    None
}

#[allow(dead_code)]
fn first_font_in_items(items: &[FrameItem]) -> Option<FontList> {
    for item in items {
        match item {
            FrameItem::TextSeparator { style, .. }
            | FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. } => {
                if let Some(fl) = &style.font {
                    return Some(fl.clone());
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => {
                if let Some(fl) = first_font_in_items(items) {
                    return Some(fl);
                }
            }
            FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
    None
}

/// Itera `font_list.as_slice()` em ordem. Para cada família,
/// consulta `font_book.select_pattern(&family.name, variant)`;
/// se devolve `Some(index)`, chama `world.font(index)`; primeira
/// família que completa ambos os passos vence. Se nenhuma
/// completa, devolve `None` (pipeline cai em fallback Helvetica).
///
/// Paridade com vanilla: semântica "primeira-que-resolve" do
/// `#set text(font: (...))` (Passo 141).
///
/// Cenário patológico (índice stale: `select` devolve `Some` mas
/// `world.font` devolve `None`) **continua** a tentar as famílias
/// seguintes — não curto-circuita.
///
/// P530 — usa a `FontVariant` real do `TextStyle` na selecção. Se o
/// sistema tiver instâncias estáticas (ex.: UbuntuSans-Bold.ttf), são
/// preferidas. Caso contrário, a fonte VF é resolvida e instanciada
/// estaticamente mais tarde no export.
pub(super) fn resolve_font(
    font_list: &FontList,
    variant: &FontVariant,
    font_book: &FontBook,
    world: &dyn World,
) -> Option<Vec<u8>> {
    for family in font_list.as_slice() {
        if let Some(index) = font_book.select_pattern(&family.name, variant) {
            if let Some(font) = world.font(index) {
                return Some(font.as_slice().to_vec());
            }
        }
    }
    None
}

#[cfg(test)]
mod instancer_gate_tests {
    use super::*;

    #[test]
    fn matching_axis_has_one_shared_diagnostic_and_unknown_axis_skips_probe() {
        let bytes =
            include_bytes!("../../fixtures/fonts/UbuntuSans-Variable.ttf").to_vec();
        let font_list = FontList::single("ubuntu sans".into());
        let key = |axis| {
            (
                (
                    font_list.clone(),
                    FontVariant::default(),
                    FontVariations(vec![(axis, 700.0)]),
                ),
                bytes.clone(),
            )
        };
        let unknown = key(*b"ZZZZ");
        assert!(missing_font_instancer_diagnostic(&[unknown], || {
            panic!("unknown axis must not probe Python")
        })
        .is_none());
        let matching = key(*b"wght");
        let diag = missing_font_instancer_diagnostic(&[matching], || false)
            .expect("matching axis requires the missing-tool diagnostic");
        assert_eq!(diag.message, "fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível");
    }
}

#[cfg(test)]
mod textual_key_tests {
    use super::*;
    #[test]
    fn duplicates_construct_one_owned_key_only() {
        let list = FontList::single("Libertinus Serif".into());
        let axes = FontVariations(vec![(*b"wght", 400.0)]);
        let variant = FontVariant::default();
        let mut seen = Vec::new();
        let mut builds = 0;
        for _ in 0..1000 {
            record_text_font(&mut seen, &list, variant, Some(&axes), || {
                builds += 1;
                (list.clone(), variant, axes.clone())
            });
        }
        assert_eq!(seen.len(), 1);
        assert_eq!(builds, 1);
    }
    #[test]
    fn first_occurrence_and_full_key_equivalences_are_preserved() {
        let a = FontList::single("First".into());
        let b = FontList::single("Second".into());
        let base = FontVariant::default();
        let bold = FontVariant {
            weight: typst_core::entities::font_book::FontWeight::BOLD,
            ..base
        };
        let empty = FontVariations::default();
        let zero = FontVariations(vec![(*b"wght", 0.0)]);
        let neg = FontVariations(vec![(*b"wght", -0.0)]);
        let nan = FontVariations(vec![(*b"wght", f32::NAN)]);
        let requests = [
            (&a, base, None),
            (&b, base, None),
            (&a, base, Some(&empty)),
            (&a, bold, None),
            (&a, base, Some(&zero)),
            (&a, base, Some(&neg)),
            (&a, base, Some(&nan)),
            (&a, base, Some(&nan)),
        ];
        let mut seen = Vec::new();
        let mut builds = 0;
        for (list, variant, axes) in requests {
            record_text_font(&mut seen, list, variant, axes, || {
                builds += 1;
                (list.clone(), variant, axes.cloned().unwrap_or_default())
            });
        }
        assert_eq!(seen.len(), 6);
        assert_eq!(builds, 6);
        assert_eq!(seen[0].0, a);
        assert_eq!(seen[1].0, b);
        assert_eq!(seen[2].1, bold);
        assert_eq!(seen[3].2, zero);
        assert!(seen[4].2 .0[0].1.is_nan());
        assert!(seen[5].2 .0[0].1.is_nan());
    }
    #[test]
    #[ignore = "diagnostic timing, run alone in release"]
    fn collection_timing_probe() {
        use std::{hint::black_box, time::Instant};
        let world = crate::world::SystemWorld::for_eval("/");
        let metrics = FallbackFontMetrics::new(&world);
        for (cohort, distinct, axes) in
            [("repeated", 1, false), ("axes", 1, true), ("diverse", 24, true)]
        {
            let items = (0..10000)
                .map(|n| {
                    let mut style = typst_core::entities::layout_types::TextStyle {
                        font: Some(FontList::single(
                            format!("Family{}", n % distinct).into(),
                        )),
                        ..typst_core::entities::layout_types::TextStyle::default()
                    };
                    if axes {
                        style.variations = Some(FontVariations(vec![
                            (*b"wght", 400.0),
                            (*b"wdth", 100.0),
                        ]));
                    }
                    #[allow(deprecated)]
                    FrameItem::Text {
                        pos: typst_core::entities::layout_types::Point {
                            x: typst_core::entities::layout_types::Pt(0.0),
                            y: typst_core::entities::layout_types::Pt(0.0),
                        },
                        text: "x".into(),
                        style,
                        bidi: None,
                    }
                })
                .collect::<Vec<_>>();
            for round in 0..31 {
                for legacy in if round % 2 == 0 { [true, false] } else { [false, true] } {
                    let mut seen = Vec::new();
                    let start = Instant::now();
                    if legacy {
                        for item in black_box(&items) {
                            #[allow(deprecated)]
                            let FrameItem::Text { style, .. } = item
                            else {
                                unreachable!()
                            };
                            let key = (
                                style.font.as_ref().unwrap().clone(),
                                text_style_to_font_variant(style),
                                style.variations.clone().unwrap_or_default(),
                            );
                            if !seen.contains(&key) {
                                seen.push(key);
                            }
                        }
                    } else {
                        collect_fonts_in_items(black_box(&items), &metrics, &mut seen);
                    }
                    let elapsed = start.elapsed().as_nanos();
                    assert_eq!(seen.len(), distinct);
                    black_box(&seen);
                    if round > 0 {
                        println!("P7,{cohort},{round},{legacy},{elapsed}");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod owned_export_font_tests {
    // Synthetic bytes isolate resource-table binding from font parsing.
    use super::bind_owned_export_fonts;
    use crate::export::FontKey;
    use typst_core::entities::{
        content::Content,
        font_book::{FontStretch, FontStyle, FontVariant, FontWeight},
        font_list::FontList,
        font_variations::FontVariations,
        frame_paint::{FramePaint, TilingFrame},
        geometry::{FillRule, ShapeKind},
        gradient::RelativeTo,
        layout_types::*,
        world_types::Font,
    };

    fn style() -> TextStyle {
        TextStyle {
            font: Some(FontList::single("Independent C22".into())),
            size: Pt(11.),
            ..Default::default()
        }
    }
    fn variant(s: &TextStyle) -> FontVariant {
        FontVariant {
            style: if s.italic { FontStyle::Italic } else { FontStyle::Normal },
            weight: FontWeight::from_number(s.weight.unwrap_or(if s.bold {
                700
            } else {
                400
            })),
            stretch: FontStretch::NORMAL,
        }
    }
    fn key(s: &TextStyle, bytes: &[u8]) -> FontKey {
        (
            (
                s.font.clone().unwrap(),
                variant(s),
                s.variations.clone().unwrap_or_default(),
            ),
            bytes.to_vec(),
        )
    }
    fn leaf(s: TextStyle, font: Option<Font>, text: &str) -> FrameItem {
        FrameItem::TextShaped {
            pos: Point { x: Pt(12.), y: Pt(23.) },
            glyphs: vec![ShapedGlyph {
                glyph_id: 7,
                x_advance: 400,
                x_offset: 9,
                y_offset: -2,
                cluster: 0,
                char_code: 'a',
            }]
            .into(),
            style: s,
            text: text.into(),
            units_per_em: 1000,
            font,
            bidi: None,
        }
    }
    fn face(s: &TextStyle, b: u8, t: &str) -> FrameItem {
        leaf(s.clone(), Some(Font::from_data(vec![b])), t)
    }
    fn page(items: Vec<FrameItem>) -> Page {
        Page {
            width: 123.,
            height: 234.,
            logical_number: 7,
            numbering: None,
            supplement: Content::text("supplement"),
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }
    }
    fn group(items: Vec<FrameItem>) -> FrameItem {
        FrameItem::Group {
            paint_root: false,
            pos: Point::ZERO,
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 12.,
            inner_height: 20.,
            items,
        }
    }
    fn wrapped(items: Vec<FrameItem>) -> FrameItem {
        group(vec![FrameItem::Link {
            target: LinkTarget::Url("https://example.test/".into()),
            pos: Point::ZERO,
            size: Size { width: Pt(10.), height: Pt(10.) },
            items: vec![FrameItem::Semantic {
                kind: SemanticKind::Formula,
                placement: SemanticPlacement::Inline,
                alt: Some("not a resource".into()),
                items: vec![FrameItem::TextReplacement {
                    text: "replacement".into(),
                    items,
                }],
            }],
        }])
    }
    fn tiling(items: Vec<FrameItem>) -> FrameItem {
        let mut frame = Frame::new(Size { width: Pt(10.), height: Pt(10.) });
        frame.items = items;
        FrameItem::Shape {
            pos: Point::ZERO,
            kind: ShapeKind::Rect,
            width: 20.,
            height: 20.,
            fill: Some(FramePaint::Tiling(std::sync::Arc::new(TilingFrame {
                frame,
                spacing: Size { width: Pt(0.), height: Pt(0.) },
                offset: Point::ZERO,
                angle: Angle::deg(0.),
                relative: RelativeTo::Self_,
            }))),
            stroke: None,
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        }
    }
    fn dump(d: &PagedDocument) -> String {
        format!("{d:?}")
    }

    // Preserve f32 payload/sign/order without relying on NaN's nonreflexive equality.
    fn snapshot(
        table: &[FontKey],
    ) -> Vec<(String, FontVariant, Vec<([u8; 4], u32)>, Vec<u8>)> {
        table
            .iter()
            .map(|((family, variant, axes), bytes)| {
                (
                    format!("{family:?}"),
                    *variant,
                    axes.0.iter().map(|(tag, v)| (*tag, v.to_bits())).collect(),
                    bytes.clone(),
                )
            })
            .collect()
    }
    fn assert_transferred(doc: &PagedDocument, table: Vec<FontKey>) {
        let pointer = table.as_ptr();
        let capacity = table.capacity();
        let expected = snapshot(&table);
        let buffers: Vec<_> =
            table.iter().map(|x| (x.1.as_ptr(), x.1.capacity())).collect();
        let before = dump(doc);
        let actual = bind_owned_export_fonts(doc, table);
        assert_eq!(snapshot(&actual), expected, "complete key/value identity");
        assert_eq!(dump(doc), before, "binder must not mutate document");
        assert_eq!(
            actual.as_ptr(),
            pointer,
            "complete table must transfer its allocation"
        );
        assert_eq!(actual.capacity(), capacity, "spare capacity must survive transfer");
        assert_eq!(
            actual
                .iter()
                .map(|x| (x.1.as_ptr(), x.1.capacity()))
                .collect::<Vec<_>>(),
            buffers,
            "all byte buffers must transfer"
        );
    }
    #[test]
    fn transfer_empty_spare_capacity() {
        assert_transferred(&PagedDocument::new(vec![]), Vec::with_capacity(19));
    }
    #[test]
    fn transfer_empty_zero_capacity() {
        assert_transferred(&PagedDocument::new(vec![]), Vec::new());
    }
    #[test]
    fn transfer_one_complete_face_with_spare_capacity() {
        let s = style();
        let mut table = Vec::with_capacity(17);
        let mut k = key(&s, &[1, 2, 3]);
        k.1.reserve(53);
        table.push(k);
        let mut p = page(vec![
            wrapped(vec![leaf(
                s.clone(),
                Some(Font::from_data(vec![1, 2, 3])),
                "nested",
            )]),
            tiling(vec![leaf(s.clone(), Some(Font::from_data(vec![1, 2, 3])), "tile")]),
        ]);
        p.background = vec![leaf(s.clone(), Some(Font::from_data(vec![1, 2, 3])), "bg")];
        p.foreground = vec![leaf(s, Some(Font::from_data(vec![1, 2, 3])), "fg")];
        assert_transferred(&PagedDocument::new(vec![p]), table);
    }
    #[test]
    fn transfer_plural_complete_faces_preserves_supplied_order_and_buffers() {
        let s = style();
        let mut bold = s.clone();
        bold.weight = Some(700);
        let mut axes = s.clone();
        axes.variations = Some(FontVariations(vec![(*b"ZZZZ", 4.)]));
        let mut table = Vec::with_capacity(31);
        for (st, b) in [(&s, 7), (&bold, 2), (&axes, 9), (&s, 1)] {
            let mut k = key(st, &[b, b + 1]);
            k.1.reserve(37);
            table.push(k);
        }
        let d = PagedDocument::new(vec![page(vec![
            leaf(axes, Some(Font::from_data(vec![9, 10])), "axes"),
            leaf(bold, Some(Font::from_data(vec![2, 3])), "bold"),
            leaf(s, Some(Font::from_data(vec![1, 2])), "last"),
        ])]);
        assert_transferred(&d, table);
    }
    #[test]
    fn missing_nested_faces_append_in_document_resource_order() {
        let s = style();
        let mut prefix = Vec::with_capacity(12);
        prefix.push(key(&s, &[90]));
        prefix.push(key(&s, &[91]));
        let mut p = page(vec![
            wrapped(vec![face(&s, 2, "wrapped")]),
            tiling(vec![face(&s, 3, "fill")]),
            face(&s, 2, "repeat"),
        ]);
        p.background = vec![face(&s, 1, "background")];
        p.foreground = vec![face(&s, 4, "foreground")];
        let d = PagedDocument::new(vec![p, page(vec![face(&s, 5, "next page")])]);
        let before = dump(&d);
        let actual = bind_owned_export_fonts(&d, prefix);
        let expected: Vec<_> =
            [90, 91, 1, 2, 3, 4, 5].into_iter().map(|n| key(&s, &[n])).collect();
        assert_eq!(snapshot(&actual), snapshot(&expected));
        assert_eq!(dump(&d), before);
    }
    #[test]
    fn missing_same_bytes_distinct_variant_axes_and_axis_order_append_complete_keys() {
        let s = style();
        let mut bold = s.clone();
        bold.weight = Some(700);
        let mut italic = s.clone();
        italic.italic = true;
        let mut a = s.clone();
        a.variations = Some(FontVariations(vec![(*b"AAAA", 1.), (*b"BBBB", 2.)]));
        let mut b = s.clone();
        b.variations = Some(FontVariations(vec![(*b"BBBB", 2.), (*b"AAAA", 1.)]));
        let styles = vec![bold, italic, a, b];
        let d = PagedDocument::new(vec![page(
            styles
                .iter()
                .flat_map(|s| [face(s, 8, "first"), face(s, 8, "again")])
                .collect(),
        )]);
        let actual = bind_owned_export_fonts(&d, vec![key(&s, &[8])]);
        let expected: Vec<_> = std::iter::once(&s)
            .chain(styles.iter())
            .map(|s| key(s, &[8]))
            .collect();
        assert_eq!(snapshot(&actual), snapshot(&expected));
    }
    #[test]
    fn signed_zero_and_none_empty_axes_use_unchanged_binder_equality() {
        let s = style();
        let mut empty = s.clone();
        empty.variations = Some(FontVariations(vec![]));
        let mut minus = s.clone();
        minus.variations = Some(FontVariations(vec![(*b"ZZZZ", -0.)]));
        let mut plus = s.clone();
        plus.variations = Some(FontVariations(vec![(*b"ZZZZ", 0.)]));
        let mut table = Vec::with_capacity(21);
        table.push(key(&s, &[8]));
        table.push(key(&minus, &[8]));
        let d = PagedDocument::new(vec![page(vec![
            face(&empty, 8, "empty"),
            face(&plus, 8, "zero"),
        ])]);
        assert_transferred(&d, table);
    }
    #[test]
    fn nan_never_deduplicates_and_keeps_payload_bits() {
        let mut s = style();
        s.variations = Some(FontVariations(vec![(*b"ZZZZ", f32::from_bits(0x7fc01234))]));
        let d = PagedDocument::new(vec![page(vec![
            face(&s, 8, "first"),
            face(&s, 8, "again"),
        ])]);
        let actual = bind_owned_export_fonts(&d, vec![key(&s, &[8])]);
        let expected = vec![key(&s, &[8]), key(&s, &[8]), key(&s, &[8])];
        assert_eq!(
            snapshot(&actual),
            snapshot(&expected),
            "existing NaN entry cannot satisfy either request"
        );
    }
    #[test]
    fn legacy_none_does_not_append_but_missing_some_does() {
        let s = style();
        let d = PagedDocument::new(vec![page(vec![
            leaf(s.clone(), None, "legacy"),
            face(&s, 2, "some"),
        ])]);
        let actual = bind_owned_export_fonts(&d, vec![key(&s, &[1])]);
        assert_eq!(snapshot(&actual), snapshot(&[key(&s, &[1]), key(&s, &[2])]));
        let legacy = PagedDocument::new(vec![page(vec![leaf(s, None, "legacy only")])]);
        assert_transferred(&legacy, Vec::with_capacity(13));
    }
    #[test]
    fn unavailable_family_with_invalid_bytes_does_not_invent_a_resource() {
        let d = PagedDocument::new(vec![page(vec![leaf(
            TextStyle::default(),
            Some(Font::from_data(vec![0, 1, 2])),
            "unknown",
        )])]);
        assert_transferred(&d, Vec::with_capacity(11));
    }
    #[test]
    fn independent_exports_large_small_large_keep_local_tables() {
        let s = style();
        for n in [24, 1, 24] {
            let d = PagedDocument::new(vec![page(
                (0..n).map(|i| face(&s, i, "face")).collect(),
            )]);
            let mut table = Vec::with_capacity(50);
            table.extend((0..n).rev().map(|i| key(&s, &[i])));
            assert_transferred(&d, table);
        }
    }

    #[test]
    fn complete_actual_face_matching_keeps_original_family_metadata() {
        let s = style();
        let mut alias = s.clone();
        alias.font = Some(FontList::single("alias family".into()));
        let d = PagedDocument::new(vec![page(vec![face(&alias, 8, "actual same face")])]);
        let mut table = Vec::with_capacity(16);
        table.push(key(&s, &[8]));
        assert_transferred(&d, table);
    }
    #[test]
    fn unused_nonreflexive_keys_transfer_without_normalization() {
        let mut s = style();
        s.variations = Some(FontVariations(vec![
            (*b"ZZZZ", f32::from_bits(0xffc01234)),
            (*b"AAAA", -0.),
        ]));
        let mut table = Vec::with_capacity(14);
        table.push(key(&s, &[5]));
        assert_transferred(&PagedDocument::new(vec![]), table);
    }
}
