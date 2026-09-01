//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export-fixtures.md
//! @prompt-hash 41241d6a
//! @layer L3
//! @updated 2026-05-19
//!
//! P307b snapshot binário — consumidor dos fixtures em
//! `03_infra/fixtures/p307b/`. Cada teste compila um fixture
//! `.typ` e compara byte-a-byte com o `.pdf` de referência gerado
//! em P307a.2. Falha → P307b.1 regrediu o output PDF para o
//! cluster que esse fixture exercita.
//!
//! Hash placeholder `ffffffff` propagado por
//! `crystalline-lint --fix-hashes .` no fecho de P307b.1.

#[cfg(test)]
mod p307b_snapshot {
    use std::path::{Path, PathBuf};

    use crate::export::StreamMode;
    use crate::fonts::discover_fonts;
    use crate::pipeline::compile_to_pdf_bytes;
    use crate::world::SystemWorld;
    use typst_core::contracts::world::World;

    /// Resolve um path relativo ao `03_infra/` (`CARGO_MANIFEST_DIR`).
    fn manifest_path(rel: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
    }

    /// Compila um fixture e devolve os bytes PDF.
    ///
    /// `src_rel`: path relativo a `03_infra/` (e.g. `"fixtures/p307b/sources/01-markup-plain.typ"`).
    /// `font_path_rel`: opcional, e.g. `"fixtures/fonts"` para fixture 09.
    fn compile_fixture(src_rel: &str, font_path_rel: Option<&str>) -> Vec<u8> {
        // P601 — datas fixas para manter os snapshots binários determinísticos.
        std::env::set_var("CRYSTALLINE_PDF_FIXED_EPOCH", "0");

        let src_path = manifest_path(src_rel);
        let root = src_path.parent().expect("source has parent dir").to_path_buf();
        let main_filename = src_path.file_name().expect("source has filename");
        let world = SystemWorld::new(&root, Path::new(main_filename))
            .expect("SystemWorld::new failed");

        let world = if let Some(fp_rel) = font_path_rel {
            let fp = manifest_path(fp_rel);
            let slots = discover_fonts(&[fp]);
            world.with_fonts(slots)
        } else {
            world
        };

        let source = world.source(world.main()).expect("source loaded");
        let (result, _warnings) = compile_to_pdf_bytes(
            &world,
            &source,
            StreamMode::Compact,
            // Snapshot histórico do formato Passo 20: também anterior ao
            // tagging P1140.6, portanto declara os dois eixos legados.
            crate::export::PdfTags::Disabled,
        );
        result.expect("compile_to_pdf_bytes failed")
    }

    fn assert_bytes_eq(actual: &[u8], reference_path: &str, fixture_name: &str) {
        let ref_full = manifest_path(reference_path);
        let expected = std::fs::read(&ref_full).unwrap_or_else(|e| {
            panic!("reference file não encontrado: {} ({})", ref_full.display(), e)
        });
        if actual != expected.as_slice() {
            // P520 — permitir regenerar referências quando o output PDF muda
            // intencionalmente (ex.: fix de kerning). Definir
            // `UPDATE_P307B_SNAPSHOTS=1` para gravar o actual como referência.
            if std::env::var("UPDATE_P307B_SNAPSHOTS").is_ok() {
                std::fs::write(&ref_full, actual).unwrap_or_else(|e| {
                    panic!("falha ao escrever referência {}: {}", ref_full.display(), e)
                });
                return;
            }
            panic!(
                "PDF binário regrediu: {} | actual={}B expected={}B",
                fixture_name,
                actual.len(),
                expected.len()
            );
        }
    }

    #[test]
    fn p307b_01_markup_plain() {
        let bytes = compile_fixture("fixtures/p307b/sources/01-markup-plain.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/01-markup-plain.pdf",
            "01-markup-plain",
        );
    }

    #[test]
    fn p307b_02_markup_heading() {
        let bytes = compile_fixture("fixtures/p307b/sources/02-markup-heading.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/02-markup-heading.pdf",
            "02-markup-heading",
        );
    }

    #[test]
    fn p307b_03_text_styling() {
        let bytes = compile_fixture("fixtures/p307b/sources/03-text-styling.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/03-text-styling.pdf",
            "03-text-styling",
        );
    }

    #[test]
    fn p307b_04_shapes() {
        let bytes = compile_fixture("fixtures/p307b/sources/04-shapes.typ", None);
        assert_bytes_eq(&bytes, "fixtures/p307b/reference/04-shapes.pdf", "04-shapes");
    }

    #[test]
    fn p307b_05_gradient_linear() {
        let bytes =
            compile_fixture("fixtures/p307b/sources/05-gradient-linear.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/05-gradient-linear.pdf",
            "05-gradient-linear",
        );
    }

    #[test]
    fn p307b_06_gradient_conic() {
        let bytes = compile_fixture("fixtures/p307b/sources/06-gradient-conic.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/06-gradient-conic.pdf",
            "06-gradient-conic",
        );
    }

    #[test]
    fn p307b_07_multi_feature() {
        let bytes = compile_fixture("fixtures/p307b/sources/07-multi-feature.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/07-multi-feature.pdf",
            "07-multi-feature",
        );
    }

    #[test]
    fn p307b_08_image_jpeg() {
        let bytes = compile_fixture("fixtures/p307b/sources/08-image-jpeg.typ", None);
        assert_bytes_eq(
            &bytes,
            "fixtures/p307b/reference/08-image-jpeg.pdf",
            "08-image-jpeg",
        );
    }

    #[test]
    fn p307b_09_cidfont() {
        let bytes = compile_fixture(
            "fixtures/p307b/sources/09-cidfont.typ",
            Some("fixtures/fonts"),
        );
        assert_bytes_eq(&bytes, "fixtures/p307b/reference/09-cidfont.pdf", "09-cidfont");
    }
}
